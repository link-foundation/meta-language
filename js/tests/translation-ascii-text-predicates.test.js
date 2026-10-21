import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { readSourceTextRustBlock } from '../scripts/source-text-byte-library-style.mjs';
import { generateAsciiTextPredicateRules, readAsciiTextPredicateSourceClosure } from '../scripts/generate-ascii-text-predicate-rules.mjs';
import { LinkNetwork, LinkType, encodeLinkNetworkBinary, decodeLinkNetworkBinary,
  createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/index.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { tokenize } from '../src/translation/lexer.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { acceptAsciiCharacterMethod, acceptRustAsciiCharacterOwner,
  planRustAsciiTextQuery, planRustAsciiCharacterCallback } from '../src/translation/ascii-text-predicate-rules.js';

const root = new URL('../../', import.meta.url);
const policy = readProjectData(new URL('../src/translation/data/ascii-text-predicate-rules.lino', import.meta.url));
const digest = value => createHash('sha256').update(value).digest('hex');
const upper = value => value >= 65 && value <= 90;
const lower = value => value >= 97 && value <= 122;
const digit = value => value >= 48 && value <= 57;
const methods = ['is_ascii_alphabetic', 'is_ascii_alphanumeric', 'is_ascii_uppercase', 'is_ascii_lowercase', 'is_ascii_digit'];
const reloads = network => [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
const facts = source => {
  const tokens = tokenize(source, 'Rust').tokens;
  return { kinds: tokens.map(token => token.kind), values: tokens.map(token => token.value),
    characters: tokens.map(token => token.kind === 'char' ? Number(token.value) : -1) };
};
const observe = (code, name, values, tail = '', globals = {}) => [...vm.runInNewContext(code + '\n'
  + JSON.stringify(values) + '.map(value => ' + name + '(value' + tail + '));', globals, { timeout: 1000 })];
const nodes = (value, predicate) => !value || typeof value !== 'object' ? []
  : [...(predicate(value) ? [value] : []), ...Object.values(value).flatMap(child => nodes(child, predicate))];
const inputs = ['', '_', 'a', 'Z', 'a0_', '0a', '-a', 'a-b', 'a\n', 'a\r\n', 'π', 'aπ', 'é', 'K', 'İ', 'a\u0301', '😀', 'a😀', '\0', '\u007f'];
for (let scalar = 0; scalar < 256; scalar += 1) inputs.push(String.fromCodePoint(scalar), 'A' + String.fromCodePoint(scalar));

function expectedIdentifier(value) {
  return value.length > 0 && [...value].every((point, index) => {
    const scalar = point.codePointAt(0);
    return upper(scalar) || lower(scalar) || scalar === 95 || (index > 0 && digit(scalar));
  });
}
function expectedTerminal(value) {
  return value.length > 0 && upper(value.codePointAt(0));
}

test('actual Node selects the recorded assertion range and reports uncovered runtime versions', () => {
  assert.deepEqual(policy.assertionVersions, { Rust: '>=1.90.0 <1.100.0', Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0' });
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok((major === 22 && minor >= 15) || major === 24,
    'uncovered Node version ' + process.versions.node + '; supported ' + policy.assertionVersions.Node);
});

test('all five ASCII methods match independent byte ranges and reject non-ASCII and invalid scalars', () => {
  for (let scalar = 0; scalar < 256; scalar += 1) {
    const expected = [upper(scalar) || lower(scalar), upper(scalar) || lower(scalar) || digit(scalar),
      upper(scalar), lower(scalar), digit(scalar)];
    for (const [index, method] of methods.entries()) assert.equal(acceptAsciiCharacterMethod(method, scalar), expected[index]);
  }
  for (const scalar of [-1, 0.5, NaN, Infinity, -Infinity, 0x100, 0x3c0, 0x17f, 0x212a, 0xd800, 0xdc00, 0x1f600, 0x10ffff]) {
    for (const method of methods) assert.equal(acceptAsciiCharacterMethod(method, scalar), false);
  }
  for (const method of ['is_alphabetic', 'is_ascii_whitespace', '', 'is_ascii_digit()']) {
    assert.equal(acceptAsciiCharacterMethod(method, 65), false);
  }
});

test('pure callback plans retain actual closing and annotation positions without altering token facts', () => {
  for (const source of ["|c:char| c.is_ascii_alphabetic() || c == '_')", "|point| point == '_' || point.is_ascii_digit())",
    "|c| c == '\\x00' || c == '\\u{7f}')"]) {
    const input = facts(source), before = structuredClone(input);
    const plan = planRustAsciiCharacterCallback(input.kinds, input.values, input.characters, 0);
    assert.ok(plan.length > 2);
    assert.equal(input.values[plan[0]], ')');
    assert.equal(plan[1], source.includes(':char') ? 3 : -1);
    assert.deepEqual(input, before);
    assert.deepEqual(plan.slice(2), [...new Set(plan.slice(2))].sort((left, right) => left - right));
  }
  for (const source of ["|c| other.is_ascii_alphabetic())", "|c| c.is_ascii_alphabetic(1))", "|c| side_effect())",
    "|mut c| c.is_ascii_alphabetic())", "|true| true.is_ascii_alphabetic())", "|c:Custom| c.is_ascii_alphabetic())",
    "|c| { c.is_ascii_alphabetic() })", "|c| c.is_ascii_alphabetic() && true)", "|c| c == 'π')", "|c| c == '😀')",
    "|c| c == b'_')", "|c,d| c.is_ascii_digit())", "|| true)", "move |c| c.is_ascii_digit())",
    "|c| c.is_ascii_digit() ||)", "|c| c.is_ascii_digit(),false)", "|c| c.is_alphabetic())"]) {
    const input = facts(source);
    assert.deepEqual(planRustAsciiCharacterCallback(input.kinds, input.values, input.characters, 0), [], source);
  }
  const input = facts('|c| c.is_ascii_digit())');
  for (const start of [-1, 0.5, NaN, Infinity, input.kinds.length]) {
    assert.deepEqual(planRustAsciiCharacterCallback(input.kinds, input.values, input.characters, start), []);
    assert.deepEqual(planRustAsciiTextQuery('chars', input.kinds, input.values, start), []);
  }
  assert.deepEqual(planRustAsciiCharacterCallback(input.kinds.slice(1), input.values, input.characters, 0), []);
  assert.deepEqual(planRustAsciiCharacterCallback(input.kinds, input.values, input.characters.slice(1), 0), []);
  assert.equal(acceptRustAsciiCharacterOwner(true, 'char', false), true);
  assert.equal(acceptRustAsciiCharacterOwner(false, '', true), true, 'the proven standard query owns inferred char');
  assert.equal(acceptRustAsciiCharacterOwner(true, 'foreign::char', false), false);
  assert.equal(acceptRustAsciiCharacterOwner(true, 'char', true), false);
});

test('query plans select only the two closed syntax shapes and preserve unrelated method routing', () => {
  for (const [method, source, mode] of [['starts_with', '(|c| c.is_ascii_digit())', 1],
    ['chars', '().all(|c| c.is_ascii_digit())', 2], ['chars', '().map(|c| c)', 0],
    ['bytes', '().all(|c| true)', 0], ['starts_with', '("a")', 0], ['starts_with', '(|| true)', 0]]) {
    const input = facts(source), before = structuredClone(input);
    const plan = planRustAsciiTextQuery(method, input.kinds, input.values, 0);
    assert.equal(plan[0] ?? 0, mode);
    if (mode) assert.equal(input.values[plan[1]], '|');
    assert.deepEqual(input, before);
  }
  const source = 'fn selected(value:&str)->bool{value.starts_with("a") && value.ends_with("z") && value.contains("b")}fn main(){}';
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseRust(source))).text, 'selected', ['abz', 'ab', '', 'πabz']), [true, false, false, false]);
});

