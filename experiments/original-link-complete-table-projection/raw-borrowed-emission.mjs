// Mechanical loans of immutable inputs; the public observation ABI stays owned.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData, writeProjectData, checkProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {readPortableSourceParameterStyles} from '../../js/scripts/portable-source-library-style.mjs';
import {ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT, readOriginalLinkRawConsumingProfile} from './raw-consuming-emission.mjs';

export const ORIGINAL_LINK_RAW_BORROWED_RECEIPT = new URL('./data/raw-borrowed-parameter-receipt.lino', import.meta.url);
const root = new URL('../../', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');
const descriptor = value => ({bytes: Buffer.byteLength(value), sha256: digest(value)});
const decisions = [
  'experiments/original-link-complete-table-projection/raw-borrowed-emission.mjs',
  'js/scripts/portable-source-library-style.mjs',
  'js/scripts/source-text-byte-library-style.mjs',
  'js/src/translation/lexer.js',
];
const cloneCalls = source => {
  const tokens = tokenize(source, 'Rust').tokens;
  return tokens.filter((token, index) => token.kind === 'identifier' && token.value === 'clone'
    && tokens[index - 1]?.value === '.' && tokens[index + 1]?.value === '(' && tokens[index + 2]?.value === ')').length;
};

/** Keep every checked owner and recover the entire predecessor from actual styles. */
export function prepareOriginalLinkRawBorrowedEmission(predecessor, {read = readFileSync, check = true} = {}) {
  const storedBytes = read(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const stored = readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  assert.equal(predecessor.profile, 'consuming');
  assert.equal(predecessor.proof.artifactSha256, digest(storedBytes),'the complete consuming predecessor bytes changed');
  assert.equal(stored.sourceSha256, predecessor.sourceSha256);
  const source = stored.completeRust;
  assert.deepEqual(descriptor(source), {bytes: predecessor.sourceBytes, sha256: predecessor.sourceSha256});
  const options = {ownedParameters: {check_original_link_raw_shape: ['columns']}};
  const styled = readPortableSourceParameterStyles(source, options);
  const names = text => [...text.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const header = text => text.split('\n').find(line => line.startsWith('pub fn check_original_link_raw_shape('));
  assert.equal(names(source).length, 62);
  assert.equal(stored.owners.length, 106);
  assert.deepEqual(names(styled.source), names(source));
  assert.equal(header(styled.source), header(source), 'the full public consumer interface stays exact');
  let inverse = styled.source;
  for (const action of [...styled.actions].reverse()) {
    assert.equal(action.op, 'replace'); assert.equal(action.field, 'source');
    assert.equal(inverse.split(action.to).length, 2, 'a whole styled function has exactly one inverse');
    inverse = inverse.replace(action.to, () => action.from);
  }
  assert.equal(inverse, source, 'every complete predecessor byte is recovered');
  const receipt = {version: 1, profile: 'borrowed consuming', options,
    predecessor: {...descriptor(storedBytes), source: descriptor(source)},
    source: descriptor(styled.source), definitionCount: 106,
    declarationNames: stored.declarationNames, publicEntryHeader: header(source),
    decisions: decisions.map(file => ({file, ...descriptor(read(new URL(file, root)))})),
    styles: styled.actions.map(action => ({name: names(action.from)[0],
      before: descriptor(action.from), after: descriptor(action.to)})),
    cloneCalls: {before: cloneCalls(source), after: cloneCalls(styled.source)},
    nativeCompilation: false, nativeExecution: false,
    boundary: 'Exact mechanical parameter styles and whole predecessor inverse; no native timing or execution claim.'};
  assert.ok(receipt.cloneCalls.after < receipt.cloneCalls.before);
  if (check) checkProjectData(ORIGINAL_LINK_RAW_BORROWED_RECEIPT, receipt);
  const profile = {profile: 'borrowed consuming', sourceBytes: receipt.source.bytes, sourceSha256: receipt.source.sha256,
    historicalSourceSha256: predecessor.historicalSourceSha256, currentSourceSha256: predecessor.currentSourceSha256,
    sourceArtifact: 'data/raw-borrowed-parameter-receipt.lino', declarationNames: predecessor.declarationNames,
    definitionCount: 106, proof: {artifactSha256: digest(structuredDataToLino(receipt)), predecessor, receipt}};
  return {source: styled.source, receipt, profile};
}

export function readOriginalLinkRawBorrowedProfile(currentProfile, read = readFileSync) {
  const predecessor = readOriginalLinkRawConsumingProfile(currentProfile, read);
  return prepareOriginalLinkRawBorrowedEmission(predecessor, {read}).profile;
}

if (process.argv[1] && import.meta.url === new URL(process.argv[1], 'file:').href) {
  assert.deepEqual(process.argv.slice(2), ['--write']);
  const stored = readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const predecessor = readOriginalLinkRawConsumingProfile(stored.predecessor);
  const {receipt} = prepareOriginalLinkRawBorrowedEmission(predecessor, {check: false});
  writeProjectData(ORIGINAL_LINK_RAW_BORROWED_RECEIPT, receipt);
  console.log('Stored mechanical loan receipt; native compilation and execution were not selected.');
}
