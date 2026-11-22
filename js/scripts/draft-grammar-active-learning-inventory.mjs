import { writeFileSync } from 'node:fs';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { renderRustItemImports, renderRustTest } from './generate-export-parity.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');

const file = new URL('../../parity/language-features.lino', import.meta.url);
const inventory = readProjectData(file);
const sharedDefinition = 'parity/self-translation/grammar-active-learning-rules-native.lino';
const id = 'active-regular-grammar-inference';
const feature = {
  id,
  required: true,
  javascript: { status: 'implemented', evidence: [
    'js/src/grammar-active-learning.js',
    'js/src/grammar-active-learning-rules.js',
    'js/tests/grammar-inference.test.js',
    sharedDefinition,
  ] },
  rust: { status: 'implemented', evidence: [
    'rust/src/grammar/inference/active.rs',
    'rust/src/grammar_active_learning_rules.rs',
    'rust/tests/unit/inference_active.rs',
    sharedDefinition,
  ] },
};
const previous = inventory.features.findIndex(row => row.id === id);
if (previous < 0) inventory.features.push(feature);
else inventory.features[previous] = feature;

for (const [javascript, suffix, kind] of [
  ['acceptActiveGrammarWord', 'Dfa::accepts', 'method'],
  ['learnActiveGrammar', 'learn_active_grammar', 'function'],
  ['learnActiveGrammarAutomaton', 'learn_active_grammar_automaton', 'function'],
  ['sampleActiveGrammarWords', 'sample_active_grammar_words', 'function'],
]) {
  inventory.javascriptExports[javascript] = { rust: 'grammar::inference::active::' + suffix, kind };
}
inventory.javascriptExports = Object.fromEntries(Object.entries(inventory.javascriptExports)
  .sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0));
writeProjectData(file, inventory);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs', import.meta.url),
  renderRustTest(inventory.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs', import.meta.url),
  renderRustItemImports(inventory.javascriptExports));
