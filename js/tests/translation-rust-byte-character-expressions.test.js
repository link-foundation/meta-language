import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { tokenize } from '../src/translation/lexer.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { readRustCharacterExpressionType, acceptRustNumericLiteralNegation } from '../src/translation/rust-character-rules.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

// Rust Reference: expressions/literal-expr.html#byte-literal-expressions and
// expressions/operator-expr.html#negation-operators. These source rules do not
// infer a wider integer type for a byte or permit unsigned negation.
const byteProgram = String.raw`fn zero()->u8{b'\0'}
fn ascii()->u8{b'0'}
fn high()->u8{b'\xFF'}
fn digit(value:u8)->u8{value-b'0'}
fn nine(value:u8)->bool{value==b'9'}
fn widened()->i16{-(b'0' as i16)}
fn main(){println!("{}",zero());println!("{}",ascii());println!("{}",high());println!("{}",digit(b'9'));println!("{}",nine(b'9'));println!("{}",widened());}`;

test('byte expression decisions preserve fixed source types and signed negation domains', () => {
  assert.equal(readRustCharacterExpressionType('byteChar'), 'u8');
  for (const kind of ['char', 'number', 'identifier', '', 'unknown', 'byteCharSuffix']) {
    assert.equal(readRustCharacterExpressionType(kind), '');
  }
  for (const [language, kind, signed, expected] of [
    ['Rust', 'fixed', false, false], ['Rust', 'fixed', true, true],
    ['Rust', '', false, true], ['Rust', 'int', false, true], ['Rust', 'float', false, true],
    ['JavaScript', 'fixed', false, true], ['Lean', 'fixed', false, true], ['Rocq', 'fixed', false, true],
  ]) assert.equal(acceptRustNumericLiteralNegation(language, kind, signed), expected);
});

test('complete byte expression programs retain u8 values and behavior after checked text and binary replay', () => {
  const checked = checkProgram(parseRust(byteProgram));
  const javascript = emitJavaScript(checked), rust = emitRust(checked);
  const encoded = encodeTypedProgramLinks(checked);
  const programs = [checked];
  for (const network of [
    LinkNetwork.fromLino(encoded.network.toLino()),
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(encoded.network)),
  ]) programs.push(recheckLinkedTypedProgram(decodeTypedProgramLinks(network, encoded.root), { wholeProgram: true }));
  for (const program of programs) {
    for (const [name, value] of [['zero', '0'], ['ascii', '48'], ['high', '255']]) {
      const declaration = program.declarations.get(name);
      assert.deepEqual(declaration.ret, { kind: 'fixed', bits: 8, signed: false });
      assert.deepEqual(declaration.body.type, declaration.ret);
      assert.equal(declaration.body.value, value);
    }
    const emitted = emitJavaScript(program), lines = [];
    assert.deepEqual(emitted, javascript);
    assert.deepEqual(emitRust(program), rust);
    const context = vm.createContext({ console: { log: (...args) => lines.push(args.join(' ')) } });
    vm.runInContext(emitted.text, context);
    assert.equal(lines.join('\n') + '\n', '0\n48\n255\n9\ntrue\n-48\n');
    for (const [name, expected] of [['zero', 0n], ['ascii', 48n], ['high', 255n], ['widened', -48n]]) {
      const target = emitted.mappings.find(mapping => mapping.kind === 'function' && mapping.source === name).target;
      assert.equal(vm.runInContext(target + '()', context), expected);
    }
  }
});

test('original byte literals retain their spans and reject implicit u32 widening in complete programs', () => {
  for (const literal of ["b'}'", String.raw`b'\xFF'`]) {
    const original = 'fn value()->u32{' + literal + '}';
    const source = original + 'fn main(){}';
    const surface = parseRust(source), start = source.indexOf(literal);
    const token = tokenize(source, 'Rust').tokens.find(token => token.kind === 'byteChar');
    assert.equal(source.slice(token.start, token.end), literal);
    assert.deepEqual(surface.items[0].body.span, { start, end: start + literal.length });
    assert.deepEqual(surface.items[0].body.type, { kind: 'fixed', bits: 8, signed: false });
    assert.throws(() => checkProgram(surface), error => error.kind === 'type'
      && error.reason === 'expected u32 but found u8');
    assert.equal(source.slice(0, original.length), original);
  }
});

test('unsigned zero negation and cast precedence refuse while signed minima and floating zero survive', () => {
  for (const [result, expression] of [
    ['u8', String.raw`-b'\0'`], ['u8', String.raw`-(b'\0')`],
    ['u8', '-0u8'], ['u8', '-0'], ['u8', '-(0)'],
    ['i16', "-b'0' as i16"], ['u8', 'let value:u8=0u8;-value'],
  ]) {
    const source = `fn value()->${result}{${expression}}fn main(){}`;
    assert.throws(() => checkProgram(parseRust(source)), error => error.kind === 'type'
      && error.reason === 'negation of u8', source);
  }
  const source = 'fn minimum()->i8{-128i8}fn zero()->i32{-0}fn float_zero()->f64{-0.0}fn main(){}';
  const checked = checkProgram(parseRust(source));
  assert.equal(checked.declarations.get('minimum').body.value, '-128');
  assert.equal(checked.declarations.get('zero').body.value, '0');
  assert.equal(checked.declarations.get('float_zero').body.value, '-0');
  assert.throws(() => checkProgram(parseRust("fn value()->u8{!b'a'}fn main(){}")), error => error.kind === 'type');
});
