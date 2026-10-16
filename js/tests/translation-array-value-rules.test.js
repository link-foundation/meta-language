import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';

import { readArrayIterableForm, readReferenceMutationDiagnostic, selectOptionalArrayRead, selectOptionalArrayIndexNumber, renderOptionalArrayRead, renderStringPointIteration, renderCollectionIteration } from '../src/translation/array-value-rules.js';
import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

test('array absence admission preserves JavaScript defaults and explicit proved iterable reads', () => {
  for (const language of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    for (const optional of [false, true]) for (const proven of [false, true]) for (const expected of [false, true]) {
      assert.equal(selectOptionalArrayRead(language, optional, proven, expected), optional || language === 'JavaScript' && !proven || expected);
    }
    for (const kind of ['float', 'option', 'unit', 'int']) for (const element of ['float', 'unit', 'int', 'bool']) {
      assert.equal(selectOptionalArrayIndexNumber(language, kind, element), language === 'JavaScript' && kind === 'option' && ['float', 'unit'].includes(element));
    }
  }
});

test('shared array read rendering retains each existing target expression', () => {
  const expected = {
    JavaScript: ['(xs)[i]', '(xs)[i]', '(xs)[i]'],
    Rust: ['crate::ml_array::optional_at(xs, crate::ml_array::number_index(i))', 'crate::ml_array::optional_at(xs, i.to_index())', 'crate::ml_array::optional_at(xs, i.to_index())'],
    Lean: ['(let ml_index := i; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then xs[ml_index.toUInt64.toNat]? else none)', '(xs[i]?)', '(let ml_index := i; if ml_index < 0 then none else xs[ml_index.toNat]?)'],
    Rocq: ['(let ml_index := (ml_float_index i) in if Z.ltb ml_index 0 then None else List.nth_error xs (Z.to_nat ml_index))', '(let ml_index := (Z.of_N i) in if Z.ltb ml_index 0 then None else List.nth_error xs (Z.to_nat ml_index))', '(let ml_index := i in if Z.ltb ml_index 0 then None else List.nth_error xs (Z.to_nat ml_index))'],
  };
  for (const [target, values] of Object.entries(expected)) ['float', 'nat', 'int'].forEach((kind, index) => assert.equal(renderOptionalArrayRead(target, kind, 'xs', 'i'), values[index]));
});

test('complete array decision source self-translates while ordinary absent reads remain observable', () => {
  const source = readFileSync(new URL('../src/translation/array-value-rules.js', import.meta.url), 'utf8');
  const checked = checkProgram(parseJavaScript(source));
  assert.equal(emitRust(checked).definitions.length, 7);
  const javascript = emitJavaScript(checked).text;
  assert.equal(vm.runInNewContext(javascript + '\nselectOptionalArrayRead("JavaScript", false, false, false);', {}, { timeout: 1000 }), true);
  const program = 'function read(xs, i) { return xs[i]; }\nconsole.log(read([1], 4) ?? 9);\n';
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translation = translateProgram(program, 'JavaScript', target);
    assert.equal(translation.diagnostic, null, target);
    assert.equal(translation.contract.assumptions.some(value => value.includes('array reads are in bounds')), false, target);
  }
});

test('iterable kind admission preserves mutable collection refusal and shared mutation diagnostics', () => {
  assert.equal(readArrayIterableForm('array', true), 'array');
  assert.equal(readArrayIterableForm('string', false), 'string');
  for (const kind of ['set', 'map']) {
    assert.equal(readArrayIterableForm(kind, false), kind);
    assert.equal(readArrayIterableForm(kind, true), 'mutable');
  }
  assert.equal(readArrayIterableForm('bool', false), '');
  assert.equal(readReferenceMutationDiagnostic('feature'), 'reference mutation');
  assert.equal(readReferenceMutationDiagnostic('reason'), 'this mutation requires its shared reference and slot operations to be represented');
  const mutation = translateProgram('const xs = [1, 2];\nxs[0] = 3;\n', 'JavaScript', 'Rust');
  assert.equal(mutation.diagnostic.message, 'reference mutation: this mutation requires its shared reference and slot operations to be represented at 19..28');
});

test('shared iterable rendering preserves code points, entry order and one receiver evaluation', () => {
  assert.equal(renderStringPointIteration('Rust', false, 's'), 's.chars().map(|ml_character| ml_character.to_string()).collect::<Vec<String>>()');
  assert.equal(renderStringPointIteration('Rust', true, 's'), 's.code_points()');
  assert.equal(renderStringPointIteration('Lean', false, 's'), '((s.toList.map (fun ml_character => String.singleton ml_character)).toArray)');
  assert.equal(renderStringPointIteration('Rocq', false, 's'), '');
  assert.equal(renderCollectionIteration('Rust', true, 'entries', 'xs', '', ''), 'xs');
  assert.equal(renderCollectionIteration('Rust', false, 'entries', 'xs', '', ''), 'xs.iter().map(|ml_value| (ml_value.clone(), ml_value.clone())).collect::<Vec<_>>()');
  assert.equal(renderCollectionIteration('Lean', true, 'keys', 'xs', '', ''), '(xs.map (fun ml_pair => ml_pair.1))');
  assert.equal(renderCollectionIteration('Rocq', true, 'values', 'xs', 'A', 'B'), '(List.map (@snd A B) xs)');
  const iteration = renderStringPointIteration('JavaScript', false, 'read()');
  assert.equal(vm.runInNewContext('let calls=0; function read(){ calls+=1; return "a😀b"; } const points=' + iteration + '; calls+":"+points.length;', {}, {timeout:1000}), '1:3');
});
