// Bounded provenance and primitive execution controls. No native planner,
// compiler, complete-source CST archive or shared fixture corpus is run here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../../js/src/structured-data-links.js';
import { LinkNetwork } from '../../js/src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../../js/src/link-network-binary.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../../js/src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../../js/src/typed-program-checked-projection.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import {
  ORIGINAL_LINK_RAW_CURRENT_ARTIFACT, ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT,
  ORIGINAL_LINK_RAW_EMISSION_BASELINE, ORIGINAL_LINK_RAW_EMISSION_CHANGES, ORIGINAL_LINK_RAW_EMISSION_PRODUCERS,
  readOriginalLinkRawEmissionProducerSources, readOriginalLinkRawEmissionProfile,
  verifyOriginalLinkRawEmissionProfile,
} from './raw-emission-profiles.mjs';
import {
  ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY,
  ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH,
  verifyOriginalLinkRawEmissionProducerApplicability,
} from './raw-emission-producer-applicability.mjs';

const rawEmissionDigest = value => createHash('sha256').update(value).digest('hex');
let originalRawEmissionInputs;
function rawEmissionFixture() {
  originalRawEmissionInputs ??= {
    historicalArtifact: readProjectData(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT),
    currentArtifact: readProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT),
    source: readFileSync(new URL('./raw-rules.mjs', import.meta.url), 'utf8'),
    producerSources: readOriginalLinkRawEmissionProducerSources(),
  };
  return {
    historicalArtifact: structuredClone(originalRawEmissionInputs.historicalArtifact),
    currentArtifact: structuredClone(originalRawEmissionInputs.currentArtifact),
    source: originalRawEmissionInputs.source,
    producerSources: Object.fromEntries(Object.entries(originalRawEmissionInputs.producerSources)
      .map(([file, bytes]) => [file, Buffer.from(bytes)])),
  };
}

function reviseRawEmissionSourceSummary(input) {
  input.currentArtifact.profile.currentSourceBytes = Buffer.byteLength(input.currentArtifact.completeRust);
  input.currentArtifact.profile.currentSourceSha256 = rawEmissionDigest(input.currentArtifact.completeRust);
}

function replaceRawEmissionDefinition(input, index, rewrite) {
  const previous = input.currentArtifact.declarations[index];
  const next = rewrite(previous);
  assert.notEqual(next, previous, 'the adversarial edit must actually change its selected owner');
  assert.equal(input.currentArtifact.completeRust.split(previous).length, 2);
  input.currentArtifact.declarations[index] = next;
  input.currentArtifact.completeRust = input.currentArtifact.completeRust.replace(previous, () => next);
  reviseRawEmissionSourceSummary(input);
}

test('historical and current raw profiles retain the complete original artifact and explicit causal applicability', () => {
  const input = rawEmissionFixture();
  const proof = verifyOriginalLinkRawEmissionProfile(input);
  assert.equal(proof.profile, 'current');
  assert.equal(proof.introducingCommit, '7bb7fe07df8a9de54dc4f987bc270cc44f09113d');
  assert.equal(proof.definitionCount, 106);
  assert.equal(proof.unchangedDefinitions, 103);
  assert.equal(proof.changedLines, 5);
  assert.deepEqual(proof.changedOwners, ['checkOriginalLinkRawShape',
    'ml_checkOriginalLinkRawShape_loop54', 'ml_checkOriginalLinkRawShape_loop48']);
  const current = readOriginalLinkRawEmissionProfile(undefined, input);
  const historical = readOriginalLinkRawEmissionProfile('historical', input);
  assert.equal(current.profile, 'current');
  assert.equal(historical.profile, 'historical');
  for (const [profile, artifact, bytes, digest] of [
    [current, input.currentArtifact, 351680, '54388dca2da4b7f1a5bcbb7308d31d5f049997898b89a4bf940b2f7bb664aec5'],
    [historical, input.historicalArtifact, 351746, '24db774177e047de0c0ea22adb2464e4e8481d03547f5a810b2d9fe317fb3046'],
  ]) {
    assert.equal(profile.source, artifact.completeRust);
    assert.equal(profile.sourceBytes, bytes);
    assert.equal(Buffer.byteLength(profile.source), bytes);
    assert.equal(profile.sourceSha256, digest);
    assert.equal(rawEmissionDigest(profile.source), digest);
    assert.deepEqual(profile.declarationNames, input.historicalArtifact.declarationNames);
    assert.equal(profile.definitionCount, 106);
    assert.equal(profile.source.split(ORIGINAL_LINK_RAW_EMISSION_BASELINE.rootHeader).length, 2);
    assert.ok(Object.isFrozen(profile));
  }
  assert.equal(Buffer.byteLength(input.source), 40766);
  assert.equal(rawEmissionDigest(input.source), 'e4bafbec028815e973f2d3c0c683db6347d77d60b64548845f7669d01a0a35c5');
  assert.equal(input.source.split('const facetCode=fieldCodes[0]??-1;').length, 2);
  for (const name of ['', 'latest', 'CURRENT', null, 0]) {
    assert.throws(() => readOriginalLinkRawEmissionProfile(name, input), /unknown raw emission profile/u);
  }
});

