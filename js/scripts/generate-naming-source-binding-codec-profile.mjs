// Normal author for a finite, complete-source naming codec profile. No bindings are written.
import assert from 'node:assert/strict';
import {readFileSync,existsSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {inspectBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {NAMING_SOURCE_BINDING_STORAGE,verifyNamingSourceBindingData} from './naming-source-bindings.mjs';
import {NAMING_SOURCE_BINDING_CODEC_POLICY as policy,verifyNamingSourceBindingCodecProfile} from './naming-source-binding-codec-profile.mjs';
const root=new URL('../../',import.meta.url);
export function generateNamingSourceBindingCodecProfile({check=false}={}){
  assert.equal(NAMING_SOURCE_BINDING_STORAGE.data,policy.source);
  const source=readFileSync(new URL(policy.source,root),'utf8');
  assert.ok(Buffer.byteLength(source)<=policy.inputByteLimit,'the original physical input byte ceiling remains required');
  // The reviewer-controlled constants are ceilings, never derived from an unbounded input.
  const inspected=inspectBootstrapStructuredDataLino(source,{nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,byteLimit:policy.expandedByteLimit});
  const rows=inspected.value;assert.ok(Array.isArray(rows));
  const migration=readProjectData(new URL(NAMING_SOURCE_BINDING_STORAGE.migration,root));
  verifyNamingSourceBindingData(rows,migration,readFileSync(new URL(NAMING_SOURCE_BINDING_STORAGE.module,root)));
  const epoch={source:{path:policy.source,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')},records:rows.length,cost:inspected.cost};
  const file=new URL(policy.profile,root),previous=existsSync(file)?verifyNamingSourceBindingCodecProfile(readProjectData(file)):null;
  if(check){assert.ok(previous,'the normal profile must already exist');assert.deepEqual(previous.profiles.at(-1),epoch,'the complete current binding source and expansion cost must be fresh');checkProjectData(file,previous);return previous;}
  const profile=previous?structuredClone(previous):{schemaVersion:policy.schemaVersion,model:policy.model,policy:{inputByteLimit:policy.inputByteLimit,nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,expandedByteLimit:policy.expandedByteLimit},profiles:[]};
  const found=profile.profiles.findIndex(row=>row.source.sha256===epoch.source.sha256);
  if(found>=0){assert.equal(found,profile.profiles.length-1,'refuse rollback to a preceding source epoch');assert.deepEqual(profile.profiles[found],epoch);}
  else profile.profiles.push(epoch);
  verifyNamingSourceBindingCodecProfile(profile);
  if(previous)assert.deepEqual(profile.profiles.slice(0,previous.profiles.length),previous.profiles,'every previous whole-source profile remains an unchanged ordered prefix');
  writeProjectData(file,profile);return profile;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(arg=>arg==='--check'));
  const profile=generateNamingSourceBindingCodecProfile({check:process.argv.includes('--check')});
  console.log(JSON.stringify({source:profile.profiles.at(-1).source,records:profile.profiles.at(-1).records,cost:profile.profiles.at(-1).cost,epochs:profile.profiles.length}));
}
