import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { inflateSync } from 'node:zlib';

let dictionary;

function readEnglishDictionary(file) {
  const lines = readFileSync(file, 'ascii').trimEnd().split('\n');
  const [, version, length, count, digest] = lines.shift().slice(1, -1).split(' ');
  const textSize = Number(length);
  const rowCount = Number(count);
  const blocks = [];
  const features = [];
  for (const line of lines) {
    if (line.startsWith('(bundle ')) features.push(line.slice(8, -1).split(' '));
    else if (line.startsWith('(data ')) blocks.push(line.slice(6, -1));
  }
  const size = textSize + rowCount * 4;
  if (version !== '1' || !Number.isSafeInteger(textSize) || textSize < 1 ||
      !Number.isSafeInteger(rowCount) || rowCount < 1 || size > 8 * 1024 * 1024) {
    throw new Error('invalid English morphology data header');
  }
  const bytes = inflateSync(Buffer.from(blocks.join(''), 'base64'), { maxOutputLength: size });
  if (bytes.length !== size || createHash('sha256').update(bytes).digest('hex') !== digest) {
    throw new Error('invalid English morphology data digest');
  }
  const table = bytes.toString('ascii', 0, textSize);
  const forms = new Uint32Array(rowCount);
  const lemmas = new Uint32Array(rowCount);
  let offset = 0;
  for (let index = 0; index < rowCount; index += 1) {
    forms[index] = offset;
    offset = table.indexOf('\n', offset) + 1;
    lemmas[index] = bytes.readUInt32LE(textSize + index * 4);
  }
  if (offset !== textSize) throw new Error('invalid AGID English morphology row count');
  return { table, features, forms, lemmas };
}

// Each index is sorted by its lookup spelling. A query decodes only the
// matching rows, keeping the expanded dictionary free of per-word objects.
export function englishMorphology(word, byLemma = false) {
  dictionary ??= readEnglishDictionary(new URL('./data/morphology/english.lino', import.meta.url));
  const { table, features, forms, lemmas } = dictionary;
  const offsets = byLemma ? lemmas : forms;
  const key = offset => {
    const start = byLemma ? table.indexOf(' ', offset) + 1 : offset + 1;
    return table.slice(start, table.indexOf(' ', start));
  };
  let left = 0;
  let right = offsets.length;
  while (left < right) {
    const middle = Math.floor((left + right) / 2);
    if (key(offsets[middle]) < word) left = middle + 1;
    else right = middle;
  }
  const readings = [];
  for (let index = left; index < offsets.length && key(offsets[index]) === word; index += 1) {
    const offset = offsets[index];
    const [form, lemma, weight, feature] = table.slice(offset + 1, table.indexOf('\n', offset) - 1).split(' ');
    const [upos, ...tags] = features[Number(feature)];
    readings.push({ form, lemma, weight: Number(weight), upos, features: tags, source: 'dictionary' });
  }
  return readings;
}
