import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { buildNativeGrammarFixtureFromParser } from '../scripts/native-grammar-fixture.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

const entry = {
  id: 'synthetic', language: 'Fixture', grammar: 'fixture.lino', oracle: 'pinned fixture oracle',
  sources: ['https://example.invalid/pinned'], hidden: [], anonymous: [], extras: [], oracleKinds: {},
  matches: ['m', 'm'], divergences: [{ source: 'd', reason: 'independent rejection' }], rejections: ['r', 'r'],
};
const rows = source => [[0, null, 'source', 1, 0, source.length, ''], [1, null, 'identifier', 1, 0, source.length, '']];
const parser = {
  parseTree(source, options) {
    if (source === 'r' && !options?.errorRecovery) return { ok: false };
    const child = source === 'r'
      ? { type: 'error', text: source, start: 0, end: source.length }
      : { type: 'token', kind: 'identifier', text: source, start: 0, end: source.length };
    return { ok: true, rejection: source === 'r' ? { reason: 'recovered' } : undefined,
      tree: { type: 'node', kind: 'source', start: 0, end: source.length, children: [child] } };
  },
};
const snapshot = source => ({ rows: rows(source), recovers: source !== 'm' });

test('fixture builds share one fresh oracle snapshot per source while retaining every assertion and row', () => {
  const calls = [];
  const build = () => buildNativeGrammarFixtureFromParser(entry, parser, (source, language) => {
    calls.push([source, language]);
    return snapshot(source);
  });
  const fixture = build();
  assert.deepEqual(calls, [['m', 'Fixture'], ['d', 'Fixture'], ['r', 'Fixture']]);
  assert.deepEqual(fixture, {
    schemaVersion: 1, id: entry.id, language: entry.language, grammar: entry.grammar, oracle: entry.oracle,
    sources: entry.sources, hidden: [], anonymous: [], extras: [], oracleKinds: {},
    matches: [{ source: 'm', rows: rows('m') }, { source: 'm', rows: rows('m') }],
    divergences: [{ source: 'd', reason: 'independent rejection', rows: rows('d') }],
    rejections: [{ source: 'r', recovered: '(source (ERROR@0..1 "r"))' }, { source: 'r', recovered: '(source (ERROR@0..1 "r"))' }],
  });
  assert.deepEqual(build(), fixture);
  assert.equal(calls.length, 6, 'another build cannot reuse another isolated oracle history');
});

test('snapshot reuse preserves independent acceptance, row, divergence and rejection checks', () => {
  const changed = (source, mutate) => value => value === source ? mutate(snapshot(value)) : snapshot(value);
  assert.throws(() => buildNativeGrammarFixtureFromParser(entry, parser,
    changed('m', value => ({ ...value, recovers: true }))), /oracle recovers from/u);
  assert.throws(() => buildNativeGrammarFixtureFromParser(entry, parser,
    changed('m', value => ({ ...value, rows: [] }))), /disagree on/u);
  assert.throws(() => buildNativeGrammarFixtureFromParser(entry, parser,
    changed('d', value => ({ ...value, recovers: false }))), /accepts the divergence/u);
  assert.throws(() => buildNativeGrammarFixtureFromParser(entry, parser,
    changed('r', value => ({ ...value, recovers: false }))), /accepts the rejection/u);
  assert.throws(() => buildNativeGrammarFixtureFromParser(entry, {
    parseTree: source => ({ ...parser.parseTree(source), ok: true }),
  }, snapshot), /accepts "r"/u);
  assert.throws(() => buildNativeGrammarFixtureFromParser(entry, {
    parseTree: (source, options) => ({ ...parser.parseTree(source, options), rejection: { reason: 'failed' } }),
  }, snapshot), /does not recover from/u);
});

test('a fresh Agda oracle worker retains pinned rows without importing the public package or concept register', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/native-grammars/agda.json', import.meta.url));
  const expected = fixture.matches.find(({ source }) => source === 'module Grove where\n');
  assert.ok(expected);
  const loader = `export async function resolve(specifier, context, nextResolve) {
    const result = await nextResolve(specifier, context);
    if (/\\/src\\/(?:index|concept-records)\\.js$/u.test(result.url)) throw new Error('oracle loaded unrelated public package data');
    return result;
  }`;
  const result = spawnSync(process.execPath, [
    '--loader', `data:text/javascript;base64,${Buffer.from(loader).toString('base64')}`,
    fileURLToPath(new URL('../scripts/isolated-grammar-oracle.mjs', import.meta.url)),
  ], { input: JSON.stringify({ source: expected.source, language: 'Agda' }), encoding: 'utf8',
    env: { ...process.env, META_LANGUAGE_ORACLE_WORKER: '1' }, timeout: 30_000, maxBuffer: 1 << 20 });
  assert.equal(result.status, 0, result.error?.message ?? result.stderr);
  assert.deepEqual(JSON.parse(result.stdout), { rows: expected.rows, recovers: false });
});
