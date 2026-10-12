#!/usr/bin/env node
// Focused exact independent-oracle fixtures for native grammar candidates.
// A candidate is promoted only after its full CI corpus and parity checks.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeOracleKinds } from './build-language-catalog.mjs';
import { oracleSnapshot, nativeRows } from './native-grammar-rows.mjs';
import { corpusCases, sourceText } from './import-native-grammars.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

export function nativeScannerProfileFixture(language) {
  const spec = JSON.parse(readFileSync(join(root, `parity/fixtures/native-grammars/${language}-profile-sources.json`), 'utf8'));
  const registry = JSON.parse(readFileSync(join(root, 'parity/grammars/sources.json'), 'utf8'));
  const entry = registry.sources.find(source => source.id === spec.source);
  assert.ok(entry, 'the fixture source is independently pinned');
  sourceText(entry);
  const grammar = readFileSync(join(root, spec.grammar), 'utf8');
  const parser = compileGrammar(parseGrammarLinks(grammar));
  const projection = { ...spec, oracleKinds: nativeOracleKinds(grammar) };
  const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
  const matches = spec.matches.map(source => {
    const oracle = oracleSnapshot(source, spec.language);
    assert.equal(oracle.recovers, false, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, projection), oracle.rows, source);
    assert.equal(text(native.tree), source);
    return { source, rows: oracle.rows };
  });
  const rejections = spec.rejections.map(source => {
    assert.equal(oracleSnapshot(source, spec.language).recovers, true, source);
    assert.equal(parser.parseTree(source).ok, false, source);
    const native = parser.parseTree(source, { errorRecovery: true });
    assert.equal(native.rejection.reason, 'recovered', source);
    assert.equal(text(native.tree), source);
    return { source };
  });
  return { ...projection, matches, rejections, oracle: entry.package, revision: entry.revision, sourceSha256: entry.sha256 };
}

export function nativeScannerProfileCorpus(language) {
  const registry = JSON.parse(readFileSync(join(root, 'parity/grammars/sources.json'), 'utf8'));
  const entry = registry.sources.find(source => source.language === language);
  assert.ok(entry?.corpus, 'the complete upstream corpus is independently pinned');
  return { schemaVersion: 1, language, revision: entry.revision, sourceSha256: entry.sha256,
    corpusSha256: entry.corpus.sha256, cases: corpusCases(entry) };
}

function main(args) {
  const index = args.indexOf('--language');
  if (index < 0 || !/^[a-z_]+$/u.test(args[index + 1] ?? '')) throw new Error('Pass --language <catalog id>');
  const language = args[index + 1];
  const outputs = {
    [`${language}-profile.json`]: nativeScannerProfileFixture(language),
    [`${language}-profile-corpus.json`]: nativeScannerProfileCorpus(language),
  };
  for (const [name, value] of Object.entries(outputs)) {
    const file = join(root, `parity/fixtures/native-grammars/${name}`);
    const expected = `${JSON.stringify(value, null, 2)}\n`;
    if (args.includes('--check')) assert.equal(readFileSync(file, 'utf8'), expected, 'generated profile fixture differs');
    else writeFileSync(file, expected);
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main(process.argv.slice(2));
