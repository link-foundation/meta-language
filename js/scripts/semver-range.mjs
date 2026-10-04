// Version comparison and the npm and Cargo requirement syntax, enough to tell
// whether a dependent's requirement admits a release
// (docs/vision.md#dependencies: the dependency inventory).

const PRERELEASE = /^v?\d+(?:\.\d+)*[-~]?[A-Za-z]/u;

/** The numeric parts of a version-like text (`v7.0.1`, `V9.3.0`, `>=22`, `5.4`), or null. */
export function versionParts(text) {
  const match = String(text ?? '').match(/(\d+)(?:\.(\d+))?(?:\.(\d+))?/u);
  if (!match) return null;
  return [match[1], match[2], match[3]].map((part) => (part === undefined ? null : Number(part)));
}

/** Whether a version names a prerelease (`5.5.0~rc1`, `1.0.0-beta.2`). */
export function isPrerelease(text) {
  // Build metadata (`1.0.4+wasi-0.2.12`) does not make a release a prerelease.
  const bare = String(text).trim().split('+')[0];
  return PRERELEASE.test(bare) || /[-~](?:alpha|beta|rc|pre|preview|dev|nightly)/iu.test(bare);
}

/** Compares two versions over their first `depth` parts; missing parts count as 0. */
export function compareVersions(left, right, depth = 3) {
  const a = versionParts(left);
  const b = versionParts(right);
  if (!a || !b) throw new Error(`cannot compare versions ${JSON.stringify(left)} and ${JSON.stringify(right)}`);
  for (let index = 0; index < depth; index += 1) {
    const difference = (a[index] ?? 0) - (b[index] ?? 0);
    if (difference !== 0) return Math.sign(difference);
  }
  return 0;
}

/** The newest stable version of a list, or null. */
export function newestStable(versions) {
  return versions.filter((version) => versionParts(version) && !isPrerelease(version)).sort((a, b) => compareVersions(b, a))[0] ?? null;
}

function comparatorsFor(operator, text) {
  const [major, minor, patch] = versionParts(text) ?? [null, null, null];
  if (major === null) return [];
  const lower = [major, minor ?? 0, patch ?? 0];
  const bound = (op, parts) => ({ op, parts });
  const exactUpper = minor === null ? [major + 1, 0, 0] : patch === null ? [major, minor + 1, 0] : null;
  switch (operator) {
    case '=':
    case '':
      if (exactUpper) return [bound('>=', lower), bound('<', exactUpper)];
      return [bound('=', lower)];
    case '>':
      return exactUpper ? [bound('>=', exactUpper)] : [bound('>', lower)];
    case '>=':
      return [bound('>=', lower)];
    case '<':
      return [bound('<', lower)];
    case '<=':
      return [bound(exactUpper ? '<' : '<=', exactUpper ?? lower)];
    case '~':
      return [bound('>=', lower), bound('<', minor === null ? [major + 1, 0, 0] : [major, minor + 1, 0])];
    case '^': {
      let upper;
      if (major > 0 || minor === null) upper = [major + 1, 0, 0];
      else if (minor > 0 || patch === null) upper = [0, minor + 1, 0];
      else upper = [0, 0, patch + 1];
      return [bound('>=', lower), bound('<', upper)];
    }
    default:
      throw new Error(`unknown requirement operator ${operator}`);
  }
}

function holds(version, { op, parts }) {
  const order = compareVersions(version, parts.join('.'));
  return { '=': order === 0, '>': order > 0, '>=': order >= 0, '<': order < 0, '<=': order <= 0 }[op];
}

function parseComparator(text, defaultOperator) {
  const trimmed = text.trim();
  if (trimmed === '' || trimmed === '*' || /^[xX*]/u.test(trimmed)) return [];
  const match = trimmed.match(/^(>=|<=|>|<|=|\^|~>?|)\s*v?([0-9][^\s]*)$/u);
  if (!match) throw new Error(`unsupported requirement ${JSON.stringify(text)}`);
  const operator = match[1] === '' ? defaultOperator : match[1].replace('~>', '~');
  const version = match[2].replace(/\.[xX*](?=\.|$)/gu, '').replace(/[-+].*$/u, '');
  return comparatorsFor(operator, version);
}

/**
 * Whether a stable `version` satisfies a requirement. `syntax` is `cargo`
 * (comma-separated comparators, a bare version is a caret requirement) or
 * `npm` (`||` alternatives of space-separated comparators, a bare version is
 * exact).
 */
export function satisfies(version, requirement, syntax) {
  if (syntax === 'cargo') {
    return requirement
      .split(',')
      .flatMap((part) => parseComparator(part, '^'))
      .every((comparator) => holds(version, comparator));
  }
  if (syntax === 'npm') {
    return requirement.split('||').some((alternative) => {
      const hyphen = alternative.match(/^\s*(\S+)\s+-\s+(\S+)\s*$/u);
      const comparators = hyphen
        ? [...parseComparator(`>=${hyphen[1]}`, '='), ...parseComparator(`<=${hyphen[2]}`, '=')]
        : alternative.trim().split(/\s+/u).flatMap((part) => parseComparator(part.replace(/^(>=|<=|>|<|=|\^|~)\s+/u, '$1'), '='));
      return comparators.every((comparator) => holds(version, comparator));
    });
  }
  throw new Error(`unknown requirement syntax ${syntax}`);
}
