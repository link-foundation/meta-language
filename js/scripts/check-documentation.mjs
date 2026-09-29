#!/usr/bin/env node
// Checks that the documentation agrees with the issue #195 requirement ledger
// (docs/vision.md#documentation-consistency): see issue-195-documentation.mjs.
//
//   node scripts/check-documentation.mjs
//       every documentation file, package README, changelog fragment and case study
//   node scripts/check-documentation.mjs --online
//       also the live pull request title and description and the release reports
//   node scripts/check-documentation.mjs --pull-request-body FILE --acceptance-report FILE
//       also a pull request description, against an evaluated acceptance report
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { auditDocumentation, completionState, fetchExternalTexts } from './issue-195-documentation.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const args = process.argv.slice(2);
const option = (name) => {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
};
const readJson = (file) => JSON.parse(readFileSync(path.resolve(file), 'utf8'));

const manifest = readJson(path.join(root, 'parity/issue-195-requirements.json'));
const reportFile = option('--acceptance-report');
const state = completionState(manifest, reportFile ? readJson(reportFile) : null);
const external = [];
const bodyFile = option('--pull-request-body');
if (bodyFile) external.push({ name: bodyFile, text: readFileSync(path.resolve(bodyFile), 'utf8') });
if (args.includes('--online')) external.push(...(await fetchExternalTexts()));

const { files, problems } = auditDocumentation(root, state, { external });
if (problems.length > 0) {
  console.error(`documentation check failed (${problems.length} problems):`);
  for (const { file, line, claim, message } of problems) console.error(`- ${file}:${line} [${claim}] ${message}`);
  process.exit(1);
}
console.log(
  `documentation OK: ${files.length} files and ${external.length} external texts agree with the ledger ` +
    `(${state.complete ? 'every requirement verified' : `${state.unimplemented.length} of ${state.requirements} requirements unimplemented${state.verified ? '' : ', not yet verified'}`})`,
);
