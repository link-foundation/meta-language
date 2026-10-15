// Generate a narrow Rust-only numeric boundary decision from inspectable links.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData, writeProjectData, checkProjectData } from './project-data.mjs';

const schema = readProjectData(new URL('../src/data/typed-program-links-schema.lino', import.meta.url));
const policy = {
  schemaVersion: 1,
  model: 'Rust decimal integer suffix separator 1',
  integerSuffixes: [...schema.machineIntegerWidths.flatMap(width => ['u' + width, 'i' + width]), 'usize', 'isize'],
  source: 'https://doc.rust-lang.org/reference/tokens.html#integer-literals',
  sourceRule: 'lex.token.literal.int.kind-dec and lex.token.literal.int.suffix',
  admittedBoundary: 'A trailing decimal separator may precede a complete supported integer suffix.',
  retainedGap: 'Unsuffixed trailing decimal separators retain the existing portable frontend refusal, including the protected 1_ fixture.',
  scope: 'Existing floating boundaries and non-Rust lexers are unchanged. Radix and arbitrary suffix admission are separate grammar gaps.',
};
const policyFile = new URL('../src/translation/data/rust-number-literals.lino', import.meta.url);
if (process.argv.includes('--check')) {
  if (!checkProjectData(policyFile, policy)) throw new Error('Rust number boundary policy is stale');
} else writeProjectData(policyFile, policy);
const source = '// Generated from the linked Rust numeric boundary policy. Do not edit.\n'
  + '/** @param {boolean} floating @param {boolean} trailingSeparator @param {string} suffix @param {boolean} continuedSuffix @returns {boolean} */\n'
  + 'export function acceptRustNumberDigitBoundary(floating, trailingSeparator, suffix, continuedSuffix) {\n'
  + '  return floating || !trailingSeparator || (!continuedSuffix && ('
  + policy.integerSuffixes.map(suffix => 'suffix === ' + JSON.stringify(suffix)).join(' || ') + '));\n}\n';
const javascript = new URL('../src/translation/rust-number-rules.js', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('Rust numeric JavaScript rules are stale');
} else writeFileSync(javascript, source);
if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  const styleFile = new URL('../../parity/self-translation/rust-number-rules-decorators.lino', import.meta.url);
  const opening = raw.indexOf('{');
  let body = raw.slice(opening + 1, raw.lastIndexOf('}')).trim();
  // Remove only parentheses enclosing the complete checked return expression.
  for (;;) {
    if (body[0] !== '(' || body.at(-1) !== ')') break;
    let depth = 0, encloses = true;
    for (let index = 0; index < body.length - 1; index += 1) {
      if (body[index] === '(') depth += 1;
      else if (body[index] === ')') depth -= 1;
      if (!depth) { encloses = false; break; }
    }
    if (!encloses) break;
    body = body.slice(1, -1).trim();
  }
  const header = raw.slice(0, opening + 1).replace('suffix: String', 'suffix: &str');
  const adapted = '/// Admit the shared Rust decimal integer suffix separator boundary.\n#[must_use]\n'
    + header + '\n    ' + body + '\n}';
  const styles = new DecoratorSet([{ id: 'rust-number-boundary-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [{ op: 'replace', field: 'source', from: raw, to: adapted }] }]).toLino();
  if (process.argv.includes('--update-styles')) writeFileSync(styleFile, styles);
  else if (readFileSync(styleFile, 'utf8') !== styles) throw new Error('Rust numeric library styles are stale');
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const generated = '// Generated from the linked Rust numeric policy through JavaScript self-translation. Do not edit.\n' + formatted.stdout;
  const native = new URL('../../rust/src/translation/lexer/number_rules.rs', import.meta.url);
  const copy = new URL('../../rust/src/data/rust-number-literals.lino', import.meta.url);
  if (process.argv.includes('--check')) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('Rust numeric native rule or policy is stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
}
console.log('Rust numeric boundary: shared supported integer suffixes; protected unsuffixed separator refusal retained');
