// Prints what the JS merge decides for parity/fixtures/grammar-merge.json, to
// review the fixture's expectations by hand (requirement rows I195-MERGE-*).
import { readFileSync } from 'node:fs';
import { importPest, mergeGrammars } from '../js/src/index.js';

const fixture = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-merge.json', import.meta.url), 'utf8'));
const sources = (list) => list.map(({ text, format, ...rest }) => ({ ...rest, grammar: importPest(text) }));
const show = (result) => JSON.stringify(result, (key, value) => {
  if (value instanceof Map) return Object.fromEntries(value);
  return value;
}, 1);
const result = mergeGrammars(sources(fixture.sources), { samples: fixture.samples });
console.log(show(result.groups.map(({ grammar, ...rest }) => ({ ...rest, rules: grammar.ruleNames(), start: grammar.start }))));
console.log(show(result.alternatives));
const changed = fixture.sources.map((source) => fixture.upstreamChange.source === source.id ? { ...source, text: fixture.upstreamChange.text } : source);
const remerged = mergeGrammars(sources(changed), { samples: fixture.samples, previous: result });
console.log(show({ reused: remerged.reused, recomputed: remerged.recomputed, identities: remerged.groups[0].identities, rules: remerged.groups[0].grammar.ruleNames() }));
const fresh = mergeGrammars(sources(changed), { samples: fixture.samples });
console.log(show({ identities: fresh.groups[0].identities }));
console.log(show(mergeGrammars(sources(fixture.sources), { samples: fixture.samples, requiredEquivalences: fixture.requiredEquivalences }).failures));
