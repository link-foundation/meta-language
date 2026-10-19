// Formatting of complete checked expressions, with their interior tokens intact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {DecoratorSet} from '../src/decorators.js';
import {tokenize} from '../src/translation/lexer.js';

export const completeExpressionParenthesisStylePrefix = 'reviewed-b49-complete-expression-parentheses-';

/** Remove only a complete condition or a block's sole grouped value. */
export function simplifyCompleteRustExpressionParentheses(source) {
  const {tokens} = tokenize(source, 'Rust');
  const matching = new Map(), openings = [];
  const punct = (index, value) => tokens[index]?.kind === 'punct' && tokens[index].value === value;
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    const value = tokens[index].value;
    if (['(', '[', '{'].includes(value)) openings.push(index);
    else if ([')', ']', '}'].includes(value)) {
      const open = openings.pop();
      assert.notEqual(open, undefined, 'a complete-expression style needs an opening delimiter');
      assert.equal(tokens[open].value, {')': '(', ']': '[', '}': '{'}[value], 'complete-expression delimiters keep their kinds');
      matching.set(open, index);
    }
  }
  assert.equal(openings.length, 0, 'a complete-expression style needs every closing delimiter');
  // Macro/attribute token trees have their own grammar. Leave their contents
  // opaque even when an identifier happens to spell an ordinary Rust keyword.
  const opaque = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    const macro = tokens[index].kind === 'macro' || punct(index, '!') && tokens[index - 1]?.kind === 'identifier';
    const attribute = punct(index, '#');
    const open = index + (attribute && punct(index + 1, '!') ? 2 : 1);
    if (!(macro || attribute) || !matching.has(open)) continue;
    for (let inner = open; inner <= matching.get(open); inner += 1) opaque.add(inner);
  }
  const removed = new Set(), separated = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    const condition = tokens[index].kind === 'identifier' && tokens[index].value === 'if';
    if (opaque.has(index) || !(condition || punct(index, '{')) || !punct(index + 1, '(')) continue;
    let open = index + 1, close = matching.get(open);
    if (close === undefined || !punct(close + 1, condition ? '{' : '}')) continue;
    while (punct(open, '(') && matching.get(open) === close) {
      // Unit and tuple syntax is a value, not a redundant grouping pair.
      if (close === open + 1) break;
      let tuple = false, braced = false;
      for (let inner = open + 1; inner < close; inner += 1) {
        if (punct(inner, ',')) tuple = true;
        if (punct(inner, '{')) braced = true;
        if (matching.has(inner)) {
          if (condition) for (let nested = inner; nested <= matching.get(inner); nested += 1) {
            if (punct(nested, '{')) braced = true;
          }
          inner = matching.get(inner);
        }
      }
      // Struct literals in conditions require their grouping. Conservatively
      // retain any braced condition instead of interpreting its inner grammar.
      if (tuple || condition && braced) break;
      removed.add(open); removed.add(close);
      if (condition && open === index + 1 && tokens[index].end === tokens[open].start) separated.add(open);
      open += 1; close -= 1;
    }
  }
  for (const index of [...removed].sort((left, right) => right - left)) {
    source = source.slice(0, tokens[index].start) + (separated.has(index) ? ' ' : '') + source.slice(tokens[index].end);
  }
  return source;
}

/** Replace one generator-owned base record; retain every other original byte. */
export function replaceRecordedLibraryBaseStyle(document, definition) {
  const replacement = new DecoratorSet([definition]).toLino();
  const existing = DecoratorSet.fromLino(document).decorators.filter(entry => entry.id === definition.id);
  if (!existing.length) return document + (document && !document.endsWith('\n') ? '\n' : '') + replacement;
  const prefix = '(decorator ' + definition.id + ' ';
  const lines = document.split('\n');
  const positions = lines.flatMap((line, index) => line.startsWith(prefix) ? [index] : []);
  assert.equal(positions.length, 1, 'a generator base style retains one complete physical record');
  lines[positions[0]] = replacement.slice(0, -1);
  return lines.join('\n');
}

/** Replay recorded styles, or append one exact new reviewed formatted module.
 * Freshness refuses a missing/stale correction even if an older replace was
 * silently inapplicable; ordinary regeneration cannot reintroduce the lint. */
export function applyCompleteExpressionParenthesisStyles(document, source, {update = false, format = value => value} = {}) {
  const styles = DecoratorSet.fromLino(document);
  const scoped = new DecoratorSet(styles.decorators.filter(({when}) =>
    when.some(([field, value]) => field === 'scope' && value === 'formatted-module')));
  for (let round = 0; round < 10; round += 1) {
    const next = format(scoped.decorate('emitter', {format: 'Rust', scope: 'formatted-module', source}).source);
    if (next === source) break;
    source = next;
    assert.ok(round < 9, 'complete-expression module styles converge');
  }
  const corrected = format(simplifyCompleteRustExpressionParentheses(source));
  if (corrected === source) return {document, source};
  assert.ok(update, 'complete-expression library styles are stale; run --update-complete-expression-styles');
  const id = completeExpressionParenthesisStylePrefix + createHash('sha256').update(source).digest('hex').slice(0, 16);
  assert.ok(!styles.decorators.some(entry => entry.id === id), 'a recorded complete-expression style must remain applicable');
  const addition = new DecoratorSet([{id, level: 'emitter', order: 4000,
    when: [['format', 'Rust'], ['scope', 'formatted-module']],
    actions: [{op: 'replace', field: 'source', from: source, to: corrected}],
  }]).toLino();
  const updated = document + (document && !document.endsWith('\n') ? '\n' : '') + addition;
  const replayed = applyCompleteExpressionParenthesisStyles(updated, source, {format});
  assert.equal(replayed.source, corrected, 'the recorded complete-expression correction replays exactly');
  return {document: updated, source: corrected};
}
