// Counts executor work on `a+a+...+a` chains of growing length: the steps,
// the grow passes of left-recursive rules, the rule bodies evaluated, and the
// results the sequence joins produce.
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { Executor } from '../src/grammar-runtime/executor.js';

const grammar = new URL('../../parity/grammars/native/rust.lino', import.meta.url);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
const counts = {};
const count = (name) => { counts[name] = (counts[name] ?? 0) + 1; };
for (const name of ['grow', 'ruleBody', 'sequence', 'precedence', 'reference']) {
  const original = Executor.prototype[name];
  Executor.prototype[name] = function counted(...args) {
    count(name);
    return original.apply(this, args);
  };
}
const shape = process.argv[2] ?? 'a+';
for (const n of [4, 8, 16, 32]) {
  for (const key of Object.keys(counts)) delete counts[key];
  const started = performance.now();
  const outcome = parser.parseTree(`fn f() { ${shape.repeat(n)}a; }`, { stepLimit: 1e9 });
  console.log(n, outcome.ok, Math.round(performance.now() - started), 'ms', JSON.stringify(counts));
}
