// Translate every callback decision and renderer through the checked frontend.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/array-callback-rules.js';
const hash = source => createHash('sha256').update(source).digest('hex');

export function readArrayCallbackSourceModule() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.doesNotMatch(source, /^\s*import\b/mu, 'the complete callback module has no unrecorded dependency');
  return source;
}

export function arrayCallbackRulesPolicy() {
  const source = readArrayCallbackSourceModule();
  const input = { path: sourcePath, source, sourceSha256: hash(source), bytes: Buffer.byteLength(source) };
  return { schemaVersion: 1, model: 'ordered array callback decisions and rendering 1', sourcePath,
    source, sourceSha256: input.sourceSha256, sourceBytes: input.bytes,
    sourceClosure: { source, sourceSha256: input.sourceSha256, bytes: input.bytes, removedImports: [], inputs: [input] },
    checking: 'The shared decisions preserve method arity and ordered callback kind, parameter and result refusals. Hosts retain actual type owners and the existing checker; reduce checks its initial expression with the outer expected type before the callback. FlatMap checks its actual array result after an absent callback return context.',
    inference: 'Existing JavaScript inference and source rules remain unchanged. Native adapters may reuse the actual Array.from callback constraints, including ordered parameter and result pairs and the existing excessive-parameter diagnostic.',
    rendering: 'The complete original filter, find, findIndex, some, every, flatMap, forEach and reduce Rust templates retain receiver, callback and initial expression order, callback arity, iteration order and short-circuiting. Map keeps its existing separate renderer.',
    boundary: 'Complete checked-source and archive replay authentication does not establish native execution, shared-reference mutation, sparse arrays, arbitrary truthy predicates or additional target encodings.' };
}


/** Only the complete authenticated String producer permits this empty test. */
export function readArrayCallbackLibraryStyles(source) {
  const items = readRustLibraryItems(source).items;
  if (items.some(item => !['fn', 'inner attribute'].includes(item.kind))) return {source, actions: []};
  const key = text => JSON.stringify(tokenize(text, 'Rust').tokens.filter(token => token.kind !== 'eof')
    .map(token => [token.kind, token.raw]));
  const callee = items.filter(item => item.kind === 'fn' && item.name === 'read_array_callback_result_form');
  const owners = items.filter(item => item.kind === 'fn' && item.name === 'read_array_callback_maximum_arguments');
  if (callee.length !== 1 || owners.length !== 1) return {source, actions: []};
  const expectedCallee = `pub fn read_array_callback_result_form(operation: &str) -> String {
    if operation == "map" { String::from("mapped array") }
    else if operation == "filter" { String::from("receiver") }
    else if operation == "find" { String::from("optional element") }
    else if operation == "findIndex" { String::from("float") }
    else if (operation == "some") || (operation == "every") { String::from("bool") }
    else if operation == "flatMap" { String::from("callback array") }
    else if operation == "forEach" { String::from("unit") }
    else if operation == "reduce" { String::from("accumulator") }
    else { String::new() }
  }`;
  const expectedOwner = `pub fn read_array_callback_maximum_arguments(operation: &str) -> f64 {
    if operation == "reduce" { 2f64 }
    else if (self::read_array_callback_result_form(operation)) == "" { -1f64 }
    else { 1f64 }
  }`;
  const from = readSourceTextRustBlock(owners[0].source, 'fn', owners[0].name);
  if (key(readSourceTextRustBlock(callee[0].source, 'fn', callee[0].name)) !== key(expectedCallee)
    || key(from) !== key(expectedOwner)) return {source, actions: []};
  const {tokens, comments} = tokenize(from, 'Rust');
  const comparison = '(self::read_array_callback_result_form(operation)) == ""';
  const signature = tokenize(comparison, 'Rust').tokens.filter(token => token.kind !== 'eof');
  const matches = [];
  for (let index = 0; index + signature.length <= tokens.length; index += 1) {
    const selected = tokens.slice(index, index + signature.length);
    if (JSON.stringify(selected.map(token => [token.kind, token.raw])) === key(comparison)) matches.push(selected);
  }
  if (matches.length !== 1) return {source, actions: []};
  const selected = matches[0], start = selected[0].start, end = selected.at(-1).end;
  if (comments.some(comment => comment.start < end && comment.end > start)) return {source, actions: []};
  const equals = selected.find(token => token.value === '==');
  assert.ok(equals, 'the authenticated comparison retains its actual equality token');
  const to = from.slice(0, start) + from.slice(start, equals.start).trimEnd() + '.is_empty()' + from.slice(end);
  assert.equal(source.split(from).length, 2, 'one complete actual maximum-argument owner');
  return {source: source.replace(from, to), actions: [{op: 'replace', field: 'source', from, to}]};
}

