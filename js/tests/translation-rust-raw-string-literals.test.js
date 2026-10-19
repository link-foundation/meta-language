import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import {
  LinkNetwork, createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks,
  emitTypedProgramLinks, encodeLinkNetworkBinary, decodeLinkNetworkBinary,
} from '../src/index.js';
import { tokenize } from '../src/translation/lexer.js';
import { decodeRustRawString } from '../src/translation/rust-raw-string-rules.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-raw-string-literals.lino', import.meta.url));
const units = text => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
const literalFor = ({ delimiters, content }) => 'r' + '#'.repeat(delimiters) + '"' + content + '"' + '#'.repeat(delimiters);
const sourceFor = literal => '\ufeff// 😀 original source\r\npub fn choose()->String{String::from(' + literal + ')}fn main(){println!("{}",choose());}';
const execute = source => {
  const output = [];
  vm.runInNewContext(source, { console: { log: value => output.push(String(value)) } }, { timeout: 1000 });
  return output;
};
const reloads = network => [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];

test('all zero through 255 raw delimiters close at the first sufficient quote without interpreting escapes', () => {
  for (let delimiters = 0; delimiters <= 255; delimiters += 1) {
    const content = delimiters === 0 ? String.raw`α😀\n\u{0}` : 'α😀\\n"' + '#'.repeat(delimiters - 1) + 'inside';
    const literal = literalFor({ delimiters, content });
    const text = '// 😀\n' + literal + '#';
    const start = text.indexOf('r');
    const decoded = decodeRustRawString(units(text), start);
    assert.deepEqual(decoded, { $: 'literal', contentStart: start + delimiters + 2, contentEnd: start + literal.length - delimiters - 1, end: start + literal.length });
    const tokens = tokenize(text, 'Rust').tokens;
    assert.deepEqual(tokens[0], { kind: 'string', value: content, start, end: start + literal.length, raw: literal });
    assert.equal(tokens[1].raw, '#');
  }
  const text = 'r#"first"#r#"second"#';
  assert.deepEqual(tokenize(text, 'Rust').tokens.filter(token => token.kind === 'string').map(token => token.value), ['first', 'second']);
  assert.deepEqual(decodeRustRawString(units('r""'), -1), { $: 'notRaw' });
  for (const start of [NaN, Infinity, -Infinity, 0.5, 3]) assert.deepEqual(decodeRustRawString(units('r""'), start), { $: 'notRaw' });
});

test('raw string original Unicode source and spans survive public CST text and binary archives byte for byte', () => {
  for (const entry of fixture.literalCases) {
    const literal = literalFor(entry), source = sourceFor(literal);
    const token = tokenize(source, 'Rust').tokens.find(item => item.kind === 'string' && item.raw === literal);
    assert.equal(token.value, entry.value, entry.name);
    assert.equal(token.start, source.indexOf(literal), entry.name);
    assert.equal(token.end, token.start + literal.length, entry.name);
    assert.equal(source.slice(token.start, token.end), token.raw);
    const byteStart = Buffer.byteLength(source.slice(0, token.start));
    assert.deepEqual(Buffer.from(source).subarray(byteStart, byteStart + Buffer.byteLength(literal)), Buffer.from(literal));
    const cst = LinkNetwork.parse(source, 'Rust');
    for (const network of reloads(cst)) assert.deepEqual(Buffer.from(network.reconstructText()), Buffer.from(source), entry.name);
  }
});

test('public source-free text and binary Links execute raw values and reparse the complete emitted JavaScript', () => {
  for (const entry of fixture.literalCases) {
    const source = sourceFor(literalFor(entry));
    const packet = createTypedProgramLinks(source, 'Rust');
    for (const network of reloads(packet.network)) {
      assert.ok(network.links().every(link => !link.metadata().definition?.includes(source)));
      const program = decodeTypedProgramLinks(network, packet.root);
      assert.equal(program.declarations.get('choose').body.value, entry.value, entry.name);
      const declarations = emitRust(program).definitions.join('\n\n');
      assert.ok(emitTypedProgramLinks(network, packet.root, 'Rust').code.includes(declarations));
      // Exercise every real emitted definition with the complete original main.
      // The generated native thread wrapper remains a separately selected model.
      const replay = checkProgram(parseRust(declarations + '\nfn main(){println!("{}",choose());}'));
      assert.deepEqual(execute(emitJavaScript(replay).text), [entry.value]);
      const javascript = emitTypedProgramLinks(network, packet.root, 'JavaScript').code;
      assert.deepEqual(execute(javascript), [entry.value], entry.name);
      const fromJavaScript = createTypedProgramLinks(javascript, 'JavaScript');
      for (const restored of reloads(fromJavaScript.network)) {
        assert.deepEqual(execute(emitTypedProgramLinks(restored, fromJavaScript.root, 'JavaScript').code), [entry.value]);
        const actualRust = emitTypedProgramLinks(restored, fromJavaScript.root, 'Rust').code;
        assert.ok(tokenize(actualRust, 'Rust').tokens.some(token => token.kind === 'string' && token.value === entry.value));
      }
    }
  }
});

