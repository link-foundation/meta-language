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
import { lstatSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, checkProjectData, serializeProjectData, writeProjectData } from './project-data.mjs';
import { buildConceptRecords, CONCEPT_RECORD_PATHS } from './build-concept-records.mjs';
import { readBootstrapStructuredDataLino, structuredDataToLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

import { checkFoundationModels } from '../src/foundation-models.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const FOUNDATION_REGISTER = 'parity/foundation-models.json';
export const FOUNDATION_MODEL_PATHS = Object.freeze(['js/src/data/foundation-models.lino', 'rust/src/data/foundation-models.lino']);

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

// A command's descriptive identity does not supply a language's foundation.
// Accept only exact generated snapshots and operation identities whose complete
// source ownership stays in real, non-symlinked authoring scripts. Every other
// reference keeps the ordinary rejection path, including stale or altered data.
function toolingConceptMetadata(base) {
  try {
    const register = readBootstrapStructuredDataLino(readFileSync(join(base, 'parity/naming/canonical-concepts.lino'), 'utf8'));
    const records = buildConceptRecords(register);
    const permitted = new Set();
    for (const record of records.concepts) {
      if (!CONSUMER_FOUNDATION_NAMES.some(pattern => pattern.test(JSON.stringify(record)))) continue;
      if (record.role !== 'operation' || !Array.isArray(record.sourceAliases) || record.sourceAliases.length === 0 ||
          !record.sourceAliases.every(({ source }) => {
            if (typeof source !== 'string' || !/^js\/scripts\/(?:[A-Za-z0-9_-]+\/)*[A-Za-z0-9_-]+\.mjs$/u.test(source)) return false;
            const status = lstatSync(join(base, source));
            return status.isFile() && !status.isSymbolicLink();
          })) return null;
      for (const line of structuredDataToLino(record.id).trim().split('\n')) {
        if (CONSUMER_FOUNDATION_NAMES.some(pattern => pattern.test(line))) {
          permitted.add(line.replace(/^\(\d+:/u, '(identity:'));
        }
      }
    }
    return { text: serializeProjectData(records, CONCEPT_RECORD_PATHS[0]), permitted };
  } catch {
    return null;
  }
}

/** Production source lines that name a consumer's foundation outside the consumer registries. */
export function consumerFoundationReferences(base = root) {
  const found = [];
  const conceptPaths = CONCEPT_RECORD_PATHS.map(file => relative(root, file).split('\\').join('/'));
  let metadata;
  for (const directory of NEUTRAL_SOURCE_ROOTS) {
    for (const file of sourceFiles(join(base, directory))) {
      const path = relative(base, file).split('\\').join('/');
      if (CONSUMER_REGISTRIES.includes(path)) continue;
      const source = readFileSync(file, 'utf8');
      const references = [];
      source.split('\n').forEach((line, index) => {
        if (CONSUMER_FOUNDATION_NAMES.some((pattern) => pattern.test(line))) references.push({ path, line: index + 1, text: line.trim() });
      });
      if (references.length === 0) continue;
      if (conceptPaths.includes(path)) {
        if (metadata === undefined) metadata = toolingConceptMetadata(base);
        if (metadata?.text === source && references.every(({ text }) =>
          metadata.permitted.has(text.replace(/^\(\d+:/u, '(identity:')))) continue;
      }
      found.push(...references);
    }
  }
  return found;
}

async function main() {
  const register = readProjectData(join(root, FOUNDATION_REGISTER));
  const check = process.argv.includes('--check');
  if (!check) await Promise.all(FOUNDATION_MODEL_PATHS.map((path) => writeProjectData(join(root, path), register)));
  const problems = checkFoundationModels(register).map(({ message }) => message);
  for (const { path, line, text: source } of consumerFoundationReferences()) {
    problems.push(`${path}:${line} names a consumer's own foundation in production code: ${source}`);
  }
  if (check) {
    for (const path of FOUNDATION_MODEL_PATHS) {
      try { checkProjectData(join(root, path), register); }
      catch { problems.push(`${path} is stale; run: node js/scripts/build-foundation-models.mjs`); }
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
  const { models, correspondences } = register;
  console.log(`foundation models are neutral: ${models.length} models, ${correspondences.length} correspondences`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
