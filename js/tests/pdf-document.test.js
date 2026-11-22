import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork, ParseConfiguration } from '../src/index.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixture = readProjectData(new URL('../../parity/fixtures/pdf-document.lino', import.meta.url));
const bytes = Buffer.from(fixture.encoded, 'hex');
const source = bytes.toString('latin1');
const fixtureFile = 'parity/fixtures/pdf-document.lino';

function checkPdfContent() {
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
}

function checkDisabledDecoding() {
  const configuration = ParseConfiguration.default().withDocumentStreamDecoding(false);
  for (const network of [LinkNetwork.parse(source, 'PDF', configuration), LinkNetwork.parseBytes(bytes, 'PDF', configuration)]) {
    assert.equal(network.verifyFullMatch().isClean(), true);
    assert.equal(network.derivedDocuments().length, 0);
    assert.equal(network.visibleText().text, 'A€é');
  }
}

function checkCorruptDecoding() {
  const damaged = Buffer.from(fixture.corrupt, 'hex');
  const network = LinkNetwork.parseBytes(damaged, 'PDF');
  assert.equal(network.verifyFullMatch().isClean(), false);
  assert.deepEqual(Buffer.from(network.reconstructBytes()), damaged);
  const error = network.links().map(link => link.metadata()).find(metadata => metadata.flags.isError);
  assert.deepEqual({ ...error.span.byteRange }, { start: fixture.start, end: fixture.end });
  assert.equal(network.visibleText().text, 'A€é');
}

function checkReadingOrder() {
  const { page, text, ranges } = fixture.readingOrder;
  const bytes = Buffer.from(page);
  for (const [raw, input] of [[false, LinkNetwork.parse(page, 'PDF')], [true, LinkNetwork.parseBytes(bytes, 'PDF')]]) {
    const network = LinkNetwork.fromLino(input.toLino());
    assert.equal(network.verifyFullMatch().isClean(), true);
    if (raw) assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
    else assert.equal(network.reconstructText(), page);
    const projection = network.visibleText();
    assert.equal(projection.text, text);
    for (const [selected, anchors] of ranges) {
      const start = projection.text.indexOf(selected);
      assert.ok(start >= 0, selected);
      assert.deepEqual(projection.sourceRange(start, start + selected.length), {
        start: page.indexOf(anchors[0]),
        end: page.indexOf(anchors.at(-1)) + anchors.at(-1).length,
      });
    }
  }
}

function encodedPdfStream({ parameters, data }) {
  const encoded = Buffer.from(data, 'hex');
  const header = `%PDF-1.7\n1 0 obj\n<< /Length ${encoded.length} ${parameters} >>\nstream\n`;
  const source = header + encoded.toString('latin1') + '\nendstream\nendobj\n%%EOF\n';
  return { source, bytes: Buffer.from(source, 'latin1'), start: header.length, end: header.length + encoded.length };
}

function checkLzwDecoding() {
  const { padding, contents, text, streams, invalid } = fixture.lzw;
  for (const entry of streams) {
    const { source, bytes, start, end } = encodedPdfStream(entry);
    for (const [raw, document] of [[false, LinkNetwork.parse(source, 'PDF')], [true, LinkNetwork.parseBytes(bytes, 'PDF')]]) {
      const network = LinkNetwork.fromLino(document.toLino());
      assert.equal(network.verifyFullMatch().isClean(), true);
      if (raw) assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
      else assert.equal(network.reconstructText(), source);
      const range = { start, end: raw ? end : Buffer.byteLength(source.slice(0, end)) };
      const documents = network.derivedDocuments();
      assert.equal(documents.length, 1);
      assert.equal(documents[0].compression, entry.compression);
      assert.deepEqual({ ...documents[0].sourceRange }, range);
      assert.equal(documents[0].network.reconstructText(), ' '.repeat(padding) + contents);
      const projection = network.visibleText();
      assert.equal(projection.text, text);
      assert.deepEqual(projection.sourceRange(0, text.length), range);
    }
  }
  for (const entry of invalid) {
    const { bytes, start, end } = encodedPdfStream(entry);
    const network = LinkNetwork.parseBytes(bytes, 'PDF');
    assert.equal(network.verifyFullMatch().isClean(), false);
    assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
    assert.equal(network.derivedDocuments().length, 0);
    const error = network.links().map(link => link.metadata()).find(metadata => metadata.flags.isError);
    assert.deepEqual({ ...error.span.byteRange }, { start, end });
  }
}

test('PDF streams and font Unicode keep the original ranges with both byte entry points', () => {
  checkPdfContent();
  checkDisabledDecoding();
  checkCorruptDecoding();
  checkReadingOrder();
  checkLzwDecoding();
  for (const [requirementId, assertions] of [
    ['I195-I239-PDF-CONTENT-STREAMS', ['pdfDerivedOperatorsKeepEncodedRange', 'pdfFiltersCanBeDisabled', 'corruptPdfFilterKeepsLocatedError', 'pdfLzwCodeWidthsAndFilterChainsKeepEncodedRange', 'corruptPdfLzwKeepsLocatedError']],
    ['I195-I239-PDF-FONT-UNICODE', ['pdfTextUsesFontMappingsAndPageOrder', 'pdfVisibleTextKeepsEncodedAndLiteralRanges', 'pdfPositionedLinesKeepReadingOrderAndRanges']],
  ]) {
    recordIssue195Observations({ requirementId, suffix: 'behavior', fixtureId: requirementId.toLowerCase(),
      fixtureFile, assertions, testName: 'PDF streams and font Unicode keep the original ranges with both byte entry points' });
  }
});
