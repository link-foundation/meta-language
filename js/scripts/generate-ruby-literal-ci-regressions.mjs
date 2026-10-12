#!/usr/bin/env node
// Exact independent-oracle regression inputs from Ruby CI run 38227158183.
// Fixture generation records the oracle, never a native implementation result.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { oracleSnapshot } from './native-grammar-rows.mjs';
import { sourceText } from './import-native-grammars.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const read = file => JSON.parse(readFileSync(join(root, file), 'utf8'));
const ciTitles = ['global variables', 'complex', 'newline-delimited strings', 'basic heredocs',
  'heredocs with interpolation', 'heredocs with in args, pairs, and arrays'];
const controls = [
  { name: 'single input-ended percent literal', source: 'x=%\nfoo' },
  { name: 'input-ended percent literal in assignment list', source: 'x="a",%\nfoo' },
  { name: 'nested quoted heredoc interpolation', source: '<<T\n#{[1,"c #{2} d", 3]}\nT\n' },
  { name: 'multiline heredoc interpolation', source: '<<T\n#{\n # multiline\n foo(\n  1,\n  bar\n ).baz\n}\nT\n' },
  { name: 'unterminated interpolation in assignment list', source: 'x="a",%\n#{' },
];

export function rubyLiteralCIRegressionFixture() {
  const registry = read('parity/grammars/sources.json');
  const entry = registry.sources.find(({ id }) => id === 'tree-sitter-ruby');
  assert.ok(entry);
  sourceText(entry);
  const corpus = read('parity/fixtures/native-grammars/ruby-profile-corpus.json');
  assert.equal(corpus.corpusSha256, entry.corpus.sha256);
  const ciCases = ciTitles.map(title => {
    const found = corpus.cases.find(value => value.file === 'literals.txt' && value.title === title);
    assert.ok(found, title);
    return { name: title, corpus: { file: found.file, title: found.title }, source: found.source };
  });
  const cases = [...ciCases, ...controls].map(value => ({ ...value, ...oracleSnapshot(value.source, 'ruby') }));
  assert.equal(cases.at(-1).recovers, true);
  assert.ok(cases.slice(0, -1).every(({ recovers }) => recovers === false));
  return { schemaVersion: 1, language: 'ruby', ci: { run: '38227158183', job: '114737371624' },
    source: entry.id, revision: entry.revision, sourceSha256: entry.sha256,
    corpusSha256: entry.corpus.sha256, cases };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const file = join(root, 'parity/fixtures/native-grammars/ruby-literal-ci-regressions.json');
  const expected = `${JSON.stringify(rubyLiteralCIRegressionFixture(), null, 2)}\n`;
  if (process.argv.includes('--check')) assert.equal(readFileSync(file, 'utf8'), expected);
  else writeFileSync(file, expected);
}
