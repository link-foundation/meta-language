import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';

test('module-scope retries expose exactly the exported declarations with portable signatures', () => {
  const source = readFileSync(new URL('../src/translation/frontend-rules.js', import.meta.url), 'utf8');
  const names = Array.from(source.matchAll(/^export function (\w+)/gmu), match => match[1]).filter(name => name !== 'decodeUnicodeEscape');
  const signatures = selfTranslationSignatures(source, 'JavaScript');
  // The cross-module signature API currently excludes algebraic data types.
  assert.ok(!signatures.some(signature => signature.name === 'decodeUnicodeEscape'));
  assert.deepEqual(signatures.map(signature => signature.name), names);
  const imported = selfTranslate("import { readArrayMethodForm as form } from './frontend-rules.js';\n/** @returns {string} */\nexport function select() { return form('concat'); }\n", 'JavaScript', 'Rust', {
    imports: { './frontend-rules.js': signatures },
  });
  assert.ok(imported.items.every(item => item.status === 'translated'));
  assert.match(imported.code, /use crate::frontend_rules::read_array_method_form as form;/u);
});
