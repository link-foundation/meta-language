// Generate shared policy data first. Translate the whole decision module only
// after the JavaScript batch passes; native compilation belongs to CI.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {RUST_DIAGNOSTIC_ATTRIBUTE_POLICY, readRustDiagnosticAttributeRuleSource} from './rust-diagnostic-attribute-policy.mjs';
import {checkProjectData, writeProjectData} from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/rust-diagnostic-attribute-rules.js';
const digest = source => createHash('sha256').update(source).digest('hex');
const decisions = ['acceptRustDiagnosticAttributeOffset', 'planRustDiagnosticAttribute',
  'readRustMustUseAttributeEnd', 'planRustDiagnosticAttributePrefix'];

function declarationInventory(program) {
  return [...program.declarations.values()].map((entry, ordinal) => ({ordinal, kind: entry.k,
    name: entry.name, fullName: entry.fullName, modulePath: entry.modulePath}));
}

function ownerMappings(emission, owners) {
  return owners.map(owner => {
    const found = emission.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(found.length, 1, 'one actual complete emitted owner for ' + owner.fullName);
    return found[0];
  });
}

/** The emitter deliberately keeps these complete parts non-enumerable. Make
 * only their data descriptors explicit for the authenticated source artifact. */
export function readRustDiagnosticAttributeEmissionData(emission) {
  return {...emission, definitions: emission.definitions, preludes: emission.preludes};
}

/** No writes, formatter or compiler. Both returned whole-program rechecks feed
 * emission; neither decoded predecessor is used as a checked result. */
