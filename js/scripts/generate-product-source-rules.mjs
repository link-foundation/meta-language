// Generate the native source-provenance solver through the checked translator.
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { formatProductRuleSource, readProductRuleStyleCorrections, readProductSourceConditionStyleCorrections } from './product-rule-style-corrections.mjs';

const input = new URL('../src/translation/product-source-rules.js', import.meta.url);
const style = new URL('../../parity/self-translation/product-source-rules-decorators.lino', import.meta.url);
const output = new URL('../../rust/src/translation/javascript/product_source_rules.rs', import.meta.url);
// The fixed point stores immutable array snapshots, without reference identity.
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'), { referenceSemantics: false })));
const generatedAllowances = translated.text.split('\n').find(line => line.startsWith('#![allow('));
if (!generatedAllowances) throw new Error('checked translator omitted its generated module header');
const raw = [generatedAllowances, ...translated.preludes, ...translated.definitions].join('\n\n');
const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
// These allowances are the existing frontend numeric model: Number equality
// and guarded numeric array-index casts, rather than blanket lint suppression.
const numericAllowances = generatedAllowances.replace(')]', ', clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]');
const numericLiteralStyles = [
  { from: '9007199254740991.0', to: '9_007_199_254_740_991.0' },
  { from: '9007199254740992.0', to: '9_007_199_254_740_992.0' },
].filter(style => raw.includes(style.from));
const continuationStyles = [...raw.matchAll(/^pub fn (ml_[A-Za-z_0-9]+)\(([^)]*)\)/gmu)]
  .filter(match => match[2].split(',').length > 7)
  .map(match => ({
    from: `pub fn ${match[1]}`,
    to: `// Checked closure lowering passes its captured state explicitly.\n#[allow(clippy::too_many_arguments)]\npub fn ${match[1]}`,
  }));
if (process.argv.includes('--update-styles')) {
  const actions = [{ op: 'replace', field: 'source', from: generatedAllowances, to: numericAllowances }];
  for (const name of declarations) {
    const reference = `crate::${name}`;
    if (raw.includes(reference)) actions.push({ op: 'replace', field: 'source', from: reference, to: `self::${name}` });
  }
  for (const { from, to } of continuationStyles) actions.push({ op: 'replace', field: 'source', from, to });
  for (const { from, to } of numericLiteralStyles) actions.push({ op: 'replace', field: 'source', from, to });
  writeFileSync(style, new DecoratorSet([{ id: 'product-source-solver-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
let decorators = DecoratorSet.fromLino(readFileSync(style, 'utf8'));
const reviewedId = 'reviewed-product-native-lints-285c8b13';
const conditionId = 'reviewed-product-source-condition-lints-26667280';
const base = new DecoratorSet(decorators.decorators.filter(definition => ![reviewedId, conditionId].includes(definition.id)));
for (const action of base.decorators.flatMap(definition => definition.actions)) {
  const numericHeader = action.from === generatedAllowances && action.to === numericAllowances;
  const continuation = continuationStyles.some(style => action.from === style.from && action.to === style.to);
  const numericLiteral = numericLiteralStyles.some(style => action.from === style.from && action.to === style.to);
  if (action.op === 'replace' && action.field === 'source' && (numericHeader || continuation || numericLiteral)) continue;
  if (action.op !== 'replace' || action.field !== 'source' || !action.from.startsWith('crate::')
    || !declarations.has(action.from.slice('crate::'.length)) || action.to !== `self::${action.from.slice('crate::'.length)}`) {
    throw new Error('product source solver style must relocate an exact generated declaration');
  }
}
const decorated = base.decorate('emitter', { format: 'Rust', source: raw }).source;
if (/\bcrate::/u.test(decorated)) throw new Error('product source solver retained an unresolved crate-root reference');
const formatted = formatProductRuleSource(decorated);
const actions = readProductRuleStyleCorrections(formatted, 'source', readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8'));
if (process.argv.includes('--update-styles')) {
  decorators = base.add({ id: reviewedId, level: 'emitter', order: 1000, when: [['format', 'Rust'], ['scope', 'formatted-product-library']], actions });
  writeFileSync(style, decorators.toLino());
}
const recorded = decorators.decorators.find(definition => definition.id === reviewedId);
if (JSON.stringify(recorded?.actions) !== JSON.stringify(actions)) throw new Error('reviewed product source styles are stale; regenerate with --update-styles');
const reviewed = new DecoratorSet([recorded]).decorate('emitter', { format: 'Rust', scope: 'formatted-product-library', source: formatted }).source;
const conditionSource = formatProductRuleSource(reviewed);
const conditionActions = readProductSourceConditionStyleCorrections(conditionSource);
if (conditionActions.length !== 1 || !conditionActions[0].from.startsWith('pub fn solve_product_source_facts(')) throw new Error('CI18 product condition context changed');
if (process.argv.includes('--update-styles')) {
  decorators = decorators.add({ id: conditionId, level: 'emitter', order: 2000, when: [['format', 'Rust'], ['scope', 'formatted-product-source-conditions']], actions: conditionActions });
  writeFileSync(style, decorators.toLino());
}
const conditionRecorded = decorators.decorators.find(definition => definition.id === conditionId);
if (JSON.stringify(conditionRecorded?.actions) !== JSON.stringify(conditionActions)) throw new Error('reviewed CI18 product condition styles are stale; regenerate with --update-styles');
const conditions = new DecoratorSet([conditionRecorded]).decorate('emitter', { format: 'Rust', scope: 'formatted-product-source-conditions', source: conditionSource }).source;
const generated = '// Generated from js/src/translation/product-source-rules.js by the checked translator.\n'
  + '// Styles: parity/self-translation/product-source-rules-decorators.lino.\n'
  + '// Do not edit by hand.\n' + formatProductRuleSource(conditions);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('native product source solver is stale');
} else writeFileSync(output, generated);
