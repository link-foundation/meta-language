// A finite successor to the original inference-source applicability proof.
// This verifier never replaces a producer snapshot or manufactures Rust text.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readStructuredDataLino, structuredDataToLino } from '../../js/src/structured-data-links.js';
import { restoreCompleteDeclarationReceipt } from '../../js/scripts/complete-declaration-receipts.mjs';
import {restoreTranslationSourceHistory} from '../../js/scripts/translation-source-history.mjs';

const applicabilityDigest = value => createHash('sha256').update(value).digest('hex');
const freezeApplicability = value => {
  if (value && typeof value === 'object') {
    Object.values(value).forEach(freezeApplicability);
    Object.freeze(value);
  }
  return value;
};
const originalProducerDescriptors = freezeApplicability([
  { file: 'js/src/translation/javascript-surface-rules.js', bytes: 8574, sha256: '794d918304dac11b384a3d29a1fd83235cacd51cd2cfe19ca630cfa2833ac40c' },
  { file: 'js/src/translation/javascript-infer.js', bytes: 62514, sha256: '55e1189678b62bdcb5542d4fcaa9db0aa33745ad7c13c3e8f7b0aa1ae2e66e62' },
]);

export const ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY = freezeApplicability({
  version: 1,
  revision: 'collection construction extraction 1',
  introducingCommit: 'b51d48140220bf2ac071334d79d28aa77273c00e',
  inference: { file: 'js/src/translation/javascript-infer.js', bytes: 61836, sha256: '3b4afafb648b8205408c4ca8832803bcb3ec20fb4ac3dc989d5ddec1548a5471' },
  addedImport: "import { inferCollectionConstruction } from './collection-construction.js';\n",
  currentBranches: "      case 'set':\n      case 'map':\n        return inferCollectionConstruction(this, node, env, expected);\n",
  originalBranches: `      case 'set': {
        const element = expected?.kind === 'set' ? expected.element : this.fresh();
        if (node.items) this.unify(this.expr(node.items, env, { kind: 'array', element }), { kind: 'array', element }, node.span);
        const term = { kind: 'set', element };
        this.collections.set(node, term);
        return term;
      }
      case 'map': {
        const key = expected?.kind === 'map' ? expected.key : this.fresh();
        const value = expected?.kind === 'map' ? expected.value : this.fresh();
        const pair = { kind: 'tuple', elements: [key, value] };
        if (node.entries) this.unify(this.expr(node.entries, env, { kind: 'array', element: pair }), { kind: 'array', element: pair }, node.span);
        const term = { kind: 'map', key, value };
        this.collections.set(node, term);
        return term;
      }
`,
  sourceClosure: {
    bytes: 10257,
    sha256: '53a5bb4cdeec63423be142da305f494d0bb4a669bbbf5025b7b7ab6f34fc86f3',
    removedImport: "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n",
    inputs: ['js/src/translation/fact-column-rules.js', 'js/src/translation/collection-construction-rules.js'],
    originalFunctions: 16,
    checkedOwners: 18,
  },
  dependencies: [
    { file: 'js/src/translation/collection-construction.js', bytes: 3471, sha256: '9ad3a43eceb170c802808c76ecb196762c896a07f35e3947045bf4ba3c75df8a' },
    { file: 'js/src/translation/collection-construction-rules.js', bytes: 8727, sha256: 'e9e21ed1c12c208190ec62595d2f3f52fd6a1329a82df133832f7add8f683502' },
    { file: 'js/src/translation/fact-column-rules.js', bytes: 1616, sha256: '66fd9a9e0628e88cef7322072ad70bbb53c5b31731cb91618afafd1e52127222' },
    { file: 'js/src/translation/surface-reference-facts.js', bytes: 1739, sha256: 'a81bc9df51bbef4622b28ff02d12d0ff4651a0022dd5127df607329448a14b91' },
    { file: 'js/src/translation/types.js', bytes: 5680, sha256: '06d5f2658cfc250fd02b7a8c309e200cc4755ae2201d6a03f5f9a6db04c1cac9' },
    { file: 'js/src/translation/diagnostics.js', bytes: 907, sha256: 'a5d8ddf02252f016b8568373248b67165b37ce747442a21a7bef3c63e2a385ab' },
    { file: 'js/src/translation/optional-value-rules.js', bytes: 20853, sha256: '6fd1821037bdbed3f914f55b2e383d50bf603f48646c3523c2921876484d9514' },
    { file: 'js/scripts/generate-collection-construction-rules.mjs', bytes: 7648, sha256: 'd82bd93b9446cbad1067726b027d72ce7f5e9f4f9d30354f794c10a0e13dd614' },
    { file: 'js/src/translation/data/collection-construction-rules.lino', bytes: 42185, sha256: '282fa967dd53403985553484c64a3399298c1eb6098efb2f62966d36c3496a04' },
    { file: 'rust/src/data/collection-construction-rules.lino', bytes: 42185, sha256: '282fa967dd53403985553484c64a3399298c1eb6098efb2f62966d36c3496a04' },
    { file: 'parity/self-translation/collection-construction-rules-native.lino', bytes: 231610, sha256: '718edf73bc046371c6e5524fc1f9956e2ef89bf6d9d5c933c2fcba54a2eaee96' },
    { file: 'js/tests/translation-collection-construction.test.js', bytes: 20006, sha256: '161219817933f71bc3af09b681c0e0b2f88220b5871063314aa0963440021e19' },
  ],
});

