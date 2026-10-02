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
// rejects them until its recovery rules land.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { hasRecovery, nativeRows, oracleRows } from './native-grammar-rows.mjs';

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
    if (hasRecovery(rows)) throw new Error(`the ${entry.oracle} oracle recovers from ${JSON.stringify(source)}`);
    if (JSON.stringify(native(source)) !== JSON.stringify(rows)) {
      throw new Error(`${entry.grammar} and ${entry.oracle} disagree on ${JSON.stringify(source)}`);
    }
    return { source, rows };
  });
  const divergences = entry.divergences.map(({ source, reason }) => {
    if (!hasRecovery(oracleRows(source, entry.language))) {
      throw new Error(`the ${entry.oracle} oracle accepts the divergence ${JSON.stringify(source)}`);
    }
    return { source, reason, rows: native(source) };
  });
  const rejections = entry.rejections.map((source) => {
    if (!hasRecovery(oracleRows(source, entry.language))) {
      throw new Error(`the ${entry.oracle} oracle accepts the rejection ${JSON.stringify(source)}`);
    }
    if (parser.parseTree(source).ok) throw new Error(`${entry.grammar} accepts ${JSON.stringify(source)}`);
    return { source };
  });
  const { id, language, grammar, oracle, sources, hidden } = entry;
  return { schemaVersion: 1, id, language, grammar, oracle, sources, hidden, matches, divergences, rejections };
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
