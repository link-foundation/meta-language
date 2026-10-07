#!/usr/bin/env node
// Generates parity/fixtures/native-grammars/<language>.json: the corpus a
// native merged grammar in parity/grammars/native/ is checked against, with
// the rows of the pinned tree-sitter grammar the language's default parse
// used before, now an oracle; and parity/fixtures/native-default-cst-expected.json,
// the default concrete syntax trees of the languages these grammars parse.
//
//   node scripts/generate-native-grammar-fixtures.mjs           write the fixtures
//   node scripts/generate-native-grammar-fixtures.mjs --check   fail when one is stale
//
// `matches` are sources whose native rows equal the oracle rows. `divergences`
// are sources one merged source accepts and the oracle does not; the native
// grammar accepts them and the fixture keeps its rows. `rejections` are
// invalid sources: the oracle recovers with error nodes, the native grammar
// rejects them by default, and with `errorRecovery` repairs each into the
// lossless `recovered` tree of ERROR and MISSING leaves. `hidden`,
// `anonymous` and `extras`, from the `nativeGrammars` of
// parity/language-grammar-inventory.json, and `oracleKinds`, the tree-sitter
// names of the renamed rules, tell
// js/scripts/native-grammar-rows.mjs how the oracle shows the native leaves
// and nodes of those kinds.
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { compileGrammar, languageEntry, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { NATIVE_BATCH_FIXTURES } from './native-grammar-batch-fixtures.mjs';
import { nativeOracleKinds } from './build-language-catalog.mjs';
import { corpusCases, grammarSourceOf } from './import-native-grammars.mjs';
import { hasRecovery, nativeRows, oracleRecovers, oracleRows } from './native-grammar-rows.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const inventory = JSON.parse(readFileSync(path.join(root, 'parity/language-grammar-inventory.json'), 'utf8'));

// The id of a native grammar in the language inventory, with the kinds its
// default tree places as the oracle does, which both runtimes read from the
// language catalog.
function nativeGrammar(id) {
  const { grammar, hidden, anonymous, extras } = inventory.nativeGrammars[id];
  const oracleKinds = nativeOracleKinds(readFileSync(path.join(root, grammar), 'utf8'));
  return { nativeGrammar: id, hidden, anonymous, extras, oracleKinds };
}

// The empty last field the RFC 4180 ABNF allows and the oracle recovers from.
const EMPTY_LAST_FIELD = 'RFC 4180 section 2 reads an empty last field (non-escaped = *TEXTDATA) at the end of the input; tree-sitter-csv f6bf6e3 recovers from it.';

// The JSON5 1.0.0 extensions tree-sitter-json5-orchard 0.1.0 recovers from.
const JSON5_WHITESPACE = 'JSON5 1.0.0 section 8 (White Space) skips NBSP, LS, PS, the byte order mark and every Zs space; tree-sitter-json5-orchard 0.1.0 skips only the ASCII spaces of /\\s/ and recovers from them.';
const JSON5_IDENTIFIER = 'JSON5 1.0.0 section 3 (Objects) reads a member name as an ECMAScript 5.1 IdentifierName, with Nl letters, \\u escapes and Mn, Mc, Nd, Pc, ZWNJ and ZWJ parts; tree-sitter-json5-orchard 0.1.0 reads only [$_\\p{L}][$_\\p{L}0-9]* and recovers from them.';
const JSON5_ESCAPE = 'JSON5 1.0.0 section 5.1 (Escapes) reads \\0 before a non-digit, any other character that is not a digit, x or u as itself (NonEscapeCharacter), and a CR, LS or PS line continuation; tree-sitter-json5-orchard 0.1.0 recovers from them.';

// The R7RS small datum syntax tree-sitter-scheme 0.24.7 recovers from.
const SCHEME_BYTEVECTOR = 'R7RS small section 6.9 (Bytevectors) and section 7.1.2 (External representations) read a bytevector as #u8( followed by its bytes and ); tree-sitter-scheme 0.24.7 reads only the R6RS #vu8( form and recovers from it.';
const SCHEME_DATUM_LABEL = 'R7RS small section 2.4 (Datum labels) reads #<n>=<datum> as a labelled datum and #<n># as a reference to it; tree-sitter-scheme 0.24.7 recovers from them.';

// The Racket Reference reader syntax tree-sitter-racket 0.25.0 recovers from.
const RACKET_CHARACTER_NEWLINE = 'Racket Reference section 1.3.14 (Reading Characters) reads #\\ followed by any character c as the character c, a line feed included; tree-sitter-racket 0.25.0 reads no line feed after #\\ and recovers from it.';
const RACKET_SYMBOL_NEWLINE = 'Racket Reference section 1.3.1 (Delimiters and Dispatch) and section 1.3.2 (Reading Symbols) read a backslash outside a | pair as quoting the next character, a line feed included; tree-sitter-racket 0.25.0 quotes any character but a line feed and recovers from it.';

// The case of the pinned tree-sitter-javascript corpus the oracle recovers
// from: two object literals in a row are no expression.
const JAVASCRIPT_ORACLE_ERROR = 'Extra complex literals in expressions';

// The case of the pinned tree-sitter-lean corpus the oracle recovers from and
// the native grammar reads as Lean does, and why.
const LEAN_EXPLICIT_FUNCTION = 'Explicit Function';
const LEAN_EXPLICIT_FUNCTION_REASON = 'Theorem Proving in Lean 4 section 2.9 (Implicit Arguments) checks `@foo`, the function with all its arguments made explicit, as in `#check @ident` and `#check @List.cons`; tree-sitter-lean4 0.3.0 ends a command before an `@`, which may open the attributes of a next declaration (`@[simp] def`), and recovers from it.';

// The sources the TypeScript and TSX fixtures share past their upstream
// corpus: what each grammar accepts as tree-sitter-typescript does, and what
// both reject. TypeScript's `!g<T>()` calls `!g` and `await g<T>;`
// instantiates `await g`, where the generated parser forks at a declared
// conflict.
const TYPESCRIPT_MATCHES = Object.freeze([
  '', 'x;', 'let x: number = 1;', 'const f = (a: string, b?: number): void => {};', 'function f<T extends object>(a: T, ...b: T[]): T { return a; }',
  'interface A extends B { x: number; readonly y?: string; [k: string]: unknown }', 'type U = A | B & C;', 'type F = (a: number) => string;',
  'enum E { A = 1, B, C = "c" }', 'const enum E { A }', 'namespace N { export const x = 1; }', 'declare module "m" { export function f(): void; }',
  'abstract class A<T> implements I { private x: T; protected abstract m(): void; constructor(public y: number) { super(); } }',
  'let x = y as unknown as string;', 'let x = y satisfies Z;', 'let x = y!;', 'type K = keyof typeof o;', 'type M = { [P in keyof T]?: T[P] };',
  'type C<T> = T extends string ? "s" : never;', 'type L = `a${B}c`;', 'type T = [a: number, b?: string, ...rest: boolean[]];',
  'import type { A } from "a";', 'export type { B };', 'import x = require("x");', 'export = x;', 'declare global { interface Window { a: 1 } }',
  'function assert(x: unknown): asserts x is string {}', 'function f(this: Window) {}', '@dec class A { @prop() x = 1; }',
  '!g<T>();', 'await g<T>;', 'typeof g<number>(x);', 'for (const [k, v] of m) {}', 'a?.b ?? c;', 'x = `a${b}c`;',
  'async function f(): Promise<void> { await g<number>(); }', '// c\nlet x = 1 /* d */;\n',
  // ECMA-262 section 12.2 (White Space) and 12.3 (Line Terminators): a
  // no-break or ideographic space, the byte order mark and a line separator
  // separate tokens, as tree-sitter-typescript's extras read them.
  'let x = 1;\u3000let y;', 'let a;\u00a0let b;', 'x;\u2028y;', 'x;\ufeff',
]);
const TYPESCRIPT_REJECTIONS = Object.freeze([
  'const = 1;', 'let x: = 1;', 'interface A {', 'function f(', 'function f(): {', 'class A {', 'enum E {', 'let x = <;', 'namespace {',
  'type T = [;', '}', 'x = 1 +;', 'if (x', 'import {', 'let x: number[;',
]);

// The sources of the pinned tree-sitter-typescript repository a native
// grammar of it is imported from.
const typescriptSources = (native) => {
  const { repository, revision, path: grammarPath, corpus } = grammarSourceOf(native);
  return [`${repository}/blob/${revision}/${grammarPath}`, `${repository}/tree/${revision}/${corpus.path}`];
};

// The cases of the upstream test corpus pinned with a native grammar's source
// whose file and title `keep` holds.
const upstreamCorpus = (native, keep = () => true) =>
  corpusCases(grammarSourceOf(native)).filter(({ file, title }) => keep(file, title)).map(({ source }) => source);

// The cases of the pinned tree-sitter-lean corpus the oracle recovers from
// (`recovers`) or reads, but the one the native grammar reads as Lean does.
const leanCorpus = (recovers) => upstreamCorpus('native-lean', (_file, title) => title !== LEAN_EXPLICIT_FUNCTION)
  .filter((source) => oracleRecovers(source, 'Lean') === recovers);

// The cases of the pinned tree-sitter-rocq corpus the oracle recovers from
// (`recovers`) or reads.
const rocqCorpus = (recovers) => upstreamCorpus('native-rocq').filter((source) => oracleRecovers(source, 'Rocq') === recovers);

// The cases of the pinned tree-sitter-java corpus the oracle recovers from
// (`recovers`) or reads.
const javaCorpus = (recovers) => upstreamCorpus('native-java').filter((source) => oracleRecovers(source, 'Java') === recovers);

// The cases of the pinned tree-sitter-go corpus the oracle recovers from
// (`recovers`) or reads.
const goCorpus = (recovers) => upstreamCorpus('native-go').filter((source) => oracleRecovers(source, 'Go') === recovers);

// The cases of the pinned tree-sitter-regex corpus the oracle recovers from
// (`recovers`) or reads.
const regexCorpus = (recovers) => upstreamCorpus('native-regex').filter((source) => oracleRecovers(source, 'Regex') === recovers);

// The cases of the pinned tree-sitter-graphql corpus the oracle recovers
// from (`recovers`) or reads.
const graphqlCorpus = (recovers) => upstreamCorpus('native-graphql').filter((source) => oracleRecovers(source, 'GraphQL') === recovers);

// The cases of the pinned tree-sitter-proto corpus the oracle recovers from
// (`recovers`) or reads.
const protoCorpus = (recovers) => upstreamCorpus('native-proto').filter((source) => oracleRecovers(source, 'Protocol Buffers') === recovers);

// The cases of the pinned tree-sitter-make corpus the oracle recovers from
// (`recovers`) or reads.
const makeCorpus = (recovers) => upstreamCorpus('native-make').filter((source) => oracleRecovers(source, 'Make') === recovers);

// The cases of the pinned tree-sitter-solidity corpus the oracle recovers
// from (`recovers`) or reads.
const solidityCorpus = (recovers) => upstreamCorpus('native-solidity').filter((source) => oracleRecovers(source, 'Solidity') === recovers);

// ECMAScript reads a brace or a bracket that opens no quantifier or POSIX
// class as a character (ECMA-262, Annex B.1.2, ExtendedPatternCharacter and
// ClassAtom), and the native grammar does too; the tree-sitter-regex 0.25.0
// lexer takes `{` and `[:` as the literals there and recovers.
const REGEX_LITERAL_BRACKET = 'ECMA-262 Annex B.1.2 reads a { that opens no count quantifier, and a [: that opens no POSIX class, as characters; the tree-sitter-regex 0.25.0 lexer takes them as the literals and recovers.';

export const NATIVE_GRAMMARS = Object.freeze([
  ...NATIVE_BATCH_FIXTURES.map((entry) => {
    const source = grammarSourceOf(`native-${entry.id}`);
    return { divergences: [], ...entry, grammar: `parity/grammars/native/${entry.id}.lino`, oracle: source.package,
      sources: [`${source.repository}/tree/${source.revision}`,
        ...(source.corpus ? [`${source.corpus.repository ?? source.repository}/tree/${source.corpus.revision ?? source.revision}/${source.corpus.path}`] : [])],
      ...(source.corpus ? { corpus: { ...source.corpus, url: `${source.corpus.repository ?? source.repository}/tree/${source.corpus.revision ?? source.revision}/${source.corpus.path}` } } : {}),
      ...nativeGrammar(`native-${entry.id}`),
    };
  }),
  {
    id: 'lua', language: 'Lua', grammar: 'parity/grammars/native/lua.lino',
    oracle: 'tree-sitter-lua 0.5.0',
    sources: ['https://github.com/tree-sitter-grammars/tree-sitter-lua/tree/10fe0054734eec83049514ea2e718b2a56acd0c9', 'https://github.com/tree-sitter-grammars/tree-sitter-lua/tree/10fe0054734eec83049514ea2e718b2a56acd0c9/test/corpus'],
    ...nativeGrammar('native-lua'),
    matches: [
      inventory.languages.find(({ name }) => name === 'Lua').source,
      '', 'function f() end', 'function f() return 1 end',
      'local x = [[]]', 'local x = [=[é😀]=]',
      '--[[block]]\nlocal x = 1', '--[==[a]=]b]==]\nlocal x=2',
      'local x = [=[a]==]b]=]', 'local x=[=[a]=]; local y=[==[b]==]',
      'local function f(...) return ... end',
      '--\n-- next\n', '-- \n-- next\n', '--\n--[[block]]', 'f()\n("hi")\n"last"', 'f()(x)(y)', 'f(); (g)()',
      'a,b=1,2', 'local f=function(x) return x end; f(2)',
      'function t:f(x) return self.x+x end; t:f(2)',
      'local x=-1+2*3^4; local y=not x and true or false',
      'local x=0x1f; local y=1.25e-2', 'local t={1,2;a=3,}; return t',
      '#!/usr/bin/lua\nreturn 1',

      'for i=1,3 do print(i) end', 'for k,v in pairs(t) do print(k,v) end',
      'if a then return 1 elseif b then return 2 else return 3 end',
      'repeat x=x+1 until x>2', 'while ready do ready=false end',
      'local t={a=1,[2]="b"}; t.a=t[2]', '::again:: goto again',
      `local x=[${'='.repeat(255)}[é]${'='.repeat(255)}]`,
      `local x=[${'='.repeat(256)}[é]]`,
      'local x <const> = 1', '-- café\nreturn "é\\n"',
    ],
    divergences: [],
    rejections: ['local x = [=[bad]]', 'local x = [==[bad]=]', 'local x = [[bad',
      'local x = [=[bad]=', 'local x=[[a\0b]]', 'local x =', 'function f(', 'if a then', 'local = 1'],
  },
  {
    id: 'json',
    language: 'JSON',
    grammar: 'parity/grammars/native/json.lino',
    oracle: 'tree-sitter-json 0.24.8',
    sources: [
      'https://www.rfc-editor.org/rfc/rfc8259',
      'https://ecma-international.org/publications-and-standards/standards/ecma-404/',
      'https://github.com/tree-sitter/tree-sitter-json/blob/v0.24.8/grammar.js',
    ],
    // tree-sitter-json skips a leading byte order mark as whitespace; the
    // native grammar keeps it as a named leaf (RFC 8259 section 8.1 lets a
    // parser ignore it).
    ...nativeGrammar('native-json'),
    matches: [
      '{"name": "meta", "tags": [1, 2.5, true, null], "nested": {"ok": false}}\n',
      '[]', '{}', '""', '0', '-0', '1.', '1.5e10', '-2E-3', '12e3', '1e-7', '"\\u00e9\\n\\"\\\\\\/"', '"\\u12"',
      '  [1, 2]  \n', '\n\n{"a": 1}\n\n', '// lead\n{"a": /* in */ 1 /* tail */}\n// end\n',
      '[1, /* c */ [2], // x\n 3]', '{"a": /* c */ [1]}', '"Ωmé 漢字 😀"', '{"k": "\\ud83d\\ude00"}',
      '[[[[[]]]]]', '1 2 "three"', '', '   ', '/* only */', '{"a":{"b":{"c":[{"d":null}]}}}',
      '[\t1,\r\n2\f]', '[1,\v2]', '\ufeff[]', '\ufeff {"a": 1}\n', '"tab\there"', '{"": ""}', '01',
      [
        '{',
        '  "name": "meta-language",',
        '  "version": "0.70.0",',
        '  "private": false,',
        '  "keywords": ["links", "grammar", "parser"],',
        '  "engines": {"node": ">=20"},',
        '  "scripts": {"test": "node --test tests/*.test.js"},',
        '  "files": [],',
        '  "ratio": -0.125e-2,',
        '  "nothing": null',
        '}',
        '',
      ].join('\n'),
    ],
    divergences: [
      {
        source: '1e+5',
        reason: 'RFC 8259 section 6 allows a plus sign in an exponent (exp = e [ minus / plus ] 1*DIGIT); tree-sitter-json 0.24.8 allows only a minus.',
      },
      {
        source: '[2.5E+10, -1e+0]',
        reason: 'RFC 8259 section 6 allows a plus sign in an exponent; tree-sitter-json 0.24.8 allows only a minus.',
      },
    ],
    rejections: ['{"a" 1}', '[1,]', 'tru', '"a\nb"', '+1', '.5', '{"a": 1', ' \ufeff[]', '[1,\ufeff2]', '\u00a0[]'],
  },
  {
    id: 'ini',
    language: 'INI',
    grammar: 'parity/grammars/native/ini.lino',
    oracle: 'tree-sitter-ini 1.4.0',
    sources: [
      'https://github.com/justinmk/tree-sitter-ini/blob/v1.4.0/grammar.js',
      'https://docs.python.org/3/library/configparser.html#supported-ini-file-structure',
    ],
    // tree-sitter-ini starts the document at a leading blank line but skips
    // the spaces before it, keeps line breaks and comment markers as hidden
    // tokens inside their node, and parses a comment as an extra.
    ...nativeGrammar('native-ini'),
    matches: [
      'a=1\n', '', '\n', '\n\n', '   ', 'k = \n', 'k==v\n', 'k=v\n', 'a = b\nc = d\n', '[x]\n# c\n', '[a]\n[b]\n',
      'k = v=w\n', '[s]\n\tk = v\n', 'k = \t v\n', '[s.t]\nk=v\n', '[s]\r\n', 'k=v\t\n', '[s]\n# a\n# b\nk=v\n',
      '[s]\nk = v\n', '; c\n[s]\n', '# c\nk=\n', '#\n', '#\r\n', '\n; c\n', '[a b]\r\nk=v\r\n', '\r\n[s]\r\n',
      'k = v ; not comment\n', 'a b = c = d\n', 'k v = w\n', 'k  v = w\n', '  k=v\n', '\tk\t=\tv\t\n', 'k\n=v\n',
      '\n\n[x]\n\nk=v w \n', ' \n[x]\n', '[ s ]\n', '[a\nb]\n', '[s]  \n', '[s]\n\n', '[s]\nk=v\n\n', 'k=v\r\n\r\n',
      'k = v\n\n\n', 'k = v\n  j = w\n', '[s]\n\nk=v\n\n\nj=w\n', 'k=v\n[s]\n[t]\nx=1\n', 'k=v\n  ; c\n',
      '[s]\n; c\nk=v\n', '[s]\n  ; c\n  k=v\n', '[a]\nk=v\n; c\n[b]\n', '[s]\n; c\n[t]\n', 'k=v\n; c\nj=w\n[s]\n',
      'k=v\n\n; c\n\nj=w\n', 'ключ=значение\n', 'a=1\n[b]\nc=2\n; d\n\n[e]\n; f\n',
      [
        '; meta-language settings',
        'root = true',
        '',
        '[core]',
        'editor = vim',
        '  # indented comment',
        'path = C:\\Program Files\\meta',
        '',
        '[remote "origin"]',
        'url = https://example.com/repo.git?a=1&b=2',
        'fetch = +refs/heads/*:refs/remotes/origin/*',
        '',
      ].join('\n'),
    ],
    divergences: [
      {
        source: '; c',
        reason: 'Python configparser reads a last line without a line break, and tree-sitter-ini 1.4.0 accepts a setting or a section header there but not a comment.',
      },
      {
        source: '[s]\nk=v\n; c  ',
        reason: 'Python configparser reads a comment on the last line without a line break; tree-sitter-ini 1.4.0 requires one after a comment.',
      },
      {
        source: 'a=1',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
      {
        source: '[s]',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
      {
        source: '[s]  ',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
      {
        source: 'k=',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
      {
        source: 'k= ',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
      {
        source: 'k=v\t',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
      {
        source: 'k=v\r',
        reason: 'Python configparser reads a last line without a line break; tree-sitter-ini 1.4.0 inserts a missing one there, which only the has-error flag of its root shows.',
      },
    ],
    rejections: [
      '[s] ; c\nk=v\n', '[s] k=v\n', '[s]\tx\n', '[a]b\n', '[]\n', '[s\n', '[s]]\n', '[[s]\n', 'k=v\n[', 'k\n', '=v\n',
      'k=v\n=x\n', 'k;=v\n', 'k#x=v\n', 'k\tv=w\n',
    ],
  },
  {
    id: 'diff',
    language: 'Diff',
    grammar: 'parity/grammars/native/diff.lino',
    oracle: 'tree-sitter-diff 0.1.0',
    sources: [
      'https://www.gnu.org/software/diffutils/manual/html_node/Detailed-Unified.html',
      'https://git-scm.com/docs/git-diff#_generating_patch_text_with_p',
      'https://git-scm.com/docs/git-config#Documentation/git-config.txt-coreabbrev',
      'https://github.com/tree-sitter-grammars/tree-sitter-diff/blob/v0.1.0/grammar.js',
    ],
    // tree-sitter-diff keeps line breaks, the rest of a changed, comment or
    // hunk header line and the words of a file name as regular expression
    // tokens without rows of their own.
    ...nativeGrammar('native-diff'),
    matches: [
      'diff --git a/x b/x\nindex 1234567..89abcde 100644\n--- a/x\n+++ b/x\n@@ -1,2 +1,2 @@ f\n a\n-b\n+c\n', '+a\n',
      '-a\n', ' a\n', '# c\n', 'x', '\n', '--- a\n+++ b\n', '', ' ', 'a\n ', ' \n', '\n\n', 'x\r\n', 'x \n', '+ \n',
      '-  \n', '--- \n', '+++ \n', '+++x\n', '+++++\n', '+++ b\n', '-- x\n', '++ x\n', '----\n', '----x\n', '+++\n',
      '---\n', '@@ -1 +1 @@\n', '@@ -1,3 +1,4 @@ fn main() {\n', '@@ -1 +1 @@  \n', '@@-1 +1@@\n', '#\n', '#!/bin/sh\n',
      'mode\n', 'de x\n', 'ne x\n', 'files a\n', 'and\n', 'Bin\n', 'simil x\n', '@x\n', '\\ No newline at end of file\n',
      'new file mode 100644\n', 'deleted file mode 100644\n', 'new mode 100755\n', 'old mode 100644\n',
      'rename from a/x\n', 'rename to b/y z\n', 'index 1234567..89abcde\n', 'index 1234567..89abcde 100644\n',
      'index 0000000000000000000000000000000000000000..1234567890abcdef1234567890abcdef12345678\n',
      'similarity index 90%\n', 'similarity index 100 %\n', 'Binary files a/x and b/x differ\n',
      'Binary files android and b and c differ\n', 'Binary files x differ and y differ\n', 'diff -u a b\n',
      'diff --git a/x b/x\n', 'diff a b c\n', 'different a b\n', 'diff a b  \n',
      'diff --git a/x b/x\nnew file mode 100644\nindex 0000000..e69de29\n',
      'diff --git a/x b/y\nsimilarity index 100%\nrename from x\nrename to y\n',
      'diff --git a/x b/x\nold mode 100644\nnew mode 100755\n',
      'diff --git a/p.png b/p.png\nindex 1234567..89abcde 100644\nBinary files a/p.png and b/p.png differ\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n-a\n@@ -5 +5 @@\n+b\ndiff c d\n', 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n#x\n',
      'diff a b\n+++ b\n', 'diff a b\n-x\n', 'diff a b\n\nindex 1234567..89abcde\n', 'diff a b\n@@ -1 +1 @@\n a\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n\n a\n', 'diff a b\n--- a\n+++ b\n@@ -1,3 +1,3 @@\n a\n\n-b\n+c\n\n\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n-a\n\\ No newline at end of file\n+a\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n+++\n---\n', 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n++++x\n----y\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n x \n', 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\r\n-a\r\n+b\r\n',
      '--- a\n+++ b\n@@ -1 +1 @@\n-a\n+b\n--- c\n+++ d\n@@ -2 +2 @@\n-c\n+d\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n-a\ndiff c d\n--- c\n+++ d\n@@ -1 +1 @@\n+e\n',
      'From 1234567 Mon Sep 17 00:00:00 2001\nSubject: x\n---\n a | 2 +-\n\ndiff --git a/a b/a\n',
      'diff --git a/src/main.rs b/src/main.rs\nindex 3b18e51..a4c2f9d 100644\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1,5 +1,6 @@\n+use std::io;\n fn main() {\n-    println!("hi");\n+    let x = 1;\n+    println!("{x}");\n }\n',
      '--- a/x\t2024-01-01 10:00:00.000000000 +0100\n+++ b/x\t2024-01-02 11:00:00.000000000 +0100\n@@ -1 +1 @@\n-a\n+b\n',
      'diff --git a/x b/x\ndeleted file mode 100644\nindex 1234567..0000000\n--- a/x\n+++ /dev/null\n@@ -1,2 +0,0 @@\n-a\n-b\n',
      'diff --git a/x b/x\nnew file mode 100644\nindex 0000000..1234567\n--- /dev/null\n+++ b/x\n@@ -0,0 +1 @@\n+a\n',
      'diff --git a/x b/y\nsimilarity index 90%\nrename from x\nrename to y\nindex 1234567..89abcde 100644\n--- a/x\n+++ b/y\n@@ -1 +1 @@\n-a\n+b\n',
      'diff --git a/s b/s\nold mode 100644\nnew mode 100755\nindex 1234567..89abcde\n--- a/s\n+++ b/s\n@@ -1 +1,2 @@\n #!/bin/sh\n+echo hi\n',
      'diff --git a/x b/x\nindex 1234567..89abcde 100644\n--- a/x\n+++ b/x\n@@ -1 +1 @@\n-a\n\\ No newline at end of file\n+a\n\\ No newline at end of file\n',
      'diff --git a/a b/a\nindex 1234567..89abcde 100644\n--- a/a\n+++ b/a\n@@ -1 +1 @@\n-x\n+y\ndiff --git a/b b/b\nindex 1234567..89abcde 100644\n--- a/b\n+++ b/b\n@@ -2,0 +3 @@ ctx\n+z\n',
      'diff -ru a/dir b/dir\n--- a/dir/f\n+++ b/dir/f\n@@ -1,2 +1,2 @@\n line\n-old\n+new\n',
      'diff --git a/x b/x\nindex 1234567..89abcde 100644\n--- a/x\n+++ b/x\n@@ -10,7 +10,7 @@ class A:\n     def f(self):\n-        return 1\n+        return 2\n \n     def g(self):\n',
      'Binary files /dev/null and b/x differ\n',
      '# HG changeset patch\n# User a\ndiff -r 1234567 -r 89abcde x\n--- a/x\n+++ b/x\n@@ -1 +1 @@\n-a\n+b\n',
      'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n-a\n+b\n\n\ndiff c d\n', '+\n', '-\n', '++\n', '--\n', '++++\n', '----  \n',
      '# \n', '@@ -1 +1 @@ x @@\n', '@@ -1,0 +1,0 @@\n', 'similarity index 0%\n',
      'index 1234567890abcdef1234567890abcdef12345678..1234567890abcdef1234567890abcdef12345678 100644\n',
      'diff --stat a b\n', 'diff a  b\n', 'diff\ta\tb\n', 'rename from a b c\n', 'new file mode 100644  \n', ' a\n\n b\n',
      '\ta\n', '  \n x\n', 'x\n\n\n',
    ],
    divergences: [
      {
        source: 'index abcd..ef01 100644\n',
        reason: 'git abbreviates the object names of an index line to core.abbrev hexadecimal digits, at least 4 (git-config); tree-sitter-diff 0.1.0 requires 7 to 40.',
      },
      { source: ' new x\n', reason: 'GNU diffutils reads a unified diff line that starts with a space as a context line; tree-sitter-diff 0.1.0 lexes new, old, deleted and rename after the space as file header keywords.' },
      { source: ' rename z\n', reason: 'GNU diffutils reads a unified diff line that starts with a space as a context line; tree-sitter-diff 0.1.0 lexes new, old, deleted and rename after the space as file header keywords.' },
      { source: 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n new x\n', reason: 'GNU diffutils reads a unified diff line that starts with a space as a context line; tree-sitter-diff 0.1.0 lexes new, old, deleted and rename after the space as file header keywords.' },
      {
        source: 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n--- c\n+++ d\n',
        reason: 'GNU diffutils reads a hunk line that starts with a minus or plus sign as a deleted or added line whatever follows the sign, so --- c deletes the line -- c; tree-sitter-diff 0.1.0 lexes --- and +++ there as file header markers.',
      },
      {
        source: 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n+++i;\n',
        reason: 'GNU diffutils reads a hunk line that starts with a plus sign as an added line, so +++i; adds the line ++i;; tree-sitter-diff 0.1.0 lexes +++ there as a file header marker.',
      },
      {
        source: 'diff a b\n--- a\n+++ b\n@@ -1 +1 @@\n-a\n+c',
        reason: 'The source rule of tree-sitter-diff 0.1.0 reads a last line without a line break and the merged grammar reads one inside a block too; tree-sitter-diff 0.1.0 inserts a missing line break there, which only the has-error flag of its root shows.',
      },
      {
        source: 'diff a b\nindex 1234567..abcdef0',
        reason: 'The source rule of tree-sitter-diff 0.1.0 reads a last line without a line break and the merged grammar reads one inside a block too; tree-sitter-diff 0.1.0 inserts a missing line break there, which only the has-error flag of its root shows.',
      },
    ],
    rejections: [
      'news\n', 'indexes\n', 'diff\n', '@@\n', 'Binary\n', 'x\ry\n', 'index 123..456\n', 'similarity index x%\n', '@@ -1 @@\n',
      'new file\n', 'rename x\n', 'Binary files a b differ\n', 'diff a\n', 'old mode\n',
    ],
  },
  {
    id: 'csv',
    language: 'CSV',
    grammar: 'parity/grammars/native/csv.lino',
    oracle: 'tree-sitter-csv f6bf6e3',
    sources: [
      'https://www.rfc-editor.org/rfc/rfc4180',
      'https://github.com/tree-sitter-grammars/tree-sitter-csv/blob/f6bf6e35eb0b95fbadea4bb39cb9709507fcb181/common/define-grammar.js',
      'rust/vendor/tree-sitter-csv/NOTICE.md',
    ],
    // tree-sitter-csv skips the spaces after a closing quote as whitespace,
    // and keeps line breaks as a regular expression token without a row.
    ...nativeGrammar('native-csv'),
    matches: [
      '', '\n', '\r\n', '\r', 'a', 'a\n', 'a,1\n', 'a,b', 'a,b\nc,d', 'a,b\r\nc,d\r\n', '1,2,3\n4,5,6\n', '1\n2\n',
      ' a , 1 \n', '1.5,true,0x1F\n', '"a""b",c\r\n', '""', '""\n', '"a"', '"a"\n', '" "', '" a "', '"a,b"', '"a\nb"',
      '"a\r\nb"', '""""', '"""a"""', '"a""b""c"', '" "" "\n', 'x,""\n', 'a,,b\n', 'a,b,\n', ',a', ',\n', 'a,\n,b\n',
      'a\n\nb\n', 'a\n\n', '\n\n', '\n\n\n', '\n\na', '\n a', ' \n', '  \n  \n', ' \n1', 'a\n ', 'a\n\n\n', 'a\n \nb',
      'a\n\n b', 'a\n\n,b', 'a\n\n"b"', 'a\r\rb', 'a\r\n\r\nb', 'a\n\r\nb', 'a\n\rb', 'a\rb', 'a\r', 'a\r\n', '\r\na',
      '\r\r', '\r\r\n', '\r\n\r\n', '\n\r', '\t1', '1\t', '1 ', ' 1', ' 1 ', '  12', '\t\t1.5', ' 0x1', '\v1', '\f1',
      '12,3.4,0X1f,0x', '00', '0x', '0xG', '1.', '.5', '.5 ', '.5,5.\n', '.', '5..', '1.2.3', '1e5', '-1', '+1', '1a\n',
      '1 2', 'true,false,TRUE', 'true\nfalse', 'false,1.0,0x0', 'True', 't', 'truex', 'true1\n', 'truefalse', ' "a"',
      '"a" ', ' "a" ', '"a" ,b', '"a" \n', '"a"\t\n', '"a" \r\n', '"a"  ,  "b"', '"a" , "b" ', '"a"\v', '"a"\f\n',
      '"a"\f,b', '\t"a"', '\v"a"', '"a", 1', 'a, b', 'a ,b', 'a ,\n', 'a, 1', 'a, "b"', 'a, "b,c"', '1, 2', '\f', '\v',
      ' a', 'a ', ' 1', 'é,漢字\n',
    ],
    divergences: [
      { source: 'a,', reason: EMPTY_LAST_FIELD },
      { source: ',', reason: EMPTY_LAST_FIELD },
      { source: ',,', reason: EMPTY_LAST_FIELD },
      { source: '1,', reason: EMPTY_LAST_FIELD },
      { source: '"a",', reason: EMPTY_LAST_FIELD },
      { source: 'true,', reason: EMPTY_LAST_FIELD },
      { source: 'a\n,', reason: EMPTY_LAST_FIELD },
      { source: 'a,\r\n,', reason: EMPTY_LAST_FIELD },
    ],
    rejections: [
      '"a"b', 'a"b', '"a', '"a" "b"', '"a""', 'a,"b', '"\n', 'x\n"y', '"a"x,b', 'a,b"c\n', '"""', '"a"\n"b', 'a\n"b',
      'x,"', '"a"""b"',
    ],
  },
  {
    id: 'json5',
    language: 'JSON5',
    grammar: 'parity/grammars/native/json5.lino',
    oracle: 'tree-sitter-json5-orchard 0.1.0',
    sources: [
      'https://spec.json5.org/',
      'https://docs.rs/crate/tree-sitter-json5-orchard/0.1.0/source/grammar.js',
    ],
    // Comments are extras both trees keep; whitespace is invisible trivia.
    ...nativeGrammar('native-json5'),
    matches: [
      '{a: 1}', '{a:1,}', '[1,]//c\n', '1', 'null', 'true', 'false', '"a"', '\'a\'', '[]', '{}', '[ ]', '{ }',
      '\ufeff1', '\t1\v\f', ' [ 1 , 2 ] ', '{true:1}', '{null:1, Infinity:2}', '{$_é9:1}', '[.]',
      '[.e5]', '[+.]', '[-Infinity, +NaN, +0x1f, -.5e-3, 5.e+2]', '"\\x41\\u0041\\/\\v"', '"a\nb"', '"a\\\nb"',
      '"a\\\r\nb"', '\'it\\\'s\'', '[1//c\r\n,2]', '[1/**/]', '[1/***/]', '[1/*a**b*/]',
      '{"a": [1, {b: null}], c: \'x\'}', '[0, 0.0, 0e0, 0E+1, 1.5, 123, 9e-9, 0X0, 0xABCdef]', '// head\n{a:1}',
      '/* head */ 1', '1 // tail', '1 /* tail */', '[1 /* in */ , /* between */ 2]', '{a /* k */ : /* v */ 1}',
      '{\n  // comment\n  a: 1,\n  b: 2,\n}\n', '[[[]]]', '[{}, [], "", \'\']', '"\\"\\\\\\b\\f\\n\\r\\t"', '\'"\'',
      '"\'"', '{_:1}', '{$:1}', '{ä:1}', '{日本:1}', '{a1b2:1}', '[Infinity, NaN, -NaN]', '[+1, -1, +0, -0]', '[1.]',
      '[.5]', '[-.5]', '[1e10, 1E-10, 1e+10]', '{"key": "value"}', '{\'key\': \'value\'}', '[true, false, null]',
      '[\n1,\n2,\n]', '\r\n1\r\n', '"\\u00e9"', '"é"', '"日本"', '"\\x00"', '[0x0, 0xf, 0XF]', '{a:{b:{c:{}}}}', '/**/1',
      '1/**/', '//x\n1', '1//x', '1\n//x', '[1,//x\n2]', '{a:1//x\n}', '{a:1,//x\n}', '"tab\there"',
      '\'multi\\\nline\'', '{"a":1,"b":2,"c":3}', '[1,2,3,4,5,6,7,8,9,10]',
      '{ name: "JSON5", version: 1.0, private: true }',
      '{\n  unquoted: \'and you can quote me on that\',\n  singleQuotes: \'I can use "double quotes" here\',\n  lineBreaks: "Look, Mom! \\\nNo \\\\n\'s!",\n  hexadecimal: 0xdecaf,\n  leadingDecimalPoint: .8675309, andTrailing: 8675309.,\n  positiveSign: +1,\n  trailingComma: \'in objects\', andIn: [\'arrays\',],\n  "backwardsCompatible": "with JSON",\n}\n',
      '"\\\\"', '\'\\\\\'', '""', '\'\'', '{"":1}', '{\'\':1}', '[[1,2],[3,4]]', '[{a:1},{b:2}]', '1e5', '-1e-5',
      '+Infinity', '-0x10', 'NaN', '.0', '0.', '"/"', '"\\/"', '{a:[],b:{}}', '[ /*a*/ ]', '{ /*a*/ }', '[\t]', '{\n}',
      '[\n\n]', '"x\\ty"', '\t\t"a"\t\t', '{A:1,B:2}', '{aB$_c:1}', '[1,\n// c\n2]', '/* multi\n line */ null',
      '{"a":true,"b":false,"c":null}', '-Infinity', '+NaN', '[ "a" , \'b\' ]', '{a : 1 , b : 2 ,}', '[-1.5e+3]',
      '[0.0e-0]', '9', '[99999999999999999999]',
    ],
    divergences: [
      { source: '{\\u0061:1}', reason: JSON5_IDENTIFIER },
      { source: '{a\\u0062:1}', reason: JSON5_IDENTIFIER },
      { source: '{Ⅻ:1}', reason: JSON5_IDENTIFIER },
      { source: '{a\u0301:1}', reason: JSON5_IDENTIFIER },
      { source: '{a‿b:1}', reason: JSON5_IDENTIFIER },
      { source: '{a\u200cb:1}', reason: JSON5_IDENTIFIER },
      { source: '{a\u200db:1}', reason: JSON5_IDENTIFIER },
      { source: '{a٣:1}', reason: JSON5_IDENTIFIER },
      { source: '{aः:1}', reason: JSON5_IDENTIFIER },
      { source: '1\u00a0', reason: JSON5_WHITESPACE },
      { source: '\u00a01', reason: JSON5_WHITESPACE },
      { source: '[1,\u2028 2]', reason: JSON5_WHITESPACE },
      { source: '[1,\u2029 2]', reason: JSON5_WHITESPACE },
      { source: '[1,\ufeff2]', reason: JSON5_WHITESPACE },
      { source: '\u16801', reason: JSON5_WHITESPACE },
      { source: '\u30001', reason: JSON5_WHITESPACE },
      { source: '[1\u2003]', reason: JSON5_WHITESPACE },
      { source: '"\\0"', reason: JSON5_ESCAPE },
      { source: '\'\\0\'', reason: JSON5_ESCAPE },
      { source: '"\\0a"', reason: JSON5_ESCAPE },
      { source: '"\\a"', reason: JSON5_ESCAPE },
      { source: '"\\q"', reason: JSON5_ESCAPE },
      { source: '"\\é"', reason: JSON5_ESCAPE },
      { source: '"\\\rb"', reason: JSON5_ESCAPE },
      { source: '"\\\u2028"', reason: JSON5_ESCAPE },
      { source: '"\\\u2029"', reason: JSON5_ESCAPE },
    ],
    rejections: [
      '', '1 2', '[,]', '{,}', '[1,,]', '{a:1,,}', '[0x]', '[01]', '[1e]', '"\\01"', '"\\x4"', '"\\u12"', '{9a:1}',
      '[nullx]', '[Infinityx]', '"\\1"', '{a}', '{a:}', '[', ']', '{', '"a', '\'a', '/* x', '{a:1 b:2}', '[1 2]',
      'undefined', '[1,]]', '{"a" 1}', '{1:1}', '[.e]', '[--1]', '[0x.1]', '//c', '/**/', '\u0085 1',
    ],
  },
  {
    id: 'scheme',
    language: 'Scheme',
    grammar: 'parity/grammars/native/scheme.lino',
    oracle: 'tree-sitter-scheme 0.24.7',
    sources: [
      'https://small.r7rs.org/attachment/r7rs.pdf',
      'https://docs.rs/crate/tree-sitter-scheme/0.24.7/source/grammar.js',
    ],
    // The oracle has no extras: white space, comment text and string text are
    // tokens it keeps inside their node without a row.
    ...nativeGrammar('native-scheme'),
    matches: [
      '', 'a', ' a', 'a ', '(a b c)', '(define (f x) (+ x 1))', '[a]', '{a}', '()', '#t', '#f', '#true', '#false',
      '#tRuE', '#truex', '#\\a', '#\\space', '#\\spacex', '#\\x41', '#\\newline', '#\\NEWLINE', '#\\(', '#\\ ',
      '"abc"', '"a\\nb"', '"\\x41;"', '"\\x"', '""', '"a\\\\b"', '"a\\"b"', '"a\\\n  b"', '; c', '; c\n', 'a ; c\nb',
      '#| a |#', '#| a #| b |# c |#', '#||#', '#;a', '#; a b', '#;#;a b c', '#!r6rs', '#! r6rs', '#!fold-case a',
      '1', '-1', '+1', '1.5', '.5', '1e5', '1/2', '1+2i', '+i', '-i', '+inf.0', '-nan.0', '+inf.0i', '#e1.5', '#x1F',
      '#b101', '#o17', '#d10', '#x#i1F', '1#', '1#.#', '1.5|53', '1@2', '+1@2', '1abc', '1#a', '.', '...', '1/', '+',
      '-', '->x', '|a b|', 'a|b c|d', '|a\\x41;b|', '|a\\|b|', '#:key', '\'a', '\' a', '`a', ',a', ',@a', ', @a',
      '#\'a', '#`a', '#,a', '#,@a', '#(1 2)', '#vu8(1 2)', '(a . b)', '(quote a)', '`(a ,b ,@c)', '(let ((x 1)) x)',
      'a\tb\nc\rd\fe\vf', 'a\u00a0b', 'a\u2028b', 'a\u2029b', 'λ', '(λ (x) x)', '日本', '#\\λ', '"λ"', '#| x\n y |#',
      '; c\r\n', ';\u2028x\n', '#;(a b) c', '#; ; c\n a b', '#!/usr/bin/env', 'a#t', '#tx', '#\\xyz', '#\\u41',
      '#\\nul', '#\\alarm', '#\\escape', '#\\rubout', '#\\SPACE', '#b', '#x', '#x+i', '#b|5', '#b+', '#e#x', '#b1/',
      '#\\x', '+.5e3', '1s5', '1.e5', '1#.#e5', '#D#E1', '1/2/3', '+INF.0', '1+inf.0i', '1@-2', '1#/2#', '#i1/2',
      '1.5e-3', '-0.0', '1.5f2', '1l2', '#e1e10', '1.5|53e2', '+nan.0i', '1-i', '1+i', '1-2.5i', '#x-ff', '#XFF',
      '#B1', '#O7', '#I1', '#E1', '1|2', '1.|2', '.5|2', '1+1/2i', '1@+inf.0', '1#e5', '#x1.5', '#b2', '#o8', '+-1',
      '1..', '1.2.3', '1e', '1e+', 'e5', '1e5x', '#\\x1F600', '#\\X41', '#\\U41', '#\\u', '#\\delete', '#\\vtab',
      '#\\page', '#\\linefeed', '#\\esc', '#\\backspace', '#\\tab', '#\\return', '#\\null', '#\\bel', '#\\ls',
      '#\\nel', '#\\vt', '#\\Space', '#\\nEwLiNe', '#\\Tab', '#\\\n', '#\\\t', '#\\"', '#\\#', '#\\|', '#\\\\',
      '"\\a\\b\\t\\n\\r\\v\\f"', '"\\X41;"', '"\\x41"', '"\\ \n "', '"\\\t\n\t"', '"\\\r\n"', '"\\\r"', '"\\\u2028"',
      '"\\\u0085"', '"\\\u2029"', '"\\  \n"', '"\\ x"', '"\\q"', '"\\λ"', '"a\nb"', '"\\\r\u0085"', '#| | # |#',
      '#|||#', '#| a |##| b |#', '#|\n|#', '; a\n; b', ';', ';\n', '#;  #| c |# a', '#! #| c |# r6rs', '#!\nx',
      '#;\n1 2', '|a|', '||', '|\\x41;|', '|\\t|', '|a\nb|', '#:|a b|', '#:a#b', 'a#b', 'a.b', '#(a #(b))', '#vu8()',
      '( a )', '( )', '(\n)', '(a\tb)', '[(a)]', '{[()]}', '\'()', '\'#(1)', '`#(1 ,a)', '#\'(a)', '#`(a #,b #,@c)',
      ',\'a', '\'\'a', '\' ; c\n a', '\'#;b a', '\'#|c|#a', '#t#f', '#t(a)', 'a"b"', '"a"b', '1"a"', 'a(b)c', 'a\'b',
      'a`b', 'a,b', '#f5', '#fa', '#falsey', '#FALSE',
    ],
    divergences: [
      { source: '#u8()', reason: SCHEME_BYTEVECTOR },
      { source: '#u8(1 2 255)', reason: SCHEME_BYTEVECTOR },
      { source: '(#u8(0))', reason: SCHEME_BYTEVECTOR },
      { source: '#u8( 1 )', reason: SCHEME_BYTEVECTOR },
      { source: '#u8(#u8())', reason: SCHEME_BYTEVECTOR },
      { source: '#u8(a)', reason: SCHEME_BYTEVECTOR },
      { source: '#u8(1 #u8(2))', reason: SCHEME_BYTEVECTOR },
      { source: '[#u8(1)]', reason: SCHEME_BYTEVECTOR },
      { source: '#u8(; c\n1)', reason: SCHEME_BYTEVECTOR },
      { source: '#u8(#;1 2)', reason: SCHEME_BYTEVECTOR },
      { source: '#0=a', reason: SCHEME_DATUM_LABEL },
      { source: '#0#', reason: SCHEME_DATUM_LABEL },
      { source: '#12=(a . #12#)', reason: SCHEME_DATUM_LABEL },
      { source: '(#1=x #1#)', reason: SCHEME_DATUM_LABEL },
      { source: '#0= a', reason: SCHEME_DATUM_LABEL },
      { source: '#0=#;c a', reason: SCHEME_DATUM_LABEL },
      { source: '\'#0#', reason: SCHEME_DATUM_LABEL },
      { source: '#0=#1=a', reason: SCHEME_DATUM_LABEL },
      { source: '(#0# #0#)', reason: SCHEME_DATUM_LABEL },
      { source: '#123#', reason: SCHEME_DATUM_LABEL },
      { source: '#0=(#0#)', reason: SCHEME_DATUM_LABEL },
      { source: '`#0#', reason: SCHEME_DATUM_LABEL },
      { source: '#0=#| c |#a', reason: SCHEME_DATUM_LABEL },
    ],
    rejections: [
      ')', '(', ']', '}', '(]', '[)', '{)', '(a', 'a)', '"a', '"', '"\\', '#|', '#| a', '#|#|a|#', '|#', '#', '#\\',
      '#e', '#q', '#:', '#;', '#!', '\'', '`', ',', ',@', '#\'', '#`', '#,', '#,@', '#(', '#vu8(', '#vu8', '#U8(1)',
      '#0=', '#0', '#u8', '|a', 'a|b', '#;)', '\' )', '#u8 (1)', '#vu8 (1)', '#t#', '(a))', '((a)', '#|#|#||#|#',
      '#\\x41 )', '#e#',
    ],
  },
  {
    id: 'racket',
    language: 'Racket',
    grammar: 'parity/grammars/native/racket.lino',
    oracle: 'tree-sitter-racket 0.25.0',
    sources: [
      'https://docs.racket-lang.org/reference/reader.html',
      'https://docs.rs/crate/tree-sitter-racket/0.25.0/source/grammar.js',
    ],
    // The runtime skips a leading byte order mark before the oracle root
    // starts. White space, comment text, string text and the here string
    // lines are tokens the oracle keeps inside their node without a row.
    ...nativeGrammar('native-racket'),
    matches: [
      '', 'a', ' a', 'a ', '(a b c)', '(define (f x) (+ x 1))', '[a]', '{a}', '()', '(a . b)', '(.)', '(. a)', '(.a)',
      '.', '...', '(a ... b)', '(1 . 2)', '(a .b)', '(a. b)', '( . )', '#t', '#f', '#true', '#false', '#T', '#F',
      '#TRUE', '#fals', '#tx', '#f(1)', '#t(1)', '#fl(1)', '#fx(1)', '#fl3(1)', '#fx2[1 2]', '#(1 2)', '#3(1)', '#[a]',
      '#{a}', '#12(1)', '#123456789(1)', '"abc"', '""', '"a\\nb"', '"\\a\\b\\t\\n\\v\\f\\r\\e"', '"\\"\\\'\\\\"',
      '"\\1"', '"\\12"', '"\\123"', '"\\1234"', '"\\x4"', '"\\x41"', '"\\x414"', '"\\u41"', '"\\u0041"', '"\\u00411"',
      '"\\U41"', '"\\U0001F600"', '"\\U000000410"', '"a\\\nb"', '"a\\\r\nb"', '"a\\\rb"', '"a\nb"', '"λ"', '#"abc"',
      '#""', '#"\\n"', '#rx"a"', '#px"a+"', '#rx#"a"', '#px#"a"', '#rx"\\\\d"', '#<<EOF\nabc\nEOF',
      '#<<EOF\nabc\nEOF\n', '#<<EOF\nEOF', '#<<\n\n', '#<<\nabc\n', '#<< EOF\nx\n EOF', '#<<EOF\r\nEOF\r',
      '(#<<A\nx\nA\n)', '#<<A\n\nx\ny z\nA', '#<<EOF\nab\ncd\nEOF\n(a)', '#<<A\nB\nA\n#<<B\nA\nB', '#\\a', '#\\space',
      '#\\nul', '#\\null', '#\\nulx', '#\\backspace', '#\\tab', '#\\newline', '#\\linefeed', '#\\vtab', '#\\page',
      '#\\return', '#\\rubout', '#\\101', '#\\10', '#\\1011', '#\\u41', '#\\u0041', '#\\u00411', '#\\U41',
      '#\\U0001F600', '#\\λ', '#\\(', '#\\ ', '#\\\r', '#\\\t', '#\\nx', '#\\uz', '#\\Space', '#\\x41', '1', '-1', '+1',
      '1.5', '.5', '1.', '1e5', '1/2', '1+2i', '+i', '-i', '+inf.0', '-nan.0', '+inf.f', '+inf.t', '1.5t3', '#e1.5',
      '#i1', '#x1F', '#b101', '#o17', '#d10', '#x#i1F', '#e#x10', '1#', '1#.#', '1@2', '1abc', '1#a', '#e1x', '#x1.5',
      '#xff/a', '1/2/3', '+inf.0i', '1+inf.0i', '1-2.5i', '1e-3', '1s5', '1l5', '1d5', '1f5', '#x1s2', '1+i', '+1/2i',
      '1#/2#', '1.5e+3', '+-1', '1..', '1e', 'e5', '-', '+', '->x', '1+', 'inf.0', '+INF.0', '+nan.F', '#i+inf.0',
      '#x+inf.t', '#d#e1', '#E1', '#I1', '#B1', '#O1', '#X1', '#D1', 'abc', 'a-b', 'λ', '日本', '|a b|', 'a|b c|d',
      '|a\nb|', 'a\\ b', '\\a', '#%app', '#%', '#ci', '#cs', '#CI', '#cix', 'a#b', 'a.b', '+a', '1a', 'a1', '\ufeffa',
      'a\u0085b', 'a\u00a0b', 'a\u2028b', 'a\u3000b', '#:key', '#:', '#:a#b', '#:|a b|', '#&a', '#& a', '#&#&a', '#0=a',
      '#0#', '#12=(a . #12#)', '#0= a', '#12345678#', '#s(a 1)', '#s[a]', '#hash()', '#hash((a . 1))', '#hasheq()',
      '#hasheqv()', '#hashalw()', '#HASH()', '#HashEq()', '\'a', '\' a', '`a', ',a', ',@a', ', @a', '#\'a', '#`a',
      '#,a', '#,@a', '#, @a', '\'()', '`(a ,b ,@c)', '\'#;b a', '\' ; c\n a', '\'#|c|#a', '; c', '; c\n', 'a ; c\nb',
      ';', '; c\r\n', '; c\u2028x', '#| a |#', '#| a #| b |# c |#', '#||#', '#|||#', '#| | # |#', '#|\n|#', '#;a',
      '#; a b', '#;#;a b c', '#;(a b) c', '#!racket', '#lang racket', '#lang racket/base', '#lang racket/', '#lang a',
      '#!/usr/bin/env racket', '#! x', '#!/x\\\ny', '#!/x\\\n', '#reader x', '#reader(a)', '#readerx',
      '#lang racket\n(define x 1)', 'a"b"', '"a"b', '1"a"', 'a(b)c', 'a\'b', 'a`b', 'a,b', '#t#f', 'a;b', 'a#|b|#',
      '(a)(b)', '((a))', '[(a)]', '{[()]}', '(a\tb)', '(\n)', '( a )', '(a . )', '#ci Apple', '#CI a', '#cs a',
      '#ci#cs Apple', '#ci(a B)', '#cI a', '#Cs (a)', '#[1 2]', '#{1}', '#fl[1.0]', '#s[a 1]', '#! comment\n1',
      '#!/bin/racket\n1', '#hash[(a . 1)]', '#fl3(1.0)', '#Fl(1.0)', '#cia', '#csx',
    ],
    divergences: [
      { source: '#\\\n', reason: RACKET_CHARACTER_NEWLINE },
      { source: '(#\\\n)', reason: RACKET_CHARACTER_NEWLINE },
      { source: '#\\\n a', reason: RACKET_CHARACTER_NEWLINE },
      { source: '\'#\\\n', reason: RACKET_CHARACTER_NEWLINE },
      { source: '[#\\\n]', reason: RACKET_CHARACTER_NEWLINE },
      { source: '#(#\\\n)', reason: RACKET_CHARACTER_NEWLINE },
      { source: '(a #\\\n b)', reason: RACKET_CHARACTER_NEWLINE },
      { source: '#\\\n;c', reason: RACKET_CHARACTER_NEWLINE },
      { source: '#&#\\\n', reason: RACKET_CHARACTER_NEWLINE },
      { source: '#;#\\\n a', reason: RACKET_CHARACTER_NEWLINE },
      { source: '#\\\n#\\\n', reason: RACKET_CHARACTER_NEWLINE },
      { source: 'a\\\nb', reason: RACKET_SYMBOL_NEWLINE },
      { source: '\\\n', reason: RACKET_SYMBOL_NEWLINE },
      { source: '(a\\\n)', reason: RACKET_SYMBOL_NEWLINE },
      { source: '\\\na', reason: RACKET_SYMBOL_NEWLINE },
      { source: 'a\\\n\\\nb', reason: RACKET_SYMBOL_NEWLINE },
      { source: '|a|\\\n', reason: RACKET_SYMBOL_NEWLINE },
      { source: '#:a\\\nb', reason: RACKET_SYMBOL_NEWLINE },
      { source: '#%a\\\n', reason: RACKET_SYMBOL_NEWLINE },
      { source: '\'a\\\n', reason: RACKET_SYMBOL_NEWLINE },
      { source: '`(a\\\nb)', reason: RACKET_SYMBOL_NEWLINE },
      { source: 'a\\\n|b|', reason: RACKET_SYMBOL_NEWLINE },
    ],
    rejections: [
      '#b2', '#o8', ')', '(', ']', '}', '(]', '[)', '(a', 'a)', '"a', '"', '"\\', '"\\q"', '#|', '#| a', '|#', '#',
      '#\\', '#e', '#q', '#;', '\'', '`', ',', ',@', '#\'', '#`', '#,', '#,@', '#(', '#fl', '#flz', '#fx', '#s', '#sa',
      '#hash', '#hashe()', '#0=', '#<<', '#<<EOF', '#<<EOF\n', '#<<EOF\nabc', '|a', 'a\\', '#lang', '#lang  racket',
      '#lang /a', '#!', '#!(a)', '#reader', '#&', '#rx', '#rx a', '#123456789#', '#u8(1)', '#x', '#b', '#<a', '#%(',
    ],
  },
  {
    id: 'c',
    language: 'C',
    grammar: 'parity/grammars/native/c.lino',
    oracle: 'tree-sitter-c 0.24.2',
    sources: [
      `${grammarSourceOf('native-c').repository}/blob/${grammarSourceOf('native-c').revision}/src/grammar.json`,
      `${grammarSourceOf('native-c').repository}/tree/${grammarSourceOf('native-c').revision}/test/corpus`,
    ],
    // The grammar is the import of the pinned tree-sitter-c grammar.json
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision and a few sources more. A
    // keyword is lexed where a statement or declaration may start, so the
    // identifier `typedef` stands only where no keyword may come.
    ...nativeGrammar('native-c'),
    matches: [
      ...upstreamCorpus('native-c'),
      '', 'int x;', 'x = typedef;', 'int typedef;', 'int main(void) { return 0; }', 'struct s { int a; } v;',
      '#include <stdio.h>\n', '#define N 1\nint a[N];\n', '/* c */ int x; // d\n', 'char *s = "a\\n" L"b";',
      'int f(int a, ...);', 'x = a ? b : c;', 'if (a) b; else c;', 'for (;;) {}', 'do x++; while (x < 3);',
    ],
    divergences: [],
    rejections: [
      'int x', 'int x = ;', '{', '}', 'int f() {', ')', '(', '"abc', "'a", '/* abc', 'int x = 1 +;', 'if (x',
      'struct {', '#include', '#define', 'int [;', 'x = = 1;', 'return', 'int f(int a,) {}', 'a[;', '#if X', '#endif',
      'for (;;', 'int main() { return 0 }', 'x->;', 'L"a', 'typedef;', 'if;', 'struct;', 'while;',
      // tree-sitter's `\s` is ASCII white space: a no-break or ideographic space is an error.
      'int a;\u00a0int b;', '\u3000int x;', 'int x;\ufeff',
    ],
  },
  {
    id: 'rust',
    language: 'Rust',
    grammar: 'parity/grammars/native/rust.lino',
    oracle: 'tree-sitter-rust 0.24.2',
    sources: [
      `${grammarSourceOf('native-rust').repository}/blob/${grammarSourceOf('native-rust').revision}/src/grammar.json`,
      `${grammarSourceOf('native-rust').repository}/tree/${grammarSourceOf('native-rust').revision}/test/corpus`,
    ],
    // The grammar is the import of the pinned tree-sitter-rust grammar.json
    // with its native scanner (js/scripts/import-native-grammars.mjs); the
    // matches are every case of its upstream corpus at the same revision but
    // those of error.txt, which the oracle recovers from and the native
    // grammar rejects, and a few sources more.
    ...nativeGrammar('native-rust'),
    matches: [
      ...upstreamCorpus('native-rust', (file) => file !== 'error.txt'),
      '', 'fn main() {}', 'let x = 1;', 'struct S { a: i32 }', '// c\nfn f() {}\n', '/* a /* b */ c */ fn f() {}',
      'fn f() -> i32 { 1 + 2 * 3 }', 'const R: &str = r#"a"#;', 'x.await?;', '//! i\n/// d\nfn f() {}\n',
      'fn f() { m!(x); }', 'm!(a +.);',
      // Shifts and reductions an LR parser settles by precedence: a closure
      // over an or-pattern, and bare, prefix and postfix ranges in a row.
      'fn f() { |(|a|b)| c; }', 'fn f() { a ..= ..; }', 'fn f() { .. ..=.. ..; }', 'fn f() { ..=.. ..=..; }',
      'fn f() { ..=..=.. .. ..=.. }',
      // Shifts LR parsing takes where two parses part deep in shared nodes (a
      // closure argument, a range after a binary operator), and a let chain
      // whose silent rule ends a node at its precedence.
      'fn f() { g(|| a, |p| p) }', 'fn f() { a + b..*c; }', 'fn f(){ if let A = b && !c && d {} }',
      // A keyword a parse tried where the tree has no node in progress (a
      // quote in a macro's token tree read as a char literal) does not make
      // the comment lexed there an error (formal-ai's matches!('"')).
      'fn a(){m!(\'"\')} //"\ntype A = _;',
      // A `break` before a token that cannot begin its value but follows an
      // expression is reduced there, as an LR parser reduces on a lookahead
      // its left-corner context follows; a block or a label is shifted.
      'fn f(){ loop { break -1; } }', 'fn f(){ loop { break {1}; } }', "fn f(){ 'a: loop { break 'a; } }",
    ],
    divergences: [],
    rejections: [
      ...upstreamCorpus('native-rust', (file) => file === 'error.txt'),
      'fn', 'fn f(', 'fn f() {', '}', '{', 'let x = ;', 'struct S {', '"abc', "'a", '/* abc', 'fn f() { 1 + }', 'if x {',
      'impl {', 'use ;', 'mod', 'x = = 1;', 'fn f(a,,) {}', 'a[;', 'match x {', 'enum E { A,, }', 'fn main() { let }',
      'trait T {', 'r#"abc', 'b"abc', 'x.;', 'let x: = 1;', 'pub', '#[derive(', 'macro_rules! m {', 'where',
      'fn f() -> {}', '1 +',
      // tree-sitter's `\s` is ASCII white space: a no-break or ideographic space is an error.
      'fn a() {}\u00a0fn b() {}', '\u3000fn f() {}', 'fn f() {}\u2028',
    ],
  },
  {
    id: 'javascript',
    language: 'JavaScript',
    grammar: 'parity/grammars/native/javascript.lino',
    oracle: 'tree-sitter-javascript 0.25.0',
    sources: [
      `${grammarSourceOf('native-javascript').repository}/blob/${grammarSourceOf('native-javascript').revision}/src/grammar.json`,
      `${grammarSourceOf('native-javascript').repository}/tree/${grammarSourceOf('native-javascript').revision}/test/corpus`,
    ],
    // The grammar is the import of the pinned tree-sitter-javascript
    // grammar.json with its native scanner (js/scripts/import-native-grammars.mjs);
    // the matches are every case of its upstream corpus at the same revision
    // but the one the oracle recovers from, and a few sources more.
    ...nativeGrammar('native-javascript'),
    matches: [
      ...upstreamCorpus('native-javascript', (_file, title) => title !== JAVASCRIPT_ORACLE_ERROR),
      '', 'x;', 'let x = 1;', 'const f = (a, b) => a + b;', 'function f(a, ...b) { return a; }',
      'class A extends B { #x = 1; static m() {} }', 'import { a as b } from "c";', 'export default function () {}', 'a?.b ?? c;',
      'x = `a${b}c`;', 'async function f() { await g(); }', 'for (const [k, v] of m) {}', 'label: while (true) break label;',
      'x = /ab+c/gi;', 'let { a, ...rest } = o;', 'x = a ? b : c;', '// c\nlet x = 1 /* d */;\n', 'a\nb\n', '<!-- c\nx;',
      'x = <div className="a">{b}</div>;', 'function* g() { yield* h(); }', 'try { a(); } catch { b(); } finally { c(); }',
      'x = 0x1fn + 1_000 + .5e3;', 'switch (x) { case 1: break; default: }', 'new.target;', 'export', 'f(a,,);',
      // ECMA-262 section 12.2 (White Space) and 12.3 (Line Terminators): a
      // no-break or ideographic space, the byte order mark and a line separator
      // separate tokens, as tree-sitter-javascript's extras read them.
      'let x = 1;\u3000let y;', 'let a;\u00a0let b;', 'x;\u2028y;', 'x;\ufeff',
    ],
    divergences: [],
    rejections: [
      ...upstreamCorpus('native-javascript', (_file, title) => title === JAVASCRIPT_ORACLE_ERROR),
      'const = 1;', 'if (ready { go(); }', 'function', 'function f(', 'function f() {', '}', '{', 'let x = ;', 'class A {', '"abc',
      "'a", '/* abc', 'x = 1 +;', 'if (x', 'import {', 'x = = 1;', 'a[;', 'switch (x) {', '`abc', 'x.;', 'for (;;', 'new',
      'x = <div>;',
    ],
  },
  {
    id: 'typescript',
    language: 'TypeScript',
    grammar: 'parity/grammars/native/typescript.lino',
    oracle: 'tree-sitter-typescript 0.23.2',
    sources: typescriptSources('native-typescript'),
    // The grammar is the import of the pinned tree-sitter-typescript
    // typescript/src/grammar.json with its native scanner
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision, and a few sources more.
    ...nativeGrammar('native-typescript'),
    matches: [
      ...upstreamCorpus('native-typescript'), ...TYPESCRIPT_MATCHES,
      // A type assertion and an arrow function with type parameters, which
      // TSX reads as JSX.
      'let x = <string>y;', 'const f = <T>(a: T) => a;', 'x = a < b > c;',
    ],
    divergences: [],
    rejections: TYPESCRIPT_REJECTIONS,
  },
  {
    id: 'tsx',
    language: 'TSX',
    grammar: 'parity/grammars/native/tsx.lino',
    oracle: 'tree-sitter-typescript 0.23.2 (tsx)',
    sources: typescriptSources('native-tsx'),
    // The grammar is the import of the pinned tree-sitter-typescript
    // tsx/src/grammar.json with its native scanner; the matches are every case
    // of the upstream corpus at the same revision the TSX oracle runs, and a
    // few sources more.
    ...nativeGrammar('native-tsx'),
    matches: [
      ...upstreamCorpus('native-tsx'), ...TYPESCRIPT_MATCHES,
      'x = <div className="a">{b}</div>;', 'const C = <T,>(a: T) => <span>{a}</span>;', 'x = <A.B c={1} {...d} />;', 'x = <></>;', 'x = <a></b>;',
    ],
    divergences: [],
    rejections: [...TYPESCRIPT_REJECTIONS, 'x = <div>;'],
  },
  {
    id: 'lean',
    language: 'Lean',
    grammar: 'parity/grammars/native/lean.lino',
    oracle: 'tree-sitter-lean4 0.3.0',
    sources: [
      `${grammarSourceOf('native-lean').repository}/blob/${grammarSourceOf('native-lean').revision}/${grammarSourceOf('native-lean').path}`,
      `${grammarSourceOf('native-lean').repository}/tree/${grammarSourceOf('native-lean').revision}/${grammarSourceOf('native-lean').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json the pinned tree-sitter-lean
    // grammar.js generates, with its native scanner
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision but those the oracle recovers
    // from, which the native grammar rejects, and a few sources more.
    ...nativeGrammar('native-lean'),
    matches: [
      ...leanCorpus(false),
      '', 'def x := 1\n', 'def f (x : Nat) : Nat := x + 1\n', 'theorem t : 1 = 1 := rfl\n', '#eval 1 + 2\n', '#check Nat\n',
      '-- c\ndef x := 1 /- d -/\n', '/-- doc -/\ndef x := 1\n', 'namespace N\ndef x := 1\nend N\n', 'open Nat\n', 'import Mathlib\n',
      'structure P where\n  x : Nat\n  y : Nat\n', 'inductive T where\n  | a\n  | b : Nat → T\n', 'def f : Nat → Nat\n  | 0 => 1\n  | n + 1 => n\n',
      'example : True := by\n  trivial\n', 'def s := "a\\nb"\n', "def c := 'a'\n", 'instance : Inhabited Nat := ⟨0⟩\n',
      'variable {α : Type} (x : α)\n', 'def f := fun x => x\n', 'def f := λ x => x\n', 'def g := do\n  let x ← pure 1\n  return x\n',
      'section\nvariable (n : Nat)\nend\n', 'def l := [1, 2, 3]\n', 'def t := (1, 2)\n',
      'def f (x : Nat) : Nat :=\n  match x with\n  | 0 => 0\n  | _ => 1\n', 'abbrev N := Nat\n', '@[simp] theorem t : 1 = 1 := rfl\n',
      'def x := if true then 1 else 2\n', 'universe u\n',
      // A dotted option name: its silent rule of level 0 right shifts on over
      // the `.` where a projection of level 90 may reduce `pp` first.
      'set_option pp.all true\n',
      // A `!` after a subscript is the subscript's own where an expression
      // follows, and an application whose argument is reduced alone where the
      // other parse shifts its first token is projected (`f a.b`).
      'def f := x[i]! + 1\n', 'example := by\n  apply f a.b\n',
    ],
    divergences: [
      {
        source: upstreamCorpus('native-lean', (_file, title) => title === LEAN_EXPLICIT_FUNCTION)[0],
        reason: LEAN_EXPLICIT_FUNCTION_REASON,
      },
    ],
    rejections: [
      ...leanCorpus(true),
      'def f :=', 'def', 'theorem t : := rfl\n', 'structure P where\n  x :\n', 'def f (x : Nat := x\n', '#eval (1 +\n', 'def s := "abc\n',
      '/- abc\n', 'def x := [1, 2\n', 'namespace\n', 'inductive\n',
      // The command ends before `!`, which a subscript reduces first.
      '#eval x[i]!\n',
    ],
  },
  {
    id: 'rocq',
    language: 'Rocq',
    grammar: 'parity/grammars/native/rocq.lino',
    oracle: 'tree-sitter-rocq 300fe33',
    sources: [
      `${grammarSourceOf('native-rocq').repository}/blob/${grammarSourceOf('native-rocq').revision}/grammar.js`,
      `${grammarSourceOf('native-rocq').repository}/tree/${grammarSourceOf('native-rocq').revision}/${grammarSourceOf('native-rocq').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json the pinned and patched
    // tree-sitter-rocq grammar.js generates, the one the oracle parser is
    // generated from (js/scripts/import-native-grammars.mjs); the matches are
    // every case of its upstream corpus at the same revision but the one the
    // oracle recovers from, which the native grammar rejects, and a few
    // sources more.
    ...nativeGrammar('native-rocq'),
    matches: [
      ...rocqCorpus(false),
      '', 'Definition x := 1.\n', 'Definition f (x : nat) : nat := x + 1.\n', 'Theorem t : 1 = 1.\nProof. reflexivity. Qed.\n',
      'Require Import Arith.\n', 'Check nat.\n', 'Compute 1 + 2.\n', '(* c *)\nDefinition x := 1. (* d *)\n', 'Module M.\nDefinition x := 1.\nEnd M.\n',
      'Inductive t : Type := a | b : nat -> t.\n', 'Fixpoint f (n : nat) : nat := match n with | O => 1 | S m => f m end.\n', 'Definition s := "a".\n',
      'Section S.\nVariable n : nat.\nEnd S.\n', 'Definition f := fun x => x.\n',
      'Lemma l : forall n : nat, n = n.\nProof.\n  intros n.\n  destruct n; reflexivity.\nQed.\n', 'Record P := { x : nat; y : nat }.\n',
      'Notation "x ++ y" := (app x y).\n', 'Ltac t := auto.\n', 'Definition p := (1, 2).\n', 'Definition x := if true then 1 else 2.\n',
      'Definition x := let y := 1 in y.\n', 'Open Scope nat_scope.\n', 'Set Implicit Arguments.\n', '#[local] Definition x := 1.\n',
      'Definition l := [1; 2].\n', 'Example e : 1 + 1 = 2.\nProof. simpl. reflexivity. Qed.\n', 'Axiom a : nat.\n', 'Fail Check x.\n',
      'Goal True.\nProof.\n  - exact I.\nQed.\n', 'Definition x := @id nat 1.\n', 'Definition f {A : Type} (x : A) := x.\n',
    ],
    divergences: [],
    rejections: [
      ...rocqCorpus(true),
      'Definition f :=', 'Definition', 'Definition x := 1', 'Definition f (x : nat := x.\n', 'Compute (1 +.\n', 'Definition s := "abc.\n',
      '(* abc\n', 'Module.\n', 'Inductive.\n', 'Definition x := [1; 2.\n', 'Theorem t : := I.\n',
    ],
  },
  {
    id: 'java',
    language: 'Java',
    grammar: 'parity/grammars/native/java.lino',
    oracle: 'tree-sitter-java 0.23.5',
    sources: [
      `${grammarSourceOf('native-java').repository}/blob/${grammarSourceOf('native-java').revision}/grammar.js`,
      `${grammarSourceOf('native-java').repository}/tree/${grammarSourceOf('native-java').revision}/${grammarSourceOf('native-java').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-java
    // 0.23.5, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more; the rejections are the cases the oracle recovers from,
    // none at this revision, and a few sources more.
    ...nativeGrammar('native-java'),
    matches: [
      ...javaCorpus(false),
      '', 'class A {}\n', 'public class A { int x = 1; }\n', 'interface I { void f(); }\n', 'enum E { A, B }\n',
      'record P(int x, int y) {}\n', 'package a.b;\nimport java.util.*;\n', 'class A { void f() { return; } }\n',
      'class A { int f(int x) { return x + 1; } }\n', '// c\nclass A {} /* d */\n', '/** Doc. */\nclass A {}\n',
      'class A { void f() { if (x) { y(); } else { z(); } } }\n', 'class A { void f() { for (int i = 0; i < n; i++) {} } }\n',
      'class A { void f() { for (String s : list) {} } }\n', 'class A { void f() { while (true) break; } }\n',
      'class A { void f() { try { g(); } catch (E e) {} finally {} } }\n', 'class A { void f() { switch (x) { case 1: break; default: } } }\n',
      'class A { int f(int x) { return switch (x) { case 1 -> 2; default -> 3; }; } }\n', 'class A<T extends B> { T t; }\n',
      'class A { List<String> l = new ArrayList<>(); }\n', 'class A { int[] a = {1, 2}; }\n', 'class A { Runnable r = () -> {}; }\n',
      'class A { Function<A, B> f = x -> x; }\n', 'class A { void f() { a = b::m; } }\n', 'class A { void f() { A<B> c; } }\n',
      '@A(v = 1) class C {}\n', '@Override\nclass A {}\n', 'class A { String s = "a" + \'b\'; }\n', 'class A { long x = 0x1FL; double d = 1.5e3; }\n',
      'class A { boolean b = x instanceof String s; }\n', 'class A { Object o = (String) x; }\n', 'class A { int x = a ? b : c; }\n',
      'class A { String s = """\n  text\n  """; }\n', 'class A { void f() throws E { throw new E(); } }\n', 'class A { static { x = 1; } }\n',
      'class A { A() { super(); } }\n', 'class A { void f() { synchronized (this) {} } }\n', 'module m { requires a; exports b; }\n',
    ],
    divergences: [],
    rejections: [
      ...javaCorpus(true),
      'class', 'class A {', 'class A { int x = ; }\n', 'class A { void f() { f(1, ); } }\n', 'class A { String s = "abc; }\n',
      '/* abc\nclass A {}\n', 'class { }\n', 'class A { void f( }\n', 'class A { int[] a = {1, 2; }\n', 'class A { void f() { if x {} } }\n',
      'import ;\n',
    ],
  },
  {
    id: 'go',
    language: 'Go',
    grammar: 'parity/grammars/native/go.lino',
    oracle: 'tree-sitter-go 0.25.0',
    sources: [
      `${grammarSourceOf('native-go').repository}/blob/${grammarSourceOf('native-go').revision}/grammar.js`,
      `${grammarSourceOf('native-go').repository}/tree/${grammarSourceOf('native-go').revision}/${grammarSourceOf('native-go').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-go
    // 0.25.0, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more; the rejections are the cases the oracle recovers from,
    // and a few sources more.
    ...nativeGrammar('native-go'),
    matches: [
      ...goCorpus(false),
      '', 'package main\n', 'package main\n\nimport "fmt"\n', 'package main\n\nimport (\n\t"fmt"\n\tos "os"\n)\n',
      'package main\n\nfunc main() {}\n', 'package main\n\nfunc f(x int, y string) (int, error) { return x, nil }\n',
      'package main\n\nvar x = 1\n', 'package main\n\nconst (\n\tA = iota\n\tB\n)\n', 'package main\n\ntype P struct {\n\tX, Y int\n}\n',
      'package main\n\ntype I interface {\n\tM() int\n}\n', 'package main\n\ntype T[K comparable, V any] map[K]V\n',
      'package main\n\nfunc f() {\n\tx := []int{1, 2}\n\t_ = x[0:1]\n}\n', 'package main\n\nfunc f() {\n\tfor i := 0; i < n; i++ {\n\t}\n}\n',
      'package main\n\nfunc f() {\n\tfor k, v := range m {\n\t\t_, _ = k, v\n\t}\n}\n', 'package main\n\nfunc f() {\n\tif x > 0 {\n\t} else if y {\n\t} else {\n\t}\n}\n',
      'package main\n\nfunc f() {\n\tswitch x {\n\tcase 1, 2:\n\tdefault:\n\t}\n}\n', 'package main\n\nfunc f() {\n\tswitch v := x.(type) {\n\tcase int:\n\t\t_ = v\n\t}\n}\n',
      'package main\n\nfunc f() {\n\tselect {\n\tcase v := <-c:\n\t\t_ = v\n\tdefault:\n\t}\n}\n', 'package main\n\nfunc f() {\n\tgo g()\n\tdefer h()\n}\n',
      'package main\n\nfunc f() {\n\tc <- 1\n\tx := <-c\n\t_ = x\n}\n', 'package main\n\nfunc (p *P) M() int { return p.X }\n',
      'package main\n\nvar f = func(x int) int { return x * 2 }\n', 'package main\n\nvar s = `raw\nstring`\n', 'package main\n\nvar r = \'é\'\n',
      'package main\n\nvar n = 0x1F + 1.5e3 + 2i\n', 'package main\n\nvar m = map[string]int{"a": 1}\n', 'package main\n\nvar p = &P{X: 1}\n',
      'package main\n\n// c\nfunc f() {} /* d */\n', 'package main\n\nfunc f[T any](x T) T { return x }\n', 'package main\n\nvar x = f[int](1)\n',
      'package main\n\nfunc f() {\nL:\n\tfor {\n\t\tbreak L\n\t}\n}\n', 'package main\n\nfunc f() {\n\tgoto L\nL:\n}\n',
      'package main\n\nfunc f(xs ...int) { f(xs...) }\n', 'package main\n\nvar c = make(chan<- int)\n', 'package main\n\ntype A = B\n',
      'package main\n\nfunc f() {\n\tx++\n\ty -= 2\n}\n', 'package main\n\nvar b = !a && c || d\n',
      'package main\n\nvar c chan<- chan int\n', 'package main\n\nvar x = <-chan int(c)\n', 'package main\n\nvar x = a[b](c)\n',
      'package main\n\nfunc main() {\n\tx := a\n\ty := b\n}\n',
    ],
    divergences: [],
    rejections: [
      ...goCorpus(true),
      'package', 'package main\nfunc f() {', 'package main\nvar x = \n', 'package main\nfunc f() { g(1, }\n', 'package main\nvar s = "abc\n',
      'package main\n/* abc\n', 'package main\nimport (\n', 'package main\nvar a = []int{1, 2\n', 'package main\nfunc f() { if {} }\n',
      'package main\ntype struct {}\n', 'package main\ntype P struct { X int\n',
    ],
  },
  {
    id: 'regex',
    language: 'Regex',
    grammar: 'parity/grammars/native/regex.lino',
    oracle: 'tree-sitter-regex 0.25.0',
    sources: [
      `${grammarSourceOf('native-regex').repository}/blob/${grammarSourceOf('native-regex').revision}/grammar.js`,
      `${grammarSourceOf('native-regex').repository}/tree/${grammarSourceOf('native-regex').revision}/${grammarSourceOf('native-regex').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-regex
    // 0.25.0, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more; the rejections are sources the oracle recovers from.
    ...nativeGrammar('native-regex'),
    matches: [
      ...regexCorpus(false),
      'a', 'abc', 'a|b|c', '^a$', '\\bword\\B', 'a*b+c?', 'a*?b+?c??', 'a{2}', 'a{2,}', 'a{2,5}?', 'a{,5}', '[abc]', '[^a-z0-9_]',
      '[\\d\\s]', '[-a]', '[a-]', '(a)(b)', '(?:a|b)', '(?<year>\\d{4})-\\k<year>', '(?P<n>x)', '(?=a)', '(?!a)', '(?<=a)', '(?<!a)',
      '\\1', '\\cA', '\\n\\t', '\\u00e9', '\\u{1F600}', '\\p{L}', '\\P{Script=Greek}', '[[:alpha:]]', '(?i)abc', '(?i-m:abc)', '(?-s)x',
      '.', 'caf[eé]', '^(?<word>[a-zé]+)\\s*(\\d{2,4})?$|caf[eé]', 'a\nb', 'x\\.y', '\\/', '{1}', 'a{2}{3}', '[]', '[^]',
    ],
    divergences: [
      { source: 'a{', reason: REGEX_LITERAL_BRACKET },
      { source: '[[:alpha:]', reason: REGEX_LITERAL_BRACKET },
    ],
    rejections: [
      ...regexCorpus(true),
      '', '(ab[c', '(', ')', '[', ']', '(?', '(?<', '(?<a', '(?:', 'a|(', '[a', '\\', '(?P<>a)', '*', '+a', '?', 'a**', '(?<n>a', 'a)',
      '\\k<', '\\p{', '(?=', '(?i',
    ],
  },
  {
    id: 'graphql',
    language: 'GraphQL',
    grammar: 'parity/grammars/native/graphql.lino',
    oracle: 'tree-sitter-graphql 0.3.0',
    sources: [
      `${grammarSourceOf('native-graphql').repository}/blob/${grammarSourceOf('native-graphql').revision}/grammar.js`,
      `${grammarSourceOf('native-graphql').repository}/tree/${grammarSourceOf('native-graphql').revision}/${grammarSourceOf('native-graphql').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-graphql
    // 0.3.0, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more, commas, the extra a variable definition and an object
    // field take as their own, included; the rejections are sources the
    // oracle recovers from.
    ...nativeGrammar('native-graphql'),
    matches: [
      ...graphqlCorpus(false),
      '{ a }', 'query { a }', 'query Q { a b c }', 'mutation M($id: ID!) { like(id: $id) { count } }', 'subscription S { events { id } }',
      'query Q($a: Int = 1, $b: [String!]! = ["x"]) @d { a }', 'query Q($a: Int, $b: Int) { a }', '{ alias: field(arg: 1) }', '{ f(o: {a: 1, b: 2}, x: [1, 2]) }',
      '{ f(a: 1.5e3, b: -2, c: true, d: false, e: null, f: ENUM, g: "s", h: """block""") }', '{ ...F ... on T { a } ... @include(if: $x) { b } }',
      'fragment F on User { id name }', 'fragment F on User @d { id }', '# comment\n{ a }', '{ a # trailing\n b }', '{ a, b, c }', ',,{ a },,',
      'schema { query: Query mutation: Mutation }', 'schema @d { query: Q }', 'extend schema { subscription: S }', 'scalar Date',
      'scalar Date @specifiedBy(url: "https://x")', 'type T { a: Int }', 'type T implements A & B { f(a: Int = 1, b: [String!]!): T @d(x: 1) }',
      'type T implements & A { a: Int }', '"desc" type T { "field" a: Int }', '"""block\ndesc""" type T', 'interface I { a: Int }', 'interface I implements J { a: Int }',
      'union U = A | B', 'union U = | A | B', 'union U @d', 'enum E { A B }', 'enum E { A, B }', 'enum E @d { A @deprecated }', 'input I { a: Int = 1, b: String }',
      'directive @d(a: Int) on FIELD | QUERY', 'directive @d repeatable on | FIELD_DEFINITION', 'extend type T { b: Int }', 'extend type T implements I',
      'extend interface I @d', 'extend union U = C', 'extend enum E { C }', 'extend input I { c: Int }', 'extend scalar S @d',
      'type Query {\n  user(id: ID!): User\n  users(first: Int = 10, after: String): [User!]!\n}\n\nquery Q {\n  user(id: "é") { name }\n}\n',
      '{ a(f: 0.5, g: 1E10, h: -0) }', '{ a(l: [], o: {}) }', 'query ($v: Int) { a(v: $v) }',
    ],
    divergences: [],
    rejections: [
      ...graphqlCorpus(true),
      '', '{', '}', '{ a', 'query', 'query Q', 'type', 'type T {', 'type T { a: }', '{ a(: 1) }', '{ a(b: ) }', 'union U =', 'enum E { A',
      'fragment F { a }', 'fragment on T { a }', 'directive @d', 'directive d on FIELD', 'schema { query }', '{ a(b: [1, 2) }', '{ a(b: {c: 1) }',
      'query Q($a) { a }', 'query Q($a: ) { a }', 'scalar', 'extend', '{ a } }', '"desc"', '{ a(s: "unterminated) }', '$a', '@d',
    ],
  },
  {
    id: 'proto',
    language: 'Protocol Buffers',
    grammar: 'parity/grammars/native/proto.lino',
    oracle: 'tree-sitter-proto 0.6.0',
    sources: [
      `${grammarSourceOf('native-proto').repository}/blob/${grammarSourceOf('native-proto').revision}/grammar.js`,
      `${grammarSourceOf('native-proto').repository}/tree/${grammarSourceOf('native-proto').revision}/${grammarSourceOf('native-proto').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-proto
    // 0.6.0, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more, proto2 groups, editions, text-format option values and
    // streaming services included; the rejections are sources the oracle
    // recovers from.
    ...nativeGrammar('native-proto'),
    matches: [
      ...protoCorpus(false),
      '', 'syntax = "proto3";', "syntax = 'proto2';", 'edition = "2023";', 'package a.b.c;', 'import "x.proto";', 'import public "x.proto";',
      'import weak "x.proto";', 'option java_package = "com.x";', 'option (my.opt).sub = 1;', 'option (a) = { b: 1 c: "s" };', 'option a = 1.5e3;',
      'option a = -inf;', 'option a = nan;', 'option a = true;', 'option a = "x" "y";', 'option a = "\\x41\\101\\n";', 'message M {}', 'message M { int32 a = 1; }',
      'message M { optional string a = 1 [deprecated = true]; }', 'message M { repeated M.N a = 1; }', 'message M { map<string, int32> m = 1; }',
      'message M { message N { bool b = 1; } }', 'message M { enum E { A = 0; B = 1 [(x) = 2]; } }', 'message M { oneof o { string a = 1; int32 b = 2; } }',
      'message M { reserved 1, 2 to 5, 9 to max; reserved "a", "b"; }', 'message M { extensions 100 to 199; }', 'message M { ; }', 'message M { .a.B f = 1; }',
      'enum E { A = 0; B = -1; option allow_alias = true; }', 'enum E { A = 0x1F; B = 017; }', 'service S { rpc F (Req) returns (Res); }',
      'service S { rpc F (stream Req) returns (stream Res) { option (x) = 1; } }', 'extend google.protobuf.MessageOptions { string my = 50000; }',
      '// c\nmessage M {}', '/* c */ message M {}', 'message M { int32 a = 1; // trailing\n }',
      'syntax = "proto3";\npackage p;\nimport "a.proto";\nmessage M {\n  string name = 1; // é\n}\n', 'message M { required group G = 1 { optional int32 a = 2; } }',
    ],
    divergences: [],
    rejections: [
      ...protoCorpus(true),
      'message', 'message M', 'message M {', 'message M { int32 a; }', 'message M { int32 a = ; }', 'syntax = ;', 'syntax "proto3";', 'import;', 'package;',
      'enum E { A }', 'service S { rpc F (A) (B); }', 'option = 1;', 'message M { map<string> m = 1; }', '}', 'message M { int32 a = 1 }', '"unterminated',
      'message M { int32 a = 1; } }', 'service S { rpc F (A) returns B; }', 'enum E { A = ; }', 'message M { reserved; }',
    ],
  },
  {
    id: 'make',
    language: 'Make',
    grammar: 'parity/grammars/native/make.lino',
    oracle: 'tree-sitter-make 1.1.1',
    sources: [
      `${grammarSourceOf('native-make').repository}/blob/${grammarSourceOf('native-make').revision}/grammar.js`,
      `${grammarSourceOf('native-make').repository}/tree/${grammarSourceOf('native-make').revision}/${grammarSourceOf('native-make').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-make
    // 1.1.1, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more, function calls, recipe prefixes, conditionals and
    // directives included; the rejections are the corpus cases the oracle
    // recovers from (a custom .RECIPEPREFIX, which it does not read) and
    // sources it recovers from.
    ...nativeGrammar('native-make'),
    matches: [
      ...makeCorpus(false),
      '', '\n', 'a = b\n', 'a := b\n', 'a ::= b\n', 'a ?= b\n', 'a += b\n', 'a != echo hi\n', 'a =\n', 'a = $(b) $(c)\n', 'a = ${b}\n', 'a = $(b:.c=.o)\n',
      'a = $(subst a,b,c)\n', 'a = $(shell ls)\n', 'a = $(wildcard *.c)\n', 'a = $(patsubst %.c,%.o,$(SRC))\n', 'a = $(foreach x,$(L),$(x).o)\n',
      'a = $(if $(b),c,d)\n', 'a = $(call f,1,2)\n', 'a = $(eval $(b))\n', 'a = $(origin b)\n', 'a = $(info é)\n', 'a = $(error x)\n', 'a = $$HOME\n',
      'all:\n', 'all: a b\n', 'all: a | b\n', 'a b &: c\n', '%.o: %.c\n\t$(CC) -c $< -o $@\n', 'all: ; echo hi\n', 'all:\n\techo a\n\techo b\n',
      'all:\n\t@echo a\n', 'all:\n\t-rm x\n', 'all:\n\t+make -C d\n', 'all:\n\techo a \\\n\tb\n', '.PHONY: all clean\n', 'objs: a.o b.o\n',
      '$(OBJ): %.o: %.c\n\tcc $<\n', 'a: b\n\n\n\tc\n', 'include a.mk b.mk\n', '-include a.mk\n', 'sinclude a.mk\n', 'vpath %.c src\n', 'vpath\n',
      'export a = b\n', 'export\n', 'unexport a\n', 'override a = b\n', 'private a = b\n', 'undefine a\n', 'define a\nb\nendef\n', 'define a =\n  b\n  c\nendef\n',
      'ifeq ($(a),b)\nc = d\nendif\n', 'ifneq "a" "b"\nc = d\nelse\nc = e\nendif\n', 'ifdef a\nb = c\nelse ifndef d\ne = f\nendif\n', 'ifndef a\nendif\n',
      '# comment\n', 'a = b # c\n', 'all: # c\n\techo\n', 'a = b \\\n  c\n', 'a: b\n\t$(MAKE) $@\n', 'a: b\n\techo $(@D) $(<F) $^ $+ $? $* $%\n',
      'VPATH = a:b\n', '.RECIPEPREFIX = >\n', 'a.o: CFLAGS += -O2\n', 'a = é\n', 'ifeq a b\nendif\n', 'a: \nb: c\n',
    ],
    divergences: [],
    rejections: [
      ...makeCorpus(true),
      'ifeq (a,b\n', 'ifeq (a,b)\n', 'define a\nb\n', 'a = $(b\n', 'a: $(\n', 'endif\n', 'else\n', 'endef\n', 'a = ${b\n', 'ifdef\n', '\techo\n', 'export a = $(\n',
      'a: b\n\techo $(\n', 'vpath %.c $(\n', 'include $(\n', ':\n', '$(a\n', 'override\n',
      'a: private b = c\n', 'a: export b = c\n', '# café\nall: é\n', 'a:\nb\n', 'all:\necho\n', 'a: ;\n>b\n',
    ],
  },  {
    id: 'solidity',
    language: 'Solidity',
    grammar: 'parity/grammars/native/solidity.lino',
    oracle: 'tree-sitter-solidity 1.2.13',
    sources: [
      `${grammarSourceOf('native-solidity').repository}/blob/${grammarSourceOf('native-solidity').revision}/grammar.js`,
      `${grammarSourceOf('native-solidity').repository}/tree/${grammarSourceOf('native-solidity').revision}/${grammarSourceOf('native-solidity').corpus.path}`,
    ],
    // The grammar is the import of the grammar.json of tree-sitter-solidity
    // 1.2.13, the one the oracle parser is generated from
    // (js/scripts/import-native-grammars.mjs); the matches are every case of
    // its upstream corpus at the same revision the oracle reads, and a few
    // sources more, contracts, interfaces, libraries, statements, assembly
    // and literals included; the rejections are sources the oracle recovers
    // from.
    ...nativeGrammar('native-solidity'),
    matches: [
      ...solidityCorpus(false),

  '', 'pragma solidity ^0.8.0;\n', 'contract C {}\n', 'contract C { uint x; }\n', 'contract C { uint256 public x = 1; }\n',
  'contract C { function f() public {} }\n', 'contract C { function f(uint a) external pure returns (uint) { return a + 1; } }\n',
  'interface I { function f() external; }\n', 'library L { function f() internal {} }\n', 'abstract contract A is B, C {}\n',
  'contract C { event E(uint indexed a); }\n', 'contract C { error E(uint a); }\n', 'contract C { modifier m() { _; } }\n',
  'contract C { struct S { uint a; } }\n', 'contract C { enum E { A, B } }\n', 'contract C { mapping(address => uint) m; }\n',
  'contract C { constructor() {} }\n', 'contract C { receive() external payable {} fallback() external {} }\n',
  'import "a.sol";\n', 'import {A as B} from "a.sol";\n', 'import * as A from "a.sol";\n', 'using L for uint;\n',
  'type T is uint;\n', 'uint constant X = 1;\n', 'function f() pure returns (uint) { return 1; }\n',
  'contract C { function f() public { if (a) { b(); } else { c(); } } }\n', 'contract C { function f() public { for (uint i = 0; i < 1; i++) {} } }\n',
  'contract C { function f() public { while (a) { break; } do { continue; } while (b); } }\n',
  'contract C { function f() public { emit E(1); revert E(); require(a, "é"); } }\n',
  'contract C { function f() public { try g() returns (uint a) {} catch Error(string memory s) {} catch {} } }\n',
  'contract C { function f() public { assembly { let x := add(1, 2) } } }\n', 'contract C { function f() public { unchecked { a++; } } }\n',
  'contract C { function f() public { (uint a, , uint b) = g(); } }\n', 'contract C { function f() public { a = b ? c : d; } }\n',
  'contract C { function f() public { x = new D{value: 1}(2); } }\n', 'contract C { function f() public { x = a[1:2]; delete a; } }\n',
  'contract C { string s = unicode"é"; bytes b = hex"00ff"; }\n', 'contract C { uint x = 1 ether + 2 gwei; }\n', '// é\n/* é */\n',
  '/// @notice é\ncontract C {}\n', 'contract C { function f() public virtual override(A, B) {} }\n',
    ],
    divergences: [],
    rejections: [
      ...solidityCorpus(true),
  'contract C {', 'contract C { function f( }\n', 'contract C { uint x = ; }\n', 'pragma solidity\n', 'contract { }\n', 'import ;\n',
  'contract C { function f() public { if (a { } } }\n', 'contract C { function f() public { x = (1; } }\n', 'contract C { struct S { } \n',
  'contract C { function f() public { return } }\n',
    ],
  },
]);

function grammarParser(entry) {
  return compileGrammar(parseGrammarLinks(readFileSync(path.join(root, entry.grammar), 'utf8')));
}

/** The fixture of `entry`, computed from the oracle and the native grammar. */
export function buildNativeGrammarFixture(entry) {
  const parser = grammarParser(entry);
  const native = (source) => {
    const outcome = parser.parseTree(source);
    if (!outcome.ok) throw new Error(`${entry.grammar} rejects ${JSON.stringify(source)}`);
    return nativeRows(outcome.tree, source, entry);
  };
  const matches = entry.matches.map((source) => {
    const rows = oracleRows(source, entry.language);
    if (oracleRecovers(source, entry.language)) throw new Error(`the ${entry.oracle} oracle recovers from ${JSON.stringify(source)}`);
    if (JSON.stringify(native(source)) !== JSON.stringify(rows)) {
      throw new Error(`${entry.grammar} and ${entry.oracle} disagree on ${JSON.stringify(source)}`);
    }
    return { source, rows };
  });
  const divergences = entry.divergences.map(({ source, reason }) => {
    if (!oracleRecovers(source, entry.language)) {
      throw new Error(`the ${entry.oracle} oracle accepts the divergence ${JSON.stringify(source)}`);
    }
    return { source, reason, rows: native(source) };
  });
  const rejections = entry.rejections.map((source) => {
    if (!oracleRecovers(source, entry.language)) {
      throw new Error(`the ${entry.oracle} oracle accepts the rejection ${JSON.stringify(source)}`);
    }
    if (parser.parseTree(source).ok) throw new Error(`${entry.grammar} accepts ${JSON.stringify(source)}`);
    const repaired = parser.parseTree(source, { errorRecovery: true });
    if (repaired.rejection?.reason !== 'recovered') throw new Error(`${entry.grammar} does not recover from ${JSON.stringify(source)}`);
    return { source, recovered: renderSyntaxTree(repaired.tree) };
  });
  const { id, language, grammar, oracle, sources, hidden, anonymous, extras, oracleKinds, corpus } = entry;
  return {
    schemaVersion: 1, id, language, grammar, oracle, sources, ...(corpus ? { corpus } : {}), hidden, anonymous, extras, oracleKinds, matches, divergences, rejections,
  };
}

export const fixturePath = (entry) => `parity/fixtures/native-grammars/${entry.id}.json`;

/** Pinned corpus inputs; these are source cases, not execution evidence. */
export function buildNativeGrammarCorpusSources(id) {
  const entry = grammarSourceOf(`native-${id}`);
  return {
    generatedBy: 'js/scripts/generate-native-grammar-fixtures.mjs',
    sourceSha256: entry.corpus.sha256,
    cases: corpusCases(entry),
  };
}

export const DEFAULT_CST_PATH = 'parity/fixtures/native-default-cst-expected.json';

const sha256 = (text) => createHash('sha256').update(text).digest('hex');

/**
 * The default concrete syntax trees of one native grammar's inventory
 * sources, in the rows of
 * parity/fixtures/default-cst-expected.json. A positive tree is the oracle's
 * tree of the source, which the native grammar must build; a recovery tree is
 * the native grammar's repair of the recovery source, which the oracle also
 * recovers from.
 */
export function buildNativeDefaultLanguageExpected(entry) {
  const language = inventory.languages.find(({ name }) => name === entry.language);
  const catalog = languageEntry(entry.language);
  if (catalog.grammars[0]?.id !== entry.nativeGrammar) {
    throw new Error(`the catalog does not parse ${entry.language} with ${entry.nativeGrammar}`);
  }
  const parser = grammarParser(entry);
  const positive = parser.parseTree(language.source);
  if (!positive.ok) throw new Error(`${entry.grammar} rejects the ${entry.language} inventory source`);
  const rows = oracleRows(language.source, entry.language);
  if (JSON.stringify(nativeRows(positive.tree, language.source, entry)) !== JSON.stringify(rows)) {
    throw new Error(`${entry.grammar} and ${entry.oracle} disagree on the ${entry.language} inventory source`);
  }
  if (!oracleRecovers(language.recoverySource, entry.language)) {
    throw new Error(`the ${entry.oracle} oracle accepts the ${entry.language} recovery source`);
  }
  const repaired = parser.parseTree(language.recoverySource, { errorRecovery: true, recovery: 'accept' });
  const recovery = nativeRows(repaired.tree, language.recoverySource, entry);
  if (!hasRecovery(recovery)) throw new Error(`${entry.grammar} repairs the ${entry.language} recovery source without an ERROR or MISSING node`);
  const versions = (grammars) => Object.fromEntries(grammars.map(({ id, version, parserSha256 }) => [id, { version, parserSha256 }]));
  return {
    grammars: versions(catalog.grammars),
    oracleGrammars: versions(catalog.oracleGrammars),
    sourceSha256: sha256(language.source),
    recoverySourceSha256: sha256(language.recoverySource),
    positive: rows,
    recovery,
    embedded: [],
  };
}

/** Every native default expectation and the shared embedded-region fixtures. */
export function buildNativeDefaultCstExpected() {
  const languages = Object.fromEntries(NATIVE_GRAMMARS.map((entry) => [entry.language, buildNativeDefaultLanguageExpected(entry)]));
  return {
    description:
      'Default concrete syntax trees of the inventory sources of the languages a native Links Notation grammar parses by default, and of the embedded regions in those languages of the shared embedded-language fixtures. Positive rows are the trees of the pinned tree-sitter oracle grammar, which the native grammar builds; recovery rows are the native repair of the recovery source, which the oracle also recovers from. Row: [depth, field, kind, named, startByte, endByte, flags]; flags: E error, M missing, X extra.',
    generator: { script: 'js/scripts/generate-native-grammar-fixtures.mjs' },
    languages,
    embeddedFixtures: nativeEmbeddedFixtures(),
  };
}

/**
 * The embedded regions of the shared embedded-language fixtures of
 * parity/fixtures/default-cst-expected.json, where a region in a language a
 * native grammar parses by default has the native rows: the oracle's rows of
 * a positive region, which the native grammar builds, and the native repair of
 * a recovery region. The host grammar places the regions, so their bounds are
 * the oracle's. Only fixtures with such a region are listed.
 */
function nativeEmbeddedFixtures() {
  const oracle = JSON.parse(readFileSync(path.join(root, 'parity/fixtures/default-cst-expected.json'), 'utf8')).embeddedFixtures;
  const evidence = JSON.parse(readFileSync(path.join(root, 'parity/fixtures/issue-195-evidence.json'), 'utf8')).embedded;
  const fixtures = {};
  for (const fixture of evidence) {
    const label = `${fixture.host} -> ${fixture.target}`;
    let native = false;
    const regions = (text, embedded, recovery) => embedded.map((region) => {
      const entry = NATIVE_GRAMMARS.find(({ language }) => language === region.language);
      if (!entry) return region;
      native = true;
      const regionText = Buffer.from(text, 'utf8').subarray(region.startByte, region.endByte).toString('utf8');
      const parser = grammarParser(entry);
      if (!recovery) {
        const positive = parser.parseTree(regionText);
        if (!positive.ok || JSON.stringify(nativeRows(positive.tree, regionText, entry)) !== JSON.stringify(region.rows)) {
          throw new Error(`${entry.grammar} and ${entry.oracle} disagree on the ${region.path} region of ${label}`);
        }
        return region;
      }
      const rows = nativeRows(parser.parseTree(regionText, { errorRecovery: true, recovery: 'accept' }).tree, regionText, entry);
      if (!hasRecovery(rows)) throw new Error(`${entry.grammar} repairs the ${region.path} region of ${label} without an ERROR or MISSING node`);
      return { ...region, rows };
    });
    const trees = (text, want, recovery = false) => ({ sourceSha256: sha256(text), embedded: regions(text, want.embedded, recovery) });
    const entry = {
      positive: trees(fixture.source, oracle[label].positive),
      recovery: trees(fixture.recoverySource, oracle[label].recovery, true),
      spellings: fixture.spellings.map((spelling, index) => trees(spelling, oracle[label].spellings[index])),
    };
    if (native) fixtures[label] = entry;
  }
  return fixtures;
}

const ROW_KEYS = new Set(['rows', 'positive', 'recovery']);

// One row per line, and invisible characters escaped so the fixture reads unambiguously.
export function renderFixture(fixture) {
  const text = JSON.stringify(fixture, (key, value) => (ROW_KEYS.has(key) && Array.isArray(value) ? value.map((row) => `@@${JSON.stringify(row)}@@`) : value), 2)
    .replace(/"@@((?:[^"\\]|\\.)*)@@"/gu, (_, row) => JSON.parse(`"${row}"`))
    .replace(/[\u00a0\u2028\u2029\ufeff]/gu, (character) => `\\u${character.codePointAt(0).toString(16).padStart(4, '0')}`);
  return `${text}\n`;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  let stale = 0;
  const outputs = [
    ...NATIVE_GRAMMARS.map((entry) => [fixturePath(entry), () => buildNativeGrammarFixture(entry)]),
    ...['lua', 'toml', 'zig', 'pascal', 'vb'].map((id) => [`parity/fixtures/native-grammars/${id}-corpus.json`, () => buildNativeGrammarCorpusSources(id)]),
    [DEFAULT_CST_PATH, buildNativeDefaultCstExpected],
  ];
  for (const [relative, build] of outputs) {
    const file = path.join(root, relative);
    const text = renderFixture(build());
    if (check) {
      let current = null;
      try {
        current = readFileSync(file, 'utf8');
      } catch {
        // A missing fixture is stale.
      }
      if (current !== text) {
        stale += 1;
        console.error(`${relative} is stale; run node scripts/generate-native-grammar-fixtures.mjs`);
      }
    } else {
      mkdirSync(path.dirname(file), { recursive: true });
      writeFileSync(file, text);
      console.log(`wrote ${relative}`);
    }
  }
  if (stale > 0) process.exit(1);
}
