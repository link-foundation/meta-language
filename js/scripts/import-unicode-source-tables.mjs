// Convert official, pinned Unicode source data to inspectable shared Links.
// This author imports declarations; it never executes the represented algorithms.
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { join } from 'node:path';
import { writeProjectData } from './project-data.mjs';

const directory = process.argv[2];
if (!directory || process.argv[3] !== '--draft' || process.argv.length !== 4)
  throw new TypeError('use <temporary Unicode source directory> --draft');
const version = '17.0.0';
const sources = ['UnicodeData.txt', 'DerivedNormalizationProps.txt', 'DerivedCoreProperties.txt', 'Unicode-License.txt'].map(name => {
  const bytes = readFileSync(join(directory, name));
  return { name, url: name === 'Unicode-License.txt' ? 'https://www.unicode.org/license.txt' : 'https://www.unicode.org/Public/' + version + '/ucd/' + name,
    bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex'), text: bytes.toString('utf8') };
});
const propertyRows = text => text.split('\n').map(line => line.split('#')[0].trim()).filter(Boolean)
  .map(line => { const [range, property] = line.split(';').map(value => value.trim());
    const endpoints = range.split('..').map(value => Number.parseInt(value, 16));
    return { first: endpoints[0], last: endpoints.at(-1), property }; });
const exclusions = propertyRows(sources[1].text).filter(row => row.property === 'Full_Composition_Exclusion');
const properties = {};
for (const name of ['ID_Start', 'ID_Continue', 'XID_Start', 'XID_Continue'])
  properties[name] = propertyRows(sources[2].text).filter(row => row.property === name).flatMap(row => [row.first, row.last]);
const classes = [], decompositions = [], values = [], compositions = [];
for (const line of sources[0].text.split('\n')) {
  if (!line) continue;
  const fields = line.split(';');
  const scalar = Number.parseInt(fields[0], 16), combining = Number(fields[3]);
  if (combining !== 0) classes.push(scalar, combining);
  if (!fields[5] || fields[5].startsWith('<')) continue;
  const decomposition = fields[5].split(' ').map(value => Number.parseInt(value, 16));
  decompositions.push(scalar, values.length, decomposition.length);
  values.push(...decomposition);
  if (decomposition.length === 2 && !exclusions.some(row => scalar >= row.first && scalar <= row.last))
    compositions.push([decomposition[0] * 1114112 + decomposition[1], scalar]);
}
compositions.sort((left, right) => left[0] - right[0]);
const target = new URL('../src/data/unicode-source-tables.lino', import.meta.url);
writeProjectData(target, { schemaVersion: 1, unicode: version,
  generatedBy: 'js/scripts/import-unicode-source-tables.mjs',
  sources: sources.map(({ text, ...source }) => source), properties, classes, decompositions, values, pairs: compositions.flat(),
  boundary: 'Automatically converted official Unicode declarations. No algorithm, fixture or normalization conformance execution.' });
writeFileSync(new URL('../../rust/src/data/unicode-source-tables.lino', import.meta.url), readFileSync(target));
for (const prefix of ['../src/data/', '../../rust/src/data/'])
  writeFileSync(new URL(prefix + 'LICENSE-UNICODE.txt', import.meta.url), sources[3].text);
