// Issue #195 faithful behavior: every program of the faithful corpus
// (`faithfulBehavior` in parity/fixtures/four-language-conformance.json) is run
// with its own toolchain and, through the public `translateProgram`, in each of
// the other three of JavaScript, Rust, Lean and Rocq. Every translation prints
// the lines the source prints and stops with the message the source aborts
// with, or does not abort where the source does not: Rust's overflow and
// division panics, `panic!` and `unreachable!`, JavaScript's thrown errors and
// BigInt division by zero, the output printed inside a function or an
// assertion before an abort, and Lean's and Rocq's total arithmetic with
// unbounded numbers. A translation is the target's own program: it carries no
// source envelope, no assumption and no escape hatch of a proof target. A
// fault-injected translation that aborts with another message is told apart.
// The Rust twin is rust/tests/unit/issue_195_faithful_behavior.rs.
import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';

import { decodeProgramTranslation, translateProgram } from '../src/index.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';

const root = new URL('../../', import.meta.url);
const fixture = JSON.parse(await readFile(new URL(ISSUE_195_FIXTURE_FILES.fourLanguage, root), 'utf8'));
const { faithfulBehavior, translationCorpus } = fixture;
const directory = new URL(`${faithfulBehavior.directory}/`, root);
const LANGUAGES = ['JavaScript', 'Rust', 'Lean', 'Rocq'];
const BEHAVIORS = ['errorsPreserved', 'abortsPreserved', 'overflowPreserved', 'effectsPreserved'];
const EXTENSIONS = { JavaScript: '.mjs', Rust: '.rs', Lean: '.lean', Rocq: '.v' };
const TOOLS = { JavaScript: 'node', Rust: 'rustc', Lean: 'lean', Rocq: 'rocq' };
// Acceptance runs must use every toolchain; other runs skip when one is absent.
const toolchainRequired = Boolean(process.env.ISSUE_195_OBSERVATION_FILE);
const REQUIREMENT_ID = 'I195-SEMANTICS-FAITHFUL-BEHAVIOR';

function run(command, args, cwd) {
  return new Promise((resolve) => {
    execFile(command, args, { cwd, maxBuffer: 1 << 26, encoding: 'utf8' }, (error, stdout, stderr) => {
      resolve({ ok: !error, code: error ? error.code : 0, stdout, stderr });
    });
  });
}

const availability = new Map();
function toolAvailable(language) {
  if (!availability.has(language)) availability.set(language, run(TOOLS[language], ['--version']).then(({ ok }) => ok));
  return availability.get(language);
}

const printed = (stdout) => stdout.split('\n').slice(0, -1);
const quoted = (text) => [...text.matchAll(/"((?:[^"]|"")*)"/gu)].map((match) => match[1].replaceAll('""', '"'));
// `Eval vm_compute in main` of a Rocq main that can abort ends with the abort, `None` or `Some "message"`.
const ROCQ_OUTCOME = /,\s*(None|Some\s+"((?:[^"]|"")*)"(?:%string)?)\s*\)\s*:\s*list string \* option string\s*$/u;

/**
 * Runs `code` as a `language` program and observes it: the lines it prints and
 * the message it aborts with (`null` when it ends normally). A program that
 * its toolchain rejects, or that warns, is `{ rejected }`.
 */
