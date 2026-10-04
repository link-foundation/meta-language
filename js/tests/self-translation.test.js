import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { Parser } from 'links-notation';

import {
  SELF_TRANSLATION_LANGUAGES,
  selfTranslate,
  selfTranslationLanguage,
  SelfTranslationError,
} from '../src/index.js';
import { readSelfTranslationCorpus } from '../scripts/generate-self-translation-cases.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const FIXTURE = 'parity/self-translation/cases.lino';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (name) => readFileSync(path.join(root, 'parity/self-translation', name), 'utf8');
const { cases, calls } = await readSelfTranslationCorpus();
const isJavaScript = (language) => language !== 'Rust';

function observe(requirementId, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile: FIXTURE,
    assertions,
    testName,
  });
}

// The items of an expected `.items.lino` file, read as Links Notation.
function expectedItems(file) {
  return new Parser().parse(read(file)).map(({ values }) => {
    const [, start, end, term, status, reason] = values.map(({ id }) => id);
    return { term, start: Number(start), end: Number(end), status, reason: reason ?? null };
  });
}

test('every shared case translates to its expected output and items', () => {
  for (const entry of cases) {
    const translation = selfTranslate(read(entry.source), entry.from, entry.to);
    assert.equal(translation.sourceLanguage, entry.from);
    assert.equal(translation.targetLanguage, entry.to);
    assert.equal(translation.code, read(entry.expected), entry.id);
    assert.deepEqual(translation.items, expectedItems(`${entry.expected}.items.lino`), entry.id);
  }
  observe('I195-SELF-TRANSLATION-SHARED-CORPUS', ['expectedOutputsInLinksNotation', 'checkedInBothRuntimes'], 'every shared case translates to its expected output and items');
  const directions = cases.map(({ from, to }) => `${isJavaScript(from)}:${isJavaScript(to)}`);
  assert.ok(directions.includes('true:false') && directions.includes('false:true'));
  observe('I195-SELF-TRANSLATION-TOOL', ['libraryApiInBothPackages', 'javascriptToRust', 'rustToJavaScript'], 'every shared case translates to its expected output and items');
});

test('an unedited translation translates back to its source byte for byte', () => {
  for (const entry of cases) {
    const source = read(entry.source);
    // A source that is itself an edited translation translates its edits again.
    if (source.startsWith('// meta-language:self-translation:v1 ')) continue;
    assert.equal(selfTranslate(read(entry.expected), entry.to, entry.from).code, source, entry.id);
  }
  // Translated items keep their source as provenance; an edited one is translated again.
  const edited = cases.find(({ id }) => id === 'arithmetic-edited-to-javascript');
  const statuses = new Map(selfTranslate(read(edited.source), edited.from, edited.to).items.map(({ term, status }) => [status, term]));
  assert.equal(statuses.get('translated'), 'function_item');
  assert.equal(statuses.get('restored'), 'line_comment');
  observe('I195-SELF-TRANSLATION-ROUND-TRIP', ['provenanceKeepsCommentsAndNames'], 'an unedited translation translates back to its source byte for byte');
});

test('meta-language\'s own modules round-trip byte for byte', () => {
  let translated = 0;
  for (const [file, language, other] of [
    ['js/src/primitives.js', 'JavaScript', 'Rust'],
    ['js/src/self-translation.js', 'JavaScript', 'Rust'],
    ['rust/src/self_translation.rs', 'Rust', 'JavaScript'],
    ['rust/src/link_flags.rs', 'Rust', 'TypeScript'],
    ['rust/src/binary_format.rs', 'Rust', 'TypeScript'],
  ]) {
    const source = readFileSync(path.join(root, file), 'utf8');
    assert.equal(selfTranslate(source, language, language).code, source, file);
    const there = selfTranslate(source, language, other);
    translated += there.items.filter(({ status }) => status === 'translated').length;
    assert.equal(selfTranslate(there.code, other, language).code, source, file);
  }
  assert.ok(translated > 0, 'some item of the modules is translated');
  observe('I195-SELF-TRANSLATION-ROUND-TRIP', ['sameLanguageByteIdentical'], 'meta-language\'s own modules round-trip byte for byte');
});

