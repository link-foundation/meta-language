// Gives every rule of the seven native grammars an English name and a concept
// record (parity/naming/canonical-concepts.json). A rule renamed from its
// tree-sitter name keeps that name as `(source-names (tree-sitter NAME))`, so
// the default tree still uses the oracle's node kind. It is idempotent: an
// annotated grammar is rewritten byte for byte.
//
//   node experiments/issue-195-annotate-native-grammars.mjs
import { readFileSync, writeFileSync } from 'node:fs';

import { Grammar } from '../js/src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks } from '../js/src/grammar-links.js';

const root = new URL('../', import.meta.url);

// Inherited names that are not English words: hidden-rule underscores,
// run-together words and abbreviations.
const RENAMES = {
  json: { _value: 'value' },
  json5: { _value: 'value' },
  diff: { linerange: 'line_range' },
  scheme: { _token: 'program_element', _intertoken: 'atmosphere', _datum: 'datum' },
  racket: {
    _token: 'program_element', _skip: 'atmosphere', _datum: 'datum',
    sexp_comment: 'datum_comment', regex: 'regular_expression', lang_name: 'language_name',
  },
};
// Alias kinds of anonymous leaves, which no rule names.
const LEAF_RENAMES = { racket: { regex_prefix: 'regular_expression_prefix' } };

const CONCEPTS = {
  csv: {
    document: 'grammar.document', row: 'table-row', field: 'table-cell', number: 'grammar.csv.integer',
    float: 'grammar.csv.floating-point-number', boolean: 'grammar.boolean-value', quoted: 'grammar.string',
    text: 'grammar.csv.unquoted-text',
  },
  diff: {
    source: 'grammar.document', line: 'grammar.diff.patch-line', header: 'grammar.diff.file-header',
    block: 'grammar.diff.file-difference', hunks: 'grammar.diff.hunk-sequence', hunk: 'grammar.diff.hunk',
    changes: 'grammar.diff.change-sequence', change: 'grammar.diff.change', command: 'grammar.diff.difference-command',
    file_change: 'grammar.diff.file-change', binary_change: 'grammar.diff.binary-file-change',
    index: 'grammar.diff.index-line', similarity: 'grammar.diff.similarity-index', old_file: 'grammar.diff.old-file-line',
    new_file: 'grammar.diff.new-file-line', location: 'grammar.diff.hunk-header', addition: 'grammar.diff.added-line',
    deletion: 'grammar.diff.deleted-line', context: 'grammar.diff.context-line', comment: 'grammar.comment',
    filename: 'grammar.diff.file-name', commit: 'grammar.diff.commit-hash', mode: 'grammar.diff.file-mode',
    line_range: 'grammar.diff.line-range',
  },
  ini: {
    document: 'grammar.document', section: 'grammar.ini.section', section_name: 'grammar.ini.section-name',
    setting: 'grammar.ini.setting', comment: 'grammar.comment',
  },
  json: {
    document: 'grammar.document', value: 'grammar.value', object: 'grammar.object', pair: 'grammar.member',
    array: 'grammar.list', string: 'grammar.string', number: 'grammar.number', true: 'grammar.true-value',
    false: 'grammar.false-value', null: 'grammar.null', comment: 'grammar.comment',
  },
  json5: {
    file: 'grammar.document', value: 'grammar.value', object: 'grammar.object', member: 'grammar.member',
    identifier: 'grammar.identifier-name', array: 'grammar.list', string: 'grammar.string',
    number: 'grammar.number', null: 'grammar.null', true: 'grammar.true-value', false: 'grammar.false-value',
    comment: 'grammar.comment',
  },
  scheme: {
    program: 'grammar.program', program_element: 'grammar.program-element', atmosphere: 'grammar.atmosphere',
    comment: 'grammar.comment', directive: 'grammar.scheme.reader-directive', block_comment: 'grammar.block-comment',
    datum: 'grammar.datum', boolean: 'grammar.boolean-value', number: 'grammar.number',
    character: 'grammar.character-literal', string: 'grammar.string', escape_sequence: 'grammar.escape-sequence',
    symbol: 'grammar.symbol', keyword: 'grammar.keyword', list: 'grammar.linked-list', quote: 'grammar.quote',
    quasiquote: 'grammar.quasiquote', syntax: 'grammar.syntax-quote', quasisyntax: 'grammar.quasisyntax',
    unquote: 'grammar.unquote', unquote_splicing: 'grammar.unquote-splicing', unsyntax: 'grammar.unsyntax',
    unsyntax_splicing: 'grammar.unsyntax-splicing', vector: 'grammar.list', byte_vector: 'grammar.scheme.byte-vector',
    datum_label: 'grammar.scheme.datum-label', label: 'grammar.scheme.datum-label-marker',
    datum_reference: 'grammar.scheme.datum-reference',
  },
  racket: {
    program: 'grammar.program', program_element: 'grammar.program-element', atmosphere: 'grammar.atmosphere',
    dot: 'grammar.racket.pair-separator', comment: 'grammar.comment', block_comment: 'grammar.block-comment',
    datum_comment: 'grammar.racket.datum-comment', datum: 'grammar.datum', boolean: 'grammar.boolean-value',
    string: 'grammar.string', byte_string: 'grammar.racket.byte-string', here_string: 'grammar.racket.here-string',
    here_terminator: 'grammar.racket.here-string-terminator', here_newline: 'grammar.racket.here-string-line-break',
    here_line: 'grammar.racket.here-string-line', here_end: 'grammar.racket.here-string-end',
    regular_expression: 'grammar.racket.regular-expression-literal', escape_sequence: 'grammar.escape-sequence',
    number: 'grammar.number', decimal: 'grammar.racket.vector-length', character: 'grammar.character-literal',
    symbol: 'grammar.symbol', keyword: 'grammar.keyword', box: 'grammar.racket.box', list: 'grammar.linked-list',
    vector: 'grammar.list', structure: 'grammar.racket.structure-literal', hash: 'grammar.racket.hash-table',
    graph: 'grammar.racket.graph-notation', quote: 'grammar.quote', quasiquote: 'grammar.quasiquote',
    syntax: 'grammar.syntax-quote', quasisyntax: 'grammar.quasisyntax', unquote: 'grammar.unquote',
    unquote_splicing: 'grammar.unquote-splicing', unsyntax: 'grammar.unsyntax',
    unsyntax_splicing: 'grammar.unsyntax-splicing', extension: 'grammar.racket.reader-extension',
    language_name: 'grammar.racket.language-name',
  },
};

