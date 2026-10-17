// Translate the complete original construction decisions and their required
// fact readers. Native algorithms are never supplied by this producer.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/collection-construction-rules.js';
const factPath = 'js/src/translation/fact-column-rules.js';
const factImport = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n";
const hash = source => createHash('sha256').update(source).digest('hex');

export function readCollectionConstructionSourceModule() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.equal(source.split(factImport).length, 2, 'the complete source retains its one actual fact dependency');
  const facts = readFileSync(new URL(factPath, root), 'utf8');
  const body = source.replace(factImport, '');
  assert.doesNotMatch(body, /^\s*import\b/mu, 'every actual source import belongs to the recorded closure');
  return facts + '\n' + body;
}

export function collectionConstructionRulesPolicy() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  const closure = readCollectionConstructionSourceModule();
  return { schemaVersion: 1, model: 'ordered primitive collection construction 1', sourcePath,
    source, sourceSha256: hash(source), sourceBytes: Buffer.byteLength(source),
    sourceClosure: { source: closure, sourceSha256: hash(closure), bytes: Buffer.byteLength(closure),
      removedImports: [factImport], inputs: [factPath, sourcePath].map(path => {
        const text = readFileSync(new URL(path, root), 'utf8');
        return { path, source: text, sourceSha256: hash(text), bytes: Buffer.byteLength(text) };
      }) },
    inference: 'Original expected element, key and value owners are retained. The ordinary engine allocates fresh terms only when the shared plan requests them, evaluates the actual iterable once and applies the original array or pair constraints.',
    admission: 'Set requires an array of primitive keys; Map an array of two-element key/value tuples with primitive keys. Actual primitive eligibility comes from the existing optional primitive decision. Original type and unsupported diagnostics remain distinct.',
    rendering: 'All four original templates retain insertion order, Map last-value replacement, SameValueZero NaN equality and positive-zero key normalization. Rocq comparison/type helpers and ordinary host evaluation keep their original order.',
    boundary: 'Constructor decisions do not admit mutable collection operations or structured key identities. Actual native compilation and execution remain CI observations.' };
}

export async function generateCollectionConstructionRules({ check = false, native = false } = {}) {
  const policy = collectionConstructionRulesPolicy();
  for (const path of ['js/src/translation/data/collection-construction-rules.lino', 'rust/src/data/collection-construction-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy);
    else writeProjectData(new URL(path, root), policy);
  }
  let artifact = null;
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const surface = parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false });
    const checked = checkProgram(surface);
    const inventory = surface.items.map((item, ordinal) => ({ ordinal, kind: item.k, name: item.name, generated: item.generated === true }));
    const owners = program => inventory.map(item => {
      const declaration = program.declarations.get(item.name);
      assert.ok(declaration && declaration.k === item.kind, 'every complete source/continuation owner remains present');
      return { ...item, fullName: declaration.fullName,
        ...(declaration.modulePath === undefined ? {} : { modulePath: declaration.modulePath }) };
    });
    const checkedNamedSourceInventory = owners(checked);
    assert.equal(checkedNamedSourceInventory.length, checked.declarations.size);
    const emitted = emitRust(checked, { arrayConsumption: true });
    const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino();
    const binary = encodeLinkNetworkBinary(packet.network), replays = [];
    for (const [format, network] of [['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.deepEqual(owners(restored), checkedNamedSourceInventory);
      const replay = emitRust(restored, { arrayConsumption: true });
      for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) assert.deepEqual(replay[field], emitted[field]);
      replays.push({ format, checkedNamedSourceInventory: owners(restored), emittedTextBytes: Buffer.byteLength(replay.text), emittedTextSha256: hash(replay.text) });
    }
    for (const owner of checkedNamedSourceInventory) assert.equal(emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind)).length, 1, 'one complete actual native owner mapping');
    const files = writeRecordRuleLibrary(emitted, 'collection_construction_rules', { check });
    artifact = { schemaVersion: 1, model: policy.model, sourceClosure: policy.sourceClosure, inventory,
      checkedNamedSourceInventory, replays,
      emitted: { text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes,
        mappings: emitted.mappings, assumptions: emitted.assumptions, encodings: emitted.encodings },
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: hash(emitted.text),
      archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: hash(text), binaryBytes: binary.byteLength, binarySha256: hash(binary) },
      library: { files: files.map(file => ({ path: file.path ?? 'collection_construction_rules.rs', bytes: Buffer.byteLength(file.source), sourceSha256: hash(file.source) })) },
      boundary: 'Complete source and archive replay evidence; native compilation and execution require actual CI controls.' };
    const output = new URL('parity/self-translation/collection-construction-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)));
  await generateCollectionConstructionRules({ check: args.includes('--check'), native: args.includes('--native') });
}