test('both complete actual project helpers retain module hashes and independent behavior through text binary and reverse archives', () => {
  assert.deepEqual(policy.originalDeclarations.map(owner => [owner.path, owner.name]), [
    ['rust/src/grammar/emit/tree_sitter_json.rs', 'is_identifier'],
    ['rust/src/grammar/emit/lark.rs', 'is_terminal_name'],
  ]);
  for (const owner of policy.originalDeclarations) {
    const context = readFileSync(new URL(owner.path, root), 'utf8');
    assert.equal(context, owner.context);
    assert.equal(Buffer.byteLength(context), owner.bytes);
    assert.equal(digest(context), owner.sourceSha256);
    assert.equal(readSourceTextRustBlock(context, 'fn', owner.name), owner.declaration);
    assert.equal(digest(owner.declaration), owner.declarationSha256);
    assert.ok(owner.declaration.startsWith('fn ' + owner.name + '('));
    const expected = inputs.map(owner.name === 'is_identifier' ? expectedIdentifier : expectedTerminal);
    const source = owner.declaration + '\nfn main(){}';
    const program = checkProgram(parseRust(source));
    const output = emitJavaScript(program);
    assert.deepEqual(observe(output.text, owner.name, inputs), expected);
    const packet = createTypedProgramLinks(source, 'Rust');
    for (const network of reloads(packet.network)) {
      assert.ok(network.links().every(link => !link.metadata().definition?.includes(owner.declaration)), 'semantic links do not contain a hidden source program');
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      const javascript = emitJavaScript(restored).text;
      assert.deepEqual(observe(javascript, owner.name, inputs), expected);
      assert.deepEqual(observe(emitTypedProgramLinks(network, packet.root, 'JavaScript').code, owner.name, inputs), expected);
      const reverseSource = javascript;
      const reverse = createTypedProgramLinks(reverseSource, 'JavaScript');
      for (const reverseNetwork of reloads(reverse.network)) {
        const reversed = recheckLinkedTypedProgram(decodeTypedProgramLinks(reverseNetwork, reverse.root), { wholeProgram: true });
        const native = emitRust(reversed, { arrayConsumption: true });
        assert.deepEqual(observe(emitJavaScript(reversed).text, owner.name, inputs), expected);
        // Definitions alone omit the generated runtime and are not a complete
        // standalone source interpretation. Preserve this exact input and its
        // explicit boundary; CI executes the complete native.text on all inputs.
        const definitionsOnly = native.definitions.join('\n\n') + '\nfn main(){}';
        if (owner.name === 'is_identifier') {
          const closure = tokenize(definitionsOnly, 'Rust').tokens.find(token => token.value === 'move');
          assert.ok(closure);
          assert.throws(() => checkProgram(parseRust(definitionsOnly)), error => error.kind === 'unsupported'
            && error.details.construct === 'closure'
            && error.reason === 'closure: higher-order values are outside the portable core'
            && error.span.start === closure.start && error.span.end === closure.end);
        } else {
          const reparsed = checkProgram(parseRust(definitionsOnly));
          assert.deepEqual(observe(emitJavaScript(reparsed).text, owner.name, inputs), expected);
        }
        const completeNative = LinkNetwork.parse(native.text, 'Rust');
        assert.ok(completeNative.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing));
        assert.deepEqual(Buffer.from(completeNative.reconstructText()), Buffer.from(native.text));
        for (const restored of reloads(completeNative)) assert.deepEqual(Buffer.from(restored.reconstructText()), Buffer.from(native.text));
        assert.deepEqual(emitRust(recheckLinkedTypedProgram(decodeTypedProgramLinks(reverseNetwork, reverse.root), { wholeProgram: true }),
          { arrayConsumption: true }).text, native.text);
      }
    }
  }
});

