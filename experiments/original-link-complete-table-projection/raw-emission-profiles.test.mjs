// Bounded provenance and primitive execution controls. No native planner,
// compiler, complete-source CST archive or shared fixture corpus is run here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../../js/scripts/project-data.mjs';
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
  ORIGINAL_LINK_RAW_EMISSION_BASELINE, ORIGINAL_LINK_RAW_EMISSION_CHANGES,
  readOriginalLinkRawEmissionProducerSources, readOriginalLinkRawEmissionProfile,
  verifyOriginalLinkRawEmissionProfile,
} from './raw-emission-profiles.mjs';

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
