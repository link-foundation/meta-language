// Future CI controls, intentionally unexecuted during the complete draft phase.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

const fixtures = readProjectData(new URL('../../parity/fixtures/array-state-methods.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + fixtures.versions.JavaScript);
const execute = source => runInNewContext(source.replace(/^export /gmu, '') + '\ncompute();', {}, { timeout: 1000 });
for (const fixture of fixtures.cases) test(fixture.name + ': actual builtin semantics and complete typed codecs', () => {
  const expected = execute(fixture.source);
  const program = checkProgram(parseJavaScript(fixture.source));
  assert.equal(execute(emitJavaScript(program).text), expected);
  const rust = emitRust(program), packet = encodeTypedProgramLinks(program);
  assert.ok(rust.definitions.length > 0);
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const checked = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.equal(execute(emitJavaScript(checked).text), expected);
    const emitted = emitRust(checked);
    for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) assert.deepEqual(emitted[field], rust[field]);
  }
});

for (const fixture of fixtures.refusals) test(fixture.name + ' retains its explicit state obligation', () => {
  assert.throws(() => emitRust(checkProgram(parseJavaScript(fixture.source))), error => error.message.includes(fixture.reason));
});
