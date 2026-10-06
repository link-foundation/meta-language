// Finds the shortest prefix of FILE (cut after a " ; ") whose native parse ends
// expecting string content, i.e. the point where the lexer entered a string
// it should not have.
//   node --max-old-space-size=2048 experiments/issue-195-memory/bisect-string-state.mjs native-javascript FILE
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../../js/src/grammar.js';
import { parseGrammarLinks } from '../../js/src/grammar-links.js';
import { nativeGrammarText } from '../../js/src/native-grammar-parser.js';

const [id, file] = process.argv.slice(2);
const source = readFileSync(file, 'utf8');
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText(id)));
const cuts = [];
for (let i = source.indexOf(' ; '); i >= 0; i = source.indexOf(' ; ', i + 1)) cuts.push(i + 2);
const inString = (end) => {
  const outcome = parser.parseTree(source.slice(0, end));
  const expected = outcome.rejection?.expected ?? [];
  return expected.length <= 3 && expected.includes("character class") && expected.some((e) => e === '"\\""' || e === "\"'\"");
};
let lo = 0;
let hi = cuts.length - 1;
while (lo < hi) {
  const mid = (lo + hi) >> 1;
  if (inString(cuts[mid])) hi = mid; else lo = mid + 1;
}
const end = cuts[lo];
console.log(JSON.stringify({ end, tail: source.slice(Math.max(0, end - 300), end) }));