test('ordinary default Rust CST of both real helpers reconstructs every comment whitespace and source byte through both codecs', () => {
  for (const owner of policy.originalDeclarations) {
    const source = '// actual owner π: ' + owner.path + '\n' + owner.declaration + '\n';
    const concrete = LinkNetwork.parse(source, 'Rust');
    assert.ok(concrete.links().some(link => link.metadata().term === 'function_item'));
    assert.ok(concrete.links().some(link => link.metadata().term === 'closure_expression'));
    assert.ok(concrete.links().some(link => link.metadata().linkType === LinkType.SourceToken));
    assert.ok(concrete.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing));
    assert.deepEqual(Buffer.from(concrete.reconstructText()), Buffer.from(source));
    for (const restored of reloads(concrete)) assert.deepEqual(Buffer.from(restored.reconstructText()), Buffer.from(source));
  }
});

test('first and all preserve one receiver evaluation hygiene empty-input semantics and Unicode refusal', () => {
  const source = 'fn provider(value:&str)->&str{value}\n'
    + 'fn first(value:&str)->bool{provider(value).starts_with(|c|c.is_ascii_digit())}\n'
    + 'fn all(value:&str)->bool{provider(value).chars().all(|c|c.is_ascii_digit())}\n'
    + 'fn selected(ascii_predicate_points0:&str,ascii_predicate_point0:bool)->bool{ascii_predicate_points0.chars().all(|c|c.is_ascii_digit()) && ascii_predicate_point0}\nfn main(){}';
  const program = checkProgram(parseRust(source));
  for (const name of ['first', 'all']) {
    assert.equal(nodes(program.declarations.get(name).body, node => node.k === 'call' && node.fn === 'provider').length, 1);
  }
  assert.equal(nodes(program.declarations.get('first').body, node => node.k === 'stringPoints').length, 0,
    'first never scans or collects the complete string');
  assert.equal(nodes(program.declarations.get('all').body, node => node.k === 'stringPoints').length, 1,
    'the explicitly recorded all profile retains its eager point collection');
  const generated = program.declarations.get('selected');
  assert.ok(nodes(generated.body, node => node.k === 'let').every(node => node.name !== 'ascii_predicate_points0'));
  assert.ok(nodes(generated.body, node => node.k === 'lambda').every(node => node.params.every(param => param.name !== 'ascii_predicate_point0')));
  const code = emitJavaScript(program).text;
  const instrumented = code.replace('function provider(value) {', 'function provider(value) { counter.calls += 1;');
  assert.notEqual(instrumented, code);
  const values = ['', '1', '12', '1x', 'x1', 'π', '1π', '😀', '1😀', '1' + 'x'.repeat(100_000)];
  const counter = { calls: 0 };
  assert.deepEqual(observe(instrumented, 'first', values, '', { counter }), values.map(value => value.length > 0 && digit(value.codePointAt(0))));
  assert.equal(counter.calls, values.length);
  assert.deepEqual(observe(instrumented, 'all', values, '', { counter }), values.map(value => [...value].every(point => digit(point.codePointAt(0)))));
  assert.equal(counter.calls, values.length * 2);
  const ordinary = ['', '0', '00', '0a', 'π'];
  assert.deepEqual(observe(code, 'selected', ordinary, ',true'), [true, true, true, false, false]);
  assert.deepEqual(observe(code, 'selected', ordinary, ',false'), [false, false, false, false, false]);
});

