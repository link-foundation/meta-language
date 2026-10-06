// Issue #195 grammar interchange API and command-line tool: every shared
// command of parity/fixtures/grammar-importers.json runs through
// runGrammarCommand and through the `meta-language grammar` executable
// (js/src/cli.js) in a directory holding its files, and both must give the
// exit code, standard output and standard error recorded there (the Rust
// binary gave the same when the fixture was written). The successful outputs
// are then read back independently: listings, conversions, exports, merges and
// renames are re-imported in their own format and must accept and reject the
// samples of the shared cases they came from. The Rust twin is
// rust/tests/unit/issue_195_interchange_api_cli.rs.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  GRAMMAR_COMMAND_USAGE,
  GRAMMAR_DIAGNOSTIC_KINDS,
  GRAMMAR_EXPORT_FORMATS,
  GRAMMAR_IMPORT_FORMATS,
  grammarImporter,
  parseNativeGrammar,
  parseWithGrammar,
  renderNativeGrammar,
  runGrammarCommand,
} from '../src/index.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const corpus = JSON.parse(readFileSync(path.join(root, ISSUE_195_FIXTURE_FILES.grammarImporters), 'utf8'));
const CLI = path.join(root, 'js', 'src', 'cli.js');
const REQUIREMENT_ID = 'I195-INTERCHANGE-API-CLI';

