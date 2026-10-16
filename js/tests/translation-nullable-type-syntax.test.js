import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { nullableTypeSyntaxRulesPolicy, readNullableTypeSyntaxSourceModule } from '../scripts/generate-nullable-type-syntax-rules.mjs';
import { readNullableTypeAbsence, readNullableTypeSyntax } from '../src/translation/nullable-type-syntax-rules.js';
import { planCallableTypeSyntax } from '../src/translation/callable-source-rules.js';
import { readCompositeType, splitType } from '../src/translation/javascript-bindings.js';
import { unsupported } from '../src/translation/diagnostics.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const observe = source => {
  const lines = [];
  vm.runInNewContext(source, { console: { log: value => lines.push(String(value)) } }, { timeout: 1000 });
  return lines;
};

test('nullable source syntax preserves the entire original reader including duplicate missing-token and parse-order behavior', () => {
  const policy = readProjectData(new URL('../src/translation/data/nullable-type-syntax.lino', import.meta.url));
  assert.deepEqual(nullableTypeSyntaxRulesPolicy(), policy);
  assert.deepEqual(readProjectData(new URL('../../rust/src/data/nullable-type-syntax.lino', import.meta.url)), policy);
  const original = vm.runInNewContext(policy.original.source.replace('export function ', 'function ') + '\nreadCompositeType;', { splitType, unsupported, planCallableTypeSyntax });
  const range = { start: 2, end: 43 };
  for (const name of ['number', 'number|null', 'undefined|bigint', 'null|string|undefined', 'number|null|null',
    'undefined|undefined|boolean', 'number|string|null', 'null|undefined', 'null|null', 'null', 'undefined',
    '?number', '?[number,string]', '??number', '?', '', '|', 'null|', '|undefined', '[number|null,string]|undefined',
    '(value:number|null)=>string|undefined', 'function(number|null):string|undefined']) {
    const evaluate = reader => {
      const calls = [];
      const parser = { type(source, span) { calls.push([source, span]); return { kind: 'probe', source }; } };
      try { return JSON.parse(JSON.stringify({ value: reader(parser, name, range), calls })); }
      catch (error) { return JSON.parse(JSON.stringify({ kind: error.kind, reason: error.reason, span: error.span, calls })); }
    };
    assert.deepEqual(evaluate(readCompositeType), evaluate(original), name);
  }
  assert.deepEqual(readNullableTypeSyntax(['number', 'null', 'null'], 110), [1, 0, 3]);
  assert.deepEqual(readNullableTypeSyntax(['null', 'undefined'], 110), [2, 0, 0]);
  assert.deepEqual(readNullableTypeSyntax(['?number'], 63), [4, 0, 1]);
  assert.equal(readNullableTypeAbsence(3), 'both');
  assert.throws(() => readNullableTypeSyntax([], NaN), RangeError);
  assert.throws(() => readNullableTypeSyntax(new Array(2), NaN), RangeError);
});

test('all three nullable source decisions and six actual dependency functions survive whole checked source translation', () => {
  const names = ['readNullableTypeSyntax', 'readNullableTypeAbsence', 'readNullableTypeTargetRefusal'];
  const helpers = ['readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact', 'refuseRequiredNumberFact', 'refuseRequiredTextFact', 'refuseRequiredBooleanFact'];
  const surface = parseJavaScript(readNullableTypeSyntaxSourceModule(), { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(item => item.k === 'fn' && !item.generated).map(item => item.name).sort(), [...names, ...helpers].sort());
  const checked = checkProgram(surface);
  const saved = encodeTypedProgramLinks(checked);
  const originalRust = emitRust(checked, { arrayConsumption: true }).text;
  for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
    const program = decodeTypedProgramLinks(network, saved.root);
    const rust = emitRust(program, { arrayConsumption: true });
    assert.equal(rust.text, originalRust);
    assert.ok(rust.definitions.length >= names.length + helpers.length);
    for (const definition of rust.definitions) assert.ok(rust.text.includes(definition));
    const api = vm.runInNewContext(emitJavaScript(program).text + '\n({' + names.join(',') + '});', {}, { timeout: 1000 });
    for (const [parts, first, expected] of [[['number', 'null'], 110, [1, 0, 1]], [['null', 'undefined', 'number'], 110, [1, 2, 3]],
      [['bigint', 'string', 'null'], 98, [2, 0, 0]], [['undefined'], 117, [3, 0, 2]], [['?number'], 63, [4, 0, 1]], [['number'], 110, [0, 0, 0]]]) {
      assert.deepEqual(Array.from(api.readNullableTypeSyntax(parts, first)), expected);
    }
    assert.equal(api.readNullableTypeAbsence(3), 'both');
    assert.equal(api.readNullableTypeTargetRefusal('Rust'), 'no Rust type for absence');
  }
});

test('actual nullable JSDoc headers retain their absence tags through source-free text and binary graphs', () => {
  for (const [spelling, absence] of [['bigint|null', 'null'], ['undefined|bigint', 'undefined'], ['bigint|null|null', 'both'], ['?bigint', 'null']]) {
    const source = `/** @param {${spelling}} unused @returns {bigint} */function choose(unused){return 1n;}console.log(String(choose(null)));`;
    const input = absence === 'undefined' ? source.replace('choose(null)', 'choose(undefined)') : source;
    assert.deepEqual(observe(input), ['1']);
    const saved = createTypedProgramLinks(input, 'JavaScript');
    for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
      const program = decodeTypedProgramLinks(network, saved.root);
      const fn = program.declarations.get('choose');
      assert.equal(fn.params[0].type.kind, 'option');
      assert.equal(fn.params[0].type.absence, absence);
      assert.deepEqual(observe(emitJavaScript(program).text), ['1']);
      assert.equal(fn.body.k, 'lit');
      fn.body.value = '7';
      const edited = encodeTypedProgramLinks(program);
      const reloaded = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(edited.network)), edited.root);
      assert.equal(reloaded.declarations.get('choose').params[0].type.absence, absence);
      assert.deepEqual(observe(emitJavaScript(reloaded).text), ['7']);
    }
  }
});
