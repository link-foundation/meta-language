import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { DecoratorSet } from '../../js/src/decorators.js';
import { readCompilerFeedbackCorrections } from '../../js/scripts/compiler-style-feedback.mjs';
import { readProjectData } from '../../js/scripts/project-data.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const startingCommit = '38b367159fbe5a20cbcc19a8d75741875113a4b5';
const original = file => execFileSync('git', ['--no-replace-objects', 'show', `${startingCommit}:${file}`], { cwd: root });
const [directory, mode] = process.argv.slice(2);
assert.ok(directory && [undefined, '--check'].includes(mode));

for (const { group, file, allowedPaths, count } of [
  { group: 'frontend-rules', file: 'parity/self-translation/frontend-rules-decorators.lino',
    allowedPaths: ['rust/src/translation/frontend_rules/continuations.rs'], count: 6 },
  { group: 'global-concept-rules', file: 'parity/self-translation/global-concept-rules-decorators.lino',
    allowedPaths: ['rust/src/global_concept_rules.rs'], count: 2 },
]) {
  const feedback = readProjectData(join(directory, group, 'round-0.lino'));
  const { actions, spans, deferred, inputs } = readCompilerFeedbackCorrections({ feedback, allowedPaths });
  assert.equal(actions.length, count);
  assert.equal(spans, count);
  assert.equal(deferred, 0);
  for (const [diagnosticPath, bytes] of inputs) {
    assert.deepEqual(bytes, original(feedback.sources[diagnosticPath].relativePath));
  }
  const prefix = original(file);
  assert.equal(prefix.at(-1), 10);
  const appendix = new DecoratorSet([{
    id: 'compiler-feedback-continuation-style', level: 'emitter', order: 1001,
    when: [['format', 'Rust'], ['scope', 'formatted-module']], actions,
  }]).toLino();
  const output = Buffer.concat([prefix, Buffer.from(`\n${appendix}`)]);
  const address = join(root, file);
  if (mode === '--check') assert.deepEqual(readFileSync(address), output);
  else writeFileSync(address, output);
  console.log(`Retained all ${prefix.length} original ${group} decorator bytes and appended ${count} compiler suggestions.`);
}