export async function prepareRustDiagnosticAttributeTranslation(source = readFileSync(new URL(sourcePath, root), 'utf8')) {
  assert.equal(source, readRustDiagnosticAttributeRuleSource(), 'the entire actual module follows the shared token policy');
  assert.deepEqual([...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]), decisions);
  const {parseJavaScript} = await import('../src/translation/javascript.js');
  const {checkProgram} = await import('../src/translation/check.js');
  const {emitRust} = await import('../src/translation/emit-rust.js');
  const {emitJavaScript} = await import('../src/translation/emit-javascript.js');
  const {encodeTypedProgramLinks, decodeTypedProgramLinks} = await import('../src/typed-program-links.js');
  const {recheckLinkedTypedProgram} = await import('../src/typed-program-checked-projection.js');
  const {LinkNetwork} = await import('../src/network.js');
  const {encodeLinkNetworkBinary, decodeLinkNetworkBinary} = await import('../src/link-network-binary.js');
  const surface = parseJavaScript(source, {referenceSemantics: false}), checked = checkProgram(surface);
  const parsedInventory = surface.items.map((item, ordinal) => {
    assert.ok(['fn', 'data'].includes(item.k), 'classify every actual source and lowering declaration');
    return {ordinal, kind: item.k, name: item.name, span: item.span};
  });
  const owners = declarationInventory(checked), sourceOwners = decisions.map(name => {
    const selected = owners.filter(owner => owner.kind === 'fn' && owner.fullName === name && owner.name === name);
    assert.equal(selected.length, 1, 'retain the complete actual flat source root ' + name);
    assert.deepEqual(selected[0].modulePath, []);
    return selected[0];
  });
  const rust = emitRust(checked, {arrayConsumption: true}), javascript = emitJavaScript(checked);
  const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino(), binary = encodeLinkNetworkBinary(packet.network);
  const programs = [{format: 'original', javascript}], replays = [];
  for (const [format, network] of [['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)]]) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram: true});
    assert.deepEqual(declarationInventory(restored), owners, 'every actual owner and order survives ' + format);
    const replayRust = emitRust(restored, {arrayConsumption: true}), replayJavaScript = emitJavaScript(restored);
    assert.deepEqual(replayRust, rust, 'the complete actual Rust emission survives ' + format);
    assert.deepEqual(replayJavaScript, javascript, 'the complete actual JavaScript emission survives ' + format);
    programs.push({format, javascript: replayJavaScript});
    replays.push({format, owners: declarationInventory(restored), rustBytes: Buffer.byteLength(replayRust.text),
      rustSha256: digest(replayRust.text), javascriptBytes: Buffer.byteLength(replayJavaScript.text), javascriptSha256: digest(replayJavaScript.text)});
  }
  for (const emitted of [rust, ...programs.map(program => program.javascript)]) ownerMappings(emitted, owners);
  return {policy: RUST_DIAGNOSTIC_ATTRIBUTE_POLICY, source, sourceBytes: Buffer.byteLength(source), sourceSha256: digest(source),
    roots: [...decisions], owners, parsedInventory, sourceOwners, rust, programs, replays,
    archive: {root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: digest(text),
      binaryBytes: binary.byteLength, binarySha256: digest(binary)}};
}

/** Inspect the actual packaged ABI instead of guessing native names or
 * equating source roots with automatically generated continuation counts. */
export async function readRustDiagnosticAttributeLibraryInterfaces(files, prepared) {
  const {readSourceTextRustBlock} = await import('./source-text-byte-library-style.mjs');
  const source = files.map(file => file.source).join('\n\n');
  const mappings = ownerMappings(prepared.rust, prepared.owners);
  for (const [at, owner] of prepared.owners.entries()) {
    const target = mappings[at].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u);
    const form = owner.kind === 'fn' ? '(?:const )?fn' : '(?:enum|struct)';
    assert.equal([...source.matchAll(new RegExp('^pub ' + form + ' ' + target + '\\b', 'gmu'))].length, 1,
      'whole-item packaging retains ' + owner.fullName);
  }
  const parameters = value => {
    const entries = value.split(',').map(part => part.trim());
    if (entries.length === 1 && entries[0] === '') return [];
    if (entries.at(-1) === '') entries.pop();
    assert.ok(entries.every(part => part !== ''), 'one optional final parameter comma never hides an interior empty cell');
    return entries.map(part => part.replace(/\s+/gu, ''));
  };
  const interfaces = ownerMappings(prepared.rust, prepared.sourceOwners).map((mapping, at) => {
    const original = readSourceTextRustBlock(prepared.rust.text, 'fn', mapping.target);
    const packaged = readSourceTextRustBlock(source, 'fn', mapping.target);
    const header = /\bfn \w+\(([^)]*)\)\s*->\s*([^\n{]+)\s*\{/u.exec(packaged);
    const before = /\bfn \w+\(([^)]*)\)\s*->\s*([^\n{]+)\s*\{/u.exec(original);
    assert.ok(before && header, 'every actual public decision has a complete primitive header');
    assert.deepEqual(parameters(header[1]), parameters(before[1]).map(parameter =>
      parameter.replace(/:Vec<String>$/u, ':&[String]').replace(/:String$/u, ':&str')),
    'only the existing immutable primitive loans change the source root ABI');
    assert.equal(header[2].replace(/\s+/gu, ''), before[2].replace(/\s+/gu, ''));
    return {source: prepared.sourceOwners[at].fullName, target: mapping.target,
      signature: header[0].slice(0, -1).trim(), parameters: parameters(header[1]), result: header[2].trim()};
  });
  return {interfaces, files: files.map(file => ({path: file.path ?? 'rust_diagnostic_attribute_rules.rs',
    source: file.source, bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source)}))};
}

/** The default path writes common data without importing the parser/checker
 * or library writer. This keeps the first policy-generation bootstrap finite. */
export async function generateRustDiagnosticAttributeRules({check = false, native = false} = {}) {
  const source = readRustDiagnosticAttributeRuleSource(), file = new URL(sourcePath, root);
  if (check) assert.equal(readFileSync(file, 'utf8'), source, 'the complete generated JavaScript module is fresh');
  else writeFileSync(file, source);
  const policy = {...RUST_DIAGNOSTIC_ATTRIBUTE_POLICY, sourcePath, source, sourceSha256: digest(source), decisions: [...decisions]};
  for (const relative of ['js/src/translation/data/rust-diagnostic-attributes.lino', 'rust/src/data/rust-diagnostic-attributes.lino']) {
    if (check) checkProjectData(new URL(relative, root), policy);
    else writeProjectData(new URL(relative, root), policy);
  }
  let artifact = null;
  if (native) {
    const prepared = await prepareRustDiagnosticAttributeTranslation(source);
    const {writeRecordRuleLibrary} = await import('./record-rule-library.mjs');
    const files = writeRecordRuleLibrary(prepared.rust, 'rust_diagnostic_attribute_rules', {check});
    const library = await readRustDiagnosticAttributeLibraryInterfaces(files, prepared);
    artifact = {schemaVersion: 1, model: policy.model, sourcePath, source, sourceBytes: prepared.sourceBytes,
      sourceSha256: prepared.sourceSha256, parsedInventory: prepared.parsedInventory, checkedInventory: prepared.owners,
      sourceOwners: prepared.sourceOwners, sourceOwnerMappings: ownerMappings(prepared.rust, prepared.sourceOwners),
      emitted: readRustDiagnosticAttributeEmissionData(prepared.rust), emittedBytes: Buffer.byteLength(prepared.rust.text), emittedSha256: digest(prepared.rust.text),
      javascript: readRustDiagnosticAttributeEmissionData(prepared.programs[0].javascript), archive: prepared.archive, replays: prepared.replays, library,
      boundary: 'Complete automatically translated source and whole-program text/binary replay. Source-profile, compiler/edition, attachment, native compilation and native runtime observations remain separate obligations.'};
    const output = new URL('parity/self-translation/rust-diagnostic-attribute-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return {policy, artifact};
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)),
    'only source-generation and explicit native-source options are admitted');
  const result = await generateRustDiagnosticAttributeRules({check: args.includes('--check'), native: args.includes('--native')});
  console.log(JSON.stringify({phase: 'whole Rust diagnostic attribute source generation', sourceBytes: Buffer.byteLength(result.policy.source),
    sourceSha256: result.policy.sourceSha256, decisions: result.policy.decisions.length,
    nativeSource: result.artifact === null ? null : {bytes: result.artifact.emittedBytes, sha256: result.artifact.emittedSha256,
      definitions: result.artifact.emitted.definitions.length, interfaces: result.artifact.library.interfaces},
    nativeCompilation: false, nativeExecution: false}));
}
