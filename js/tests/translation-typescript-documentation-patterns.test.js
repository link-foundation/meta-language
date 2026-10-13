import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { test } from 'node:test';
import vm from 'node:vm';
import { projectTypeScriptTyping } from '../src/index.js';
import { projectTypeScriptTyping as projectTypeScriptTypingDetails } from '../src/typescript-portable-typing.js';
import { readProjectData } from '../scripts/project-data.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/typescript-documentation-patterns.lino', import.meta.url));

test('bounded TypeScript documentation fixtures reproduce from their registered generator', () => {
  const result = spawnSync(process.execPath,
    [new URL('../scripts/generate-typescript-documentation-fixtures.mjs', import.meta.url).pathname, '--check'],
    { encoding: 'utf8', timeout: 30_000 });
  assert.equal(result.status, 0, result.stderr);
});

test('projected parameter patterns retain independent JavaScript values and original byte provenance', () => {
  for (const entry of fixture.cases) {
    if (entry.expectedError) {
      assert.throws(() => projectTypeScriptTyping(entry.source),
        (error) => error.reason === entry.expectedError, entry.id);
      continue;
    }
    const code = projectTypeScriptTyping(entry.source);
    assert.equal(code, entry.projectedSource, entry.id);
    assert.equal(new vm.Script(`${code}\n${entry.invocation}`).runInNewContext({}, { timeout: 1_000 }),
      entry.expectedValue, entry.id);
    const details = projectTypeScriptTypingDetails(entry.source);
    assert.deepEqual(details.projections, entry.projections, entry.id);
    for (const projection of details.projections) {
      for (const pattern of projection.parameterPatterns) {
        assert.equal(Buffer.from(entry.source).subarray(pattern.sourceSpan.start, pattern.sourceSpan.end).toString(),
          pattern.sourceText, entry.id);
        assert.ok(code.includes(pattern.sourceText), 'the executable pattern remains in the source');
        assert.ok(!pattern.documentationText.includes('*/'), 'supplemental pattern text cannot close its comment');
      }
    }
    assert.equal((code.match(/\r\n/gu) ?? []).length, (entry.source.match(/\r\n/gu) ?? []).length,
      'original carriage returns and line feeds remain intact');
  }
});

test('ambiguous supplemental spellings preserve distinct authoritative executable patterns', () => {
  const literal = fixture.cases.find(({ id }) => id === 'quoted-comment-terminator');
  const escaped = fixture.cases.find(({ id }) => id === 'existing-escaped-slash');
  const pattern = (entry) => entry.projections[0].parameterPatterns[0];
  assert.equal(pattern(literal).documentationText, pattern(escaped).documentationText);
  assert.notEqual(pattern(literal).sourceText, pattern(escaped).sourceText);
  assert.notEqual(projectTypeScriptTyping(literal.source), projectTypeScriptTyping(escaped.source));
});
