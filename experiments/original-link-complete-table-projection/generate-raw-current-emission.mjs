// Opt-in source-only successor. The original generator/artifact are immutable.
// This driver never starts a native compiler, CST parser or observation matrix.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {RAW_CONSUMING_SNAPSHOT_HOST} from '../../js/scripts/raw-consuming-snapshot-source-profile.mjs';
import {withHistoricalJavaScriptModules} from '../historical-javascript-source.mjs';
import {regenerateHistoricalRawInstructions} from './historical-raw-generation.mjs';
import {
  ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT,
  createOriginalLinkRawCurrentArtifact,readOriginalLinkRawEmissionProducerSources,
} from './raw-emission-profiles.mjs';

export const ORIGINAL_LINK_RAW_COMPILER_REVISION='f73a10b22f8d0bcfbf4707621bcc9edd6989533d';
const originalInstructions='experiments/original-link-complete-table-projection/raw-rules.mjs';

/** Re-run the exact compiler whose complete codec output is independently pinned. */
export function withOriginalLinkRawEmissionCompiler(run) {
  return withHistoricalJavaScriptModules(ORIGINAL_LINK_RAW_COMPILER_REVISION,[originalInstructions],async directory=>{
    const readSource=path=>readFileSync(join(directory,path));
    const load=path=>import(pathToFileURL(join(directory,'js/src',path)).href);
    const [{parseJavaScript},{checkProgram},{emitRust},{emitJavaScript}]=await Promise.all([
      load('translation/javascript.js'),load('translation/check.js'),load('translation/emit-rust.js'),load('translation/emit-javascript.js'),
    ]);
    return run({readSource,load,parseJavaScript,checkProgram,emitRust,emitJavaScript});
  },[RAW_CONSUMING_SNAPSHOT_HOST]);
}

function prepareCurrentOriginalLinkRawEmission({parseJavaScript,checkProgram,emitRust},{check=true}={}) {
  const source=regenerateHistoricalRawInstructions();
  const historicalArtifact=readProjectData(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT);
  const producerSources=readOriginalLinkRawEmissionProducerSources();
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const rust=emitRust(program);
  const currentArtifact=createOriginalLinkRawCurrentArtifact({historicalArtifact,source,emission:rust,
    declarationNames:program.items.map(row=>row.fullName),producerSources});
  if(check)checkProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,currentArtifact);
  return {source,historicalArtifact,currentArtifact,producerSources,program,rust};
}

export function withPreparedOriginalLinkRawEmission(run,{check=true}={}) {
  return withOriginalLinkRawEmissionCompiler(compiler=>run(prepareCurrentOriginalLinkRawEmission(compiler,{check}),compiler));
}

export async function generateCurrentOriginalLinkRawEmission(check=false) {
  return withPreparedOriginalLinkRawEmission(({currentArtifact})=>{
    if(!check)writeProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,currentArtifact);
    return currentArtifact;
  },{check});
}

if(process.argv[1]===fileURLToPath(import.meta.url)) {
  const args=process.argv.slice(2);
  assert.ok(args.includes('--native')&&new Set(args).size===args.length&&args.every(arg=>['--native','--check'].includes(arg)),
    'explicit --native is required; only the read-only --check flag is also supported');
  const artifact=await generateCurrentOriginalLinkRawEmission(args.includes('--check'));
  console.log('Current raw emission profile: '+artifact.declarations.length+' complete definitions; '
    +artifact.profile.currentSourceBytes+' bytes; SHA256 '+artifact.profile.currentSourceSha256+'; native execution not selected');
  if(args.includes('--check')) {
    // Release the separate compiler worker when the active text and binary check completes.
    const worker=fileURLToPath(new URL('./active-raw-emission.mjs',import.meta.url));
    execFileSync(process.execPath,[...process.execArgv,worker,'--check'],{stdio:'inherit'});
  }
}
