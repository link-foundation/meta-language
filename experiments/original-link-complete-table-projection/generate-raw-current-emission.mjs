// Opt-in source-only successor. The original generator/artifact are immutable.
// This driver never starts a native compiler, CST parser or observation matrix.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {generateOriginalLinkRawJavaScript} from './generate-raw.mjs';
import {
  ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT,
  createOriginalLinkRawCurrentArtifact,readOriginalLinkRawEmissionProducerSources,
} from './raw-emission-profiles.mjs';

/** Check the actual complete current compiler result, not a patched old output. */
export function prepareCurrentOriginalLinkRawEmission({readProjectData,parseJavaScript,checkProgram,emitRust},{check=true}={}) {
  const source=generateOriginalLinkRawJavaScript(true);
  const historicalArtifact=readProjectData(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT);
  const producerSources=readOriginalLinkRawEmissionProducerSources();
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const rust=emitRust(program);
  const currentArtifact=createOriginalLinkRawCurrentArtifact({historicalArtifact,source,emission:rust,
    declarationNames:program.items.map(row=>row.fullName),producerSources});
  if(check)checkProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,currentArtifact);
  return {source,historicalArtifact,currentArtifact,producerSources,program,rust};
}

export async function generateCurrentOriginalLinkRawEmission(check=false) {
  const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([
    import('../../js/src/translation/javascript.js'),import('../../js/src/translation/check.js'),import('../../js/src/translation/emit-rust.js'),
  ]);
  const prepared=prepareCurrentOriginalLinkRawEmission({readProjectData,parseJavaScript,checkProgram,emitRust},{check});
  if(!check)writeProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,prepared.currentArtifact);
  return prepared.currentArtifact;
}

if(process.argv[1]===fileURLToPath(import.meta.url)) {
  const args=process.argv.slice(2);
  assert.ok(args.includes('--native')&&new Set(args).size===args.length&&args.every(arg=>['--native','--check'].includes(arg)),
    'explicit --native is required; only the read-only --check flag is also supported');
  const artifact=await generateCurrentOriginalLinkRawEmission(args.includes('--check'));
  console.log('Current raw emission profile: '+artifact.declarations.length+' complete definitions; '
    +artifact.profile.currentSourceBytes+' bytes; SHA256 '+artifact.profile.currentSourceSha256+'; native execution not selected');
}
