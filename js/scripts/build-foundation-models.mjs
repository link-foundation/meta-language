#!/usr/bin/env node
// Copies the foundation register parity/foundation-models.json into both
// runtimes and checks that foundations stay neutral: the register passes
// checkFoundationModels (every model is data in a declared family, distinct
// models stay distinct, every correspondence states its condition, and no
// model is a universal logic), and no production source names a consumer's
// own foundation, which the consumer brings instead of meta-language.
//
//   node js/scripts/build-foundation-models.mjs          # write both copies
//   node js/scripts/build-foundation-models.mjs --check  # fail on drift or a violation
import { readFileSync, readdirSync } from 'node:fs';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { checkFoundationModels } from '../src/foundation-models.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const FOUNDATION_REGISTER = 'parity/foundation-models.json';
export const FOUNDATION_MODEL_PATHS = Object.freeze(['js/src/data/foundation-models.json', 'rust/src/data/foundation-models.json']);

/** Production source directories whose code must stay foundation-neutral. */
export const NEUTRAL_SOURCE_ROOTS = Object.freeze(['js/src', 'rust/src']);

/**
 * Names of a downstream consumer's own foundation. The consumer registries
 * that pin its corpora as parity evidence may name it; no other production
 * source may, so its syntax, foundation or proof authority is never built in.
 */
export const CONSUMER_FOUNDATION_NAMES = Object.freeze([/relative[-_ ]?meta[-_ ]?logic/iu, /\bRML\b/u]);
export const CONSUMER_REGISTRIES = Object.freeze(['rust/src/parity.rs', 'rust/src/parity_fixtures.rs']);

const SOURCE_EXTENSIONS = /\.(?:js|mjs|ts|rs|json|lino)$/u;

function sourceFiles(directory) {
  const files = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (entry.name !== 'vendor') files.push(...sourceFiles(path));
    } else if (SOURCE_EXTENSIONS.test(entry.name)) {
      files.push(path);
    }
  }
  return files;
}

/** Production source lines that name a consumer's foundation outside the consumer registries. */
export function consumerFoundationReferences(base = root) {
  const found = [];
  for (const directory of NEUTRAL_SOURCE_ROOTS) {
    for (const file of sourceFiles(join(base, directory))) {
      const path = relative(base, file).split('\\').join('/');
      if (CONSUMER_REGISTRIES.includes(path)) continue;
      readFileSync(file, 'utf8').split('\n').forEach((line, index) => {
        if (CONSUMER_FOUNDATION_NAMES.some((pattern) => pattern.test(line))) found.push({ path, line: index + 1, text: line.trim() });
      });
    }
  }
  return found;
}

async function main() {
  const text = await readFile(join(root, FOUNDATION_REGISTER), 'utf8');
  const check = process.argv.includes('--check');
  if (!check) await Promise.all(FOUNDATION_MODEL_PATHS.map((path) => writeFile(join(root, path), text)));
  const problems = checkFoundationModels(JSON.parse(text)).map(({ message }) => message);
  for (const { path, line, text: source } of consumerFoundationReferences()) {
    problems.push(`${path}:${line} names a consumer's own foundation in production code: ${source}`);
  }
  if (check) {
    for (const path of FOUNDATION_MODEL_PATHS) {
      const current = await readFile(join(root, path), 'utf8').catch(() => '');
      if (current !== text) problems.push(`${path} is stale; run: node js/scripts/build-foundation-models.mjs`);
    }
  }
  if (problems.length > 0) {
    for (const problem of problems) console.error(problem);
    process.exit(1);
  }
  if (!check) {
    console.log(`wrote the foundation models to ${FOUNDATION_MODEL_PATHS.join(' and ')}`);
    return;
  }
  const { models, correspondences } = JSON.parse(text);
  console.log(`foundation models are neutral: ${models.length} models, ${correspondences.length} correspondences`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
