import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { LANGUAGE_CATALOG, LinkNetwork } from '../src/index.js';
import { loadedGrammarIds } from '../src/programming-language-parser.js';

const root = fileURLToPath(new URL('../..', import.meta.url));
const defaultParsers = [
  ['HTML', '<p>hello</p>', []],
  ['JSON', '{"value":1}', []],
  ['PDF', '%PDF-1.7\n%%EOF\n', []],
  ['Markdown', '# Title\n\n- item\n', ['markdown', 'markdown_inline']],
  ['YAML', 'key: value\n', ['yaml']],
  ['Kotlin', 'val x = 1\n', ['kotlin']],
];

test('default parser availability reports missing checkout assets without loading grammars', () => {
  const loadedBefore = loadedGrammarIds();
  const availability = LinkNetwork.languageParserAvailability();
  assert.deepEqual(availability.map(({ language }) => language), LANGUAGE_CATALOG.languages.map(({ name }) => name));

  const cache = process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE ?? resolve(root, '.external-grammars');
  for (const [language, , ids] of defaultParsers) {
    const missingGrammarAssets = ids.map((id) => `js/src/vendor/grammars/${id}.wasm.gz`)
      .filter((path) => !existsSync(resolve(root, path)) && !existsSync(resolve(cache, path)));
    assert.deepEqual(availability.find((entry) => entry.language === language), {
      language, available: missingGrammarAssets.length === 0, missingGrammarAssets,
    });
  }
  assert.deepEqual(loadedGrammarIds(), loadedBefore);
});

test('reported parser availability matches parsing in the current install', () => {
  const availability = LinkNetwork.languageParserAvailability();
  for (const [language, source] of defaultParsers) {
    const { available, missingGrammarAssets } = availability.find((entry) => entry.language === language);
    if (available) {
      assert.equal(LinkNetwork.parse(source, language).reconstructText(), source, language);
    } else {
      assert.throws(() => LinkNetwork.parse(source, language), (error) =>
        missingGrammarAssets.some((path) => error.message.includes(`${path}: pinned grammar artifact is absent`)));
    }
  }
});
