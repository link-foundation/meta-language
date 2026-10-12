// Generate native Rust source product admission from the checked JavaScript model.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const input = new URL('../src/translation/rust-product-rules.js', import.meta.url);
const style = new URL('../../parity/self-translation/rust-product-rules-decorators.lino', import.meta.url);
const output = new URL('../../rust/src/translation/rust/product_rules.rs', import.meta.url);
const definitions = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8')))).definitions;
const raw = definitions.join('\n\n');
if (process.argv.includes('--update-styles')) {
  const actions = [];
  for (const name of ['accept_rust_immutable_product_type', 'accept_rust_tuple_index_spelling']) {
    actions.push({ op: 'replace', field: 'source', from: `pub fn ${name}`, to: `/// Admit the corresponding immutable Rust source product.\n#[must_use]\npub fn ${name}` });
  }
  for (const name of ['kind', 'text']) actions.push({ op: 'replace', field: 'source', from: `${name}: crate::ml_string::Text`, to: `${name}: &str` });
  // This classifier removes ASCII digits and asks only whether anything remains.
  // A non-ASCII code unit always survives, so byte length and UTF-16 length give
  // the same empty/one-character decisions relevant to its Boolean result.
  for (const literal of raw.matchAll(/crate::ml_string::Text::from_units\(vec!\[([\d, ]*)\]\)/gu)) {
    const units = literal[1].trim() ? literal[1].split(',').map(unit => Number(unit.trim())) : [];
    if (units.some(unit => unit > 127)) throw new Error('product style only admits ASCII decision literals');
    actions.push({ op: 'replace', field: 'source', from: literal[0], to: JSON.stringify(String.fromCharCode(...units)) });
  }
  actions.push({ op: 'replace', field: 'source', from: '.length()', to: '.len()' });
  for (const value of ['0', '1']) actions.push({ op: 'replace', field: 'source', from: `${value}f64`, to: value });
  actions.push({ op: 'replace', field: 'source', from: '.starts_with(&"0")', to: '.starts_with("0")' });
  for (let digit = 0; digit <= 9; digit += 1) actions.push({ op: 'replace', field: 'source', from: `.replace(&"${digit}", &"", true)`, to: `.replace("${digit}", "")` });
  const nativeBodies = [
    '!indirected && (kind == "bool" || kind == "float" || kind == "fixed" || kind == "unit" || (kind == "tuple" && children_accepted))',
    'if text.is_empty() || (text.len() > 1 && text.starts_with("0")) {',
    `text.replace([${Array.from({ length: 10 }, (_, digit) => `'${digit}'`).join(', ')}], "").is_empty()`,
  ];
  const intermediate = new DecoratorSet([{ id: 'rust-product-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).decorate('emitter', { format: 'Rust', source: raw }).source;
  const bodies = intermediate.split('\n').filter(line => line.startsWith('    ') && !['false', '} else {', '}'].includes(line.trim()));
  if (bodies.length !== nativeBodies.length) throw new Error('unexpected Rust product admission bodies');
  for (const [index, line] of bodies.entries()) actions.push({ op: 'replace', field: 'source', from: line.trim(), to: nativeBodies[index] });
  writeFileSync(style, new DecoratorSet([{ id: 'rust-product-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
if (decorated.includes('crate::ml_string')) throw new Error('product decision style left an unbound string runtime');
const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
const generated = '// Generated from rust-product-rules.js by the checked translator.\n// Do not edit by hand.\n' + result.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('Rust source product rules are stale');
} else writeFileSync(output, generated);
