// Review metadata only: exact source changes, never a runtime observation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
const previous='8bc0ef9725388c7927acf1854870bf46f5e365be';
const revision='f73a10b22f8d0bcfbf4707621bcc9edd6989533d';
const output=new URL('../../parity/self-translation/reviewed-overlay-optional-domain-provider-review.lino',import.meta.url);
export function generateOptionalDomainProviderReview({check=false}={}){
 const specifications=[
  {module:'typed-program-links-validation.js',variable:'node',indent:'      '},
  {module:'translation/emit-rust.js',variable:'e',indent:'    '},
 ];
 const modules=specifications.map(({module,variable,indent})=>{
  const before=captureGitArtifactCertificate(previous,'js/src/'+module).blob;
  const after=captureGitArtifactCertificate(revision,'js/src/'+module).blob;
  const source=before.toString('utf8');assert.deepEqual(Buffer.from(source),before);
  const lines=source.split('\n'),matches=lines.filter(line=>line.includes('readReferenceIdentityComparisonError('));
  assert.equal(matches.length,1);const old=matches[0];
  const current=indent+'const domainPresent = '+variable+'.domain !== undefined && '+variable+'.domain !== null;\n'
    +old.replaceAll(variable+'.domain !== undefined','domainPresent');
  assert.equal(source.replace(old,current),after.toString('utf8'),'only the exact explicit-absence correction is reviewed: '+module);
  return{module,previousSha256:createHash('sha256').update(before).digest('hex'),currentBytes:after.length,currentSha256:createHash('sha256').update(after).digest('hex')};
 });
 const review={kind:'explicit optional binary domain absence 1',previous,modules,
  review:'Reviewed both complete providers against the certified8bc0ef97 predecessor. Each changes only its reference-comparison domain-presence projection: null and undefined are absent; every other value remains present and is validated. No comparison policy, operand, ownership rule or historical assertion is removed. The retained optional binary domain controls exercise valid Boolean domains, null, absence and malformed domains. Complete Git certificates and source inverses preserve all prior bytes. This records source succession, not semantic equivalence or native execution.'};
 if(check)checkProjectData(output,review);else writeProjectData(output,review);
 return review;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 assert.ok(process.argv.slice(2).every(value=>value==='--check'));
 generateOptionalDomainProviderReview({check:process.argv.includes('--check')});
 console.log('Reviewed both complete optional-domain provider changes.');
}
