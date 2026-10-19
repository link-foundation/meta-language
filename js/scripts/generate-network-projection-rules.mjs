// Draft/debug the complete JavaScript kernel before requesting its native
// translation. Both packages keep the same inspectable policy and fixtures.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/network-projection-rules.js', import.meta.url);

export function readNetworkProjectionSourceModule() {
  return readFileSync(sourceFile, 'utf8');
}

export async function generateNetworkProjectionRules({ check = false, native = false } = {}) {
  const source = readNetworkProjectionSourceModule();
  const policy = {
    schemaVersion: 1,
    model: 'original link network projection membership 1',
    source: 'js/src/network-projection-rules.js',
    sourceSha256: createHash('sha256').update(source).digest('hex'),
    decisions: ['acceptNetworkProjection', 'acceptNetworkProjectionLink'],
    defaultProjection: 'ConcreteSyntax',
    identity: 'Each selected result is the existing source Link, with its original identifier, references and metadata.',
    provenance: 'References resolve against the retained complete source network, including source tokens and trivia omitted from the selected view.',
    scope: 'Finite projection API parity only. This neither changes default parsing nor proves complete concrete syntax or catalog language coverage.',
  };
  const policyFile = new URL('../src/data/network-projection-rules.lino', import.meta.url);
  if (check) checkProjectData(policyFile, policy);
  else writeProjectData(policyFile, policy);

  // Independent, explicit expectations for every current public link role
  // plus absent metadata. Preserve the existing Rust projection membership.
  const fixture = {
    schemaVersion: 1,
    linkTypes: ['Dynamic', 'Reference', 'Relation', 'Language', 'Grammar', 'Type', 'Concept',
      'Syntax', 'Field', 'Trivia', 'SourceToken', 'Document', 'Semantic', 'Region', 'Object', null],
    cases: [
      { projection: 'Lossless', included: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] },
      { projection: 'ConcreteSyntax', included: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16] },
      { projection: 'AbstractSyntax', included: [1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 14, 15, 16] },
      { projection: 'Semantic', included: [4, 6, 7, 13] },
    ],
  };
  const fixtureFile = new URL('../../parity/fixtures/network-projection-expectations.lino', import.meta.url);
  if (check) checkProjectData(fixtureFile, fixture);
  else writeProjectData(fixtureFile, fixture);

  if (native) {
    const nativePolicy = new URL('../../rust/src/data/network-projection-rules.lino', import.meta.url);
    if (check) checkProjectData(nativePolicy, policy);
    else writeProjectData(nativePolicy, policy);
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    assert.deepEqual([...program.declarations.keys()].sort(), [...policy.decisions].sort(),
      'the complete source retains both actual decision declarations');
    const emitted = emitRust(program, { arrayConsumption: true });
    assert.equal(emitted.definitions.length, program.declarations.size,
      'every complete checked definition remains in the generated library');
    writeRecordRuleLibrary(emitted, 'network_projection_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  await generateNetworkProjectionRules({
    check,
    native: !process.argv.includes('--javascript-only') && (check || process.argv.includes('--native')),
  });
  console.log('Network projection rules: original-link membership and retained source provenance');
}
