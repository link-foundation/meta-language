// Capture and translate the complete closed source-view kernel. The shared
// writer records library loans/styles; no native algorithm is authored here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/grammar-selection-included-range-rules.js';
const libraryName = 'grammar_selection_included_range_rules';
const digest = source => createHash('sha256').update(source).digest('hex');

/** Retain the whole actual source; new dependencies must be closed explicitly. */
export function readGrammarSelectionIncludedRangeSource() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.doesNotMatch(source, /^\s*(?:import\b|export\s+(?:\*|\{))/mu,
    'the included-range kernel is one complete module without unrecorded imports or reexports');
  const tokens = tokenize(source, 'JavaScript').tokens;
  assert.ok(!tokens.some((token, index) => token.kind === 'identifier' && (token.value === 'import'
    || token.value === 'export' && ['*', '{'].includes(tokens[index + 1]?.value))),
  'actual dependency tokens cannot be hidden by line layout');
  assert.ok(Buffer.byteLength(source) > 0, 'the complete actual source is present');
  const input = { path: sourcePath, source, bytes: Buffer.byteLength(source),
    sourceSha256: digest(source), removedImports: [] };
  return { source, bytes: input.bytes, sourceSha256: input.sourceSha256,
    sourceClosure: { source, bytes: input.bytes, sourceSha256: input.sourceSha256, inputs: [input] } };
}

export function readGrammarSelectionIncludedRangePolicy(closed = readGrammarSelectionIncludedRangeSource()) {
  return { schemaVersion: 1, model: 'proper ordered included range source views 1',
    sourcePath, source: closed.source, sourceBytes: closed.bytes, sourceSha256: closed.sourceSha256,
    sourceClosure: closed.sourceClosure,
    parameters: 'Nonempty ordered disjoint integral relative UTF8 byte ranges bound to the actual syntactically matched source occurrence. Every range is nonempty and scalar aligned; the total selected width must be strictly smaller than the matched width.',
    byteBoundary: 'The existing host source reader first validates the actual complete parent UTF8 plane. Shared boundary decisions then consume actual parent byte and end facts; this kernel does not decode or replace Unicode semantics.',
    correspondence: 'The complete chunk plan retains zero-output excluded gaps and endpoint rows. Fixed-size part results use the unchanged plan and actual scalar widths from the existing decoder to retain each scalar correspondence without cloning the complete plan per part.',
    sourcePlanes: 'Once-owned ordered source definitions and explicit positional input/output references retain each actual parent occurrence and every admitted primary or alternative CST. Equal bytes, names, hashes or host pointers do not establish source ownership.',
    resources: 'Reservations and all mapping/child work use the original parse memory, step and remaining depth budgets. Proper byte contraction is this profile\'s progress premise; empty and full-cover views are outside it.',
    archiveBoundary: 'Generic Links decoding does not authenticate source mappings. The domain checker must replay these same generated plans against complete physical CST bytes and explicit source definitions. Language-CST validity still requires actual parsing.',
    legacyBoundary: 'Existing identity and JSON-string decision modules, policies and native algorithms are separate unchanged inputs, not regenerated or broadened by this producer.',
    boundary: 'This finite explicit-range profile does not discover ranges from Markdown/CST fields, implement full/no-op source-lineage views, discharge scanner context obligations, or establish complete Markdown or other priority-language support. Native compilation and execution remain CI observations.' };
}

function checkedOwners(program, inventory) {
  const owners = inventory.map(item => {
    const declaration = program.declarations.get(item.name);
    assert.ok(declaration && declaration.k === item.kind && declaration.name === item.name,
      'retain the actual complete checked owner ' + item.name);
    return { sourceOrdinal: item.ordinal, kind: declaration.k, name: declaration.name,
      fullName: declaration.fullName, modulePath: declaration.modulePath, generated: item.generated };
  });
  assert.equal(owners.length, program.declarations.size, 'no checked source or continuation owner is omitted');
  assert.equal(new Set(owners.map(owner => owner.fullName)).size, owners.length, 'every actual owner has one identity');
  return owners;
}

