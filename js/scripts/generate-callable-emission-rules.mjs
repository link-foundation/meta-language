// Preserve the complete closed source and all automatically emitted declarations.
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/callable-emission-rules.js', import.meta.url);
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/callable-emission-rules.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/callable-emission-rules.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/callable-emission-rules-native.lino', import.meta.url);
const boundary = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n";
const hash = text => createHash('sha256').update(text).digest('hex');

export function readCallableEmissionSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(boundary).length !== 2) throw new Error('callable emission retains exactly its recorded fact-column import');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(boundary, '');
}

export function callableEmissionRulesPolicy() {
  const source = readFileSync(sourceFile, 'utf8');
  const closure = readCallableEmissionSourceModule(source);
  return { schemaVersion: 1, model: 'callable capture and latent output effects 1', source, sourceSha256: hash(source),
    sourceClosure: { source: closure, sourceSha256: hash(closure), removedImport: boundary,
      inputs: [{path: 'js/src/translation/callable-emission-rules.js', sourceSha256: hash(source)},
        {path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factsFile, 'utf8'))}] },
    captureSelection: 'First occurrence determines name order; the last actual reference determines its retained type, exactly as the original Map.set operation.',
    outputBoundary: 'The existing total-operation settling runs first. Creating a Lambda does not execute its body. Actual latent output or abort obligations refuse Lean/Rocq until their callable types thread output. JavaScript and Rust retain direct invocation effects.' };
}

export async function generateCallableEmissionRules({ check = false, native = false } = {}) {
  const policy = callableEmissionRulesPolicy();
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('callable emission source policy is stale');
    } else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.sourceClosure.source, {referenceSemantics: false})), { arrayConsumption: true });
    const artifact = {schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256, sourceClosure: policy.sourceClosure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text), assumptions: emitted.assumptions, encodings: emitted.encodings};
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('complete translated callable emission source is stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'callable_emission_rules', {check});
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateCallableEmissionRules({check: process.argv.includes('--check'), native: process.argv.includes('--native')});
  console.log('Callable emission: retained capture identities and explicit latent output boundaries');
}
