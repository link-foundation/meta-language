// Records the concept of every native grammar rule in
// parity/naming/canonical-concepts.json: a new record for each concept no
// record defines yet, and on every record the native grammars use, one
// `native:<language>` source alias per rule that references it. A record two
// or more native grammars use carries the sharing constraint; a record one
// grammar uses says that only that grammar defines the construct. It is
// idempotent: the native aliases and constraints are recomputed each run.
//
//   node experiments/issue-195-add-native-concept-records.mjs
//   node js/scripts/build-concept-records.mjs
import { readdirSync, readFileSync, writeFileSync } from 'node:fs';

const root = new URL('../', import.meta.url);
const registerPath = new URL('parity/naming/canonical-concepts.json', root);

const LANGUAGE_NAMES = { csv: 'CSV', diff: 'diff', ini: 'INI', json: 'JSON', json5: 'JSON5', racket: 'Racket', scheme: 'Scheme' };
const SHARED = 'Native grammars share this concept only where the construct means the same in each language.';
const specific = (language) => `Only the native ${LANGUAGE_NAMES[language]} grammar defines this construct.`;

// The definitions of the concepts the native grammars introduce.
const DEFINITIONS = {
  'grammar.document': 'A rule for the whole text of a data or document format.',
  'grammar.program': 'A rule for the whole source text of a program.',
  'grammar.program-element': 'A rule for one top-level element of a program: a datum or the atmosphere around it.',
  'grammar.atmosphere': 'A rule for the white space and comments between data, which the reader skips.',
  'grammar.comment': 'A rule for a comment, text the reader skips as no part of the data.',
  'grammar.block-comment': 'A rule for a comment between an opening and a closing delimiter that may nest.',
  'grammar.datum': 'A rule for one external representation of a value that the reader reads as data.',
  'grammar.character-literal': 'A rule for a literal that denotes a single character.',
  'grammar.escape-sequence': 'A rule for a backslash sequence inside a string that stands for one character.',
  'grammar.symbol': 'A rule for an identifier read as a symbol datum rather than as text.',
  'grammar.keyword': 'A rule for a hash-colon keyword datum, which is never a variable.',
  'grammar.linked-list': 'A rule for a parenthesized list of pairs that may end in a dotted pair.',
  'grammar.quote': 'A rule for an apostrophe before a datum, which reads as a quote form.',
  'grammar.quasiquote': 'A rule for a backquote before a datum, which reads as a quasiquote form.',
  'grammar.syntax-quote': 'A rule for a hash-apostrophe before a datum, which reads as a syntax form.',
  'grammar.quasisyntax': 'A rule for a hash-backquote before a datum, which reads as a quasisyntax form.',
  'grammar.unquote': 'A rule for a comma before a datum, which reads as an unquote form.',
  'grammar.unquote-splicing': 'A rule for a comma-at before a datum, which reads as an unquote-splicing form.',
  'grammar.unsyntax': 'A rule for a hash-comma before a datum, which reads as an unsyntax form.',
  'grammar.unsyntax-splicing': 'A rule for a hash-comma-at before a datum, which reads as an unsyntax-splicing form.',
  'grammar.true-value': 'A rule for the literal true alone.',
  'grammar.false-value': 'A rule for the literal false alone.',
  'grammar.identifier-name': 'A rule for an ECMAScript identifier name used as an unquoted member name.',
  'grammar.csv.integer': 'A rule for a CSV field that holds a whole number.',
  'grammar.csv.floating-point-number': 'A rule for a CSV field that holds a number with a fractional part or an exponent.',
  'grammar.csv.unquoted-text': 'A rule for a CSV field of plain text without quotation marks.',
  'grammar.diff.patch-line': 'A rule for one line of a patch outside a file difference block.',
  'grammar.diff.file-header': 'A rule for one extended header line of a file difference block.',
  'grammar.diff.file-difference': 'A rule for the difference of one file: its diff command, header lines and hunks.',
  'grammar.diff.hunk-sequence': 'A rule for the consecutive hunks of one file difference.',
  'grammar.diff.hunk': 'A rule for one hunk: its hunk header and the changed lines it covers.',
  'grammar.diff.change-sequence': 'A rule for the consecutive changed and context lines of one hunk.',
  'grammar.diff.change': 'A rule for one added, deleted or context line of a hunk.',
  'grammar.diff.difference-command': 'A rule for the diff command line that opens a file difference.',
  'grammar.diff.file-change': 'A rule for a header line that creates, deletes, renames or copies a file or changes its mode.',
  'grammar.diff.binary-file-change': 'A rule for the line that reports two binary files differ.',
  'grammar.diff.index-line': 'A rule for the index line with the object hashes of both file versions.',
  'grammar.diff.similarity-index': 'A rule for the line that gives how similar a renamed or copied file is.',
  'grammar.diff.old-file-line': 'A rule for the three-minus line that names the original file.',
  'grammar.diff.new-file-line': 'A rule for the three-plus line that names the changed file.',
  'grammar.diff.hunk-header': 'A rule for the double-at line that gives the line ranges of a hunk.',
  'grammar.diff.added-line': 'A rule for a line the change adds, marked with a plus sign.',
  'grammar.diff.deleted-line': 'A rule for a line the change deletes, marked with a minus sign.',
  'grammar.diff.context-line': 'A rule for an unchanged line shown around the changes of a hunk.',
  'grammar.diff.file-name': 'A rule for the path of a file a patch line names.',
  'grammar.diff.commit-hash': 'A rule for an abbreviated or full hexadecimal object hash.',
  'grammar.diff.file-mode': 'A rule for the octal permission mode of a file.',
  'grammar.diff.line-range': 'A rule for the start line and line count of one side of a hunk.',
  'grammar.ini.section': 'A rule for an INI section: its bracketed name and the settings under it.',
  'grammar.ini.section-name': 'A rule for the bracketed name that opens an INI section.',
  'grammar.ini.setting': 'A rule for one key and value line of an INI file.',
  'grammar.scheme.reader-directive': 'A rule for a hash-bang directive such as fold-case that changes how the reader reads.',
  'grammar.scheme.byte-vector': 'A rule for a bytevector literal, a hash-u8 list of bytes.',
  'grammar.scheme.datum-label': 'A rule for a datum labelled for later reference, as in hash-n-equals.',
  'grammar.scheme.datum-label-marker': 'A rule for the hash-n-equals marker that labels a datum.',
  'grammar.scheme.datum-reference': 'A rule for the hash-n-hash reference to a labelled datum.',
  'grammar.racket.pair-separator': 'A rule for the dot between the two parts of a pair.',
  'grammar.racket.datum-comment': 'A rule for a hash-semicolon comment that skips the next datum.',
  'grammar.racket.byte-string': 'A rule for a hash-quoted byte string literal.',
  'grammar.racket.here-string': 'A rule for a here string, whose lines run up to a line equal to its terminator.',
  'grammar.racket.here-string-terminator': 'A rule for the terminator text that opens a here string.',
  'grammar.racket.here-string-line-break': 'A rule for a line break inside a here string.',
  'grammar.racket.here-string-line': 'A rule for one line of a here string that is not its terminator.',
  'grammar.racket.here-string-end': 'A rule for the closing line of a here string, equal to its terminator.',
  'grammar.racket.regular-expression-literal': 'A rule for a hash-rx or hash-px regular expression literal.',
  'grammar.racket.vector-length': 'A rule for the decimal length prefix of a vector or flonum vector.',
  'grammar.racket.box': 'A rule for a hash-ampersand box around a datum.',
  'grammar.racket.structure-literal': 'A rule for a hash-s prefab structure literal.',
  'grammar.racket.hash-table': 'A rule for a hash-hash literal of key and value pairs, keyed by equality.',
  'grammar.racket.graph-notation': 'A rule for a graph label or reference that shares or cycles a datum.',
  'grammar.racket.reader-extension': 'A rule for a hash-reader or hash-lang line that hands reading to another reader.',
  'grammar.racket.language-name': 'A rule for the module path a hash-lang line names.',
};

