import { readFileSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/record-value-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/record-values.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/record-values.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/record-value-rules-native.lino', import.meta.url);
const hash = text => createHash('sha256').update(text).digest('hex');
const requiredImport = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n";
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);

// Resolve the one actual module edge using its complete implementation. This
// never substitutes a native body or drops unselected dependency declarations.
export function readRecordValueSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(requiredImport).length !== 2) throw new Error('the record planner retains one exact required-column import');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

function originalMethod() {
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
  const source = execFileSync('git', ['show', 'HEAD:js/src/translation/check.js'], { encoding: 'utf8' });
  const start = source.indexOf('  recordLiteral(node, env, path, expected) {');
  const end = source.indexOf('\n  method(node, env, path, expected) {', start);
  if (start < 0 || end < start) throw new Error('the original complete record checker method is unavailable');
  const method = source.slice(start, end).trim();
  if (!method.includes('const fields = new Map();') || !method.includes('bindings.reverse()')) throw new Error('the authoring baseline no longer contains the original record algorithm');
  return { path: 'js/src/translation/check.js', revision, sourceSha256: hash(source), method, methodSha256: hash(method) };
}

function originalInference() {
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
  const source = execFileSync('git', ['show', 'HEAD:js/src/translation/javascript-infer.js'], { encoding: 'utf8' });
  const method = (startText, endText) => {
    const start = source.indexOf(startText);
    const end = source.indexOf(endText, start);
    if (start < 0 || end < start) throw new Error('an original complete record inference method is unavailable');
    return source.slice(start, end).trim();
  };
  const record = method('  record(node, env, expected) {', '\n  methodStep(');
  const unify = method('  unify(left, right, where) {', '\n  contains(');
  if (!record.includes('const fields = new Map();') || !unify.includes('for (const field of a.fields)')) throw new Error('the authoring baseline no longer contains the original record inference algorithms');
  return { path: 'js/src/translation/javascript-infer.js', revision, sourceSha256: hash(source),
    record, recordSha256: hash(record), unify, unifySha256: hash(unify) };
}

function originalOwnerMethods(inference) {
  const source = execFileSync('git', ['show', inference.revision + ':' + inference.path], { encoding: 'utf8' });
  if (hash(source) !== inference.sourceSha256) throw new Error('the original record inference source hash differs');
  const start = source.indexOf('  contains(term, id, seen = new Set()) {');
  const middle = source.indexOf('\n  addInferredField(', start);
  const end = source.indexOf('\n  run(', middle);
  if (start < 0 || middle < start || end < middle) throw new Error('the original complete record owner methods are unavailable');
  const contains = source.slice(start, middle).trim();
  const add = source.slice(middle, end).trim();
  const fieldStart = source.indexOf('  fieldStep({ object, field, result, where }) {');
  const fieldEnd = source.indexOf('\n  /** The inferred type as a surface type;', fieldStart);
  if (fieldStart < 0 || fieldEnd < fieldStart) throw new Error('the original complete record field inference method is unavailable');
  const field = source.slice(fieldStart, fieldEnd).trim();
  return { path: inference.path, revision: inference.revision, sourceSha256: hash(source),
    contains, containsSha256: hash(contains), add, addSha256: hash(add), field, fieldSha256: hash(field) };
}

export function recordValueRulesPolicy(baseline, inferenceBaseline, ownerBaseline) {
  const source = readFileSync(sourceFile, 'utf8');
  const stored = baseline ? null : readProjectData(policyFile);
  const original = baseline ?? stored.original;
  const inference = inferenceBaseline ?? stored?.originalInference;
  const owner = ownerBaseline ?? stored?.originalOwner;
  if (hash(original.method) !== original.methodSha256) throw new Error('the original record checker method hash differs');
  if (!inference || hash(inference.record) !== inference.recordSha256 || hash(inference.unify) !== inference.unifySha256) throw new Error('the original record inference method hashes differ');
  if (!owner || hash(owner.contains) !== owner.containsSha256 || hash(owner.add) !== owner.addSha256 || hash(owner.field) !== owner.fieldSha256) throw new Error('the original record owner method hashes differ');
  return {
    schemaVersion: 1,
    model: 'ordered own record field values 1',
    original,
    originalInference: inference,
    originalOwner: owner,
    source,
    sourceSha256: hash(source),
    sourceClosure: {
      source: readRecordValueSourceModule(source),
      sourceSha256: hash(readRecordValueSourceModule(source)),
      removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/record-value-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factsFile, 'utf8')) }],
    },
    instructions: {
      evaluation: 'each original entry once in ascending source order',
      binding: 'wrap the checked body in descending entry order',
      replacement: 'last occurrence supplies the final field while every overwritten value evaluates',
      layout: 'ascending lexicographic UTF16 code units with prefix before extension',
      spread: 'one checked receiver binding followed by its actual ordered field projections',
      inference: 'retain actual shared record and variable identities; re-read fields after each recursive constraint and refuse a field reaching its actual owner',
    },
    numericDomain: { count: 'nonnegative safe integer', unitMinimum: 0, unitMaximum: 65535 },
    boundary: 'The plan returns original positions. Existing child checking, source owners, reference allocation and target rendering remain mandatory; callable values and native execution require their independent implementations.',
  };
}

export async function generateRecordValueRules({ check = false, native = false } = {}) {
  const baseline = !check && !existsSync(policyFile) ? originalMethod() : undefined;
  const inferenceBaseline = !check && (!existsSync(policyFile) || !readProjectData(policyFile).originalInference) ? originalInference() : undefined;
  const ownerBaseline = !check && (!existsSync(policyFile) || !readProjectData(policyFile).originalOwner?.field)
    ? originalOwnerMethods(inferenceBaseline ?? readProjectData(policyFile).originalInference) : undefined;
  const policy = recordValueRulesPolicy(baseline, inferenceBaseline, ownerBaseline);
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('the shared record value rules are stale');
    } else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    // Translate the entire actual shared source. Do not substitute a native
    // planner or extract only a convenient function from the module.
    const program = checkProgram(parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false }));
    const emitted = emitRust(program, { arrayConsumption: true });
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256,
      sourceClosure: policy.sourceClosure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text),
      assumptions: emitted.assumptions, encodings: emitted.encodings };
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('the complete automatically translated record rules are stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'record_value_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateRecordValueRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Record value rules: complete shared source and retained original checker authenticated');
}
