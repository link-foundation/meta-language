import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const read = (file) => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = (file) => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = readData('parity/fixtures/native-grammars/python.json');
const corpus = readData('parity/fixtures/native-grammars/python-corpus.json');
const parser = compileGrammar(parseGrammarLinks(read('parity/grammars/native/python.lino')));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

function matchesOracle(source, label) {
  assert.equal(oracleRecovers(source, 'Python'), false, label);
  // No resource override: repeated scanner precedence probes must share their
  // existing memo, rather than make an ordinary source exhaust this budget.
  const outcome = parser.parseTree(source);
  assert.equal(outcome.ok, true, `${label}: ${JSON.stringify(outcome.rejection)}`);
  assert.deepEqual(outcome.ambiguities, [], label);
  assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Python'), label);
  assert.equal(text(outcome.tree), source, label);
}

test('Python Strings retains its exact pinned corpus tree within the default resource budget', () => {
  const entry = corpus.cases.find(({ file, title }) => file === 'literals.txt' && title === 'Strings');
  assert.ok(entry, 'the original pinned Strings case is present');
  matchesOracle(entry.source, `${entry.file}: ${entry.title}`);
});

test('Python quoted scanner neighbors retain prefix, escape and continuation boundaries', () => {
  for (const source of [
    `"I'm ok"\n`, "'\"ok\"'\n", "UR'bye'\n", '`1`\n',
    '"multiline \\\nstring"\n',
    'b"\\x12\\u12\\U12\\x13\\N{WINKING FACE}"\n',
    "f'\\N{GREEK CAPITAL LETTER DELTA}'\n",
  ]) matchesOracle(source, JSON.stringify(source));
});
