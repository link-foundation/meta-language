import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { createSelfTranslationReplayPlan, rankSelfTranslationBlockers,
  requireReportSourcePath, validateSelfTranslationReplayPlan } from '../scripts/self-translation-report-blockers.mjs';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const commit = '1'.repeat(40);
const hash = text => createHash('sha256').update(text).digest('hex');

// These are authored diagnostic controls, not observations of repository files.
function authoredReport(module, source, reason) {
  const item = { term: 'class_declaration', start: 0, end: Buffer.byteLength(source), status: 'carried', reason };
  item.diagnostic = { kind: 'unsupported', reason, message: `${reason} at 0..${item.end}`,
    span: { start: 0, end: item.end }, details: { construct: 'authored class control' }, sourceRange: { start: 0, end: item.end } };
  const translation = { sourceLanguage: 'JavaScript', targetLanguage: 'Rust', items: [item] };
  const coverage = { sourceBytes: item.end, itemBytes: item.end, layoutBytes: 0, unrepresentedBytes: 0,
    unrepresentedRanges: [], bytesByStatus: { carried: item.end } };
  const pin = { module, sourceSha256: hash(source), rust: null, rustSha256: null };
  const row = { ...pin, items: { carried: 1, translated: 0, comment: 0 }, coverage, translation,
    decorated: { coverage, translation } };
  return { schemaVersion: 1, commit, modules: [row], failures: [], inventory: [pin] };
}

test('blocker ranking retains exact reasons, individual ranges, modes, failures and the complete source denominator', () => {
  const reports = [authoredReport('js/src/first.js', 'class First {}', 'unsupported class'),
    authoredReport('js/src/second.js', 'class Second {}', 'unsupported class'),
    authoredReport('js/src/large.js', ' '.repeat(500), 'unsupported larger item'),
    authoredReport('js/src/different.js', 'class Different {}', 'unsupported class at 2:3')];
  const failure = { module: 'js/src/refused.js', error: 'source coverage refused', assertions: ['retain exact source'] };
  reports.push({ schemaVersion: 1, commit, modules: [], failures: [failure],
    inventory: [{ module: failure.module, sourceSha256: hash('refused'), rust: null, rustSha256: null }] });
  const sourceCatalog = [...reports.flatMap(report => report.inventory.map(row => row.module)), 'js/src/not-measured.js'].sort();
  for (const report of reports) report.sourceCatalog = sourceCatalog;
  const original = structuredClone(reports);
  const ranked = rankSelfTranslationBlockers(reports);
  assert.deepEqual(reports, original, 'analysis retains all input measurements without mutation');
  assert.deepEqual(ranked, rankSelfTranslationBlockers(reports.slice().reverse()), 'shard ordering does not affect ranking');
  assert.equal(ranked.commit, commit);
  assert.equal(ranked.groups[0].moduleCount, 2, 'distinct affected modules precede a larger single item');
  assert.equal(ranked.groups[0].reason, 'unsupported class');
  assert.equal(ranked.groups[0].sourceBytes, Buffer.byteLength('class First {}class Second {}'));
  assert.deepEqual(ranked.groups.find(group => group.reason === 'unsupported class' && group.mode === 'generic')
    .occurrences.map(row => row.item), reports.slice(0, 2).map(report => report.modules[0].translation.items[0]));
  assert.ok(ranked.groups.some(group => group.reason === 'unsupported class at 2:3'));
  assert.ok(ranked.groups.some(group => group.mode === 'decorated' && group.reason === 'unsupported class'));
  assert.deepEqual(ranked.failures, [failure]);
  assert.equal(ranked.groups.find(group => group.kind === 'module-error').occurrences[0].sourceSha256, hash('refused'));
  assert.deepEqual(ranked.unreportedModules, ['js/src/not-measured.js']);
  assert.equal(ranked.inventory.length, 5);
  assert.deepEqual(ranked.diagnosticGaps, []);
});

test('legacy measurements expose missing individual reasons instead of inventing zero blockers or full coverage', () => {
  const report = authoredReport('js/src/legacy.js', 'class Old {}', 'original unsupported reason');
  delete report.inventory;
  delete report.modules[0].translation;
  delete report.modules[0].decorated.translation;
  const blockers = rankSelfTranslationBlockers([report]);
  assert.equal(blockers.inventory[0].sourceSha256, report.modules[0].sourceSha256);
  assert.deepEqual(blockers.groups, []);
  assert.equal(blockers.diagnosticGaps.length, 2);
  assert.equal(blockers.diagnosticGaps.find(row => row.mode === 'generic').carriedItems, 1);
  assert.equal(blockers.sourceCatalog, null);
  assert.equal(blockers.unreportedModules, null, 'old reports do not declare a complete source catalog');
});

