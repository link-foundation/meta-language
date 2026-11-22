import { writeFileSync } from 'node:fs';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { renderRustTest, renderRustItemImports } from './generate-export-parity.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');

const featureFile = new URL('../../parity/language-features.lino', import.meta.url);
const inventory = readProjectData(featureFile);
const common = [
  'parity/fixtures/regular-grammar-inference.lino',
  'parity/self-translation/grammar-structural-inference-rules-native.lino',
];
const id = 'grammar-delimiter-production-inference';
const feature = {
  id,
  required: true,
  javascript: { status: 'implemented', evidence: [
    'js/src/grammar-structural-inference.js', 'js/src/grammar-structural-inference-rules.js',
    'js/tests/grammar-inference.test.js', ...common,
  ] },
  rust: { status: 'implemented', evidence: [
    'rust/src/grammar_inference/structural.rs', 'rust/src/grammar_structural_inference_rules.rs',
    'rust/tests/unit/regular_grammar_inference.rs', ...common,
  ] },
};
const previous = inventory.features.findIndex(row => row.id === id);
if (previous < 0) inventory.features.push(feature);
else inventory.features[previous] = feature;

for (const [javascript, rust] of Object.entries({
  inferStructuralGrammar: 'infer_structural_grammar',
  readStructuralGrammarInferenceLinks: 'read_structural_grammar_inference_links',
})) inventory.javascriptExports[javascript] = { rust, kind: 'function' };
inventory.javascriptExports = Object.fromEntries(
  Object.entries(inventory.javascriptExports).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0),
);
writeProjectData(featureFile, inventory);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs', import.meta.url), renderRustTest(inventory.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs', import.meta.url), renderRustItemImports(inventory.javascriptExports));
