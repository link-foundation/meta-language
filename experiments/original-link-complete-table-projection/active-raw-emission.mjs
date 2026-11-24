// The active compiler's source and archive checks are independent of the frozen
// 106-owner historical profiles and their exact emitted-text comparisons.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData, checkProjectData } from '../../js/scripts/project-data.mjs';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { ORIGINAL_LINK_RAW_EMISSION_BASELINE, ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT } from './raw-emission-profiles.mjs';

const maximumArchiveBytes = 64 * 1024 * 1024;
const maximumRecords = 1_000_000;
const artifactFile = new URL('./data/raw-active-source-emission.lino', import.meta.url);
const identity = text => ({ bytes: Buffer.byteLength(text), sha256: createHash('sha256').update(text).digest('hex') });
const orderedOwners = program => program.items.map(item => ({ kind: item.k, name: item.fullName, generated: item.generated === true }));

function emitProfiles(program) {
  return { javascript: emitJavaScript(program),
    rust: emitRust(program), consumingRust: emitRust(program, { arrayConsumption: true }) };
}

export async function generateActiveRawEmission({ check = false } = {}) {
  const source = readFileSync(new URL('./raw-rules.mjs', import.meta.url), 'utf8');
  const baseline = ORIGINAL_LINK_RAW_EMISSION_BASELINE;
  assert.deepEqual(identity(source), { bytes: baseline.sourceBytes, sha256: baseline.sourceSha256 });
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const owners = orderedOwners(program);
  assert.equal(owners.length, baseline.definitionCount);
  assert.equal(new Set(owners.map(owner => owner.name)).size, baseline.definitionCount);
  const original = readProjectData(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT);
  assert.deepEqual(owners.filter(owner => !owner.generated).map(owner => owner.name),
    original.declarationNames.filter(name => !name.startsWith('ml_')));
  const emitted = emitProfiles(program);
  for (const output of [emitted.rust, emitted.consumingRust]) {
    assert.equal(output.definitions.length, baseline.definitionCount);
    assert.equal(output.text.split(baseline.rootHeader).length, 2, 'the public root signature remains exact');
  }
  assert.notEqual(emitted.consumingRust.text, emitted.rust.text, 'the selected final-use optimization changes emitted Rust');
  const artifact = { version: 1, source: identity(source), owners,
    profiles: Object.entries(emitted).map(([profile, output]) => ({ profile, ...identity(output.text) })) };
  if (!check) return writeProjectData(artifactFile, artifact);

  checkProjectData(artifactFile, artifact);
  const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../../js/src/typed-program-links.js');
  const { recheckLinkedTypedProgram } = await import('../../js/src/typed-program-checked-projection.js');
  const { LinkNetwork } = await import('../../js/src/network.js');
  const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../../js/src/link-network-binary.js');
  const packet = encodeTypedProgramLinks(program);
  const recordCount = packet.network.links().length;
  assert.ok(recordCount > 0 && recordCount <= maximumRecords);
  const limits = { byteLimit: maximumArchiveBytes, recordLimit: recordCount };
  const text = packet.network.toLino();
  assert.ok(Buffer.byteLength(text) <= maximumArchiveBytes);
  const binary = encodeLinkNetworkBinary(packet.network, limits);
  assert.ok(binary.byteLength <= maximumArchiveBytes);
  for (const [format, replay] of [['text', () => LinkNetwork.fromLino(text)],
    ['binary', () => decodeLinkNetworkBinary(binary, limits)]]) {
    const network = replay();
    assert.equal(network.links().length, recordCount, `${format} preserves every Links record`);
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual(orderedOwners(restored), owners, `${format} preserves declaration identity and order`);
    assert.deepEqual(emitProfiles(restored), emitted, `${format} preserves complete emissions and source maps`);
    globalThis.gc?.();
  }
  return artifact;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const [mode] = process.argv.slice(2);
  assert.ok(process.argv.length === 3 && ['--draft', '--check'].includes(mode));
  await generateActiveRawEmission({ check: mode === '--check' });
}
