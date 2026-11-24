import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const check = process.argv.includes('--check');
const paths = process.argv.slice(2).filter(argument => argument !== '--check');
assert.equal(paths.length, 3, 'Provide Unicode 17.0.0 UnicodeData.txt, DerivedNormalizationProps.txt and NormalizationTest.txt');

// https://www.unicode.org/Public/17.0.0/ucd/
const digests = [
  '2e1efc1dcb59c575eedf5ccae60f95229f706ee6d031835247d843c11d96470c',
  '71fd6a206a2c0cdd41feb6b7f656aa31091db45e9cedc926985d718397f9e488',
  '5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db',
];
const [unicodeData, normalizationProperties, conformance] = paths.map((path, index) => {
  const bytes = readFileSync(path);
  assert.equal(createHash('sha256').update(bytes).digest('hex'), digests[index], path);
  return bytes.toString('utf8').split('\n').map(line => line.split('#', 1)[0].trim()).filter(Boolean);
});

const code = text => Number.parseInt(text, 16);
const combiningClasses = [];
const decompositions = new Map();
const canonicalMappings = new Map();
for (const line of unicodeData) {
  const [point, , , combiningClass, , mapping] = line.split(';');
  const scalar = code(point);
  if (Number(combiningClass)) combiningClasses.push(`(class ${point} ${combiningClass})`);
  if (!mapping) continue;
  const fields = mapping.split(' ');
  const canonical = !fields[0].startsWith('<');
  const sequence = (canonical ? fields : fields.slice(1)).map(code);
  decompositions.set(scalar, sequence);
  if (canonical && sequence.length === 2) canonicalMappings.set(scalar, sequence);
}

const excluded = new Set();
for (const line of normalizationProperties) {
  const [points, property] = line.split(';').map(field => field.trim());
  if (property !== 'Full_Composition_Exclusion') continue;
  const [start, end = start] = points.split('..').map(code);
  for (let scalar = start; scalar <= end; scalar++) excluded.add(scalar);
}

const sequence = values => values.map(value => value.toString(16)).join(' ');
const flattenedDecomposition = scalar => decompositions.get(scalar)?.flatMap(flattenedDecomposition) ?? [scalar];
const decompositionLinks = [...decompositions.keys()].map(scalar =>
  `(decompose ${sequence([scalar, ...flattenedDecomposition(scalar)])})`);
const compositionLinks = [...canonicalMappings].filter(([scalar]) => !excluded.has(scalar)).map(([scalar, parts]) =>
  `(compose ${sequence([...parts, scalar])})`);

const cases = conformance.filter(line => !line.startsWith('@')).map(line => {
  const fields = line.split(';').slice(0, 5).map(field => field.trim());
  assert.equal(fields.length, 5);
  // Every input field must produce the fourth field under NFKC.
  const samples = [...new Set([fields[3], ...fields])];
  return `(case ${samples.map(field => `(${field})`).join(' ')})`;
});
const outputs = [
  ['../src/data/text-location-normalization.lino', `(unicode-normalization 17.0.0 ${digests.slice(0, 2).join(' ')})\n${[...combiningClasses, ...decompositionLinks, ...compositionLinks].join('\n')}\n`],
  ['../../parity/fixtures/text-location-normalization-conformance.lino', `(unicode-normalization-conformance 17.0.0 ${digests[2]})\n${cases.join('\n')}\n`],
];
for (const [location, links] of outputs) {
  const path = new URL(location, import.meta.url);
  if (check) assert.equal(readFileSync(path, 'utf8'), links, path.pathname);
  else writeFileSync(path, links);
}
