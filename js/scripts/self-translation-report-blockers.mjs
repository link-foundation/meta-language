// Reuses published report measurements; grouping and replay selection are not
// translation or behavioral evidence. Exact reasons deliberately remain distinct.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';

const digest = value => createHash('sha256').update(value).digest('hex');
const compare = (left, right) => left < right ? -1 : left > right ? 1 : 0;

export function requireReportSourcePath(module) {
  assert.ok(typeof module === 'string' && module.startsWith('js/src/') && module.endsWith('.js')
    && !module.includes('\\') && !module.includes('\0')
    && module.split('/').every(part => part && part !== '.' && part !== '..'),
  `report module must be a source file inside js/src: ${module}`);
  return module;
}

/** Rank exact diagnostics without changing or discarding the original reports. */
export function rankSelfTranslationBlockers(reports) {
  assert.ok(Array.isArray(reports) && reports.length > 0, 'at least one report is required');
  const commit = reports[0].commit;
  assert.match(commit ?? '', /^[0-9a-f]{40}$/u, 'the recorded report commit is required');
  const inventory = new Map(), groups = new Map(), failures = [], diagnosticGaps = [];
  let catalog = null;
  const decorators = reports[0].decorators ?? null;
  const add = (mode, kind, reason, occurrence) => {
    assert.equal(typeof reason, 'string', 'a retained diagnostic must have its exact reason');
    const diagnostic = occurrence.item?.diagnostic;
    const definition = { sourceLanguage: 'JavaScript', targetLanguage: 'Rust', mode, kind,
      term: occurrence.item?.term ?? null, diagnosticKind: diagnostic?.kind ?? null,
      reason: diagnostic?.reason ?? reason };
    const key = digest(JSON.stringify(definition));
    if (!groups.has(key)) groups.set(key, { key, ...definition, occurrences: [] });
    groups.get(key).occurrences.push(occurrence);
  };
  for (const report of reports) {
    assert.equal(report.schemaVersion, 1, 'unsupported report schema');
    assert.equal(report.commit, commit, 'reports must retain one recorded commit');
    assert.deepEqual(report.decorators ?? null, decorators, 'report decorator selections differ');
    assert.ok(Array.isArray(report.modules) && Array.isArray(report.failures), 'report rows are required');
    if (report.sourceCatalog !== undefined) {
      assert.ok(Array.isArray(report.sourceCatalog), 'source catalog must be an array');
      const current = report.sourceCatalog.map(requireReportSourcePath).slice().sort(compare);
      assert.equal(new Set(current).size, current.length, 'duplicate source catalog module');
      if (catalog) assert.deepEqual(current, catalog, 'report source catalogs differ');
      catalog = current;
    }
    const outcomes = [...report.modules, ...report.failures];
    const selected = report.inventory ?? outcomes.map(row => ({
      module: row.module, sourceSha256: row.sourceSha256 ?? null,
      rust: row.rust ?? null, rustSha256: row.rustSha256 ?? null,
    }));
    assert.ok(Array.isArray(selected), 'selected report inventory must be an array');
    const remaining = new Set();
    for (const source of selected) {
      requireReportSourcePath(source.module);
      assert.ok(!inventory.has(source.module), `duplicate report module ${source.module}`);
      if (source.sourceSha256 !== null) assert.match(source.sourceSha256 ?? '', /^[0-9a-f]{64}$/u, 'source hash');
      if (catalog) assert.ok(catalog.includes(source.module), `module outside source catalog ${source.module}`);
      inventory.set(source.module, { ...source });
      remaining.add(source.module);
    }
    for (const row of outcomes) {
      assert.ok(remaining.delete(row.module), `missing or duplicate selected outcome ${row.module}`);
      const source = inventory.get(row.module);
      if (row.sourceSha256 !== undefined) assert.equal(row.sourceSha256, source.sourceSha256, 'measurement source hash');
      if (row.rust !== undefined) assert.equal(row.rust, source.rust, 'measurement Rust counterpart');
      if (row.rustSha256 !== undefined) assert.equal(row.rustSha256, source.rustSha256, 'measurement Rust source hash');
    }
    assert.equal(remaining.size, 0, `selected modules have no outcome: ${[...remaining].join(', ')}`);
    for (const failure of report.failures) {
      failures.push({ ...failure });
      add('measurement', 'module-error', failure.error, { ...inventory.get(failure.module), error: failure.error });
    }
    for (const row of report.modules) {
      for (const mode of ['generic', 'decorated']) {
        const measurement = mode === 'generic' ? row : row.decorated;
        const translation = measurement?.translation;
        if (!translation) {
          diagnosticGaps.push({ module: row.module, mode, sourceSha256: row.sourceSha256,
            carriedItems: mode === 'generic' ? row.items?.carried ?? null : null,
            reason: 'This report does not retain individual translation item diagnostics.' });
          continue;
        }
        assert.equal(translation.sourceLanguage, 'JavaScript', 'unsupported report source language');
        assert.equal(translation.targetLanguage, 'Rust', 'unsupported report target language');
        assert.ok(Array.isArray(translation.items), 'individual translation items are required');
        let end = 0;
        for (const item of translation.items) {
          assert.ok(Number.isSafeInteger(item.start) && Number.isSafeInteger(item.end)
            && item.start >= end && item.end > item.start && item.end <= measurement.coverage.sourceBytes,
          `invalid retained item range in ${row.module}`);
          assert.equal(typeof item.term, 'string', 'retained item term');
          assert.ok(['translated', 'carried', 'comment', 'kept', 'restored', 'provenance'].includes(item.status), 'retained item status');
          end = item.end;
          if (item.status === 'carried') {
            if (item.diagnostic) {
              for (const field of ['kind', 'reason', 'message']) assert.equal(typeof item.diagnostic[field], 'string', `retained diagnostic ${field}`);
              assert.ok(Object.hasOwn(item.diagnostic, 'span') && Object.hasOwn(item.diagnostic, 'details'), 'retained diagnostic span and details');
              assert.ok(item.diagnostic.sourceRange.start <= item.start && item.diagnostic.sourceRange.end >= item.end
                && item.diagnostic.sourceRange.start >= 0 && item.diagnostic.sourceRange.end <= measurement.coverage.sourceBytes,
              'diagnostic source group must contain its complete item');
            } else diagnosticGaps.push({ module: row.module, mode, item: { ...item },
              reason: 'This carried item has no retained TranslationError detail; its original reason remains grouped exactly.' });
            add(mode, 'carried-item', item.reason, { ...inventory.get(row.module), item: { ...item } });
          }
        }
        if (mode === 'generic') assert.equal(translation.items.filter(item => item.status === 'carried').length,
          row.items.carried, 'carried item count differs from retained diagnostics');
      }
    }
  }
  const ranked = [...groups.values()].map(group => ({ ...group,
    occurrences: group.occurrences.sort((left, right) => compare(left.module, right.module)
      || (left.item?.start ?? 0) - (right.item?.start ?? 0)),
    moduleCount: new Set(group.occurrences.map(row => row.module)).size,
    itemCount: group.occurrences.filter(row => row.item).length,
    sourceBytes: group.occurrences.reduce((sum, row) => sum + (row.item ? row.item.end - row.item.start : 0), 0),
  })).sort((left, right) => right.moduleCount - left.moduleCount || right.sourceBytes - left.sourceBytes
    || compare(left.reason, right.reason) || compare(left.mode, right.mode) || compare(left.key, right.key));
  const sources = [...inventory.values()].sort((left, right) => compare(left.module, right.module));
  if (catalog) for (const source of sources) assert.ok(catalog.includes(source.module), `module outside source catalog ${source.module}`);
  return { schemaVersion: 1, commit, sourceLanguage: 'JavaScript', targetLanguage: 'Rust', decorators,
    inventory: sources, sourceCatalog: catalog,
    unreportedModules: catalog ? catalog.filter(module => !inventory.has(module)) : null,
    failures: failures.sort((left, right) => compare(left.module, right.module)),
    diagnosticGaps: diagnosticGaps.sort((left, right) => compare(left.module, right.module) || compare(left.mode, right.mode)),
    groups: ranked };
}

