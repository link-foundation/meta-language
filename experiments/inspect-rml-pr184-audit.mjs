#!/usr/bin/env node
// Inspect the immutable head's inventory through GitHub without cloning or
// running downstream workloads. Blob hashes are verified by readRmlPr184.
import { readFileSync, writeFileSync } from 'node:fs';
import { readRmlPr184 } from '../js/scripts/issue-195-rml-pr184.mjs';

const [headFile, outputFile, mode] = process.argv.slice(2);
if (!headFile || !outputFile || (mode && mode !== '--pinned')) {
  throw new Error('usage: inspect-rml-pr184-audit.mjs HEAD_JSON OUTPUT_JSON [--pinned]');
}
const { headRefOid } = JSON.parse(readFileSync(headFile, 'utf8'));
const live = await readRmlPr184(headRefOid, { live: mode !== '--pinned' });
if (live.head !== headRefOid) throw new Error('the pull request moved during the inspection');
writeFileSync(outputFile, `${JSON.stringify(live, null, 2)}\n`);
console.log(`verified ${live.workloads.length} workloads at ${live.head}`);
