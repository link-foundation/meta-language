// Recorded native recovery: where a language parses with its native grammar, a malformed
// input is repaired by the native executor's cost-based recovery, which can place its ERROR
// and MISSING nodes elsewhere than tree-sitter's LR recovery. Each such case of the
// conformance and generative suites is recorded in parity/fixtures/native-recovery.json with
// the digests of both trees, the repair sites of both and a category, whose justification the
// file gives. A recorded case checks that the oracle is malformed and is the recorded one,
// that the native tree is the recorded one, malformed, lossless and consistent with its
// diagnostics, and that the category follows from the two trees; a case that matches the
// oracle, or a clean oracle, may not be recorded. Mirrored by
// rust/tests/unit/native_recovery_records.rs.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

import { parseCstLines, rowOffsets } from './cst-lines.js';

export const NATIVE_RECOVERY_FILE = 'parity/fixtures/native-recovery.json';
const sha256 = (text) => createHash('sha256').update(text).digest('hex');
const point = ({ row, column }) => `${row}:${column}`;

export function readNativeRecovery() {
  return JSON.parse(readFileSync(new URL(`../../../${NATIVE_RECOVERY_FILE}`, import.meta.url), 'utf8'));
}

/** Whether canonical CST lines hold an ERROR or MISSING node. */
export function isMalformed(text) {
  return parseCstLines(text).some((node) => node.error || node.missing);
}

/** The ERROR spans and MISSING leaves of canonical CST lines, and the bytes the ERROR spans cover. */
export function repairSites(text, source) {
  const offsets = rowOffsets(source);
  const nodes = parseCstLines(text);
  const errors = nodes.filter((node) => node.error);
  const covered = new Set();
  for (const node of errors) {
    const end = offsets[node.end.row] + node.end.column;
    for (let byte = offsets[node.start.row] + node.start.column; byte < end; byte += 1) covered.add(byte);
  }
  return {
    errors: errors.map((node) => `${point(node.start)}-${point(node.end)}`),
    missing: nodes.filter((node) => node.missing).map((node) => `${node.named ? node.kind : JSON.stringify(node.kind)} ${point(node.start)}`),
    skipped: covered.size,
  };
}

/** The category of a recovery discrepancy, from the repair sites of the oracle and the native tree. */
export function recoveryCategory(oracle, native) {
  const at = (missing) => JSON.stringify(missing.map((entry) => entry.slice(entry.lastIndexOf(' ') + 1)));
  if (JSON.stringify(oracle.errors) === JSON.stringify(native.errors) && at(oracle.missing) === at(native.missing)) {
    return 'same-repair-sites';
  }
  if (oracle.skipped > native.skipped) return 'oracle-skips-more';
  if (oracle.skipped < native.skipped) return 'native-skips-more';
  return 'same-skipped-bytes';
}

/** The record of one discrepancy: the digests and repair sites of both trees and its category. */
export function recoveryRecord({ language, suite, id, source, oracle, native }) {
  const [oracleSites, nativeSites] = [repairSites(oracle, source), repairSites(native, source)];
  const sites = ({ errors, missing }) => ({ errors, missing });
  return {
    language,
    suite,
    case: id,
    category: recoveryCategory(oracleSites, nativeSites),
    oracle: { cstSha256: sha256(oracle), ...sites(oracleSites) },
    native: { cstSha256: sha256(native), ...sites(nativeSites) },
  };
}

/**
 * The recorded discrepancies of one suite and language: `resolve` turns the oracle problems
 * of a case into the problems of its record, and `unused` lists the records no case met.
 */
export function nativeRecovery(suite, language, file = readNativeRecovery()) {
  const records = new Map(file.cases.filter((record) => record.suite === suite && record.language === language)
    .map((record) => [record.case, record]));
  const used = new Set();
  return {
    records,
    /**
     * `problems` compare the native tree `text` of case `id` with the oracle tree `oracle`;
     * `check(cst)` gives the problems of the same parse against other CST lines.
     */
    resolve({ id, source, oracle, text, problems, check }) {
      const record = records.get(id);
      if (!record) return problems;
      used.add(id);
      if (!problems.length) return ['the native tree equals the oracle, so the recorded recovery discrepancy is stale'];
      const expected = recoveryRecord({ language, suite, id, source, oracle, native: text });
      const found = [];
      if (!isMalformed(oracle)) found.push('a clean oracle tree may not be recorded as a recovery discrepancy');
      if (!isMalformed(text)) found.push('the native tree is clean, so it is no recovery');
      if (!Object.hasOwn(file.categories, record.category)) found.push(`category ${record.category} has no justification`);
      if (JSON.stringify(expected) !== JSON.stringify(record)) {
        found.push(`the recorded recovery differs: ${JSON.stringify(expected)} (regenerate with node scripts/generate-native-recovery.mjs after checking it)`);
      }
      found.push(...check(text).map((problem) => `against the native tree: ${problem}`));
      return found;
    },
    unused: () => [...records.keys()].filter((id) => !used.has(id)),
  };
}
