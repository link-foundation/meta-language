// Replay authentic CI suggestions, followed by separately reviewed ownership
// and control-flow corrections. Never edit generated Rust or diagnostic data.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../js/src/decorators.js';
import { parseProgrammingLanguage } from '../js/src/programming-language-parser.js';
import { readProjectData, writeProjectData } from '../js/scripts/project-data.mjs';

const feedbackPath = 'parity/self-translation/ci-38300345224-frontend-rules-feedback.lino';
const feedback = readProjectData(feedbackPath);
assert.equal(feedback.runId, '38300345224');
const stylesPath = 'parity/self-translation/frontend-rules-decorators.lino';
const styles = readFileSync(stylesPath, 'utf8');
const id = 'reviewed-ci-38300345224-frontend-styles';
assert.equal(DecoratorSet.fromLino(styles).decorators.some(item => item.id === id), false, 'reviewed actions are appended once');
const actions = [];
const hash = value => createHash('sha256').update(value).digest('hex');

function definition(source, name) {
  const bytes = Buffer.from(source);
  const tree = parseProgrammingLanguage(source, 'Rust').tree;
  const node = tree.children.find(item => item.term === 'function_item'
    && item.children.some(child => child.field === 'name'
      && bytes.subarray(child.span.byteRange.start, child.span.byteRange.end).toString() === name));
  assert.ok(node, name);
  return bytes.subarray(node.span.byteRange.start, node.span.byteRange.end).toString();
}

function flattenSingleElseIf(source) {
  while (true) {
    const bytes = Buffer.from(source);
    const pending = [parseProgrammingLanguage(source, 'Rust').tree];
    let replacement = null;
    while (pending.length && !replacement) {
      const node = pending.pop();
      pending.push(...node.children);
      if (node.term !== 'else_clause') continue;
      const block = node.children.find(child => child.term === 'block');
      const expressions = block?.children.filter(child => !['{', '}'].includes(child.term) && !child.flags.isExtra);
      if (expressions?.length !== 1) continue;
      let expression = expressions[0];
      if (expression.term === 'expression_statement') {
        const items = expression.children.filter(child => !child.flags.isExtra);
        if (items.length !== 1) continue;
        expression = items[0];
      }
      if (expression.term !== 'if_expression') continue;
      const inner = expression.span.byteRange;
      replacement = { ...node.span.byteRange, text: `else ${bytes.subarray(inner.start, inner.end).toString()}` };
    }
    if (!replacement) return source;
    source = Buffer.concat([bytes.subarray(0, replacement.start), Buffer.from(replacement.text), bytes.subarray(replacement.end)]).toString();
  }
}

const strings = readFileSync('rust/src/translation/frontend_rules/strings.rs', 'utf8');
for (const name of ['read_lexical_cache_expression_form', 'read_unit_alternative_expression_form']) {
  const from = definition(strings, name);
  assert.ok(from.includes('(kind: String)'));
  const to = flattenSingleElseIf(from).replace('(kind: String)', '(kind: &str)');
  assert.notEqual(to, from);
  actions.push({ op: 'replace', field: 'source', from, to });
}
const continuations = readFileSync('rust/src/translation/frontend_rules/continuations.rs', 'utf8');
const from = definition(continuations, 'ml_accept_module_binding_scope_loop16');
const needle = '} else if ((!(match (term.clone(), String::from("function_declaration"))';
assert.equal(from.split(needle).length, 2);
const to = from.replace(needle, '}\n                if ((!(match (term.clone(), String::from("function_declaration"))');
actions.push({ op: 'replace', field: 'source', from, to });
for (const action of actions) {
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: action.to, encoding: 'utf8' });
  assert.equal(formatted.status, 0, formatted.stderr);
  action.to = formatted.stdout.trimEnd();
}
writeFileSync(stylesPath, styles.trimEnd() + '\n\n' + new DecoratorSet([{
  id, level: 'emitter', order: 1001, when: [['format', 'Rust'], ['scope', 'formatted-module']], actions,
}]).toLino());
writeProjectData('parity/self-translation/ci-38300345224-frontend-rule-style-correction.lino', {
  schemaVersion: 1, feedback: feedbackPath, feedbackSha256: hash(readFileSync(feedbackPath)),
  runId: feedback.runId, compilerHead: feedback.head,
  machineTransactions: 4, deferredOverlappingMachineSuggestions: 3,
  reviewedActions: actions.map(action => ({ fromSha256: hash(action.from), toSha256: hash(action.to) })),
  review: 'Borrow strings compared without consumption; flatten single else-if expressions; remove an else after an unconditional loop continuation. Unspecified applicability remains unspecified in the authentic artifact.',
});
console.log(`recorded ${actions.length} reviewed style actions separately from authentic compiler applicability`);
