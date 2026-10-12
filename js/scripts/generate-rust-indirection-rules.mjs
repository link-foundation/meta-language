// Generate native source-layer admissions with the checked JavaScript translator.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const input = new URL('../src/translation/rust-indirection-rules.js', import.meta.url);
const style = new URL('../../parity/self-translation/rust-indirection-rules-decorators.lino', import.meta.url);
const output = new URL('../../rust/src/translation/rust/indirection_rules.rs', import.meta.url);
const definitions = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8')))).definitions;
const raw = definitions.join('\n\n');
if (process.argv.includes('--update-styles')) {
  const actions = [
    { op: 'replace', field: 'source', from: 'layer: String', to: 'layer: &str' },
    { op: 'replace', field: 'source', from: 'owner: String', to: 'owner: &str' },
    { op: 'replace', field: 'source', from: 'kind: String', to: 'kind: &str' },
    { op: 'replace', field: 'source', from: 'operation: String', to: 'operation: &str' },
  ];
  for (const name of ['accept_rust_immutable_indirection_layer', 'accept_rust_source_box_owner', 'accept_rust_source_indirection_argument', 'accept_rust_source_box_coercion_layer', 'read_rust_source_expression_origin']) {
    actions.push({ op: 'replace', field: 'source', from: `pub fn ${name}`, to: `/// Admit the corresponding immutable Rust source projection.\n#[must_use]\npub fn ${name}` });
  }
  // These functions contain only Boolean expressions, so parentheses added
  // by the general expression emitter can be removed without changing precedence.
  const booleanExpressions = [
    'layer == "reference" || layer == "box"',
    '!shadowed && (owner == "Box" || owner == "std::boxed::Box" || owner == "alloc::boxed::Box")',
    '!indirected || exact || (borrowed && coerced)',
    'initial && layer == "box"',
  ];
  for (const [index, line] of definitions.slice(0, 4).join('\n').split('\n').filter(line => line.startsWith('    ')).entries()) {
    const from = line.trim();
    const to = booleanExpressions[index];
    if (!to) throw new Error('unexpected indirection classifier body');
    if (from !== to) actions.push({ op: 'replace', field: 'source', from, to });
  }
  for (const expression of raw.matchAll(/\(kind == "[^"]+"\)/gu)) actions.push({ op: 'replace', field: 'source', from: expression[0], to: expression[0].slice(1, -1) });
  for (const line of definitions[4].split('\n').filter(line => /^\s*if /u.test(line))) {
    actions.push({ op: 'replace', field: 'source', from: line.trim(), to: line.trim().replace(/[()]/gu, '') });
  }
  writeFileSync(style, new DecoratorSet([{ id: 'rust-indirection-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
const generated = '// Generated from rust-indirection-rules.js by the checked translator.\n// Do not edit by hand.\n' + result.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('Rust source indirection rules are stale');
} else writeFileSync(output, generated);
