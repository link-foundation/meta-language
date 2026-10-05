import { readFileSync, writeFileSync } from 'node:fs';
import { nativeGrammarRuleConcepts, nativeGrammarConceptReuse } from '../src/index.js';
const p = '../parity/fixtures/grammar-shared-concepts.json';
const text = readFileSync(p, 'utf8');
const go = nativeGrammarRuleConcepts('native-go');
const byConcept = new Map();
for (const { rule, concept } of go) byConcept.set(concept, [...(byConcept.get(concept) ?? []), rule]);
const lines = text.split('\n');
const listed = new Set();
let added = 0;
for (let i = 0; i < lines.length; i += 1) {
  const m = lines[i].match(/^    \{ "construct": .*"concept": "([^"]+)", "rules": \[(.*)\] \},?$/);
  if (!m) continue;
  listed.add(m[1]);
  const rules = byConcept.get(m[1]);
  if (!rules || m[2].includes('"native-go"')) continue;
  if (rules.length !== 1) { console.log('multiple', m[1], rules); continue; }
  lines[i] = lines[i].replace(/\]\] \}(,?)$/, `], ["native-go", "${rules[0]}"]] }$1`);
  added += 1;
}
writeFileSync(p, lines.join('\n'));
console.log('added', added);
for (const { concept, languages } of nativeGrammarConceptReuse().concepts) if (languages.length >= 3 && !listed.has(concept)) console.log('unlisted', concept, languages);

// Constructs that Go brings to three or more native grammars.
const constructs = {
  'grammar.block': 'block of statements',
  'grammar.field-declaration-list': 'field declaration list',
  'grammar.integer-literal': 'integer literal',
  'grammar.field-identifier-alias': 'field name',
  'grammar.parameter-list': 'parameter list',
  'grammar.parenthesized-type': 'parenthesized type',
  'grammar.source-file': 'source file as a whole',
};
const after = readFileSync(p, 'utf8');
const rows = [];
for (const { concept, languages } of nativeGrammarConceptReuse().concepts) {
  if (!constructs[concept] || after.includes(`"concept": "${concept}"`)) continue;
  const rules = languages.flatMap((language) => nativeGrammarRuleConcepts(`native-${language}`).filter((entry) => entry.concept === concept).map(({ rule }) => [`native-${language}`, rule]));
  rows.push(`    { "construct": "${constructs[concept]}", "concept": "${concept}", "rules": ${JSON.stringify(rules).replaceAll('],[', '], [').replaceAll('","', '", "')} }`);
}
if (rows.length > 0) {
  const at = after.indexOf('\n  ],\n  "lookalikes"');
  writeFileSync(p, `${after.slice(0, at)},\n${rows.join(',\n')}${after.slice(at)}`);
  console.log(rows.join('\n'));
}
