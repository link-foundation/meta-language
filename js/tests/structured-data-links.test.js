import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { decodeStructuredDataLinks, encodeStructuredDataLinks, readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { acceptStructuredDataShape, readStructuredDataKind } from '../src/structured-data-link-rules.js';
import { convertStructuredData } from '../scripts/convert-structured-data.mjs';

const linkWith = (network, term) => network.links().find(link => link.metadata().term === term);
const keysInOrder = (first, second) => {
  if (!first || typeof first !== 'object') return;
  assert.deepEqual(Object.keys(second), Object.keys(first));
  for (const name of Object.keys(first)) keysInOrder(first[name], second[name]);
};

test('inspectable data round trips exact primitive values, ordered fields and shared collections', () => {
  const shared = { text: 'λ 😀\n\u0000', empty: '' };
  const source = JSON.parse('{"__proto__":{"safe":true},"constructor":"ordinary key"}');
  source.scalars = [null, true, false, -0, 0, 1.25, Number.MIN_VALUE, Number.MAX_VALUE, 1e-7, 1e21, '\ud800', '\udfff'];
  source.first = shared; source.second = shared;
  const { network, root } = encodeStructuredDataLinks(source);
  const result = decodeStructuredDataLinks(network, root);
  assert.deepEqual(result, source);
  assert.deepEqual(structuredClone(result), source);
  assert.equal(result.first, result.second);
  assert.equal(Object.getPrototypeOf(result), Object.prototype);
  assert.equal(Object.hasOwn(result, '__proto__'), true);
  assert.equal(Object.is(result.scalars[3], -0), true);
  keysInOrder(source, result);
  const lino = network.toLino();
  assert.match(lino, /key%20value%20pair/u);
  assert.match(lino, /Unicode%20code%20unit/u);
  assert.deepEqual(readStructuredDataLino(lino), source);
  assert.equal(structuredDataToLino(readStructuredDataLino(lino)), lino);
  // Explicit object entry order survives even when numeric-looking names are edited.
  const ordered = encodeStructuredDataLinks({ 1: 'one', 2: 'two' });
  const object = linkWith(ordered.network, 'object value');
  object.setReferences(object.references().reverse());
  const decoded = decodeStructuredDataLinks(ordered.network, ordered.root);
  assert.deepEqual(Object.keys(decoded), ['2', '1']);
  assert.deepEqual(Object.keys(readStructuredDataLino(structuredDataToLino(decoded))), ['2', '1']);
});

test('standalone reader refuses counterfeit shapes, identities, scalar types and hidden components', () => {
  const base = structuredDataToLino({ key: [1, true, null] });
  const cases = [
    base.replace('(term: structured%20data) (def: 1)', '(term: structured%20data) (def: 2)'),
    base.replace('number%20value', 'foreign%20value'),
    base.replace('(def: true)', '(def: perhaps)'),
    base.replace('(def: 1)', '(def: NaN)'),
    base.replace('(def: 1)', '(def: 1.0)'),
    base.replace('(n: 1)', '(n: 2)'),
    base.replace('(t: semantic)', '(t: semantic) (t: semantic)'),
    base.replace('(lang: meta-language%20structured%20data)', '(lang: foreign)'),
    base + base.split('\n')[0] + '\n',
    base + '(999: (meta: (t: semantic) (n: 1) (term: null%20value) (lang: meta-language%20structured%20data)))\n',
    base.replace('(meta:', '(meta: (reg: 1)'),
  ];
  for (const document of cases) assert.throws(() => readStructuredDataLino(document), /structured data|serialization/u);
  for (const mutation of [
    ({ network }) => linkWith(network, 'object value').setReferences([linkWith(network, 'number value').id()]),
    ({ network }) => { const object = linkWith(network, 'object value'); object.setReferences([...object.references(), ...object.references()]); },
    ({ network }) => { const array = linkWith(network, 'array value'); array.setReferences([array.id()]); },
    ({ network }) => linkWith(network, 'array value').setReferences([999]),
  ]) {
    const encoded = encodeStructuredDataLinks({ key: [1, true, null] });
    mutation(encoded);
    assert.throws(() => decodeStructuredDataLinks(encoded.network, encoded.root), /structured data/u);
  }
  for (const source of [NaN, Infinity, undefined, () => 1, new Date(), [, 1], Object.defineProperty({}, 'x', { get() { return 1; }, enumerable: true })]) {
    assert.throws(() => encodeStructuredDataLinks(source), /structured data/u);
  }
  const cycle = {}; cycle.self = cycle;
  assert.throws(() => encodeStructuredDataLinks(cycle), /structural cycle/u);
});

test('node, expanded sharing, depth and byte budgets reject bounded counterexamples', () => {
  assert.throws(() => encodeStructuredDataLinks({ x: 'y' }, { nodeLimit: 2 }), /resource limit/u);
  assert.throws(() => encodeStructuredDataLinks([[[1]]], { depthLimit: 1 }), /resource limit/u);
  assert.throws(() => encodeStructuredDataLinks({ key: 1 }, { depthLimit: 2 }), /resource limit/u);
  const boundary = encodeStructuredDataLinks({ key: 1 }, { depthLimit: 3 });
  assert.deepEqual(decodeStructuredDataLinks(boundary.network, boundary.root, { depthLimit: 3 }), { key: 1 });
  assert.throws(() => readStructuredDataLino(structuredDataToLino('long'), { byteLimit: 8 }), /resource limit/u);
  let repeated = 0;
  for (let index = 0; index < 12; index += 1) repeated = [repeated, repeated];
  const graph = encodeStructuredDataLinks(repeated);
  assert.ok(graph.network.links().length < 20);
  assert.throws(() => decodeStructuredDataLinks(graph.network, graph.root, { nodeLimit: 1000 }), /resource limit/u);
  assert.throws(() => readStructuredDataLino(graph.network.toLino(), { nodeLimit: 1000 }), /resource limit/u);
});

test('automatic conversion preserves recorded fixture values and canonical generated decisions', () => {
  const source = '{"version":1,"rules":[{"name":"α","numbers":[-0,1e-7,1e21]}],"__proto__":null}';
  const lino = convertStructuredData(source);
  const decoded = readStructuredDataLino(lino);
  assert.deepEqual(decoded, JSON.parse(source));
  keysInOrder(JSON.parse(source), decoded);
  assert.equal(lino, structuredDataToLino(decoded));
  assert.equal(readStructuredDataKind('key value pair'), 'pair');
  assert.equal(acceptStructuredDataShape('pair', '2', false), true);
  assert.equal(acceptStructuredDataShape('pair', '1', false), false);
  const generated = readFileSync(new URL('../../rust/src/structured_data_link_rules.rs', import.meta.url), 'utf8');
  assert.match(generated, /Generated automatically from js\/src\/structured-data-link-rules/u);
  const shared = readFileSync(new URL('../../parity/fixtures/structured-data-links.lino', import.meta.url), 'utf8');
  const expected = [{ a: 'λ 😀\n\u0000', empty: '', keys: JSON.parse('{"__proto__":true,"constructor":false}'),
    numbers: [-0, 0, 1.25, Number.MIN_VALUE, Number.MAX_VALUE, 1e-7, 1e21] }, ['\ud800', '\udfff'], null, true, false];
  assert.deepEqual(readStructuredDataLino(shared), expected);
  assert.equal(structuredDataToLino(expected), shared);
  // Bootstrap decoding must succeed without any grammar/cache/parser imports.
  const bootstrap = spawnSync(process.execPath, ['--input-type=module', '-e',
    `import {readStructuredDataLino,structuredDataToLino} from './src/structured-data-links.js';
    if(readStructuredDataLino(structuredDataToLino({pinned:true})).pinned!==true)throw Error('bootstrap');`], {
    cwd: new URL('..', import.meta.url), encoding: 'utf8', env: { ...process.env, META_LANGUAGE_OFFLINE: '1' },
  });
  assert.equal(bootstrap.status, 0, bootstrap.stderr);
});
