// CI-only observation of the complete recorded library candidate. The unchanged
// oracle owns the forty inputs/results and its consumer is reused byte for byte.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { tokenize } from '../../js/src/translation/lexer.js';
import { LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary, structuredDataToLino, readStructuredDataLino } from '../../js/src/index.js';
import { createSourceNativeObservationRunner } from '../source-native-observations.mjs';
import { planRustTextCertificateNative } from '../rust-text-certificate/native-observations.mjs';
import { createTextCertificateLibrary } from './generate.mjs';

export const RUST_TEXT_CERTIFICATE_LIBRARY_MODEL_COMMIT = '5696add6322fceffb89d7ad433809ed4262597a1';
const sourceDigests = {
  '../rust-text-certificate/native-observations.mjs': '2a9ee52f6b2bf005d92a3bc8e4e3aae9ac59dfbf0e698c3cb9a928b5b731f527',
  '../../js/scripts/rust-library-packaging.mjs': '8f0d17f29e9ee655c937080445916bce770d40a84f580431a7a650d031a02abe',
  'generate.mjs': '16bec4d69e965a5acbe8c60e7993597ddf29cf540f8273b6a8ae3fd510472490',
  'data/policy.lino': 'a3b9418856ebd653878abfcf62a88c811439e40f1f1477dddbed1b80fccbfcce',
  'data/decorators.lino': '847970c8c109d3888a5922c2e6de88f583479ea7412b7c7bbb114399c525bf09',
  'data/library.lino': 'b245306f1a65ea35f78be09617d38ac16aed0d1d8a266a98e39201426b35fbd7',
};
const checkedPlans = new WeakSet();
const sha256 = value => createHash('sha256').update(value).digest('hex');
function textBytes(value) {
  if (typeof value !== 'string' || !value.isWellFormed()) throw new TypeError('library oracle source must be primitive Unicode scalar text');
  return Buffer.from(value, 'utf8');
}
function freeze(value) {
  if (value && typeof value === 'object') { Object.values(value).forEach(freeze); Object.freeze(value); }
  return value;
}
export function verifyRustTextCertificateLibraryModel(read = readFileSync) {
  return Object.entries(sourceDigests).map(([file, expected]) => {
    const actual = sha256(read(new URL(file, import.meta.url)));
    assert.equal(actual, expected, 'the frozen Rust library candidate changed: ' + file);
    return { file, sha256: actual };
  });
}
export function rustTextCertificateLibraryNativeMode(environment = process.env) {
  if (environment.RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE !== '1') return false;
  if (environment.CI !== 'true') throw new TypeError('RUST_TEXT_CERTIFICATE_LIBRARY_NATIVE=1 requires CI=true; native compilation is forbidden locally');
  return true;
}
function sourceReceipt(relativePath, source) {
  return { relativePath, source, sourceBytes: textBytes(source).length, sourceSha256: sha256(textBytes(source)) };
}
function publicHeader(source, original) {
  const tokens = tokenize(source, 'Rust').tokens, occurrences = [];
  let depth = 0;
  for (let index = 0; index < tokens.length; index += 1) {
    if (depth === 0 && tokens[index].value === 'pub' && tokens[index + 1]?.value === 'fn'
      && tokens[index + 2]?.value === original.functionName) occurrences.push(index);
    if (tokens[index].kind === 'punct' && tokens[index].value === '{') depth += 1;
    if (tokens[index].kind === 'punct' && tokens[index].value === '}') depth -= 1;
  }
  assert.equal(depth, 0); assert.equal(occurrences.length, 1, 'the complete library owns one actual public checker');
  assert.equal(original.tableOrder.length, 17); assert.deepEqual(original.parameterModes, Array(17).fill('Vec<f64>'));
  assert.equal(original.resultMode, 'Vec<f64>');
  const expected = ['pub', 'fn', original.functionName, '('];
  original.tableOrder.forEach((name, index) => { if (index) expected.push(','); expected.push(name, ':', 'Vec', '<', 'f64', '>'); });
  expected.push(')', '->', 'Vec', '<', 'f64', '>', '{');
  assert.deepEqual(tokens.slice(occurrences[0], occurrences[0] + expected.length).map(token => token.value), expected,
    'the actual packaged checker must preserve the original ordered seventeen-vector ABI');
  return { tokenCount: tokens.length, functionName: original.functionName, parameterModes: original.parameterModes, resultMode: original.resultMode };
}
/** JavaScript planning only: no compiler, formatter or child process is called. */
export function planRustTextCertificateLibraryNative() {
  const modelSources = verifyRustTextCertificateLibraryModel(), { packet, decorators } = createTextCertificateLibrary();
  assert.deepEqual(readProjectData(new URL('./data/library.lino', import.meta.url)), packet);
  assert.equal(readFileSync(new URL('./data/decorators.lino', import.meta.url), 'utf8'), decorators);
  assert.equal(packet.modulePath, 'text_certificate_library'); assert.equal(packet.definitions.length, 92); assert.equal(packet.items.length, 96);
  assert.deepEqual(packet.modules.map(module => module.file), ['mod.rs', 'part_0.rs', 'part_1.rs', 'part_2.rs', 'part_3.rs']);
  const originalPlan = planRustTextCertificateNative(), original = originalPlan.entries[0];
  assert.equal(originalPlan.entries.length, 1); assert.equal(original.observations.length, 40);
  assert.deepEqual(packet.definitions.map(row => row.originalName), original.declarations);
  const header = publicHeader(packet.modules.slice(1).map(module => module.source).join('\n'), original);
  const source = '#[path = "library/mod.rs"]\npub mod text_certificate_library;\npub use text_certificate_library::check_rust_text_certificate;\n';
  const primary = sourceReceipt('original.rs', source);
  const additionalSources = packet.modules.map(module => {
    const receipt = sourceReceipt('library/' + module.file, module.source);
    assert.equal(receipt.sourceSha256, module.sha256); return receipt;
  });
  const sourceFiles = [primary, ...additionalSources];
  const lino = structuredDataToLino(sourceFiles), replay = readStructuredDataLino(
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(lino))).toLino());
  assert.deepEqual(replay, sourceFiles, 'all six compilation sources must retain exact bytes through text and binary Links');
  const entry = {
    id: 'complete recorded Rust text certificate library', kind: 'complete packaged generated library with namespace reexport', fixtureId: 'data/library.lino',
    source, sourceBytes: primary.sourceBytes, sourceSha256: primary.sourceSha256, replaySha256: replay[0].sourceSha256,
    originalReceipt: { sourceBytes: primary.sourceBytes, sourceSha256: primary.sourceSha256 }, additionalSources,
    sourceFiles: sourceFiles.map(({ source: _source, ...receipt }) => receipt),
    originalArtifactReceipt: original.originalReceipt, originalModel: originalPlan.model, originalModelCommit: originalPlan.modelCommit,
    ...header, tableOrder: original.tableOrder, outputRoles: original.outputRoles, declarations: original.declarations,
    definitionOwners: packet.definitions, itemCount: packet.items.length, packagedModuleCount: packet.modules.length,
    invocationCount: original.invocationCount, originalGraphCount: original.originalGraphCount, corruptionCount: original.corruptionCount,
    consumerSource: original.consumerSource, consumerSha256: original.consumerSha256, javaScriptSha256: original.javaScriptSha256,
    observations: original.observations, validationBoundary: packet.validationBoundary,
  };
  assert.equal(sha256(entry.consumerSource), original.consumerSha256, 'reuse the original forty-call consumer without replacement');
  const plan = freeze({ model: packet.model, modelCommit: RUST_TEXT_CERTIFICATE_LIBRARY_MODEL_COMMIT,
    modelSources: [...originalPlan.modelSources.map(row => ({ ...row, file: '../rust-text-certificate/' + row.file })), ...modelSources],
    originalCount: 1, entries: [entry] });
  checkedPlans.add(plan); return plan;
}
function parseObservations(output, expected) {
  const lines = output.split('\n'); assert.equal(lines.pop(), '', 'the native bits stream must end with a newline');
  assert.equal(lines.length, 40, 'the complete library must report every original invocation once'); assert.equal(lines.length, expected.length);
  return lines.map((line, index) => {
    const parts = line.split(':'); assert.equal(parts.length, 6); assert.equal(parts[0], String(index));
    const actual = parts.slice(1); actual.forEach(bits => assert.match(bits, /^[0-9a-f]{16}$/u));
    assert.deepEqual(actual, expected[index].resultBits, 'the actual packaged library differs from the unchanged forty-call oracle');
    return { id: expected[index].id, kind: expected[index].kind, fixtureId: expected[index].fixtureId,
      sourceReceipt: expected[index].sourceReceipt, archiveFormats: expected[index].archiveFormats,
      inputTables: expected[index].inputBits.map(bits => ({ cells: bits.length, sha256: sha256(bits.join('')) })),
      resultBits: actual, decision: actual.map(bits => Buffer.from(bits, 'hex').readDoubleBE()) };
  });
}
export const runRustTextCertificateLibraryNative = createSourceNativeObservationRunner({
  nativeMode: rustTextCertificateLibraryNativeMode, checkedPlan: plan => checkedPlans.has(plan), textBytes, parseObservations,
  entryReceipt: entry => ({ functionName: entry.functionName, parameterModes: entry.parameterModes, resultMode: entry.resultMode,
    tokenCount: entry.tokenCount, tableOrder: entry.tableOrder, outputRoles: entry.outputRoles, declarations: entry.declarations,
    definitionOwners: entry.definitionOwners, itemCount: entry.itemCount, packagedModuleCount: entry.packagedModuleCount,
    sourceFileCount: entry.sourceFiles.length, sourceFiles: entry.sourceFiles, originalArtifactReceipt: entry.originalArtifactReceipt,
    originalModel: entry.originalModel, originalModelCommit: entry.originalModelCommit,
    invocationCount: entry.invocationCount, originalGraphCount: entry.originalGraphCount, corruptionCount: entry.corruptionCount,
    // The immutable candidate packet describes its generation-time boundary;
    // successful native observations below add evidence without editing it.
    generationBoundary: entry.validationBoundary }),
  temporaryPrefix: 'rust-text-certificate-library-', failureLabel: 'Complete recorded Rust library observations failed',
});
