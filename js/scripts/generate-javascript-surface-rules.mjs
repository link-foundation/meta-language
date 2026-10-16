// Translate the complete JavaScript source-decision module without a native implementation copy.
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles } from './portable-source-library-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const input = new URL('../src/translation/javascript-surface-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/javascript/surface_rules.rs', import.meta.url);
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'), { referenceSemantics: false })), { arrayConsumption: true });
if (translated.preludes.length !== 0) throw new Error('source decisions must remain closed scalar operations');
const header = translated.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new Error('the checked emitter omitted its generated module header');
const definitions = translated.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
const raw = header + '\n\n' + definitions;
const loans = readPortableSourceParameterStyles(raw);
const reviewed = readPortableSourceLintStyles(loans.source);
const styles = new DecoratorSet([{ id: 'javascript-surface-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions: [...loans.actions, ...reviewed.actions] }]);
const styleFile = new URL('../../parity/self-translation/javascript-surface-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(styleFile, 'utf8') !== styles.toLino()) throw new Error('JavaScript surface library styles are stale');
} else writeFileSync(styleFile, styles.toLino());
const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
if (decorated !== reviewed.source) throw new Error('recorded JavaScript surface styles must reproduce the complete reviewed source');
const text = '// Generated from js/src/translation/javascript-surface-rules.js by\n'
  + '// js/scripts/generate-javascript-surface-rules.mjs. Do not edit by hand.\n'
  + formatProductRuleSource(decorated);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('native JavaScript source decisions are stale');
} else writeFileSync(output, text);
