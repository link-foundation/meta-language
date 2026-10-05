// Tries hand-written Go sources against the native Go grammar and its
// tree-sitter-go oracle, as the fixture generator would take them: a match
// is a source both accept with the same rows, a rejection one the oracle
// recovers from and the native grammar rejects and repairs. Also checks the
// inventory source and recovery source of Go.
//   node experiments/native-go-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { hasRecovery, nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/go.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const go = inventory.languages.find(({ name }) => name === 'Go');

const matches = [
  go.source,
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
];
const rejections = [
  go.recoverySource,
  'package', 'package main\nfunc f() {', 'package main\nvar x = \n', 'package main\nfunc f() { g(1, }\n', 'package main\nvar s = "abc\n',
  'package main\n/* abc\n', 'package main\nimport (\n', 'package main\nvar a = []int{1, 2\n', 'package main\nfunc f() { if {} }\n',
  'package main\ntype struct {}\n', 'package main\ntype P struct { X int\n',
];
for (const source of matches) {
  let verdict;
  if (oracleRecovers(source, 'Go')) verdict = 'ORACLE-ERROR';
  else {
    const outcome = parser.parseTree(source);
    if (!outcome.ok) verdict = 'REJECT';
    else verdict = JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, 'Go')) ? 'MATCH' : 'DIFF';
  }
  console.log(`match ${verdict} ${JSON.stringify(source)}`);
}
for (const source of rejections) {
  let verdict;
  if (!oracleRecovers(source, 'Go')) verdict = 'ORACLE-ACCEPTS';
  else if (parser.parseTree(source).ok) verdict = 'NATIVE-ACCEPTS';
  else {
    const repaired = parser.parseTree(source, { errorRecovery: true });
    const accepted = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' });
    verdict = repaired.rejection?.reason !== 'recovered' ? 'NO-REPAIR'
      : hasRecovery(nativeRows(accepted.tree, source, options)) ? 'REJECTION' : 'REPAIR-WITHOUT-ERROR';
  }
  console.log(`rejection ${verdict} ${JSON.stringify(source)}`);
}