test('the five raw emission line changes restore all historical bytes and leave the other 103 owners unchanged', () => {
  const { historicalArtifact, currentArtifact } = rawEmissionFixture();
  assert.deepEqual(currentArtifact.declarationNames, historicalArtifact.declarationNames);
  assert.equal(currentArtifact.declarations.length, 106);
  const changed = currentArtifact.declarations.flatMap((definition, index) =>
    definition === historicalArtifact.declarations[index] ? [] : [index]);
  assert.deepEqual(changed, [20, 91, 92]);
  assert.equal(106 - changed.length, 103);
  let forward = historicalArtifact.completeRust;
  let inverse = currentArtifact.completeRust;
  for (const change of ORIGINAL_LINK_RAW_EMISSION_CHANGES) {
    assert.equal(historicalArtifact.declarationNames[change.declarationIndex], change.owner);
    assert.equal(forward.split(change.from).length, 2);
    forward = forward.replace(change.from, () => change.to);
  }
  for (const change of [...ORIGINAL_LINK_RAW_EMISSION_CHANGES].reverse()) {
    assert.equal(inverse.split(change.to).length, 2);
    inverse = inverse.replace(change.to, () => change.from);
  }
  assert.equal(forward, currentArtifact.completeRust);
  assert.equal(inverse, historicalArtifact.completeRust);
  for (let index = 0; index < 106; index += 1) {
    if (!changed.includes(index)) assert.equal(currentArtifact.declarations[index], historicalArtifact.declarations[index]);
  }
});

