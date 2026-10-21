// Unexecuted private draft. Run only after the semantic JS slice passes and
// Root runs the normal generator against that exact applied source batch.
import assert from 'node:assert/strict';
import test from 'node:test';
import { generateJavaScriptRecordParameterRules } from '../scripts/generate-javascript-record-parameter-rules.mjs';
import { LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/index.js';

const archives = network => [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];

test('the entire recorded dependency closure and actual generated ABI retain both honest archives', async () => {
  const result = await generateJavaScriptRecordParameterRules({ check: true, native: true });
  assert.deepEqual(result.artifact.replays.map(row => row.format), ['text', 'binary']);
  assert.equal(result.artifact.checkedNamedSourceInventory.length, result.artifact.inventory.length);
  assert.ok(result.artifact.libraryInterface.length >= result.policy.decisions.length);
  for (const owner of result.policy.originalDeclarations) {
    const concrete = LinkNetwork.parse(owner.declaration, 'JavaScript');
    assert.ok(concrete.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing));
    for (const network of archives(concrete)) assert.deepEqual(Buffer.from(network.reconstructText()), Buffer.from(owner.declaration));
    assert.ok(owner.context.includes(owner.declaration));
  }
});
