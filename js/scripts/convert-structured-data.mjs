// Automatically convert structured JSON input to inspectable Links Notation.
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { isDeepStrictEqual } from 'node:util';
import { pathToFileURL } from 'node:url';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

export function convertStructuredData(source) {
  const value = JSON.parse(source);
  const lino = structuredDataToLino(value);
  const decoded = readStructuredDataLino(lino);
  if (!isDeepStrictEqual(value, decoded)) throw new Error('structured data conversion changed a value');
  function ordered(first, second) {
    if (!first || typeof first !== 'object') return;
    if (!isDeepStrictEqual(Object.keys(first), Object.keys(second))) throw new Error('structured data conversion changed collection order');
    for (const name of Object.keys(first)) ordered(first[name], second[name]);
  }
  ordered(value, decoded);
  return lino;
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  const positional = process.argv.slice(2).filter(argument => !argument.startsWith('--'));
  if (positional.length < 1 || positional.length > 2 || process.argv.slice(2).some(argument => argument.startsWith('--') && argument !== '--check')) {
    throw new Error('usage: convert-structured-data.mjs input.json [output.lino] [--check]');
  }
  const [input, output = input.replace(/\.json$/u, '') + '.lino'] = positional;
  const lino = convertStructuredData(readFileSync(input, 'utf8'));
  if (process.argv.includes('--check')) {
    const actual = readFileSync(output, 'utf8');
    if (actual !== lino || !isDeepStrictEqual(readStructuredDataLino(actual), JSON.parse(readFileSync(input, 'utf8')))) throw new Error(`structured data conversion is stale: ${output}`);
  } else writeFileSync(output, lino);
}