test('coherent raw emission hashes cannot authorize body owner ABI source or profile substitutions', () => {
  const wrapper = ORIGINAL_LINK_RAW_EMISSION_CHANGES[0];
  const mutations = [
    ['unaffected owner', input => replaceRawEmissionDefinition(input, 0, text => text + '\n// coherent unrelated owner edit')],
    ['historical wrapper on current ABI', input => replaceRawEmissionDefinition(input, 20,
      text => text.replace(wrapper.to, () => wrapper.from))],
    ['optional parameter under current profile', input => replaceRawEmissionDefinition(input, 91,
      text => text.replace('mut facet_code: f64,', 'mut facet_code: Option<f64>,'))],
    ['comparison operation', input => replaceRawEmissionDefinition(input, 91,
      text => text.replace('), facet_code) { (Some(ml_left), ml_right) => ml_left == ml_right,',
        '), facet_code) { (Some(ml_left), ml_right) => ml_left != ml_right,'))],
    ['public root ABI', input => replaceRawEmissionDefinition(input, 20,
      text => text.replace('root_high: f64,', 'root_high: Option<f64>,'))],
    ['unowned source suffix', input => {
      input.currentArtifact.completeRust += '\nfn counterfeit_extra_owner() {}\n';
      reviseRawEmissionSourceSummary(input);
    }],
    ['declaration order', input => {
      [input.currentArtifact.declarationNames[0], input.currentArtifact.declarationNames[1]] =
        [input.currentArtifact.declarationNames[1], input.currentArtifact.declarationNames[0]];
      [input.currentArtifact.declarations[0], input.currentArtifact.declarations[1]] =
        [input.currentArtifact.declarations[1], input.currentArtifact.declarations[0]];
    }],
    ['missing owner', input => { input.currentArtifact.declarationNames.pop(); input.currentArtifact.declarations.pop(); }],
    ['extra owner', input => { input.currentArtifact.declarationNames.push('counterfeit'); input.currentArtifact.declarations.push('pub fn counterfeit() {}'); }],
    ['unregistered artifact field', input => { input.currentArtifact.unownedProof = true; }],
    ['historical source', input => {
      input.historicalArtifact.completeRust += '\n';
      input.currentArtifact.profile.historicalSourceBytes += 1;
      input.currentArtifact.profile.historicalSourceSha256 = rawEmissionDigest(input.historicalArtifact.completeRust);
    }],
    ['original coalescing premise', input => {
      input.source = input.source.replace('const facetCode=fieldCodes[0]??-1;', 'const facetCode=fieldCodes[0]??0;');
      input.currentArtifact.sourceSha256 = rawEmissionDigest(input.source);
      input.currentArtifact.profile.concreteFacetPremise = 'const facetCode=fieldCodes[0]??0;';
    }],
    ['causal producer bytes and pin', input => {
      const [file] = Object.keys(input.producerSources);
      input.producerSources[file] = Buffer.concat([input.producerSources[file], Buffer.from('\n')]);
      const row = input.currentArtifact.profile.producerSources.find(row => row.file === file);
      row.bytes = input.producerSources[file].length;
      row.sha256 = rawEmissionDigest(input.producerSources[file]);
    }],
    ['causal producer order', input => { input.producerSources = Object.fromEntries(Object.entries(input.producerSources).reverse()); }],
    ['interpretation', input => { input.currentArtifact.profile.interpretation = 'unregistered equality inference'; }],
    ['introducing source revision', input => { input.currentArtifact.profile.introducingCommit = '0'.repeat(40); }],
    ['native claim', input => { input.currentArtifact.profile.scope = 'native execution succeeded'; }],
  ];
  for (const [label, mutate] of mutations) {
    const input = rawEmissionFixture();
    mutate(input);
    assert.throws(() => verifyOriginalLinkRawEmissionProfile(input), assert.AssertionError, label);
  }
});

test('a coalesced Number captured by a lifted loop preserves absent reads NaN and signed zero through both typed archives', () => {
  const source = '/** @param {number[]} parents @param {number[]} labels @param {number[][]} policy @returns {number} */'
    + 'function inspect(parents,labels,policy){const fieldCodes=policy[11]??[];const facetCode=fieldCodes[0]??-1;'
    + 'let count=0;for(let index=0;index<parents.length;index=index+1)if(parents[index]===7&&labels[index]===facetCode)count=count+1;return count;}';
  const policy = value => { const rows = Array.from({ length: 12 }, () => []); rows[11] = [value]; return rows; };
  const cases = [
    { name: 'absent read differs from the concrete fallback', args: [[7], [], []], expected: 0 },
    { name: 'the actual sentinel matches', args: [[7], [-1], []], expected: 1 },
    { name: 'a present selected value matches', args: [[7], [3], policy(3)], expected: 1 },
    { name: 'a distinct selected value differs', args: [[7], [4], policy(3)], expected: 0 },
    { name: 'the original parent guard remains', args: [[8], [3], policy(3)], expected: 0 },
    { name: 'short labels never create absent-equals-absent matches', args: [[7, 7, 7], [3], policy(3)], expected: 1 },
    { name: 'NaN remains unequal to NaN', args: [[7], [NaN], policy(NaN)], expected: 0 },
    { name: 'negative zero equals positive zero', args: [[7], [-0], policy(0)], expected: 1 },
    { name: 'positive zero equals negative zero', args: [[7], [0], policy(-0)], expected: 1 },
  ];
  const observe = text => {
    const inspect = vm.runInNewContext(text + '\ninspect;', {}, { timeout: 1000 });
    for (const { name, args, expected } of cases) {
      const before = structuredClone(args);
      assert.equal(inspect(...args), expected, name);
      assert.deepEqual(args, before, 'all actual argument values and Number bits remain unchanged: ' + name);
    }
  };
  observe(source);
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const captures = checked.items.flatMap(item => item.params?.filter(param => param.name === 'facetCode') ?? []);
  assert.ok(captures.length > 0, 'the actual lowering must exercise a captured facet in a generated loop');
  assert.ok(captures.every(param => param.type.kind === 'float'), 'the actual concrete caller constrains every lifted facet parameter');
  const javascript = emitJavaScript(checked).text;
  const rust = emitRust(checked).text;
  observe(javascript);
  const packet = encodeTypedProgramLinks(checked);
  const forms = [LinkNetwork.fromLino(packet.network.toLino()),
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
  for (const network of forms) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(emitJavaScript(restored).text, javascript);
    assert.equal(emitRust(restored).text, rust);
    observe(emitJavaScript(restored).text);
  }
});

