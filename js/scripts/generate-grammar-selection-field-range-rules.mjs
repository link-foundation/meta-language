// Capture and translate the complete closed physical-field selection kernel. The shared
// writer records library loans/styles; no native algorithm is authored here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readPortableSourceParameterStyles } from './portable-source-library-style.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { readGrammarSelectionRangeLibraryStyles } from './generate-grammar-selection-included-range-rules.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/grammar-selection-field-range-rules.js';
const libraryName = 'grammar_selection_field_range_rules';
const digest = source => createHash('sha256').update(source).digest('hex');

/** Retain the whole actual source; new dependencies must be closed explicitly. */
export function readGrammarSelectionFieldRangeSource() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.doesNotMatch(source, /^\s*(?:import\b|export\s+(?:\*|\{))/mu,
    'the field-range kernel is one complete module without unrecorded imports or reexports');
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

export function readGrammarSelectionFieldRangePolicy(closed = readGrammarSelectionFieldRangeSource()) {
  return { schemaVersion: 1, model: 'proper physical CST field range selection 1',
    sourcePath, source: closed.source, sourceBytes: closed.bytes, sourceSha256: closed.sourceSha256,
    sourceClosure: closed.sourceClosure,
    profile: 'The field-ranges mapping admits one source-field-selector with an exact nonempty field label, include or exclude mode, nonnegative integral first occurrence and strictly positive integral count. Counts and source positions are bounded by the original unsigned 32-bit source domain.',
    context: 'A field profile cannot be mixed with another source mapping or selected inside a lexical token. Zero count never means all; the exact label is a capture field, not a grammar rule name.',
    occurrences: 'The host supplies topmost non-trivia physical CST field occurrences in their original child order. Every supplied absolute span is validated, including unselected occurrences. The closed kernel neither sorts nor merges spans nor discovers trees through callbacks.',
    correspondence: 'The complete plan consumes one read-only flat numeric span column. Include mode retains selected relative pairs; exclude mode retains their ordered complement. Empty and full-width results refuse, so admitted output is a proper contraction consumed by the unchanged scalar-aware included-range mapper.',
    ownership: 'The existing physical tree traversal and source-plane checker retain actual child ownership, exact occurrence paths, complete source bytes and UTF8 scalar boundaries. Equal labels, byte strings or hashes do not establish those obligations.',
    resources: 'Host traversal, witness storage, source mapping and recursion retain their existing memory, step and depth budgets. The planner does not copy the complete immutable span column at each continuation.',
    archiveBoundary: 'Generic Links decoding does not authenticate source mappings. Source-free checking must replay the actual field occurrences and generated range plan against the complete physical CST and retained source definitions.',
    legacyBoundary: 'Identity, JSON-string and explicit included-range decision modules remain separate unchanged inputs. This producer retains every complete source and generated continuation owner rather than selecting a convenient function subset.',
    boundary: 'This finite physical-field profile does not establish complete Markdown or other priority-language support, scanner context discharge or language-CST validity. Native compilation and execution require actual CI observations.' };
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
    if (tokens[index].kind !== 'identifier' || !names.has(tokens[index].value)
      || ['.', '::'].includes(tokens[index - 1]?.value)) continue;
    let left = index, right = index;
    while (tokens[left - 1]?.value === '(' && matching.get(left - 1) === right + 1) {
      const previous = tokens[left - 2];
      if (previous?.kind === 'macro' || previous?.kind === 'identifier'
        && !['if', 'while', 'return', 'match', 'break'].includes(previous.value)
        || [')', ']', '?'].includes(previous?.value)) break;
      left -= 1; right += 1;
    }
    assert.ok(!(tokens[right + 1]?.value === '.' && ['clone', 'to_vec', 'to_owned'].includes(tokens[right + 2]?.value)
      && tokens[right + 3]?.value === '(' && tokens[right + 4]?.value === ')'),
    'an immutable whole input vector is not copied in a complete source or continuation owner');
  }
}