// Only the two complete checked-archive digests changed with the reviewed
// constant-declaration source spans. This does not repin any causal producer.
export const ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH = freezeApplicability({
  version: 1,
  revision: 'collection construction checked archive caller spans 1',
  introducingCommit: '41109ef24a0441b87107f572123ab8f9b6626e62',
  originalRevision: '858665058648a3f1f9ce05ff3b0668b9ccfd66eb',
  previous: { file: 'parity/self-translation/collection-construction-rules-native.lino',
    bytes: 231610, sha256: '718edf73bc046371c6e5524fc1f9956e2ef89bf6d9d5c933c2fcba54a2eaee96' },
  current: { file: 'parity/self-translation/collection-construction-rules-native.lino',
    bytes: 231610, sha256: 'ab9c395b5fd388564ff23801bf4c07b114fb206bd5895b7d619a07653a231dc9' },
  changes: [
    { field: 'textSha256', before: '931e56cc8ffe201d74db2a9154ee8d0beb7ba8e1560140bbdba63ab4a5812887',
      after: '0a191c72fee0db6db444335e729366e4c9f6fd9d9764aecb27b8135b8687b114' },
    { field: 'binarySha256', before: 'dde6292a58a87f4eb43c1109919582e4f99544d0a633377e7813037c80ebc758',
      after: 'f460be77aa8316195e37a9612f9fb64605cac5b0644bb2040e64916e40ece8cd' },
  ],
});

function verifyApplicabilitySource(sources, descriptor) {
  const bytes = sources[descriptor.file];
  assert.ok(Buffer.isBuffer(bytes), 'actual complete applicability input bytes are supplied: ' + descriptor.file);
  assert.equal(bytes.length, descriptor.bytes, 'applicability input byte count changed: ' + descriptor.file);
  assert.equal(applicabilityDigest(bytes), descriptor.sha256, 'applicability input changed: ' + descriptor.file);
  return { ...descriptor };
}

function verifyCollectionArchiveReceiptRefresh(sources, descriptor) {
  const refresh = ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH;
  assert.deepEqual(descriptor, refresh.previous, 'only the exact recorded artifact has a receipt successor');
  const actual = sources[refresh.current.file];
  assert.ok(Buffer.isBuffer(actual));
  const current = readStructuredDataLino(actual.toString('utf8'));
  assert.deepEqual(actual, Buffer.from(structuredDataToLino(current)), 'the complete supplied receipt bytes are canonical and exact');
  const artifact = restoreCompleteDeclarationReceipt(current, 'collection construction');
  const previousBytes = Buffer.from(structuredDataToLino(artifact));
  verifyApplicabilitySource({ ...sources, [refresh.current.file]: previousBytes }, refresh.current);
  const source = { file: refresh.current.file, bytes: actual.length, sha256: applicabilityDigest(actual) };
  const inverse = structuredClone(artifact);
  for (const change of refresh.changes) {
    assert.equal(inverse.archive[change.field], change.after, 'the exact new archive digest is recorded');
    inverse.archive[change.field] = change.before;
  }
  const previous = Buffer.from(structuredDataToLino(inverse));
  verifyApplicabilitySource({ [descriptor.file]: previous }, descriptor);
  return { source, receiptRefresh: refresh };
}