test('a source-free raw string value edit changes both targets without retaining the original source', () => {
  const original = sourceFor(literalFor(fixture.literalCases[1]));
  const packet = createTypedProgramLinks(original, 'Rust');
  const program = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root);
  const expected = 'edited\\n\r\n\0"#α😀';
  program.declarations.get('choose').body.value = expected;
  const edited = encodeTypedProgramLinks(program);
  for (const network of reloads(edited.network)) {
    assert.ok(network.links().every(link => !link.metadata().definition?.includes(original)));
    assert.deepEqual(execute(emitTypedProgramLinks(network, edited.root, 'JavaScript').code), [expected]);
    const declarations = emitRust(decodeTypedProgramLinks(network, edited.root)).definitions.join('\n\n');
    const replay = checkProgram(parseRust(declarations + '\nfn main(){println!("{}",choose());}'));
    assert.deepEqual(execute(emitJavaScript(replay).text), [expected]);
  }
});

test('truncated or mismatched raw delimiters and isolated carriage returns refuse through actual source import', () => {
  for (const literal of fixture.malformed) {
    const expected = literal.startsWith('r' + '#'.repeat(256)) ? 'tooManyDelimiters' : 'unterminated';
    assert.equal(decodeRustRawString(units(literal), 0).$, expected);
    assert.throws(() => tokenize(literal, 'Rust'), error => error.kind === 'syntax' && /Rust raw string/u.test(error.reason));
    assert.throws(() => createTypedProgramLinks(sourceFor(literal), 'Rust'));
  }
  for (const content of fixture.isolatedCarriageReturns) {
    const source = sourceFor(literalFor({ delimiters: 2, content }));
    // The first source CR belongs to the original comment CRLF; locate the
    // isolated return only inside the actual raw token.
    const start = source.indexOf('\r', source.indexOf('r##"'));
    for (const operation of [() => tokenize(source, 'Rust'), () => createTypedProgramLinks(source, 'Rust')]) {
      assert.throws(operation, error => error.kind === 'syntax' && error.reason === 'isolated carriage return in a Rust string'
        && error.span.start === start && error.span.end === start + 1);
    }
  }
  for (const literal of ['r"value"suffix', 'r#"value"##']) assert.throws(() => createTypedProgramLinks(sourceFor(literal), 'Rust'));
  for (const invalidUnits of fixture.nonScalarUnits) {
    const content = String.fromCharCode(...invalidUnits);
    const source = sourceFor(literalFor({ delimiters: 1, content }));
    const start = source.indexOf(content) + (content.startsWith('a') ? 1 : 0);
    for (const operation of [() => tokenize(source, 'Rust'), () => createTypedProgramLinks(source, 'Rust')]) {
      assert.throws(operation, error => error.kind === 'syntax' && /unpaired Unicode surrogate/u.test(error.reason)
        && error.span.start === start && error.span.end === start + 1);
    }
  }
});

test('raw identifiers, byte and C literals and other languages retain their previous token paths', () => {
  for (const text of fixture.unchangedPrefixes) {
    assert.equal(decodeRustRawString(units(text), 0).$, 'notRaw', text);
    assert.equal(tokenize(text, 'Rust').tokens[0].kind, 'identifier', text);
    assert.throws(() => createTypedProgramLinks(sourceFor(text), 'Rust'), text);
  }
  for (const language of ['JavaScript', 'Lean', 'Rocq']) {
    const tokens = tokenize('r"value"', language).tokens;
    assert.equal(tokens[0].kind, 'identifier');
    assert.equal(tokens[0].value, 'r');
    assert.equal(tokens[1].raw, '"value"');
  }
});

