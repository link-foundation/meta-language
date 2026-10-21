// Capture the complete actual decision closure, translate every declaration,
// and retain both honest archive replays. No native semantics are handwritten.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { readAsciiTextPredicateLibraryStyles } from './ascii-text-predicate-library-style.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/ascii-text-predicate-rules.js';
const dependencyPath = 'js/src/translation/rust-const-function-rules.js';
const boundary = "import { acceptRustConstFunctionName, acceptRustConstFunctionNameCharacter } from './rust-const-function-rules.js';\n";
const digest = source => createHash('sha256').update(source).digest('hex');
const decisions = ['acceptAsciiCharacterMethod', 'acceptRustAsciiCharacterOwner',
  'planRustAsciiTextQuery', 'planRustAsciiCharacterCallback'];
const actualOwners = [
  ['rust/src/grammar/emit/tree_sitter_json.rs', 'is_identifier'],
  ['rust/src/grammar/emit/lark.rs', 'is_terminal_name'],
];

export function readAsciiTextPredicateSourceClosure() {
  const inputs = [], parts = [];
  for (const [path, imports] of [[dependencyPath, []], [sourcePath, [boundary]]]) {
    const source = readFileSync(new URL(path, root), 'utf8');
    let body = source;
    const removedImports = [];
    for (const statement of imports) {
      assert.equal(source.split(statement).length, 2, 'one complete actual dependency import');
      const start = source.indexOf(statement), end = start + statement.length;
      removedImports.push({ statement, start, end,
        byteStart: Buffer.byteLength(source.slice(0, start)),
        byteEnd: Buffer.byteLength(source.slice(0, end)) });
      assert.equal(body.slice(start, end), statement);
      body = body.slice(0, start) + body.slice(end);
    }
    assert.doesNotMatch(body, /^\s*import\b/mu, 'the whole source closure has no unrecorded imports');
    inputs.push({ path, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), removedImports });
    parts.push(body);
  }
  const source = parts.join('\n\n');
  return { source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), inputs };
}

function actualDeclaration(path, name) {
  const context = readFileSync(new URL(path, root), 'utf8');
  const declaration = readSourceTextRustBlock(context, 'fn', name);
  assert.ok(declaration.startsWith('fn ' + name + '('), 'retain the original ordinary function and all its parameter/body syntax');
  const tokens = tokenize(context, 'Rust').tokens;
  const found = tokens.filter((token, index) => token.value === 'fn' && tokens[index + 1]?.value === name);
  assert.equal(found.length, 1, 'exactly one actual named source declaration');
  const index = tokens.indexOf(found[0]);
  assert.notEqual(tokens[index - 1]?.value, 'const', 'do not silently drop any original const qualification');
  return { path, name, context, bytes: Buffer.byteLength(context), sourceSha256: digest(context),
    declaration, declarationSha256: digest(declaration) };
}

function checkedOwners(program, inventory) {
  return inventory.map(owner => {
    const declaration = program.declarations.get(owner.name);
    assert.ok(declaration && declaration.k === owner.kind);
    return { ...owner, fullName: declaration.fullName, modulePath: declaration.modulePath };
  });
}

// Record actual generated signatures rather than guessing a native host ABI.
function libraryInterfaces(files, emitted, owners) {
  const interfaces = [];
  for (const owner of owners) {
    const mappings = emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(mappings.length, 1);
    const target = mappings[0].target;
    const found = [];
    for (const file of files) {
      const tokens = tokenize(file.source, 'Rust').tokens;
      for (let index = 0; index < tokens.length; index += 1) {
        if (tokens[index].value !== 'fn' || tokens[index + 1]?.value !== target) continue;
        let body = index + 2;
        while (body < tokens.length && tokens[body].value !== '{') body += 1;
        assert.ok(body < tokens.length, 'the actual generated declaration has a complete header');
        found.push({ source: owner.fullName, target, kind: owner.kind,
          path: file.path ?? 'ascii_text_predicate_rules.rs',
          signature: file.source.slice(tokens[index].start, tokens[body].start).trim() });
      }
    }
    if (owner.kind === 'fn') assert.equal(found.length, 1, 'one actual generated function ABI');
    interfaces.push(...found);
  }
  return interfaces;
}

