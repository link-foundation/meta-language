// Raw shared-operation probes use an explicit simultaneous token mask. They do
// not infer LR eligibility or establish complete Markdown grammar support.
import assert from 'node:assert/strict';
import {markdownInlineScanner} from '../../js/scripts/markdown-inline-scanner-family.mjs';
import {parseGrammarLinks} from '../../js/src/grammar-links.js';
import {loadProgram} from '../../js/src/grammar-runtime/load.js';
import {OperationFailed, freezeState, workingState, runStatements} from '../../js/src/grammar-runtime/operations.js';

export const inlineSourceTokens = Object.freeze(['error', 'trigger_error', 'code_span_start', 'code_span_close',
  'emphasis_open_star', 'emphasis_open_underscore', 'emphasis_close_star', 'emphasis_close_underscore',
  'last_token_whitespace', 'last_token_punctuation', 'strikethrough_open', 'strikethrough_close',
  'latex_span_start', 'latex_span_close', 'unclosed_span']);
const source = markdownInlineScanner({name: 'inline', tokens: inlineSourceTokens, eligibility: 'complete-source-symbol-context'});
const program = loadProgram(parseGrammarLinks('(grammar (start source))\n' + source + '(rule source normal (literal x))'));
const operations = program.scanners.get('error').operations;

/** Execute actual shared operations against a supplied original source context. */
export function probeInlineLinksOperations({input, valid, state = [0, 0, 0, 0]}) {
  assert.equal(typeof input, 'string');
  assert.ok(Array.isArray(valid) && new Set(valid).size === valid.length && valid.every(token => inlineSourceTokens.includes(token)));
  assert.ok(Array.isArray(state) && state.length === 4 && state.every(value => Number.isInteger(value) && value >= 0 && value <= 255));
  const eligible = new Set(valid);
  const working = workingState(freezeState(['default'], {}, {inline_flags: state[0], inline_code_width: state[1],
    inline_latex_width: state[2], inline_emphasis_remaining: state[3]}));
  let cursor = 0, mark = null, steps = 0;
  const machine = {state: working, requested: null,
    step() { assert.ok(++steps <= 250_000, 'a bounded inline probe retains the shared operation budget'); },
    expected(item) { assert.equal(item.kind, 'ref'); return eligible.has(item.name); },
    atEnd() { return cursor === input.length; },
    lookahead(item) {
      if (item.kind === 'literal') return input.startsWith(item.value, cursor);
      assert.equal(item.kind, 'charClass');
      const character = cursor < input.length ? String.fromCodePoint(input.codePointAt(cursor)) : null;
      assert.ok(item.items.every(part => part.kind === 'char'));
      const matched = character !== null && item.items.some(part => part.value === character);
      return character !== null && (item.negated ? !matched : matched);
    },
    advance() { if (cursor === input.length) throw new OperationFailed(); cursor += String.fromCodePoint(input.codePointAt(cursor)).length; },
    mark() { mark = cursor; },
  };
  let signal = null;
  try { signal = runStatements(operations, machine); }
  catch (error) { if (!(error instanceof OperationFailed)) throw error; }
  const byteOffset = position => Buffer.byteLength(input.slice(0, position));
  return {accepted: signal !== null, token: signal?.emit ?? null, cursor: byteOffset(cursor),
    mark: mark === null ? null : byteOffset(mark),
    state: [working.variables.inline_flags, working.variables.inline_code_width,
      working.variables.inline_latex_width, working.variables.inline_emphasis_remaining]};
}

/** Retain boundary cases and simultaneous contexts for the independent C probe. */
export function inlineSourceProbeCases() {
  const cases = [{input: 'ordinary', valid: []}, {input: 'ordinary', valid: ['error']}, {input: '', valid: ['trigger_error']},
    {input: '*x', valid: ['trigger_error', 'emphasis_open_star'], state: [255, 2, 3, 4]}];
  for (const [marker, open, close, width] of [['`', 'code_span_start', 'code_span_close', 1], ['$', 'latex_span_start', 'latex_span_close', 2]]) {
    for (const count of [1, 2, 3, 255, 256, 257]) {
      for (const suffix of ['', 'x', 'x' + marker.repeat(count), 'x' + marker.repeat(count + 1), '\n' + marker.repeat(count) + 'x']) {
        cases.push({input: marker.repeat(count) + suffix, valid: [open, close, 'unclosed_span']});
      }
      const state = [173, 0, 0, 11]; state[width] = count % 256;
      cases.push({input: marker.repeat(count) + 'after', valid: [close], state});
    }
    cases.push({input: marker + 'no closing run', valid: ['unclosed_span']});
  }
  for (const [marker, open, close] of [['*', 'emphasis_open_star', 'emphasis_close_star'],
    ['_', 'emphasis_open_underscore', 'emphasis_close_underscore'], ['~', 'strikethrough_open', 'strikethrough_close']]) {
    for (const suffix of ['', ' ', '\t', '\r', '\n', 'x', '!', 'é', '\u00a0', '\v', '\f', '。', '😀', '\0']) {
      for (const preceding of [[], ['last_token_whitespace'], ['last_token_punctuation'], ['last_token_whitespace', 'last_token_punctuation']]) {
        for (const enabled of [[open], [close], [open, close]]) cases.push({input: marker.repeat(3) + suffix, valid: [...enabled, ...preceding]});
      }
    }
    for (const flags of [0, 4, 251, 255]) {
      cases.push({input: marker + 'x', valid: [open, close], state: [flags, 5, 6, 2]});
      cases.push({input: marker + 'x', valid: [close], state: [flags, 5, 6, 2]});
    }
    cases.push({input: marker.repeat(256) + 'x', valid: [open, 'last_token_whitespace']});
  }
  return cases;
}
