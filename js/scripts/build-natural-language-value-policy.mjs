import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

for (const [name, decoratorsName, nativePath] of [
  ['natural-language-value-policy', 'natural-language-value', 'natural_language_values/value_policy'],
  ['morphology-policy', 'morphology', 'morphology/agreement_policy'],
]) {
  const source = readFileSync(new URL(`../src/${name}.js`, import.meta.url), 'utf8');
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.ok(translated.items.every(item => item.status === 'translated'));
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
  const definitions = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  const decorators = DecoratorSet.fromLino(readFileSync(new URL(`../../parity/self-translation/${decoratorsName}-decorators.lino`, import.meta.url), 'utf8'));
  const { source: policy } = decorators.decorate('emitter', { format: 'Rust', source: definitions });
  const output = new URL(`../../rust/src/link_network/${nativePath}.rs`, import.meta.url);
  const generated = `// Generated from js/src/${name}.js.\n${policy}\n`;
  if (process.argv.includes('--check')) assert.equal(readFileSync(output, 'utf8'), generated);
  else writeFileSync(output, generated);
}
