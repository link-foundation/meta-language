// The corrected scanner is a separate independently compiled development
// oracle. Ordinary parsing and the original pinned oracle are unchanged.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { loadGrammarLanguage } from '../src/grammar-tiering.js';
import { oracleSnapshotFromGrammar } from './native-grammar-rows.mjs';
import { AGDA_LAYOUT_ORACLE_OUTPUT, AGDA_LAYOUT_ORACLE_SOURCE } from './build-agda-layout-oracle.mjs';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';

const digest = value => createHash('sha256').update(value).digest('hex');
let grammar;

/** Every source compares with the pinned grammar and the complete scanner-state oracle. */
export function agdaLayoutOracleSnapshot(source) {
  if (!grammar) {
    const wasm = readFileSync(AGDA_LAYOUT_ORACLE_OUTPUT);
    const build = readProjectData(`${AGDA_LAYOUT_ORACLE_OUTPUT}.json`);
    const sourceManifestFile = join(AGDA_LAYOUT_ORACLE_SOURCE, 'source-manifest.json');
    const sourceManifest = readFileSync(resolveProjectDataPath(sourceManifestFile));
    const manifest = readProjectData(sourceManifestFile);
    if (build.wasmSha256 !== digest(wasm) || build.sourceManifestSha256 !== digest(sourceManifest)
      || build.parserSha256 !== manifest.parserSha256 || build.scannerSha256 !== manifest.scannerSha256) throw new Error('Agda layout oracle build does not match its pinned inputs');
    grammar = loadGrammarLanguage(wasm);
  }
  return oracleSnapshotFromGrammar(source, grammar);
}
