import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/record-emission-rules.js', import.meta.url);
const policyFile = new URL('../src/translation/data/record-emission.lino', import.meta.url);
const nativePolicyFile = new URL('../../rust/src/data/record-emission.lino', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/record-emission-rules-native.lino', import.meta.url);
const hash = text => createHash('sha256').update(text).digest('hex');
const requiredImport = "import { readRequiredTextFact } from './fact-column-rules.js';\n";
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);

export function readRecordEmissionSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(requiredImport).length !== 2) throw new Error('the record renderer retains one exact required-column import');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

export function recordEmissionRulesPolicy() {
  const source = readFileSync(sourceFile, 'utf8');
  return {
    schemaVersion: 1,
    model: 'ordered record value emission 1',
    source,
    sourceSha256: hash(source),
    sourceClosure: {
      source: readRecordEmissionSourceModule(source),
      sourceSha256: hash(readRecordEmissionSourceModule(source)),
      removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/record-emission-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factsFile, 'utf8')) }],
    },
    targets: ['JavaScript', 'Rust', 'Lean', 'Rocq'],
    boundary: 'The existing checker supplies the ordered field layout, existing emitters render every actual child once, and the JavaScript source-string adapter quotes field labels. Target syntax does not replace declaration, binding, reference or type validation.',
  };
}

export async function generateRecordEmissionRules({ check = false, native = false } = {}) {
  const policy = recordEmissionRulesPolicy();
  for (const file of [policyFile, nativePolicyFile]) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('the shared record emission rules are stale');
    } else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const { LinkNetwork } = await import('../src/network.js');
    const program = checkProgram(parseJavaScript(policy.sourceClosure.source, { referenceSemantics: false }));
    const packet = encodeTypedProgramLinks(program);
    const textProgram = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    const binaryProgram = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root);
    const emitted = emitRust(textProgram);
    if (emitRust(binaryProgram).text !== emitted.text) throw new Error('record emission rules differ after complete text and binary Links reload');
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256,
      sourceClosure: policy.sourceClosure,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text),
      assumptions: emitted.assumptions, encodings: emitted.encodings };
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('the complete automatically translated record emission rules are stale');
    } else writeProjectData(artifactFile, artifact);
    writeRecordRuleLibrary(emitted, 'record_emission_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateRecordEmissionRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Record emission rules: shared complete source authenticated');
}
