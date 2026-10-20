// Reproduce native syntax transport from complete certified predecessor files.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData } from './project-data.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles } from './portable-source-library-style.mjs';
import { simplifySourceTextRustParentheses } from './source-text-byte-library-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { simplifyCompleteRustExpressionParentheses } from './complete-expression-parenthesis-styles.mjs';
import { tokenize } from '../src/translation/lexer.js';

export const javaScriptMutableArrayBindingStyleId = 'reviewed-javascript-mutable-array-binding-style';
const root = new URL('../../', import.meta.url);
const revision = '897bfc8b2dcc5ec561feffa455ddeb9f95d24335';
const paths = ['rust/src/translation/javascript/loops.rs',
  'rust/src/translation/javascript/array_bindings.rs',
  'rust/src/translation/javascript/declarations.rs',
  'rust/src/translation/javascript/callables.rs',
  'rust/src/translation/javascript/lexical_bindings.rs',
  'rust/tests/unit/translation_javascript_array_bindings.rs'];
const digest = source => createHash('sha256').update(source).digest('hex');

function foldMutableArrayBindingNumberGuard(source) {
  const { tokens, comments } = tokenize(source, 'Rust'), pairs = new Map(), stack = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
    else if ([')', ']', '}'].includes(tokens[at].value)) {
      const open = stack.pop();
      assert.equal(tokens[open]?.value, { ')': '(', ']': '[', '}': '{' }[tokens[at].value]);
      pairs.set(open, at);
    }
  }
  assert.equal(stack.length, 0);
  const declaration = tokens.findIndex(token => token.value === 'fn'), open = declaration + 2, close = pairs.get(open);
  assert.equal(tokens[declaration + 1].value, 'accept_java_script_mutable_array_binding');
  const parameters = tokens.slice(open + 1, close).map(token => token.value).join(' ');
  assert.match(parameters, /^value_kind : & str , item_count : f64 , binding_count : f64 , plain_bindings : bool , has_spread : bool(?: ,)?$/u,
    'the actual primitive count parameters prove the native Number type');
  const body = close + 3;
  assert.equal(tokens[body].value, '{');
  const numbers = tokens.flatMap((token, at) => at > body && token.kind === 'string' && token.value === 'number' ? [at] : []);
  if (!numbers.length) return source;
  assert.equal(numbers.length, 2, 'the actual native Number type guard retains both original literals');
  let start = numbers[0], end = numbers[1];
  assert.equal(end, start + 2);
  assert.equal(tokens[start + 1].value, '==');
  while (tokens[start - 1]?.value === '(' && pairs.get(start - 1) === end + 1) { start -= 1; end += 1; }
  assert.equal(tokens[end + 1].value, '&&', 'the checked type test is a complete leading conjunction operand');
  assert.ok(tokens.slice(body + 1, start).every(token => token.kind === 'punct' && token.value === '('),
    'a type-test fold cannot remove any earlier effect or operand');
  assert.ok(!comments.some(comment => comment.start >= tokens[start].start && comment.end <= tokens[end + 1].end),
    'the complete type-test region has no comment to erase');
  return source.slice(0, tokens[start].start) + source.slice(tokens[end + 1].end);
}

