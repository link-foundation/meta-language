// Issue #195 proof preservation: every Lean and Rocq theorem of the pinned
// translation corpus (parity/fixtures/four-language-conformance.json) is
// carried by the public `translateProgram` as an obligation stating the source
// statement, so every false restatement is rejected by the source kernel and
// by every target; into Lean and Rocq each theorem and assertion is discharged
// by the target kernel itself, whose assumption report shows the obligation
// closed without an axiom of the translation or of another kernel; and into
// JavaScript and Rust a theorem is a bounded check reported apart from the
// source-kernel proof, which a restatement true only on the bounded domain
// passes while every kernel rejects it. The Rust twin is
// rust/tests/unit/issue_195_semantics_proof_preservation.rs.
import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';

import { translateProgram } from '../src/index.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';

const root = new URL('../../', import.meta.url);
const corpus = JSON.parse(await readFile(new URL(ISSUE_195_FIXTURE_FILES.fourLanguage, root), 'utf8')).translationCorpus;
const corpusDirectory = new URL(`${corpus.directory}/`, root);
const LANGUAGES = Object.keys(corpus.sources);
const PROOF_SOURCES = LANGUAGES.filter((language) => corpus.sources[language].theorems.length > 0);
const TOOLS = { JavaScript: 'node', Rust: 'rustc', Lean: 'lean', Rocq: 'rocq' };
// The axioms of Lean's own foundations; any other axiom would be an outside authority.
const LEAN_STANDARD_AXIOMS = new Set(['propext', 'Quot.sound', 'Classical.choice']);
const OBLIGATION_FIELDS = ['check', 'closedGoal', 'discharge', 'kind', 'source', 'target'];
// Acceptance runs must use every toolchain; other runs skip when one is absent.
const toolchainRequired = Boolean(process.env.ISSUE_195_OBSERVATION_FILE);
const REQUIREMENT_ID = 'I195-SEMANTICS-PROOF-PRESERVATION';

function record(assertion, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT_ID,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT_ID.toLowerCase()}`,
    fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
    assertions: [assertion],
    testName,
  });
}

function run(command, args, cwd) {
  return new Promise((resolve) => {
    execFile(command, args, { cwd, maxBuffer: 1 << 26, encoding: 'utf8' }, (error, stdout, stderr) => {
      resolve({ ok: !error, stdout, stderr });
    });
  });
}

async function toolchainsPresent(t) {
  const present = await Promise.all(LANGUAGES.map((language) => run(TOOLS[language], ['--version'])));
  if (present.every(({ ok }) => ok)) return true;
  assert.equal(toolchainRequired, false, 'node, rustc, lean and rocq are required for acceptance');
  t.skip('a JavaScript, Rust, Lean or Rocq toolchain is not installed');
  return false;
}

const sourceText = (language) => readFile(new URL(corpus.sources[language].file, corpusDirectory), 'utf8');

function mutate(text, { find, replace }) {
  assert.equal(text.split(find).length, 2, `the mutation site ${JSON.stringify(find)} is unique`);
  return text.replace(find, replace);
}

/**
 * Checks `code` with the `language` toolchain. A kernel (Lean, Rocq) checks
 * every proof and reports the assumptions each of `names` rests on; a program
 * (JavaScript, Rust) runs its `--ml-check-theorems` bounded checks.
 */
async function check(language, code, directory, name, names = []) {
  if (language === 'Lean' || language === 'Rocq') {
    const lean = language === 'Lean';
    const file = path.join(directory, `${name}${lean ? '.lean' : '.v'}`);
    const report = names.map((theorem) => (lean ? `#print axioms ${theorem}` : `Print Assumptions ${theorem}.`));
    await writeFile(file, `${code}\n${report.join('\n')}\n`);
    const result = lean ? await run('lean', [file]) : await run('rocq', ['compile', '-q', path.basename(file)], directory);
    const diagnostics = `${result.stdout}\n${result.stderr}`;
    return { accepted: result.ok && !/warning|error|sorry/iu.test(diagnostics), diagnostics };
  }
  let program;
  if (language === 'JavaScript') {
    program = path.join(directory, `${name}.mjs`);
    await writeFile(program, code);
    const theorems = await run('node', [program, '--ml-check-theorems']);
    return { accepted: true, diagnostics: theorems.stderr, theorems };
  }
  const file = path.join(directory, `${name}.rs`);
  program = path.join(directory, `${name}${process.platform === 'win32' ? '.exe' : ''}`);
  await writeFile(file, code);
  const linker = process.env.CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER;
  const compiled = await run('rustc', [
    '--edition', '2024', '-O', ...(process.platform === 'win32' && linker ? ['-C', `linker=${linker}`] : []),
    '-o', program, file,
  ]);
  if (!compiled.ok) return { accepted: false, diagnostics: compiled.stderr };
  const theorems = await run(program, ['--ml-check-theorems']);
  return { accepted: true, diagnostics: theorems.stderr, theorems };
}

