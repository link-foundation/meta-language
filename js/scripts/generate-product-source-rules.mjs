// Generate the native source-provenance solver through the checked translator.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

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
const decorators = DecoratorSet.fromLino(readFileSync(style, 'utf8'));
for (const action of decorators.decorators.flatMap(definition => definition.actions)) {
  const numericHeader = action.from === generatedAllowances && action.to === numericAllowances;
  const continuation = continuationStyles.some(style => action.from === style.from && action.to === style.to);
  const numericLiteral = numericLiteralStyles.some(style => action.from === style.from && action.to === style.to);
  if (action.op === 'replace' && action.field === 'source' && (numericHeader || continuation || numericLiteral)) continue;
  if (action.op !== 'replace' || action.field !== 'source' || !action.from.startsWith('crate::')
    || !declarations.has(action.from.slice('crate::'.length)) || action.to !== `self::${action.from.slice('crate::'.length)}`) {
    throw new Error('product source solver style must relocate an exact generated declaration');
  }
}
const decorated = decorators.decorate('emitter', { format: 'Rust', source: raw }).source;
if (/\bcrate::/u.test(decorated)) throw new Error('product source solver retained an unresolved crate-root reference');
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
  input: decorated, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024,
});
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const generated = '// Generated from js/src/translation/product-source-rules.js by the checked translator.\n'
  + '// Styles: parity/self-translation/product-source-rules-decorators.lino.\n'
  + '// Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('native product source solver is stale');
} else writeFileSync(output, generated);
