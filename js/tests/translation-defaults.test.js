import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';

import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { readParameterDefaultArgumentInference, readParameterDefaultValueStep, requireParameterDefaultArgumentBindings } from '../src/translation/parameter-default-rules.js';
import { LinkNetwork } from '../src/network.js';
import { decodeLinkNetworkBinary, encodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { preservesTranslationBehavior } from './support/translation-behavior.js';

test('translation stages retain constant bindings and explicitly absent parameter defaults', () => {
  const source = "function identity(value) { return value; }\nconst answer = identity(42);\n";
  const surface = parseJavaScript(source);
  assert.equal(surface.main.effects[0].constant, true);
  const program = checkProgram(surface);
  assert.equal(program.main.effects[0].constant, true);
  assert.equal(program.declarations.get('identity').params[0].default, null);
});

const DEFAULTS = `function label(name, level = 'grammar-rule', depth = 2) {
  return \`\${name}:\${level}:\${depth}\`;
}
/**
 * @param {string} term
 * @param {number[]} children
 * @returns {number}
 */
function size(term, children = []) {
  return children.length + (term === '' ? 0 : 1);
}
console.log('%s', label('a'), label('b', 'x'), label('c', 'y', 3), size('ab'), size('', [1, 2]));
`;

const refusal = (source) => translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message;

test('a call that leaves out defaulted parameters passes the defaults in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(DEFAULTS, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
  const rust = translateProgram(DEFAULTS, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('pub fn label(name: String, level: String, depth: f64) -> String {'), rust);
  assert.ok(rust.includes('crate::label(String::from("a"), String::from("grammar-rule"), 2f64)'), rust);
  assert.ok(rust.includes('crate::label(String::from("b"), String::from("x"), 2f64)'), rust);
  assert.ok(rust.includes('crate::size(String::from("ab"), Vec::<f64>::new())'), rust);
});

test('defaults that are not filled in by a call are refused with a reason', () => {
  assert.match(refusal('function f(a = 1, b) {\n  return b;\n}\n'), /parameter b after a default/u);
  assert.match(refusal('function f(a, b = a) {\n  return b;\n}\nconsole.log(f(1));\n'), /default reading parameter a/u);
});

test('default calls resolve names outside the callers local scope', () => {
  const source = 'function base() { return 7; }\nfunction chosen(value = base()) { return value; }\nfunction caller(base) { return chosen(); }\nconsole.log(caller(99));\n';
  const translated = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(translated.diagnostic, null);
  assert.ok(translated.code.includes('crate::chosen(crate::base())'), translated.code);
});

test('self-translation writes top-level constants as Rust constants and restores them', async () => {
  const { selfTranslate } = await import('../src/self-translation.js');
  const source = "const MAX_LENGTH = 2048;\nexport const TERM = 'identifier:trigram';\nexport const PAIRS = Object.freeze([['en', 'Hawaii']]);\n";
  const rust = selfTranslate(source, 'JavaScript', 'Rust');
  assert.deepEqual(rust.items.map(({ status }) => status), ['translated', 'translated', 'translated']);
  assert.ok(rust.code.includes('pub const MAX_LENGTH: f64 = 2048f64;'), rust.code);
  assert.ok(rust.code.includes('pub const TERM: &str = "identifier:trigram";'), rust.code);
  assert.ok(rust.code.includes('pub static PAIRS: std::sync::LazyLock<Vec<Vec<String>>> = std::sync::LazyLock::new(|| vec![vec![String::from("en"), String::from("Hawaii")]]);'), rust.code);
  assert.equal(selfTranslate(rust.code, 'Rust', 'JavaScript').code, source);
  // A top-level let and a statement with effects still run as a program.
  assert.equal(selfTranslate('let count = 1;\n', 'JavaScript', 'Rust').items[0].status, 'carried');
});


test('optional JSDoc names preserve declared types without supplying runtime defaults', () => {
  const source = "/** @param {number} [value=99] @returns {number} */\nfunction add(value = 2) { return value + 1; }\nconsole.log(add(), add(7));\n";
  const rust = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.ok(rust.code.includes('crate::add(2f64)'), rust.code);
  assert.ok(rust.code.includes('crate::add(7f64)'), rust.code);
  const identity = parseJavaScript('/** @param {string} [value] @returns {string} */\nfunction identity(value) { return value; }\n');
  assert.deepEqual(identity.items[0].params[0].type, { kind: 'option', element: { kind: 'string' }, absence: 'undefined' });
  assert.equal(identity.items[0].params[0].optional, true);
  assert.deepEqual(identity.items[0].params[0].default, { k: 'none', absence: 'undefined', span: identity.items[0].params[0].span });
  preservesTranslationBehavior('/** @param {number} [value=99] @returns {number} */\nfunction add(value) { return value + 1; }\nconsole.log(add());\nconsole.log(add(undefined));\nconsole.log(add(7));\n', 'NaN\nNaN\n8\n');
  preservesTranslationBehavior('/** @param {number} [value=99] @returns {number} */\nfunction add(value = 2) { return value + 1; }\nconsole.log(add());\nconsole.log(add(undefined));\nconsole.log(add(7));\n', '3\n3\n8\n');
  assert.match(refusal('/** @param {number} [other] */\nfunction identity(value) { return value; }\n'), /other is not a parameter/u);
});

