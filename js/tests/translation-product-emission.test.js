import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { acceptTupleCopyType } from '../src/translation/product-emission-rules.js';

const FLOAT = { kind: 'float' };
const BOOL = { kind: 'bool' };
const name = value => ({ k: 'name', path: [value] });
const number = value => ({ k: 'num', value: String(value), type: FLOAT });
const tupleType = elements => ({ kind: 'tuple', elements });

function valueProgram(frozen) {
  const inner = tupleType([FLOAT]);
  const result = tupleType([inner, BOOL]);
  return checkProgram({ language: 'JavaScript', items: [{
    k: 'fn', name: 'value', params: [], ret: result,
    body: { k: 'tuple', frozen, ...(frozen && { productModel: 'immutable value product' }), items: [
      { k: 'tuple', frozen, ...(frozen && { productModel: 'immutable value product' }), items: [number(5)] }, { k: 'bool', value: true },
    ] },
  }], main: null });
}

test('selected nested product allocations retain their immutable JavaScript representation', () => {
  const emitted = emitJavaScript(valueProgram(true)).text;
  const value = runInNewContext(`${emitted}\nvalue()`);
  assert.equal(value[0][0], 5);
  assert.equal(value[1], true);
  assert.equal(Object.isFrozen(value), true);
  assert.equal(Object.isFrozen(value[0]), true);
  assert.equal(Reflect.set(value, '1', false), false);
  assert.equal(Reflect.set(value[0], '0', 9), false);
  assert.equal(value[0][0], 5);
  assert.equal(value[1], true);
});

test('legacy tuple allocations retain ordinary array behavior', () => {
  const value = runInNewContext(`${emitJavaScript(valueProgram(false)).text}\nvalue()`);
  assert.equal(Object.isFrozen(value), false);
  assert.equal(Object.isFrozen(value[0]), false);
  assert.equal(Reflect.set(value[0], '0', 9), true);
  assert.equal(value[0][0], 9);
});

test('Rust product projections preserve nested arity and repeated scalar Copy uses', () => {
  const pair = tupleType([tupleType([FLOAT]), BOOL]);
  const checked = checkProgram({ language: 'Rust', items: [{
    k: 'fn', name: 'choose', params: [{ name: 'value', type: pair }], ret: FLOAT,
    body: { k: 'binary', op: 'add',
      left: { k: 'tupleIndex', object: { k: 'tupleIndex', object: name('value'), index: 0 }, index: 0 },
      right: { k: 'tupleIndex', object: { k: 'tupleIndex', object: name('value'), index: 0 }, index: 0 },
    },
  }], main: null });
  const emitted = emitRust(checked).text;
  assert.ok(emitted.includes('value: ((f64,), bool)'), emitted);
  assert.ok(emitted.includes('((value).0).0'), emitted);
  assert.equal(emitted.includes('value.clone()'), false);
  const target = emitJavaScript(checked).text;
  assert.equal(runInNewContext(`${target}\nchoose(Object.freeze([Object.freeze([5]), true]))`), 10);
});

test('Copy decisions keep ownership-bearing values outside the scalar product model', () => {
  assert.equal(acceptTupleCopyType('tuple', [true, true]), true);
  assert.equal(acceptTupleCopyType('tuple', [true, false]), false);
  for (const kind of ['string', 'array', 'map', 'option', 'cell', 'data']) {
    assert.equal(acceptTupleCopyType(kind, [true]), false, kind);
  }
});

test('a tuple parameter alone retains conservative ownership without selected model evidence', () => {
  const type = tupleType([FLOAT, BOOL]);
  const program = checkProgram({ language: 'Rust', items: [{
    k: 'fn', name: 'identity', params: [{ name: 'value', type }], ret: type, body: name('value'),
  }], main: null });
  assert.ok(emitRust(program).text.includes('value.clone()'));
});

test('each executable target refuses a retained product model with missing immutability evidence', () => {
  for (const emit of [emitJavaScript, emitRust]) for (const model of ['immutable value product', 'unknown model', null, undefined]) {
    const program = valueProgram(true);
    const declaration = program.declarations.get('value');
    declaration.body = { ...declaration.body, productModel: model, frozen: false };
    assert.throws(() => emit(program), /immutable tuple product model/, String(model));
  }
});
