import assert from 'node:assert/strict';
import { test } from 'node:test';

import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';
import { translateProgram } from '../src/program-translation.js';

const recursiveSources = [
  'function width(expression){return expression.kind === "repeat" ? width(expression.item) * expression.min : 0;}',
  'function width(expression){switch(expression.kind){case "repeat":return width(expression.item)*expression.min;default:return 0;}}',
];

for (const [index, source] of recursiveSources.entries()) {
  test(`recursive inferred record ${index + 1} produces a finite obligation and restores its complete source`, () => {
    const program = translateProgram(source, 'JavaScript', 'Rust');
    assert.ok(program.diagnostic, 'a recursive structural type requires an explicit representable type');
    assert.match(program.diagnostic.message, /recursive/u);
    const translated = selfTranslate(source, 'JavaScript', 'Rust');
    assert.equal(translated.items.length, 1);
    assert.equal(translated.items[0].status, 'carried');
    assert.equal(translated.items[0].start, 0);
    assert.equal(translated.items[0].end, Buffer.byteLength(source));
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
    assert.deepEqual(selfTranslationSignatures(`export ${source}`, 'JavaScript'), []);
  });
}

test('an unsupported recursive sibling does not prevent an independent declaration from translating', () => {
  const source = `${recursiveSources[0]}\nfunction increment(number){return number+1;}\nfunction caller(expression){return width(expression);}\n`;
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  const item = (name) => translated.items.find(({ start, end }) => source.slice(start, end).includes(`function ${name}(`));
  assert.equal(item('width').status, 'carried');
  assert.equal(item('increment').status, 'translated');
  assert.equal(item('caller').status, 'carried');
  assert.equal(item('caller').reason, 'type');
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});