function record(assertion, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT_ID,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT_ID.toLowerCase()}`,
    fixtureFile: ISSUE_195_FIXTURE_FILES.grammarImporters,
    assertions: [assertion],
    testName,
  });
}

const sharedCase = (id) => corpus.cases.find((entry) => entry.id === id) ?? assert.fail(`no shared case ${id}`);
const command = (id) => corpus.commands.find((entry) => entry.id === id) ?? assert.fail(`no shared command ${id}`);
const commands = (prefix) => corpus.commands.filter(({ id }) => id.startsWith(prefix));

function fileContents(files) {
  return Object.fromEntries(Object.entries(files).map(([name, value]) => [
    name,
    typeof value === 'string' ? value : 'case' in value ? sharedCase(value.case).source : corpus.malformed[value.malformed].source,
  ]));
}

function checkRecorded(entry, actual, runtime) {
  assert.equal(actual.exitCode, entry.exitCode, `${entry.id} (${runtime}) exit code\n${actual.stderr}`);
  assert.equal(actual.stdout, entry.stdout, `${entry.id} (${runtime}) standard output`);
  if ('stderrHead' in entry) assert.equal(actual.stderr.split('\n')[0], entry.stderrHead, `${entry.id} (${runtime}) error`);
  else assert.equal(actual.stderr, entry.stderr, `${entry.id} (${runtime}) standard error`);
}

const runs = new Map();

/** Runs a shared command through the library and the executable, checks both against the record and returns the library result. */
function run(entry) {
  if (runs.has(entry.id)) return runs.get(entry.id);
  const files = fileContents(entry.files);
  const library = runGrammarCommand(entry.args, {
    readFile: (name) => {
      if (!Object.hasOwn(files, name)) throw new Error(`ENOENT: ${name}`);
      return files[name];
    },
  });
  checkRecorded(entry, library, 'library');
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-grammar-cli-'));
  try {
    for (const [name, text] of Object.entries(files)) {
      mkdirSync(path.dirname(path.join(directory, name)), { recursive: true });
      writeFileSync(path.join(directory, name), text);
    }
    const result = spawnSync(process.execPath, [CLI, 'grammar', ...entry.args], { cwd: directory, encoding: 'utf8' });
    checkRecorded(entry, { exitCode: result.status, stdout: result.stdout, stderr: result.stderr }, 'executable');
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
  runs.set(entry.id, library);
  return library;
}

function accepts(grammar, text) {
  try {
    parseWithGrammar(grammar, text);
    return true;
  } catch {
    return false;
  }
}

/** The output, re-imported in `format`, accepts and rejects the samples of `caseId`. */
function assertSamples(grammar, caseId, label) {
  const { accepts: accepted, rejects: rejected } = sharedCase(caseId);
  for (const sample of accepted) assert.ok(accepts(grammar, sample), `${label} accepts ${JSON.stringify(sample)}`);
  for (const sample of rejected) assert.ok(!accepts(grammar, sample), `${label} rejects ${JSON.stringify(sample)}`);
}

const optionValue = (args, name) => args[args.indexOf(name) + 1];
const caseOf = (entry) => Object.values(entry.files)[0].case;

test('every shared command gives the recorded result through the library and the executable', () => {
  assert.equal(corpus.commands.length, 75);
  assert.equal(new Set(corpus.commands.map(({ id }) => id)).size, corpus.commands.length);
  for (const entry of corpus.commands) run(entry);
  assert.equal(run(command('help')).stdout, GRAMMAR_COMMAND_USAGE);
  assert.equal(run(command('usage-without-command')).stderr, GRAMMAR_COMMAND_USAGE);
  assert.deepEqual([...GRAMMAR_IMPORT_FORMATS], ['abnf', 'antlr', 'bnf', 'ebnf', 'gbnf', 'lark', 'native', 'pest', 'tree-sitter-json']);
  assert.deepEqual(GRAMMAR_EXPORT_FORMATS, GRAMMAR_IMPORT_FORMATS);
  assert.equal(
    run(command('formats')).stdout,
    `import: ${GRAMMAR_IMPORT_FORMATS.join(' ')}\nexport: ${GRAMMAR_EXPORT_FORMATS.join(' ')}\n`,
  );
  for (const entry of commands('error:')) {
    assert.equal(entry.exitCode, 2, entry.id);
    assert.equal(entry.stdout, '', entry.id);
    assert.match(entry.stderr ?? entry.stderrHead, /^error: /, entry.id);
  }
});

test('grammar import lists every shared case as a native grammar that reads back to the same language', () => {
  const imports = commands('import:');
  assert.equal(imports.length, corpus.cases.length);
  for (const entry of imports) {
    const kase = sharedCase(caseOf(entry));
    assert.deepEqual(entry.args, ['import', '--from', kase.format, Object.keys(entry.files)[0]]);
    const listing = parseNativeGrammar(run(entry).stdout);
    assert.equal(listing.start, kase.start, entry.id);
    for (const rule of kase.rules) assert.ok(listing.rule(rule), `${entry.id} lists ${rule}`);
    assert.equal(renderNativeGrammar(listing), entry.stdout, `${entry.id} listing is a fixed point`);
    assertSamples(listing, kase.id, entry.id);
  }
  const rejected = [...commands('error:malformed:'), ...commands('error:native:')];
  assert.equal(rejected.length, corpus.malformed.length + 11);
  for (const entry of rejected) {
    const [, , format, file] = entry.args;
    assert.ok((entry.stderr ?? entry.stderrHead).startsWith(`error: cannot import ${file} as ${format} (`), entry.id);
  }
  record('importCommand', 'grammar import lists every shared case as a native grammar that reads back to the same language');
});

test('grammar validate reports every diagnostic of a broken grammar and exits 1', () => {
  assert.equal(run(command('validate:abnf:message')).stdout, '0 error(s), 0 warning(s)\n');
  const problems = command('validate:problems');
  assert.equal(run(problems).exitCode, 1);
  const lines = problems.stdout.trimEnd().split('\n');
  const found = lines.slice(0, -1).map((line) => {
    const [, severity, kind, rule] = /^(error|warning) ([a-z-]+) ([^:]+): /.exec(line) ?? assert.fail(line);
    assert.ok(GRAMMAR_DIAGNOSTIC_KINDS.includes(kind), kind);
    return `${severity} ${kind} ${rule}`;
  });
  // Read off problems.grammar by hand: `expr` and `a -> b -> a` recurse on
  // the left, `missing` is undefined, `repeat0(optional(...))` and `b` can
  // match nothing, `lbl` is never used, and `orphan`, `a` and `b` are not
  // reachable from `expr`.
  assert.deepEqual(found.toSorted(), [
    'error left-recursion a',
    'error left-recursion expr',
    'error undefined-non-terminal term',
    'warning nullable-repetition b',
    'warning nullable-repetition term',
    'warning unreachable-rule a',
    'warning unreachable-rule b',
    'warning unreachable-rule orphan',
    'warning unused-capture term',
  ]);
  const errors = found.filter((line) => line.startsWith('error ')).length;
  assert.equal(lines.at(-1), `${errors} error(s), ${found.length - errors} warning(s)`);
  record('validateCommand', 'grammar validate reports every diagnostic of a broken grammar and exits 1');
});

test('grammar convert writes every target format so that it re-imports to the same language', () => {
  const converts = commands('convert:');
  assert.deepEqual(new Set(converts.filter(({ exitCode }) => exitCode === 0).map(({ args }) => optionValue(args, '--to'))),
    new Set(['pest', 'tree-sitter-json', 'ebnf', 'gbnf', 'antlr', 'lark']));
  for (const entry of converts) {
    const { exitCode, stdout, stderr } = run(entry);
    const to = optionValue(entry.args, '--to');
    if (exitCode !== 0) {
      assert.equal(exitCode, 2, entry.id);
      assert.equal(stdout, '', entry.id);
      assert.ok(stderr.startsWith(`error: cannot export as ${to}\n${to} emit unsupported construct: `), entry.id);
      continue;
    }
    for (const note of stderr.split('\n').filter(Boolean)) assert.match(note, /^lossy: /, entry.id);
    assertSamples(grammarImporter(to)(stdout), caseOf(entry), entry.id);
  }
  record('convertCommand', 'grammar convert writes every target format so that it re-imports to the same language');
});

test('grammar export writes a native listing in another format or as the normalized listing', () => {
  const [, sumListing] = Object.entries(command('export:native-to-abnf').files)[0];
  const abnf = grammarImporter('abnf')(run(command('export:native-to-abnf')).stdout);
  for (const sample of ['1', '1+2', '9+0+5']) assert.ok(accepts(abnf, sample), sample);
  for (const sample of ['', '+', '1+', '1+x', '12']) assert.ok(!accepts(abnf, sample), sample);
  const native = run(command('export:native-to-native')).stdout;
  assert.equal(native, sumListing.replace(/^# .*\n/u, ''));
  assert.equal(renderNativeGrammar(parseNativeGrammar(native)), native);
  record('exportCommand', 'grammar export writes a native listing in another format or as the normalized listing');
});

test('grammar merge writes one grammar for all sources and fails on an unproven required equivalence', () => {
  const merged = command('merge:bnf-and-ebnf');
  const { stdout, stderr } = run(merged);
  const grammar = grammarImporter('ebnf')(stdout);
  assertSamples(grammar, 'bnf:message', merged.id);
  assertSamples(grammar, 'ebnf:message', merged.id);
  // The equal `letter` and `digit` rules of both sources become one rule each.
  assert.equal(stdout.match(/^letter = /gmu).length, 1);
  assert.match(stderr, /^merged letter: message\.bnf:letter message\.ebnf:letter \(/mu);
  assert.match(stderr, /^merged digit: message\.bnf:digit message\.ebnf:digit \(/mu);
  const unresolved = command('merge:required-equivalence-unresolved');
  assert.equal(run(unresolved).exitCode, 1);
  assert.equal(unresolved.stderr.trimEnd().split('\n').at(-1), 'unresolved message.abnf:message = message.pest:word: not-proven');
  assertSamples(parseNativeGrammar(unresolved.stdout), 'abnf:message', unresolved.id);
  record('mergeCommand', 'grammar merge writes one grammar for all sources and fails on an unproven required equivalence');
});

test('grammar rename renames a rule with its references and keeps the old name as an alias', () => {
  const renamed = command('rename:referenced-rule');
  const { stdout, stderr } = run(renamed);
  const grammar = grammarImporter('ebnf')(stdout);
  assert.ok(grammar.rule('name'));
  assert.equal(grammar.rule('word'), undefined);
  assert.doesNotMatch(stdout, /\bword\b/u);
  assert.equal(stderr, 'alias name = word\n');
  assertSamples(grammar, 'ebnf:message', renamed.id);
  assert.equal(run(command('rename:unknown-rule')).stderr, 'error: grammar has no rule nothing\n');
  record('renameCommand', 'grammar rename renames a rule with its references and keeps the old name as an alias');
});

test('grammar round-trip preserves every shared case and reports samples a grammar gets wrong', () => {
  const trips = commands('round-trip:').filter(({ id }) => id !== 'round-trip:native-wrong-samples');
  assert.equal(trips.length, corpus.cases.length);
  for (const entry of trips) {
    const kase = sharedCase(caseOf(entry));
    const accepted = entry.args.flatMap((arg, index) => (entry.args[index - 1] === '--accept' ? [arg] : []));
    const rejected = entry.args.flatMap((arg, index) => (entry.args[index - 1] === '--reject' ? [arg] : []));
    assert.deepEqual([accepted, rejected], [kase.accepts, kase.rejects], entry.id);
    assert.equal(run(entry).stdout, 'preserved\n', entry.id);
  }
  const wrong = command('round-trip:native-wrong-samples');
  assert.equal(run(wrong).exitCode, 1);
  assert.deepEqual(wrong.stdout.trimEnd().split('\n'), [
    'broken',
    'sample-rejected imported: 1+x',
    'sample-accepted imported: 1+2',
    'sample-rejected reimported: 1+x',
    'sample-accepted reimported: 1+2',
  ]);
  record('roundTripCommand', 'grammar round-trip preserves every shared case and reports samples a grammar gets wrong');
});
