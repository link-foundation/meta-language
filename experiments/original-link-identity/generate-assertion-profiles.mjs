// Assertion metadata is additive; the frozen executable generator is untouched.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,readProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {originalLinkIdentityAssertionProfiles} from './assertion-profiles.mjs';

const sha256=value=>createHash('sha256').update(value).digest('hex');

export function verifyOriginalLinkIdentityAssertionSources(){
 const profiles=originalLinkIdentityAssertionProfiles;
 for(const row of profiles.frozenSourceInputs){
  const source=readFileSync(new URL(row.path,import.meta.url));
  assert.equal(source.length,row.sourceBytes,'the original assertion baseline byte length is frozen: '+row.path);
  assert.equal(sha256(source),row.sha256,'the original assertion baseline source is frozen: '+row.path);
 }
 assert.deepEqual(readProjectData(new URL('./data/policy.lino',import.meta.url)),profiles.originalPolicy);
 const capture=profiles.publishedGrowthMetadata;
 assert.equal(sha256(capture.growthMetadataSource),capture.growthMetadataSha256,'the published growth diagnostics and profile provenance retain their exact source bytes');
 assert.ok(readFileSync(new URL('./assertion-profiles.mjs',import.meta.url),'utf8').includes(capture.growthMetadataSource),'the additive authoring object retains the exact published growth metadata');
 return profiles;
}

export function generateOriginalLinkIdentityAssertionProfiles(check=false){
 const profiles=verifyOriginalLinkIdentityAssertionSources();
 const path=new URL('./data/assertion-profiles.lino',import.meta.url);
 if(check)checkProjectData(path,profiles);
 else writeProjectData(path,profiles);
 return profiles;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
 const profiles=generateOriginalLinkIdentityAssertionProfiles(process.argv.includes('--check'));
 console.log('Original link identity assertion profiles: '+profiles.feasibility.growthProfiles.length+' additive profiles; executable source pins unchanged');
}
