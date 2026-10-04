// Lists the words of tree-sitter rule and alias names that the naming check
// (scripts/issue-195-naming.mjs) rejects, over the grammar.json files given.
//   node experiments/tree-sitter-name-words.mjs GRAMMAR.json...
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import { loadWordNet } from '../scripts/english-vocabulary.mjs';
import { loadNamingRegisters, nameProblems } from '../scripts/issue-195-naming.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const context = { wordnet: loadWordNet(), vocabulary: loadNamingRegisters(root).vocabulary };
const failing = new Map();
const aliasNames = (node, out) => {
  if (!node || typeof node !== 'object') return out;
  if (node.type === 'ALIAS' && node.named) out.add(node.value);
  for (const value of Object.values(node)) if (value && typeof value === 'object') aliasNames(value, out);
  return out;
};
for (const file of process.argv.slice(2)) {
  const grammar = JSON.parse(readFileSync(file, 'utf8'));
  const names = new Set([...Object.keys(grammar.rules), ...aliasNames(grammar.rules, new Set())]);
  for (const name of names) {
    for (const word of name.replace(/([a-z])([A-Z])/gu, '$1_$2').toLowerCase().split(/[_\d]+/u).filter(Boolean)) {
      if (nameProblems(word, context).length === 0) continue;
      if (!failing.has(word)) failing.set(word, new Set());
      failing.get(word).add(`${grammar.name}:${name}`);
    }
  }
}
for (const [word, uses] of [...failing].sort()) console.log(`${word}\t${uses.size}\t${[...uses].slice(0, 3).join(' ')}`);
console.error(`${failing.size} failing words`);
