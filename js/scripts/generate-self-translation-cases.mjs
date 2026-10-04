#!/usr/bin/env node
// Generates parity/self-translation/expected/: for every case of
// parity/self-translation/cases.lino, the self-translation of its source and
// the listing of its items (`start..end term status (reason)`, as the
// `translate --items` command prints it). Both runtimes check their
// self-translation against these files, translate each expected file back to
// its source byte for byte, and run the case's calls on their own side.
//
//   node js/scripts/generate-self-translation-cases.mjs          # write
//   node js/scripts/generate-self-translation-cases.mjs --check  # verify
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { Parser } from 'links-notation';

import { selfTranslate } from '../src/self-translation.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const corpus = join(root, 'parity/self-translation');

/** The links of `text` as nested arrays of reference names. */
const tree = (link) => (link.values.length ? link.values.map(tree) : link.id);

/** The cases and calls of cases.lino. */
export async function readSelfTranslationCorpus() {
  const statements = new Parser().parse(await readFile(join(corpus, 'cases.lino'), 'utf8')).map(tree);
  const field = (statement, name) => statement.find((entry) => Array.isArray(entry) && entry[0] === name)?.slice(1);
  const cases = statements.filter(([kind]) => kind === 'case').map((statement) => ({
    id: statement[1],
    source: field(statement, 'source')[0],
    from: field(statement, 'from')[0],
    to: field(statement, 'to')[0],
    expected: field(statement, 'expected')[0],
  }));
  const calls = statements.filter(([kind]) => kind === 'call').map((statement) => ({
    case: statement[1],
    javascript: field(statement, 'javascript')[0],
    rust: field(statement, 'rust')[0],
    arguments: field(statement, 'arguments').map(([type, value]) => ({ type, value })),
    result: field(statement, 'result')[0],
  }));
  return { cases, calls };
}

/** The items of a self-translation as Links Notation. */
export function itemLinks(translation) {
  return translation.items.map(({ start, end, term, status, reason }) => `(item ${start} ${end} ${term} ${status}${reason ? ` "${reason}"` : ''})\n`).join('');
}

async function main() {
  const check = process.argv.includes('--check');
  const { cases } = await readSelfTranslationCorpus();
  const stale = [];
  for (const entry of cases) {
    const translation = selfTranslate(await readFile(join(corpus, entry.source), 'utf8'), entry.from, entry.to);
    for (const [file, text] of [[entry.expected, translation.code], [`${entry.expected}.items.lino`, itemLinks(translation)]]) {
      const path = join(corpus, file);
      if (check) {
        if ((await readFile(path, 'utf8').catch(() => null)) !== text) stale.push(file);
      } else {
        await writeFile(path, text);
      }
    }
  }
  if (stale.length) {
    console.error(`parity/self-translation is stale (${stale.join(', ')}): run node js/scripts/generate-self-translation-cases.mjs`);
    process.exit(1);
  }
  if (!check) console.log(`wrote ${cases.length} self-translation cases`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) await main();
