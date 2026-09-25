#!/usr/bin/env node
// Generates the language catalog both runtimes ship from
// parity/language-grammar-inventory.json and the grammar lock: every
// registered language with its aliases, file extensions, and the grammars
// (with exact version and generated-parser digest) that parse it by default.
//
//   node js/scripts/build-language-catalog.mjs          # write both copies
//   node js/scripts/build-language-catalog.mjs --check  # fail on drift
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = join(root, 'parity/language-grammar-inventory.json');
const lockPath = join(root, 'js/src/vendor/grammars/grammar-lock.json');
export const CATALOG_PATHS = Object.freeze([
  join(root, 'js/src/data/language-catalog.json'),
  join(root, 'rust/src/data/language-catalog.json'),
]);

export function buildLanguageCatalog(inventory, lock) {
  const languages = inventory.languages.map((language) => {
    const grammars = (language.grammars ?? []).map((id) => {
      const locked = lock.grammars[id];
      if (!locked) throw new Error(`${language.name} uses unlocked grammar ${id}`);
      return { id, version: locked.version, parserSha256: locked.parserSha256 };
    });
    return {
      name: language.name,
      family: language.family,
      aliases: language.aliases,
      extensions: inventory.extensionDispatch[language.name] ?? [],
      grammars,
    };
  });
  const owners = new Map();
  for (const language of languages) {
    for (const alias of [language.name, ...language.aliases]) {
      const key = alias.toLowerCase();
      const owner = owners.get(key);
      if (owner && owner !== language.name) {
        throw new Error(`alias ${alias} is shared by ${owner} and ${language.name}`);
      }
      owners.set(key, language.name);
    }
  }
  return {
    generatedBy: 'js/scripts/build-language-catalog.mjs',
    source: 'parity/language-grammar-inventory.json',
    treeSitterCli: lock.treeSitterCli,
    languages,
  };
}

export function formatLanguageCatalog(catalog) {
  const languages = catalog.languages.map((language) => `    ${JSON.stringify(language)}`);
  const header = Object.entries(catalog)
    .filter(([key]) => key !== 'languages')
    .map(([key, value]) => `  ${JSON.stringify(key)}: ${JSON.stringify(value)},`);
  return `{\n${header.join('\n')}\n  "languages": [\n${languages.join(',\n')}\n  ]\n}\n`;
}

async function main() {
  const [inventory, lock] = await Promise.all(
    [inventoryPath, lockPath].map(async (path) => JSON.parse(await readFile(path, 'utf8'))),
  );
  const text = formatLanguageCatalog(buildLanguageCatalog(inventory, lock));
  if (process.argv.includes('--check')) {
    const stale = [];
    for (const path of CATALOG_PATHS) {
      const current = await readFile(path, 'utf8').catch(() => '');
      if (current !== text) stale.push(path.slice(root.length + 1));
    }
    if (stale.length > 0) {
      console.error(`language catalog is stale: ${stale.join(', ')}`);
      console.error('run: node js/scripts/build-language-catalog.mjs');
      process.exit(1);
    }
    console.log(`language catalog matches the inventory for ${inventory.languages.length} languages`);
    return;
  }
  await Promise.all(CATALOG_PATHS.map((path) => writeFile(path, text)));
  console.log(`wrote the language catalog for ${inventory.languages.length} languages`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
