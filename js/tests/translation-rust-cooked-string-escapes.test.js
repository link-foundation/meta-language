import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from '../src/index.js';
import { tokenize } from '../src/translation/lexer.js';
import { decodeUnicodeEscape } from '../src/translation/frontend-rules.js';
import { acceptRustCookedStringEscapePrefix, readRustAsciiEscapeDigit, readRustAsciiEscapeValue } from '../src/translation/rust-cooked-string-rules.js';
import { acceptRustCookedStringEscapePrefix as draftPrefix } from '../../experiments/issue-199-rust-string-continuation-rules.mjs';
import { checkProgram } from '../src/translation/check.js';
import { parseRust } from '../src/translation/rust.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-cooked-string-escapes.lino', import.meta.url));
const project = fileURLToPath(new URL('../../', import.meta.url));
const asciiEscape = code => '\\x' + code.toString(16).padStart(2, '0');
const sourceFor = text => '// 😀 α\nfn choose()->String{String::from("' + text + '")}fn main(){println!("{}",choose());}';
const execute = source => {
  const output = [];
  vm.runInNewContext(source, { console: { log: value => output.push(String(value)) } }, { timeout: 1000 });
  return output;
};
const reload = packet => ({ root: packet.root, network: LinkNetwork.fromLino(packet.network.toLino()) });

test('all 128 ASCII escapes preserve exact values and original Unicode source offsets', () => {
  assert.equal(fixture.ascii.length, 128);
  for (const code of fixture.ascii) {
    for (const escape of [asciiEscape(code), asciiEscape(code).replace(/[a-f]/gu, letter => letter.toUpperCase())]) {
      const literal = '"α😀' + escape + 'β"';
      const source = '// 😀\n' + literal;
      const token = tokenize(source, 'Rust').tokens[0];
      assert.equal(token.value, 'α😀' + String.fromCharCode(code) + 'β', escape);
      assert.equal(token.raw, literal);
      assert.deepEqual({ start: token.start, end: token.end }, { start: source.indexOf(literal), end: source.length });
      const byteStart = Buffer.byteLength(source.slice(0, token.start));
      assert.deepEqual(Buffer.from(source).subarray(byteStart), Buffer.from(literal));
    }
  }
  for (const [index, escape] of fixture.suffixEscapes.entries()) assert.equal(tokenize('"' + escape + '"', 'Rust').tokens[0].value, fixture.suffixValues[index], escape);
  for (const [index, escape] of fixture.simpleEscapes.entries()) assert.equal(tokenize('"' + escape + '"', 'Rust').tokens[0].value, fixture.simpleValues[index], escape);
  assert.equal(tokenize('"\\x0a  b"', 'Rust').tokens[0].value, '\n  b');
  assert.equal(tokenize('"a\\\n  b"', 'Rust').tokens[0].value, 'ab');
});

test('public Rust import refuses every non-ASCII hex value and malformed escape with original spans', () => {
  assert.equal(fixture.nonAscii.length, 128);
  const invalid = [...fixture.nonAscii.map(asciiEscape), ...fixture.invalidEscapes];
  for (const escape of invalid) {
    const source = sourceFor(escape);
    const start = source.indexOf('\\');
    for (const operation of [() => tokenize(source, 'Rust'), () => createTypedProgramLinks(source, 'Rust')]) {
      assert.throws(operation, error => error.kind === 'syntax'
        && /unsupported string escape|malformed Rust ASCII escape|malformed unicode escape/u.test(error.message)
        && error.span.start === start && error.span.end === start + 2, escape);
    }
  }
  for (const source of ['"\\', '"\\x', '"\\x0']) assert.throws(() => tokenize(source, 'Rust'), error => error.kind === 'syntax', source);
});

test('isolated source carriage returns refuse without changing CRLF or escaped carriage-return values', () => {
  for (const body of fixture.isolatedCarriageReturns) {
    const source = sourceFor(body);
    const start = source.indexOf('\r');
    for (const operation of [() => tokenize(source, 'Rust'), () => createTypedProgramLinks(source, 'Rust')]) {
      assert.throws(operation, error => error.kind === 'syntax' && error.reason === 'isolated carriage return in a Rust string'
        && error.span.start === start && error.span.end === start + 1);
    }
    for (const language of ['Lean', 'Rocq']) assert.equal(tokenize('"' + body + '"', language).tokens[0].value, body);
  }
  for (const [text, value] of [['a\r\nb', 'a\nb'], ['a\\r\\x0db', 'a\r\rb'], ['a\\\r\n  b', 'ab']]) {
    const packet = reload(createTypedProgramLinks(sourceFor(text), 'Rust'));
    assert.deepEqual(execute(emitTypedProgramLinks(packet.network, packet.root, 'JavaScript').code), [value]);
  }
  assert.throws(() => tokenize('"a\rb"', 'JavaScript'), /unterminated string literal/u);
});