/** The axioms Lean reports for `name`, or `null` when it reports nothing. */
function leanAxioms(diagnostics, name) {
  const line = diagnostics.split('\n').find((text) => text.startsWith(`'${name}' `));
  if (line === undefined) return null;
  if (line === `'${name}' does not depend on any axioms`) return [];
  const match = /^'[^']+' depends on axioms: \[([^\]]*)\]$/u.exec(line);
  return match ? match[1].split(',').map((axiom) => axiom.trim()) : null;
}

/** Whether the kernel run reports every one of `names` closed by its own foundations. */
function closedByKernel(language, diagnostics, names) {
  if (language === 'Lean') {
    return names.every((name) => leanAxioms(diagnostics, name)?.every((axiom) => LEAN_STANDARD_AXIOMS.has(axiom)) === true);
  }
  return !/^Axioms:/mu.test(diagnostics)
    && diagnostics.split('Closed under the global context').length - 1 === names.length;
}

function forbiddenMarkers(target, code) {
  const body = code.split('\n').slice(1).join('\n');
  return (corpus.targets[target].forbidden ?? []).filter((marker) => new RegExp(`\\b${marker}\\b`, 'u').test(body));
}

async function withDirectory(prefix, body) {
  const directory = await mkdtemp(path.join(tmpdir(), `issue-195-proof-${prefix}-`));
  try {
    return await body(directory);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

const theoremObligations = (translation) => translation.semantics.obligations.filter(({ kind }) => kind === 'theorem');

test('issue 195 theorems are carried as obligations stating the source theorem', { timeout: 900_000 }, async (t) => {
  if (!(await toolchainsPresent(t))) return;
  assert.deepEqual(PROOF_SOURCES, ['Lean', 'Rocq']);
  let rejected = 0;
  for (const source of PROOF_SOURCES) {
    const from = corpus.sources[source];
    const text = await sourceText(source);
    assert.deepEqual(from.theoremMutations.map(({ theorem }) => theorem), from.theorems);
    for (const target of LANGUAGES.filter((language) => language !== source)) {
      const translation = translateProgram(text, source, target);
      assert.equal(translation.contract.support, 'semantic-translation', `${source} -> ${target}`);
      const obligations = theoremObligations(translation);
      assert.deepEqual(obligations.map(({ source: name }) => name), from.theorems, `${source} -> ${target}`);
      for (const obligation of obligations) {
        const closed = new RegExp(`^(?:theorem|Theorem) ${obligation.source} :`, 'mu').test(text);
        assert.equal(obligation.closedGoal, closed, `${source} -> ${target} ${obligation.source}`);
        if (corpus.targets[target].proof) {
          assert.match(translation.code, new RegExp(`^(?:theorem|Theorem) ${obligation.target}\\b`, 'mu'));
        } else {
          assert.match(translation.code, new RegExp(`theorem ${obligation.source}: holds on the bounded domain`, 'u'));
        }
      }
    }
    // Every false restatement of a theorem is rejected by the source kernel
    // and by every target, so the carried obligation is the source statement.
    await withDirectory(source.toLowerCase(), async (directory) => {
      for (const mutation of from.theoremMutations) {
        const mutated = mutate(text, mutation);
        const own = await check(source, mutated, directory, 'source');
        assert.equal(own.accepted, false, `${source} kernel rejects the false ${mutation.theorem}`);
        await Promise.all(LANGUAGES.filter((language) => language !== source).map(async (target) => {
          const translation = translateProgram(mutated, source, target);
          assert.equal(translation.contract.support, 'semantic-translation', `${source} -> ${target}`);
          assert.deepEqual(theoremObligations(translation).map(({ source: name }) => name), from.theorems);
          const result = await check(target, translation.code, directory, `${target}_${mutation.theorem}`.toLowerCase());
          if (corpus.targets[target].proof) {
            assert.equal(result.accepted, false, `${target} kernel rejects the false ${source} ${mutation.theorem}`);
          } else {
            assert.equal(result.accepted, true, result.diagnostics);
            assert.equal(result.theorems.ok, false, `${target} bounded check fails on the false ${mutation.theorem}`);
            assert.match(result.theorems.stderr, new RegExp(`theorem ${mutation.theorem} fails on a bounded input`, 'u'));
          }
          rejected += 1;
        }));
      }
    });
  }
  assert.equal(rejected, 24);
  record('theoremsCarriedAsObligations', 'issue 195 theorems are carried as obligations stating the source theorem');
});

test('issue 195 proof obligations into Lean and Rocq are discharged by the target kernel', { timeout: 900_000 }, async (t) => {
  if (!(await toolchainsPresent(t))) return;
  let discharged = 0;
  for (const target of LANGUAGES.filter((language) => corpus.targets[language].proof)) {
    await withDirectory(target.toLowerCase(), async (directory) => {
      await Promise.all(LANGUAGES.filter((language) => language !== target).map(async (source) => {
        const from = corpus.sources[source];
        const translation = translateProgram(await sourceText(source), source, target);
        const { obligations } = translation.semantics;
        assert.equal(obligations.filter(({ kind }) => kind === 'assertion').length, from.assertions);
        assert.equal(obligations.length, from.theorems.length + from.assertions, `${source} -> ${target}`);
        for (const obligation of obligations) {
          // The record names the obligation and its discharger; it carries no
          // verdict, which only the kernel run below gives.
          assert.deepEqual(Object.keys(obligation).sort(), OBLIGATION_FIELDS);
          assert.equal(obligation.discharge, 'target-kernel', `${source} -> ${target} ${obligation.source}`);
          assert.equal(obligation.check, null, `${source} -> ${target} ${obligation.source}`);
        }
        assert.deepEqual(forbiddenMarkers(target, translation.code), [], `${source} -> ${target} admits nothing`);
        const names = obligations.map(({ target: name }) => name);
        const result = await check(target, translation.code, directory, `from_${source}`.toLowerCase(), names);
        assert.equal(result.accepted, true, result.diagnostics);
        assert.equal(closedByKernel(target, result.diagnostics, names), true, result.diagnostics);
        discharged += names.length;
      }));
    });
  }
  // 4 theorems from each kernel source and 1 assertion from each program
  // source, into both kernels.
  assert.equal(discharged, 2 * (4 + 1 + 1));
  record('obligationsDischargedByTarget', 'issue 195 proof obligations into Lean and Rocq are discharged by the target kernel');
});

test('issue 195 bounded theorem checks are reported separately from proofs', { timeout: 900_000 }, async (t) => {
  if (!(await toolchainsPresent(t))) return;
  const programs = LANGUAGES.filter((language) => !corpus.targets[language].proof);
  assert.deepEqual(programs, ['JavaScript', 'Rust']);
  for (const source of LANGUAGES) {
    const from = corpus.sources[source];
    const text = await sourceText(source);
    await withDirectory(`bounded-${source}`.toLowerCase(), async (directory) => {
      // The proof of every theorem stays with the source kernel, which closes it.
      if (from.theorems.length > 0) {
        const own = await check(source, text, directory, 'source', from.theorems);
        assert.equal(own.accepted, true, own.diagnostics);
        assert.equal(closedByKernel(source, own.diagnostics, from.theorems), true, own.diagnostics);
      }
      await Promise.all(programs.filter((target) => target !== source).map(async (target) => {
        const translation = translateProgram(text, source, target);
        const { obligations } = translation.semantics;
        for (const obligation of obligations) {
          assert.notEqual(obligation.discharge, 'target-kernel', `${source} -> ${target} ${obligation.source}`);
          if (obligation.kind === 'theorem') {
            assert.equal(obligation.discharge, 'source-kernel');
            assert.equal(obligation.check, 'bounded');
          } else {
            assert.equal(obligation.discharge, 'runtime-assertion');
            assert.equal(obligation.check, null);
          }
        }
        const encoding = translation.semantics.encodings.find(({ id }) => id === 'theorem-properties');
        assert.equal(encoding !== undefined, from.theorems.length > 0, `${source} -> ${target}`);
        if (encoding) assert.match(encoding.statement, /bounded domain, and its proof remains checked by the source kernel$/u);
        const result = await check(target, translation.code, directory, `${target}_from_${source}`.toLowerCase());
        assert.equal(result.accepted, true, result.diagnostics);
        assert.equal(result.theorems.ok, true, result.theorems.stderr);
        const reported = result.theorems.stdout.split('\n').filter((line) => line.startsWith('theorem '));
        assert.deepEqual(reported, from.theorems.map((name) => `theorem ${name}: holds on the bounded domain`));
      }));
      // A restatement true on the bounded domain but false in general passes
      // the bounded checks, while the source kernel and both target kernels
      // reject it, so a passing bounded check is never counted as a proof.
      if (from.boundedOnlyMutation) {
        const mutated = mutate(text, from.boundedOnlyMutation);
        const { theorem } = from.boundedOnlyMutation;
        const own = await check(source, mutated, directory, 'bounded_only');
        assert.equal(own.accepted, false, `${source} kernel rejects the bounded-only ${theorem}`);
        await Promise.all(LANGUAGES.filter((target) => target !== source).map(async (target) => {
          const translation = translateProgram(mutated, source, target);
          const obligation = theoremObligations(translation).find(({ source: name }) => name === theorem);
          const result = await check(target, translation.code, directory, `bounded_only_${target}`.toLowerCase());
          if (corpus.targets[target].proof) {
            assert.equal(obligation.discharge, 'target-kernel');
            assert.equal(result.accepted, false, `${target} kernel rejects the bounded-only ${source} ${theorem}`);
          } else {
            assert.deepEqual([obligation.discharge, obligation.check], ['source-kernel', 'bounded']);
            assert.equal(result.theorems.ok, true, result.theorems.stderr);
            assert.match(result.theorems.stdout, new RegExp(`^theorem ${theorem}: holds on the bounded domain$`, 'mu'));
          }
        }));
      }
    });
  }
  assert.deepEqual(PROOF_SOURCES.map((source) => corpus.sources[source].boundedOnlyMutation?.theorem), ['sumTo_formula', 'sumTo_formula']);
  record('boundedChecksReportedSeparately', 'issue 195 bounded theorem checks are reported separately from proofs');
});

test('issue 195 Rocq proof steps rewrite with each rule a bounded number of times', () => {
  // An unbounded `rewrite <- ?ih` with `ih : Tree.mirror l = l` rewrites `l`
  // into `Tree.mirror l` forever, so a false restatement must meet a bound to
  // be rejected in seconds rather than after minutes of search.
  let steps = 0;
  for (const source of PROOF_SOURCES.filter((language) => language !== 'Rocq')) {
    const text = readFileSync(new URL(corpus.sources[source].file, corpusDirectory), 'utf8');
    for (const variant of [text, ...corpus.sources[source].theoremMutations.map((mutation) => mutate(text, mutation))]) {
      const { code } = translateProgram(variant, source, 'Rocq');
      for (const [, , rules] of code.matchAll(/\((rewrite(?: <-)?) ([^;]*); ml_close\)/gu)) {
        for (const rule of rules.split(', ')) assert.match(rule, /^\d+\?\S+$/u, `${source} -> Rocq rewrites with ${rule} unboundedly`);
        steps += 1;
      }
    }
  }
  assert.ok(steps > 0, 'the corpus proofs rewrite with hypotheses or lemmas');
});
