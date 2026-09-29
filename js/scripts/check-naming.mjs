#!/usr/bin/env node
// Checks the readable English names of docs/vision.md#readable-english-names:
// the concept records in parity/naming/canonical-concepts.json against WordNet
// 3.1, the technical vocabulary and the abbreviation register; every name the
// sources define against the records; and the positive and negative fixtures
// in parity/naming/naming-fixtures.json against the check itself.
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadWordNet } from './english-vocabulary.mjs';
import { checkNamingFixtures, checkRepositoryNames, loadNamingRegisters, NAMING_FIXTURES } from './issue-195-naming.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const wordnet = loadWordNet();
const { vocabulary } = loadNamingRegisters(root);
const { fixtures } = JSON.parse(readFileSync(path.join(root, NAMING_FIXTURES), 'utf8'));
const { problems, records, names, formers } = checkRepositoryNames(root, wordnet);
problems.push(...checkNamingFixtures(fixtures, { wordnet, vocabulary }));
if (problems.length > 0) {
  console.error(`naming check failed (${problems.length} problems):\n- ${problems.map((problem) => `[${problem.kind}] ${problem.message}`).join('\n- ')}`);
  process.exit(1);
}
const inventories = new Set(names.map(({ inventory }) => inventory)).size;
console.log(
  `naming OK: ${records.length} concept records cover ${names.length} source names in ${inventories} inventories and ${formers.length} former names, ` +
    `checked against ${wordnet.source}; ${fixtures.length} fixtures judged as expected`,
);