function restoreRawEmissionInferenceSource(source) {
  const revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  assert.equal(source.split(revision.addedImport).length, 2, 'the extraction has exactly one actual import');
  assert.equal(source.split(revision.currentBranches).length, 2, 'the extraction has exactly one actual constructor dispatcher');
  assert.equal(source.split(revision.originalBranches).length, 1, 'the original branch block is not already present');
  return source.replace(revision.addedImport, '').replace(revision.currentBranches, () => revision.originalBranches);
}

function verifyCollectionApplicabilityClosure(sources) {
  const revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  const text = file => sources[file].toString('utf8');
  const [factPath, kernelPath] = revision.sourceClosure.inputs;
  const facts = text(factPath), kernel = text(kernelPath);
  assert.equal(kernel.split(revision.sourceClosure.removedImport).length, 2, 'the complete kernel retains its sole recorded fact import');
  const source = facts + '\n' + kernel.replace(revision.sourceClosure.removedImport, '');
  assert.equal(Buffer.byteLength(source), revision.sourceClosure.bytes);
  assert.equal(applicabilityDigest(source), revision.sourceClosure.sha256, 'the complete checked closure stays exact');
  const functions = [...source.matchAll(/^(?:export )?function ([A-Za-z_$][\w$]*)\(/gmu)].map(match => match[1]);
  assert.equal(functions.length, revision.sourceClosure.originalFunctions, 'all ten decisions, three exported readers and three private refusal helpers remain');
  const policy = readStructuredDataLino(text('js/src/translation/data/collection-construction-rules.lino'));
  assert.equal(text('rust/src/data/collection-construction-rules.lino'), text('js/src/translation/data/collection-construction-rules.lino'));
  assert.equal(policy.sourcePath, kernelPath);
  assert.equal(policy.source, kernel);
  assert.equal(policy.sourceBytes, Buffer.byteLength(kernel));
  assert.equal(policy.sourceSha256, applicabilityDigest(kernel));
  assert.deepEqual(policy.sourceClosure, {
    source, sourceSha256: revision.sourceClosure.sha256, bytes: revision.sourceClosure.bytes,
    removedImports: [revision.sourceClosure.removedImport],
    inputs: [factPath, kernelPath].map(path => ({ path, source: text(path), sourceSha256: applicabilityDigest(sources[path]), bytes: sources[path].length })),
  });
  const artifact = readStructuredDataLino(text('parity/self-translation/collection-construction-rules-native.lino'));
  assert.equal(artifact.model, policy.model);
  assert.deepEqual(artifact.sourceClosure, policy.sourceClosure);
  assert.equal(artifact.inventory.length, revision.sourceClosure.checkedOwners);
  assert.equal(artifact.checkedNamedSourceInventory.length, revision.sourceClosure.checkedOwners);
  assert.deepEqual(artifact.inventory.filter(row => row.kind === 'fn' && !row.generated).map(row => row.name), functions);
  assert.deepEqual(artifact.checkedNamedSourceInventory.map(({ fullName, modulePath, ...row }) => row), artifact.inventory);
  assert.equal(artifact.emittedTextBytes, Buffer.byteLength(artifact.emitted.text));
  assert.equal(artifact.emittedTextSha256, applicabilityDigest(artifact.emitted.text));
  assert.deepEqual(artifact.replays.map(row => row.format), ['text', 'binary']);
  for (const replay of artifact.replays) {
    assert.deepEqual(replay.checkedNamedSourceInventory, artifact.checkedNamedSourceInventory);
    assert.equal(replay.emittedTextBytes, artifact.emittedTextBytes);
    assert.equal(replay.emittedTextSha256, artifact.emittedTextSha256);
  }
  for (const owner of artifact.checkedNamedSourceInventory) {
    assert.equal(artifact.emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind)).length, 1);
  }
  const oracle = /const originalBranches=(\{[\s\S]*?^\});/mu.exec(text('js/tests/translation-collection-construction.test.js'));
  assert.ok(oracle, 'the complete retained original constructor oracle is present');
  assert.equal(JSON.parse(oracle[1]).infer, revision.originalBranches);
  return { bytes: Buffer.byteLength(source), sha256: applicabilityDigest(source),
    originalFunctions: functions.length, checkedOwners: artifact.checkedNamedSourceInventory.length,
    emittedTextBytes: artifact.emittedTextBytes, emittedTextSha256: artifact.emittedTextSha256,
    replayFormats: artifact.replays.map(row => row.format) };
}

