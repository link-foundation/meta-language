import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { readProjectData } from './project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { stripRustSourceHeaderStyleParentheses, flattenRustSourceHeaderStyleFormat } from './rust-source-header-library-style.mjs';

const policyFile = new URL('../src/translation/data/rust-borrowed-call-interface.lino', import.meta.url);
const policy = readProjectData(policyFile);
const javascript = new URL('../src/translation/rust-borrowed-call-interface-rules.js', import.meta.url);
const native = new URL('../../rust/src/translation/borrowed_call_interface_rules.rs', import.meta.url);
const copy = new URL('../../rust/src/data/rust-borrowed-call-interface.lino', import.meta.url);
const style = new URL('../../parity/self-translation/rust-borrowed-call-interface-rules-decorators.lino', import.meta.url);
const strings = names => names.map(name => `@param {string} ${name}`).join(' ');
const functionSource = (name, parameters, result, body) => `/** ${parameters} @returns {${result}} */\nexport function ${name}(${[...parameters.matchAll(/@param \{[^}]+\} (\w+)/gu)].map(match => match[1]).join(', ')}) { ${body} }\n`;
const rule = (name, names, body) => functionSource(name, strings(names), 'string', body);
let source = '// Generated from the linked closed primitive borrowed Rust call interface policy.\n';
source += functionSource('acceptRustBorrowedCallInterfaceModel', strings(['model']), 'boolean', `return model === ${JSON.stringify(policy.model)};`);
source += functionSource('acceptRustBorrowedCallTarget', '@param {boolean} owned @param {boolean} leaf @param {boolean} staticLifetime', 'boolean', 'return owned && leaf && !staticLifetime;');
source += functionSource('acceptRustBorrowedCallShape', `${strings(['kind'])} @param {boolean} unrepresented @param {boolean} correctArity`, 'boolean', `return kind === ${JSON.stringify(policy.callerKind)} && !unrepresented && correctArity;`);
source += functionSource('acceptRustBorrowedCallInventory', '@param {boolean} hasCaller', 'boolean', `return ${policy.requireCaller ? 'hasCaller' : 'true'};`);
source += functionSource('acceptRustBorrowedCallType', '@param {boolean} argumentTypes @param {boolean} resultTypes', 'boolean', 'return argumentTypes && resultTypes;');
source += functionSource('renderRustBorrowedCallHeaderReference', `${strings(['lifetime', 'inner'])} @param {boolean} innerReference`, 'string', 'return lifetime === "" ? "&" + (innerReference ? " " : "") + inner : "&\'" + lifetime + " " + inner;');
source += rule('renderRustBorrowedCallReference', ['value'], 'return "&(" + value + ")";');
source += rule('renderRustBorrowedCallInvocation', ['head', 'argumentsText'], 'return head + "(" + argumentsText + ")";');
source += rule('renderRustBorrowedCallBlock', ['body'], 'return "{\\n" + body + "\\n}";');

if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('borrowed call interface JavaScript rules are stale');
} else writeFileSync(javascript, source);
if (process.argv.includes('--javascript-only')) {
  console.log('Rust borrowed call interface: generated JavaScript admission and rendering rules');
} else {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  if (process.argv.includes('--update-styles')) {
    const actions = []; let current = raw;
    const replace = (from, to) => {
      if (current.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); current = current.split(from).join(to); }
    };
    for (const name of ['model', 'kind', 'lifetime', 'inner', 'value', 'head', 'arguments_text', 'body']) replace(`${name}: String`, `${name}: &str`);
    for (const match of raw.matchAll(/^pub fn (\w+)/gmu)) replace(`pub fn ${match[1]}`, `/// Apply the shared closed primitive borrowed call interface policy.\n#[must_use]\npub fn ${match[1]}`);
    replace('lifetime == ""', 'lifetime.is_empty()'); replace('String::from("")', 'String::new()');
    replace('if (lifetime.is_empty())', 'if lifetime.is_empty()'); replace('if (inner_reference)', 'if inner_reference');
    for (const definition of current.matchAll(/pub fn accept_rust_borrowed_call_[\s\S]*?\n\}/gu)) {
      const block = definition[0]; const open = block.indexOf('{');
      let expression = stripRustSourceHeaderStyleParentheses(block.slice(open + 1, -1));
      expression = expression.replace(/\((\w+\s*==\s*"[^"]*")\)/gu, '$1');
      replace(block, `${block.slice(0, open + 1)}\n    ${expression}\n}`);
    }
    for (let pass = 0; pass < 30; pass += 1) {
      let changed = false;
      for (const match of [...current.matchAll(/format!\("\{\}\{\}", /gu)].reverse()) {
        const start = match.index; let depth = 1; let end = start + 8; let quoted = false; let escaped = false;
        for (; end < current.length; end += 1) {
          const character = current[end];
          if (quoted) { if (escaped) escaped = false; else if (character === '\\') escaped = true; else if (character === '"') quoted = false; continue; }
          if (character === '"') quoted = true;
          else if (character === '(') depth += 1;
          else if (character === ')' && --depth === 0) break;
        }
        const from = current.slice(start, end + 1); const to = flattenRustSourceHeaderStyleFormat(from);
        if (from !== to) { replace(from, to); changed = true; }
      }
      if (!changed) break;
    }
    writeFileSync(style, new DecoratorSet([{ id: 'rust-borrowed-call-interface-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
  }
  const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const generated = '// Generated through the checked JavaScript translator. Do not edit.\n' + formatted.stdout;
  if (process.argv.includes('--check')) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('native borrowed call interface rules or policy are stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
  console.log('Rust borrowed call interface: shared classifiers and renderers translated automatically');
}
