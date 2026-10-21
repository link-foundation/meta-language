import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { runModernArrayParityAssertions } from './support/modern-javascript-array-parity.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

test('assertion corrections execute retained historical inputs and the applicable replacement expectations', context => {
  const request = readFileSync(new URL('../../parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt', import.meta.url));
  assert.equal(createHash('sha256').update(request).digest('hex'),
    '5d4a7d12bfb3d97937ec7c2ab2a5d60729d27a65e7ca53ec0cd62381402d21d2');
  const instructions = readFileSync(new URL('../../AGENTS.md', import.meta.url), 'utf8');
  assert.match(instructions, /Correct wrong assertions with documented evidence, retaining inputs and\s+coverage/u);
  assert.match(instructions, /Never weaken requirements, oracles or gates to conceal failures/u);
  assert.match(instructions, /Every PR check must pass; implement obligations and fix failures/u);
  assert.match(instructions, /Claim completion or add closing directives only when every ledger row passes/u);

  // This executes every retained original input, every replacement expectation,
  // every genuine refusal and all fixture tampering controls. The helper also
  // authenticates the complete historical Rust source and byte-exact corrected
  // test bodies, rather than using an instruction or a fixture label as proof.
  const execution = runModernArrayParityAssertions();
  assert.deepEqual(execution.cases.filter(row => row.originalStdout !== undefined).map(row => row.id), [
    'original-array-corpus', 'original-empty-array-inference', 'original-data-array',
    'original-array-identity-refusal', 'original-string-length', 'supplementary-string-length', 'observable-array-absence',
  ]);
  assert.equal(execution.cases.length, 11);
  assert.equal(execution.metadata.admitted, 7);
  assert.equal(execution.metadata.refused, 4);
  assert.equal(execution.metadata.nodeVersion, process.versions.node);
  for (const row of execution.cases) {
    for (const { descriptor } of row.targets) {
      if (row.originalStdout !== undefined) assert.equal(descriptor.diagnostic, null);
      else assert.ok(descriptor.diagnostic, 'a corrected expectation never drops a genuine refusal');
    }
  }

  recordIssue195Observations({
    requirementId: 'I195-ASSERTION-EVIDENCE-BASED-CORRECTION', runtime: 'tooling', suffix: 'behavior',
    fixtureId: 'conversation-contract:compactContributorInstructionsRequest',
    fixtureFile: 'parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt',
    assertions: ['wrongAssertionCorrectionSupportedByEvidence', 'originalInputsAndCoverageRetained',
      'correctedApplicableAssertionsExecuted'],
    testName: context.name,
  });
});
