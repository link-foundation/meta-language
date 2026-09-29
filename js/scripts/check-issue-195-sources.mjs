#!/usr/bin/env node
// Checks the issue #195 source register (parity/issue-195-sources.json).
//
//   node scripts/check-issue-195-sources.mjs            offline: register against the ledger
//   node scripts/check-issue-195-sources.mjs --online   also against the live issue and PR comments
//   node scripts/check-issue-195-sources.mjs --update   rewrite registered revisions from the live
//                                                       discussion, after re-reading every edit
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { ISSUE_195_SOURCES, buildIssue195Manifest } from './issue-195-requirements.mjs';
import {
  ISSUE_195_SOURCE_REGISTER_PATH,
  compareWithLiveDiscussion,
  fetchLiveDiscussion,
  refreshRegister,
  validateSourceRegister,
} from './issue-195-sources.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const registerPath = path.join(root, ISSUE_195_SOURCE_REGISTER_PATH);
const args = new Set(process.argv.slice(2));

let register = JSON.parse(await readFile(registerPath, 'utf8'));
if (args.has('--update')) {
  register = refreshRegister(register, await fetchLiveDiscussion());
  register.reconciledAt = new Date().toISOString().slice(0, 10);
  await writeFile(registerPath, `${JSON.stringify(register, null, 2)}\n`);
  console.log(`updated ${ISSUE_195_SOURCE_REGISTER_PATH}`);
}

const errors = validateSourceRegister(register, await buildIssue195Manifest(root), ISSUE_195_SOURCES);
if (args.has('--online') || args.has('--update')) {
  errors.push(...compareWithLiveDiscussion(register, await fetchLiveDiscussion()));
}
if (errors.length > 0) {
  console.error(`issue #195 source register check failed:\n- ${errors.join('\n- ')}`);
  process.exit(1);
}
console.log(
  `issue #195 source register OK: ${register.requirementSources.length} requirement sources, ` +
    `${register.statusReports.length} status reports${args.has('--online') ? ', matching the live discussion' : ''}`,
);
