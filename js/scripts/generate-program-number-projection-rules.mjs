// Translate the complete portable module, never a selected function excerpt.
// Native output is explicitly gated after the JavaScript batch has passed.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const sourceFile = new URL('../src/typed-program-number-projection-rules.js', import.meta.url);
export const PROGRAM_NUMBER_PROJECTION_SOURCE = '/** @returns {number} */function probe(){const values=Array.from([2,4],(value,index)=>value+index);return values[1]+2;}console.log(probe());';

export function readProgramNumberProjectionSourceModule() {
  return readFileSync(sourceFile, 'utf8');
}

export async function generateProgramNumberProjectionRules({ check = false, native = false } = {}) {
  const source = readProgramNumberProjectionSourceModule();
  const policy = { schemaVersion: 1, model: 'checked optional arithmetic operand projection 1',
    source: 'js/src/typed-program-number-projection-rules.js',
    sourceSha256: createHash('sha256').update(source).digest('hex'),
    decisions: [...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]),
    boundary: 'Only the actual binary arithmetic left or right operand can project a validated derived number wrapper to its actual optional child. The unchanged checker must reconstruct and compare the complete original wrapper; there is no root projection.',
  };
  assert.deepEqual(policy.decisions, ['acceptProgramDerivedNumberOperand']);
  const file = new URL('../src/data/program-number-projection-rules.lino', import.meta.url);
  if (check) checkProjectData(file, policy);
  else writeProjectData(file, policy);
  const { createTypedProgramLinks } = await import('../src/typed-program-links.js');
  const original = createTypedProgramLinks(PROGRAM_NUMBER_PROJECTION_SOURCE, 'JavaScript');
  const fixture = { schemaVersion: 1, language: 'JavaScript', source: PROGRAM_NUMBER_PROJECTION_SOURCE,
    sourceSha256: createHash('sha256').update(PROGRAM_NUMBER_PROJECTION_SOURCE).digest('hex'),
    output: ['7'], editedOutput: ['12'], edit: { original: '2', replacement: '7' },
    links: original.network.toLino(), root: original.root.asU64() };
  const fixtureFile = new URL('../../parity/fixtures/program-number-projection.lino', import.meta.url);
  if (check) checkProjectData(fixtureFile, fixture);
  else writeProjectData(fixtureFile, fixture);
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const { writeRecordRuleLibrary } = await import('./record-rule-library.mjs');
    const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    assert.deepEqual([...program.declarations.keys()], policy.decisions, 'every declaration in the complete portable module remains checked');
    const packet = encodeTypedProgramLinks(program);
    const text = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    const binary = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root);
    const emitted = emitRust(program, { arrayConsumption: true });
    assert.equal(emitRust(text, { arrayConsumption: true }).text, emitted.text);
    assert.equal(emitRust(binary, { arrayConsumption: true }).text, emitted.text);
    const copy = new URL('../../rust/src/data/program-number-projection-rules.lino', import.meta.url);
    if (check) checkProjectData(copy, policy);
    else writeProjectData(copy, policy);
    writeRecordRuleLibrary(emitted, 'program_number_projection_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateProgramNumberProjectionRules({ check: process.argv.includes('--check'),
    native: process.argv.includes('--native') && !process.argv.includes('--javascript-only') });
  console.log('Derived numeric projection: complete arithmetic operand admission and checker reconstruction');
}