function originalRawProducerSnapshots(input) {
  const revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  const current = input.producerSources[revision.inference.file].toString('utf8');
  assert.equal(current.split(revision.addedImport).length, 2);
  assert.equal(current.split(revision.currentBranches).length, 2);
  const original = current.replace(revision.addedImport, '').replace(revision.currentBranches, () => revision.originalBranches);
  return Object.fromEntries(ORIGINAL_LINK_RAW_EMISSION_PRODUCERS.map(row => [row.file,
    row.file === revision.inference.file ? Buffer.from(original) : Buffer.from(input.producerSources[row.file])]));
}

test('the collection extraction restores the complete original producer without changing either raw emission artifact', () => {
  const input = rawEmissionFixture(), revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  const before = structuredClone(input.currentArtifact.profile);
  const proof = verifyOriginalLinkRawEmissionProfile(input);
  assert.equal(proof.producerApplicability.revision, 'collection construction extraction 1');
  assert.equal(proof.producerApplicability.introducingCommit, 'b51d48140220bf2ac071334d79d28aa77273c00e');
  assert.deepEqual(proof.producerApplicability.causalProducers, ORIGINAL_LINK_RAW_EMISSION_PRODUCERS);
  assert.deepEqual(proof.producerApplicability.sources.map(row => row.file), Object.keys(input.producerSources));
  assert.equal(proof.producerApplicability.sources.length, 14);
  assert.deepEqual(proof.producerApplicability.inverse, ORIGINAL_LINK_RAW_EMISSION_PRODUCERS[1]);
  const historicalSources = originalRawProducerSnapshots(input);
  for (const descriptor of ORIGINAL_LINK_RAW_EMISSION_PRODUCERS) {
    assert.equal(historicalSources[descriptor.file].length, descriptor.bytes);
    assert.equal(rawEmissionDigest(historicalSources[descriptor.file]), descriptor.sha256);
  }
  const historicalInputs = { ...input, producerSources: historicalSources };
  const historicalProof = verifyOriginalLinkRawEmissionProfile(historicalInputs);
  assert.equal(historicalProof.producerApplicability.revision, 'original causal inference producers');
  assert.equal(historicalProof.producerApplicability.sources.length, 2);
  assert.equal(historicalProof.producerApplicability.inverse, null);
  assert.equal(historicalProof.producerApplicability.sourceClosure, null);
  for (const profile of ['historical', 'current']) {
    const actual = readOriginalLinkRawEmissionProfile(profile, input);
    const original = readOriginalLinkRawEmissionProfile(profile, historicalInputs);
    assert.equal(actual.source, original.source);
    assert.equal(actual.sourceBytes, original.sourceBytes);
    assert.equal(actual.sourceSha256, original.sourceSha256);
    assert.deepEqual(actual.declarationNames, original.declarationNames);
    assert.deepEqual(actual.proof.producerSources, original.proof.producerSources);
  }
  assert.deepEqual(input.currentArtifact.profile, before);
  assert.deepEqual(input.currentArtifact.profile, readProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT).profile);
  assert.equal(rawEmissionDigest(readFileSync(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT)), 'd4b2a0c42bba5ae8f29137ead5bc264b633fcb6fad953b13d2adb5fed2d9d6cd');
  assert.equal(rawEmissionDigest(readFileSync(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT)), '7d77b7c1261c4125a9bce247c43bab75a23262e2a8c5d849d4e9f10951c51f6b');
  assert.ok(Object.isFrozen(proof.producerApplicability));
  assert.ok(Object.isFrozen(revision.dependencies));
});

