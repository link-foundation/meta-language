import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { Parser } from 'links-notation';

import {
  SELF_TRANSLATION_LANGUAGES,
  selfTranslate,
  selfTranslationLanguage,
  selfTranslationSignatures,
  SelfTranslationError,
  translateProgram,
} from '../src/index.js';
import { caseDecorators, readSelfTranslationCorpus } from '../scripts/generate-self-translation-cases.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';
import { compileRustTranslation } from './support/self-translation-rust.js';

const FIXTURE = 'parity/self-translation/cases.lino';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (name) => readFileSync(path.join(root, 'parity/self-translation', name), 'utf8');
const { cases, calls, handWritten } = await readSelfTranslationCorpus();
const decoratorsOf = new Map(await Promise.all(cases.map(async (entry) => [entry.id, await caseDecorators(entry)])));
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
    console.log(`checking self-translation fixture ${entry.id}`);
    const translation = selfTranslate(read(entry.source), entry.from, entry.to, { decorators: decoratorsOf.get(entry.id) });
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
    console.log(`checking self-translation round trip ${file}`);
    const source = readFileSync(path.join(root, file), 'utf8');
    assert.equal(selfTranslate(source, language, language).code, source, file);
    console.log(`checked same-language source graph ${file}`);
    const there = selfTranslate(source, language, other);
    console.log(`checked cross-language definitions ${file}`);
    translated += there.items.filter(({ status }) => status === 'translated').length;
    assert.equal(selfTranslate(there.code, other, language).code, source, file);
  }
  assert.ok(translated > 0, 'some item of the modules is translated');
  observe('I195-SELF-TRANSLATION-ROUND-TRIP', ['sameLanguageByteIdentical'], 'meta-language\'s own modules round-trip byte for byte');
});

// The top-level `fn NAME` items of Rust `text`, by name, up to whitespace.
function rustFunctions(text) {
  const found = new Map();
  for (const match of text.matchAll(/^(?:pub )?fn ([a-z_0-9]+)[^\n]*\{\n(?:[^\n]*\n)*?\}$/gmu)) found.set(match[1], match[0].replace(/\s+/gu, ' '));
  return found;
}

