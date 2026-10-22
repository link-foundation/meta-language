// Review metadata only; complete source membership is captured at append time.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
const root=new URL('../../',import.meta.url),previous='f73a10b22f8d0bcfbf4707621bcc9edd6989533d';
const reviewed=[
  {
    "module": "typed-program-links.js",
    "currentBytes": 24397,
    "currentSha256": "91ca764e0cdb26b9c89ab38f13392071e2b74b0d066c30dea5db05b2604d46f5"
  },
  {
    "module": "typed-program-checked-projection.js",
    "currentBytes": 11251,
    "currentSha256": "54acda74bd29dbf1134c7f36c5f2f8434cb9790442a390c287c8aef9a19f2341"
  }
];
export function generateSharedArrayProviderReview({check=false}={}){
 const modules=reviewed.map(specification=>{
  const file='js/src/'+specification.module,old=captureGitArtifactCertificate(previous,file).blob,current=readFileSync(new URL(file,root));
  assert.equal(current.length,specification.currentBytes);assert.equal(createHash('sha256').update(current).digest('hex'),specification.currentSha256,'only the whole reviewed shared-array provider is admitted');
  return{module:specification.module,previousSha256:createHash('sha256').update(old).digest('hex'),currentBytes:current.length,currentSha256:specification.currentSha256};
 });
 const review={kind:'actual standard Rust shared array handle source projection 1',previous,modules,
  review:'Reviewed both complete providers against f73a10b2. The typed source factory transports the independently selected Rust shared-array source model and provenance, retains every other option and rejects non-Rust selection. Whole-program rechecking transports the actual source language into expression projection; explicit Rust array reference identity reconstructs its owned source operation while ordinary Rust binary equality retains its old refusal. Validator and both emitters remain byte exact. The whole-source pins and append-time Git certificates bind the reviewed modules; complete inverses retain every original field, source, decorator action, refusal and historical overlay. Actual current parsing, source-free codecs, publication and native runtime parity remain independently gated. This is source review, not execution evidence or a meaning-equivalence proof.'};
 const target=new URL('../../parity/self-translation/reviewed-overlay-shared-array-provider-review.lino',import.meta.url);
 if(check)checkProjectData(target,review);else writeProjectData(target,review);return review;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 assert.ok(process.argv.slice(2).every(value=>value==='--check'));
 generateSharedArrayProviderReview({check:process.argv.includes('--check')});console.log('Reviewed complete shared-array source factory and rechecking providers.');
}
