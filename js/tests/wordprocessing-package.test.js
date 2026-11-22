import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork } from '../src/index.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixture = readProjectData(new URL('../../parity/fixtures/wordprocessing-package.lino', import.meta.url));
const relationshipFixture = readProjectData(new URL('../../parity/fixtures/wordprocessing-relationships.lino', import.meta.url));

function checkPackagedParts(data, firstWord) {
  for (const { archive, compression, ranges } of data.cases) {
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
      assert.deepEqual(documents.map(document => document.name), data.names);
      assert.deepEqual(documents.map(document => document.network.reconstructText()), data.parts);
      const convert = encoding === 'bytes' ? offset => offset : coordinate;
      assert.deepEqual(documents.map(document => [document.sourceRange.start, document.sourceRange.end]),
        ranges.map(([start, end]) => [convert(start), convert(end)]));
      assert.ok(documents.every(document => document.compression === compression && document.encoding === encoding));
      const projection = network.visibleText();
      assert.equal(projection.text, data.expectedText);
      const exact = data.parts[0].indexOf(firstWord);
      const start = ranges[0][0] + Buffer.byteLength(data.parts[0].slice(0, exact));
      const expected = compression === 'stored' ? [start, start + Buffer.byteLength(firstWord)] : ranges[0];
      assert.deepEqual(projection.sourceRange(0, firstWord.length), { start: convert(expected[0]), end: convert(expected[1]) });
      const all = projection.sourceRange(0, projection.text.length);
      if (data === fixture) {
        assert.ok(all.start <= convert(ranges[1][0] + 200) && all.end >= convert(ranges[3][0]));
      } else {
        for (const [start, end] of ranges) assert.ok(all.start < convert(end) && all.end > convert(start));
      }
    }
  }
}

function checkDamagedPart() {
  const bytes = Buffer.from(fixture.corrupt, 'hex');
  const network = LinkNetwork.parseBytes(bytes, 'DOCX');
  assert.equal(network.verifyFullMatch().isClean(), false);
  const diagnostics = network.links().map(link => link.metadata()).filter(metadata => metadata.flags.isError);
  const checksum = diagnostics.find(metadata => metadata.definition?.includes('OPC part size or checksum mismatch'));
  assert.equal(checksum.term, 'ERROR');
  assert.deepEqual([checksum.span.byteRange.start, checksum.span.byteRange.end], fixture.cases[0].ranges[0]);
  assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
  assert.deepEqual(network.derivedDocuments().map(document => document.name), fixture.names.slice(1));
}

test('WordprocessingML packages keep parts and character ranges', () => {
  checkPackagedParts(fixture, 'Кошка');
  checkDamagedPart();
  recordIssue195Observations({ requirementId: 'I195-I239-WORDPROCESSING-PACKAGE', suffix: 'behavior',
    fixtureId: 'i195-i239-wordprocessing-package', fixtureFile: 'parity/fixtures/wordprocessing-package.lino',
    assertions: ['opcPackageKeepsStoredAndDeflatedParts', 'opcVisibleTextKeepsEncodedAndStoredRanges', 'corruptOpcPartKeepsLocatedError'],
    testName: 'WordprocessingML packages keep parts and character ranges' });
});

test('WordprocessingML package relationships select named parts and resolve internal targets', () => {
  checkPackagedParts(relationshipFixture, 'Корпус');
  const bytes = Buffer.from(relationshipFixture.missing.archive, 'hex');
  const network = LinkNetwork.parseBytes(bytes, 'DOCX');
  assert.equal(network.verifyFullMatch().isClean(), false);
  assert.deepEqual(Buffer.from(network.reconstructBytes()), bytes);
  const missing = network.links().map(link => link.metadata()).find(metadata =>
    metadata.flags.isError && metadata.definition?.includes('../../../outside.xml'));
  assert.equal(missing?.term, 'ERROR');
  assert.deepEqual([missing.span.byteRange.start, missing.span.byteRange.end], relationshipFixture.missing.relationshipRange);
  assert.deepEqual(network.derivedDocuments().map(document => document.name), relationshipFixture.names.filter((_, index) => index !== 1));
  recordIssue195Observations({ requirementId: 'I195-I239-PACKAGE-RELATIONSHIPS', suffix: 'behavior',
    fixtureId: 'i195-i239-package-relationships', fixtureFile: 'parity/fixtures/wordprocessing-relationships.lino',
    assertions: ['opcRelationshipsResolveInternalParts', 'opcRelationshipsKeepSourceRanges', 'invalidOpcTargetKeepsRelationshipRange'],
    testName: 'WordprocessingML package relationships select named parts and resolve internal targets' });
});
