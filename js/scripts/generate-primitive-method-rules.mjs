// Translate the complete primitive-method decision module through the checked frontend.
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { applyRecordedFamilyLibraryStyles } from './recorded-family-library-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const source = new URL('../src/translation/primitive-method-rules.js', import.meta.url);
const target = new URL('../../rust/src/translation/primitive_method_rules.rs', import.meta.url);
const emitted = emitRust(checkProgram(parseJavaScript(readFileSync(source, 'utf8'), { referenceSemantics: false })), { arrayConsumption: true });
if (emitted.preludes.length !== 0) throw new Error('primitive-method decisions must remain closed scalar operations');
const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new Error('the checked emitter omitted its generated module header');
const definitions = emitted.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
const raw = header + '\n\n' + definitions;
const names = [...raw.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
const styleFile = new URL('../../parity/self-translation/primitive-method-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--check') && process.argv.includes('--update-library-lint-styles')) throw new Error('freshness checks cannot update recorded styles');
const document = readFileSync(styleFile, 'utf8');
const base = DecoratorSet.fromLino(document).decorate('emitter', { format: 'Rust', source: raw }).source;
const correction = applyRecordedFamilyLibraryStyles(document, formatProductRuleSource(base), {
  family: 'primitive-method', update: process.argv.includes('--update-library-lint-styles'), format: formatProductRuleSource,
  parameterLoans: {normalizeBorrowedArguments: true},
  baseRecordHashes: ['1d9aa8f1081ca8778572c8c33a29eb58d8c121125d6bf7dcab1a11f04dac168a'],
});
if (correction.document !== document) writeFileSync(styleFile, correction.document);
const decorated = correction.source;
if (names.some(name => decorated.includes('crate::' + name))) throw new Error('primitive methods retain an unresolved local declaration');
const text = '// Generated from js/src/translation/primitive-method-rules.js. Do not edit.\n' + formatProductRuleSource(decorated);
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== text) throw new Error('native primitive-method decisions are stale');
} else writeFileSync(target, text);
