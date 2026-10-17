import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { DecoratorSet } from '../src/decorators.js';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { readProjectData } from '../scripts/project-data.mjs';
import cases from './fixtures/translation-cases/frontend-javascript.mjs';

const stages = readProjectData(new URL('../../parity/fixtures/translation-stages.lino', import.meta.url));

function observe(source) {
  const output = [];
  vm.runInNewContext(source, { console: { log: value => output.push(String(value)) } }, { timeout: 1000 });
  return output;
}

test('the original BigInt slice bound fails through normal method argument inference', () => {
  const name = 'frontend-javascript-error-method-call';
  const source = cases[name][1];
  const fixture = stages.programs.find(program => program.name === 'frontend-javascript/' + name);
  assert.equal(fixture.source, source);
  assert.throws(() => parseJavaScript(source), error => {
    assert.deepEqual({ kind: error.kind, message: error.message }, fixture.parse.error);
    return true;
  });
});

test('valid string and array slice operands retain checked results and actual JavaScript observations', () => {
  const source = '/** @param {string} text @returns {string} */function crop(text){return text.slice(1,3);}\n'
    + '/** @param {number[]} values @returns {number[]} */function middle(values){return values.slice(1,-1);}\n'
    + "console.log(crop('A😀B'));console.log(middle([1,2,3,4]).length);";
  const checked = checkProgram(parseJavaScript(source));
  assert.deepEqual(observe(source), ['😀', '2']);
  assert.deepEqual(observe(emitJavaScript(checked).text), ['😀', '2']);
  assert.doesNotThrow(() => emitRust(checked));
  for (const emit of [emitLean, emitRocq]) assert.throws(() => emit(checked), error => error.kind === 'unsupported'
    && error.reason === '.slice(): this target needs a UTF-16 operational string encoding for this operation'
    && error.span.start === source.indexOf('text.slice')
    && error.span.end === source.indexOf(';', source.indexOf('text.slice')));
  const copied = parseJavaScript('/** @param {string} text @returns {string} */function copy(text){return text.slice();}');
  assert.equal(copied.items[0].body.k, 'copySlice');
  assert.doesNotThrow(() => checkProgram(copied));
});

test('bound parameter types drive slice refusals independently of literal spelling', () => {
  for (const argument of ['1n', 'position']) {
    const baseline = '/** @param {number} value @returns {number} */function takeNumber(value){return value;}\n'
      + '/** @param {bigint} position */function ordinary(position){return takeNumber(' + argument + ');}';
    let ordinaryError;
    assert.throws(() => parseJavaScript(baseline), error => { ordinaryError = error; return error.kind === 'type'; });
    const source = '/** @param {string} text @param {bigint} position @returns {string} */'
      + 'function crop(text,position){return text.slice(' + argument + ');}';
    assert.throws(() => parseJavaScript(source), error => {
      assert.deepEqual({ kind: error.kind, reason: error.reason, span: error.span }, {
        kind: 'type',
        reason: argument === '1n'
          ? 'a BigInt is used as a number; JavaScript does not mix BigInt with other types'
          : 'one value is used as a bigint and as a number; declare the types of the function with JSDoc',
        span: { start: source.indexOf('text.slice'), end: source.indexOf(';', source.indexOf('text.slice')) },
      });
      assert.equal(error.reason, ordinaryError.reason);
      return true;
    });
  }
});

test('string and array slice keep their existing argument traversal and arity boundaries', () => {
  const string = '/** @param {string} text @returns {string} */function crop(text){return text.slice(1,2,3n);}';
  const surface = parseJavaScript(string);
  assert.throws(() => checkProgram(surface), error => error.kind === 'unsupported'
    && error.reason === 'method call .slice(): .slice() expects 0 to 2 arguments');
  const array = '/** @param {number[]} values @returns {number[]} */function crop(values){return values.slice(1,2,3n);}';
  assert.throws(() => parseJavaScript(array), error => error.kind === 'type'
    && error.reason === 'a BigInt is used as a number; JavaScript does not mix BigInt with other types');
});