test('precise report diagnostics retain actual frontend errors while default translation output remains exact', () => {
  const source = '// café 😀\nexport class Unsupported {}\n';
  const ordinary = selfTranslate(source, 'JavaScript', 'Rust');
  const detailed = selfTranslate(source, 'JavaScript', 'Rust', { retainDiagnostics: true });
  let original;
  try { parseJavaScript(source, { readonlyModuleConstants: true }); } catch (error) { original = error; }
  assert.ok(original, 'the independent frontend rejects this exact unsupported export');
  assert.equal(detailed.code, ordinary.code);
  assert.deepEqual(detailed.items.map(({ diagnostic, ...item }) => item), ordinary.items);
  assert.ok(ordinary.items.every(item => !Object.hasOwn(item, 'diagnostic')));
  for (const item of detailed.items.filter(item => item.status === 'carried')) {
    assert.deepEqual(item.diagnostic, { kind: original.kind, reason: original.reason, message: original.message,
      span: original.span, details: original.details,
      sourceRange: { start: 0, end: Buffer.byteLength(source.trimEnd()) } });
    assert.ok(item.diagnostic.sourceRange.start <= item.start && item.diagnostic.sourceRange.end >= item.end);
  }
  assert.deepEqual(selfTranslate(source, 'JavaScript', 'Rust', { retainDiagnostics: false }), ordinary);
  const first = authoredReport('js/src/first.js', 'class First {}', 'unsupported');
  const second = authoredReport('js/src/second.js', 'class Second {}', 'unsupported');
  first.modules[0].translation.items[0].diagnostic.reason = 'export class: only portable declarations are exported';
  second.modules[0].translation.items[0].diagnostic.reason = 'namespace import: the namespace object needs faithful encoding';
  const groups = rankSelfTranslationBlockers([first, second]).groups;
  assert.equal(groups.filter(group => group.mode === 'generic').length, 2, 'broad unsupported kinds do not conflate distinct blockers');
  assert.ok(groups.every(group => group.diagnosticKind === 'unsupported' && group.term === 'class_declaration'));
  assert.deepEqual(groups.find(group => group.reason.startsWith('export class') && group.mode === 'generic')
    .occurrences[0].item.diagnostic, first.modules[0].translation.items[0].diagnostic);
});

test('blocker analysis refuses lost, duplicate, mixed-commit and inconsistent source outcomes', () => {
  const original = authoredReport('js/src/first.js', 'class First {}', 'unsupported class');
  for (const [mutate, message] of [
    [report => { report.modules = []; }, /no outcome/u],
    [report => { report.failures.push({ module: report.modules[0].module, error: 'duplicate' }); }, /duplicate selected outcome/u],
    [report => { report.inventory.push(report.inventory[0]); }, /duplicate report module/u],
    [report => { report.modules[0].sourceSha256 = '0'.repeat(64); }, /source hash/u],
    [report => { report.modules[0].items.carried = 0; }, /carried item count/u],
    [report => { report.modules[0].translation.items[0].end += 1; }, /retained item range/u],
    [report => { report.modules[0].translation.items[0].reason = null; }, /exact reason/u],
  ]) {
    const report = structuredClone(original);
    mutate(report);
    assert.throws(() => rankSelfTranslationBlockers([report]), message);
  }
  const later = authoredReport('js/src/second.js', 'class Second {}', 'unsupported class');
  later.commit = '2'.repeat(40);
  assert.throws(() => rankSelfTranslationBlockers([original, later]), /recorded commit/u);
});

