// Translate the complete checked ownership decisions with their actual module
// dependency. The common library writer records only ownership/loan styles.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/owned-array-append-rules.js', import.meta.url);
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const requiredImport = "import { readRequiredNumberFact } from './fact-column-rules.js';\n";
const hash = source => createHash('sha256').update(source).digest('hex');

export function readOwnedArrayAppendSourceModule() {
  const source = readFileSync(sourceFile, 'utf8');
  assert.equal(source.split(requiredImport).length, 2, 'one actual required-column module edge is resolved');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

export async function generateOwnedArrayAppendRules({ check = false, native = false } = {}) {
  const source = readFileSync(sourceFile, 'utf8');
  const facts = readFileSync(factsFile, 'utf8');
  const closure = readOwnedArrayAppendSourceModule();
  const policy = {
    schemaVersion: 1,
    model: 'fresh local array construction and final binding consumption 1',
    source: 'js/src/translation/owned-array-append-rules.js',
    sourceSha256: hash(source),
    decisions: ['acceptOwnedArrayAppendShape', 'acceptOwnedArrayRemovalShape', 'readOwnedArrayRemovalSliceBounds', 'acceptOwnedArrayRead', 'acceptOwnedArrayReaderResult',
      'acceptOwnedArrayAppendUses', 'acceptOwnedArrayFinalUse'],
    sourceClosure: { source: closure, sourceSha256: hash(closure), removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/owned-array-append-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(facts) }] },
    boundary: 'Only actual fresh source allocations with no earlier or loop-carried escape lower to immutable array value rebinding. A native storage move additionally requires no reachable later read of the same actual renamed binding. Links reload retains ordinary immutable value semantics.',
  };
  for (const file of [new URL('../src/translation/data/owned-array-append.lino', import.meta.url),
    new URL('../../rust/src/data/owned-array-append.lino', import.meta.url)]) {
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const program = checkProgram(parseJavaScript(closure, { referenceSemantics: false }));
    const emitted = emitRust(program, { arrayConsumption: true });
    assert.equal(emitted.definitions.length, program.declarations.size, 'every actual declaration and generated loop helper is retained');
    for (const decision of policy.decisions) assert.ok(program.declarations.has(decision), decision);
    writeRecordRuleLibrary(emitted, 'owned_array_append_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateOwnedArrayAppendRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Owned array construction: actual fresh allocations, escape relations and final lexical consumption');
}
