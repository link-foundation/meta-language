import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import test from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { restoreGeneratedPackageOwnerHistory } from './support/generated-package-owner-history.js';

test('actual generated scheduling preserves every check once under both original deadlines after a measured timeout', () => {
  const evidence = readProjectData(new URL('../../parity/fixtures/generated-package-owner-history.lino', import.meta.url));
  const current = restoreCiNodeRuntimeHistory(readFileSync(new URL('../../' + evidence.path, import.meta.url), 'utf8'));
  assert.equal(current, evidence.after.source);
  assert.equal(restoreGeneratedPackageOwnerHistory(current), evidence.before.source);
  const before = parseWorkflow(evidence.before.source).find(job => job.id === 'package-checks');
  const after = parseWorkflow(current).find(job => job.id === 'package-checks');
  assert.equal(after.header, before.header, 'every matrix cell, display name, cancellation key and deadline remains exact');
  assert.equal(after.steps.length, before.steps.length);
  assert.deepEqual(after.steps.map(step => step.name), before.steps.map(step => step.name));
  const expected = ['Run Rust owned local container checks', 'Check strict public declaration consumers',
    'Authenticate reviewed generated-stage migration refreshes'];
  assert.deepEqual(evidence.changes.map(change => change.name), expected);
  for (const [index, step] of after.steps.entries()) {
    const previous = before.steps[index];
    if (expected.includes(step.name)) assert.equal(step.text,
      previous.text.replace("matrix.generated_slice == 'second'", "matrix.generated_slice == 'first'"));
    else assert.equal(step.text, previous.text, 'every unrelated command and predicate remains exact');
  }
  const first = after.steps.filter(step => step.text.includes("matrix.generated_slice == 'first'")).map(step => step.name);
  assert.deepEqual(first, ['Check generated owned Rust literal value edit experiment rules',
    'Check complete generated owned Rust literal value edit fixtures', 'Run generated owned Rust literal value edit experiment', ...expected]);
  const second = after.steps.filter(step => step.text.includes("matrix.generated_slice == 'second'")).map(step => step.name);
  assert.equal(new Set([...first, ...second]).size, first.length + second.length);
  assert.equal(first.length + second.length, before.steps.filter(step => step.text.includes('matrix.generated_slice')).length);
  assert.deepEqual(second.slice(-3), ['Check generated frontend rule compilation and style',
    'Check generated global concept rule compilation and style', 'Retain exact generated sources and compiler feedback']);
  for (const response of Object.values(evidence.responses)) {
    assert.equal(Buffer.byteLength(response.raw), response.bytes);
    assert.equal(createHash('sha256').update(response.raw).digest('hex'), response.sha256);
    assert.deepEqual(JSON.parse(response.raw), response.data);
  }
  assert.equal(evidence.responses.first.data.id, 118143113860);
  assert.equal(evidence.responses.first.data.conclusion, 'success');
  assert.equal(evidence.responses.second.data.id, 118143114075);
  assert.equal(evidence.responses.second.data.conclusion, 'cancelled');
  assert.ok(evidence.responses.annotations.data.some(row => row.message === 'The job has exceeded the maximum execution time of 10m0s'));
  assert.equal(Buffer.byteLength(evidence.log.source), 253635);
  assert.equal(createHash('sha256').update(evidence.log.source).digest('hex'),
    'b9262aecfaba8c6bc0d9f3b1445eb406fc6ba77a107949b9f4a4140433b99b2e');
  for (const change of evidence.changes) for (const replacement of ['', change.before, change.after + change.after,
    change.after.replace('!cancelled()', 'always()'), change.after.includes('working-directory:')
      ? change.after.replace('working-directory:', 'removed-directory:')
      : change.after.replace('        run:', '        working-directory: wrong\n        run:'),
    change.after.replace('run:', 'removed-command:'), change.after.replace("generated_slice == 'first'", "generated_slice == 'first' || true")]) {
    assert.notEqual(replacement, change.after);
    assert.throws(() => restoreGeneratedPackageOwnerHistory(current.replace(change.after, replacement)), change.name);
  }
  assert.throws(() => restoreGeneratedPackageOwnerHistory(evidence.before.source));
});

import {restoreCiNodeRuntimeHistory} from './support/ci-node-runtime-history.js';

