import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { decodeLinkNetworkBinary, encodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

export function generateSelfTranslationRelativeArrayRules({ check = false } = {}) {
  const source = readFileSync(new URL('../src/self-translation-relative-array-rules.js', import.meta.url), 'utf8');
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  assert.deepEqual([...program.declarations.keys()], ['acceptSelfTranslationRelativeArrayCall']);
  const emitted = emitRust(program, { arrayConsumption: true });
  const packet = encodeTypedProgramLinks(program);
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual(emitRust(restored, { arrayConsumption: true }), emitted);
  }
  return writeRecordRuleLibrary(emitted, 'self_translation_relative_array_rules', { check });
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.length === 2 || process.argv.length === 3 && process.argv[2] === '--check');
  generateSelfTranslationRelativeArrayRules({ check: process.argv[2] === '--check' });
}
