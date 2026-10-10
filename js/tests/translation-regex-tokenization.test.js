import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
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

test('regex literals after control parentheses stay indivisible through nested expressions and comments', () => {
  for (const source of fixtures.controlRegexPrograms) {
    const tokens = tokenize(source, 'JavaScript').tokens;
    assert.equal(tokens.filter(({ kind }) => kind === 'regex').length, 1, source);
    assert.ok(tokens.some(({ raw }) => raw.includes('9A')), source);
  }
  for (const source of fixtures.qualifiedDivisionPrograms) {
    assert.equal(tokenize(source, 'JavaScript').tokens.filter(({ kind }) => kind === 'regex').length, 0, source);
  }
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


test('template substitutions ignore braces in opaque lexical tokens', () => {
  for (const { source, expression } of fixtures.templateBoundaries) {
    const tokens = tokenize(`${source}; const following = 1;`, 'JavaScript').tokens;
    const template = tokens[0];
    assert.equal(template.raw, source);
    assert.equal(template.parts[0].expression.source, expression, source);
    assert.equal(template.parts[0].text, 'head ');
    assert.equal(template.parts[1].text, ' tail');
    assert.equal(source.slice(template.parts[0].expression.offset, template.parts[0].expression.offset + expression.length), expression);
    assert.ok(tokens.some(({ value }) => value === 'following'));
  }
  for (const source of fixtures.malformedTemplates) assert.throws(() => tokenize(source, 'JavaScript'), (error) => error.kind === 'syntax', source);
});


test('template boundary corrections preserve emitted program behavior', () => {
  const source = fixtures.templateBoundaryProgram;
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  for (const program of [source, emitted]) {
    assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', program], { encoding: 'utf8' }), fixtures.templateBoundaryOutput);
  }
});

test('template regex substitutions share one source encoding', () => {
  const { count, literal } = fixtures.repeatedRegex;
  const source = Array.from({ length: count }, (_, index) => `const r${index} = \`prefix \${${literal}} suffix\`;`).join('\n');
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
  const templates = result.tokens.filter(({ kind }) => kind === 'template');
  assert.equal(templates.length, count);
  for (const template of templates) assert.equal(template.parts[0].expression.source, literal);
});