test('equivalent raw fixed runtime facts retain actual owners and complete selected artifact replay', () => {
  const runtime = readProjectData(new URL('../../parity/fixtures/rust-runtime-shell.lino', import.meta.url));
  const entry = runtime.cases.find(row => row.id === 'ordered-repeated');
  const options = { rustRuntimeShellModel: runtime.model };
  const replacements = [['"--ml-check-theorems"', 'r"--ml-check-theorems"'], ['"spawn the program thread"', 'r#"spawn the program thread"#']];
  const operations = (node, result = []) => {
    if (node.kind === 'source operation') { result.push(node); node.parts.forEach(child => operations(child, result)); }
    return result;
  };
  for (const changes of [[replacements[0]], [replacements[1]], replacements]) {
    let source = entry.fullEmittedRust;
    for (const [before, after] of changes) { assert.ok(source.includes(before)); source = source.replace(before, after); }
    const packet = createTypedProgramLinks(source, 'Rust', options);
    for (const network of reloads(packet.network)) {
      const program = decodeTypedProgramLinks(network, packet.root), proof = program.sourceRuntimeShell;
      assert.equal(proof.span.end, source.length);
      assert.equal(proof.tokens.at(-1).span.end, source.length);
      const nodes = operations(proof.operations);
      for (const [, raw] of changes) {
        const token = proof.tokens.find(item => item.kind === 'string' && source.slice(item.span.start, item.span.end) === raw);
        assert.ok(token, raw + ' retains its original complete source extent');
        const owner = raw.includes('--ml-check-theorems') ? 'standard argument flag comparison' : 'standard thread success requirement';
        assert.ok(nodes.find(node => node.op === owner)?.parts.includes(token), raw + ' is the actual owned operation token');
      }
      for (const token of proof.tokens) {
        if (token.declaration) assert.equal(program.declarations.get(token.declaration.fullName), token.declaration);
        if (token.binding) assert.ok(proof.tokens.includes(token.binding));
      }
      assert.equal(proof.assumptions.length, 4);
      for (const assumption of proof.assumptions) assert.ok(nodes.includes(assumption.operationNode));
      const javascript = emitTypedProgramLinks(network, packet.root, 'JavaScript');
      const output = [], context = vm.createContext({ console: { log: value => output.push(String(value)) } });
      vm.runInContext(javascript.code, context, { timeout: 1000 });
      assert.equal(output.join('\n'), entry.output);
      const calls = [];
      for (const name of entry.names) context[name] = () => { calls.push(name); return null; };
      vm.runInContext('main();', context, { timeout: 1000 });
      assert.deepEqual(calls, entry.calls);
      const rust = emitTypedProgramLinks(network, packet.root, 'Rust');
      assert.equal(rust.assumptions.length, 4);
      const replay = createTypedProgramLinks(rust.code, 'Rust', options);
      for (const restored of reloads(replay.network)) {
        const rechecked = decodeTypedProgramLinks(restored, replay.root);
        assert.deepEqual(rechecked.main.effects.map(effect => effect.value.fn), entry.calls);
        assert.equal(rechecked.sourceRuntimeShell.span.end, rust.code.length);
        assert.doesNotThrow(() => recheckLinkedTypedProgram(rechecked, { wholeProgram: true }));
      }
    }
    for (const [before, after] of [['--ml-check-theorems', '--changed-check-theorems'], ['spawn the program thread', 'changed thread expectation']]) {
      const changed = source.replace(before, after);
      assert.notEqual(changed, source);
      assert.throws(() => createTypedProgramLinks(changed, 'Rust', options));
    }
    for (const mutate of [
      program => { program.sourceRuntimeShell.tokens.find(token => token.kind === 'string').value = 'changed fact'; },
      program => { const token = program.sourceRuntimeShell.tokens.find(item => item.declaration); token.declaration = structuredClone(token.declaration); },
      program => { const token = program.sourceRuntimeShell.tokens.find(item => item.binding); token.binding = structuredClone(token.binding); },
    ]) {
      const corrupted = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
      mutate(corrupted);
      assert.throws(() => encodeTypedProgramLinks(corrupted));
      assert.throws(() => recheckLinkedTypedProgram(corrupted, { wholeProgram: true }));
    }
  }
});

test('the generated linked raw-string policy, fixtures and whole portable decoder remain fresh', () => {
  execFileSync(process.execPath, ['js/scripts/generate-rust-raw-string-rules.mjs', '--check', '--javascript-only'], {
    cwd: fileURLToPath(new URL('../../', import.meta.url)), stdio: 'pipe',
  });
});
