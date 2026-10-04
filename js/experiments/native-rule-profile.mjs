// Counts the rule bodies a native grammar evaluates for a source: per rule,
// and the (rule, offset) pairs evaluated more than once (memo misses for
// another parse state, or calls a left recursion left unmemoized).
// Usage: node experiments/native-rule-profile.mjs <grammar id> <source> [top]
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { Executor } from '../src/grammar-runtime/executor.js';

const [id, source, top = '25'] = process.argv.slice(2);
const root = resolve(import.meta.dirname, '../..');
const parser = compileGrammar(parseGrammarLinks(readFileSync(resolve(root, `parity/grammars/native/${id}.lino`), 'utf8')));
const byRule = new Map();
const byCall = new Map();
const states = new Map();
const original = Executor.prototype.ruleBody;
Executor.prototype.ruleBody = function ruleBody(rule, position, state, inToken) {
  const name = rule.name ?? rule.nodeKind ?? String(rule.index);
  byRule.set(name, (byRule.get(name) ?? 0) + 1);
  const call = `${name}@${position}`;
  byCall.set(call, (byCall.get(call) ?? 0) + 1);
  if (!states.has(call)) states.set(call, new Set());
  states.get(call).add(state.key);
  return original.call(this, rule, position, state, inToken);
};
const out = parser.parseTree(source.replaceAll('\\n', '\n'), { stepLimit: 5e6 });
console.log('ok', out.ok, out.rejection?.reason ?? '');
const sorted = (map) => [...map].sort((a, b) => b[1] - a[1]).slice(0, Number(top));
console.log('rules:', sorted(byRule).map(([k, v]) => `${k}=${v}`).join(' '));
console.log('calls:', sorted(byCall).map(([k, v]) => `${k}=${v}/${states.get(k).size}`).join(' '));
for (const call of (process.env.SHOW ?? '').split(',').filter(Boolean)) {
  console.log(call, [...(states.get(call) ?? [])]);
}
