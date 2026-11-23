import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { deflateSync, inflateSync } from 'node:zlib';
import { readNamedZipEntries } from '../src/open-packaging-conventions.js';

const version = '2016.01.19';
const archiveDigest = '561266f842d6aa09da09fbf68e919ee9e1d32bbdcd435e6a0df51546d35a4e91';
const [input, option] = process.argv.slice(2);
assert.ok(input && (!option || option === '--check'), 'Provide the AGID 2016.01.19 ZIP archive');
const require = createRequire(import.meta.url);
const wordNetDirectory = dirname(require.resolve('wordnet-db/package.json'));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const wordNetFiles = new Map([
  ['adj', '7d2b4d4037f0571ca309ce2ebadbe860a1607d494a70cd40f9bd61bc25a57016'],
  ['adv', '0ea0811bb06eeeb973805ab0b066cb724ebc9dc01f07583582b59579a5b4a36c'],
]);
const archive = readFileSync(input);
assert.equal(digest(archive), archiveDigest);
const prefix = 'agid-' + version + '/';
const files = readNamedZipEntries(archive, new Map([[prefix + 'infl.txt', 3486996], [prefix + 'README', 15838]]));

// AGID combines adjectives and adverbs into one source class. WordNet
// identifies each supported part of speech without assigning an invented tag.
const wordNet = new Map([...wordNetFiles].map(([part, expected]) => {
  const bytes = readFileSync(join(wordNetDirectory, 'dict', 'index.' + part));
  assert.equal(digest(bytes), expected);
  const lemmas = bytes.toString('utf8').split('\n').filter(line => line && !line.startsWith(' '))
    .map(line => line.slice(0, line.indexOf(' ')));
  return [part, new Set(lemmas)];
}));
const noun = ['NOUN Number=Sing', 'NOUN Number=Plur'];
const adjective = ['ADJ Degree=Pos', 'ADJ Degree=Cmp', 'ADJ Degree=Sup'];
const adverb = ['ADV Degree=Pos', 'ADV Degree=Cmp', 'ADV Degree=Sup'];
const infinitive = 'VERB VerbForm=Inf';
const past = 'VERB Tense=Past VerbForm=Fin';
const pastParticiple = 'VERB Tense=Past VerbForm=Part';
const presentParticiple = 'VERB Tense=Pres VerbForm=Part';
const thirdPerson = 'VERB Number=Sing Person=3 Tense=Pres VerbForm=Fin';
const present = 'VERB Tense=Pres VerbForm=Fin';
const featureBundles = [
  ...noun, ...adjective, ...adverb, infinitive, past, pastParticiple, presentParticiple, thirdPerson, present,
  'VERB Number=Sing Person=1 Tense=Pres VerbForm=Fin', 'VERB Number=Plur Tense=Pres VerbForm=Fin',
];
const bundleIds = new Map(featureBundles.map((bundle, index) => [bundle, index]));
const rows = [];
const seen = new Set();
function add(lemma, spelling, bundle, weight) {
  const id = bundleIds.get(bundle);
  assert.notEqual(id, undefined, bundle);
  const row = `(${spelling.toLowerCase()} ${lemma} ${weight} ${id})`;
  if (!seen.has(row)) {
    seen.add(row);
    rows.push(row);
  }
}

function groupsFor(lemma, part, size) {
  if (part === 'N') return [[noun[0]], [[noun[1]]]];
  if (part === 'A') {
    const bundles = [];
    if (wordNet.get('adj').has(lemma)) bundles.push(adjective);
    if (wordNet.get('adv').has(lemma)) bundles.push(adverb);
    const degree = index => bundles.map(bundle => bundle[index]);
    return [degree(0), size === 1 ? [degree(2)] : [degree(1), degree(2)]];
  }
  const base = [infinitive];
  if (lemma === 'be') return [base, [
    [past], [past], [pastParticiple], [presentParticiple], [featureBundles[14]], [present], [thirdPerson], [featureBundles[15]],
  ]];
  if (lemma === 'wit') return [base, [[past, pastParticiple], [presentParticiple], [presentParticiple], [present], [present]]];
  if (size === 1) return [base, [[past]]];
  if (size === 2) return [base, [[past], [present]]];
  return [base, [
    [past, ...(size === 3 ? [pastParticiple] : [])], ...(size === 4 ? [[pastParticiple]] : []),
    [presentParticiple], [thirdPerson],
  ]];
}

