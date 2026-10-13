// Derive the admission policy for readonly source queries in both packages.
import { readFileSync, writeFileSync } from 'node:fs';
import { readProjectData } from './project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const policyFile = new URL('../src/translation/data/rust-read-only-queries.lino', import.meta.url);
const policy = readProjectData(policyFile);
const source = '// Generated from the linked Rust readonly query policy. Do not edit.\n'
  + '/** @param {string} query @param {string} kind @param {string} path @returns {boolean} */\n'
  + 'export function acceptRustReadOnlyQueryOwner(query, kind, path) {\n'
  + Object.entries(policy.owners).map(([query, owners]) => `  if (query === ${JSON.stringify(query)}) return ${owners.map(([kind, path]) => `(kind === ${JSON.stringify(kind)} && path === ${JSON.stringify(path)})`).join(' || ')};`).join('\n')
  + '\n  return false;\n}\n'
  + '/** @param {string} kind @param {string} model @param {boolean} proven @param {string} layer @param {boolean} borrowed @param {boolean} shadowed @returns {boolean} */\n'
  + `export function acceptRustInclusiveRangeOperand(kind, model, proven, layer, borrowed, shadowed) {\n  return proven && !shadowed && kind === ${JSON.stringify(policy.range.kind)} && model === ${JSON.stringify(policy.range.model)} && ((!borrowed && layer === ${JSON.stringify(policy.range.endpointLayer)}) || (borrowed && layer === ${JSON.stringify(policy.range.argumentLayer)}));\n}\n`
  + '/** @param {string} kind @param {string} model @param {boolean} proven @param {boolean} immutable @param {boolean} owner @returns {boolean} */\n'
  + `export function acceptRustStringEmptyReceiver(kind, model, proven, immutable, owner) {\n  return proven && immutable && owner && kind === ${JSON.stringify(policy.string.kind)} && (${policy.string.models.map(model => `model === ${JSON.stringify(model)}`).join(' || ')});\n}\n`;
const javascript = new URL('../src/translation/rust-read-only-query-rules.js', import.meta.url);
const native = new URL('../../rust/src/translation/rust/read_only_query_rules.rs', import.meta.url);
const copy = new URL('../../rust/src/data/rust-read-only-queries.lino', import.meta.url);
const style = new URL('../../parity/self-translation/rust-read-only-query-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('Rust readonly query JavaScript rules are stale');
} else writeFileSync(javascript, source);
if (!process.argv.includes('--javascript-only')) {
  const definitions = emitRust(checkProgram(parseJavaScript(source))).definitions;
  const raw = definitions.join('\n\n');
  if (process.argv.includes('--update-styles')) {
    // Whole function records preserve the exact owning signature. The style
    // renders the same scalar predicates from the same policy in precedence
    // order, avoiding detached global replacements and redundant parentheses.
    const bodies = [Object.entries(policy.owners).map(([query, owners], index) => `${index ? '} else ' : ''}if query == ${JSON.stringify(query)} {\n    ${owners.map(([kind, path]) => `kind == ${JSON.stringify(kind)} && path == ${JSON.stringify(path)}`).join(' || ')}\n`).join('') + '} else {\n    false\n}',
      `proven && !shadowed && kind == ${JSON.stringify(policy.range.kind)} && model == ${JSON.stringify(policy.range.model)} && ((!borrowed && layer == ${JSON.stringify(policy.range.endpointLayer)}) || (borrowed && layer == ${JSON.stringify(policy.range.argumentLayer)}))`,
      `proven && immutable && owner && kind == ${JSON.stringify(policy.string.kind)} && (${policy.string.models.map(model => `model == ${JSON.stringify(model)}`).join(' || ')})`];
    const actions = definitions.map((definition, index) => {
      const header = definition.slice(0, definition.indexOf('{')).replace(/\b(query|kind|path|model|layer): String/gu, '$1: &str');
      const normalized = '/// Admit the corresponding standard Rust readonly source query.\n#[must_use]\n' + header + '{\n' + bodies[index] + '\n}';
      return { op: 'replace', field: 'source', from: definition, to: normalized };
    });
    writeFileSync(style, new DecoratorSet([{ id: 'rust-read-only-query-source-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
  }
  const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const generated = '// Generated from the linked Rust readonly query policy through the checked translator.\n// Do not edit by hand.\n' + formatProductRuleSource(decorated);
  if (process.argv.includes('--check')) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('Rust readonly query native rules or policy are stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
}
