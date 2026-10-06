// Counts, per rule, the evaluations of rule bodies (memo misses) and the
// grow iterations while parsing one source with parity/grammars/native/c.lino
// under a step limit, to find where a parse spends its steps.
//   node experiments/native-c-rule-profile.mjs 'void f() { a(s)->t; }' [STEPS] [recover]
// With `recover`, the parse runs with automatic error recovery.
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { Executor } from '../src/grammar-runtime/executor.js';

const [source, limit = '2000000', recover] = process.argv.slice(2);
const compiled = compileGrammar(parseGrammarLinks(readFileSync(process.env.LINO ?? new URL('../../parity/grammars/native/c.lino', import.meta.url), 'utf8')));
const bodies = new Map();
const grows = new Map();
const count = (map, key) => map.set(key, (map.get(key) ?? 0) + 1);
const { ruleBody, grow } = Executor.prototype;
Executor.prototype.ruleBody = function (rule, position, ...rest) {
  count(bodies, `${rule.nodeKind}@${position}`);
  return ruleBody.call(this, rule, position, ...rest);
};
Executor.prototype.grow = function (entry, rule, position, ...rest) {
  count(grows, `${rule.nodeKind}@${position}`);
  return grow.call(this, entry, rule, position, ...rest);
};
const out = compiled.parseTree(source, { stepLimit: Number(limit), errorRecovery: recover === 'recover', recovery: 'accept' });
console.log('ok', out.ok, 'rejection', out.rejection?.reason);
const top = (map) => [...map].sort((a, b) => b[1] - a[1]).slice(0, 25).map(([k, v]) => `${v}\t${k}`).join('\n');
console.log('rule bodies:\n' + top(bodies));
console.log('grows:\n' + top(grows));
