// JavaScript-only normal generation checks; Rust execution belongs to CI.
import assert from 'node:assert/strict';
import test from 'node:test';
import {generateArrayClassificationRules} from '../scripts/generate-array-classification-rules.mjs';
test('the complete shared array classification source generates every native owner and identical text and binary replays',async()=>{
  const {policy,artifact}=await generateArrayClassificationRules({check:true,native:true});
  assert.equal(policy.decisions.length,2);assert.equal(artifact.replays.length,2);
  assert.deepEqual(artifact.replays.map(row=>row.format),['text','binary']);
  for(const decision of policy.decisions)assert.equal(artifact.libraryInterface.filter(row=>row.source===decision).length,1);
  for(const replay of artifact.replays)assert.deepEqual(replay.checkedNamedSourceInventory,artifact.checkedNamedSourceInventory);
});