test('decorators bring a translation to the hand-written Rust and still restore its source', () => {
  assert.ok(handWritten.length > 0);
  for (const { case: id, rust, functions } of handWritten) {
    const entry = cases.find((candidate) => candidate.id === id);
    const decorators = decoratorsOf.get(id);
    assert.ok(decorators.size > 0, id);
    const expected = rustFunctions(read(rust));
    const decorated = rustFunctions(selfTranslate(read(entry.source), entry.from, entry.to, { decorators }).code);
    const generic = rustFunctions(selfTranslate(read(entry.source), entry.from, entry.to).code);
    for (const name of functions) {
      assert.equal(decorated.get(name), expected.get(name), `${id} ${name} with its decorators`);
      assert.notEqual(generic.get(name), expected.get(name), `${id} ${name} differs without them`);
    }
    // Removing every decorator gives exactly the generic translation.
    const removed = decorators.ids().reduce((set, decoratorId) => set.remove(decoratorId), decorators);
    assert.equal(selfTranslate(read(entry.source), entry.from, entry.to, { decorators: removed }).code, selfTranslate(read(entry.source), entry.from, entry.to).code);
    // The decorated code is the provenance, so the translation back restores the source.
    assert.equal(selfTranslate(read(entry.expected), entry.to, entry.from).code, read(entry.source), id);
  }
  observe('I195-SELF-TRANSLATION-SHARED-CORPUS', ['decoratorsMatchHandWritten'], 'decorators bring a translation to the hand-written Rust and still restore its source');
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

test('the JavaScript acceptance stage compiles every generated Rust case with clippy', () => {
  // The ordinary JavaScript test job intentionally needs only Node. The issue-195
  // evidence stage installs the pinned Rust toolchain and sets this path; only
  // that executed acceptance stage is allowed to record generatedRustCompiles.
  // SELF_TRANSLATION_CHECK_RUST also executes the compilers for local checks,
  // without an observation file or any acceptance evidence.
  if (!process.env.ISSUE_195_OBSERVATION_FILE && !process.env.SELF_TRANSLATION_CHECK_RUST) return;
  const directory = mkdtempSync(path.join(tmpdir(), 'self-translation-rust-'));
  try {
    const rustCases = cases.filter(({ to }) => to === 'Rust');
    assert.ok(rustCases.length > 0, 'the shared corpus includes JavaScript/TypeScript -> Rust cases');
    const translate = (entry) => selfTranslate(read(entry.source), entry.from, entry.to, { decorators: decoratorsOf.get(entry.id) }).code;
    // The case that translates the sibling source a module path names.
    const moduleCase = (name) => rustCases.find((entry) => entry.from !== 'Rust' && path.basename(entry.source).replace(/\.[^.]+$/u, '') === name);
    for (const entry of rustCases) {
      const code = translate(entry);
      // A relative import becomes `use crate::<module>::…`, so a case that
      // imports compiles as the root of a crate holding the translations of
      // the sibling sources it names, each as a module file beside it.
      const imported = [...new Set([...code.matchAll(/^use crate::([A-Za-z_][A-Za-z0-9_]*)::/gmu)].map((match) => match[1]))];
      const crate = path.join(directory, entry.id);
      mkdirSync(crate);
      for (const name of imported) {
        const sibling = moduleCase(name);
        assert.ok(sibling, `${entry.id} imports crate module ${name}, which a corpus case translates`);
        writeFileSync(path.join(crate, `${name}.rs`), translate(sibling));
      }
      const source = path.join(crate, 'lib.rs');
      const metadata = path.join(directory, `${entry.id}.rmeta`);
      writeFileSync(source, `${code}${imported.map((name) => `\nmod ${name};`).join('')}\n`);
      // Rust -> Rust restores an original fixture byte for byte. Apply the
      // strict generated-code lint policy to JavaScript/TypeScript -> Rust.
      compileRustTranslation(entry.from, source, metadata);
      assert.ok(readFileSync(metadata).length > 0, `${entry.id} emitted Rust metadata`);
    }
    observe('I195-SELF-TRANSLATION-TOOL', ['generatedRustCompiles'], 'the JavaScript acceptance stage compiles every generated Rust case with clippy');
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
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

// The Rust a translation emits, without its provenance lines.
const emittedRust = (translation) => translation.code.split('\n').filter((line) => !line.startsWith('// ')).join('\n');
const statusOf = (translation, source, needle) => translation.items.find(({ start, end }) => source.slice(start, end).includes(needle))?.status;

test('an item calls and reads the other top-level items of its module', () => {
  const source = [
    'const LIMIT = 3;',
    '',
    '/** @param {number} x @returns {number} */',
    'function square(x) {',
    '  return x * x;',
    '}',
    '',
    '/** @param {number} x @returns {number} */',
    'export function cappedSquare(x) {',
    '  return Math.min(square(x), LIMIT);',
    '}',
    '',
  ].join('\n');
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  assert.deepEqual(translation.items.map(({ status }) => status), ['translated', 'translated', 'translated', 'translated', 'translated']);
  const rust = emittedRust(translation);
  assert.match(rust, /pub const LIMIT: f64 = 3f64;/u);
  assert.match(rust, /pub fn square\(x: f64\) -> f64 \{/u);
  assert.match(rust, /pub fn capped_square\(x: f64\) -> f64 \{\n {4}crate::ml_math::min\(square\(x\), LIMIT\)\n\}/u);
  // The translation back restores the source.
  assert.equal(selfTranslate(translation.code, 'Rust', 'JavaScript').code, source);
});

test('a translated item whose definitions carry attributes translates back to its source', () => {
  const source = "/** @param {string[]} parts @returns {boolean} */\nexport function balanced(parts) {\n  let depth = 0;\n  for (const part of parts) {\n    if (part === '(') depth += 1;\n    if (part === ')') depth -= 1;\n    if (depth < 0) return false;\n  }\n  return depth === 0;\n}\n";
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  // The loop's result is a data type, whose `#[derive(…)]` is an item of its own in Rust.
  assert.match(translation.code, /items=3 /u);
  assert.match(translation.code, /#\[derive\(/u);
  assert.equal(selfTranslate(translation.code, 'Rust', 'JavaScript').code, source);
});

test('a sibling that does not translate leaves its callers carried', () => {
  const source = [
    '/** @param {bigint} n @returns {boolean} */',
    'function isEven(n) {',
    '  return n === 0n ? true : isOdd(n - 1n);',
    '}',
    '',
    '/** @param {bigint} n @returns {boolean} */',
    'function isOdd(n) {',
    '  return n === 0n ? false : isEven(n - 1n);',
    '}',
    '',
    '/** @param {number} x @returns {number} */',
    'function opaque(x) {',
    '  return [x].map((y) => y)[0];',
    '}',
    '',
    '/** @param {number} x @returns {number} */',
    'function caller(x) {',
    '  return opaque(x) + 1;',
    '}',
    '',
  ].join('\n');
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  // Mutually recursive siblings are a cycle, which stays unbound.
  assert.equal(statusOf(translation, source, 'function isEven'), 'carried');
  assert.equal(statusOf(translation, source, 'function isOdd'), 'carried');
  assert.equal(statusOf(translation, source, 'function opaque'), 'carried');
  assert.equal(statusOf(translation, source, 'function caller'), 'carried');
  assert.equal(translation.items.find(({ start }) => source.slice(start).startsWith('function caller')).reason, 'type');
});

test('a relative import of items of the crate translates as a use declaration', () => {
  const math = [
    '/** @param {number} x @returns {number} */',
    'export function double(x) {',
    '  return x * 2;',
    '}',
    '',
    '/** @param {number} x @returns {number} */',
    'function hidden(x) {',
    '  return x;',
    '}',
    '',
    'export const UNIT = \'m\';',
    '',
  ].join('\n');
  const signatures = selfTranslationSignatures(math, 'JavaScript');
  assert.deepEqual(signatures, [
    { k: 'fn', name: 'double', params: [{ name: 'x', type: { kind: 'float' } }], ret: { kind: 'float' } },
    { k: 'const', name: 'UNIT', type: { kind: 'string' }, literal: true },
  ]);
  const quad = [
    'import { double, UNIT as unit } from \'./math.mjs\';',
    '',
    '/** @param {number} x @returns {number} */',
    'export function quadruple(x) {',
    '  return double(double(x));',
    '}',
    '',
    '/** @param {string} name @returns {boolean} */',
    'export function isUnit(name) {',
    '  return name === unit;',
    '}',
    '',
  ].join('\n');
  const bound = selfTranslate(quad, 'JavaScript', 'Rust', { imports: { './math.mjs': signatures } });
  assert.ok(bound.items.every(({ status }) => status === 'translated'));
  const rust = emittedRust(bound);
  assert.match(rust, /^use crate::math::\{double, UNIT as unit\};$/mu);
  assert.match(rust, /^ {4}double\(double\(x\)\)$/mu);
  assert.match(rust, /^ {4}\(name == unit\)$/mu);
  // Without the signatures the import still translates; its users are carried.
  const unbound = selfTranslate(quad, 'JavaScript', 'Rust');
  assert.equal(statusOf(unbound, quad, 'import {'), 'translated');
  assert.equal(statusOf(unbound, quad, 'function quadruple'), 'carried');
  // A renamed function is named in snake case on both sides of `as`.
  const renamed = selfTranslate('import { cappedSquare as capped } from \'./math.mjs\';\n', 'JavaScript', 'Rust', {
    imports: { './math.mjs': [{ k: 'fn', name: 'cappedSquare', params: [], ret: { kind: 'float' } }] },
  });
  assert.match(emittedRust(renamed), /^use crate::math::capped_square as capped;$/mu);
  // The module directory places the module inside the crate.
  const nested = selfTranslate('import { realm } from \'../host.mjs\';\nimport { a, b } from \'./sub/part-two.js\';\n', 'JavaScript', 'Rust', { moduleDirectory: ['agentic', 'crate'] });
  assert.match(emittedRust(nested), /^use crate::agentic::host::realm;\nuse crate::agentic::crate_::sub::part_two::\{a, b\};$/mu);
});

test('imports outside the crate are refused with their own diagnostics', () => {
  const refused = (source) => {
    const translation = selfTranslate(source, 'JavaScript', 'Rust');
    assert.equal(translation.items[0].status, 'carried', source);
    return translateProgram(source, 'JavaScript', 'Rust').diagnostic.message;
  };
  assert.match(refused('import fs from \'node:fs\';\n'), /^import from 'node:fs': Node\.js built-in modules are outside the portable core/u);
  assert.match(refused('import { readFileSync } from \'node:fs\';\n'), /^import from 'node:fs': Node\.js built-in modules/u);
  assert.match(refused('import { parse } from \'links-notation\';\n'), /^import from 'links-notation': packages are outside the portable core/u);
  assert.match(refused('import * as m from \'./m.mjs\';\n'), /^namespace import: import the items of a module by name/u);
  assert.match(refused('import m from \'./m.mjs\';\n'), /^default import from '\.\/m\.mjs': a translated module has no default export/u);
  assert.match(refused('import {} from \'./m.mjs\';\n'), /^import \{\} from '\.\/m\.mjs': an import names the items it imports/u);
  // A whole program is one module, so translateProgram refuses a relative import.
  assert.match(translateProgram('import { f } from \'./m.mjs\';\n', 'JavaScript', 'Rust').diagnostic.message, /^import from '\.\/m\.mjs': a relative import names another module of a crate/u);
  // The crate root has no parent directory.
  const climbing = selfTranslate('import { f } from \'../m.mjs\';\n', 'JavaScript', 'Rust');
  assert.equal(climbing.items[0].status, 'carried');
  assert.throws(() => selfTranslationSignatures('fn main() {}\n', 'Rust'), SelfTranslationError);
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
    console.log('checking self-translation report module dependency contexts');
    execFileSync(process.execPath, [path.join(root, 'js/scripts/generate-self-translation-report.mjs'), '--out-dir', outDir, '--modules', 'language-support.js,self-translation.js'], { encoding: 'utf8' });
    const { schemaVersion, commit, decorators, modules, failures } = JSON.parse(readFileSync(path.join(outDir, 'self-translation-report.json'), 'utf8'));
    assert.equal(schemaVersion, 1);
    assert.equal(commit, process.env.GITHUB_SHA ?? execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim());
    assert.deepEqual(failures, []);
    assert.deepEqual(decorators, ['borrowed-name', 'bare-sum', 'bare-comparison']);
    assert.deepEqual(modules.map(({ module, rust }) => [module, rust]), [
      ['js/src/language-support.js', 'rust/src/language_support.rs'],
      ['js/src/self-translation.js', 'rust/src/self_translation.rs'],
    ]);
    for (const row of modules) {
      assert.equal(row.coverage.sourceBytes, readFileSync(path.join(root, row.module)).length, row.module);
      assert.equal(row.coverage.itemBytes + row.coverage.layoutBytes, row.coverage.sourceBytes, row.module);
      assert.equal(row.coverage.unrepresentedBytes, 0, row.module);
      assert.deepEqual(row.decorated.coverage, row.coverage, row.module);
      for (const [file, field] of [[row.module, 'sourceSha256'], [row.rust, 'rustSha256']]) {
        assert.equal(row[field], createHash('sha256').update(readFileSync(path.join(root, file))).digest('hex'));
      }
      assert.ok(row.items.translated > 0 && row.handWrittenLines > 0, row.module);
      assert.ok(row.sharedLines <= row.codeLines && row.identical <= row.matched && row.matched <= row.functions, row.module);
      // The decorated translation is measured beside the generic one.
      assert.ok(row.decorated.sharedLines <= row.decorated.codeLines && row.decorated.identical <= row.decorated.matched, row.module);
    }
    const report = readFileSync(path.join(outDir, 'self-translation-report.md'), 'utf8');
    for (const { module } of modules) assert.match(report, new RegExp(`\\| ${module} \\| `, 'u'));
    assert.match(report, /\| identical, decorated \|/u);
  } finally {
    rmSync(outDir, { recursive: true, force: true });
  }
  if (process.env.ISSUE_195_OBSERVATION_FILE) {
    assert.ok(process.env.ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY, 'acceptance downloads the complete published reports');
    execFileSync(process.execPath, [
      path.join(root, 'js/scripts/generate-self-translation-report.mjs'),
      '--verify-reports', process.env.ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY,
      '--commit', process.env.ISSUE_195_COMMIT,
    ], { encoding: 'utf8' });
  }
  observe('I195-SELF-TRANSLATION-SHARED-CORPUS', ['differencePerModulePublished'], 'the report measures each translated module against its hand-written Rust');
});

test('the report splits the modules into shards of about the same size', () => {
  const script = path.join(root, 'js/scripts/generate-self-translation-report.mjs');
  const list = (...args) => execFileSync(process.execPath, [script, '--list', ...args], { encoding: 'utf8' }).trim().split('\n');
  const all = list();
  const shards = [1, 2, 3].map((shard) => list('--shard', `${shard}/3`));
  // Every module is in exactly one shard.
  assert.deepEqual(shards.flat().sort(), [...all].sort());
  const bytes = (modules) => modules.reduce((sum, module) => sum + readFileSync(path.join(root, module)).length, 0);
  const largest = Math.max(...all.map((module) => bytes([module])));
  const sizes = shards.map(bytes);
  // Largest first to the lightest shard keeps the shards within one module of each other.
  assert.ok(Math.max(...sizes) - Math.min(...sizes) <= largest, String(sizes));
  assert.deepEqual(list('--modules', 'language-support.js,self-translation.js', '--shard', '2/2').length, 1);
  assert.throws(() => list('--shard', '3/2'), /--shard takes K\/N/u);
});
