// Normal writer: complete JavaScript semantic bodies, both Links codecs, actual generated ABI.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { referenceOperation } from '../src/translation/reference-model.js';
const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/reference-identity-rules.js';
const decisions = ['acceptReferenceIdentityComparison', 'readReferenceIdentityComparisonError', 'renderReferenceIdentityComparison'];
const digest = value => createHash('sha256').update(value).digest('hex');
const cases = [
  { name: 'aliased arrays equal while distinct equal-content allocations differ',
    source: "function probe(){const array=[1];const alias=array;const other=[1];console.log(array===alias);console.log(array===other);console.log(array!==alias);return array!==other;}console.log(probe());",
    expected: 'true\nfalse\nfalse\ntrue\n' },
  { name: 'separate empty arrays retain distinct allocation identities',
    source: '/** @returns {number[]} */function empty(){return [];}console.log(empty()===empty());console.log(empty()!==empty());',
    expected: 'false\ntrue\n' },
  { name: 'actual comparison operands evaluate once in left to right order',
    source: "/** @returns {number[]} */function left(){console.log('left');return [1];}/** @returns {number[]} */function right(){console.log('right');return [1];}console.log(left()===right());console.log(left()!==right());",
    expected: 'left\nright\nfalse\nleft\nright\ntrue\n' },
  { name: 'record aliases and equal-content records retain distinct identities',
    source: 'function probe(){const value={count:1};const alias=value;const other={count:1};console.log(value===alias);return value!==other;}console.log(probe());',
    expected: 'true\ntrue\n' },
  { name: 'map aliases and equal-content maps retain distinct identities',
    source: "function probe(){const value=new Map([['key',1]]);const alias=value;const other=new Map([['key',1]]);console.log(value===alias);return value!==other;}console.log(probe());",
    expected: 'true\ntrue\n' },
  { name: 'set aliases and equal-content sets retain distinct identities',
    source: 'function probe(){const value=new Set([1]);const alias=value;const other=new Set([1]);console.log(value===alias);return value!==other;}console.log(probe());',
    expected: 'true\ntrue\n' },
];
const refusals = [
  { name: 'ordinary value arrays do not acquire reference identity', source: '/** @param {number[]} left @param {number[]} right */function probe(left,right){return left===right;}', options: {referenceSemantics: false} },
  { name: 'reference ordering is outside identity equality', source: '/** @param {number[]} left @param {number[]} right */function probe(left,right){return left<right;}', options: {referenceSemantics: true} },
  { name: 'nullable array equality keeps its separate refusal', source: '/** @param {number[]|null} left @param {number[]|null} right */function probe(left,right){return left===right;}', options: {referenceSemantics: true} },
  { name: 'different structural reference domains are not coerced', source: "function probe(){const left=[1];const right={count:1};return left===right;}", options: {referenceSemantics: true} },
  { name: 'different element domains are not coerced', source: "function probe(){return [1]===['1'];}", options: {referenceSemantics: true} },
];
export function referenceIdentityRulesPolicy() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.doesNotMatch(source, /^\s*import\b/mu, 'the entire semantic module is closed');
  assert.deepEqual([...source.matchAll(/^export function (\w+)\(/gmu)].map(row => row[1]), decisions);
  const operationPath = 'js/src/data/reference-operations.lino', operationSource = readFileSync(new URL(operationPath, root), 'utf8');
  const operation = referenceOperation('compare-reference-identity');
  assert.equal(operation.kind, 'reference'); assert.equal(operation.mutates, false);
  assert.equal(operation.primitive, 'compare-reference-identity');
  assert.deepEqual(operation.program, [['compare-identities', 'receiver', 'arguments', 'result'], ['return-value', 'result']]);
  return {schemaVersion: 1, model: 'same-domain structural reference identity equality 1', sourcePath, decisions,
    sourceClosure: {source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), inputs: [{path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source)}]},
    linkedOperation: {path: operationPath, source: operationSource, bytes: Buffer.byteLength(operationSource), sha256: digest(operationSource),
      id: operation.id, kind: operation.kind, primitive: operation.primitive, mutates: operation.mutates, program: operation.program},
    cases, refusals, assertionVersions: {Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0'},
    representation: 'Existing Binary with Boolean result and the complete matching reference-qualified operand domain. No new expression kind, type kind, schema field or source replacement is introduced. JavaScript keeps existing strict equality/inequality; Rust compares the existing Rc<RefCell<T>> allocation identities without reading either payload.',
    admission: 'JavaScript source-language eq/ne on two actual reference-qualified arrays, records, maps or sets with identical complete checked types. Cross-kind/element domains, optional references, reference ordering and ordinary value-array comparisons retain independent refusals.',
    validation: 'Source-free binary validation rejects missing/changed reference domains, incorrect result/operand types and unsupported operations. Both semantic codecs recheck the actual original program and preserve complete emitted output.',
    boundary: 'Declarations and source preservation only. Native compilation, executed runtime parity and emitted-Rust parsing remain independent CI obligations; this profile does not claim a general mutable collection runtime.'};
}
export async function generateReferenceIdentityRules({check = false, native = false} = {}) {
  const policy = referenceIdentityRulesPolicy();
  for (const path of ['js/src/translation/data/reference-identity-rules.lino', 'rust/src/data/reference-identity-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy); else writeProjectData(new URL(path, root), policy);
  }
  let artifact = null;
  if (native) {
    const {parseJavaScript} = await import('../src/translation/javascript.js');
    const {checkProgram} = await import('../src/translation/check.js');
    const {emitRust} = await import('../src/translation/emit-rust.js');
    const {encodeTypedProgramLinks, decodeTypedProgramLinks} = await import('../src/typed-program-links.js');
    const {recheckLinkedTypedProgram} = await import('../src/typed-program-checked-projection.js');
    const {LinkNetwork} = await import('../src/network.js');
    const {encodeLinkNetworkBinary, decodeLinkNetworkBinary} = await import('../src/link-network-binary.js');
    const surface = parseJavaScript(policy.sourceClosure.source, {referenceSemantics: false});
    const inventory = surface.items.map((item, ordinal) => ({ordinal, name: item.name, kind: item.k, generated: item.generated === true}));
    assert.deepEqual(inventory.filter(row => !row.generated).map(row => row.name), decisions);
    const owners = program => inventory.map(row => {const declaration = program.declarations.get(row.name); assert.ok(declaration && declaration.k === row.kind); return {...row, fullName: declaration.fullName};});
    const checked = checkProgram(surface), checkedNamedSourceInventory = owners(checked); assert.equal(checkedNamedSourceInventory.length, checked.declarations.size);
    const emitted = emitRust(checked, {arrayConsumption: true}), packet = encodeTypedProgramLinks(checked), replays = [];
    const programText = packet.network.toLino(), programPaths = ['js/src/data/reference-identity-program.lino', 'rust/src/data/reference-identity-program.lino'];
    for (const path of programPaths) {if (check) assert.equal(readFileSync(new URL(path, root), 'utf8'), programText); else writeFileSync(new URL(path, root), programText);}
    for (const [format, network] of [['text', LinkNetwork.fromLino(programText)], ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram: true}); assert.deepEqual(owners(restored), checkedNamedSourceInventory);
      const replay = emitRust(restored, {arrayConsumption: true});
      for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) assert.deepEqual(replay[field], emitted[field]);
      replays.push({format, checkedNamedSourceInventory: owners(restored), emittedTextSha256: digest(replay.text)});
    }
    for (const owner of checkedNamedSourceInventory) assert.equal(emitted.mappings.filter(row => row.source === owner.fullName && row.kind === 'function').length, 1);
    const files = writeRecordRuleLibrary(emitted, 'reference_identity_rules', {check}), libraryInterface = [];
    for (const file of files) {
      const tokens = tokenize(file.source, 'Rust').tokens;
      for (let at = 0; at < tokens.length; at += 1) {
        if (tokens[at].value !== 'fn' || tokens[at + 1]?.kind !== 'identifier') continue;
        const target = tokens[at + 1].value, mapping = emitted.mappings.find(row => row.target === target && row.kind === 'function'); if (!mapping) continue;
        let body = at + 2; while (body < tokens.length && tokens[body].value !== '{') body += 1; assert.ok(body < tokens.length);
        libraryInterface.push({source: mapping.source, target, path: file.path ?? 'reference_identity_rules.rs', signature: file.source.slice(tokens[at].start, tokens[body].start).trim()});
      }
    }
    for (const name of decisions) assert.equal(libraryInterface.filter(row => row.source === name).length, 1);
    artifact = {schemaVersion: 1, model: policy.model, sourceClosure: policy.sourceClosure, linkedOperation: policy.linkedOperation,
      inventory, checkedNamedSourceInventory, replays, libraryInterface,
      archive: {root: packet.root.asU64(), paths: programPaths, bytes: Buffer.byteLength(programText), sha256: digest(programText)},
      emitted: {text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes, mappings: emitted.mappings, assumptions: emitted.assumptions, encodings: emitted.encodings},
      files: files.map(file => ({path: file.path ?? 'reference_identity_rules.rs', source: file.source, bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source)})),
      boundary: 'Normal actual source closure and generated ABI only. Native CI remains mandatory.'};
    const output = new URL('parity/self-translation/reference-identity-rules-native.lino', root); if (check) checkProjectData(output, artifact); else writeProjectData(output, artifact);
  }
  return {policy, artifact};
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2); assert.ok(new Set(args).size === args.length && args.every(value => ['--check', '--native'].includes(value)));
  await generateReferenceIdentityRules({check: args.includes('--check'), native: args.includes('--native')});
}