function ownerMappings(emitted, owners) {
  return owners.map(owner => {
    const mappings = emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(mappings.length, 1, 'one complete actual emitted mapping for ' + owner.fullName);
    return mappings[0];
  });
}

function nativeParameters(value) {
  const parts = value.split(',').map(part => part.trim());
  if (parts.length === 1 && parts[0] === '') return [];
  if (parts.at(-1) === '') parts.pop();
  assert.ok(parts.every(part => part !== ''), 'only one optional trailing parameter comma is admitted');
  return parts.map(part => {
    const declaration = /^(mut\s+)?([A-Za-z_][A-Za-z_0-9]*)\s*:\s*([\s\S]+)$/u.exec(part);
    assert.ok(declaration, 'the actual primitive parameter retains its name and type');
    return (declaration[1] ? 'mut ' : '') + declaration[2] + ':' + declaration[3].replace(/\s+/gu, '');
  });
}

function rejectInputCopies(block, names) {
  const tokens = tokenize(block, 'Rust').tokens, matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    const value = tokens[index].value;
    if (['(', '[', '{'].includes(value)) stack.push(index);
    else if ([')', ']', '}'].includes(value)) {
      const opening = stack.pop();
      assert.equal(tokens[opening]?.value, ({ ')': '(', ']': '[', '}': '{' })[value]);
      matching.set(opening, index);
    }
  }
  assert.equal(stack.length, 0, 'the complete actual function has balanced delimiters');
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'identifier' || !names.has(tokens[index].value)) continue;
    let left = index, right = index;
    while (tokens[left - 1]?.value === '(' && matching.get(left - 1) === right + 1) { left -= 1; right += 1; }
    assert.ok(!(tokens[right + 1]?.value === '.' && ['clone', 'to_vec', 'to_owned'].includes(tokens[right + 2]?.value)
      && tokens[right + 3]?.value === '(' && tokens[right + 4]?.value === ')'),
    'an immutable whole input vector is not copied at an actual generated call');
  }
}

/** Inspect actual packaged owners and ABI after the writer's recorded proofs. */
export function readGrammarSelectionIncludedRangeLibraryInterface(files, emitted, owners) {
  const items = files.flatMap(file => readRustLibraryItems(file.source).items);
  const mappings = ownerMappings(emitted, owners), interfaces = [];
  for (const [index, owner] of owners.entries()) {
    const target = mappings[index].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u, 'one actual flat library owner');
    const declarations = items.filter(item => item.name === target
      && (owner.kind === 'fn' ? item.kind === 'fn' : ['enum', 'struct'].includes(item.kind)));
    assert.equal(declarations.length, 1, 'packaging retains every actual source/continuation owner ' + owner.fullName);
    // A continuation's owned mutable output/state vector is distinct from an
    // original decision's immutable input. Do not invent a loan for that state.
    if (owner.generated || owner.kind !== 'fn') continue;
    const block = declarations[0].source;
    const raw = readSourceTextRustBlock(emitted.text, 'fn', target);
    const header = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(block);
    const original = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(raw);
    assert.ok(header && original, 'both complete actual function signatures are present');
    const before = nativeParameters(original[1]);
    const expected = before.map(parameter => parameter.replace(/^(?:mut )?(\w+):Vec<(f64|String|bool)>$/u, '$1:&[$2]')
      .replace(/^(?:mut )?(\w+):String$/u, '$1:&str'));
    const parameters = nativeParameters(header[1]);
    assert.deepEqual(parameters, expected, 'only the existing immutable parameter loans change the actual source ABI');
    assert.equal(header[2].replace(/\s+/gu, ''), original[2].replace(/\s+/gu, ''), 'the complete return type is unchanged');
    const vectors = new Set(before.flatMap(parameter => /^(?:mut )?(\w+):Vec<(?:f64|String|bool)>$/u.exec(parameter)?.[1] ?? []));
    rejectInputCopies(block, vectors);
    interfaces.push({ source: owner.fullName, target, parameters,
      result: header[2].trim(), signature: header[0].slice(0, -1).trim() });
  }
  const part = interfaces.find(item => item.source === 'readGrammarSelectionIncludedRangePart');
  const plan = interfaces.find(item => item.source === 'readGrammarSelectionIncludedRangePlan');
  assert.ok(part && plan, 'both actual whole-plan and streamed-part source interfaces survive');
  assert.ok(part.parameters.includes('plan:&[f64]'), 'the streamed part borrows the complete original numeric plan');
  assert.ok(plan.parameters.includes('ranges:&[f64]'), 'the whole planner borrows the original range column');
  return { ownerMappings: mappings, interfaces, files: files.map(file => ({
    path: file.path ?? libraryName + '.rs', bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })) };
}

