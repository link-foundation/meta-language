// Close the real fact-column import and translate the complete decision module.
// Native generation is deferred until the whole JavaScript batch has passed.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import { JAVASCRIPT_ARRAY_BINDING_POLICY } from '../src/translation/array-binding-policy.js';
import { accepted, refused } from '../tests/fixtures/array-binding-cases.mjs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export function readArrayBindingSourceModule(source = readFileSync(new URL('../src/translation/array-binding-rules.js', import.meta.url), 'utf8')) {
  const boundary = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';";
  assert.equal(source.split(boundary).length, 2, 'the complete array-binding module owns its one recorded dependency');
  const dependency = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8');
  return dependency + '\n' + source.replace(boundary, '');
}

export async function generateArrayBindingRules({ check = false, javascriptOnly = false } = {}) {
  const input = new URL('../src/translation/array-binding-rules.js', import.meta.url);
  const source = readFileSync(input, 'utf8');
  const closedSource = readArrayBindingSourceModule(source);
  const policy = { ...JAVASCRIPT_ARRAY_BINDING_POLICY,
    source: 'js/src/translation/array-binding-rules.js',
    sourceSha256: createHash('sha256').update(source).digest('hex'),
    closedSourceSha256: createHash('sha256').update(closedSource).digest('hex'),
    decisions: [...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]),
  };
  const file = new URL('../src/translation/data/array-bindings.lino', import.meta.url);
  if (check) checkProjectData(file, policy);
  else writeProjectData(file, policy);
  const fixtures = { schemaVersion: 1, model: policy.model, accepted, refused };
  const fixtureFile = new URL('../../parity/fixtures/javascript-array-bindings.lino', import.meta.url);
  if (check) checkProjectData(fixtureFile, fixtures);
  else writeProjectData(fixtureFile, fixtures);
  if (!javascriptOnly) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const { writeRecordRuleLibrary } = await import('./record-rule-library.mjs');
    const { writeFileSync } = await import('node:fs');
    const checked = checkProgram(parseJavaScript(closedSource, { referenceSemantics: false }));
    const packet = encodeTypedProgramLinks(checked);
    const text = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    const binary = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root);
    const emitted = emitRust(checked, { arrayConsumption: true });
    assert.equal(emitRust(text, { arrayConsumption: true }).text, emitted.text, 'source-free text keeps every complete decision');
    assert.equal(emitRust(binary, { arrayConsumption: true }).text, emitted.text, 'source-free binary keeps every complete decision');
    const copy = new URL('../../rust/src/data/array-bindings.lino', import.meta.url);
    if (check) assert.ok(readFileSync(copy).equals(readFileSync(file)), 'both runtimes share exact array-binding policy bytes');
    else writeFileSync(copy, readFileSync(file));
    writeRecordRuleLibrary(emitted, 'array_binding_rules', { check });
  }
}

if (process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url) {
  await generateArrayBindingRules({ check: process.argv.includes('--check'), javascriptOnly: process.argv.includes('--javascript-only') });
  console.log('Array bindings: ordered suffix reads and undefined-only primitive defaults');
}
