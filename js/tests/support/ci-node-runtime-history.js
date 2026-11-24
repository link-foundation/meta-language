import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../src/structured-data-links.js';

const bytes=readFileSync(new URL('../../../parity/naming/ci-node-runtime-source-history.lino',import.meta.url));
assert.equal(createHash('sha256').update(bytes).digest('hex'),'ca3566e0989eb61227ce2785c6f8a77bce9949bcb944847fabd2815b4a58279a');
const history=readStructuredDataLino(bytes.toString('utf8'));

export function restoreCiNodeRuntimeHistory(source) {
  assert.equal(typeof source,'string');
  source=restoreTranslationWorkflowHistory(source);
  source=restorePlatformTestDeadlineHistory(source);
  const owners = history.owners.filter(owner => source.startsWith(owner.header + '\n'));
  assert.equal(owners.length, 1, 'one original workflow header is required');
  assert.equal(/^\s*node-version: 24$/mu.test(source), false, 'floating selections cannot coexist with exact runtime pins');

  // Later jobs have their own audited pins and do not enter the historical count.
  const addedJobs = ['rust-source-versions', 'rust-state-source-originals'];
  let originalPins = 0;
  const jobs = source.split(/(?=^  [a-z][a-z0-9-]*:\n)/mu);
  const priorRuntime = jobs.map(job => {
    const name = /^  ([a-z][a-z0-9-]*):\n/u.exec(job)?.[1];
    const pin = /^( +node-version: )24\.22\.0$/gmu;
    if (addedJobs.includes(name)) {
      assert.equal([...job.matchAll(pin)].length, 1, name + ' keeps its exact Node patch');
      return job;
    }
    return job.replace(pin, (_match, prefix) => {
      originalPins += 1;
      return prefix + '24';
    });
  }).join('');
  assert.equal(originalPins, owners[0].count, 'every original major-24 setup has the exact supported patch');
  return priorRuntime;
}

import {restorePlatformTestDeadlineHistory} from './platform-test-deadline-history.js';
import {restoreTranslationWorkflowHistory} from './translation-workflow-history.js';