test('a third required generated slice keeps every command and feedback producer under the ten-minute deadline', () => {
  const current = readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const jobs = parseWorkflow(current);
  const checks = jobs.find(job => job.id === 'package-checks');
  const extraCell = '          - group: generated\n            generated_slice: third\n';
  const extraLabel = " || matrix.generated_slice == 'third' && ' / third'";
  assert.equal(checks.header.split(extraCell).length, 2, 'the overloaded generated slice gains one distinct required peer');
  assert.equal(checks.header.split(extraLabel).length, 2, 'the extra slice has its own check name');
  assert.match(checks.header, /timeout-minutes: 10\n/u);
  assert.match(checks.header, /-\$\{\{ matrix\.generated_slice \}\}\n/u);
  const moved = [
    'Check generated JavaScript Unit module experiment rules',
    'Check complete generated JavaScript Unit module experiment fixtures',
    'Run generated JavaScript Unit module experiment',
    'Check generated owned Rust module experiment rules',
    'Check complete generated owned Rust module experiment fixtures',
    'Run generated owned Rust module experiment',
    'Check generated Rust text value projection experiment rules',
    'Check complete Rust text value projection experiment fixtures',
    'Run Rust text value projection experiment',
    'Check shared owned occurrence position fixtures',
    'Run owned occurrence position checks',
    'Authenticate reviewed generated-stage migration refreshes',
  ];
  const thirdSlice = "matrix.generated_slice == 'third'";
  assert.deepEqual(checks.steps.filter(step => step.text.includes(thirdSlice)).map(step => step.name), moved);
  const previousHeader = checks.header.replace(extraCell, '').replace(extraLabel, '');
  const rawCheckoutSetup = [
    '          # Keep commit ancestry for fresh merge simulation and fetch old blobs on demand.',
    "          filter: ${{ matrix.group == 'source-model-raw' && 'blob:none' || '' }}",
    '          # Defer the retained development logs and the two naming-only inventories.',
    "          sparse-checkout: ${{ matrix.group == 'source-model-raw' && "
      + String.raw`fromJSON('"/*\n!/dev/log/\n!/parity/naming/canonical-concepts.lino\n!/parity/naming/source-bindings.lino"') || '' }}`,
    '          sparse-checkout-cone-mode: false',
    '',
  ].join('\n');
  const previousSteps = checks.steps.map(step => {
    if (step.name === '- uses: actions/checkout@v7') {
      assert.equal(step.text.split(rawCheckoutSetup).length, 2, 'only raw planning uses the blobless checkout');
      return step.text.replace(rawCheckoutSetup, '');
    }
    if (!moved.includes(step.name)) return step.text;
    assert.equal(step.text.split(thirdSlice).length, 2);
    assert.ok(step.afterFailure);
    const oldSlice = step.name === 'Authenticate reviewed generated-stage migration refreshes' ? 'first' : 'second';
    return step.text.replace(thirdSlice, `matrix.generated_slice == '${oldSlice}'`);
  });
  // The timed-out run used 897b1f6b2ff7df5e094ca5d420b9c6e0057ef030.
  assert.equal(createHash('sha256').update(previousHeader + previousSteps.join('')).digest('hex'),
    '715f120d38b81b51a9b6cad39f496d1f2a3cc67be4f19410e7c3519335b58de6',
    'the schedule preserves every original command, setup step, artifact and predicate');
  const aggregate = jobs.find(job => job.id === 'test');
  const gate = aggregate.steps.find(step => step.name === 'Require every JavaScript test group');
  assert.equal(createHash('sha256').update(aggregate.header + gate.text).digest('hex'),
    'eeb8ab8db81f053f173fe98e55e847a990e04fabd038907e2ea287c614e8c7a6',
    'every generated slice feeds the same required package gate');
});