/** Retains the full original denominator and diagnostics beside the selection. */
export function createSelfTranslationReplayPlan(blockers, groupKeys) {
  assert.equal(blockers.sourceLanguage, 'JavaScript', 'unsupported replay source language');
  assert.equal(blockers.targetLanguage, 'Rust', 'unsupported replay target language');
  assert.ok(Array.isArray(groupKeys) && groupKeys.length > 0 && new Set(groupKeys).size === groupKeys.length,
    'select one or more distinct blocker keys');
  const selected = new Set();
  for (const key of groupKeys) {
    const group = blockers.groups.find(group => group.key === key);
    assert.ok(group, `unknown blocker key ${key}`);
    for (const occurrence of group.occurrences) selected.add(occurrence.module);
  }
  const sources = blockers.inventory.filter(source => selected.has(source.module));
  assert.equal(sources.length, selected.size, 'every selected blocker source must have one inventory record');
  assert.equal(new Set(sources.map(source => source.module)).size, sources.length, 'duplicate replay source');
  for (const source of sources) assert.match(source.sourceSha256 ?? '', /^[0-9a-f]{64}$/u, `replay source hash ${source.module}`);
  return { schemaVersion: 1, kind: 'self-translation-report-replay', originCommit: blockers.commit,
    sourceLanguage: 'JavaScript', targetLanguage: 'Rust', mode: 'generic-and-decorated',
    groupKeys: groupKeys.slice().sort(compare), sources, origin: blockers };
}