/** Authenticate exact historical inputs or the one reviewed whole-source extraction. */
export function verifyOriginalLinkRawEmissionProducerApplicability(producerSources) {
  const revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  const inference=producerSources[revision.inference.file];
  if(Object.keys(producerSources).length!==originalProducerDescriptors.length&&Buffer.isBuffer(inference)
    &&applicabilityDigest(inference)!==revision.inference.sha256){
    const previous=Buffer.from(restoreTranslationSourceHistory(inference.toString('utf8'),revision.inference.file,
      '81107a5945651f17c962e4bb81fb560f96c65ccd'));
    const original=verifyOriginalLinkRawEmissionProducerApplicability({...producerSources,[revision.inference.file]:previous});
    const current={file:revision.inference.file,bytes:inference.length,sha256:applicabilityDigest(inference)};
    return freezeApplicability({...original,sources:original.sources.map(row=>row.file===current.file?current:row),
      sourceHistory:{revision:'81107a5945651f17c962e4bb81fb560f96c65ccd',previous:{...revision.inference},current}});
  }
  const originalFiles = originalProducerDescriptors.map(row => row.file);
  const suppliedFiles = Object.keys(producerSources);
  if (suppliedFiles.length === originalFiles.length) {
    assert.deepEqual(suppliedFiles, originalFiles, 'the original causal producer inventory stays exact');
    const sources = originalProducerDescriptors.map(row => verifyApplicabilitySource(producerSources, row));
    return freezeApplicability({ version: 1, revision: 'original causal inference producers',
      introducingCommit: '7bb7fe07df8a9de54dc4f987bc270cc44f09113d',
      causalProducers: sources, sources, inverse: null, sourceClosure: null });
  }
  assert.deepEqual(suppliedFiles, [...originalFiles, ...revision.dependencies.map(row => row.file)],
    'only the reviewed complete extraction input inventory is admitted');
  const currentDescriptors = [originalProducerDescriptors[0], revision.inference, ...revision.dependencies];
  let receiptRefresh;
  const sources = currentDescriptors.map(row => {
    const bytes = producerSources[row.file];
    if (row.file === ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH.current.file
      && Buffer.isBuffer(bytes) && applicabilityDigest(bytes) !== row.sha256) {
      const successor = verifyCollectionArchiveReceiptRefresh(producerSources, row);
      receiptRefresh = successor.receiptRefresh;
      return successor.source;
    }
    return verifyApplicabilitySource(producerSources, row);
  });
  const restored = restoreRawEmissionInferenceSource(producerSources[revision.inference.file].toString('utf8'));
  const original = originalProducerDescriptors[1];
  assert.equal(Buffer.byteLength(restored), original.bytes, 'the whole original inference source length is restored');
  assert.equal(applicabilityDigest(restored), original.sha256, 'the whole original inference source has the exact historical inverse');
  const sourceClosure = verifyCollectionApplicabilityClosure(producerSources);
  return freezeApplicability({ version: 1, revision: revision.revision, introducingCommit: revision.introducingCommit,
    causalProducers: originalProducerDescriptors.map(row => ({ ...row })), sources,
    inverse: { ...original }, sourceClosure, ...(receiptRefresh ? { receiptRefresh } : {}) });
}
