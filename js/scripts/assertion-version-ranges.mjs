// Tooling assertions select a bounded expectation for the actual active version.
// Numeric release coordinates and declared language-edition order are distinct.
import { compareVersions } from './semver-range.mjs';

function assertionVersionOrder(comparison, left, right) {
  if (comparison.kind === 'ordered') {
    const a = comparison.versions.indexOf(left);
    const b = comparison.versions.indexOf(right);
    if (a < 0 || b < 0) throw new Error(`unknown declared version ${a < 0 ? JSON.stringify(left) : JSON.stringify(right)}`);
    return Math.sign(a - b);
  }
  const pattern = new RegExp(`^(?:0|[1-9][0-9]*)(?:\\.(?:0|[1-9][0-9]*)){${comparison.precision - 1}}$`, 'u');
  for (const version of [left, right]) {
    if (typeof version !== 'string' || !pattern.test(version)
      || version.split('.').some(part => !Number.isSafeInteger(Number(part)))) {
      throw new Error(`invalid ${comparison.precision}-coordinate release ${JSON.stringify(version)}`);
    }
  }
  return compareVersions(left, right, comparison.precision);
}

function assertionVersionRange(expectation) {
  return `${expectation.includeMinimum ? '[' : '('}${expectation.minimum}, ${expectation.maximum}${expectation.includeMaximum ? ']' : ')'}`;
}

/**
 * Return the sole applicable expectation; missing coverage and overlapping
 * ranges are errors. The caller still executes the selected actual assertion.
 * This is repository tooling, not a declaration that the assertion passed.
 */
export function selectVersionedAssertion(policy, activeVersion) {
  const context = `${policy?.id ?? 'unnamed assertion'} (${policy?.subject ?? 'unknown subject'} ${String(activeVersion)})`;
  try {
    if (!policy || typeof policy !== 'object' || typeof policy.id !== 'string' || !policy.id
      || typeof policy.subject !== 'string' || !policy.subject) throw new Error('assertion identity and version subject are required');
    const comparison = policy.comparison;
    if (comparison?.kind === 'release') {
      if (![1, 2, 3].includes(comparison.precision)) throw new Error('release comparison requires an explicit precision of 1, 2 or 3');
    } else if (comparison?.kind === 'ordered') {
      if (!Array.isArray(comparison.versions) || comparison.versions.length === 0
        || comparison.versions.some(version => typeof version !== 'string' || !version)
        || new Set(comparison.versions).size !== comparison.versions.length) {
        throw new Error('edition comparison requires a nonempty, unique declared version order');
      }
    } else throw new Error('comparison must explicitly select release coordinates or a declared version order');
    if (!Array.isArray(policy.expectations) || policy.expectations.length === 0) throw new Error('bounded expectations are required');
    const identities = new Set();
    for (const expectation of policy.expectations) {
      if (!expectation || typeof expectation.id !== 'string' || !expectation.id || identities.has(expectation.id)) {
        throw new Error('expectation identities must be nonempty and unique');
      }
      identities.add(expectation.id);
      if (typeof expectation.includeMinimum !== 'boolean' || typeof expectation.includeMaximum !== 'boolean'
        || !Object.hasOwn(expectation, 'expected') || expectation.expected === undefined
        || typeof expectation.evidence !== 'string' || !expectation.evidence) {
        throw new Error(`${expectation.id}: bounds, expected value and evidence must be explicit`);
      }
      const order = assertionVersionOrder(comparison, expectation.minimum, expectation.maximum);
      if (order > 0 || (order === 0 && !(expectation.includeMinimum && expectation.includeMaximum))) {
        throw new Error(`${expectation.id}: empty or reversed range ${assertionVersionRange(expectation)}`);
      }
    }
    // Validate every range, including inactive alternatives, before selection.
    // No priority rule can hide conflicting or duplicate expectations.
    for (let left = 0; left < policy.expectations.length; left += 1) {
      for (let right = left + 1; right < policy.expectations.length; right += 1) {
        const a = policy.expectations[left];
        const b = policy.expectations[right];
        const before = assertionVersionOrder(comparison, a.maximum, b.minimum);
        const after = assertionVersionOrder(comparison, b.maximum, a.minimum);
        const disjoint = before < 0 || (before === 0 && !(a.includeMaximum && b.includeMinimum))
          || after < 0 || (after === 0 && !(b.includeMaximum && a.includeMinimum));
        if (!disjoint) throw new Error(`overlapping expectation ranges ${a.id} ${assertionVersionRange(a)} and ${b.id} ${assertionVersionRange(b)}`);
      }
    }
    assertionVersionOrder(comparison, activeVersion, activeVersion);
    for (const expectation of policy.expectations) {
      const lower = assertionVersionOrder(comparison, activeVersion, expectation.minimum);
      const upper = assertionVersionOrder(comparison, activeVersion, expectation.maximum);
      if ((lower > 0 || (lower === 0 && expectation.includeMinimum))
        && (upper < 0 || (upper === 0 && expectation.includeMaximum))) return expectation;
    }
    throw new Error(`uncovered active version; declared ranges: ${policy.expectations.map(expectation => `${expectation.id} ${assertionVersionRange(expectation)}`).join('; ')}`);
  } catch (error) {
    throw new Error(`${context}: ${error.message}`, { cause: error });
  }
}
