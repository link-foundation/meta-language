// Complete source closure and ordered source-owner capture. This writer emits
// Rust via the checked translator and never supplies replacement algorithms.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { tokenize } from '../src/translation/lexer.js';

const root = new URL('../../', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');
const sourcePath = 'js/src/translation/javascript-record-parameter-rules.js';
const inputs = [
  ['js/src/translation/fact-column-rules.js', []],
  ['js/src/translation/array-binding-rules.js', ["import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n"]],
  [sourcePath, ["import { acceptJavaScriptArrayBindingName } from './array-binding-rules.js';\n",
    "import { readRequiredTextFact, readRequiredBooleanFact } from './fact-column-rules.js';\n"]],
];
const decisions = ['findJavaScriptRecordParameterPattern', 'acceptJavaScriptRecordParameterBinding',
  'planJavaScriptRecordParameters', 'acceptJavaScriptRecordParameterProfile',
  'acceptJavaScriptStandardBuiltinOwner', 'findJavaScriptRecordParameterStrictDirective'];
const owners = [
  ['js/src/grammar-interchange.js', 'formatsCommand'],
  ['js/src/decorators.js', 'renderDecorator'],
  ['js/src/grammar-emitters/pest.js', 'emitRepeat'],
  ['js/src/grammar-emitters/tree-sitter-json.js', 'emitRepeat'],
  ['js/src/program-binding-extents.js', 'resolveRustPath'],
];

export function readJavaScriptRecordParameterSourceClosure() {
  const captures = [], parts = [];
  for (const [path, imports] of inputs) {
    const source = readFileSync(new URL(path, root), 'utf8');
    const removedImports = [];
    for (const statement of imports) {
      assert.equal(source.split(statement).length, 2, 'one actual complete dependency import');
      const start = source.indexOf(statement), end = start + statement.length;
      removedImports.push({ statement, start, end, coordinateUnit: 'UTF-16 code units' });
    }
    let body = source;
    for (const interval of [...removedImports].sort((left, right) => right.start - left.start)) {
      assert.equal(source.slice(interval.start, interval.end), interval.statement);
      body = body.slice(0, interval.start) + body.slice(interval.end);
    }
    assert.doesNotMatch(body, /^\s*import\b/mu, 'an unknown actual import must not be dropped');
    captures.push({ path, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), removedImports });
    parts.push(body);
  }
  const source = parts.join('\n\n');
  return { source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), inputs: captures };
}

export function readJavaScriptRecordParameterDeclaredOwners(source) {
  const tokens = tokenize(source, 'JavaScript').tokens;
  const names = tokens.flatMap((token, index) => token.kind === 'identifier' && token.value === 'function'
    && tokens[index + 1]?.kind === 'identifier' && tokens[index + 2]?.value === '(' ? [tokens[index + 1].value] : []);
  assert.equal(names.length, new Set(names).size, 'the complete dependency closure has distinct actual declaration owners');
  return names;
}

export function readOriginalJavaScriptRecordParameterDeclaration(path, name) {
  const context = readFileSync(new URL(path, root), 'utf8');
  const tokens = tokenize(context, 'JavaScript').tokens, stack = [], matching = new Map();
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token.kind !== 'punct') continue;
    if (['(', '[', '{'].includes(token.value)) stack.push(index);
    else if ([')', ']', '}'].includes(token.value)) {
      const open = stack.pop();
      assert.ok(open !== undefined && tokens[open].value === ({')':'(',']':'[','}':'{'})[token.value]);
      matching.set(open, index);
    }
  }
  assert.equal(stack.length, 0);
  const declarations = tokens.flatMap((token, index) => token.value === 'function' && token.kind === 'identifier'
    && tokens[index + 1]?.value === name && tokens[index + 2]?.value === '(' ? [index] : []);
  assert.equal(declarations.length, 1, 'one actual complete named source owner');
  const at = declarations[0], body = matching.get(at + 2) + 1;
  assert.equal(tokens[body]?.value, '{');
  const end = matching.get(body);
  assert.ok(end > body);
  const start = tokens[at - 1]?.value === 'export' ? at - 1 : at;
  const declaration = context.slice(tokens[start].start, tokens[end].end);
  return { path, name, context, bytes: Buffer.byteLength(context), sourceSha256: digest(context),
    declaration, declarationSha256: digest(declaration),
    range: { start: tokens[start].start, end: tokens[end].end, coordinateUnit: 'UTF-16 code units' } };
}

