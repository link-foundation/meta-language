import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { translateProgram } from '../src/program-translation.js';
import { referenceOperation } from '../src/translation/reference-model.js';
import { readAbsentNumberValue, renderAbsentNumberExpression, readNumberDisplayRefusal, readNumberDisplayFunction, readScalarCellOperation } from '../src/translation/primitive-value-rules.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/translation-primitive-values.json', import.meta.url), 'utf8'));

test('absence conversion distinguishes arithmetic from missing array property selection', () => {
  for (const entry of fixture.absences) {
    assert.equal(readAbsentNumberValue(entry.operation, entry.absence), entry.value);
    for (const [target, expression] of Object.entries(entry.expressions)) assert.equal(renderAbsentNumberExpression(target, entry.operation, entry.absence), expression, `${target}:${entry.operation}:${entry.absence}`);
    const value = entry.absence === 'null' ? null : undefined;
    const rendered = runInNewContext(entry.expressions.JavaScript);
    if (entry.operation === 'arithmetic') assert.ok(Object.is(Number(value), rendered));
    else assert.equal([7, 9][value], [7, 9][rendered]);
  }
  assert.equal(readAbsentNumberValue('unknown', 'null'), '');
  assert.equal(readAbsentNumberValue('arithmetic', 'both'), '');
  assert.equal(renderAbsentNumberExpression('unknown', 'arithmetic', 'null'), '');
});

test('shared scalar-cell selectors resolve executable linked operations', () => {
  for (const entry of fixture.cells) {
    assert.equal(readScalarCellOperation(entry.expression), entry.operation);
    assert.equal(referenceOperation(readScalarCellOperation(entry.expression)).id, entry.operation);
  }
  assert.equal(readScalarCellOperation('unknown'), '');
});

test('both source-display paths refuse Rust f64 text while nonprinting arithmetic remains faithful', () => {
  assert.equal(readNumberDisplayRefusal('Rust', 'float'), fixture.displayRefusal);
  assert.equal(readNumberDisplayRefusal('JavaScript', 'float'), '');
  assert.equal(readNumberDisplayRefusal('Rust', 'int'), '');
  assert.equal(readNumberDisplayFunction('float', false), 'js_number');
  assert.equal(readNumberDisplayFunction('float', true), 'js_console');
  assert.equal(readNumberDisplayFunction('int', false), '');
  for (const entry of fixture.rustFloatCases) {
    const printed = `fn main() { println!("{}", ${entry.expression}); }`;
    for (const target of ['JavaScript', 'Lean', 'Rocq']) {
      const result = translateProgram(printed, 'Rust', target);
      assert.equal(result.diagnostic?.kind, 'unsupported', `${entry.name}:${target}`);
      assert.ok(result.diagnostic.message.startsWith(`Rust Float display: ${fixture.displayRefusal}`), `${entry.name}:${target}`);
    }
    const source = `pub fn value() -> f64 { ${entry.expression} } fn main() {}`;
    const translated = translateProgram(source, 'Rust', 'JavaScript');
    assert.equal(translated.diagnostic, null, entry.name);
    const actual = runInNewContext(`${translated.code}\nvalue()`);
    assert.equal(Object.is(actual, -0) ? '-0' : String(actual), entry.expected, entry.name);
  }
  const stringified = translateProgram('pub fn text(value: f64) -> String { value.to_string() } fn main() {}', 'Rust', 'JavaScript');
  assert.equal(stringified.diagnostic?.kind, 'unsupported');
  assert.ok(stringified.diagnostic.message.startsWith(`Rust Float display: ${fixture.displayRefusal}`));
});