test('actual receiver owners primitive annotations aliases captures effects arities and const boundaries refuse', () => {
  const callbackBodies = ['other.is_ascii_digit()', 'side_effect()', '{ c.is_ascii_digit() }',
    'c.is_ascii_digit(1)', 'c.is_alphabetic()', "c == 'π'", "c == b'_'", 'c.is_ascii_digit() && true', 'c.is_ascii_digit() ||', 'c.is_ascii_digit(),false'];
  for (const callback of callbackBodies) assert.throws(() => checkProgram(parseRust(
    'fn selected(value:&str)->bool{value.starts_with(|c|' + callback + ')}fn main(){}')), /ASCII|outside|expected|unsupported/u, callback);
  for (const source of [
    'fn selected(value:String)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:&&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:bool)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}',
    'enum str{Value} fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}',
    'use crate::foreign::str;fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}use crate::foreign::other as str;fn main(){}',
    'mod nested{use crate::foreign::str;fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}}fn main(){}',
    'enum char{Value} fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}fn main(){}',
    'use crate::foreign::char;fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}use crate::foreign::other as char;fn main(){}',
    'mod nested{use crate::foreign::other as char;fn selected(value:&str)->bool{value.chars().all(|c:char|c.is_ascii_digit())}}fn main(){}',
    'fn selected(value:&str)->bool{value.starts_with(|mut c|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:&str)->bool{value.chars().all(|c,d|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:&str)->bool{value.chars(1).all(|c|c.is_ascii_digit())}fn main(){}',
    'fn selected(value:&str)->bool{value.bytes().all(|c|c.is_ascii_digit())}fn main(){}',
    'const fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}',
  ]) assert.throws(() => checkProgram(parseRust(source)), /ASCII|outside|expected|unsupported|char|const/u, source);
});

