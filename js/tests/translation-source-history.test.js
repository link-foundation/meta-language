import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {restoreTranslationSourceHistory} from '../scripts/translation-source-history.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';

const root = new URL('../../', import.meta.url);
const policy = readProjectData(new URL('parity/self-translation/translation-source-history.lino', root));

test('reviewed source intervals restore every immutable original byte without replacing historical assertion inputs', () => {
  assert.equal(policy.sources.length, 6);
  let intervals = 0;
  for (const row of policy.sources) {
    const current = readFileSync(new URL(row.file, root), 'utf8');
    const original = execFileSync('git', ['show', row.revision + ':' + row.file], {cwd: root, encoding: 'utf8', maxBuffer: 1024 * 1024});
    assert.equal(row.previous.sourceText, original);
    assert.equal(restoreTranslationSourceHistory(current, row.file, row.revision), original);
    assert.equal(restoreTranslationSourceHistory(original, row.file, row.revision), original);
    assert.equal(createHash('sha256').update(original).digest('hex'), row.previous.sha256);
    intervals += row.edits.length;
  }
  assert.equal(intervals, 16);
  const previous=readStructuredDataLino(execFileSync('git', ['show',
    '6af83c50596d6d267c506539bf5d5d266bba2ee7:parity/self-translation/translation-source-history.lino'],
    {cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
  assert.equal(previous.sources.reduce((sum,row)=>sum+row.edits.length,0),15);
  assert.deepEqual(policy.sources.map(({file,revision,previous})=>({file,revision,previous})),
    previous.sources.map(({file,revision,previous})=>({file,revision,previous})), 'every earlier immutable source profile remains exact');
});

test('source history refuses edits inside and outside approved intervals unknown owners and unregistered revisions', () => {
  let refused = 0;
  for (const row of policy.sources) {
    const current = readFileSync(new URL(row.file, root), 'utf8');
    for (const changed of [current + '\n', '// unreviewed\n' + current, current.replace('\n', '\n ')]) {
      assert.notEqual(changed, current);
      assert.throws(() => restoreTranslationSourceHistory(changed, row.file, row.revision), assert.AssertionError);
      refused++;
    }
    if (row.edits.length) {
      const bytes = Buffer.from(current), first = row.edits[0];
      const changed = Buffer.concat([bytes.subarray(0, first.start), Buffer.from('// changed interval\n'), bytes.subarray(first.start)]).toString();
      assert.throws(() => restoreTranslationSourceHistory(changed, row.file, row.revision), assert.AssertionError);
      refused++;
    }
    for (const [file, revision] of [[row.file + '.other', row.revision], [row.file, '0'.repeat(40)]]) {
      assert.throws(() => restoreTranslationSourceHistory(current, file, revision), /one exact recorded source/u);
      refused++;
    }
  }
  assert.equal(refused, 35);
  assert.throws(() => restoreTranslationSourceHistory('\ud800', policy.sources[0].file, policy.sources[0].revision), /valid UTF8/u);
  assert.throws(() => restoreTranslationSourceHistory('x'.repeat(1024 * 1024 + 1), policy.sources[0].file, policy.sources[0].revision), /bounded complete source/u);
});
