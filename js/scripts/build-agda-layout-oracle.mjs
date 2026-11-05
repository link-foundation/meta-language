#!/usr/bin/env node
// CI-only C/WebAssembly build of the pinned Agda development oracle. The
// original released oracle and all of its fixture expectations remain intact.
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gunzipSync } from 'node:zlib';
import { makeScratchDirectory } from '../../scripts/lib/scratch.mjs';
import { readExternalGrammarAsset } from './external-grammar-cache.mjs';
import { readProjectData, resolveProjectDataPath, writeProjectData } from './project-data.mjs';
import { readGrammarLicense } from './grammar-files.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const AGDA_LAYOUT_ORACLE_OUTPUT = join(root, '.issue-195-work/agda-layout-oracle.wasm');
export const AGDA_LAYOUT_ORACLE_SOURCE = join(root, 'rust/vendor/tree-sitter-agda-layout');
const digest = value => createHash('sha256').update(value).digest('hex');
const sourceAsset = file => file === 'layout-state.patch'
  ? readFileSync(join(AGDA_LAYOUT_ORACLE_SOURCE, file))
  : file === 'LICENSE' ? readGrammarLicense(`rust/vendor/tree-sitter-agda-layout/${file}`)
    : readExternalGrammarAsset(`rust/vendor/tree-sitter-agda-layout/${file}`);

/** Verify the exact pinned sources, including the reproducible scanner-only patch. */
export function checkAgdaLayoutOracleSources() {
  const manifest = readProjectData(join(AGDA_LAYOUT_ORACLE_SOURCE, 'source-manifest.json'));
  for (const [file, hash] of Object.entries(manifest.files)) {
    if (digest(sourceAsset(file)) !== hash) throw new Error(`Agda layout oracle source digest differs: ${file}`);
  }
  const parser = gunzipSync(sourceAsset('src/parser.c.gz'));
  const scanner = gunzipSync(sourceAsset('src/scanner.c.upstream.gz'));
  if (digest(parser) !== manifest.parserSha256 || digest(scanner) !== manifest.originalScannerSha256
    || digest(sourceAsset('src/scanner.c')) !== manifest.scannerSha256) throw new Error('Agda layout oracle decoded source digest differs');
  const scratch = makeScratchDirectory('agda-layout-source-', 'verify Agda scanner patch');
  try {
    mkdirSync(join(scratch, 'src'));
    writeFileSync(join(scratch, 'src/scanner.c'), scanner);
    execFileSync('git', ['apply', join(AGDA_LAYOUT_ORACLE_SOURCE, 'layout-state.patch')], { cwd: scratch, stdio: 'pipe' });
    if (digest(readFileSync(join(scratch, 'src/scanner.c'))) !== manifest.scannerSha256) throw new Error('Agda layout scanner patch does not reproduce the build input');
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
  return manifest;
}

/** Retry transient compiler downloads without retrying compiler/source failures. */
export function runAgdaLayoutOracleBuild(arguments_,options,{execute=execFileSync,pause=milliseconds=>Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds),report=message=>process.stderr.write(message)}={}) {
  for(let attempt=0;attempt<3;attempt++){
    try{
      const result=execute('npx',arguments_,{...options,stdio:'pipe',maxBuffer:2*1024*1024});
      if(result?.length)report(String(result));
      return result;
    }catch(error){
      const output=[error?.stdout,error?.stderr].filter(Boolean).map(String).join('\n');
      const downloadFailure=/Failed to download/u.test(output)
        && /(?:returned error: (?:429|502|503|504)|Could not resolve host|Connection (?:reset|refused)|Failed to connect|SSL connection timeout)/u.test(output);
      if(!downloadFailure||attempt===2)throw error;
      report('Agda oracle compiler download failed transiently; retry '+(attempt+1)+'/2.\n'+output.slice(-4096)+'\n');
      pause(2000*(attempt+1));
    }
  }
}

export function buildAgdaLayoutOracle() {
  const manifest = checkAgdaLayoutOracleSources();
  const scratch = makeScratchDirectory('agda-layout-oracle-', 'build Agda development oracle');
  try {
    const grammar = join(scratch, 'grammar');
    for (const file of Object.keys(manifest.files)) {
      const target = join(grammar, file);
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(target, sourceAsset(file));
    }
    writeFileSync(join(grammar, 'src/parser.c'), gunzipSync(readFileSync(join(grammar, 'src/parser.c.gz'))));
    const prefix = ['--yes', '--package', `tree-sitter-cli@${manifest.treeSitterCli}`, 'tree-sitter'];
    const version = execFileSync('npx', [...prefix, '--version'], { cwd: root, encoding: 'utf8' }).trim();
    if (version.split(/\s+/u).slice(0, 2).join(' ') !== `tree-sitter ${manifest.treeSitterCli}`) throw new Error(`Unexpected Agda oracle compiler: ${version}`);
    mkdirSync(dirname(AGDA_LAYOUT_ORACLE_OUTPUT), { recursive: true });
    runAgdaLayoutOracleBuild([...prefix, 'build', '--wasm', '-o', AGDA_LAYOUT_ORACLE_OUTPUT, grammar], {
      cwd: scratch, timeout: 300_000,
    });
    const wasm = readFileSync(AGDA_LAYOUT_ORACLE_OUTPUT);
    writeProjectData(`${AGDA_LAYOUT_ORACLE_OUTPUT}.json`, {
      schemaVersion: 1, sourceManifestSha256: digest(readFileSync(resolveProjectDataPath(join(AGDA_LAYOUT_ORACLE_SOURCE, 'source-manifest.json')))),
      parserSha256: manifest.parserSha256, scannerSha256: manifest.scannerSha256,
      treeSitterCli: version, wasmSha256: digest(wasm),
    });
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--check-sources')) checkAgdaLayoutOracleSources();
  else buildAgdaLayoutOracle();
}