function observeParameterDefaultCalls(source) {
  const lines = [];
  let error = null;
  try {
    runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  } catch (failure) {
    error = failure.message;
  }
  return { output: lines.length ? lines.join('\n') + '\n' : '', error };
}

function assertParameterDefaultCalls(source, output, error = null) {
  const expected = { output, error };
  assert.deepEqual(observeParameterDefaultCalls(source), expected, 'the original complete source supplies the oracle');
  const checked = checkProgram(parseJavaScript(source));
  const archive = encodeTypedProgramLinks(checked);
  const rust = emitRust(checked, { arrayConsumption: true }).text;
  for (const program of [checked,
    decodeTypedProgramLinks(LinkNetwork.fromLino(archive.network.toLino()), archive.root),
    decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network)), archive.root),
  ]) {
    assert.deepEqual(program, checked, 'text and binary archives retain the actual checked bindings');
    recheckLinkedTypedProgram(program, { wholeProgram: true });
    assert.deepEqual(observeParameterDefaultCalls(emitJavaScript(program).text), expected, 'checked and source-free execution retain caller/default order');
    assert.equal(emitRust(program, { arrayConsumption: true }).text, rust, 'source-free Rust uses the same checked initializer bindings');
  }
  return checked;
}

test('shared parameter default decisions retain exact checked children and translate the complete module', () => {
  for (const kind of ['lit', 'none', 'unit', 'var']) assert.equal(readParameterDefaultValueStep(kind, ''), 'pure');
  assert.equal(readParameterDefaultValueStep('cast', 'exact'), 'arg');
  assert.equal(readParameterDefaultValueStep('some', ''), 'value');
  for (const [kind, flavor] of [['call', ''], ['abort', ''], ['cast', 'checked'], ['cast', 'clamp'], ['array', ''], ['', '']]) {
    assert.equal(readParameterDefaultValueStep(kind, flavor), 'effect');
  }
  for (const index of [-1, 0, 1, 2, 3]) for (const provided of [0, 1, 2, 3]) for (const pure of [false, true]) {
    assert.equal(requireParameterDefaultArgumentBindings(index, provided, pure), index >= 0 && index < provided - 1 && !pure);
  }
  for (const defaulted of [false, true]) for (const optional of [false, true]) {
    for (const kind of ['var', 'option', 'float', 'int', 'unit']) for (const absence of ['', 'null', 'undefined', 'both']) {
      const expected = !defaulted || optional ? 'identity' : kind === 'var' ? 'defer' : kind === 'option' && absence === 'undefined' ? 'element' : 'value';
      assert.equal(readParameterDefaultArgumentInference(defaulted, optional, kind, absence), expected);
    }
  }
  const source = readFileSync(new URL('../src/translation/parameter-default-rules.js', import.meta.url), 'utf8');
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  assert.deepEqual([...checked.declarations.values()].filter(entry => entry.k === 'fn').map(entry => entry.name), [
    'readParameterDefaultValueStep', 'requireParameterDefaultArgumentBindings', 'readParameterDefaultArgumentInference',
  ]);
  const rust = emitRust(checked, { arrayConsumption: true }).text;
  assert.match(rust, /pub fn read_parameter_default_value_step\(/u);
  assert.match(rust, /pub fn require_parameter_default_argument_bindings\(/u);
  assert.match(rust, /pub fn read_parameter_default_argument_inference\(/u);
});

test('an effectful parameter initializer follows every provided argument after source-free reload', () => {
  const source = '/** @returns {number} */function fallback(){console.log("default");return 2;}\n'
    + '/** @returns {number} */function later(){console.log("later");return 7;}\n'
    + 'function chosen(value=fallback(),other=0){return value+other;}\n'
    + 'console.log(chosen(undefined,later()));\n';
  assertParameterDefaultCalls(source, 'later\ndefault\n9\n');
});

test('provided optional values evaluate once before lazy defaults in their callee scope', () => {
  const source = '/** @returns {number} */function fallback(){console.log("default");return 2;}\n'
    + '/** @param {boolean} present @returns {number} */function maybe(present){console.log("maybe");if(present)return 7;}\n'
    + '/** @returns {number} */function later(){console.log("later");return 3;}\n'
    + '/** @param {number} value @param {number} other @returns {number} */function chosen(value=fallback(),other=0){return value+other;}\n'
    + 'function caller(fallback){return chosen(maybe(false),later());}\n'
    + 'console.log(chosen(maybe(true),later()));console.log(caller(99));console.log(chosen());\n';
  const checked = assertParameterDefaultCalls(source, 'maybe\nlater\n10\nmaybe\nlater\ndefault\n5\ndefault\n2\n');
  const declaration = checked.declarations.get('fallback');
  const call = { k: 'call', fn: declaration.fullName, args: [], type: declaration.ret };
  assert.deepEqual(projectLinkedProgramExpression(call, checked.declarations).fn.path, ['crate', 'fallback']);
  const nested = { ...declaration, modulePath: ['Tools'], fullName: 'Tools.fallback' };
  assert.deepEqual(projectLinkedProgramExpression({ ...call, fn: nested.fullName }, new Map([[nested.fullName, nested]])).fn.path, ['crate', 'Tools', 'fallback']);
  for (const forged of [{ ...declaration, fullName: 'other' }, { ...declaration, modulePath: ['Other'] }, { ...declaration, name: 'other' }]) {
    assert.throws(() => projectLinkedProgramExpression(call, new Map([[call.fn, forged]])), /actual declaration|actual declaration binding/u);
  }
  assert.deepEqual(projectLinkedProgramExpression({ ...call, fn: 'external' }, checked.declarations).fn.path, ['external']);
  const deferred = 'function chosen(value=2,other=0){return value+other;}\n'
    + 'function caller(){return chosen(maybe(false),later());}\n'
    + 'function maybe(present){console.log("maybe");if(present)return 7;}\n'
    + 'function later(){console.log("later");return 3;}\nconsole.log(caller());\n';
  assertParameterDefaultCalls(deferred, 'maybe\nlater\n5\n');
});

test('aborting provided arguments precede defaults and aborting defaults stop later initializers', () => {
  const declarations = '/** @returns {number} */function fallback(){console.log("default");return 2;}\n'
    + '/** @returns {number} */function stopped(){console.log("argument");throw new Error("argument stopped");}\n'
    + 'function chosen(value=fallback(),other=0){return value+other;}\n';
  assertParameterDefaultCalls(declarations + 'console.log(chosen(undefined,stopped()));\n', 'argument\n', 'argument stopped');
  const source = '/** @returns {number} */function first(){console.log("first default");throw new Error("default stopped");}\n'
    + '/** @returns {number} */function second(){console.log("second default");return 3;}\n'
    + '/** @returns {number} */function later(){console.log("later");return 7;}\n'
    + 'function chosen(value=first(),other=second(),last=0){return value+other+last;}\n'
    + 'console.log(chosen(undefined,undefined,later()));\n';
  assertParameterDefaultCalls(source, 'later\nfirst default\n', 'default stopped');
});

test('source guards run after actual arguments and parameter initializers', () => {
  const source = 'function fallback(){console.log("default");return 2n;}\n'
    + 'function later(){console.log("later");return -1n;}\n'
    + 'function chosen(value=fallback(),other=0n){if(other<0n)throw new RangeError("negative");return value+other;}\n'
    + 'console.log(chosen(undefined,later()));\n';
  assertParameterDefaultCalls(source, 'later\ndefault\n', 'negative');
});

test('pure and omitted parameter defaults keep their existing call bytes and scope refusals', () => {
  const pure = '/** @returns {number} */function later(){console.log("later");return 7;}\n'
    + 'function chosen(value=2,other=0){return value+other;}\nconsole.log(chosen(undefined,later()));\n';
  assertParameterDefaultCalls(pure, 'later\n9\n');
  const rust = emitRust(checkProgram(parseJavaScript(pure))).text;
  assert.ok(rust.includes('crate::chosen('), rust);
  assert.ok(!rust.includes('let ml_a'), 'a primitive literal fallback does not add argument bindings');
  const omitted = 'function base(){return 7;}function chosen(value=base()){return value;}console.log(chosen());';
  assertParameterDefaultCalls(omitted, '7\n');
  assert.ok(emitRust(checkProgram(parseJavaScript(omitted))).text.includes('crate::chosen(crate::base())'));
  assert.match(refusal('function f(a,b=a){return b;}console.log(f(1));'), /default reading parameter a/u);
  assert.match(refusal('function f(a=1,b){return b;}'), /parameter b after a default/u);
});
