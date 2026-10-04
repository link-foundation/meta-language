// Times the built-in LiNo CST against the official links-notation parser on
// inputs that made earlier parsers superlinear or overflow the stack.
import { Parser } from '../js/node_modules/links-notation/src/Parser.js';
import { parseLinoCst } from '../js/src/lino-grammar.js';

const inputs = {
  deepParens: (n) => `${'('.repeat(n)}a${')'.repeat(n)}\n`,
  unclosedGroups: (n) => `${'(a '.repeat(n)}\n`,
  staircase: (n) => Array.from({ length: n }, (_, d) => `${' '.repeat(d % 60)}a`).join('\n'),
  longQuoteRun: (n) => `${'"'.repeat(n)}x${'"'.repeat(n)}\n`,
  unclosedQuotes: (n) => `${'"a '.repeat(n)}\n`,
  manyLines: (n) => 'a: b c # note\n  child (x y)\n'.repeat(n),
  failingSiblings: (n) => `x\n${'  (broken\n'.repeat(n)}`,
};
for (const [name, make] of Object.entries(inputs)) {
  for (const n of [1000, 4000, 16000]) {
    const text = make(n);
    let t = performance.now();
    let cst = 'ok';
    try { parseLinoCst(text); } catch (e) { cst = e.constructor.name; }
    const cstMs = performance.now() - t;
    t = performance.now();
    let off = 'ok';
    try { new Parser().parse(text); } catch (e) { off = e.constructor.name; }
    const offMs = performance.now() - t;
    console.log(name.padEnd(16), String(n).padStart(6), `cst ${cstMs.toFixed(1)}ms ${cst}`.padEnd(26), `official ${offMs.toFixed(1)}ms ${off}`);
  }
}
