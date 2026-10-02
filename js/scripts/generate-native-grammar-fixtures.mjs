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