test('the raw planning checkout defers unrelated blobs and still merges and reads full history', () => {
  const workflow = readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const job = parseWorkflow(workflow).find(row => row.id === 'package-checks');
  const checkout = job.steps.find(row => row.name === '- uses: actions/checkout@v7').text;
  const patternInput = /^          sparse-checkout: \$\{\{ matrix\.group == 'source-model-raw' && fromJSON\(('.*')\) \|\| '' \}\}$/mu.exec(checkout);
  assert.ok(patternInput, 'only the full raw planning cell defers its unrelated current blobs');
  assert.match(checkout, /^          sparse-checkout-cone-mode: false$/mu);
  const patterns = JSON.parse(patternInput[1].slice(1, -1));
  const omitted = ['dev/log/retained.log', 'parity/naming/canonical-concepts.lino', 'parity/naming/source-bindings.lino'];
  const required = [
    'js/package-lock.json',
    'scripts/with-cache-cleanup.mjs',
    'js/src/data/concept-records.lino',
    'parity/fixtures/raw-native-contract-partitions.lino',
    'parity/grammars/native/rust.lino',
    'parity/naming/unicode-string-order-source-history.lino',
    'parity/self-translation/complete-declaration-receipts.lino',
    'experiments/original-link-complete-table-projection/raw-native-observations.test.mjs',
    'rust/src/data/concept-records.lino',
  ];
  const directory = mkdtempSync(join(tmpdir(), 'raw-planning-checkout-'));
  try {
    const git = (cwd, ...args) => execFileSync('git', args, {
      cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'],
    }).trim();
    const origin = join(directory, 'origin');
    const planner = join(directory, 'planner');
    git(directory, 'init', '--initial-branch=main', origin);
    git(origin, 'config', 'user.name', 'Raw Planning Test');
    git(origin, 'config', 'user.email', 'raw-planning@example.test');
    git(origin, 'config', 'uploadpack.allowFilter', 'true');
    git(origin, 'config', 'uploadpack.allowAnySHA1InWant', 'true');
    for (const file of [...omitted, ...required]) {
      mkdirSync(dirname(join(origin, file)), { recursive: true });
      writeFileSync(join(origin, file), file + '\n');
    }
    git(origin, 'add', '--force', '.');
    git(origin, 'commit', '-m', 'Retain planner and historical inputs');
    const ancestor = git(origin, 'rev-parse', 'HEAD');
    git(origin, 'checkout', '-b', 'planner');
    writeFileSync(join(origin, omitted[0]), 'Changed log on the planner branch\n');
    git(origin, 'commit', '-am', 'Change unrelated planner worktree input');
    git(origin, 'checkout', 'main');
    writeFileSync(join(origin, required[0]), 'Latest base lock\n');
    git(origin, 'commit', '-am', 'Advance the merge base');
    const base = git(origin, 'rev-parse', 'HEAD');

    git(directory, 'clone', '--filter=blob:none', '--no-checkout', pathToFileURL(origin).href, planner);
    const plannerGit = (...args) => git(planner, ...args);
    plannerGit('config', 'core.sparseCheckout', 'true');
    plannerGit('config', 'core.sparseCheckoutCone', 'false');
    writeFileSync(join(planner, '.git/info/sparse-checkout'), patterns + '\n');
    plannerGit('checkout', 'planner');
    const head = plannerGit('rev-parse', 'HEAD');
    for (const file of required) assert.equal(readFileSync(join(planner, file), 'utf8'), file + '\n');
    for (const file of omitted) assert.equal(existsSync(join(planner, file)), false, file);
    const missingBlobs = plannerGit('rev-list', '--objects', '--missing=print', '--all').split('\n');
    for (const file of omitted) assert.ok(missingBlobs.includes('?' + plannerGit('rev-parse', `${head}:${file}`)), file);
    const mergeScript = fileURLToPath(new URL('../scripts/simulate-fresh-merge.sh', import.meta.url));
    const merged = execFileSync('bash', [mergeScript], {
      cwd: planner, env: { ...process.env, BASE_REF: 'main' }, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'],
    });
    assert.match(merged, /Fresh merge simulation succeeded/u);
    plannerGit('merge-base', '--is-ancestor', base, 'HEAD');
    plannerGit('merge-base', '--is-ancestor', head, 'HEAD');
    assert.equal(plannerGit('rev-parse', '--is-shallow-repository'), 'false');
    assert.equal(readFileSync(join(planner, required[0]), 'utf8'), 'Latest base lock\n');
    for (const file of omitted) {
      assert.equal(plannerGit('show', `${ancestor}:${file}`), file);
      assert.equal(existsSync(join(planner, file)), false, file);
    }
    assert.equal(plannerGit('status', '--porcelain'), '');
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