/** Inspect every actual packaged owner and the shared recorded parameter loans. */
export function readGrammarSelectionFieldRangeLibraryInterface(files, emitted, owners) {
  const items = files.flatMap(file => readRustLibraryItems(file.source).items);
  const mappings = ownerMappings(emitted, owners), interfaces = [];
  const rawItems = emitted.definitions.flatMap(definition => readRustLibraryItems(definition).items);
  const stylePath = 'parity/self-translation/grammar-selection-field-range-rules-decorators.lino';
  const styleSource = readFileSync(new URL(stylePath, root), 'utf8');
  const styles = DecoratorSet.fromLino(styleSource).decorators;
  const recordedStyles = styles.filter(style => style.id === libraryName + '-library-style');
  assert.equal(recordedStyles.length, 1, 'the actual writer records one complete family library style');
  const recordedStyle = recordedStyles[0];
  assert.equal(recordedStyle.level, 'emitter');
  assert.equal(recordedStyle.order, 0);
  assert.deepEqual(recordedStyle.when, [['format', 'Rust']]);
  const typeOwners = owners.flatMap((owner, index) => owner.kind === 'data'
    ? [{ owner, target: mappings[index].target }] : []);
  const relocateReturnType = value => {
    const tokens = tokenize(value, 'Rust').tokens, edits = [], relocations = [];
    for (let index = 0; index + 2 < tokens.length; index += 1) {
      if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'crate'
        || tokens[index + 1].value !== '::' || tokens[index + 2].kind !== 'identifier'
        || tokens[index - 1]?.value === '::' || tokens[index + 3]?.value === '::') continue;
      const target = tokens[index + 2].value;
      const mapped = typeOwners.filter(item => item.target === target);
      if (mapped.length === 0) continue;
      assert.equal(mapped.length, 1, 'a relocated local return type has one actual checked data owner');
      const original = rawItems.filter(item => item.name === target && ['enum', 'struct'].includes(item.kind));
      const packaged = items.filter(item => item.name === target && ['enum', 'struct'].includes(item.kind));
      assert.equal(original.length, 1, 'the mapped data owner has one complete raw declaration');
      assert.equal(packaged.length, 1, 'the mapped data owner has one complete packaged declaration');
      assert.equal(packaged[0].kind, original[0].kind, 'library placement retains the actual data declaration kind');
      const from = 'crate::' + target, to = 'self::' + target;
      const actions = recordedStyle.actions.flatMap((action, actionIndex) => action.op === 'replace'
        && action.field === 'source' && action.from === from && action.to === to ? [{ action, actionIndex }] : []);
      assert.equal(actions.length, 1, 'the existing writer records this exact local type owner relocation once');
      edits.push({ start: tokens[index].start, end: tokens[index + 2].end, text: to });
      relocations.push({ source: mapped[0].owner.fullName, target, kind: original[0].kind, from, to,
        styleId: recordedStyle.id, actionIndex: actions[0].actionIndex,
        rawDeclarationSha256: digest(original[0].source), packagedDeclarationSha256: digest(packaged[0].source) });
    }
    let result = value;
    for (const edit of edits.sort((left, right) => right.start - left.start)) {
      result = result.slice(0, edit.start) + edit.text + result.slice(edit.end);
    }
    return { result, relocations };
  };
  for (const [index, owner] of owners.entries()) {
    const target = mappings[index].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u, 'one actual flat library owner');
    const declarations = items.filter(item => item.name === target
      && (owner.kind === 'fn' ? item.kind === 'fn' : ['enum', 'struct'].includes(item.kind)));
    assert.equal(declarations.length, 1, 'packaging retains every actual source/continuation owner ' + owner.fullName);
    if (owner.kind !== 'fn') continue;
    const block = declarations[0].source;
    const raw = readSourceTextRustBlock(emitted.text, 'fn', target);
    const header = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(block);
    const original = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(raw);
    const parameterStyles = readPortableSourceParameterStyles(raw);
    const recorded = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(parameterStyles.source);
    assert.ok(header && original && recorded, 'complete raw, shared-loan and packaged signatures are present');
    const before = nativeParameters(original[1]), parameters = nativeParameters(header[1]);
    assert.deepEqual(parameters, nativeParameters(recorded[1]),
      'every source and continuation interface uses exactly the existing shared loan decision');
    if (!owner.generated) {
      const immutable = before.map(parameter => parameter.replace(/^(?:mut )?(\w+):Vec<(f64|String|bool)>$/u, '$1:&[$2]')
        .replace(/^(?:mut )?(\w+):String$/u, '$1:&str'));
      assert.deepEqual(parameters, immutable, 'original primitive columns and string inputs are borrowed without changing scalar parameters');
    }
    const resultProof = relocateReturnType(original[2]);
    assert.equal(header[2].replace(/\s+/gu, ''), resultProof.result.replace(/\s+/gu, ''),
      'the complete return type changes only through authenticated recorded local data owner relocation');
    const vectors = new Set(parameters.flatMap(parameter => /^(\w+):&\[(?:f64|String|bool)\]$/u.exec(parameter)?.[1] ?? []));
    rejectInputCopies(block, vectors);
    interfaces.push({ source: owner.fullName, target, generated: owner.generated, parameters,
      immutableVectorInputs: [...vectors], result: header[2].trim(), rawResult: original[2].trim(),
      resultTypeRelocations: resultProof.relocations, signature: header[0].slice(0, -1).trim() });
  }
  assert.equal(interfaces.length, owners.filter(owner => owner.kind === 'fn').length,
    'the ABI inventory contains every original and generated function');
  assert.ok(interfaces.some(item => !item.generated && item.parameters.some(parameter => /^\w+:&\[f64\]$/u.test(parameter))),
    'the actual original numeric input column remains borrowed');
  assert.ok(interfaces.some(item => !item.generated && item.parameters.some(parameter => /^\w+:&\[String\]$/u.test(parameter))),
    'the actual original mapping column remains borrowed');
  return { ownerMappings: mappings, interfaces,
    libraryStyle: { path: stylePath, bytes: Buffer.byteLength(styleSource), sourceSha256: digest(styleSource) },
    files: files.map(file => ({
      path: file.path ?? libraryName + '.rs', bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })) };
}

