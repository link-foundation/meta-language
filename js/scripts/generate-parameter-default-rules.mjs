// Record the complete shared initializer-order decisions and self-translate them.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { readProjectData, writeProjectData } from './project-data.mjs';

const sourceFile = new URL('../src/translation/parameter-default-rules.js', import.meta.url);
const policyFiles = [new URL('../src/translation/data/parameter-default-rules.lino', import.meta.url),
  new URL('../../rust/src/data/parameter-default-rules.lino', import.meta.url)];
const artifactFile = new URL('../../parity/self-translation/parameter-default-rules-native.lino', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');

export async function generateParameterDefaultRules({ check = false, native = false } = {}) {
  const source = readFileSync(sourceFile, 'utf8');
  const policy = { schemaVersion: 1, model: 'parameter default argument order 1', source,
    sourceSha256: digest(source), decisions: [...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]),
    boundary: 'Hosts retain the actual checked children selected by the shared traversal, bind caller inputs before effectful early defaults, and keep initializer scope, laziness and argument guards in the existing checker.' };
  for (const file of policyFiles) {
    if (check) assert.ok(isDeepStrictEqual(readProjectData(file), policy), 'the complete parameter default decision source is fresh');
    else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { writeRecordRuleLibrary } = await import('./record-rule-library.mjs');
    const emitted = emitRust(checkProgram(parseJavaScript(source, { referenceSemantics: false })), { arrayConsumption: true });
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: digest(emitted.text),
      assumptions: emitted.assumptions, encodings: emitted.encodings };
    if (check) assert.ok(isDeepStrictEqual(readProjectData(artifactFile), artifact), 'the actual translated parameter default module is fresh');
    else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'parameter_default_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateParameterDefaultRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Parameter defaults retain caller argument order before lazy callee initializers');
}
