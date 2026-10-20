// Preserve and translate the complete JavaScript policy module; no native
// pattern recognizer or replacement function body is authored by this writer.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/rust-literal-match-rules.js';
const digest = source => createHash('sha256').update(source).digest('hex');
const owners = [
  ['rust/src/grammar/emit/javascript.rs', 'is_reserved_identifier'],
  ['rust/src/grammar/emit/tree_sitter.rs', 'is_reserved_identifier'],
  ['rust/src/grammar/emit/rust.rs', 'is_rust_keyword'],
];

export async function generateRustLiteralMatchRules({ check = false, native = false } = {}) {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.doesNotMatch(source, /^\s*import\b/mu, 'the whole decision module has no hidden import closure');
  const decisions = [...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]);
  assert.deepEqual(decisions, ['acceptRustLiteralMatchReceiver', 'acceptRustLiteralMatchCatalogOwner', 'planRustLiteralMatchPattern']);
  const sourceClosure = { source, bytes: Buffer.byteLength(source), sourceSha256: digest(source),
    inputs: [{ path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), removedImports: [] }] };
  const originalDeclarations = owners.map(([path, name]) => {
    const context = readFileSync(new URL(path, root), 'utf8');
    const declaration = readSourceTextRustBlock(context, 'fn', name);
    assert.ok(declaration.startsWith('fn ' + name + '('), 'retain the actual ordinary declaration without dropping const qualification');
    return { path, bytes: Buffer.byteLength(context), sourceSha256: digest(context), context,
      name, declaration, declarationSha256: digest(declaration) };
  });
  const policy = { schemaVersion: 1, model: 'Rust borrowed string literal match 1', sourcePath,
    source, sourceSha256: digest(source), sourceClosure, decisions, originalDeclarations,
    references: ['https://doc.rust-lang.org/std/macro.matches.html',
      'https://doc.rust-lang.org/reference/patterns.html#literal-patterns'],
    receiver: 'The actual proven unshadowed immutable borrowed str source owner.',
    pattern: 'One or more actual string literal tokens, optional leading | and trailing comma; no guard, binding, range, wildcard or other pattern.',
    evaluation: 'Evaluate the actual receiver once, then compare immutable values in original alternative order.',
    boundary: 'Original complete declarations and source-free semantic emission are checked independently. No source inventory is native execution evidence.' };
  for (const path of ['js/src/translation/data/rust-literal-match-rules.lino', 'rust/src/data/rust-literal-match-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy);
    else writeProjectData(new URL(path, root), policy);
  }
  let artifact = null;
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { writeRecordRuleLibrary } = await import('./record-rule-library.mjs');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const surface = parseJavaScript(source, { referenceSemantics: false });
    const inventory = surface.items.map((item, ordinal) => ({ ordinal, kind: item.k, name: item.name, generated: item.generated === true }));
    assert.deepEqual(inventory.filter(item => !item.generated).map(item => item.name), decisions);
    const checked = checkProgram(surface);
    const checkedOwners = program => inventory.map(owner => {
      const declaration = program.declarations.get(owner.name);
      assert.ok(declaration && declaration.k === owner.kind);
      return { sourceOrdinal: owner.ordinal, kind: declaration.k, name: declaration.name,
        fullName: declaration.fullName, generated: owner.generated };
    });
    const checkedNamedSourceInventory = checkedOwners(checked);
    assert.equal(checkedNamedSourceInventory.length, checked.declarations.size, 'every actual generated and source owner is retained');
    const output = emitRust(checked, { arrayConsumption: true });
    const emitted = { text: output.text, definitions: output.definitions, preludes: output.preludes,
      mappings: output.mappings, assumptions: output.assumptions, encodings: output.encodings };
    for (const owner of checkedNamedSourceInventory) assert.equal(emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind)).length, 1);
    const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino(), binary = encodeLinkNetworkBinary(packet.network), replays = [];
    for (const [format, network] of [['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.deepEqual(checkedOwners(restored), checkedNamedSourceInventory);
      const replay = emitRust(restored, { arrayConsumption: true });
      for (const field of Object.keys(emitted)) assert.deepEqual(replay[field], emitted[field], 'complete ' + format + ' replay of ' + field);
      replays.push({ format, checkedNamedSourceInventory: checkedOwners(restored), emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, 'rust_literal_match_rules', { check });
    artifact = { schemaVersion: 1, model: policy.model, sourceClosure, inventory, checkedNamedSourceInventory,
      emitted, replays, files: files.map(file => ({ path: file.path ?? 'rust_literal_match_rules.rs',
        bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })),
      boundary: 'Complete source-only checked translation and both source-free archive replays; native execution remains CI-only.' };
    const path = new URL('parity/self-translation/rust-literal-match-rules-native.lino', root);
    if (check) checkProjectData(path, artifact);
    else writeProjectData(path, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)));
  const result = await generateRustLiteralMatchRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ decisions: result.policy.decisions, originalDeclarations: result.policy.originalDeclarations.map(owner => owner.path + '::' + owner.name),
    checkedOwners: result.artifact?.checkedNamedSourceInventory.length ?? null, nativeCompilation: false, nativeExecution: false }));
}
