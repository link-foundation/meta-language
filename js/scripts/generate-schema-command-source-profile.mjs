// Capture complete immutable command sources; never edit their original files.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {SCHEMA_COMMAND_SOURCE_PROFILE,deriveSchemaCommandSourceView} from './schema-command-source-profile.mjs';

export function generateSchemaCommandSourceProfile({check=false}={}) {
  const specification=SCHEMA_COMMAND_SOURCE_PROFILE;
  const capture=file=>{
    const{blob,certificate}=captureGitArtifactCertificate(specification.revision,file);
    const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);assert.ok(blob.length<=131072);
    return{file,source,sourceText:source,commit:specification.revision,bytes:blob.length,sha256:createHash('sha256').update(blob).digest('hex'),original:{bytes:blob.length,sha256:createHash('sha256').update(blob).digest('hex')},certificate};
  };
  const sources=specification.files.map(capture),gateSources=[capture(specification.sourceGate)];
  for(const row of sources)row.view=deriveSchemaCommandSourceView(row.file,row.source,row.certificate).receipt;
  const packet={schemaVersion:1,revision:specification.revision,sources,gateSources,
    boundary:'Original immutable schema command membership only. Separate check-only source adapters validate actual current string-order schemas and retain all original assertions and command bytes. No Rust compilation or runtime observation.'};
  const output=new URL('../../'+specification.archive,import.meta.url);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packet=generateSchemaCommandSourceProfile({check:process.argv.includes('--check')});
  console.log('Authenticated '+packet.sources.length+' complete immutable schema command sources.');
}
