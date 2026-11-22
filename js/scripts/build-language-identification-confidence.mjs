import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

export function buildLanguageIdentificationConfidence(check) {
  const input = new URL('../src/language-identification-confidence.js', import.meta.url);
  const output = new URL('../../rust/src/language_identification/confidence.rs', import.meta.url);
  const styles = new URL('../../parity/self-translation/language-identification-confidence-decorators.lino', import.meta.url);
  const source = readFileSync(input, 'utf8');
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  if (translation.items.some(({ status }) => status !== 'translated')
    || selfTranslate(translation.code, 'Rust', 'JavaScript').code !== source) {
    throw new Error('The language confidence rules did not translate losslessly');
  }
  const { definitions } = emitRust(checkProgram(parseJavaScript(source)));
  const unstyled = formatProductRuleSource(definitions.join('\n\n'));
  const decorators = DecoratorSet.fromLino(readFileSync(styles, 'utf8'));
  const { source: styled } = decorators.decorate('emitter', { format: 'Rust', source: unstyled });
  if (styled === unstyled) throw new Error('The Rust confidence style did not match');
  const generated = '// Generated from the JavaScript language identification confidence rules.\n'
    + formatProductRuleSource(styled);
  if (check) {
    if (readFileSync(output, 'utf8') !== generated) throw new Error('The Rust confidence rules are stale');
  } else {
    writeFileSync(output, generated);
  }
}