export async function generateArrayCallbackRules({ check = false, native = false } = {}) {
  const policy = arrayCallbackRulesPolicy();
  for (const path of ['js/src/translation/data/array-callback-rules.lino', 'rust/src/data/array-callback-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy);
    else writeProjectData(new URL(path, root), policy);
  }
  let artifact = null;
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const surface = parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false });
    const checked = checkProgram(surface);
    const inventory = surface.items.map((item, ordinal) => ({ ordinal, kind: item.k, name: item.name, generated: item.generated === true }));
    const owners = program => inventory.map(item => {
      const declaration = program.declarations.get(item.name);
      assert.ok(declaration && declaration.k === item.kind, 'every complete callback source/continuation owner remains present');
      return { ...item, fullName: declaration.fullName,
        ...(declaration.modulePath === undefined ? {} : { modulePath: declaration.modulePath }) };
    });
    const checkedNamedSourceInventory = owners(checked);
    assert.equal(checkedNamedSourceInventory.length, checked.declarations.size);
    const emitted = emitRust(checked, { arrayConsumption: true });
    assert.deepEqual(emitted.preludes, [], 'the complete callback decisions remain a closed scalar library');
    const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino();
    const binary = encodeLinkNetworkBinary(packet.network), replays = [];
    for (const [format, network] of [['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.deepEqual(restored, checked, 'every checked callback record remains present after archive reload');
      assert.deepEqual(owners(restored), checkedNamedSourceInventory);
      const replay = emitRust(restored, { arrayConsumption: true });
      for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) assert.deepEqual(replay[field], emitted[field]);
      replays.push({ format, checkedNamedSourceInventory: owners(restored), emittedTextBytes: Buffer.byteLength(replay.text), emittedTextSha256: hash(replay.text) });
    }
    for (const owner of checkedNamedSourceInventory) assert.equal(emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind)).length, 1, 'one complete actual callback owner mapping');
    const files = writeRecordRuleLibrary(emitted, 'array_callback_rules', { check, sourceStyles: source => {
      const styled = readArrayCallbackLibraryStyles(source);
      assert.equal(styled.actions.length, 1, 'the exact authenticated String empty comparison retains its recorded style');
      return styled;
    } });
    artifact = { schemaVersion: 1, model: policy.model, sourceClosure: policy.sourceClosure, inventory,
      checkedNamedSourceInventory, replays,
      emitted: { text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes,
        mappings: emitted.mappings, assumptions: emitted.assumptions, encodings: emitted.encodings },
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: hash(emitted.text),
      archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: hash(text), binaryBytes: binary.byteLength, binarySha256: hash(binary) },
      library: { files: files.map(file => ({ path: file.path ?? 'array_callback_rules.rs', bytes: Buffer.byteLength(file.source), sourceSha256: hash(file.source) })) },
      boundary: 'Complete source and archive replay authentication; native compilation and execution require actual CI controls.' };
    const output = new URL('parity/self-translation/array-callback-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)), 'callback generator accepts only native generation and freshness flags');
  await generateArrayCallbackRules({ check: args.includes('--check'), native: args.includes('--native') });
}
