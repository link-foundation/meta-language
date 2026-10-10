import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { tokenize } from '../src/translation/lexer.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const fixtures = JSON.parse(readFileSync(new URL('../../parity/fixtures/translation-lexical-boundaries.json', import.meta.url), 'utf8'));

test('regex bodies are indivisible tokens and retain the honest unsupported diagnostic', () => {
  for (const literal of fixtures.regexLiterals) {
    const source = `function matches(text) { return ${literal}.test(text); }`;
    const { tokens } = tokenize(source, 'JavaScript');
    assert.ok(tokens.some(({ raw }) => raw === literal), literal);
    assert.throws(() => parseJavaScript(source), (error) => error.kind === 'unsupported' && /regular expression/u.test(error.message));
  }
});

test('division and division assignment stay separate from regex literals', () => {
  for (const source of fixtures.divisionPrograms) {
    assert.doesNotThrow(() => checkProgram(parseJavaScript(source)));
    assert.equal(tokenize(source, 'JavaScript').tokens.filter(({ raw }) => raw.startsWith('/') && raw.length > 2).length, 0);
  }
  for (const source of fixtures.malformedRegex) assert.throws(() => tokenize(source, 'JavaScript'), /unterminated regular expression/u);
});

test('NUL template escapes translate while legacy octal escapes are rejected', () => {
  const source = fixtures.templateProgram;
  assert.doesNotThrow(() => emitJavaScript(checkProgram(parseJavaScript(source))));
  assert.throws(() => tokenize('`\\01`', 'JavaScript'), /octal/u);
});

test('many regex literals encode the source only once and keep absolute Unicode offsets', () => {
  const { count, literal: raw } = fixtures.repeatedRegex;
  const source = Array.from({ length: count }, (_, index) => `const r${index} = ${raw};`).join('\n');
  const original = String.prototype.charCodeAt;
  let calls = 0;
  let result;
  try {
    String.prototype.charCodeAt = function (index) { calls += 1; return original.call(this, index); };
    result = tokenize(source, 'JavaScript');
  } finally {
    String.prototype.charCodeAt = original;
  }
  assert.equal(calls, source.length);
  const literals = result.tokens.filter(({ kind }) => kind === 'regex');
  assert.equal(literals.length, count);
  for (const literal of literals) {
    assert.equal(literal.raw, raw);
    assert.equal(source.slice(literal.start, literal.end), literal.raw);
  }
});