test('unannotated standard char and separate function namespaces do not acquire an unrelated type owner', () => {
  for (const prefix of ['enum char{Value}', 'fn char(value:bool)->bool{value}', 'fn str(value:bool)->bool{value}']) {
    const source = prefix + 'fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}';
    assert.deepEqual(observe(emitJavaScript(checkProgram(parseRust(source))).text, 'selected', ['', '0', 'a', 'π']), [false, true, false, false]);
  }
  const source = 'fn char(value:bool)->bool{value}fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}fn main(){}';
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseRust(source))).text, 'selected', ['', '0', 'a', 'π']), [false, true, false, false]);
});

test('source-free set edits change actual accepted characters after complete semantic rechecking', () => {
  const packet = createTypedProgramLinks("fn selected(value:&str)->bool{value.chars().all(|c|c=='_')}fn main(){}", 'Rust');
  const original = ['', '_', '__', 'x', 'xx', '_x', 'π'];
  const expected = [true, true, true, false, false, false, false];
  assert.deepEqual(observe(emitTypedProgramLinks(packet.network, packet.root, 'JavaScript').code, 'selected', original), expected);
  const literal = packet.network.links().find(link => link.metadata().definition === '_');
  assert.ok(literal);
  const edited = LinkNetwork.fromLino(packet.network.toLino().split('\n').map(line => line.startsWith('(' + literal.id() + ': ')
    ? line.replace('(def: _)', '(def: x)') : line).join('\n'));
  assert.notEqual(edited.toLino(), packet.network.toLino());
  for (const network of reloads(edited)) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual(observe(emitJavaScript(restored).text, 'selected', original), [true, false, false, true, true, false, false]);
  }
});

test('the complete dependency closure all generated owners actual ABI and both archive replays are fresh', async () => {
  const result = await generateAsciiTextPredicateRules({ check: true, native: true });
  assert.deepEqual(result.policy, policy);
  assert.deepEqual(result.policy.sourceClosure, readAsciiTextPredicateSourceClosure());
  assert.deepEqual(result.policy.sourceClosure.inputs.map(input => input.path), [
    'js/src/translation/rust-const-function-rules.js', 'js/src/translation/ascii-text-predicate-rules.js']);
  for (const input of result.policy.sourceClosure.inputs) {
    const actual = readFileSync(new URL(input.path, root), 'utf8');
    assert.equal(actual, input.source);
    assert.equal(Buffer.byteLength(actual), input.bytes);
    assert.equal(digest(actual), input.sourceSha256);
  }
  assert.deepEqual(result.artifact.replays.map(replay => replay.format), ['text', 'binary']);
  assert.ok(result.artifact.libraryInterface.length >= result.policy.decisions.length);
  for (const decision of result.policy.decisions) assert.equal(result.artifact.libraryInterface.filter(row => row.source === decision).length, 1);
  assert.equal(result.artifact.checkedNamedSourceInventory.length, result.artifact.inventory.length);
});


test('actual JavaScript array literal spreads reuse Unicode iteration without changing internal array construction', () => {
  const source = '/** @param {string} value @returns {string[]} */function points(value){return [...value];}';
  const values = ['', 'A', 'Aπ', '😀', 'A😀Z', '\ud800', '\udc00', '\ud800x\udc00'];
  const expected = values.map(value => [...value]);
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const network of reloads(packet.network)) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    const code = emitJavaScript(program).text;
    assert.deepEqual(observe(code, 'points', values).map(value => [...value]), expected);
    assert.ok(emitRust(program, { arrayConsumption: true }).definitions.length > 0);
  }
  const wrong = '/** @param {number} value */function points(value){return [...value];}';
  assert.throws(() => createTypedProgramLinks(wrong, 'JavaScript'), /iterable|iterator|array|spread/u);
});
