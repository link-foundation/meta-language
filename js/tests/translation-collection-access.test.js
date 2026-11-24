// Future controls only; intentionally not executed during the whole code draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
const fixture = readProjectData(new URL('../../parity/fixtures/collection-access.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + fixture.versions.JavaScript);
const execute = source => runInNewContext(source.replace(/^export /gmu, '') + '\ncompute();', {}, { timeout: 1000 });
for (const input of fixture.cases) test(input.name + ': independent builtins and four-target Links replay', () => {
  const expected = execute(input.source), program = checkProgram(parseJavaScript(input.source));
  assert.equal(execute(emitJavaScript(program).text), expected);
  const packet = encodeTypedProgramLinks(program);
  for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    const original = emitTypedProgramLinks(packet.network, packet.root, target);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      assert.deepEqual(emitTypedProgramLinks(network, packet.root, target), original);
    }
  }
});
