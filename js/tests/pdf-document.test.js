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

test('PDF streams and font Unicode keep the original ranges with both byte entry points', () => {
  checkPdfContent();
  checkDisabledDecoding();
  checkCorruptDecoding();
  for (const [requirementId, assertions] of [
    ['I195-I239-PDF-CONTENT-STREAMS', ['pdfDerivedOperatorsKeepEncodedRange', 'pdfFiltersCanBeDisabled', 'corruptPdfFilterKeepsLocatedError']],
    ['I195-I239-PDF-FONT-UNICODE', ['pdfTextUsesFontMappingsAndPageOrder', 'pdfVisibleTextKeepsEncodedAndLiteralRanges']],
  ]) {
    recordIssue195Observations({ requirementId, suffix: 'behavior', fixtureId: requirementId.toLowerCase(),
      fixtureFile, assertions, testName: 'PDF streams and font Unicode keep the original ranges with both byte entry points' });
  }
});
