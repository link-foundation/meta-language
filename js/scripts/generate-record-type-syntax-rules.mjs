import { existsSync, readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/record-type-syntax-rules.js', import.meta.url);
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/record-type-syntax.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/record-type-syntax.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/record-type-syntax-rules-native.lino', import.meta.url);
const requiredImport = "import { readRequiredNumberFact } from './fact-column-rules.js';\n";
const hash = source => createHash('sha256').update(source).digest('hex');

export function readRecordTypeSyntaxSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(requiredImport).length !== 2) throw new Error('record type syntax retains its exact required-column source import');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

function originalCompositeType() {
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
  const path = 'js/src/translation/javascript-bindings.js';
  const source = execFileSync('git', ['show', 'HEAD:' + path], { encoding: 'utf8' });
  const start = source.indexOf('export function readCompositeType(');
  const end = source.indexOf('\nexport function isLiteralBinding(', start);
  if (start < 0 || end < start) throw new Error('the complete original composite type reader is unavailable');
  const method = source.slice(start, end).trim();
  if (!method.includes("const index = part.indexOf(':');")) throw new Error('the original record field reader is no longer the authoring baseline');
  return { path, revision, sourceSha256: hash(source), source: method, sha256: hash(method) };
}

export function recordTypeSyntaxRulesPolicy(original) {
  const source = readFileSync(sourceFile, 'utf8');
  original ??= readProjectData(policyFile).original;
  if (hash(original.source) !== original.sha256) throw new Error('the retained original composite type reader hash differs');
  return { schemaVersion: 1, model: 'source record type field syntax 1', original, source, sourceSha256: hash(source),
    sourceClosure: { source: readRecordTypeSyntaxSourceModule(source), sourceSha256: hash(readRecordTypeSyntaxSourceModule(source)), removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/record-type-syntax-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factsFile, 'utf8')) }] },
    boundary: 'Exact record boundaries, first field colon and optional suffix decisions retain the existing reader. Common outer separators, source UTF16 slices/trim and recursive child type checking remain mandatory; no optional property omission or union coercion is granted.' };
}

export async function generateRecordTypeSyntaxRules({ check = false, native = false } = {}) {
  const original = !check && !existsSync(policyFile) ? originalCompositeType() : undefined;
  const policy = recordTypeSyntaxRulesPolicy(original);
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('record type syntax policy is stale');
    } else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false })));
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256, sourceClosure: policy.sourceClosure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text), assumptions: emitted.assumptions, encodings: emitted.encodings };
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('complete translated record type syntax source is stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'record_type_syntax_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateRecordTypeSyntaxRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Record type syntax: retained original reader and complete checked source');
}
