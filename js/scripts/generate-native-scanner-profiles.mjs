#!/usr/bin/env node
// Declarative profiles produce native scanner data; production runtimes
// execute the resulting Links operations without foreign scanner callbacks.
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { scannerFamilies } from './scanner-families.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const namingFile = 'parity/naming/grammar-name-expansions.json';

/** Canonical scanner declarations from a language-owned JSON profile. */
export function nativeScannerProfile(profile) {
  const generated = scannerFamilies(profile);
  const text = renderGrammarLinks(parseGrammarLinks(`(grammar (start unused))\n${generated}(rule unused normal (literal x))\n`));
  return `${text.split('\n').filter(line => line.startsWith('(scanner ') || line.startsWith('(rule ') && !line.startsWith('(rule unused ')).join('\n')}\n`;
}

function main(args) {
  const index = args.indexOf('--language');
  const wanted = index < 0 ? null : args[index + 1];
  const naming = JSON.parse(readFileSync(join(root, namingFile), 'utf8'));
  const profiles = Object.entries(naming.grammars).filter(([language, data]) => data.scannerFamilies && (wanted === null || wanted === language));
  if (!profiles.length) throw new Error(`No native scanner profile for ${wanted ?? 'the catalog'}`);
  for (const [language, data] of profiles) {
    const file = join(root, `parity/grammars/scanners/${language}.lino`);
    const expected = nativeScannerProfile(data.scannerFamilies);
    if (args.includes('--check')) {
      if (readFileSync(file, 'utf8') !== expected) throw new Error(`Generated native scanner profile differs: ${file}`);
    } else writeFileSync(file, expected);
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main(process.argv.slice(2));