test('a complete program containing all ASCII values executes from source-free Links and actual emitted definitions', () => {
  const text = fixture.ascii.map(asciiEscape).join('');
  const expected = String.fromCharCode(...fixture.ascii);
  const source = sourceFor(text);
  const packet = reload(createTypedProgramLinks(source, 'Rust'));
  assert.ok(packet.network.links().every(link => !link.metadata().definition?.includes(source)));
  assert.deepEqual(execute(emitTypedProgramLinks(packet.network, packet.root, 'JavaScript').code), [expected]);
  const program = decodeTypedProgramLinks(packet.network, packet.root);
  assert.equal(program.declarations.get('choose').body.value, expected);
  const definitions = emitRust(program).definitions.join('\n\n');
  // Replay every actual emitted source definition and the complete source main.
  // The generated native process/thread wrapper is a separate frontend boundary.
  const replay = checkProgram(parseRust(definitions + '\nfn main(){println!("{}",choose());}'));
  assert.deepEqual(execute(emitJavaScript(replay).text), [expected]);
  assert.equal(emitRust(replay).definitions.join('\n\n'), definitions);
});

test('a source-free linked string edit changes both emitted targets and replayed whole-program behavior', () => {
  const packet = reload(createTypedProgramLinks(sourceFor('\\x41\\x00\\x7f'), 'Rust'));
  const program = decodeTypedProgramLinks(packet.network, packet.root);
  const expected = 'edited\0\b\f\v{α😀';
  program.declarations.get('choose').body.value = expected;
  const edited = reload(encodeTypedProgramLinks(program));
  assert.deepEqual(execute(emitTypedProgramLinks(edited.network, edited.root, 'JavaScript').code), [expected]);
  const declarations = emitRust(decodeTypedProgramLinks(edited.network, edited.root)).definitions.join('\n\n');
  assert.ok(emitTypedProgramLinks(edited.network, edited.root, 'Rust').code.includes(declarations));
  const replay = checkProgram(parseRust(declarations + '\nfn main(){println!("{}",choose());}'));
  assert.deepEqual(execute(emitJavaScript(replay).text), [expected]);
});

test('JavaScript, Lean and Rocq keep their existing escape interpretations and the generic decoder scope', () => {
  for (const [escape, expected] of [['\\b', '\b'], ['\\f', '\f'], ['\\v', '\v'], ['\\{', '\\{']]) {
    assert.equal(tokenize('"' + escape + '"', 'Lean').tokens[0].value, expected, escape);
    assert.equal(tokenize('"' + escape + '"', 'Rocq').tokens[0].value, escape, escape);
  }
  assert.equal(tokenize('"\\b\\f\\v\\{"', 'JavaScript').tokens[0].value, '\b\f\v{');
  assert.equal(tokenize('"\\xE9"', 'JavaScript').tokens[0].value, 'é');
  assert.throws(() => tokenize('"\\00"', 'JavaScript'), /legacy octal/u);
  for (const language of ['Lean', 'Rocq']) assert.equal(tokenize('"\\\\"', language).tokens[0].value, language === 'Lean' ? '\\' : '\\\\');
  assert.deepEqual(decodeUnicodeEscape([...String.raw`\xE9`].map(character => character.charCodeAt(0)), false), { $: 'malformed' });
});

test('the generated prefix preserves the existing draft decision and ASCII decisions have an exact numeric domain', () => {
  for (let unit = 0; unit < 256; unit += 1) {
    assert.equal(acceptRustCookedStringEscapePrefix(unit), draftPrefix(unit));
    const text = String.fromCharCode(unit);
    const expected = /^[0-9a-f]$/iu.test(text) ? Number.parseInt(text, 16) : -1;
    assert.equal(readRustAsciiEscapeDigit(unit), expected);
  }
  for (const code of fixture.ascii) {
    const spelling = code.toString(16).padStart(2, '0');
    assert.equal(readRustAsciiEscapeValue(spelling.charCodeAt(0), spelling.charCodeAt(1)), code);
  }
  for (const value of [-1, 0.5, 48.5, NaN, Infinity, -Infinity]) assert.equal(readRustAsciiEscapeDigit(value), -1);
  for (const code of fixture.nonAscii) {
    const spelling = code.toString(16);
    assert.equal(readRustAsciiEscapeValue(spelling.charCodeAt(0), spelling.charCodeAt(1)), -1);
  }
});

test('the JS-first escape rules and independent shared fixture remain fresh', () => {
  for (const [script, options] of [['generate-rust-cooked-string-rules.mjs', ['--javascript-only']], ['build-rust-cooked-string-escape-fixtures.mjs', []]]) {
    execFileSync(process.execPath, ['js/scripts/' + script, '--check', ...options], { cwd: project, timeout: 30_000, stdio: 'pipe' });
  }
});