async function observe(language, code, workspace, name) {
  const file = path.join(workspace, `${name}${EXTENSIONS[language]}`);
  await writeFile(file, code);
  if (language === 'JavaScript') {
    const result = await run('node', [file]);
    // An uncaught error ends node with status 1 and its `Name: message` on stderr.
    const abort = result.ok ? null : result.stderr.match(/^\w*Error(?:: (.*))?$/mu)?.[1] ?? result.stderr;
    return { lines: printed(result.stdout), abort, failed: result.ok ? 0 : result.code };
  }
  if (language === 'Rust') {
    const binary = path.join(workspace, `${name}${process.platform === 'win32' ? '.exe' : ''}`);
    const linker = process.env.CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER;
    // The source's arithmetic is checked, as in a debug build; a translation into Rust must abort on its own.
    const checks = name === 'source' ? ['-C', 'overflow-checks=on'] : [];
    const built = await run('rustc', [
      '--edition', '2024', '-O', ...checks, ...(process.platform === 'win32' && linker ? ['-C', `linker=${linker}`] : []),
      '-o', binary, file,
    ]);
    if (!built.ok || /warning/u.test(built.stderr)) return { rejected: built.stderr };
    const result = await run(binary, []);
    // A panic ends the program with status 101 and its message on the line after `panicked at`.
    const abort = result.ok ? null : result.stderr.match(/panicked at [^\n]*\n([^\n]*)/u)?.[1] ?? result.stderr;
    return { lines: printed(result.stdout), abort, failed: result.ok ? 0 : result.code };
  }
  if (language === 'Lean') {
    const result = await run('lean', ['--run', file]);
    if (/:\d+:\d+: (?:warning|error)/u.test(result.stdout + result.stderr)) return { rejected: result.stdout + result.stderr };
    // An uncaught `IO.userError` ends the program with status 1 and `uncaught exception: message`.
    const abort = result.ok ? null : result.stderr.match(/^uncaught exception: (.*)$/mu)?.[1] ?? result.stderr;
    return { lines: printed(result.stdout), abort, failed: result.ok ? 0 : result.code };
  }
  const result = await run('rocq', ['compile', '-q', path.basename(file)], workspace);
  if (!result.ok || /warning|error/iu.test(result.stderr)) return { rejected: result.stdout + result.stderr };
  const outcome = result.stdout.match(ROCQ_OUTCOME);
  if (!outcome) return { lines: quoted(result.stdout), abort: null, failed: 0 };
  return {
    lines: quoted(result.stdout.slice(0, outcome.index)),
    abort: outcome[1] === 'None' ? null : outcome[2].replaceAll('""', '"'),
    failed: 0,
  };
}

async function withWorkspace(prefix, body) {
  const workspace = await mkdtemp(path.join(tmpdir(), `issue-195-faithful-${prefix}-`));
  try {
    return await body(workspace);
  } finally {
    await rm(workspace, { recursive: true, force: true });
  }
}

const sourceText = (entry) => readFile(new URL(entry.file, directory), 'utf8');
const expected = (entry) => ({ lines: entry.lines, abort: entry.abort });
const outcome = ({ lines, abort }) => ({ lines, abort });

function forbiddenMarkers(target, code) {
  const body = code.split('\n').slice(1).join('\n');
  return (translationCorpus.targets[target].forbidden ?? []).filter((marker) => new RegExp(`\\b${marker}\\b`, 'u').test(body));
}

// The pairs and behaviors each translation was observed to preserve, filled by the tests below.
const preserved = new Map(BEHAVIORS.map((behavior) => [behavior, new Set()]));
const translated = new Set();
const completed = new Set();

test('the faithful corpus covers every source language and behavior', () => {
  assert.deepEqual(Object.keys(EXTENSIONS), LANGUAGES);
  for (const language of LANGUAGES) {
    for (const behavior of BEHAVIORS) {
      assert.ok(
        faithfulBehavior.cases.some((entry) => entry.language === language && entry.preserves.includes(behavior)),
        `a ${language} program shows ${behavior}`,
      );
    }
  }
  for (const entry of faithfulBehavior.cases) {
    assert.equal(EXTENSIONS[entry.language], path.extname(entry.file), entry.file);
    assert.ok(entry.preserves.every((behavior) => BEHAVIORS.includes(behavior)), entry.file);
  }
  // Aborts of every kind: arithmetic, explicit panics and thrown errors, and output before them.
  assert.ok(faithfulBehavior.cases.filter((entry) => entry.abort !== null).length >= 8);
  assert.ok(faithfulBehavior.cases.some((entry) => entry.abort !== null && entry.lines.length > 0));
});

