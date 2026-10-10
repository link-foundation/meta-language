import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import test from 'node:test';

import { acquireLease, liveLeases, resolveRepository } from '../../scripts/lib/cache-cleanup.mjs';
import { processIdentity, processIdentityAlive } from '../../scripts/lib/process-identity.mjs';
import { makeBase, makeRepository } from './support/cache-fixtures.js';

test('process identity detects exit and a reused PID without inheriting ownership', { skip: process.platform !== 'linux' }, () => {
  const own = processIdentity();
  assert.equal(own.pid, Number(readFileSync('/proc/self/stat', 'utf8').split(' ')[0]));
  assert.equal(processIdentityAlive(own), true);
  assert.equal(processIdentityAlive({ ...own, startTime: '0' }), false);
  assert.equal(processIdentityAlive({ ...own, bootId: 'another-boot' }), false);
  const module = new URL('../../scripts/lib/process-identity.mjs', import.meta.url).href;
  const child = spawnSync(process.execPath, ['--input-type=module', '-e', `import { processIdentity } from ${JSON.stringify(module)}; console.log(JSON.stringify(processIdentity()));`], { encoding: 'utf8' });
  assert.equal(child.status, 0, child.stderr);
  assert.equal(processIdentityAlive(JSON.parse(child.stdout)), false);
});

test('leases discard a reused PID while preserving another live process', { skip: process.platform !== 'linux' }, (t) => {
  const fixture = makeRepository(makeBase(t));
  const lease = acquireLease({ cwd: fixture.root, label: 'reused PID' });
  const owner = JSON.parse(readFileSync(lease.file, 'utf8'));
  assert.deepEqual({ pid: owner.pid, startTime: owner.startTime, bootId: owner.bootId }, processIdentity());
  writeFileSync(lease.file, JSON.stringify({ ...owner, startTime: '0' }));
  assert.deepEqual(liveLeases(resolveRepository(fixture.root).stateDir), []);
  assert.equal(existsSync(lease.file), false);
  const parent = acquireLease({ cwd: fixture.root, label: 'live parent', pid: process.ppid });
  t.after(() => parent.release());
  assert.deepEqual(liveLeases(resolveRepository(fixture.root).stateDir).map(({ label }) => label), ['live parent']);
});
