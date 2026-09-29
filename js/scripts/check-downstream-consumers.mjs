#!/usr/bin/env node
// Checks the downstream consumer matrix (docs/downstream-consumers.md): both
// consumers are mapped, every mapping names a known status and at least one
// ledger row, and every mapped ledger row and test exists.
import { existsSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { CONSUMER_MATRIX, parseConsumerMatrix, validateConsumerMatrix } from './issue-195-downstream.mjs';
import { buildIssue195Manifest } from './issue-195-requirements.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const matrix = parseConsumerMatrix(await readFile(path.join(root, CONSUMER_MATRIX), 'utf8'));
const manifest = await buildIssue195Manifest(root);
const errors = validateConsumerMatrix(matrix, {
  rowIds: new Set(manifest.atomicRequirements.map(({ id }) => id)),
  fileExists: (relative) => existsSync(path.join(root, relative)),
});
if (errors.length > 0) {
  console.error(`downstream consumer matrix check failed:\n- ${errors.join('\n- ')}`);
  process.exit(1);
}
const counts = Object.entries(matrix.consumers).map(([name, { mappings }]) => `${mappings.length} ${name} mappings`);
console.log(`downstream consumer matrix OK: ${counts.join(', ')}`);