// The JavaScript value a corpus argument names.
function value({ type, value: text }) {
  if (type === 'f64') return Number(text);
  if (type === 'str') return text;
  if (type === 'bool') return text === 'true';
  return BigInt(text);
}

test('the JavaScript side of every case computes the shared results', async () => {
  for (const call of calls) {
    const entry = cases.find(({ id }) => id === call.case);
    const module = isJavaScript(entry.from) ? read(entry.source) : read(entry.expected);
    const loaded = await import(`data:text/javascript;base64,${Buffer.from(module).toString('base64')}`);
    assert.equal(String(loaded[call.javascript](...call.arguments.map(value))), call.result, `${call.case} ${call.javascript}`);
  }
  observe('I195-SELF-TRANSLATION-ROUND-TRIP', ['crossLanguageBehaviorPreserved'], 'the JavaScript side of every case computes the shared results');
});

test('the translate command prints the translation and its items', () => {
  const cli = path.join(root, 'js/src/cli.js');
  const entry = cases.find(({ id }) => id === 'arithmetic-to-rust');
  const source = path.join(root, 'parity/self-translation', entry.source);
  assert.equal(execFileSync(process.execPath, [cli, 'translate', '--to', 'rust', source], { encoding: 'utf8' }), read(entry.expected));
  const listed = execFileSync(process.execPath, [cli, 'translate', '--to', 'rs', '--from', 'js', '--items', source], { encoding: 'utf8' });
  assert.equal(listed, expectedItems(`${entry.expected}.items.lino`).map(({ start, end, term, status, reason }) => `${start}..${end} ${term} ${status}${reason ? ` (${reason})` : ''}\n`).join(''));
  observe('I195-SELF-TRANSLATION-TOOL', ['cliInBothPackages'], 'the translate command prints the translation and its items');
});

test('languages are named by name or extension, and others are refused', () => {
  assert.deepEqual(SELF_TRANSLATION_LANGUAGES, ['JavaScript', 'TypeScript', 'Rust']);
  assert.equal(selfTranslationLanguage('rs'), 'Rust');
  assert.equal(selfTranslationLanguage('TS'), 'TypeScript');
  assert.equal(selfTranslationLanguage('mjs'), 'JavaScript');
  assert.equal(selfTranslationLanguage('python'), null);
  assert.throws(() => selfTranslate('x\n', 'python', 'rust'), SelfTranslationError);
});

test('the report measures each translated module against its hand-written Rust', () => {
  const outDir = mkdtempSync(path.join(tmpdir(), 'self-translation-report-'));
  try {
    execFileSync(process.execPath, [path.join(root, 'js/scripts/generate-self-translation-report.mjs'), '--out-dir', outDir, '--modules', 'language-support.js,self-translation.js'], { encoding: 'utf8' });
    const { modules, failures } = JSON.parse(readFileSync(path.join(outDir, 'self-translation-report.json'), 'utf8'));
    assert.deepEqual(failures, []);
    assert.deepEqual(modules.map(({ module, rust }) => [module, rust]), [
      ['js/src/language-support.js', 'rust/src/language_support.rs'],
      ['js/src/self-translation.js', 'rust/src/self_translation.rs'],
    ]);
    for (const row of modules) {
      assert.ok(row.items.translated > 0 && row.handWrittenLines > 0, row.module);
      assert.ok(row.sharedLines <= row.codeLines && row.identical <= row.matched && row.matched <= row.functions, row.module);
    }
    const report = readFileSync(path.join(outDir, 'self-translation-report.md'), 'utf8');
    for (const { module } of modules) assert.match(report, new RegExp(`\\| ${module} \\| `, 'u'));
  } finally {
    rmSync(outDir, { recursive: true, force: true });
  }
  observe('I195-SELF-TRANSLATION-SHARED-CORPUS', ['differencePerModulePublished'], 'the report measures each translated module against its hand-written Rust');
});
