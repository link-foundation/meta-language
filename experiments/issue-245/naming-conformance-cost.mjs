// Run from the repository root with Node.js 22. Each phase uses the full
// naming gate inputs, including the verified historical source views.
import { performance } from 'node:perf_hooks';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../../', import.meta.url));

function elapsed(label, run) {
  const started = performance.now();
  const value = run();
  console.log(`${label}: ${((performance.now() - started) / 1000).toFixed(2)} s`);
  return value;
}

const started = performance.now();
const naming = await import('../../js/scripts/issue-195-naming.mjs');
const { extractOwnCodebaseHistoricalNames } = await import('../../js/scripts/own-codebase-baseline-naming-history.mjs');
const { loadWordNet } = await import('../../js/scripts/english-vocabulary.mjs');
console.log(`load naming code: ${((performance.now() - started) / 1000).toFixed(2)} s`);

const wordnet = elapsed('load WordNet', loadWordNet);
const { records, exemptions, vocabulary } = elapsed('read registers', () => naming.loadNamingRegisters(root));
const current = elapsed('extract current names', () => naming.extractNameInventory(root));
const historical = elapsed('verify historical names', () => extractOwnCodebaseHistoricalNames(root, naming.NAME_INVENTORIES));
const names = [...current, ...historical];
const formers = elapsed('extract former names', () => naming.extractFormerNames(root));
const descriptions = elapsed('extract source descriptions', () => naming.extractSourceDescriptions(root));
const context = { wordnet, vocabulary };

const problems = [
  ...elapsed('check concept records', () => naming.checkConceptRecords(records, context)),
  ...elapsed('check source descriptions', () => naming.checkSourceDescriptions(records, descriptions)),
  ...elapsed('check inventory coverage', () => naming.checkInventoryCoverage(records, exemptions, names, context, formers)),
];
console.log(`${records.length} records, ${names.length} names, ${problems.length} problems`);
if (problems.length > 0) process.exitCode = 1;
