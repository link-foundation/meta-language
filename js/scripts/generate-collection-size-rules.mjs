import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { decodeLinkNetworkBinary, encodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

export function generateCollectionSizeRules({ check = false } = {}) {
  const root = new URL('../../', import.meta.url);
  const sourcePath = 'js/src/translation/collection-size-rules.js';
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  const hash = text => createHash('sha256').update(text).digest('hex');
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const owners = program => [...program.declarations.values()].map(({ k, name, fullName }) => ({ k, name, fullName }));
  const inventory = owners(checked);
  assert.deepEqual(inventory.map(owner => owner.name), ['selectCollectionSizeField', 'renderCollectionSize']);
  const emitted = emitRust(checked, { arrayConsumption: true });
  const packet = encodeTypedProgramLinks(checked);
  const text = packet.network.toLino();
  const binary = encodeLinkNetworkBinary(packet.network);
  const replays = [['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)]].map(([format, network]) => {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual(owners(restored), inventory);
    const replay = emitRust(restored, { arrayConsumption: true });
    for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) {
      assert.deepEqual(replay[field], emitted[field]);
    }
    return { format, emittedTextSha256: hash(replay.text), inventory: owners(restored) };
  });
  const library = writeRecordRuleLibrary(emitted, 'collection_size_rules', { check });
  const artifact = { schemaVersion: 1, sourcePath, source, sourceSha256: hash(source), inventory, replays,
    archive: { root: packet.root.asU64(), textSha256: hash(text), binarySha256: hash(binary) },
    library: library.map(file => ({ path: file.path ?? 'collection_size_rules.rs', sourceSha256: hash(file.source) })) };
  const artifactPath = new URL('parity/self-translation/collection-size-rules-native.lino', root);
  if (check) checkProjectData(artifactPath, artifact);
  else writeProjectData(artifactPath, artifact);
  return artifact;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.length === 2 || process.argv[2] === '--check' && process.argv.length === 3);
  generateCollectionSizeRules({ check: process.argv[2] === '--check' });
}
