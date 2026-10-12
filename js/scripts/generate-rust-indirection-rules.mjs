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
  for (const name of ['accept_rust_immutable_indirection_layer', 'accept_rust_source_box_owner', 'accept_rust_source_indirection_argument', 'accept_rust_source_box_coercion_layer', 'read_rust_source_expression_origin', 'accept_rust_source_box_availability', 'accept_rust_source_text_equality']) {
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
  for (const line of definitions[5].split('\n').filter(line => line.startsWith('    '))) {
    const from = line.trim();
    const to = 'owner != "alloc::boxed::Box" || declared';
    if (from !== to) actions.push({ op: 'replace', field: 'source', from, to });
  }
  for (const line of definitions[6].split('\n').filter(line => line.startsWith('    '))) {
    const from = line.trim();
    const to = '(operation == "eq" || operation == "ne") && ((left_direct_string && right_borrowed_slice) || (right_direct_string && left_borrowed_slice))';
    if (from !== to) actions.push({ op: 'replace', field: 'source', from, to });
  }
  const intermediate = new DecoratorSet([{ id: 'rust-indirection-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).decorate('emitter', { format: 'Rust', source: raw }).source;
  const sourceClassifier = /^pub fn read_rust_source_expression_origin\([\s\S]*?^\}/mu.exec(intermediate)?.[0];
  if (!sourceClassifier) throw new Error('generated source origin classifier is absent');
  const normalizedClassifier = sourceClassifier.split('\n').map(line => {
    const condition = /^(\s*if )(.+)( \{)$/u.exec(line);
    if (!condition) return line;
    let expression = condition[2];
    for (;;) {
      if (!expression.startsWith('(') || !expression.endsWith(')')) break;
      let depth = 0;
      let closing = -1;
      for (let index = 0; index < expression.length; index += 1) {
        if (expression[index] === '(') depth += 1;
        if (expression[index] === ')') depth -= 1;
        if (depth === 0) { closing = index; break; }
      }
      if (closing !== expression.length - 1) break;
      expression = expression.slice(1, -1);
    }
    return condition[1] + expression + condition[3];
  }).join('\n');
  if (normalizedClassifier !== sourceClassifier) actions.push({ op: 'replace', field: 'source', from: sourceClassifier, to: normalizedClassifier });
  writeFileSync(style, new DecoratorSet([{ id: 'rust-indirection-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
const generated = '// Generated from rust-indirection-rules.js by the checked translator.\n// Do not edit by hand.\n' + result.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('Rust source indirection rules are stale');
} else writeFileSync(output, generated);
