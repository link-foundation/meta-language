import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';
import { openRussianMorphology, russianMorphologySource } from './morphology-dictionary.js';
import { readNamedZipEntries } from './open-packaging-conventions.js';

const [directory, source] = process.argv.slice(2);
if (!directory) throw new Error('usage: npm run morphology:provision -- DIRECTORY [PINNED_WHEEL]');
const root = resolve(directory);
const destination = join(root, 'ru');
const specification = russianMorphologySource();
const digest = bytes => createHash('sha256').update(bytes).digest('hex');

if (existsSync(destination)) {
  for (const [name, { size, digest: expected }] of specification.files) {
    const bytes = readFileSync(join(destination, name));
    if (bytes.length !== size || digest(bytes) !== expected) throw new Error(`invalid existing morphology data at ${destination}/${name}`);
  }
} else {
  let bytes;
  if (source) bytes = readFileSync(source);
  else {
    const response = await fetch(specification.url, { redirect: 'error' });
    if (!response.ok) throw new Error(`dictionary download failed with HTTP ${response.status}`);
    const chunks = [];
    let size = 0;
    for await (const chunk of response.body) {
      size += chunk.length;
      if (size > 10 * 1024 * 1024) throw new Error('dictionary download exceeds its size limit');
      chunks.push(chunk);
    }
    bytes = Buffer.concat(chunks);
  }
  if (digest(bytes) !== specification.sha256) throw new Error('invalid source digest for the OpenCorpora snapshot');
  const prefix = 'pymorphy3_dicts_ru/data/';
  const names = new Map([...specification.files].map(([name, { size }]) => [prefix + name, size]));
  const entries = readNamedZipEntries(bytes, names);
  mkdirSync(root, { recursive: true });
  const temporary = mkdtempSync(join(root, '.morphology-'));
  try {
    for (const [name, content] of entries) {
      const file = basename(name);
      if (digest(content) !== specification.files.get(file).digest) throw new Error(`invalid digest for ${file}`);
      writeFileSync(join(temporary, file), content, { flag: 'wx' });
    }
    const license = new URL('./data/morphology/OPEN-CORPORA-LICENSE.txt', import.meta.url);
    writeFileSync(join(temporary, 'LICENSE.txt'), readFileSync(license), { flag: 'wx' });
    openRussianMorphology(temporary);
    renameSync(temporary, destination);
  } finally {
    if (existsSync(temporary)) rmSync(temporary, { recursive: true });
  }
}
console.log(`OpenCorpora ${specification.version} is installed at ${destination}`);
