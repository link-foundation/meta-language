// Where a grammar-lock entry's WebAssembly build and license live. The npm
// package ships the tree-sitter grammars of src/vendor/grammars; the oracles
// of the languages a native grammar parses (lock entries with `oracle: true`)
// are development files under js/oracles/grammars, which only the tests and
// the fixture generators load.

/** The repository-relative directory of the grammars the package ships. */
export const PACKAGED_GRAMMAR_DIRECTORY = 'js/src/vendor/grammars';
/** The repository-relative directory of the oracle grammars. */
export const ORACLE_GRAMMAR_DIRECTORY = 'js/oracles/grammars';

/** The repository-relative directory of a grammar-lock entry's files. */
export function grammarDirectory(entry) {
  return entry.oracle ? ORACLE_GRAMMAR_DIRECTORY : PACKAGED_GRAMMAR_DIRECTORY;
}

/** The repository-relative path of one file of a grammar-lock entry. */
export function grammarFile(entry, file) {
  return `${grammarDirectory(entry)}/${file}`;
}
