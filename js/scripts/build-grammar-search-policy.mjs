import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const source = readFileSync(new URL('../src/grammar-search-policy.js', import.meta.url), 'utf8');
const output = new URL('../../rust/src/grammar/feature_runtime/search_policy.rs', import.meta.url);
const translation = selfTranslate(source, 'JavaScript', 'Rust');
if (translation.items.some(({ status }) => status !== 'translated')
  || selfTranslate(translation.code, 'Rust', 'JavaScript').code !== source) {
  throw new Error('The grammar search rules did not translate losslessly');
}
const { definitions } = emitRust(checkProgram(parseJavaScript(source)));
const unstyled = definitions.join('\n\n');
const decorators = DecoratorSet.fromLino(readFileSync(new URL(
  '../../parity/self-translation/grammar-search-decorators.lino', import.meta.url,
), 'utf8'));
const { source: styled } = decorators.decorate('emitter', { format: 'Rust', source: unstyled });
if (styled === unstyled) throw new Error('The Rust grammar search style did not match');
const generated = '// Generated from the JavaScript grammar search policy.\n' + styled + '\n';
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('The Rust grammar search policy is stale');
} else {
  writeFileSync(output, generated);
}