/** Style only the two complete new decisions; all other generated owners stay exact. */
export function readJavaScriptMutableArrayBindingStyles(source) {
  const inventory = readRustLibraryItems(source).items;
  assert.ok(!inventory.some(item => item.kind === 'use' || ['f64', 'bool', 'String', 'Vec', 'Option'].includes(item.name)),
    'the actual generated library retains primitive type ownership');
  const names = ['accept_java_script_mutable_array_binding', 'accept_java_script_mutable_array_binding_owner'];
  return names.flatMap(name => {
    const selected = inventory.filter(item => item.kind === 'fn' && item.name === name);
    assert.equal(selected.length, 1, 'one complete mutable binding decision: ' + name);
    const from = selected[0].source;
    const loans = readPortableSourceParameterStyles(from).source;
    const styled = readPortableSourceLintStyles(loans).source;
    const prepared = name === names[0] ? foldMutableArrayBindingNumberGuard(styled) : styled;
    const to = simplifyCompleteRustExpressionParentheses(simplifySourceTextRustParentheses(prepared));
    assert.doesNotMatch(to, /#\[(?:allow|expect)\(/u, 'the new decision requires no lint suppression');
    return from === to ? [] : [{ op: 'replace', field: 'source', from, to }];
  });
}

/** Record an additive source-derived style; the ordinary frontend generator consumes it. */
export async function recordJavaScriptMutableArrayBindingStyles() {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const file = new URL('parity/self-translation/frontend-rules-decorators.lino', root);
  const previous = readFileSync(file, 'utf8');
  const all = DecoratorSet.fromLino(previous);
  const base = new DecoratorSet(all.decorators.filter(row => row.id !== javaScriptMutableArrayBindingStyleId));
  const emitted = emitRust(checkProgram(parseJavaScript(readFileSync(new URL('js/src/translation/frontend-rules.js', root), 'utf8'))));
  let source = formatProductRuleSource(decorateEmitted('Rust', { source: emitted.text }, base).source);
  for (let round = 0; round < 10; round += 1) {
    const next = formatProductRuleSource(base.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source }).source);
    if (next === source) break;
    source = next;
    assert.ok(round < 9, 'the retained decorators converge');
  }
  const row = { id: javaScriptMutableArrayBindingStyleId, level: 'emitter', order: 3400,
    when: [['format', 'Rust'], ['scope', 'formatted-module']], actions: readJavaScriptMutableArrayBindingStyles(source) };
  const retained = previous.split('\n').filter(line => !line.startsWith('(decorator ' + row.id + ' ')).join('\n');
  writeFileSync(file, retained + (retained.endsWith('\n') ? '\n' : '\n\n') + new DecoratorSet([row]).toLino());
  return row;
}

/** Reconstruct all six actual hosts offline, including every predecessor byte. */
export function generateJavaScriptMutableArrayBindings({ check = false } = {}) {
  const archive = readProjectData(new URL('parity/self-translation/javascript-mutable-array-binding-hosts.lino', root));
  assert.equal(archive.schemaVersion, 1);
  assert.equal(archive.revision, revision);
  assert.equal(archive.decision.source, 'js/src/translation/frontend-rules.js');
  assert.equal(archive.decision.name, 'acceptJavaScriptMutableArrayBinding');
  assert.equal(archive.decision.target, 'accept_java_script_mutable_array_binding');
  assert.equal(archive.ownerDecision.source, archive.decision.source);
  assert.equal(archive.ownerDecision.name, 'acceptJavaScriptMutableArrayBindingOwner');
  assert.equal(archive.ownerDecision.target, 'accept_java_script_mutable_array_binding_owner');
  assert.equal(archive.ownerDecision.sourceSha256, archive.decision.sourceSha256);
  assert.equal(digest(readFileSync(new URL(archive.decision.source, root))), archive.decision.sourceSha256,
    'the current complete shared decision source is independently pinned');
  assert.deepEqual(archive.hosts.map(row => row.path), paths);
  const outputs = archive.hosts.map(row => {
    verifyGitArtifactCertificate(row.certificate, Buffer.from(row.source), { revision, path: row.path });
    let generated = row.source;
    for (const edit of row.edits) {
      assert.ok(Number.isSafeInteger(edit.start) && edit.start >= 0 && edit.start <= generated.length);
      assert.equal(generated.slice(edit.start, edit.start + edit.removed.length), edit.removed);
      generated = generated.slice(0, edit.start) + edit.inserted + generated.slice(edit.start + edit.removed.length);
    }
    assert.equal(digest(generated), row.currentSha256, row.path);
    let inverse = generated;
    for (const edit of [...row.edits].reverse()) {
      assert.equal(inverse.slice(edit.start, edit.start + edit.inserted.length), edit.inserted);
      inverse = inverse.slice(0, edit.start) + edit.removed + inverse.slice(edit.start + edit.inserted.length);
    }
    assert.equal(inverse, row.source, 'the complete original module and context remain recoverable');
    const file = new URL(row.path, root), current = readFileSync(file, 'utf8');
    if (check) assert.equal(current, generated, row.path);
    else assert.ok(current === row.source || current === generated, 'unknown current source ' + row.path);
    return { file, generated };
  });
  if (!check) for (const { file, generated } of outputs) writeFileSync(file, generated);
  return archive;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.length <= 1 && args.every(value => ['--check', '--record-styles'].includes(value)));
  if (args.includes('--record-styles')) await recordJavaScriptMutableArrayBindingStyles();
  else generateJavaScriptMutableArrayBindings({ check: args.includes('--check') });
}
