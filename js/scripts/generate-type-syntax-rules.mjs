import { readFileSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/type-syntax-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/type-syntax.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/type-syntax.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/type-syntax-rules-native.lino', import.meta.url);
const hash = text => createHash('sha256').update(text).digest('hex');
const requiredImport = "import { readRequiredNumberFact } from './fact-column-rules.js';\n";
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);

export function readTypeSyntaxSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(requiredImport).length !== 2) throw new Error('the type scanner retains one exact required-column import');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

function originalSeparator() {
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
  const source = execFileSync('git', ['show', 'HEAD:js/src/translation/javascript-bindings.js'], { encoding: 'utf8' });
  const start = source.indexOf('export function splitType(text, separator) {');
  const end = source.indexOf('\nexport function readCompositeType(', start);
  if (start < 0 || end < start) throw new Error('the complete original type separator is unavailable');
  const original = source.slice(start, end).trim();
  if (!original.includes("'{[<('.includes(text[at])") || !original.includes("text[at - 1] === '='")) throw new Error('the original separator baseline no longer contains the actual scan');
  return { path: 'js/src/translation/javascript-bindings.js', revision, sourceSha256: hash(source), source: original, sha256: hash(original) };
}

export function typeSyntaxRulesPolicy(baseline) {
  const source = readFileSync(sourceFile, 'utf8');
  const original = baseline ?? readProjectData(policyFile).original;
  if (hash(original.source) !== original.sha256) throw new Error('the retained original type separator hash differs');
  return { schemaVersion: 1, model: 'outer composite type separators 1', source, sourceSha256: hash(source), original,
    sourceClosure: { source: readTypeSyntaxSourceModule(source), sourceSha256: hash(readTypeSyntaxSourceModule(source)), removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/type-syntax-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factsFile, 'utf8')) }] },
    boundary: 'Positions retain UTF16 source coordinates and the original bracket and arrow behavior. Host slice/trim and existing recursive type parsing remain mandatory; this scanner does not prove arbitrary type syntax or union coercion.' };
}

export async function generateTypeSyntaxRules({ check = false, native = false } = {}) {
  const baseline = !check && !existsSync(policyFile) ? originalSeparator() : undefined;
  const policy = typeSyntaxRulesPolicy(baseline);
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('the shared composite type syntax rules are stale');
    } else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false })));
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256,
      sourceClosure: policy.sourceClosure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text),
      assumptions: emitted.assumptions, encodings: emitted.encodings };
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('the complete automatically translated type syntax rules are stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'type_syntax_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateTypeSyntaxRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Type syntax rules: whole source and retained original separator authenticated');
}