export async function generateGrammarSelectionIncludedRangeRules({ check = false, native = false } = {}) {
  const closed = readGrammarSelectionIncludedRangeSource();
  const policy = readGrammarSelectionIncludedRangePolicy(closed);
  for (const relative of ['js/src/translation/data/grammar-selection-included-range-rules.lino',
    'rust/src/data/grammar-selection-included-range-rules.lino']) {
    const file = new URL(relative, root);
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
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
    const surface = parseJavaScript(closed.source, { referenceSemantics: false });
    const inventory = surface.items.map((item, ordinal) => {
      assert.ok(['fn', 'data'].includes(item.k) && typeof item.name === 'string' && item.name !== '',
        'classify every actual declaration, including all generated continuations');
      return { ordinal, kind: item.k, name: item.name, generated: item.generated === true, span: item.span };
    });
    const checked = checkProgram(surface), owners = checkedOwners(checked, inventory);
    const emitted = emitRust(checked, { arrayConsumption: true });
    ownerMappings(emitted, owners);
    const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino();
    const binary = encodeLinkNetworkBinary(packet.network), replays = [];
    for (const [format, network] of [
      ['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)],
    ]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      const restoredOwners = checkedOwners(restored, inventory);
      assert.deepEqual(restoredOwners, owners, 'every complete checked owner/order survives ' + format);
      const replay = emitRust(restored, { arrayConsumption: true });
      ownerMappings(replay, restoredOwners);
      assert.equal(replay.text, emitted.text, 'complete native source survives whole-program ' + format + ' recheck');
      for (const field of ['definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) {
        assert.deepEqual(replay[field], emitted[field], 'complete native ' + field + ' survives ' + format);
      }
      replays.push({ format, checkedNamedSourceInventory: restoredOwners,
        emittedTextBytes: Buffer.byteLength(replay.text), emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, libraryName, { check });
    const library = readGrammarSelectionIncludedRangeLibraryInterface(files, emitted, owners);
    artifact = { schemaVersion: 1, model: policy.model, source: closed.source,
      sourceSha256: closed.sourceSha256, sourceClosure: closed.sourceClosure,
      inventory, checkedNamedSourceInventory: owners, replays,
      archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: digest(text),
        binaryBytes: binary.byteLength, binarySha256: digest(binary) },
      emitted: { ...emitted, definitions: emitted.definitions, preludes: emitted.preludes },
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: digest(emitted.text), library,
      boundary: 'Actual complete source capture, whole-program archive rechecks, raw Rust emission and recorded library packaging. Native compilation/execution, parser-host parity and language-CST validity require their actual CI/runtime assertions.' };
    const output = new URL('parity/self-translation/grammar-selection-included-range-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)),
    'only complete source-generation/freshness options are admitted');
  const result = await generateGrammarSelectionIncludedRangeRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ phase: 'complete included-range source generation', sourceBytes: result.policy.sourceBytes,
    sourceSha256: result.policy.sourceSha256, nativeSource: result.artifact === null ? null : {
      bytes: result.artifact.emittedTextBytes, sha256: result.artifact.emittedTextSha256,
      owners: result.artifact.checkedNamedSourceInventory.length,
      interfaces: result.artifact.library.interfaces }, nativeCompilation: false, nativeExecution: false }, null, 2));
}
