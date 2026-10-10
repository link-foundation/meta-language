#!/usr/bin/env node
// Generates the concept records both runtimes ship from
// parity/naming/canonical-concepts.json: every canonical concept and
// operation with its stable identity, readable English phrase, role,
// definition, constraints, the names other sources give it, the former names
// that still decode to it, and the synonymous concepts it is distinct from.
//
//   node js/scripts/build-concept-records.mjs          # write both copies
//   node js/scripts/build-concept-records.mjs --check  # fail on drift
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const registerPath = join(root, 'parity/naming/canonical-concepts.json');
export const CONCEPT_RECORD_PATHS = Object.freeze([
  join(root, 'js/src/data/concept-records.json'),
  join(root, 'rust/src/data/concept-records.json'),
]);

const RECORD_FIELDS = Object.freeze([
  'id', 'phrase', 'role', 'definition', 'constraints', 'sourceAliases', 'formerNames', 'represents', 'distinctFrom',
]);

export function buildConceptRecords(register) {
  const owners = new Map();
  const concepts = register.concepts.map((concept) => {
    for (const name of [concept.id, ...concept.formerNames]) {
      const owner = owners.get(name);
      if (owner) throw new Error(`${name} names both ${owner} and ${concept.id}`);
      owners.set(name, concept.id);
    }
    const unknown = Object.keys(concept).filter((field) => !RECORD_FIELDS.includes(field));
    if (unknown.length > 0) throw new Error(`${concept.id} has fields the runtimes do not read: ${unknown.join(', ')}`);
    const record = {};
    for (const field of RECORD_FIELDS) {
      if (concept[field] !== undefined) record[field] = concept[field];
    }
    return record;
  });
  for (const concept of concepts) {
    if (concept.represents !== undefined && !concepts.some(({ id }) => id === concept.represents)) {
      throw new Error(`${concept.id} represents unknown concept ${concept.represents}`);
    }
  }
  return {
    generatedBy: 'js/scripts/build-concept-records.mjs',
    source: 'parity/naming/canonical-concepts.json',
    concepts,
  };
}

export function formatConceptRecords(records) {
  const concepts = records.concepts.map((concept) => `    ${JSON.stringify(concept)}`);
  const header = Object.entries(records)
    .filter(([key]) => key !== 'concepts')
    .map(([key, value]) => `  ${JSON.stringify(key)}: ${JSON.stringify(value)},`);
  return `{\n${header.join('\n')}\n  "concepts": [\n${concepts.join(',\n')}\n  ]\n}\n`;
}

async function main() {
  const register = JSON.parse(await readFile(registerPath, 'utf8'));
  const text = formatConceptRecords(buildConceptRecords(register));
  const distinctions = JSON.parse(await readFile(join(root, 'parity/required-concept-distinctions.json'), 'utf8'));
  const identities = new Set(register.concepts.map(({ id }) => id));
  const pairs = new Set();
  for (const { concepts, reason } of distinctions) {
    if (!Array.isArray(concepts) || concepts.length !== 2 || concepts[0] === concepts[1] || concepts.some((id) => !identities.has(id)) || typeof reason !== 'string' || !reason.endsWith('.')) throw new TypeError('required distinctions need two recorded concepts and an explanatory sentence');
    const pair = [...concepts].sort().join('\0');
    if (pairs.has(pair)) throw new TypeError('required distinction pairs must be unique');
    pairs.add(pair);
  }
  const generated = [
    [join(root, 'js/src/data/required-concept-distinctions.json'), JSON.stringify(distinctions, null, 2) + '\n'],
    [join(root, 'rust/src/data/required-concept-distinctions.rs'), '&[\n' + distinctions.map(({ concepts, reason }) => '    RequiredDistinction {\n        concepts: [' + concepts.map((id) => JSON.stringify(id)).join(', ') + '],\n        reason: ' + JSON.stringify(reason) + ',\n    },').join('\n') + '\n]\n'],
  ];
  const outputs = [...CONCEPT_RECORD_PATHS.map((path) => [path, text]), ...generated];
  if (process.argv.includes('--check')) {
    const stale = [];
    for (const [path, expected] of outputs) {
      const current = await readFile(path, 'utf8').catch(() => '');
      if (current !== expected) stale.push(path.slice(root.length + 1));
    }
    if (stale.length > 0) {
      console.error(`concept records are stale: ${stale.join(', ')}`);
      console.error('run: node js/scripts/build-concept-records.mjs');
      process.exit(1);
    }
    console.log(`concept records match the register for ${register.concepts.length} concepts`);
    return;
  }
  await Promise.all(outputs.map(([path, text]) => writeFile(path, text)));
  console.log(`wrote the concept records for ${register.concepts.length} concepts`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
