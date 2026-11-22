import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork } from '../src/index.js';
import { readProjectData } from '../scripts/project-data.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/wordprocessing-package.lino', import.meta.url));

for (const { archive, compression, ranges } of fixture.cases) {
  test(`WordprocessingML package retains ${compression} parts and original source bytes`, () => {
    const bytes = Buffer.from(archive, 'hex');
    const binaryString = bytes.toString('latin1');
    const coordinate = offset => Buffer.byteLength(binaryString.slice(0, offset));
    for (const [encoding, original] of [
      ['bytes', LinkNetwork.parseBytes(bytes, 'DOCX')],
      ['binary-string', LinkNetwork.parse(binaryString, 'DOCX')],
    ]) {
      const network = LinkNetwork.fromLino(original.toLino());
      assert.equal(network.verifyFullMatch().isClean(), true);
      if (encoding === 'bytes') assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
      else assert.equal(network.reconstructText(), binaryString);
      const documents = network.derivedDocuments();
      assert.deepEqual(documents.map(document => document.name), fixture.names);
      assert.deepEqual(documents.map(document => document.network.reconstructText()), fixture.parts);
      const convert = encoding === 'bytes' ? offset => offset : coordinate;
      assert.deepEqual(documents.map(document => [document.sourceRange.start, document.sourceRange.end]),
        ranges.map(([start, end]) => [convert(start), convert(end)]));
      assert.ok(documents.every(document => document.compression === compression && document.encoding === encoding));
      const projection = network.visibleText();
      assert.equal(projection.text, fixture.expectedText);
      const exact = fixture.parts[0].indexOf('Кошка');
      const start = ranges[0][0] + Buffer.byteLength(fixture.parts[0].slice(0, exact));
      const expected = compression === 'stored' ? [start, start + Buffer.byteLength('Кошка')] : ranges[0];
      assert.deepEqual(projection.sourceRange(0, 'Кошка'.length), { start: convert(expected[0]), end: convert(expected[1]) });
      const all = projection.sourceRange(0, projection.text.length);
      assert.ok(all.start <= convert(ranges[1][0] + 200) && all.end >= convert(ranges[3][0]));
    }
  });
}

test('a damaged ZIP part reports a specific error without losing the package bytes', () => {
  const bytes = Buffer.from(fixture.corrupt, 'hex');
  const network = LinkNetwork.parseBytes(bytes, 'DOCX');
  assert.equal(network.verifyFullMatch().isClean(), false);
  const diagnostics = network.links().map(link => link.metadata()).filter(metadata => metadata.flags.isError);
  const checksum = diagnostics.find(metadata => metadata.definition?.includes('OPC part size or checksum mismatch'));
  assert.equal(checksum.term, 'ERROR');
  assert.deepEqual([checksum.span.byteRange.start, checksum.span.byteRange.end], fixture.cases[0].ranges[0]);
  assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
  assert.deepEqual(network.derivedDocuments().map(document => document.name), fixture.names.slice(1));
});
