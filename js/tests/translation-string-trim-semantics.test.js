import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/string-trim-semantics.json';
const fixtures = JSON.parse(readFileSync(new URL('../../' + fixtureFile, import.meta.url), 'utf8')).cases;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-STRING-TRIM-SEMANTICS', suffix: 'behavior', fixtureId: 'string-trim-semantics', fixtureFile, assertions, testName,
});

test('Rust trim forms execute in JavaScript with Unicode White_Space boundaries', () => {
  const source = 'pub fn trimmed(s: &str) -> String { s.trim().to_string() }\npub fn start(s: &str) -> String { s.trim_start().to_string() }\npub fn end(s: &str) -> String { s.trim_end().to_string() }\nfn main() {}\n';
  const result = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(result.diagnostic, null);
  assert.doesNotMatch(result.code, /portable-source-envelope/u);
  const functions = runInNewContext(result.code + '\n[trimmed, start, end]');
  for (const fixture of fixtures) assert.deepEqual(Array.from(functions, (fn) => fn(fixture.input)), fixture.rust, fixture.name);
  observe(['rustWhitespacePreserved'], 'Rust trim forms execute in JavaScript with Unicode White_Space boundaries');
});

test('JavaScript trim forms retain ECMAScript boundaries and generate their distinct Rust predicate', () => {
  const source = '/** @param {string} s @returns {string} */ function trimmed(s) { return s.trim(); }\n/** @param {string} s @returns {string} */ function start(s) { return s.trimStart(); }\n/** @param {string} s @returns {string} */ function end(s) { return s.trimEnd(); }\n';
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  const functions = runInNewContext(emitted + '\n[trimmed, start, end]');
  const rust = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.doesNotMatch(rust.code, /portable-source-envelope/u);
  assert.match(rust.code, /c\.is_whitespace\(\) && c != '\\u\{85\}'/u);
  assert.match(rust.code, /c == '\\u\{feff\}'/u);
  for (const fixture of fixtures) assert.deepEqual(Array.from(functions, (fn) => fn(fixture.input)), fixture.javascript, fixture.name);
  observe(['ecmascriptWhitespacePreserved', 'bidirectionalUnicodeTrimFixtures', 'trimTranslationsHaveNoSourceEnvelope'], 'JavaScript trim forms retain ECMAScript boundaries and generate their distinct Rust predicate');
});