test('selected replay retains every origin diagnostic and requires exact language, mode, paths and current source bytes', () => {
  const sources = new Map([['js/src/first.js', 'class First {}'], ['js/src/second.js', 'class Second {}']]);
  const reports = [...sources].map(([module, source]) => authoredReport(module, source, 'unsupported class'));
  const blockers = rankSelfTranslationBlockers(reports);
  const plan = createSelfTranslationReplayPlan(blockers, [blockers.groups[0].key]);
  assert.deepEqual(plan.origin, blockers);
  assert.deepEqual(validateSelfTranslationReplayPlan(plan, [...sources.keys()], module => sources.get(module)), [...sources.keys()]);
  assert.throws(() => validateSelfTranslationReplayPlan(plan, [...sources.keys()], () => 'changed'), /source hash changed/u);
  assert.throws(() => validateSelfTranslationReplayPlan(plan, ['js/src/first.js'], module => sources.get(module)), /missing from the source inventory/u);
  for (const [mutate, message] of [
    [value => { value.sourceLanguage = 'Rust'; }, /JavaScript to Rust/u],
    [value => { value.targetLanguage = 'JavaScript'; }, /JavaScript to Rust/u],
    [value => { value.mode = 'generic'; }, /both existing measurements/u],
    [value => { value.originCommit = '2'.repeat(40); }, /origin commit differs/u],
    [value => { value.sources.pop(); }, /selection differs/u],
  ]) {
    const changed = structuredClone(plan);
    mutate(changed);
    assert.throws(() => validateSelfTranslationReplayPlan(changed, [...sources.keys()], module => sources.get(module)), message);
  }
  for (const module of ['js/src/../outside.js', 'js/src/./first.js', 'js/src//first.js', '/tmp/first.js', 'js/src\\first.js', 'js/src/first.js\0']) {
    assert.throws(() => requireReportSourcePath(module), /inside js\/src/u);
  }
  assert.throws(() => createSelfTranslationReplayPlan(blockers, ['missing']), /unknown blocker/u);
  assert.throws(() => createSelfTranslationReplayPlan(blockers, []), /one or more distinct/u);
  assert.throws(() => createSelfTranslationReplayPlan(blockers, [blockers.groups[0].key, blockers.groups[0].key]), /distinct/u);
});

test('Links artifact analysis preserves original reports and the existing worker replays only the pinned selected source', () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'self-report-blockers-'));
  try {
    const module = 'js/src/translation/primitive-value-rules.js';
    const source = readFileSync(path.join(root, module));
    const report = authoredReport(module, source, 'authored replay-selection control');
    const input = path.join(directory, 'input.lino'), output = path.join(directory, 'analysis.lino');
    writeProjectData(input, report);
    const helper = path.join(root, 'js/scripts/self-translation-report-blockers.mjs');
    const analysis = spawnSync(process.execPath, [helper, '--report', input, '--out', output], { encoding: 'utf8' });
    assert.equal(analysis.status, 0, analysis.stderr);
    const packet = readProjectData(output);
    assert.deepEqual(packet.reports, [report]);
    const replay = path.join(directory, 'replay.lino');
    const selection = spawnSync(process.execPath, [helper, '--report', input, '--group', packet.blockers.groups[0].key, '--out', replay], { encoding: 'utf8' });
    assert.equal(selection.status, 0, selection.stderr);
    const plan = readProjectData(replay);
    const script = path.join(root, 'js/scripts/generate-self-translation-report.mjs');
    const result = spawnSync(process.execPath, [script, '--replay-plan', replay, '--out-dir', path.join(directory, 'measured')],
      { encoding: 'utf8', env: { ...process.env, CI: 'true', GITHUB_SHA: commit } });
    assert.equal(result.status, 0, result.stderr);
    const measured = readProjectData(path.join(directory, 'measured', 'self-translation-report.lino'));
    assert.deepEqual(measured.replay, plan);
    assert.equal(measured.commit, commit);
    assert.deepEqual(measured.failures, []);
    assert.deepEqual(measured.modules.map(row => row.module), [module]);
    assert.deepEqual(measured.inventory.map(row => row.module), [module]);
    assert.deepEqual(measured.blockers, rankSelfTranslationBlockers([measured]));
    for (const row of measured.modules) {
      assert.equal(row.sourceSha256, hash(source));
      assert.equal(row.coverage.unrepresentedBytes, 0);
      assert.equal(row.coverage.itemBytes + row.coverage.layoutBytes, source.length);
      assert.ok(row.translation.items.length > 0);
      assert.deepEqual(row.decorated.translation.items, row.translation.items);
    }
    const narrowed = spawnSync(process.execPath, [script, '--replay-plan', replay, '--modules', 'language-support.js', '--list'], { encoding: 'utf8' });
    assert.notEqual(narrowed.status, 0);
    assert.match(narrowed.stderr, /cannot silently narrow/u);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
