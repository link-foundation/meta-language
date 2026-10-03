#!/usr/bin/env node
// Generates parity/fixtures/native-grammars/<language>.json: the corpus a
// native merged grammar in parity/grammars/native/ is checked against, with
// the rows of the tree-sitter oracle that still backs the language's default
// parse.
//
//   node scripts/generate-native-grammar-fixtures.mjs           write the fixtures
//   node scripts/generate-native-grammar-fixtures.mjs --check   fail when one is stale
//
// `matches` are sources whose native rows equal the oracle rows. `divergences`
// are sources one merged source accepts and the oracle does not; the native
// grammar accepts them and the fixture keeps its rows. `rejections` are
// invalid sources: the oracle recovers with error nodes, the native grammar
// rejects them until its recovery rules land. `hidden`, `anonymous` and
// `extras` tell js/scripts/native-grammar-rows.mjs how the oracle shows the
// native leaves and nodes of those kinds.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from './native-grammar-rows.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

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

export const NATIVE_GRAMMARS = Object.freeze([
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
    hidden: ['byte_order_mark'],
    anonymous: [],
    extras: [],
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
    hidden: ['blank_space'],
    anonymous: ['newline', 'comment_marker'],
    extras: ['comment'],
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
    hidden: [],
    anonymous: ['newline', 'anything', 'word'],
    extras: [],
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
    hidden: ['blank_space'],
    anonymous: ['newline'],
    extras: [],
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
    hidden: [],
    anonymous: [],
    extras: [],
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
    hidden: [],
    anonymous: ['whitespace', 'comment_text', 'string_text', 'directive_name'],
    extras: [],
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
    hidden: ['byte_order_mark'],
    anonymous: [
      'whitespace', 'comment_text', 'string_text', 'regex_prefix', 'hash_prefix', 'graph_mark', 'here_terminator',
      'here_newline', 'here_line', 'here_end',
    ],
    extras: [],
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
    return { source };
  });
  const { id, language, grammar, oracle, sources, hidden, anonymous, extras } = entry;
  return { schemaVersion: 1, id, language, grammar, oracle, sources, hidden, anonymous, extras, matches, divergences, rejections };
}

export const fixturePath = (entry) => `parity/fixtures/native-grammars/${entry.id}.json`;

// One row per line, and invisible characters escaped so the fixture reads unambiguously.
export function renderFixture(fixture) {
  const text = JSON.stringify(fixture, (key, value) => (key === 'rows' ? value.map((row) => `@@${JSON.stringify(row)}@@`) : value), 2)
    .replace(/"@@((?:[^"\\]|\\.)*)@@"/gu, (_, row) => JSON.parse(`"${row}"`))
    .replace(/[\u00a0\u2028\u2029\ufeff]/gu, (character) => `\\u${character.codePointAt(0).toString(16).padStart(4, '0')}`);
  return `${text}\n`;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  let stale = 0;
  for (const entry of NATIVE_GRAMMARS) {
    const file = path.join(root, fixturePath(entry));
    const text = renderFixture(buildNativeGrammarFixture(entry));
    if (check) {
      let current = null;
      try {
        current = readFileSync(file, 'utf8');
      } catch {
        // A missing fixture is stale.
      }
      if (current !== text) {
        stale += 1;
        console.error(`${fixturePath(entry)} is stale; run node scripts/generate-native-grammar-fixtures.mjs`);
      }
    } else {
      mkdirSync(path.dirname(file), { recursive: true });
      writeFileSync(file, text);
      console.log(`wrote ${fixturePath(entry)}`);
    }
  }
  if (stale > 0) process.exit(1);
}
