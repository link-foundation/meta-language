#!/usr/bin/env node
// Checks the dependency inventory (requirement I195-DEPENDENCY-INVENTORY):
// every dependency the repository declares is in parity/dependency-inventory.lino
// with its recorded pin, the audit date is valid, every retained item behind its
// current stable release records a compatibility reason, and
// docs/dependency-audit.md is the inventory's rendering.
//
// --delivery is the delivery gate (I195-DEPENDENCY-CURRENT-STABLE-DELIVERY):
// every retained item must be at its current stable release or at its
// verified newest compatible release. Pass --live to refresh the inventory from
// the registries in memory and check that it agrees with the recorded audit.
// The default and --offline both use the recorded audit.
//
//   node js/scripts/check-dependencies.mjs            offline check (CI)
//   node js/scripts/check-dependencies.mjs --refresh  query the registries and rewrite
//                                                     the inventory and the audit document
//   node js/scripts/check-dependencies.mjs --delivery [--live|--offline] --consumer-lock <package-lock.json>
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, resolveProjectDataPath, writeProjectData } from './project-data.mjs';
import {retainDependencyInventoryOrder} from './dependency-inventory-order.mjs';
import { checkDependencyFreshness, refreshDependencyBlockers } from './dependency-freshness.mjs';

import {
  AUDIT_DOCUMENT,
  INVENTORY_FILE,
  checkDeliveredDependencies,
  checkInventory,
  collectDependencies,
  compareAudits,
  refreshInventory,
  reconcileLocalDeclarations,
  registryHttp,
  releaseResolvers,
  renderAuditDocument,
} from './dependency-inventory.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const http = registryHttp();

function github(route) {
  return Promise.resolve(JSON.parse(execFileSync('gh', ['api', route], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] })));
}

async function githubIssue(route) {
  try {
    return await github(route);
  } catch (error) {
    if (String(error.stderr).includes('HTTP 404')) return null;
    throw error;
  }
}

async function dependencyRepository(source) {
  let repository;
  if (source.type === 'crate') {
    const document = JSON.parse(await http(`https://crates.io/api/v1/crates/${encodeURIComponent(source.crate)}`));
    repository = document.crate?.repository;
  } else if (source.type === 'npm') {
    const document = JSON.parse(await http(`https://registry.npmjs.org/${encodeURIComponent(source.package)}/latest`));
    repository = document.repository?.url ?? document.repository;
  }
  if (typeof repository !== 'string') return undefined;
  return /^(?:github:|(?:git\+)?(?:https?|git|ssh):\/\/(?:git@)?github\.com\/|git@github\.com:)([\w.-]+\/[\w.-]+?)(?:\.git)?\/?$/iu.exec(repository)?.[1];
}

const blockerSources = { github: githubIssue, repository: dependencyRepository };

const metadata = new Map();
function cargoMetadata(manifest) {
  if (!metadata.has(manifest)) {
    const run = (offline) => execFileSync('cargo', ['metadata', '--format-version', '1', '--locked', '--all-features', ...(offline ? ['--offline'] : []), '--manifest-path', path.join(root, manifest)], {
      encoding: 'utf8',
      maxBuffer: 256 * 1024 * 1024,
      stdio: ['ignore', 'pipe', offline ? 'ignore' : 'inherit'],
    });
    let output;
    try {
      output = run(true);
    } catch {
      // A fresh runner has no registry cache: fetch the locked crates.
      output = run(false);
    }
    metadata.set(manifest, JSON.parse(output));
  }
  return metadata.get(manifest);
}

const readInventory = () => (existsSync(resolveProjectDataPath(path.join(root, INVENTORY_FILE))) ? readProjectData(path.join(root, INVENTORY_FILE)) : null);

