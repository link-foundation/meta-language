import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const input = new URL('../src/translation/rust-array-rules.js', import.meta.url);
const style = new URL('../../parity/self-translation/rust-array-rules-decorators.lino', import.meta.url);
const output = new URL('../../rust/src/translation/rust/array_rules.rs', import.meta.url);
const definitions = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8')))).definitions;
const raw = definitions.join('\n\n');
if (process.argv.includes('--update-styles')) {
  const actions = ['owner', 'kind', 'actual', 'expected', 'model'].map(name => ({ op: 'replace', field: 'source', from: `${name}: String`, to: `${name}: &str` }));
  const bodies = [
    '!shadowed && (owner == "Vec" || owner == "std::vec::Vec")',
    '!indirected && (kind == "bool" || kind == "float" || kind == "fixed" || kind == "unit" || kind == "string")',
    'actual == expected || (borrowed && actual == "owned vector" && expected == "immutable slice")',
    'model.is_empty() || model == "immutable fresh scalar vector results 1"',
    'model == "immutable fresh scalar vector results 1" && owned && fresh && (kind == "bool" || kind == "float" || kind == "fixed" || kind == "unit")',
    '(kind == "array" && constructor) || ((kind == "let" || kind == "if") && children)',
  ];
  for (const [index, definition] of definitions.entries()) {
    const name = /^pub fn (\w+)/u.exec(definition)?.[1];
    if (!name) throw new Error('the checked array admission has no declaration');
    actions.push({ op: 'replace', field: 'source', from: `pub fn ${name}(`, to: `/// Admit the selected immutable Rust array source model.\n#[must_use]\npub fn ${name}(` });
    const lines = definition.split('\n').filter(line => line.startsWith('    '));
    if (lines.length !== 1) throw new Error('the checked array admission body changed');
    actions.push({ op: 'replace', field: 'source', from: lines[0].trim(), to: bodies[index] });
  }
  writeFileSync(style, new DecoratorSet([{ id: 'rust-array-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
const generated = '// Generated from rust-array-rules.js by the checked translator.\n// Do not edit by hand.\n' + result.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('Rust source array rules are stale');
} else writeFileSync(output, generated);
