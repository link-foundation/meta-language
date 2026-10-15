// Generate the ECMAScript line-boundary decision from shared links data.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const policy = {
  schemaVersion: 1,
  model: 'ECMAScript line terminators 1',
  source: 'https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-line-terminators',
  codeUnits: [10, 13, 8232, 8233],
  nativeComparison: 'For these four positive finite binary64 constants, equality of bit representations is exactly numeric equality. Neither signed zero nor any NaN is a member; all hosts supply actual UTF-16 code units.',
  scope: 'Single-line comment boundaries and the no-LineTerminator restrictions of supported JavaScript statements. Other language comment policies retain their existing boundaries.',
};
const policyFile = new URL('../src/translation/data/javascript-line-terminators.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(policyFile, policy);
else writeProjectData(policyFile, policy);
const source = '// Generated from the linked ECMAScript line-terminator policy. Do not edit.\n'
  + '/** @param {number} unit @returns {boolean} */\n'
  + 'export function acceptJavaScriptLineTerminator(unit) {\n'
  + '  return ' + policy.codeUnits.map(unit => `unit === ${unit}`).join(' || ') + ';\n}\n';
const javascript = new URL('../src/translation/javascript-line-terminator-rules.js', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('JavaScript line-terminator rule is stale');
} else writeFileSync(javascript, source);

if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  const opening = raw.indexOf('{');
  let body = raw.slice(opening + 1, raw.lastIndexOf('}')).trim();
  // Remove only parentheses enclosing the complete checked expression.
  while (body.startsWith('(') && body.endsWith(')')) {
    let depth = 0, complete = true;
    for (let index = 0; index < body.length - 1; index += 1) {
      if (body[index] === '(') depth += 1;
      else if (body[index] === ')') depth -= 1;
      if (!depth) { complete = false; break; }
    }
    if (!complete) break;
    body = body.slice(1, -1).trim();
  }
  let comparisons = 0;
  body = body.replace(/unit == \((\d+f64)\)/gu, (comparison, literal) => {
    if (!policy.codeUnits.includes(Number(literal.slice(0, -3)))) throw new Error('Unexpected line-terminator comparison');
    comparisons += 1;
    return `unit.to_bits() == ${literal}.to_bits()`;
  });
  if (comparisons !== policy.codeUnits.length) throw new Error('Line-terminator comparison inventory changed');
  const adapted = '/// Recognize the four ECMAScript line-terminator code units.\n#[must_use]\n'
    + raw.slice(0, opening + 1).replace('pub fn ', 'pub const fn ') + '\n    ' + body + '\n}';
  const styles = new DecoratorSet([{ id: 'javascript-line-terminator-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [{ op: 'replace', field: 'source', from: raw, to: adapted }] }]).toLino();
  const styleFile = new URL('../../parity/self-translation/javascript-line-terminator-rules-decorators.lino', import.meta.url);
  if (process.argv.includes('--update-styles')) writeFileSync(styleFile, styles);
  else if (readFileSync(styleFile, 'utf8') !== styles) throw new Error('JavaScript line-terminator library styles are stale');
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const native = new URL('../../rust/src/translation/javascript_line_terminator_rules.rs', import.meta.url);
  const copy = new URL('../../rust/src/data/javascript-line-terminators.lino', import.meta.url);
  const generated = '// Generated through JavaScript self-translation from the linked line-terminator policy. Do not edit.\n' + formatted.stdout;
  if (process.argv.includes('--check')) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('Native JavaScript line-terminator rule or policy is stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
}
console.log('JavaScript line-terminator policy and generated decisions agree');
