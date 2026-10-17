// Append actual product implementation paths and the later coordination source.
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { readProjectData, writeProjectData } from '../js/scripts/project-data.mjs';

const featureFile = new URL('../parity/language-features.lino', import.meta.url);
const features = readProjectData(featureFile);
const translation = features.features.find(row => row.id === 'four-language-portable-translation');
if (!translation) throw new Error('the existing translation feature family is missing');
for (const [runtime, paths] of Object.entries({
  javascript: ['js/src/translation/javascript-products.js', 'js/src/translation/product-source-rules.js',
    'js/src/translation/rust-product-rules.js', 'js/src/translation/product-emission-rules.js',
    'js/src/typed-program-checked-projection.js', 'js/tests/translation-javascript-products.test.js',
    'js/tests/translation-rust-products.test.js', 'js/tests/translation-tuple-checker.test.js',
    'js/tests/typed-program-links-checked-projection.test.js', 'js/tests/typed-program-links-tuples.test.js',
    'js/tests/translation-product-unit.test.js'],
  rust: ['rust/src/translation/rust/product_rules.rs', 'rust/src/translation/javascript/product_source_rules.rs',
    'rust/src/translation/javascript/products.rs', 'rust/src/translation/javascript/infer/products.rs',
    'rust/src/translation/product_emission_rules.rs', 'rust/tests/unit/translation_tuple_checker.rs',
    'rust/tests/unit/typed_program_tuples.rs', 'rust/tests/unit/translation_product_unit.rs'],
})) for (const path of [...paths, 'parity/fixtures/typed-program-tuples.lino', 'parity/fixtures/translation-product-unit.lino']) {
  if (!existsSync(new URL(`../${path}`, import.meta.url))) throw new Error(`missing implementation path: ${path}`);
  if (!translation[runtime].evidence.includes(path)) translation[runtime].evidence.push(path);
}
writeProjectData(featureFile, features);

const body = 'Slowly reduce number of subagents and work only yourself once they all finish.';
const id = 'gradual-agent-reduction-2026-10-12';
const source = new URL(`../parity/sources/${id}.txt`, import.meta.url);
if (existsSync(source) && readFileSync(source, 'utf8') !== body) throw new Error('the existing captured instruction must not be overwritten');
if (!existsSync(source)) writeFileSync(source, body);
const registerFile = new URL('../parity/issue-195-sources.lino', import.meta.url);
const register = readProjectData(registerFile);
const sha256 = createHash('sha256').update(body).digest('hex');
const existing = register.requirementSources.find(entry => entry.key === 'agentReductionRequest');
if (existing && existing.sha256 !== sha256) throw new Error('the existing coordination source hash differs');
if (!existing) register.requirementSources.push({
  key: 'agentReductionRequest', id, kind: 'direct-instruction', url: `parity/sources/${id}.txt`,
  title: 'Later user instruction to conclude agent work gradually and continue with one coordinator',
  role: 'Later operational steering: existing scoped agent work finishes gradually, then the coordinator continues alone while preserving all inherited project scope.',
  ledgerCoverage: ['I195-WORKFLOW-GRADUAL-AGENT-REDUCTION'], author: 'konard',
  createdAt: '2026-10-12T19:18:16.000Z', updatedAt: '2026-10-12T19:18:16.000Z',
  revisionOrigin: 'capture time; the conversation does not expose a source timestamp',
  bytes: Buffer.byteLength(body), sha256,
});
writeProjectData(registerFile, register);
console.log(`product evidence paths and ${id} recorded`);
