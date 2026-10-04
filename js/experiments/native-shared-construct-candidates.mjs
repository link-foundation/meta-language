// Lists what parity/fixtures/grammar-shared-concepts.json lacks once a native
// grammar joins: the rules of listed shared constructs a newly native grammar
// names, and every concept three or more native grammars name that no shared
// construct lists yet.
//   node experiments/native-shared-construct-candidates.mjs
import { readFileSync } from 'node:fs';

import { nativeGrammarConceptReuse, nativeGrammarIds, nativeGrammarRuleConcepts } from '../src/index.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/grammar-shared-concepts.json', import.meta.url), 'utf8'));
const rulesOf = new Map();
for (const grammar of nativeGrammarIds()) {
  for (const { rule, concept } of nativeGrammarRuleConcepts(grammar)) {
    if (!rulesOf.has(concept)) rulesOf.set(concept, []);
    rulesOf.get(concept).push([grammar, rule]);
  }
}
for (const { concept, rules } of fixture.sharedConstructs) {
  const listed = new Set(rules.map((rule) => rule.join(' ')));
  const missing = (rulesOf.get(concept) ?? []).filter((rule) => !listed.has(rule.join(' ')));
  if (missing.length > 0) console.log('EXTEND', concept, JSON.stringify(missing));
}
const listed = new Set(fixture.sharedConstructs.map(({ concept }) => concept));
for (const { concept, languages } of nativeGrammarConceptReuse().concepts) {
  if (languages.length >= 3 && !listed.has(concept)) console.log('ADD', concept, JSON.stringify(rulesOf.get(concept)));
}
