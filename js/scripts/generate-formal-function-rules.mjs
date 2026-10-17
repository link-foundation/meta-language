// Translate the shared decisions automatically; recorded emitter decorators
// adapt the generated functions to the native borrowed-string library boundary.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';

const source = readFileSync(new URL('../src/formal-function-rules.js', import.meta.url), 'utf8');
const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
const styleFile = new URL('../../parity/self-translation/formal-function-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--update-styles')) {
  let styled = raw;
  const actions = [];
  const replace = (from, to) => {
    if (styled.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); styled = styled.split(from).join(to); }
  };
  for (const parameter of ['kind', 'label']) replace(`${parameter}: String`, `${parameter}: &str`);
  for (const match of [...styled.matchAll(/\([a-z_]+ (?:==|!=) "[^"\n]*"\)/gu)]) replace(match[0], match[0].slice(1, -1));
  for (const expression of [
    '(kind == "fn" || kind == "data")', '(label != "source span" && label != "provenance")',
  ]) replace(expression, expression.replace(/[()]/gu, ''));
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(styled);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1; let cursor = opening + 1; let quoted = false;
    for (; cursor < styled.length && depth; cursor += 1) {
      const character = styled[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('a translated formal function decision has an unclosed branch');
    replace(styled.slice(match.index, cursor), `} else ${styled.slice(opening + 1, cursor - 1).trim()}`);
  }
  for (const name of ['accept_formal_function_declaration', 'accept_formal_function_dependency', 'include_formal_function_relation']) {
    replace(`pub fn ${name}`, `/// Read a shared formal function graph decision.\n#[must_use]\npub fn ${name}`);
  }
  writeFileSync(styleFile, new DecoratorSet([{ id: 'formal-function-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8', maxBuffer: 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const generated = '// Generated automatically from js/src/formal-function-rules.js.\n'
  + '// Emitter styles are recorded in parity/self-translation/formal-function-rules-decorators.lino.\n'
  + '// Do not edit by hand.\n' + formatted.stdout;
const output = new URL('../../rust/src/formal_function_rules.rs', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('formal function link decisions are stale');
} else writeFileSync(output, generated);