for (const entry of faithfulBehavior.cases) {
  test(`${entry.file} behaves as its source in every other language`, { timeout: 900_000 }, async (t) => {
    const tools = await Promise.all(LANGUAGES.map(toolAvailable));
    if (!tools.every(Boolean)) {
      assert.equal(toolchainRequired, false, 'node, rustc, lean and rocq are required for acceptance');
      t.skip('a JavaScript, Rust, Lean or Rocq toolchain is not installed');
      return;
    }
    const text = await sourceText(entry);
    await withWorkspace(entry.file.replace(/\W/gu, '-'), async (workspace) => {
      const source = await observe(entry.language, text, workspace, 'source');
      assert.equal(source.rejected, undefined, source.rejected);
      assert.deepEqual(outcome(source), expected(entry), `${entry.file} runs as recorded`);
      for (const target of LANGUAGES.filter((language) => language !== entry.language)) {
        const pair = `${entry.language} -> ${target}`;
        const translation = translateProgram(text, entry.language, target);
        // No pass-through: the target's own program, with nothing assumed and no proof escape hatch.
        assert.equal(translation.contract.support, 'semantic-translation', `${pair}: ${JSON.stringify(translation.diagnostic)}`);
        assert.equal(translation.diagnostic, null, pair);
        assert.deepEqual(translation.semantics.assumptions, [], `${pair} assumes nothing`);
        assert.doesNotMatch(translation.code, /meta-language:portable-source-envelope/u, pair);
        assert.throws(() => decodeProgramTranslation(translation.code, target), pair);
        assert.deepEqual(forbiddenMarkers(target, translation.code), [], pair);
        if (entry.abort !== null && (target === 'Lean' || target === 'Rocq')) {
          assert.ok(translation.semantics.encodings.some(({ id }) => id === 'abort-threading'), `${pair} threads the abort`);
        }
        const observed = await observe(target, translation.code, workspace, `to_${target.toLowerCase()}`);
        assert.equal(observed.rejected, undefined, `${pair}: ${observed.rejected}`);
        assert.deepEqual(outcome(observed), expected(entry), `${pair} prints and aborts as the source`);
        // A program target ends with a failure status where the source aborts.
        if (target === 'JavaScript' || target === 'Rust' || target === 'Lean') {
          assert.equal(observed.failed !== 0, entry.abort !== null, `${pair} exit status`);
        }
        translated.add(pair);
        for (const behavior of entry.preserves) preserved.get(behavior).add(pair);
      }
    });
    completed.add(entry.file);
  });
}

test('a translation that aborts with another message is told apart from its source', { timeout: 900_000 }, async (t) => {
  if (!(await Promise.all(LANGUAGES.map(toolAvailable))).every(Boolean)) {
    assert.equal(toolchainRequired, false, 'node, rustc, lean and rocq are required for acceptance');
    t.skip('a JavaScript, Rust, Lean or Rocq toolchain is not installed');
    return;
  }
  await withWorkspace('mutant', async (workspace) => {
    for (const target of LANGUAGES) {
      const entry = faithfulBehavior.cases.find((candidate) => candidate.language !== target && candidate.abort !== null);
      const translation = translateProgram(await sourceText(entry), entry.language, target);
      assert.equal(translation.code.split(entry.abort).length > 1, true, `${target} states the abort message`);
      const mutant = translation.code.replaceAll(entry.abort, 'a different message');
      const observed = await observe(target, mutant, workspace, `mutant_${target.toLowerCase()}`);
      assert.equal(observed.rejected, undefined, `${target}: ${observed.rejected}`);
      assert.notDeepEqual(outcome(observed), expected(entry), `${target} observes the fault-injected abort`);
      assert.equal(observed.abort, 'a different message', target);
    }
  });
  completed.add('fault injection');
});

test('issue 195 faithful behavior holds in all 12 directed translations', (t) => {
  if (translated.size === 0 && !toolchainRequired) {
    t.skip('no translation ran');
    return;
  }
  const pairs = LANGUAGES.flatMap((source) => LANGUAGES.filter((target) => target !== source).map((target) => `${source} -> ${target}`));
  assert.deepEqual([...completed].sort(), [...faithfulBehavior.cases.map(({ file }) => file), 'fault injection'].sort());
  assert.deepEqual([...translated].sort(), [...pairs].sort());
  for (const behavior of BEHAVIORS) {
    assert.deepEqual([...preserved.get(behavior)].sort(), [...pairs].sort(), `${behavior} in every pair`);
  }
  recordIssue195Observations({
    requirementId: REQUIREMENT_ID,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT_ID.toLowerCase()}`,
    fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
    assertions: [...BEHAVIORS, 'noPassThrough'],
    testName: t.name,
  });
});