/** Current source bytes must match; a new translator commit remains observable. */
export function validateSelfTranslationReplayPlan(plan, availableModules, readSource) {
  assert.equal(plan.schemaVersion, 1, 'unsupported replay schema');
  assert.equal(plan.kind, 'self-translation-report-replay', 'unsupported replay kind');
  assert.equal(plan.sourceLanguage, 'JavaScript', 'this report runner supports JavaScript to Rust replay only');
  assert.equal(plan.targetLanguage, 'Rust', 'this report runner supports JavaScript to Rust replay only');
  assert.equal(plan.mode, 'generic-and-decorated', 'replay requires both existing measurements');
  assert.match(plan.originCommit ?? '', /^[0-9a-f]{40}$/u, 'replay origin commit');
  assert.equal(plan.origin?.commit, plan.originCommit, 'replay origin commit differs');
  assert.deepEqual(plan, createSelfTranslationReplayPlan(plan.origin, plan.groupKeys), 'replay selection differs from original diagnostics');
  assert.ok(plan.sources.length > 0, 'replay selection is empty');
  const available = new Set(availableModules);
  for (const source of plan.sources) {
    requireReportSourcePath(source.module);
    assert.ok(available.has(source.module), `replay source is missing from the source inventory: ${source.module}`);
    assert.equal(digest(readSource(source.module)), source.sourceSha256, `replay source hash changed: ${source.module}`);
  }
  return plan.sources.map(source => source.module);
}

// Analyze retained artifacts without rerunning translations. Repeating --report
// merges shards; --group KEY writes a replay plan instead of an analysis packet.
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const args = process.argv.slice(2), paths = [], keys = [];
    let output = null;
    for (let index = 0; index < args.length; index += 2) {
      const [name, value] = args.slice(index, index + 2);
      assert.ok(value, `missing value for ${name}`);
      if (name === '--report') paths.push(value);
      else if (name === '--group') keys.push(value);
      else if (name === '--out') output = value;
      else throw new Error(`unknown option ${name}`);
    }
    assert.ok(output, 'usage: self-translation-report-blockers.mjs --report report.lino [--report shard.lino] [--group KEY] --out output.lino');
    const reports = paths.map(path => readProjectData(path));
    const blockers = rankSelfTranslationBlockers(reports);
    writeProjectData(output, keys.length ? createSelfTranslationReplayPlan(blockers, keys) : { reports, blockers });
    console.log(`${blockers.inventory.length} reported modules, ${blockers.groups.length} exact blocker groups, ${blockers.diagnosticGaps.length} diagnostic gaps`);
  } catch (error) {
    console.error(String(error?.message ?? error));
    process.exitCode = 1;
  }
}
