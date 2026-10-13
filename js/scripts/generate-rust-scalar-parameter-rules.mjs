// Translate the JavaScript policy; replay only reviewed emitter spelling edits.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const input = new URL('../src/translation/rust-scalar-parameter-rules.js', import.meta.url);
const style = new URL('../../parity/self-translation/rust-scalar-parameter-rules-decorators.lino', import.meta.url);
const output = new URL('../../rust/src/translation/rust/scalar_parameter_rules.rs', import.meta.url);
const definitions = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8')))).definitions;
const raw = definitions.join('\n\n');
if (process.argv.includes('--update-styles')) {
  const actions = ['kind', 'text'].map(name => ({ op: 'replace', field: 'source', from: `${name}: String`, to: `${name}: &str` }));
  const bodies = [
    'direct_copy && !shadowed && closed && !possible_write && (kind == "bool" || kind == "float" || kind == "fixed")',
    'kind == "punct" && (text == "=" || text == "+=" || text == "-=" || text == "*=" || text == "/=" || text == "%=" || text == "&=" || text == "|=" || text == "^=" || text == "<<=" || text == ">>=")',
  ];
  if (definitions.length !== bodies.length) throw new Error('the checked scalar parameter policy declarations changed');
  for (const [index, definition] of definitions.entries()) {
    const name = /^pub fn (\w+)/u.exec(definition)?.[1];
    const lines = definition.split('\n').filter(line => line.startsWith('    '));
    if (!name || lines.length !== 1) throw new Error('the checked scalar parameter policy body changed');
    actions.push({ op: 'replace', field: 'source', from: `pub fn ${name}`, to: `/// Evaluate the selected unwritten scalar parameter premise.\n#[must_use]\npub fn ${name}` });
    actions.push({ op: 'replace', field: 'source', from: lines[0].trim(), to: bodies[index] });
  }
  const styled = new DecoratorSet([{ id: 'rust-scalar-parameter-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).decorate('emitter', { format: 'Rust', source: raw }).source;
  const header = /pub fn accept_rust_unwritten_scalar_parameter\([^)]*\) -> bool \{/u.exec(styled)?.[0];
  if (!header) throw new Error('the checked scalar parameter declaration is absent');
  actions.push({ op: 'replace', field: 'source', from: header,
    to: 'pub fn accept_rust_unwritten_scalar_parameter(kind: &str, (direct_copy, shadowed, closed, possible_write): (bool, bool, bool, bool)) -> bool {' });
  writeFileSync(style, new DecoratorSet([{ id: 'rust-scalar-parameter-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
const generated = '// Generated from rust-scalar-parameter-rules.js by the checked translator.\n// Do not edit by hand.\n' + result.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('Rust scalar parameter rules are stale');
} else writeFileSync(output, generated);
