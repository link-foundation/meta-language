// Normal complete-source author; native algorithms come only from the checked
// JavaScript translator and the existing recorded library style writer.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { readDeclarationDependencyModuleFixtures, readOrdinaryDeclarationModuleFixtures, ordinaryModuleOwnerSource } from './declaration-dependency-fixtures.mjs';
import { readDeclarationModuleOwnerFixtures, readDeclarationSpecialGlobalSources, declarationSpecialGlobalInventory } from './declaration-module-owner-fixtures.mjs';
import { tokenize } from '../src/translation/lexer.js';
import { deriveDeclarationDependencyNativeInterface } from './declaration-dependency-native-interface.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const root = new URL('../../', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');
const sourcePath = 'js/src/translation/declaration-dependency-rules.js';
const factPath = 'js/src/translation/fact-column-rules.js';
const boundary = "import { readRequiredNumberFact } from './fact-column-rules.js';\n";
const decisions = ['acceptDeclarationDependencyFacts', 'readDeclarationDependencyComponents',
  'acceptRecursiveDeclarationGroup', 'acceptRecursiveDeclarationPublication', 'acceptDeclarationModuleScope', 'acceptDeclarationSignatureBinding'];

export function readDeclarationDependencySourceClosure() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  const facts = readFileSync(new URL(factPath, root), 'utf8');
  assert.equal(source.split(boundary).length, 2, 'one actual complete fact-column import');
  const start = source.indexOf(boundary), end = start + boundary.length;
  const body = source.slice(0, start) + source.slice(end);
  assert.doesNotMatch(body, /^\s*import\b/mu, 'no unknown import is dropped');
  const combined = facts + '\n\n' + body;
  return { source: combined, bytes: Buffer.byteLength(combined), sourceSha256: digest(combined), inputs: [
    { path: factPath, source: facts, bytes: Buffer.byteLength(facts), sourceSha256: digest(facts), removedImports: [] },
    { path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source),
      removedImports: [{ start, end, source: boundary, coordinateUnit: 'UTF-16 code units' }] },
  ] };
}

export async function generateDeclarationDependencyRules({ check = false, native = false } = {}) {
  const closure = readDeclarationDependencySourceClosure();
  assert.deepEqual([...closure.inputs[1].source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]), decisions);
  const policy = { schemaVersion: 1, model: 'atomic checked recursive declaration groups 1', sourcePath, decisions,
    sourceClosure: closure,
    moduleFixtures: readDeclarationDependencyModuleFixtures(),
    ordinaryModuleFixtures: readOrdinaryDeclarationModuleFixtures(),
    ordinaryModuleOwnerSource,
    moduleOwnerFixtures: readDeclarationModuleOwnerFixtures(),
    specialGlobalSources: readDeclarationSpecialGlobalSources(),
    specialGlobalInventory: declarationSpecialGlobalInventory,
    assertionVersions: { Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0' },
    interpretation: 'Dense source-group dependency candidates determine scheduling only. Every original body is checked together before signatures are obtained; every original member is rechecked and emitted with those actual peer signatures. Only the final complete batch publishes. The ordinary binder remains the fallback, without provisional peer signatures. Ordinary dependency and public/importable signatures additionally require actual certified module scope and exactly one actual binding owner; isolated outputs alone do not prove those module bindings.',
    boundary: 'Both new atomic groups and legacy partial-scope retries require an actually known module surface: actual named function/import groups, comments, and pure primitive literals certified by the existing complete literal proof. A wider form requires the existing parser/checker to check the complete original module with actual translated import signatures. Unknown bindings, writes or evaluation are never omitted. New atomic admission additionally requires ordinary synchronous hoisted named function declarations with unique actual owners, flat checked declarations, complete member signatures, and matching per-member target mappings. Constructor errors and other unsupported constructs remain obligations. Exact empty named class declarations with no inheritance or members are inert module-form facts only; their actual class names participate in ownership counts while class translation stays unsupported. Duplicate actual owners refuse before either module-scope proof route. Unknown module context cannot certify ordinary/public binding signatures; preserved isolated item outputs make no whole-module behavior claim.' };
  for (const path of ['js/src/translation/data/declaration-dependency-rules.lino', 'rust/src/data/declaration-dependency-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy); else writeProjectData(new URL(path, root), policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { writeRecordRuleLibrary } = await import('./record-rule-library.mjs');
    const emitted = emitRust(checkProgram(parseJavaScript(closure.source, { referenceSemantics: false })), { arrayConsumption: true });
    const files = writeRecordRuleLibrary(emitted, 'declaration_dependency_rules', { check });
    const libraryInterface = files.flatMap(file => {
      const tokens = tokenize(file.source, 'Rust').tokens;
      return tokens.flatMap((token, index) => {
        if (token.value !== 'fn' || tokens[index + 1]?.kind !== 'identifier') return [];
        const name = tokens[index + 1].value;
        const owner = emitted.mappings.find(mapping => mapping.target === name && mapping.kind === 'function');
        if (!owner) return [];
        let body = index + 2;
        while (body < tokens.length && tokens[body].value !== '{') body += 1;
        assert.ok(body < tokens.length, 'the actual generated function owns its complete header');
        return [{ source: owner.source, target: name, path: file.path ?? 'declaration_dependency_rules.rs',
          signature: file.source.slice(token.start, tokens[body].start).trim() }];
      });
    });
    for (const decision of decisions) assert.equal(libraryInterface.filter(row => row.source === decision).length, 1);
    const transportSource = formatProductRuleSource(deriveDeclarationDependencyNativeInterface(files, libraryInterface));
    const transportPath = 'rust/src/self_translation/binding/dependency_rules.rs';
    const transportFile = new URL(transportPath, root);
    if (check) assert.equal(readFileSync(transportFile, 'utf8'), transportSource, 'the actual generated native interface transport is fresh');
    else {
      mkdirSync(new URL('.', transportFile), { recursive: true });
      writeFileSync(transportFile, transportSource);
    }
    const artifact = { schemaVersion: 1, model: policy.model, sourceClosure: closure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: digest(emitted.text),
      assumptions: emitted.assumptions, encodings: emitted.encodings, mappings: emitted.mappings,
      libraryInterface, files: files.map(file => ({ path: file.path ?? 'declaration_dependency_rules.rs', source: file.source,
        bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })),
      nativeTransport: { path: transportPath, source: transportSource, bytes: Buffer.byteLength(transportSource), sourceSha256: digest(transportSource) },
      boundary: 'Complete automatic native generation and actual exported ABI inventory only; no native compilation or execution observation.' };
    const file = new URL('parity/self-translation/declaration-dependency-rules-native.lino', root);
    if (check) checkProjectData(file, artifact); else writeProjectData(file, artifact);
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateDeclarationDependencyRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Declaration dependencies retain complete checked recursive groups and atomic signature publication');
}