async function main(argv) {
  const collected = collectDependencies(root);
  if (argv.includes('--refresh-declarations')) {
    const previous = readInventory();
    if (!previous) throw new Error('a registry audit is required before reconciling local declarations');
    const inventory = retainDependencyInventoryOrder(previous, reconcileLocalDeclarations(previous, collected));
    writeProjectData(path.join(root, INVENTORY_FILE), inventory);
    writeFileSync(path.join(root, AUDIT_DOCUMENT), renderAuditDocument(inventory));
    console.log(`reconciled local declarations; registry audit date remains ${inventory.auditDate}`);
  }
  if (argv.includes('--refresh')) {
    const today = new Date().toISOString().slice(0, 10);
    const previous = readInventory();
    const refreshed = await refreshInventory({
      root,
      previous,
      collected,
      resolvers: releaseResolvers({ http, github, today }),
      cargoMetadata,
      today,
      onProgress: (done, total) => {
        if (done % 50 === 0 || done === total) process.stderr.write(`resolved ${done}/${total}\n`);
      },
    });
    const audited = await refreshDependencyBlockers(root, refreshed, blockerSources);
    const inventory = previous ? retainDependencyInventoryOrder(previous, audited) : audited;
    writeProjectData(path.join(root, INVENTORY_FILE), inventory);
    writeFileSync(path.join(root, AUDIT_DOCUMENT), renderAuditDocument(inventory));
    console.log(`wrote ${INVENTORY_FILE} and ${AUDIT_DOCUMENT} (${inventory.items.length} items, audit date ${inventory.auditDate})`);
  }
  const inventory = readInventory();
  if (!inventory) {
    console.error(`${INVENTORY_FILE} is missing; run node js/scripts/check-dependencies.mjs --refresh`);
    return 1;
  }
  const consumerIndex = argv.indexOf('--consumer-lock');
  if (consumerIndex >= 0 && (!argv[consumerIndex + 1] || argv[consumerIndex + 1].startsWith('--'))) {
    throw new Error('--consumer-lock requires a clean consumer package-lock.json path');
  }
  const npmConsumerLock = consumerIndex >= 0 ? readProjectData(path.resolve(argv[consumerIndex + 1]), { format: 'json' }) : undefined;
  const delivery = argv.includes('--delivery');
  // The live comparison runs only when asked for: pull request checks compare
  // against the committed audit, and the scheduled refresh on main goes live.
  const live = argv.includes('--live') && !argv.includes('--offline');
  const problems = delivery
    ? checkDeliveredDependencies(inventory, collected, { npmConsumerLock })
    : checkInventory(inventory, collected);
  problems.push(...checkDependencyFreshness(root, inventory));
  if (live) {
    try {
      const today = new Date().toISOString().slice(0, 10);
      const current = await refreshInventory({ root, previous: inventory, collected, resolvers: releaseResolvers({ http, github, today }), cargoMetadata, today });
      problems.push(...compareAudits(inventory, current));
      problems.push(...checkDependencyFreshness(root, await refreshDependencyBlockers(root, current, blockerSources)));
      if (delivery) {
        for (const problem of checkDeliveredDependencies(current, collected, { npmConsumerLock })) {
          if (problem.kind === 'stale-delivered-dependency') problems.push({ kind: 'stale-live-dependency', message: problem.message });
        }
      }
    } catch (error) {
      problems.push({ kind: 'live-registry-unavailable', message: `the live comparison needs the registries and gh: ${error.message}` });
    }
  }
  const documentPath = path.join(root, AUDIT_DOCUMENT);
  const rendered = renderAuditDocument(inventory);
  if (!existsSync(documentPath) || readFileSync(documentPath, 'utf8') !== rendered) {
    problems.push({ kind: 'audit-document', message: `${AUDIT_DOCUMENT} is not the rendering of ${INVENTORY_FILE}; run node js/scripts/check-dependencies.mjs --refresh` });
  }
  for (const { kind, message } of problems) console.error(`${kind}: ${message}`);
  if (problems.length > 0) {
    console.error(`${problems.length} dependency inventory problem(s)`);
    return 1;
  }
  console.log(`dependency inventory ok: ${inventory.items.length} items, audit date ${inventory.auditDate}${delivery ? ', delivered current or at the newest compatible release' : ''}${live ? ', matching the live registries' : ''}`);
  return 0;
}

process.exitCode = await main(process.argv.slice(2));
