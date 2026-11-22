import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork, ParseConfiguration } from '../src/index.js';
import { readProjectData } from '../scripts/project-data.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/pdf-document.lino', import.meta.url));
const bytes = Buffer.from(fixture.encoded, 'hex');
const source = bytes.toString('latin1');

test('PDF preserves encoded bytes and links the decoded content operators and font-mapped text', () => {
  for (const [document, coordinates, encoding] of [
    [LinkNetwork.parse(source, 'PDF'), offset => Buffer.byteLength(source.slice(0, offset)), 'binary-string'],
    [LinkNetwork.parseBytes(bytes, 'PDF'), offset => offset, 'bytes'],
  ]) {
    const network = LinkNetwork.fromLino(document.toLino());
    assert.equal(network.verifyFullMatch().isClean(), true);
    if (encoding === 'bytes') assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
    else assert.equal(network.reconstructText(), source);
    const documents = network.derivedDocuments();
    assert.equal(documents.length, 1);
    const [stream] = documents;
    assert.equal(stream.name, 'content_stream');
    assert.equal(stream.compression, 'FlateDecode');
    assert.equal(stream.network.reconstructText(), fixture.contents);
    assert.ok(stream.network.links().some(link => link.metadata().term === 'operation'));
    const range = { start: coordinates(fixture.start), end: coordinates(fixture.end) };
    assert.deepEqual({ ...stream.sourceRange }, range);
    const projection = network.visibleText();
    assert.equal(projection.text, fixture.text);
    assert.deepEqual(projection.sourceRange(0, 'чай 🧪B'.length), range);
    const raw = network.visibleText({ blockSeparator: '|' });
    assert.equal(raw.text, fixture.text.replace('\n', '|'));
    const rawPosition = source.indexOf(String.raw`(A\200\201)`);
    assert.deepEqual(raw.sourceRange(raw.text.indexOf('A'), raw.text.length), {
      start: coordinates(rawPosition), end: coordinates(rawPosition + String.raw`(A\200\201)`.length),
    });
  }
});

test('PDF stream decoding can be disabled while raw text remains available', () => {
  const network = LinkNetwork.parse(source, 'PDF', ParseConfiguration.default().withDocumentStreamDecoding(false));
  assert.equal(network.verifyFullMatch().isClean(), true);
  assert.equal(network.derivedDocuments().length, 0);
  assert.equal(network.visibleText().text, 'A€é');
});

test('corrupt PDF Flate data is flagged at the encoded stream and stays lossless', () => {
  const damaged = Buffer.from(fixture.corrupt, 'hex');
  const network = LinkNetwork.parseBytes(damaged, 'PDF');
  assert.equal(network.verifyFullMatch().isClean(), false);
  assert.deepEqual(Buffer.from(network.reconstructBytes()), damaged);
  const error = network.links().map(link => link.metadata()).find(metadata => metadata.flags.isError);
  assert.deepEqual({ ...error.span.byteRange }, { start: fixture.start, end: fixture.end });
  assert.equal(network.visibleText().text, 'A€é');
});
