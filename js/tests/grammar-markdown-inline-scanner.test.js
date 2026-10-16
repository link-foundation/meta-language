import assert from 'node:assert/strict';
import test from 'node:test';
import {spawnSync} from 'node:child_process';
import {markdownInlineScanner} from '../scripts/markdown-inline-scanner-family.mjs';
import {inlineSourceTokens, probeInlineLinksOperations, inlineSourceProbeCases} from '../../experiments/markdown-inline-source-oracle/probe.mjs';

const probe = (input, valid, state) => probeInlineLinksOperations({input, valid, ...(state ? {state} : {})});

test('inline source scanner requires every original token and an explicit complete eligibility context', () => {
  assert.throws(() => markdownInlineScanner({name: 'inline', tokens: inlineSourceTokens}), /complete source-symbol context/u);
  for (const tokens of [inlineSourceTokens.slice(1), [...inlineSourceTokens, 'extra'], inlineSourceTokens.map(() => 'same')]) {
    assert.throws(() => markdownInlineScanner({name: 'inline', tokens, eligibility: 'complete-source-symbol-context'}), /fifteen distinct/u);
  }
  assert.deepEqual(probe('', ['trigger_error'], [255, 7, 8, 9]),
    {accepted: true, token: 'error', cursor: 0, mark: null, state: [255, 7, 8, 9]});
  assert.equal(probe('ordinary', []).accepted, false);
});

test('inline code and mathematics retain marked runs future matching and independent remembered widths', () => {
  assert.deepEqual(probe('``a`b``tail', ['code_span_start']),
    {accepted: true, token: 'code_span_start', cursor: 7, mark: 2, state: [0, 2, 0, 0]});
  assert.deepEqual(probe('``after', ['code_span_start', 'code_span_close'], [173, 2, 3, 4]),
    {accepted: true, token: 'code_span_close', cursor: 2, mark: 2, state: [173, 0, 3, 4]});
  assert.deepEqual(probe('$x$', ['latex_span_start']),
    {accepted: true, token: 'latex_span_start', cursor: 3, mark: 1, state: [0, 0, 1, 0]});
  assert.equal(probe('`unclosed', ['unclosed_span']).accepted, false);
  assert.equal(probe('`unclosed', ['code_span_start', 'unclosed_span']).token, 'unclosed_span');
  const wrapped = probe('`'.repeat(256), ['code_span_start']);
  assert.equal(wrapped.token, 'code_span_start'); assert.equal(wrapped.state[1], 0); assert.equal(wrapped.mark, 256);
});

test('inline emphasis retains close precedence shared continuation state and an ineligible opening source case', () => {
  assert.equal(probe('*x', ['emphasis_open_star', 'emphasis_close_star']).token, 'emphasis_close_star');
  assert.equal(probe('*x', ['emphasis_close_star', 'last_token_whitespace']).token, 'emphasis_open_star');
  assert.deepEqual(probe('~x', ['strikethrough_open', 'strikethrough_close'], [255, 2, 3, 2]),
    {accepted: true, token: 'strikethrough_open', cursor: 1, mark: null, state: [251, 2, 3, 1]});
  assert.deepEqual(probe('_x', ['emphasis_close_underscore'], [255, 2, 3, 2]),
    {accepted: true, token: 'emphasis_close_underscore', cursor: 1, mark: null, state: [255, 2, 3, 1]});
  const wrapped = probe('*'.repeat(256) + 'x', ['emphasis_open_star', 'last_token_whitespace']);
  assert.equal(wrapped.token, 'emphasis_open_star'); assert.equal(wrapped.state[3], 255); assert.equal(wrapped.mark, 1);
});

test('inline flanking preserves the exact source whitespace and punctuation classes without preceding-text inference', () => {
  for (const marker of ['*', '_', '~']) {
    const open = marker === '*' ? 'emphasis_open_star' : marker === '_' ? 'emphasis_open_underscore' : 'strikethrough_open';
    for (const suffix of [' ', '\t', '\r', '\n', '']) assert.equal(probe(marker + suffix, [open, 'last_token_whitespace']).accepted, false);
    for (const suffix of ['\v', '\f', '\u00a0', '。', 'é']) assert.equal(probe(marker + suffix, [open]).token, open);
    assert.equal(probe(marker + '!', [open]).accepted, false);
    assert.equal(probe(marker + '!', [open, 'last_token_punctuation']).token, open);
  }
});

test('inline source probe inventory exercises every source token with bounded actual shared operations', () => {
  const cases = inlineSourceProbeCases();
  assert.ok(cases.length > 500);
  const used = new Set(), emitted = new Set();
  for (const fixture of cases) {
    for (const token of fixture.valid) used.add(token);
    const result = probeInlineLinksOperations(fixture);
    if (result.accepted) emitted.add(result.token);
    assert.ok(result.state.every(value => Number.isInteger(value) && value >= 0 && value <= 255));
    assert.ok(result.cursor >= (result.mark ?? 0));
    assert.ok(result.cursor <= Buffer.byteLength(fixture.input));
  }
  assert.deepEqual([...used].sort(), [...inlineSourceTokens].sort());
  assert.deepEqual([...emitted].sort(), inlineSourceTokens.filter(token => !['trigger_error', 'last_token_whitespace', 'last_token_punctuation'].includes(token)).sort());
});

test('inline native source comparisons refuse local compilation before reading or extracting any archive', () => {
  const result = spawnSync(process.execPath, [new URL('../../experiments/markdown-inline-source-oracle/check.mjs', import.meta.url).pathname,
    '--archive', '/missing-inline-control.crate', '--out', '/missing-inline-control.lino'], {
    encoding: 'utf8', env: {...process.env, CI: 'false', GITHUB_ACTIONS: 'false'},
  });
  assert.equal(result.status, 1);
  assert.match(result.stderr, /native inline scanner compilation runs only in GitHub CI/u);
  assert.doesNotMatch(result.stderr, /ENOENT|Command failed/u);
});
