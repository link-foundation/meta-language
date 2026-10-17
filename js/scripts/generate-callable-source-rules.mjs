// Translate complete closed callable algorithms; styles relocate exact
// generated declarations and immutable loans, preserving their operations.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { CALLABLE_SOURCE_POLICY } from '../src/translation/callable-source-policy.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { readCallableSourceModule } from './callable-source-module.mjs';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles, partitionPortableRustLibrary } from './portable-source-library-style.mjs';

const check = process.argv.includes('--check');
const policyFile = new URL('../src/translation/data/callable-source-rules.lino', import.meta.url);
if (check) checkProjectData(policyFile, CALLABLE_SOURCE_POLICY);
else writeProjectData(policyFile, CALLABLE_SOURCE_POLICY);

const input = new URL('../src/translation/callable-source-rules.js', import.meta.url);
const before = readFileSync(input, 'utf8');
const quote = value => "'" + value + "'";
const decisions = '// Linked policy decisions begin.\n'
  + '/** @param {string} kind @returns {string} */\n'
  + 'export function readCallableStatementRole(kind) {\n'
  + CALLABLE_SOURCE_POLICY.statementRoles.map(({ kinds, role }) => '  if (' + kinds.map(kind => 'kind === ' + quote(kind)).join(' || ') + ') return ' + quote(role) + ';\n').join('')
  + "  return '';\n}\n// Linked policy decisions end.";
const whitespace = '// Linked callable type whitespace decisions begin.\n'
  + '/** @param {number} unit @returns {boolean} */\n'
  + 'export function acceptCallableTypeWhitespace(unit) {\n  return '
  + CALLABLE_SOURCE_POLICY.typeWhitespaceRanges.map(([start, end]) => start === end ? 'unit === ' + start : 'unit >= ' + start + ' && unit <= ' + end).join(' || ')
  + ';\n}\n// Linked callable type whitespace decisions end.';
const source = before.replace(/\/\/ Linked policy decisions begin\.[\s\S]*?\/\/ Linked policy decisions end\./u, decisions)
  .replace(/\/\/ Linked callable type whitespace decisions begin\.[\s\S]*?\/\/ Linked callable type whitespace decisions end\./u, whitespace);
assert.notEqual(source.indexOf(decisions), -1, 'the complete callable module owns one linked decision region');
if (check) assert.equal(before, source, 'callable JavaScript policy decisions are fresh');
else if (before !== source) writeFileSync(input, source);

if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const translated = emitRust(checkProgram(parseJavaScript(readCallableSourceModule(source), { referenceSemantics: false })), { arrayConsumption: true });
  const moduleHeader = translated.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(moduleHeader, 'the checked translator retains its existing generated module header');
  const raw = [moduleHeader, ...translated.preludes, ...translated.definitions].join('\n\n');
  const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
  const actions = [...declarations].filter(name => raw.includes('crate::' + name)).map(name => ({ op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name }));
  const relocated = new DecoratorSet([{ id: 'callable-source-relocation', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).decorate('emitter', {format: 'Rust', source: raw}).source;
  const loans = readPortableSourceParameterStyles(relocated);
  actions.push(...loans.actions);
  const reviewed = readPortableSourceLintStyles(loans.source);
  actions.push(...reviewed.actions);
  const styles = new DecoratorSet([{ id: 'callable-source-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
  const styleFile = new URL('../../parity/self-translation/callable-source-rules-decorators.lino', import.meta.url);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'callable library styles contain only actual declaration relocation');
  else writeFileSync(styleFile, styles.toLino());
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.equal(decorated, reviewed.source, 'recorded callable styles reproduce the complete reviewed source');
  assert.ok(!/\bcrate::/u.test(decorated), 'the callable library has no unresolved crate-root reference');
  const notice = '// Generated from complete callable-source-rules.js through the checked translator.\n// Do not edit by hand.\n';
  const generated = partitionPortableRustLibrary(formatProductRuleSource(decorated), {notice, directory: 'callable_source_rules'});
  const output = new URL('../../rust/src/translation/callable_source_rules.rs', import.meta.url);
  const policyCopy = new URL('../../rust/src/data/callable-source-rules.lino', import.meta.url);
  if (check) {
    assert.ok(readFileSync(policyCopy).equals(readFileSync(policyFile)), 'both runtimes share exact callable policy bytes');
  } else {
    writeFileSync(policyCopy, readFileSync(policyFile));
    mkdirSync(new URL('../../rust/src/translation/callable_source_rules/', import.meta.url), {recursive: true});
  }
  for (const part of generated) {
    const file = part.path === null ? output : new URL('../../rust/src/translation/' + part.path, import.meta.url);
    if (check) assert.equal(readFileSync(file, 'utf8'), part.source, 'complete native callable algorithms are fresh');
    else writeFileSync(file, part.source);
  }
}
console.log('Callable sources: lexical ownership, captures, type constraints, constructors and four-target rendering');
