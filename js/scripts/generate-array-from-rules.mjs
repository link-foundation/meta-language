// Preserve every declaration of Array.from and its two actual source inputs.
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/array-from-rules.js', import.meta.url);
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const iterableFile = new URL('../src/translation/array-value-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/array-from-rules.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/array-from-rules.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/array-from-rules-native.lino', import.meta.url);
const factBoundary = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n";
const iterableBoundary = "import { readArrayIterableForm } from './array-value-rules.js';\n";
const hash = source => createHash('sha256').update(source).digest('hex');

export function readArrayFromSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  for (const boundary of [factBoundary, iterableBoundary]) {
    if (source.split(boundary).length !== 2) throw new Error('Array.from retains each recorded dependency import exactly once');
    source = source.replace(boundary, '');
  }
  return readFileSync(factsFile, 'utf8') + '\n' + readFileSync(iterableFile, 'utf8') + '\n' + source;
}

export function arrayFromRulesPolicy() {
  const source = readFileSync(sourceFile, 'utf8');
  const closure = readArrayFromSourceModule(source);
  return { schemaVersion: 1, model: 'Array.from iterable and callback construction 1', source, sourceSha256: hash(source),
    sourceClosure: { source: closure, sourceSha256: hash(closure), removedImports: [factBoundary, iterableBoundary],
      inputs: [sourceFile, factsFile, iterableFile].map(file => ({path: file === sourceFile ? 'js/src/translation/array-from-rules.js' : file === factsFile ? 'js/src/translation/fact-column-rules.js' : 'js/src/translation/array-value-rules.js', sourceSha256: hash(readFileSync(file, 'utf8'))})) },
    inferenceBoundary: 'Plans select the original iterable child types, expected element and callback terms. The existing unifier applies parameter pairs in order, followed by the result pair; a callback may declare fewer supplied parameters.',
    renderingBoundary: 'The map-only formatter retains the existing four target strings. Hosts emit receiver then callback once and retain each target runtime and unsupported boundary.' };
}

export async function generateArrayFromRules({ check = false, native = false } = {}) {
  const policy = arrayFromRulesPolicy();
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('Array.from source policy is stale');
    } else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false })), { arrayConsumption: true });
    const artifact = {schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256, sourceClosure: policy.sourceClosure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text), assumptions: emitted.assumptions, encodings: emitted.encodings};
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('complete translated Array.from source is stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'array_from_rules', {check});
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateArrayFromRules({check: process.argv.includes('--check'), native: process.argv.includes('--native')});
  console.log('Array.from: retained iterable type owners and ordered callback constraints');
}
