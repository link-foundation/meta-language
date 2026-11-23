import { readFileSync } from 'node:fs';
import path from 'node:path';

import { ownStatus, parseCargoManifest } from './dependency-inventory.mjs';

const issueUrl = /^https:\/\/github\.com\/([\w.-]+\/[\w.-]+)\/issues\/([1-9]\d*)$/u;
const packageSections = ['dependencies', 'devDependencies', 'optionalDependencies', 'peerDependencies'];
const packageLocks = new Set(['js/package-lock.json', 'rust/Cargo.lock', 'rust/web/Cargo.lock']);

function staleDeclarations(root, item) {
  const files = item.declaredIn.filter((file) => /(?:Cargo\.toml|package\.json)$/u.test(file));
  return files.flatMap((file) => {
    const text = readFileSync(path.join(root, file), 'utf8');
    if (item.category === 'crate') {
      return parseCargoManifest(text).dependencies.filter(({ name, path: localPath }) => name === item.name && !localPath)
        .map(({ requirement, kind, blocker }) => ({ file, requirement, section: kind, blocker }));
    }
    const manifest = JSON.parse(text);
    const name = item.source.package;
    return packageSections.filter((section) => Object.hasOwn(manifest[section] ?? {}, name)).map((section) => {
      const annotation = manifest.dependencyBlockers?.[section]?.[name];
      const blocker = annotation?.requirement === manifest[section][name] ? annotation.issue : undefined;
      return { file, requirement: manifest[section][name], section, blocker };
    });
  });
}

function staleDirectDependencies(inventory) {
  return inventory.items.filter((item) => item.role === 'direct' && packageLocks.has(item.scope)
    && ['crate', 'npm'].includes(item.category) && item.current && ownStatus(item) === 'behind');
}

function declarationKey(item, { file, section, requirement }) {
  return JSON.stringify([item.id, file, section, requirement]);
}

/** Verify each exception against the GitHub issue endpoint at audit time. */
export async function refreshDependencyBlockers(root, inventory, { github, repository }) {
  const reports = new Map();
  const upstreams = new Map();
  const dependencyBlockers = {};
  const reasons = new Map();
  for (const item of staleDirectDependencies(inventory)) {
    const declarations = staleDeclarations(root, item);
    for (const declaration of declarations) {
      const { blocker } = declaration;
      const match = typeof blocker === 'string' && blocker.match(issueUrl);
      if (!match) continue;
      const sourceKey = JSON.stringify(item.source);
      if (!upstreams.has(sourceKey)) upstreams.set(sourceKey, await repository(item.source));
      if (upstreams.get(sourceKey)?.toLowerCase() !== match[1].toLowerCase()) continue;
      if (!reports.has(blocker)) {
        const issue = await github(`repos/${match[1]}/issues/${match[2]}`);
        reports.set(blocker, issue?.html_url === blocker && !issue.pull_request ? issue.state : null);
      }
      if (reports.get(blocker) === 'open') dependencyBlockers[declarationKey(item, declaration)] = blocker;
    }
    if (declarations.length && declarations.every((declaration) => dependencyBlockers[declarationKey(item, declaration)])) {
      reasons.set(item.id, `Audited open upstream issues: ${[...new Set(declarations.map(({ blocker }) => blocker))].join(', ')}`);
    }
  }
  const items = inventory.items.map((item) => {
    const { reason, ...facts } = item;
    const currentReason = reason?.startsWith('Audited open upstream issues: ') ? reasons.get(item.id) : reason ?? reasons.get(item.id);
    return currentReason ? { ...facts, reason: currentReason } : facts;
  });
  return { ...inventory, items, dependencyBlockers };
}

/** Check each direct pin against the committed, pre-merge registry audit. */
export function checkDependencyFreshness(root, inventory) {
  const problems = [];
  for (const item of staleDirectDependencies(inventory)) {
    const declarations = staleDeclarations(root, item);
    if (!declarations.length || declarations.some((declaration) => typeof declaration.blocker !== 'string'
      || !issueUrl.test(declaration.blocker)
      || inventory.dependencyBlockers?.[declarationKey(item, declaration)] !== declaration.blocker)) {
      problems.push({ kind: 'stale-direct-dependency', message: `${item.id} is behind ${item.current}. Each pin declaration needs an open issue in the dependency's GitHub repository verified by the dependency audit (${item.declaredIn.join(', ')}).` });
    }
  }
  return problems;
}
