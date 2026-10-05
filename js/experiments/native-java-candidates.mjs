// Tries hand-written Java sources against the native Java grammar and its
// tree-sitter-java oracle, as the fixture generator would take them: a match
// is a source both accept with the same rows, a rejection one the oracle
// recovers from and the native grammar rejects and repairs. Also checks the
// inventory source and recovery source of Java.
//   node experiments/native-java-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { hasRecovery, nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/java.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['line_comment', 'block_comment'], oracleKinds: nativeOracleKinds(text) };
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const java = inventory.languages.find(({ name }) => name === 'Java');

const matches = [
  java.source,
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
];
const rejections = [
  java.recoverySource,
  'class', 'class A {', 'class A { int x = ; }\n', 'class A { void f() { f(1, ); } }\n', 'class A { String s = "abc; }\n',
  '/* abc\nclass A {}\n', 'class { }\n', 'class A { void f( }\n', 'class A { int[] a = {1, 2; }\n', 'class A { void f() { if x {} } }\n',
  'import ;\n',
];
for (const source of matches) {
  let verdict;
  if (oracleRecovers(source, 'Java')) verdict = 'ORACLE-ERROR';
  else {
    const outcome = parser.parseTree(source);
    if (!outcome.ok) verdict = 'REJECT';
    else verdict = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, 'Java')) ? 'MATCH' : 'DIFF';
  }
  console.log(`match ${verdict} ${JSON.stringify(source)}`);
}
for (const source of rejections) {
  let verdict;
  if (!oracleRecovers(source, 'Java')) verdict = 'ORACLE-ACCEPTS';
  else if (parser.parseTree(source).ok) verdict = 'NATIVE-ACCEPTS';
  else {
    const repaired = parser.parseTree(source, { errorRecovery: true });
    const accepted = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' });
    verdict = repaired.rejection?.reason !== 'recovered' ? 'NO-REPAIR'
      : hasRecovery(nativeRows(accepted.tree, source, options)) ? 'REJECTION' : 'REPAIR-WITHOUT-ERROR';
  }
  console.log(`rejection ${verdict} ${JSON.stringify(source)}`);
}
