// Normal complete Git membership capture; no compiler or native execution.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {REQUIRED_FIELD_NATIVE_HOST_REVISION,REQUIRED_FIELD_NATIVE_HOST_OUTPUT,REQUIRED_FIELD_NATIVE_HOST_SOURCES,
  pinRequiredFieldNativeHostSource,verifyRequiredFieldNativeHostSuccession,deriveRequiredFieldNativeHostSource} from './required-field-native-host-succession.mjs';
const root=new URL('../../',import.meta.url);
export function generateRequiredFieldNativeHostSuccession({check=false}={}){
  const sources=REQUIRED_FIELD_NATIVE_HOST_SOURCES.map(specification=>{
    const{blob,certificate}=captureGitArtifactCertificate(REQUIRED_FIELD_NATIVE_HOST_REVISION,specification.file);
    const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);
    const current=readFileSync(new URL(specification.file,root),'utf8');
    const replay=deriveRequiredFieldNativeHostSource(source,current,specification);
    assert.equal(current,replay.source,'the finite original-source replay is the actual complete native host');
    return{file:specification.file,previous:{source,...pinRequiredFieldNativeHostSource(blob),certificate},
      current:{source:replay.source,...pinRequiredFieldNativeHostSource(replay.source)},edits:replay.edits};
  });
  const packet={schemaVersion:1,model:'required field native host source succession 1',revision:REQUIRED_FIELD_NATIVE_HOST_REVISION,
    coordinateUnit:'UTF-16 code units',sources,
    boundary:'Two exact registration insertions, one generated-decision builtin-owner guard, and two complete pinned required-field transport host successors with automatically discovered lossless source intervals. Whole independently certified a8 sources equal the separately frozen program-string/ASCII/mutable-array reader inputs. Original policies, packets, assertions and controls remain required. This source view establishes no native execution, source semantic equivalence, grammar completeness or requirement completion.'};
  verifyRequiredFieldNativeHostSuccession(packet);
  const helper=new URL('js/scripts/required-field-native-host-succession.mjs',root),before=readFileSync(helper,'utf8');
  const pattern=/^const packetSha256='(?:PENDING|[a-f0-9]{64})';$/mu;assert.equal(before.split(pattern).length,2);
  const output=new URL(REQUIRED_FIELD_NATIVE_HOST_OUTPUT,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const after=before.replace(pattern,"const packetSha256='"+pinRequiredFieldNativeHostSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(before,after);else writeFileSync(helper,after);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRequiredFieldNativeHostSuccession({check:process.argv.includes('--check')});
  console.log('Authenticated five finite native host successors; every original packet and complete predecessor remains exact.');
}