test('the complete collection applicability closure retains sixteen functions eighteen checked owners and both source free replays', () => {
  const input = rawEmissionFixture(), revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  const proof = verifyOriginalLinkRawEmissionProducerApplicability(input.producerSources);
  const text = file => input.producerSources[file].toString('utf8');
  const [factPath, kernelPath] = revision.sourceClosure.inputs;
  const source = text(factPath) + '\n' + text(kernelPath).replace(revision.sourceClosure.removedImport, '');
  assert.equal(Buffer.byteLength(source), 10257);
  assert.equal(rawEmissionDigest(source), '53a5bb4cdeec63423be142da305f494d0bb4a669bbbf5025b7b7ab6f34fc86f3');
  const names = [...source.matchAll(/^(?:export )?function ([A-Za-z_$][\w$]*)\(/gmu)].map(match => match[1]);
  assert.deepEqual(names, ['refuseRequiredNumberFact', 'refuseRequiredTextFact', 'refuseRequiredBooleanFact',
    'readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact',
    'readCollectionConstructionField', 'readCollectionConstructionElement', 'readCollectionConstructionKind',
    'planCollectionConstructionInference', 'planCollectionConstructionChecked', 'readCollectionConstructionKeyAdmission',
    'readCollectionConstructionDiagnostic', 'readCollectionConstructionBinding', 'renderCollectionConstructionType', 'renderCollectionConstruction']);
  assert.equal(proof.sourceClosure.originalFunctions, 16);
  assert.equal(proof.sourceClosure.checkedOwners, 18);
  const policy = readProjectData(new URL('../../js/src/translation/data/collection-construction-rules.lino', import.meta.url));
  const native = readProjectData(new URL('../../parity/self-translation/collection-construction-rules-native.lino', import.meta.url));
  assert.equal(policy.sourceClosure.source, source);
  assert.deepEqual(native.sourceClosure, policy.sourceClosure);
  assert.deepEqual(native.inventory.filter(row => row.kind === 'fn' && !row.generated).map(row => row.name), names);
  assert.equal(native.inventory.length, 18);
  assert.equal(new Set(native.checkedNamedSourceInventory.map(row => row.fullName)).size, 18);
  assert.deepEqual(native.replays.map(row => row.format), ['text', 'binary']);
  for (const replay of native.replays) {
    assert.deepEqual(replay.checkedNamedSourceInventory, native.checkedNamedSourceInventory);
    assert.equal(replay.emittedTextBytes, Buffer.byteLength(native.emitted.text));
    assert.equal(replay.emittedTextSha256, rawEmissionDigest(native.emitted.text));
  }
  assert.deepEqual(proof.sourceClosure.replayFormats, ['text', 'binary']);
  assert.equal(proof.sourceClosure.emittedTextSha256, rawEmissionDigest(native.emitted.text));
  assert.equal(text('js/src/translation/data/collection-construction-rules.lino'), text('rust/src/data/collection-construction-rules.lino'));
});

test('authored extraction applicability refuses altered bodies incomplete dependencies and coherent caller supplied pins', () => {
  const revision = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY;
  const mutations = [
    ['extraction import', input => { input.producerSources[revision.inference.file] = Buffer.from(input.producerSources[revision.inference.file].toString('utf8')
      .replace(revision.addedImport, revision.addedImport.replace('inferCollectionConstruction }', 'inferCollectionConstruction as renamed }'))); }],
    ['constructor dispatch', input => { input.producerSources[revision.inference.file] = Buffer.from(input.producerSources[revision.inference.file].toString('utf8')
      .replace('inferCollectionConstruction(this, node, env, expected)', 'inferCollectionConstruction(this, node, env, undefined)')); }],
    ['unrelated inference source', input => { input.producerSources[revision.inference.file] = Buffer.concat([input.producerSources[revision.inference.file], Buffer.from('\n')]); }],
    ['missing dependency', input => { delete input.producerSources[revision.dependencies[0].file]; }],
    ['extra dependency', input => { input.producerSources['unregistered.js'] = Buffer.from(''); }],
    ['dependency order', input => { const rows = Object.entries(input.producerSources); [rows[2], rows[3]] = [rows[3], rows[2]]; input.producerSources = Object.fromEntries(rows); }],
    ['current source without its complete closure', input => { input.producerSources = Object.fromEntries(ORIGINAL_LINK_RAW_EMISSION_PRODUCERS.map(row => [row.file, input.producerSources[row.file]])); }],
    ['old source under the new closure', input => { input.producerSources[revision.inference.file] = originalRawProducerSnapshots(input)[revision.inference.file]; }],
    ['causal profile repin', input => { const descriptor = input.currentArtifact.profile.producerSources[1];
      descriptor.bytes = input.producerSources[descriptor.file].length; descriptor.sha256 = rawEmissionDigest(input.producerSources[descriptor.file]); }],
    ['caller applicability authority', input => { input.currentArtifact.producerApplicability = structuredClone(revision); }],
  ];
  for (const descriptor of revision.dependencies) {
    mutations.push(['changed complete dependency ' + descriptor.file, input => {
      input.producerSources[descriptor.file] = Buffer.concat([input.producerSources[descriptor.file], Buffer.from('\n')]);
      input.currentArtifact.profile.producerSources.push({ file: descriptor.file, bytes: input.producerSources[descriptor.file].length,
        sha256: rawEmissionDigest(input.producerSources[descriptor.file]) });
    }]);
  }
  for (const [label, mutate] of mutations) {
    const input = rawEmissionFixture();
    mutate(input);
    assert.throws(() => verifyOriginalLinkRawEmissionProfile(input), assert.AssertionError, label);
  }
  const input = rawEmissionFixture(), original = originalRawProducerSnapshots(input);
  original[ORIGINAL_LINK_RAW_EMISSION_PRODUCERS[1].file][0] ^= 1;
  assert.throws(() => verifyOriginalLinkRawEmissionProducerApplicability(original), assert.AssertionError);
});

function originalRawArchiveReceiptInput(input) {
  const refresh = ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH;
  const inverse = readStructuredDataLino(input.producerSources[refresh.current.file].toString('utf8'));
  for (const change of refresh.changes) {
    assert.equal(inverse.archive[change.field], change.after);
    inverse.archive[change.field] = change.before;
  }
  const bytes = Buffer.from(structuredDataToLino(inverse));
  assert.equal(bytes.length, refresh.previous.bytes);
  assert.equal(rawEmissionDigest(bytes), refresh.previous.sha256,
    'the complete original canonical artifact is reconstructed, not a selected metadata projection');
  return { ...input, producerSources: Object.fromEntries(Object.entries(input.producerSources)
    .map(([file, source]) => [file, file === refresh.previous.file ? bytes : Buffer.from(source)])) };
}

test('the archive receipt successor retains every original producer pin and complete raw emission artifact', () => {
  const current = rawEmissionFixture(), refresh = ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH;
  const historical = originalRawArchiveReceiptInput(current);
  const before = verifyOriginalLinkRawEmissionProfile(historical);
  const after = verifyOriginalLinkRawEmissionProfile(current);
  assert.equal(before.producerApplicability.sources.length, 14);
  assert.equal(after.producerApplicability.sources.length, 14);
  assert.equal(Object.hasOwn(before.producerApplicability, 'receiptRefresh'), false);
  assert.deepEqual(after.producerApplicability.receiptRefresh, refresh);
  assert.ok(Object.isFrozen(after.producerApplicability.receiptRefresh));
  const oldDescriptor = ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY.dependencies
    .find(row => row.file === refresh.previous.file);
  assert.deepEqual(oldDescriptor, refresh.previous, 'the exact original source pin is unchanged');
  assert.deepEqual(before.producerApplicability.sources, [ORIGINAL_LINK_RAW_EMISSION_PRODUCERS[0],
    ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY.inference,
    ...ORIGINAL_LINK_RAW_COLLECTION_EXTRACTION_APPLICABILITY.dependencies]);
  assert.deepEqual(after.producerApplicability.sources, before.producerApplicability.sources.map(row =>
    row.file === refresh.current.file ? refresh.current : row), 'actual current bytes have their actual descriptor');
  assert.deepEqual(before.producerApplicability.sourceClosure, after.producerApplicability.sourceClosure);
  assert.deepEqual(before.producerApplicability.causalProducers, after.producerApplicability.causalProducers);
  assert.deepEqual(before.producerApplicability.inverse, after.producerApplicability.inverse);
  assert.deepEqual(current.currentArtifact, historical.currentArtifact);
  assert.deepEqual(current.historicalArtifact, historical.historicalArtifact);
  for (const profile of ['historical', 'current']) {
    const original = readOriginalLinkRawEmissionProfile(profile, historical);
    const successor = readOriginalLinkRawEmissionProfile(profile, current);
    for (const field of ['source', 'sourceBytes', 'sourceSha256', 'declarationNames', 'definitionCount']) {
      assert.deepEqual(successor[field], original[field], profile + ': complete emission and ABI remain exact');
    }
  }
  const originalCausal = { ...current, producerSources: originalRawProducerSnapshots(current) };
  assert.equal(verifyOriginalLinkRawEmissionProfile(originalCausal).producerApplicability.sources.length, 2,
    'the entire original two-producer branch remains admitted');
});

test('archive receipt successors refuse unknown digests changed owners third fields and caller repins', () => {
  const refresh = ORIGINAL_LINK_RAW_COLLECTION_ARCHIVE_RECEIPT_REFRESH;
  for (const mutate of [
    artifact => { artifact.archive.textSha256 = artifact.archive.textSha256 === refresh.changes[0].before ? refresh.changes[0].after : refresh.changes[0].before; },
    artifact => { artifact.archive.binarySha256 = '0'.repeat(64); },
    artifact => { artifact.archive.textBytes += 1; },
    artifact => { artifact.emitted.text += '\n'; },
    artifact => { artifact.inventory[0].name += '_changed'; },
    artifact => { artifact.replays[0].emittedTextSha256 = '0'.repeat(64); },
  ]) {
    for (const original of [false, true]) {
      const input = original ? originalRawArchiveReceiptInput(rawEmissionFixture()) : rawEmissionFixture();
      const artifact = readStructuredDataLino(input.producerSources[refresh.current.file].toString('utf8'));
      mutate(artifact);
      const bytes = Buffer.from(structuredDataToLino(artifact));
      assert.notEqual(rawEmissionDigest(bytes), rawEmissionDigest(input.producerSources[refresh.current.file]));
      input.producerSources[refresh.current.file] = bytes;
      input.currentArtifact.profile.producerSources.push({ file: refresh.current.file,
        bytes: bytes.length, sha256: rawEmissionDigest(bytes) });
      assert.throws(() => verifyOriginalLinkRawEmissionProducerApplicability(input.producerSources), assert.AssertionError);
      assert.throws(() => verifyOriginalLinkRawEmissionProfile(input), assert.AssertionError);
    }
  }
  for (const mutate of [
    input => { delete input.producerSources[refresh.current.file]; },
    input => { input.producerSources = Object.fromEntries(Object.entries(input.producerSources).reverse()); },
    input => { input.currentArtifact.receiptRefresh = structuredClone(refresh); },
    input => { input.producerSources[refresh.current.file] = Buffer.concat([
      input.producerSources[refresh.current.file], Buffer.from('\n')]); },
  ]) {
    const input = rawEmissionFixture();
    mutate(input);
    assert.throws(() => verifyOriginalLinkRawEmissionProfile(input), assert.AssertionError);
  }
});
