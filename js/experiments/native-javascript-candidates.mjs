// Sorts candidate sources for the native JavaScript fixture: MATCH (the oracle
// accepts and both trees agree), REJECT (the oracle recovers, the native
// grammar rejects and recovers), or what keeps them out of the fixture.
//   node experiments/native-javascript-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/javascript.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment', 'html_comment'], oracleKinds: nativeOracleKinds(text) };
const candidates = [
  '', 'x;', 'let x = 1;', 'const f = (a, b) => a + b;', 'function f(a, ...b) { return a; }', 'class A extends B { #x = 1; static m() {} }',
  'import { a as b } from "c";', 'export default function () {}', 'a?.b ?? c;', 'x = `a${b}c`;', 'async function f() { await g(); }',
  'for (const [k, v] of m) {}', 'label: while (true) break label;', 'x = /ab+c/gi;', 'let { a, ...rest } = o;', 'x = a ? b : c;',
  '// c\nlet x = 1 /* d */;\n', 'a\nb\n', '<!-- c\nx;', 'x = <div className="a">{b}</div>;', 'function* g() { yield* h(); }',
  'try { a(); } catch { b(); } finally { c(); }', 'x = 0x1fn + 1_000 + .5e3;', 'switch (x) { case 1: break; default: }', 'new.target;',
  "// Greeting module — café\nimport { readFile } from 'node:fs';\n/* block comment */\nexport const greet = (name = \"wörld\") => `Hello, ${name}!`;\nclass Counter { #count = 0; increment() { return ++this.#count; } }\n",
  'const = 1;', 'if (ready { go(); }', 'function', 'function f(', 'function f() {', '}', '{', 'let x = ;', 'class A {', '"abc', "'a",
  '/* abc', 'x = 1 +;', 'if (x', 'import {', 'export', 'x = = 1;', 'f(a,,);', 'a[;', 'switch (x) {', '`abc', 'x.;', 'for (;;', 'new', 'x = <div>;',
  'let x = 1;　let y;', 'let a; let b;', 'x; y;', 'x;﻿', '\u0085x;',
];
for (const source of candidates) {
  let kind;
  if (oracleRecovers(source, 'JavaScript')) {
    if (compiled.parseTree(source).ok) kind = 'NATIVE-ACCEPTS';
    else kind = compiled.parseTree(source, { errorRecovery: true }).rejection?.reason === 'recovered' ? 'REJECT' : 'NO-RECOVERY';
  } else {
    const outcome = compiled.parseTree(source);
    if (!outcome.ok) kind = 'NATIVE-REJECTS';
    else kind = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, 'JavaScript')) ? 'MATCH' : 'DIFF';
  }
  console.log(kind, JSON.stringify(source));
}