const native = readdirSync(new URL('parity/grammars/native/', root))
  .filter((file) => file.endsWith('.lino'))
  .map((file) => file.slice(0, -'.lino'.length))
  .sort();
const uses = new Map();
for (const language of native) {
  const text = readFileSync(new URL(`parity/grammars/native/${language}.lino`, root), 'utf8');
  for (const [, rule, concept] of text.matchAll(/^\(rule ([a-z0-9_]+) .*\(concept ([^()\s]+)\)/gmu)) {
    if (!uses.has(concept)) uses.set(concept, []);
    uses.get(concept).push({ source: `native:${language}`, name: rule, language });
  }
}

const register = JSON.parse(readFileSync(registerPath, 'utf8'));
const isNativeConstraint = (text) => text === SHARED || /^Only the native .* grammar defines this construct\.$/u.test(text);
for (const record of register.concepts) {
  record.sourceAliases = record.sourceAliases.filter(({ source }) => !source.startsWith('native:'));
  record.constraints = record.constraints.filter((text) => !isNativeConstraint(text));
}
const byId = new Map(register.concepts.map((record) => [record.id, record]));
const added = [];
for (const [id, aliases] of [...uses].sort(([first], [second]) => first.localeCompare(second))) {
  let record = byId.get(id);
  if (!record) {
    const definition = DEFINITIONS[id];
    if (!definition) throw new Error(`${id} has no definition`);
    record = { id, phrase: id.split('.').at(-1).split('-').join(' '), role: 'concept', definition, constraints: [], sourceAliases: [], formerNames: [] };
    byId.set(id, record);
    added.push(record);
  }
  const languages = [...new Set(aliases.map(({ language }) => language))];
  record.constraints.push(languages.length > 1 ? SHARED : specific(languages[0]));
  record.sourceAliases.push(...aliases.map(({ source, name }) => ({ source, name })));
}
// The new records follow the grammar records the data grammars already use.
const anchor = register.concepts.findIndex(({ id }) => id === 'grammar.null');
register.concepts.splice(anchor + 1, 0, ...added);
for (const id of Object.keys(DEFINITIONS)) if (!uses.has(id)) throw new Error(`${id} is defined but no native rule uses it`);
writeFileSync(registerPath, `${JSON.stringify(register, null, 2)}\n`);
console.log(`${added.length} records added; ${uses.size} records carry native aliases`);
