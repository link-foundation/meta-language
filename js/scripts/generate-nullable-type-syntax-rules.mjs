import { existsSync, readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/nullable-type-syntax-rules.js', import.meta.url);
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/nullable-type-syntax.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/nullable-type-syntax.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/nullable-type-syntax-rules-native.lino', import.meta.url);
const requiredImport = "import { readRequiredTextFact } from './fact-column-rules.js';\n";
const hash = source => createHash('sha256').update(source).digest('hex');

export function readNullableTypeSyntaxSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(requiredImport).length !== 2) throw new Error('nullable type syntax retains its exact required-column source import');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

function originalCompositeType() {
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
  const path = 'js/src/translation/javascript-bindings.js';
  const source = execFileSync('git', ['show', 'HEAD:' + path], { encoding: 'utf8' });
  const start = source.indexOf('export function readCompositeType(');
  const end = source.indexOf('\nexport function isLiteralBinding(', start);
  if (start < 0 || end < start) throw new Error('the complete original nullable type reader is unavailable');
  const method = source.slice(start, end).trim();
  if (!method.includes("const missing = alternatives.filter((part) => part === 'null' || part === 'undefined');")) throw new Error('the original nullable alternative reader is no longer the authoring baseline');
  return { path, revision, sourceSha256: hash(source), source: method, sha256: hash(method) };
}

export function nullableTypeSyntaxRulesPolicy(original) {
  const source = readFileSync(sourceFile, 'utf8');
  original ??= readProjectData(policyFile).original;
  if (hash(original.source) !== original.sha256) throw new Error('the retained original nullable type reader hash differs');
  const closed = readNullableTypeSyntaxSourceModule(source);
  return { schemaVersion: 1, model: 'source nullable type syntax 1', original, source, sourceSha256: hash(source),
    sourceClosure: { source: closed, sourceSha256: hash(closed), removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/nullable-type-syntax-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factsFile, 'utf8')) }] },
    decisions: ['retain every ordered union alternative', 'fold exactly one value with at least one missing token', 'multiple missing tokens retain the original both policy including repeated identical missing tokens', 'retain standalone absence and nullable prefix', 'no union coercion or target absence representation'],
    boundary: 'Common outer separators, actual UTF16 slices/trim and recursive child type checking remain mandatory. Standalone absence remains outside the legacy checked Links type admission; this source grammar does not grant new runtime optional-value or union coercion semantics.' };
}

export async function generateNullableTypeSyntaxRules({ check = false, native = false } = {}) {
  const original = !check && !existsSync(policyFile) ? originalCompositeType() : undefined;
  const policy = nullableTypeSyntaxRulesPolicy(original);
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('nullable type syntax policy is stale');
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
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('complete translated nullable type syntax source is stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'nullable_type_syntax_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateNullableTypeSyntaxRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Nullable type syntax: retained original reader and complete checked source');
}
