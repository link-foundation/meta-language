// Prints, for a native parse of each source, the keyword spans the keyword
// lexing marks keyword-only and the call chains (rule@position) the keyword
// matched in there (`\n` escapes a line break).
//   node experiments/keyword-match-chains.mjs <grammar-id> <source>...
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';
import { KeywordLexing } from '../js/src/grammar-runtime/executor.js';

const [id, ...sources] = process.argv.slice(2);
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../parity/grammars/native/${id}.lino`, import.meta.url), 'utf8')));
const chain = (call) => {
  const out = [];
  for (let current = call, depth = 0; current && depth < 40; depth += 1) {
    out.push(`${current.rule ?? '?'}${current.builds ? '' : '~'}@${current.position}`);
    current = [...current.parents][0];
  }
  return out.join(' < ');
};
const conflicts = KeywordLexing.prototype.conflicts;
KeywordLexing.prototype.conflicts = function (root) {
  const before = new Set(this.only);
  const saved = new Map(this.matched);
  const found = conflicts.call(this, root);
  for (const span of this.only) {
    if (before.has(span)) continue;
    console.log('  keyword-only', span);
    for (const call of saved.get(span) ?? []) console.log('    ', chain(call));
  }
  return found;
};
for (const raw of sources) {
  const source = raw.replaceAll('\\n', '\n');
  console.log(JSON.stringify(source));
  const outcome = parser.parseTree(source);
  console.log('  ok', outcome.ok, outcome.rejection?.offset);
}