export async function generateGrammarSelectionFieldRangeRules({ check = false, native = false } = {}) {
  const closed = readGrammarSelectionFieldRangeSource();
  const policy = readGrammarSelectionFieldRangePolicy(closed);
  for (const relative of ['js/src/translation/data/grammar-selection-field-range-rules.lino',
    'rust/src/data/grammar-selection-field-range-rules.lino']) {
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
    const files = writeRecordRuleLibrary(emitted, libraryName, { check, sourceStyles: readGrammarSelectionRangeLibraryStyles });
    const library = readGrammarSelectionFieldRangeLibraryInterface(files, emitted, owners);
    artifact = { schemaVersion: 1, model: policy.model, source: closed.source,
      sourceSha256: closed.sourceSha256, sourceClosure: closed.sourceClosure,
      inventory, checkedNamedSourceInventory: owners, replays,
      archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: digest(text),
        binaryBytes: binary.byteLength, binarySha256: digest(binary) },
      emitted: { ...emitted, definitions: emitted.definitions, preludes: emitted.preludes },
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: digest(emitted.text), library,
      boundary: 'Actual complete source capture, whole-program archive rechecks, raw Rust emission and recorded library packaging. Native compilation/execution, parser-host parity and language-CST validity require their actual CI/runtime assertions.' };
    const output = new URL('parity/self-translation/grammar-selection-field-range-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)),
    'only complete source-generation/freshness options are admitted');
  const result = await generateGrammarSelectionFieldRangeRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ phase: 'complete field-range source generation', sourceBytes: result.policy.sourceBytes,
    sourceSha256: result.policy.sourceSha256, nativeSource: result.artifact === null ? null : {
      bytes: result.artifact.emittedTextBytes, sha256: result.artifact.emittedTextSha256,
      owners: result.artifact.checkedNamedSourceInventory.length,
      interfaces: result.artifact.library.interfaces }, nativeCompilation: false, nativeExecution: false }, null, 2));
}
