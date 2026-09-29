#!/usr/bin/env node
// Checks the dependency inventory (requirement I195-DEPENDENCY-INVENTORY):
// every dependency the repository declares is in parity/dependency-inventory.json
// with its recorded pin, the audit date is valid, every retained item behind its
// current stable release records a compatibility reason, and
// docs/dependency-audit.md is the inventory's rendering.
//
//   node js/scripts/check-dependencies.mjs            offline check (CI)
//   node js/scripts/check-dependencies.mjs --refresh  query the registries and rewrite
//                                                     the inventory and the audit document
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  AUDIT_DOCUMENT,
  INVENTORY_FILE,
  checkInventory,
  collectDependencies,
  refreshInventory,
  releaseResolvers,
  renderAuditDocument,
} from './dependency-inventory.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

async function http(url, headers = {}) {
  for (let attempt = 1; ; attempt += 1) {
    const response = await fetch(url, { headers: { 'user-agent': 'meta-language-dependency-audit', ...headers } });
    if (response.ok) return response.text();
    if (attempt >= 3 || response.status < 500) throw new Error(`${url}: HTTP ${response.status}`);
    await new Promise((resolve) => setTimeout(resolve, 1000 * attempt));
  }
}

function github(route) {
  return Promise.resolve(JSON.parse(execFileSync('gh', ['api', route], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] })));
}

const metadata = new Map();
function cargoMetadata(manifest) {
  if (!metadata.has(manifest)) {
    const output = execFileSync('cargo', ['metadata', '--format-version', '1', '--locked', '--offline', '--manifest-path', path.join(root, manifest)], {
      encoding: 'utf8',
      maxBuffer: 256 * 1024 * 1024,
      stdio: ['ignore', 'pipe', 'inherit'],
    });
    metadata.set(manifest, JSON.parse(output));
  }
  return metadata.get(manifest);
}

const readInventory = () => (existsSync(path.join(root, INVENTORY_FILE)) ? JSON.parse(readFileSync(path.join(root, INVENTORY_FILE), 'utf8')) : null);

async function main(argv) {
  const collected = collectDependencies(root);
  if (argv.includes('--refresh')) {
    const today = new Date().toISOString().slice(0, 10);
    const inventory = await refreshInventory({
      root,
      previous: readInventory(),
      collected,
      resolvers: releaseResolvers({ http, github, today }),
      cargoMetadata,
      today,
      onProgress: (done, total) => {
        if (done % 50 === 0 || done === total) process.stderr.write(`resolved ${done}/${total}\n`);
      },
    });
    writeFileSync(path.join(root, INVENTORY_FILE), `${JSON.stringify(inventory, null, 2)}\n`);
    writeFileSync(path.join(root, AUDIT_DOCUMENT), renderAuditDocument(inventory));
    console.log(`wrote ${INVENTORY_FILE} and ${AUDIT_DOCUMENT} (${inventory.items.length} items, audit date ${inventory.auditDate})`);
  }
  const inventory = readInventory();
  if (!inventory) {
    console.error(`${INVENTORY_FILE} is missing; run node js/scripts/check-dependencies.mjs --refresh`);
    return 1;
  }
  const problems = checkInventory(inventory, collected);
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
  console.log(`dependency inventory ok: ${inventory.items.length} items, audit date ${inventory.auditDate}`);
  return 0;
}

process.exitCode = await main(process.argv.slice(2));
