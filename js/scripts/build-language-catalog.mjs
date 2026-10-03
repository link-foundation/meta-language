#!/usr/bin/env node
// Generates the language catalog both runtimes ship from
// parity/language-grammar-inventory.json and the grammar lock: every
// registered language with its aliases, file extensions, and the grammars
// (with exact version and generated-parser digest) that parse it by default.
// A built-in grammar is recorded with the digest of its specification in
// parity/grammars, since the runtimes implement it instead of loading it.
// A native Links Notation grammar (parity/grammars/native) parses its
// language by default: it is recorded with the digest of its file, which both
// runtimes ship in src/data/native-grammars, and the tree-sitter grammars it
// replaces stay as `oracleGrammars`, the oracles its trees are checked against.
//
//   node js/scripts/build-language-catalog.mjs          # write both copies
//   node js/scripts/build-language-catalog.mjs --check  # fail on drift
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = join(root, 'parity/language-grammar-inventory.json');
const lockPath = join(root, 'js/src/vendor/grammars/grammar-lock.json');
export const CATALOG_PATHS = Object.freeze([
  join(root, 'js/src/data/language-catalog.json'),
  join(root, 'rust/src/data/language-catalog.json'),
]);
/** The directories both runtimes ship the native grammars from. */
export const NATIVE_GRAMMAR_DIRECTORIES = Object.freeze([
  join(root, 'js/src/data/native-grammars'),
  join(root, 'rust/src/data/native-grammars'),
]);

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

export function buildLanguageCatalog(inventory, lock) {
  const languages = inventory.languages.map((language) => {
    const lockedGrammars = (language.grammars ?? []).map((id) => {
      const locked = lock.grammars[id];
      if (!locked) throw new Error(`${language.name} uses unlocked grammar ${id}`);
      return { id, version: locked.version, parserSha256: locked.parserSha256 };
    });
    let grammars = lockedGrammars;
    let oracleGrammars;
    if (language.nativeGrammar) {
      const id = language.nativeGrammar;
      const native = inventory.nativeGrammars?.[id];
      if (!native) throw new Error(`${language.name} uses undeclared native grammar ${id}`);
      grammars = [{ id, version: native.version, parserSha256: sha256(readFileSync(join(root, native.grammar))) }];
      oracleGrammars = lockedGrammars;
    }
    if (language.builtinGrammar) {
      const id = language.builtinGrammar;
      const builtin = inventory.builtinGrammars?.[id];
      if (!builtin) throw new Error(`${language.name} uses undeclared built-in grammar ${id}`);
      const specification = readFileSync(join(root, builtin.specification));
      grammars.push({
        id,
        version: builtin.version,
        parserSha256: sha256(specification),
      });
    }
    return {
      name: language.name,
      family: language.family,
      aliases: language.aliases,
      extensions: inventory.extensionDispatch[language.name] ?? [],
      grammars,
      ...(oracleGrammars ? { oracleGrammars } : {}),
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
    nativeGrammars: nativeGrammarCatalog(inventory),
    languages,
  };
}

/**
 * The native grammars by id: the file each runtime loads from its
 * `src/data` directory and the leaf and node kinds whose tree-sitter
 * placement the default tree reproduces (docs/grammar/native-grammars.md).
 */
function nativeGrammarCatalog(inventory) {
  return Object.fromEntries(Object.entries(inventory.nativeGrammars ?? {}).map(([id, native]) => [id, {
    file: `native-grammars/${basename(native.grammar)}`,
    hidden: native.hidden,
    anonymous: native.anonymous,
    extras: native.extras,
  }]));
}

/** The shipped copies of the native grammars: `[path, text]` for each runtime. */
export function nativeGrammarCopies(inventory) {
  return Object.values(inventory.nativeGrammars ?? {}).flatMap((native) => {
    const text = readFileSync(join(root, native.grammar), 'utf8');
    return NATIVE_GRAMMAR_DIRECTORIES.map((directory) => [join(directory, basename(native.grammar)), text]);
  });
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
  const files = [...CATALOG_PATHS.map((path) => [path, text]), ...nativeGrammarCopies(inventory)];
  if (process.argv.includes('--check')) {
    const stale = [];
    for (const [path, wanted] of files) {
      const current = await readFile(path, 'utf8').catch(() => '');
      if (current !== wanted) stale.push(path.slice(root.length + 1));
    }
    if (stale.length > 0) {
      console.error(`language catalog is stale: ${stale.join(', ')}`);
      console.error('run: node js/scripts/build-language-catalog.mjs');
      process.exit(1);
    }
    console.log(`language catalog matches the inventory for ${inventory.languages.length} languages`);
    return;
  }
  await Promise.all(NATIVE_GRAMMAR_DIRECTORIES.map((directory) => mkdir(directory, { recursive: true })));
  await Promise.all(files.map(([path, wanted]) => writeFile(path, wanted)));
  console.log(`wrote the language catalog for ${inventory.languages.length} languages`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