// Renames a name wherever the links form uses it: a rule head, a reference,
// the start rule, and an alias kind of the same name.
function renameText(text, from, to) {
  const name = from.replace(/[$()*+.?[\\\]^{|}]/gu, '\\$&');
  return text
    .replace(new RegExp(`\\(rule ${name} `, 'gu'), `(rule ${to} `)
    .replace(new RegExp(`\\(ref ${name}\\)`, 'gu'), `(ref ${to})`)
    .replace(new RegExp(`\\(start ${name}\\)`, 'gu'), `(start ${to})`)
    .replace(new RegExp(`\\(alias ${name} `, 'gu'), `(alias ${to} `);
}

for (const [language, concepts] of Object.entries(CONCEPTS)) {
  const path = new URL(`parity/grammars/native/${language}.lino`, root);
  let text = readFileSync(path, 'utf8');
  const renames = { ...RENAMES[language], ...LEAF_RENAMES[language] };
  for (const [from, to] of Object.entries(renames)) text = renameText(text, from, to);
  const parsed = parseGrammarLinks(text);
  const rules = [...parsed.rules].map(([name, rule]) => {
    const concept = concepts[name];
    if (!concept) throw new Error(`${language}: rule ${name} has no concept`);
    const original = Object.entries(RENAMES[language] ?? {}).find(([, to]) => to === name)?.[0];
    return [name, { ...rule, concept, ...(original ? { sourceNames: [{ source: 'tree-sitter', name: original }] } : {}) }];
  });
  const grammar = new Grammar(parsed.start, rules, parsed.sourceFormat, parsed.declarations);
  for (const name of Object.keys(concepts)) {
    if (!grammar.rules.has(name)) throw new Error(`${language}: no rule ${name}`);
  }
  const rendered = renderGrammarLinks(grammar);
  if (renderGrammarLinks(parseGrammarLinks(rendered)) !== rendered) throw new Error(`${language}: unstable rendering`);
  writeFileSync(path, rendered);
  console.log(`${language}: ${grammar.rules.size} rules annotated`);
}