export async function generateJavaScriptRecordParameterRules({ check = false, native = false } = {}) {
  const sourceClosure = readJavaScriptRecordParameterSourceClosure();
  assert.deepEqual([...sourceClosure.inputs.at(-1).source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]), decisions);
  const policy = { schemaVersion: 1, model: 'ordered required JavaScript object parameter field projections 1', sourcePath, decisions, sourceClosure,
    originalDeclarations: owners.map(([path, name]) => readOriginalJavaScriptRecordParameterDeclaration(path, name)),
    assertionVersions: { Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0' },
    admission: 'Ordinary synchronous named function declarations, nonempty flat required identifier properties with aliases, fixed parameters and no initializer anywhere in the list. Actual lexical BoundNames are unique. The first body profile has no writes to any actual whole-list parameter name, captured writes, var declarations, parameter documentation tags or actual Use Strict Directive. Standard Map, Set and console normalization requires an unshadowed actual lexical/module/context owner; the original four decision interfaces remain unchanged.',
    observation: 'Actual checked receivers supply existing portable field meanings: represented record fields, String or Array length, and Map or Set size. Selection follows the actual checked owner and operation, never the property spelling alone. All actual arguments evaluate before callee entry; entry initializes each actual parameter/pattern binding in original order. One logical argument stays one argument. Present null fields remain values. Nonempty required field operations reject missing/nullish receivers through the existing checker. Source syntax/type inference alone establishes no accessor, proxy, inherited-field or unknown-external invocation owner.',
    specificationContext: [
      { url: 'https://tc39.es/ecma262/multipage/abstract-operations.html#sec-getv', clause: 'GetV',
        inference: 'A nonempty required pattern can use the existing checked field projection for its actual receiver; the operation alone proves no arbitrary external JS owner.' },
      { url: 'https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-runtime-semantics-propertybindinginitialization', clause: 'PropertyBindingInitialization',
        inference: 'Retain each original property read in pattern order after argument evaluation.' },
      { url: 'https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-runtime-semantics-keyedbindinginitialization', clause: 'KeyedBindingInitialization',
        inference: 'Existing field values initialize each binding; defaults remain a separate undefined-only, ordered callee-entry obligation.' },
    ],
    boundary: 'Ordinary CST is unchanged. Selected unsupported forms refuse. Anonymous arrows, function values, namespace methods, defaults, nested/rest/computed/string-key/empty patterns, mutable initialization and default-environment composition remain separate obligations. Captured whole own-source declarations establish provenance/header/CST coverage, not full body translation or native execution.' };
  for (const path of ['js/src/translation/data/javascript-record-parameter-rules.lino', 'rust/src/data/javascript-record-parameter-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy); else writeProjectData(new URL(path, root), policy);
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
    assert.deepEqual(inventory.filter(owner => !owner.generated && owner.kind === 'fn').map(owner => owner.name).sort(),
      readJavaScriptRecordParameterDeclaredOwners(sourceClosure.source).sort(),
      'every actual declaration in every complete dependency module remains an owned checked source declaration');
    const checkedOwners = program => inventory.map(owner => {
      const declaration = program.declarations.get(owner.name);
      assert.ok(declaration && declaration.k === owner.kind);
      return { ...owner, fullName: declaration.fullName };
    });
    const checked = checkProgram(surface), checkedNamedSourceInventory = checkedOwners(checked);
    assert.equal(checkedNamedSourceInventory.length, checked.declarations.size);
    const output = emitRust(checked, { arrayConsumption: true });
    const emitted = { text: output.text, definitions: output.definitions, preludes: output.preludes,
      mappings: output.mappings, assumptions: output.assumptions, encodings: output.encodings };
    for (const owner of checkedNamedSourceInventory) assert.equal(emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind)).length, 1);
    const packet = encodeTypedProgramLinks(checked), replays = [];
    for (const [format, network] of [['text', LinkNetwork.fromLino(packet.network.toLino())],
      ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.deepEqual(checkedOwners(restored), checkedNamedSourceInventory);
      const replay = emitRust(restored, { arrayConsumption: true });
      for (const field of Object.keys(emitted)) assert.deepEqual(replay[field], emitted[field]);
      replays.push({ format, checkedNamedSourceInventory: checkedOwners(restored), emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, 'javascript_record_parameter_rules', { check });
    const libraryInterface = files.flatMap(file => {
      const tokens = tokenize(file.source, 'Rust').tokens;
      return tokens.flatMap((token, index) => {
        if (token.value !== 'fn' || tokens[index + 1]?.kind !== 'identifier') return [];
        const name = tokens[index + 1].value;
        const owner = emitted.mappings.find(mapping => mapping.target === name && mapping.kind === 'function');
        if (!owner) return [];
        let body = index + 2;
        while (body < tokens.length && tokens[body].value !== '{') body += 1;
        assert.ok(body < tokens.length);
        return [{ source: owner.source, target: name, path: file.path ?? 'javascript_record_parameter_rules.rs',
          signature: file.source.slice(token.start, tokens[body].start).trim() }];
      });
    });
    for (const decision of decisions) assert.equal(libraryInterface.filter(row => row.source === decision).length, 1);
    artifact = { schemaVersion: 1, model: policy.model, sourceClosure, inventory, checkedNamedSourceInventory,
      emitted, replays, libraryInterface,
      files: files.map(file => ({ path: file.path ?? 'javascript_record_parameter_rules.rs', source: file.source,
        bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })),
      boundary: 'Complete source generation and actual native ABI inventory; no native compiler or execution observation. Native frontend transport must be authored only after this actual ABI is inspected.' };
    const file = new URL('parity/self-translation/javascript-record-parameter-rules-native.lino', root);
    if (check) checkProjectData(file, artifact); else writeProjectData(file, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)));
  const result = await generateJavaScriptRecordParameterRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ decisions, checkedOwners: result.artifact?.checkedNamedSourceInventory.length ?? null,
    nativeCompilation: false, nativeExecution: false }));
}