test('the entire JavaScript frontend decision module remains checked scalar self translation', () => {
  const source = readFileSync(new URL('../src/translation/javascript-surface-rules.js', import.meta.url), 'utf8');
  const surface = parseJavaScript(source, { referenceSemantics: false });
  const emitted = emitRust(checkProgram(surface));
  assert.deepEqual(emitted.preludes, []);
  assert.equal(emitted.definitions.length, surface.items.length);
  assert.ok(emitted.definitions.some(definition => definition.includes('read_java_script_method_argument_kind')));
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header);
  const raw = header + '\n\n' + emitted.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
  const styles = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/javascript-surface-rules-decorators.lino', import.meta.url), 'utf8'));
  const styled = styles.decorate('emitter', { format: 'Rust', source: raw }).source;
  // Recorded Boolean source styles may qualify a function const while keeping
  // its declaration, source behavior and position in the complete module.
  const expectedNames = [...raw.matchAll(/^pub (?:const )?fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
  assert.equal(expectedNames.length, surface.items.length);
  assert.deepEqual([...styled.matchAll(/^pub (?:const )?fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]), expectedNames);
  assert.match(styled, /^pub const fn accept_java_script_value_switch\(/mu);
  assert.match(styled, /pub fn read_java_script_method_receiver_kind\(operation: &str\)/u);
});

test('terminal tag guard source styles append exact new Boolean functions without rewriting the historical library row', async () => {
  const { createHash } = await import('node:crypto');
  const { readPortableSourceParameterStyles, readPortableSourceLintStyles } = await import('../scripts/portable-source-library-style.mjs');
  const { readSourceTextRustBlock } = await import('../scripts/source-text-byte-library-style.mjs');
  const source = readFileSync(new URL('../src/translation/javascript-surface-rules.js', import.meta.url), 'utf8');
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const emitted = emitRust(checked, { arrayConsumption: true });
  assert.deepEqual(emitted.preludes, []);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header);
  const raw = header + '\n\n' + emitted.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
  const document = readFileSync(new URL('../../parity/self-translation/javascript-surface-rules-decorators.lino', import.meta.url), 'utf8');
  const lines = document.match(/[^\n]*\n|[^\n]+$/gu) ?? [];
  assert.equal(Buffer.byteLength(lines[0]), 106060);
  assert.equal(createHash('sha256').update(lines[0]).digest('hex'), 'ac9cf3ef2a0a4a4b16e59b9c2d33a2619fd8cbe44bf4042913f52419cc275e6a');
  const id = 'javascript-terminal-tag-guard-library-style';
  assert.equal(lines.filter(line => line.startsWith('(decorator ' + id + ' ')).length, 1);
  const baseDocument = lines.filter(line => !line.startsWith('(decorator ' + id + ' ')).join('');
  const base = DecoratorSet.fromLino(baseDocument).decorate('emitter', { format: 'Rust', source: raw }).source;
  const names = ['accept_java_script_terminal_tag_guard', 'accept_java_script_tag_guard_sequence'];
  let expected = base;
  for (const name of names) {
    const from = '#[must_use]\n' + readSourceTextRustBlock(base, 'fn', name);
    assert.equal(base.split(from).length, 2);
    const to = readPortableSourceLintStyles(readPortableSourceParameterStyles(from).source).source;
    assert.match(to, new RegExp('^pub const fn ' + name + '\\(', 'mu'));
    assert.equal((to.match(/^#\[must_use\]$/gmu) ?? []).length, 1);
    assert.equal(readPortableSourceLintStyles(to).source, to);
    expected = expected.replace(from, to);
  }
  const styled = DecoratorSet.fromLino(document).decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.equal(styled, expected);
  const additional = new DecoratorSet(DecoratorSet.fromLino(document).decorators.filter(row => row.id === id));
  assert.equal(additional.decorate('emitter', { format: 'Rust', source: styled }).source, styled);
  for (const name of [...raw.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\(/gmu)].map(match => match[1]).filter(name => !names.includes(name))) {
    assert.equal(readSourceTextRustBlock(styled, 'fn', name), readSourceTextRustBlock(base, 'fn', name), name + ': unchanged complete historical declaration');
  }
  const original = vm.runInNewContext(source.replace(/^export /gmu, '') + '\n({acceptJavaScriptTerminalTagGuard,acceptJavaScriptTagGuardSequence})');
  const generated = vm.runInNewContext(emitJavaScript(checked).text + '\n({acceptJavaScriptTerminalTagGuard,acceptJavaScriptTagGuardSequence})');
  for (const positive of [false, true]) for (const alternate of [false, true]) for (const terminal of [false, true]) for (const same of [false, true]) for (const duplicate of [false, true]) {
    const args = [positive, alternate, terminal, same, duplicate], expected = positive && !alternate && terminal && same && !duplicate;
    assert.equal(original.acceptJavaScriptTerminalTagGuard(...args), expected);
    assert.equal(generated.acceptJavaScriptTerminalTagGuard(...args), expected);
  }
  for (const multiple of [false, true]) for (const terminal of [false, true]) {
    assert.equal(original.acceptJavaScriptTagGuardSequence(multiple, terminal), multiple && terminal);
    assert.equal(generated.acceptJavaScriptTagGuardSequence(multiple, terminal), multiple && terminal);
  }
});
