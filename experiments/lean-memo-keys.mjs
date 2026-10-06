// Groups the memo keys of a native parse by rule and offset and prints the
// calls made in the most distinct states, by wrapping Executor.prototype.run.
//   node experiments/lean-memo-keys.mjs LANGUAGE SOURCE
import { Executor } from '../js/src/grammar-runtime/executor.js';
import { parseNative } from '../js/src/native-grammar-parser.js';

const [language, source] = process.argv.slice(2);
const run = Executor.prototype.run;
let last = null;
const set = Map.prototype.set;
Executor.prototype.run = function (start) {
  const memo = this.memo;
  const all = new Set();
  memo.set = function (key, value) { all.add(key); return set.call(this, key, value); };
  const outcome = run.call(this, start);
  last = { program: this.program, keys: all };
  return outcome;
};
parseNative(`native-${language.toLowerCase()}`, source);
const names = [...last.program.rules.values()];
const groups = new Map();
const states = new Set();
for (const key of last.keys) {
  const [index, position, state, rest] = key.split('|');
  states.add(state);
  const group = `${names.find((rule) => String(rule.index) === index)?.nodeKind ?? index}@${position}`;
  if (!groups.has(group)) groups.set(group, new Set());
  groups.get(group).add(`${state}|${rest}`);
}
console.log(`${last.keys.size} keys, ${groups.size} rule offsets, ${states.size} states`);
for (const [group, keys] of [...groups].sort((a, b) => b[1].size - a[1].size).slice(0, 8)) console.log(keys.size, group, [...keys].slice(0, 6).join('  '));
