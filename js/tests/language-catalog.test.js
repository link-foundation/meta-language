import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
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

test('grammar provenance names the locked grammar versions and parser digests', async () => {
  for (const language of inventory.languages) {
    const provenance = grammarProvenance(language.name);
    assert.deepEqual(
      provenance.map(({ id }) => id),
      [...(language.grammars ?? []), ...(language.builtinGrammar ? [language.builtinGrammar] : [])],
      language.name,
    );
    for (const grammar of provenance) {
      if (grammar.id === language.builtinGrammar) {
        // A built-in grammar is recorded with the digest of its specification.
        const declared = inventory.builtinGrammars[grammar.id];
        const specification = await readFile(new URL(`../../${declared.specification}`, import.meta.url));
        assert.equal(grammar.version, declared.version);
        assert.equal(grammar.parserSha256, createHash('sha256').update(specification).digest('hex'));
        continue;
      }
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

test('parsed networks record the grammar provenance of every language they parse', () => {
  for (const language of inventory.languages.filter(({ name }) => grammarProvenance(name).length)) {
    const network = LinkNetwork.parse(language.source, language.name);
    const recorded = network
      .parseGrammars()
      .filter((grammar) => grammar.language === language.name)
      .map(({ id, version, parserSha256 }) => ({ id, version, parserSha256 }));
    assert.deepEqual(recorded, grammarProvenance(language.name), language.name);
    assert.equal(network.reconstructText(), language.source, language.name);
  }
  const markdown = inventory.languages.find(({ name }) => name === 'Markdown');
  const network = LinkNetwork.parse(markdown.source, 'Markdown');
  const expected = ['Markdown', 'JavaScript', 'HTML'].flatMap((language) =>
    grammarProvenance(language).map((grammar) => ({ language, ...grammar })),
  );
  assert.deepEqual(network.parseGrammars(), expected);
  assert.deepEqual(
    LinkNetwork.parse('plain text\n', 'txt').parseGrammars(),
    grammarProvenance('txt').map((grammar) => ({ language: 'txt', ...grammar })),
  );
});
