// Read one native grammar's row projection without loading fixture definitions
// or parsing the other native grammars in the inventory.
import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { nativeOracleKinds } from './build-language-catalog.mjs';
import { resolveProjectDataPath } from './project-data.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

/** Preserve the inventory fields and derive names from the selected actual grammar. */
export function readNativeGrammarProjection(id, inventory, readGrammar = file =>
  readFileSync(resolveProjectDataPath(join(root, file)), 'utf8')) {
  const { grammar, hidden, anonymous, extras, rootIncludesLeadingTrivia } = inventory.nativeGrammars[id];
  const oracleKinds = nativeOracleKinds(readGrammar(grammar));
  return { nativeGrammar: id, hidden, anonymous, extras, oracleKinds, ...(rootIncludesLeadingTrivia ? { rootIncludesLeadingTrivia } : {}) };
}
