import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { runStages } from '../scripts/translation-stage-lib.mjs';
import { normalizeMigrationTranslationStages } from '../scripts/validate-structured-data-migration.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { parseRust } from '../src/translation/rust.js';
import cases from './fixtures/translation-cases/frontend-rust.mjs';

const name = 'frontend-rust/frontend-rust-stringEscapes';
const snapshot = readProjectData(new URL('../../parity/fixtures/translation-stages.lino', import.meta.url));
const entry = snapshot.programs.find(program => program.name === name);
const [extension, source] = cases['frontend-rust-stringEscapes'];
const diagnostic = { kind: 'syntax', message: 'unsupported string escape \\{ at 84..86' };

test('the original Rust string escape source and descriptor remain unchanged', () => {
  assert.equal(createHash('sha256').update(source).digest('hex'),
    'a0242f076bb95ee97abc8be1bfde6507df9455a926753f301eea852275f025e7');
  assert.deepEqual(entry, { name, extension, source, parse: { error: diagnostic } });
  assert.equal(source.slice(84, 86), '\\{');
});

test('the invalid cooked escape fails lexically before checking or emission', () => {
  // Rust's STRING_LITERAL permits QUOTE_ESCAPE, ASCII_ESCAPE,
  // UNICODE_ESCAPE and STRING_CONTINUE; none admits backslash + opening brace.
  // https://doc.rust-lang.org/reference/tokens.html#string-literals
  for (const operation of [() => tokenize(source, 'Rust'), () => parseRust(source)]) {
    assert.throws(operation, error => {
      assert.equal(error.kind, diagnostic.kind);
      assert.equal(error.message, diagnostic.message);
      assert.deepEqual(error.span, { start: 84, end: 86 });
      return true;
    });
  }
  assert.deepEqual(runStages(extension, source), { parse: { error: diagnostic } });
});

test('valid escapes and format braces keep their separate parsing behavior', () => {
  assert.doesNotThrow(() => parseRust(cases['frontend-rust-stringEscapesSimple'][1]));
  assert.doesNotThrow(() => parseRust(source.replace('brace\\{', 'brace{{')));
  // Removing only the invalid backslash reaches the existing format check;
  // this does not make an unmatched format brace acceptable.
  assert.throws(() => parseRust(source.replace('brace\\{', 'brace{')), error =>
    error.kind === 'syntax' && error.message === 'unclosed { in format string at 23..86');
});

test('a selected parse refresh cannot alter descriptors, neighbors or later stages', () => {
  const index = snapshot.programs.findIndex(program => program.name === name);
  assert.ok(index > 0 && index < snapshot.programs.length - 1);
  const current = { generator: snapshot.generator, programs: snapshot.programs.slice(index - 1, index + 2) };
  const historical = structuredClone(current);
  historical.programs[1].parse.error.message = 'unclosed { in format string at 23..87';
  const refreshes = [{ program: name, stages: ['parse'] }];
  assert.deepEqual(normalizeMigrationTranslationStages(historical, current, refreshes), historical);

  for (const corrupt of [
    value => { value.programs[1].source += ' '; },
    value => { value.programs[1].extension = 'mjs'; },
    value => { value.programs[1].check = { error: diagnostic }; },
    value => { value.programs[0].parse = { error: diagnostic }; },
    value => { value.programs.reverse(); },
    value => { value.programs.splice(0, 1); },
    value => { value.programs.push(structuredClone(value.programs[0])); },
  ]) {
    const counterfeit = structuredClone(current);
    corrupt(counterfeit);
    assert.throws(() => normalizeMigrationTranslationStages(historical, counterfeit, refreshes),
      /protected translation source and stage fields|cannot duplicate a program/u);
  }
});
