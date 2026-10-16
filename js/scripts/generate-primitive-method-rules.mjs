// Translate the complete primitive-method decision module through the checked frontend.
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles } from './portable-source-library-style.mjs';
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
const actions = names.filter(name => raw.includes('crate::' + name)).map(name => ({
  op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name,
}));
const relocated = new DecoratorSet([{ id: 'primitive-method-relocation', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }])
  .decorate('emitter', { format: 'Rust', source: raw }).source;
const loans = readPortableSourceParameterStyles(relocated);
actions.push(...loans.actions);
const reviewed = readPortableSourceLintStyles(loans.source);
actions.push(...reviewed.actions);
const styles = new DecoratorSet([{ id: 'primitive-method-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
const styleFile = new URL('../../parity/self-translation/primitive-method-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(styleFile, 'utf8') !== styles.toLino()) throw new Error('primitive method library relocation rules are stale');
} else writeFileSync(styleFile, styles.toLino());
const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
if (decorated !== reviewed.source) throw new Error('recorded primitive method styles must reproduce the complete reviewed source');
if (names.some(name => decorated.includes('crate::' + name))) throw new Error('primitive methods retain an unresolved local declaration');
const text = '// Generated from js/src/translation/primitive-method-rules.js. Do not edit.\n' + formatProductRuleSource(decorated);
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== text) throw new Error('native primitive-method decisions are stale');
} else writeFileSync(target, text);
