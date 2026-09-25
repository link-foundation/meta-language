import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  buildLanguageCatalog,
  formatLanguageCatalog,
} from '../scripts/build-language-catalog.mjs';
import { GRAMMAR_LOCK } from '../src/programming-language-parser.js';
import {
  LANGUAGE_CATALOG,
  LinkNetwork,
  canonicalLanguageName,
  grammarProvenance,
  languageCandidatesForPath,
  languageEntry,
  languageForPath,
} from '../src/index.js';

const inventory = JSON.parse(
  await readFile(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'),
);

test('language catalog is generated from the inventory and shipped identically to Rust', async () => {
  const generated = formatLanguageCatalog(buildLanguageCatalog(inventory, GRAMMAR_LOCK));
  const shipped = await readFile(new URL('../src/data/language-catalog.json', import.meta.url), 'utf8');
  const rust = await readFile(
    new URL('../../rust/src/data/language-catalog.json', import.meta.url),
    'utf8',
  );
  assert.equal(shipped, generated);
  assert.equal(rust, shipped);
  assert.deepEqual(
    LANGUAGE_CATALOG.languages.map(({ name }) => name),
    inventory.languages.map(({ name }) => name),
  );
});

test('every inventory name and alias resolves case-insensitively to its language', () => {
  for (const language of inventory.languages) {
    for (const alias of [language.name, ...language.aliases]) {
      assert.equal(canonicalLanguageName(alias), language.name, alias);
      assert.equal(canonicalLanguageName(alias.toUpperCase()), language.name, alias);
      assert.equal(languageEntry(alias).name, language.name, alias);
    }
  }
  assert.equal(canonicalLanguageName('klingon'), undefined);
});

test('every registered extension dispatches to its language', () => {
  for (const language of inventory.languages) {
    for (const extension of inventory.extensionDispatch[language.name]) {
      const path = `src/Fixture${extension.toUpperCase()}`;
      assert.ok(languageCandidatesForPath(path).includes(language.name), `${language.name} ${extension}`);
      const owners = inventory.languages.filter(({ name }) =>
        inventory.extensionDispatch[name].includes(extension));
      // A shared suffix (every SQL dialect accepts .sql) selects the first
      // registered owner; a unique suffix selects its language exactly.
      assert.equal(languageForPath(path), owners[0].name, `${language.name} ${extension}`);
    }
  }
  assert.equal(languageForPath('schema.sql'), 'sql-ansi');
  assert.equal(languageForPath('schema.postgres.sql'), 'sql-postgres');
  assert.equal(languageForPath('proc.TSQL'), 'sql-server');
  assert.deepEqual(languageCandidatesForPath('q.sql'), [
    'sql-ansi',
    'sql-postgres',
    'sql-mysql',
    'sql-sqlite',
    'sql-server',
    'sql-oracle',
    'sql-bigquery',
    'sql-snowflake',
  ]);
  assert.equal(languageCandidatesForPath('q.mysql.sql')[0], 'sql-mysql');
  assert.equal(languageForPath('README'), undefined);
  assert.equal(languageForPath('archive.tar.gz'), undefined);
});

test('grammar provenance names the locked grammar versions and parser digests', () => {
  for (const language of inventory.languages) {
    const provenance = grammarProvenance(language.name);
    assert.deepEqual(provenance.map(({ id }) => id), language.grammars ?? [], language.name);
    for (const grammar of provenance) {
      const locked = GRAMMAR_LOCK.grammars[grammar.id];
      assert.equal(grammar.version, locked.version);
      assert.equal(grammar.parserSha256, locked.parserSha256);
    }
  }
});

test('extension dispatch parses through the ordinary API with the dispatched grammar', () => {
  const source = 'fn main() {}\n';
  const network = LinkNetwork.parse(source, languageForPath('main.rs'));
  assert.equal(network.reconstructText(), source);
  assert.ok(network.links().some((link) => link.metadata().term === 'function_item'));
});
