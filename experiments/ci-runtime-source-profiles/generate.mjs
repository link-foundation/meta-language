// Retain complete original sources while extending declared CI runtime profiles.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {readProjectData,serializeProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
const root=new URL('../../',import.meta.url),revision='478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8';
const files=['.gitattributes','js/scripts/schema-command-source-profile.mjs',
  'js/scripts/generate-program-string-representation-rules.mjs',
  'js/tests/translation-schema-command-source-views.test.js','js/tests/translation-package-source-profile.test.js',
  'parity/self-translation/schema-command-source-profile.lino','parity/self-translation/program-string-representation-rules.lino'];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Capture every predecessor byte and invert precisely the current edit. */
export function generateCiRuntimeSourceProfiles({check=false}={}) {
  const sources=files.map(file=>{
    const{blob,certificate}=captureGitArtifactCertificate(revision,file),actual=readFileSync(new URL(file,root));
    assert.deepEqual(Buffer.from(blob.toString()),blob);assert.deepEqual(Buffer.from(actual.toString()),actual);
    let start=0,end=actual.length,oldEnd=blob.length;
    while(start<end&&start<oldEnd&&actual[start]===blob[start])start++;
    while(start>0&&(actual[start]&192)===128)start--;
    while(end>start&&oldEnd>start&&actual[end-1]===blob[oldEnd-1]){end--;oldEnd--;}
    while(end<actual.length&&((actual[end]&192)===128||(blob[oldEnd]&192)===128)){end++;oldEnd++;}
    const removed=blob.subarray(start,oldEnd),inserted=actual.subarray(start,end);
    assert.deepEqual(Buffer.from(removed.toString()),removed);assert.deepEqual(Buffer.from(inserted.toString()),inserted);
    assert.deepEqual(Buffer.concat([actual.subarray(0,start),removed,actual.subarray(end)]),blob);
    return{file,commit:revision,...pin(blob),sourceText:blob.toString(),certificate,current:pin(actual),
      edit:{offsetUnit:'UTF-8 bytes',start,end,removed:removed.toString(),inserted:inserted.toString()}};
  });
  const schemaFile=files[5],programFile=files[6];
  const schema=readProjectData(new URL(schemaFile,root)),originalSchema=readStructuredDataLino(sources[5].sourceText);
  assert.equal(schema.sources.length,2);
  for(const row of schema.sources){assert.equal(row.view.nodeRange,'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0');row.view.nodeRange='>=24.0.0 <25.0.0';}
  assert.deepEqual(schema,originalSchema);assert.equal(serializeProjectData(schema,schemaFile),sources[5].sourceText);
  const program=readProjectData(new URL(programFile,root));
  assert.equal(program.nodeRange,'>=22.0.0 <23.0.0 || >=24.0.0 <25.0.0');program.nodeRange='>=24.0.0 <25.0.0';
  assert.equal(serializeProjectData(program,programFile),sources[6].sourceText);
  const packet={schemaVersion:1,revision,sources,
    synchronousHooks:{minimum:'22.15.0',source:'https://nodejs.org/download/release/v22.15.0/docs/api/module.html#moduleregisterhooksoptions',
      coveredRange:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0'},
    sourceBytes:'Canonical native unit source and the version-qualified package manifest retain LF bytes with Windows automatic checkout conversion enabled.',
    boundary:'Every original source and model byte is retained with independent Git membership. Model inverses change only the declared Node ranges; all immutable schema commands and generated native decisions stay exact. Local checks cover the active Node 24 runtime and version boundaries. Actual Node 22 behavior and native Windows execution remain CI checks.'};
  const output=new URL('parity/self-translation/ci-runtime-source-profiles.lino',root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packet=generateCiRuntimeSourceProfiles({check:process.argv.includes('--check')});
  console.log('Preserved '+packet.sources.length+' complete CI runtime profile predecessors and original model bytes.');
}