for (const line of files.get(prefix + 'infl.txt').toString('ascii').trimEnd().split('\r\n')) {
  const match = /^([A-Za-z']+) ([NAV])(\??): (.+)$/u.exec(line);
  assert.ok(match, line);
  const [, word, part, tentative, values] = match;
  // A question-marked part of speech was inferred solely from spelling.
  if (tentative) continue;
  const lemma = word.toLowerCase();
  const groups = values.split(' | ');
  const [base, assignments] = groupsFor(lemma, part, groups.length);
  assert.equal(groups.length, assignments.length, line);
  for (const bundle of base) add(lemma, word, bundle, 4);
  for (const [index, group] of groups.entries()) {
    for (const value of group.split(', ')) {
      if (value === '-') continue;
      const form = /^([A-Za-z']+)([~<!?]*)(?: (\d+(?:\.\d+)?))?(?: \{[^}]+\})?$/u.exec(value);
      assert.ok(form, line);
      const [, spelling, marks, variant = '0'] = form;
      // Marked forms and level-two variants can lack lexical evidence.
      if (marks || Number(variant) >= 2) continue;
      for (const bundle of assignments[index]) add(lemma, spelling, bundle, Number(variant) < 1 ? 4 : 2);
    }
  }
}
const key = (row, field) => row.split(' ')[field];
const weight = row => Number(key(row, 2));
const sortedRows = (items, field) => items.map(row => ({ row, key: key(row, field), weight: weight(row) }))
  .sort((left, right) => (left.key < right.key ? -1 : left.key > right.key ? 1 : right.weight - left.weight))
  .map(({ row }) => row);
const byForm = sortedRows(rows, 0);
const lookup = new Map();
let offset = 0;
for (const row of byForm) {
  lookup.set(row, offset);
  offset += row.length + 1;
}
const table = byForm.join('\n') + '\n';
const byLemma = sortedRows(rows, 1);
const offsets = Buffer.alloc(byLemma.length * 4);
for (const [index, row] of byLemma.entries()) offsets.writeUInt32LE(lookup.get(row), index * 4);

const encoded = Buffer.concat([Buffer.from(table, 'ascii'), offsets]);
const payload = deflateSync(encoded, { level: 9 });
const hash = digest(encoded);
const header = `(english-morphology 1 ${table.length} ${byLemma.length} ${hash})`;
const pins = [
  `(source agid ${version} ${archiveDigest})`,
  ...Array.from(wordNetFiles, ([part, sha256]) => `(source wordnet-3.1 index.${part} ${sha256})`),
];
const featureRows = featureBundles.map(bundle => `(bundle ${bundle})`);
const data = payload.toString('base64').match(/.{1,4096}/gu).map(block => `(data ${block})`);
const dictionary = [header, ...pins, ...featureRows, ...data, ''].join('\n');
const wordNetLicense = readFileSync(join(wordNetDirectory, 'LICENSE'));
assert.equal(digest(wordNetLicense), 'c4163fc9f8aea62a084f355753bf6d0c54d2582384eae13574e8847aa6bc227e');
const agidLicense = files.get(prefix + 'README').toString('utf8').replaceAll('\r\n', '\n').trimEnd();
const license = (agidLicense + '\n\n' + wordNetLicense.toString('utf8').replaceAll('\r\n', '\n').trimEnd() + '\n')
  .replace(/[ \t]+$/gmu, '');
for (const directory of ['../src/data/morphology/', '../../rust/src/data/morphology/']) {
  const dictionaryPath = new URL(directory + 'english.lino', import.meta.url);
  const licensePath = new URL(directory + 'ENGLISH-MORPHOLOGY-LICENSE.txt', import.meta.url);
  // zlib releases may choose a different compressed representation.
  if (option) {
    const existing = readFileSync(dictionaryPath, 'ascii').trimEnd().split('\n');
    const spelling = existing.splice(0, 1 + pins.length + featureRows.length);
    assert.deepEqual(spelling, [header, ...pins, ...featureRows]);
    const bytes = inflateSync(Buffer.from(existing.map(row => row.slice(6, -1)).join(''), 'base64'), {
      maxOutputLength: encoded.length,
    });
    assert.ok(bytes.equals(encoded), dictionaryPath.pathname);
    assert.equal(readFileSync(licensePath, 'utf8'), license, licensePath.pathname);
  } else {
    writeFileSync(dictionaryPath, dictionary);
    writeFileSync(licensePath, license);
  }
}
console.log(`English morphology contains ${rows.length} readings with AGID ${version} and WordNet 3.1`);