export async function generateAsciiTextPredicateRules({ check = false, native = false } = {}) {
  const sourceClosure = readAsciiTextPredicateSourceClosure();
  const source = sourceClosure.inputs.at(-1).source;
  assert.deepEqual([...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]), decisions);
  const policy = { schemaVersion: 1, model: 'Rust borrowed string ASCII character queries 1',
    sourcePath, decisions, sourceClosure,
    originalDeclarations: actualOwners.map(([path, name]) => actualDeclaration(path, name)),
    assertionVersions: { Rust: '>=1.90.0 <1.100.0', Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0' },
    receiver: 'Actual proven unshadowed immutable borrowed str, exactly one reference layer. Explicit char annotations retain their actual primitive type owner; the standard query supplies unannotated char.',
    syntax: 'Only starts_with(|c[:char]| predicate) and direct chars().all(|c[:char]| predicate); a nonempty OR of five standard ASCII methods or c == actual ASCII character literal. No captures, effects, blocks, mutable binders, arguments, general char values or general iterator admission.',
    observation: 'Primitive Unicode scalar string inputs, standard unmodified string/array intrinsics, and successful allocation. The receiver is evaluated once. An empty all input is true; an empty first input is false. Every non-ASCII scalar is rejected.',
    resources: 'First uses at most 128 pure startsWith tests, independent of input length. All uses existing eager StringPoints plus every, adding O(n) temporary point storage compared with lazy Rust Iterator::all. Allocation/failure, timing, pointer and ABI equivalence are not claimed.',
    boundary: 'Original complete sources are preserved, not rewritten into handwritten replacement helpers. The original unrestricted JavaScript regex helper remains outside this interpretation. Source generation does not prove native execution.',
    references: ['https://doc.rust-lang.org/1.99.1/std/primitive.char.html#method.is_ascii_alphabetic',
      'https://doc.rust-lang.org/1.99.1/std/primitive.str.html#method.starts_with',
      'https://doc.rust-lang.org/1.99.1/std/primitive.str.html#method.chars',
      'https://doc.rust-lang.org/1.99.1/std/iter/trait.Iterator.html#method.all'] };
  for (const path of ['js/src/translation/data/ascii-text-predicate-rules.lino', 'rust/src/data/ascii-text-predicate-rules.lino']) {
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
    const surface = parseJavaScript(sourceClosure.source, { referenceSemantics: false });
    const inventory = surface.items.map((item, sourceOrdinal) => {
      assert.ok(['fn', 'data'].includes(item.k) && typeof item.name === 'string');
      return { sourceOrdinal, kind: item.k, name: item.name, generated: item.generated === true };
    });
    const sourceNames = sourceClosure.inputs.flatMap(input => [...input.source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]));
    assert.deepEqual(inventory.filter(owner => !owner.generated).map(owner => owner.name), sourceNames,
      'retain every actual dependency and new policy owner in original order');
    const checked = checkProgram(surface), owners = checkedOwners(checked, inventory);
    assert.equal(owners.length, checked.declarations.size);
    const output = emitRust(checked, { arrayConsumption: true });
    const emitted = { text: output.text, definitions: output.definitions, preludes: output.preludes,
      mappings: output.mappings, assumptions: output.assumptions, encodings: output.encodings };
    const packet = encodeTypedProgramLinks(checked), replays = [];
    for (const [format, network] of [['text', LinkNetwork.fromLino(packet.network.toLino())],
      ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.deepEqual(checkedOwners(restored, inventory), owners);
      const replay = emitRust(restored, { arrayConsumption: true });
      for (const field of Object.keys(emitted)) assert.deepEqual(replay[field], emitted[field], format + ' preserves complete ' + field);
      replays.push({ format, checkedNamedSourceInventory: checkedOwners(restored, inventory),
        emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, 'ascii_text_predicate_rules', {
      check, sourceStyles: readAsciiTextPredicateLibraryStyles,
    });
    const librarySourceStyles = {
      rustVersionRange: '>=1.90.0 <1.100.0',
      references: ['https://doc.rust-lang.org/std/primitive.f64.html#method.is_finite'],
      inputs: ['js/scripts/ascii-text-predicate-library-style.mjs',
        'rust/src/translation/rust/const_function_rules.rs'].map(path => {
        const source = readFileSync(new URL(path, root), 'utf8');
        return { path, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source) };
      }),
      boundary: 'Complete current style/reference source captures and two exact primitive qualification actions; no Git membership, runtime execution, or equivalence between distinct current source owners is inferred.',
    };
    artifact = { schemaVersion: 1, model: policy.model, sourceClosure, inventory,
      checkedNamedSourceInventory: owners, emitted, replays, librarySourceStyles,
      libraryInterface: libraryInterfaces(files, emitted, owners),
      files: files.map(file => ({ path: file.path ?? 'ascii_text_predicate_rules.rs',
        source: file.source, bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })),
      boundary: 'Entire checked closure, actual generated ABI, and both source-free replays; native compilation and execution are CI observations only.' };
    const file = new URL('parity/self-translation/ascii-text-predicate-rules-native.lino', root);
    if (check) checkProjectData(file, artifact);
    else writeProjectData(file, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)));
  const result = await generateAsciiTextPredicateRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ decisions, actualOwners: result.policy.originalDeclarations.map(owner => owner.path + '::' + owner.name),
    checkedOwners: result.artifact?.checkedNamedSourceInventory.length ?? null,
    nativeCompilation: false, nativeExecution: false }));
}
