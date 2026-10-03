// Lists the rule names and aliased node kinds of the native grammars with
// their naming problems (vocabulary, abbreviation) under the readable-English
// naming check, to plan the concept-based rename of section 3.
import { readFileSync, readdirSync } from 'node:fs';
import { loadWordNet } from '../js/scripts/english-vocabulary.mjs';
import { loadNamingRegisters, nameProblems } from '../js/scripts/issue-195-naming.mjs';
import { parseGrammarLinks } from '../js/src/grammar-links.js';

const root = new URL('../', import.meta.url).pathname;
const wordnet = loadWordNet();
const { vocabulary } = loadNamingRegisters(root);
const dir = `${root}parity/grammars/native/`;
for (const file of readdirSync(dir).sort()) {
  const text = readFileSync(dir + file, 'utf8');
  const grammar = parseGrammarLinks(text);
  const rules = [...grammar.rules.keys()];
  const aliases = [...new Set([...text.matchAll(/\(alias ([^ ]+)/gu)].map((m) => m[1]))].filter((n) => !grammar.rules.has(n));
  for (const [what, list] of [['rule', rules], ['alias', aliases]]) {
    for (const name of list) {
      const problems = nameProblems(name, { wordnet, vocabulary });
      console.log(`${file}\t${what}\t${grammar.rules.get(name)?.kind ?? ''}\t${name}\t${problems.map((p) => p.message).join('; ')}`);
    }
  }
}
