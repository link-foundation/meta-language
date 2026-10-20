// Extend the checked overlay chain without rewriting any historical provider.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {writeProjectData,checkProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {DecoratorSet} from '../../js/src/decorators.js';
import {reviewedOverlayProviderPolicy} from '../rust-text-certificate-adapter/reviewed-providers.mjs';
import {restoreReviewedOverlayProviderSources} from '../rust-text-certificate-adapter/reviewed-providers.mjs';
import {rustTextCertificateOverlayRecords} from '../rust-text-certificate-adapter/generate.mjs';
import {rawBindingCertificateOverlayRecords} from '../rust-raw-binding-certificate-adapter/generate-overlay.mjs';

const root=new URL('../../',import.meta.url),revision='478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8';
const predecessor='9f867bfac32f59a3341dabf54e8c0c02fbd83314';
const file='parity/self-translation/program-string-overlay-provider-revision.lino';
const hash=value=>createHash('sha256').update(value).digest('hex');

/** Derive whole-source inverse actions from independently recorded Git objects. */
export function generateProgramStringOverlayProviderRevision({check=false}={}) {
  const previous=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit===predecessor);assert.ok(previous);
  const providers=[],inverse=[],certificates=[];
  for(const provider of previous.providers){
    const path='js/src/'+provider.module;
    const current=captureGitArtifactCertificate(revision,path),old=captureGitArtifactCertificate(predecessor,path);
    verifyGitArtifactCertificate(current.certificate,current.blob,{revision,path});
    verifyGitArtifactCertificate(old.certificate,old.blob,{revision:predecessor,path});
    assert.equal(hash(old.blob),provider.sourceSha256);assert.deepEqual(readFileSync(new URL(path,root)),current.blob);
    const source=current.blob.toString(),previousSource=old.blob.toString();
    assert.deepEqual(Buffer.from(source),current.blob);assert.deepEqual(Buffer.from(previousSource),old.blob);
    providers.push({module:provider.module,sourceSha256:hash(current.blob)});
    certificates.push({module:provider.module,current:{revision,source,certificate:current.certificate},
      previous:{revision:predecessor,source:previousSource,certificate:old.certificate}});
    if(source!==previousSource)inverse.push({module:provider.module,from:source,to:previousSource});
  }
  assert.deepEqual(inverse.map(row=>row.module),['typed-program-links-validation.js','typed-program-checked-projection.js']);
  const value={sourceCommit:revision,previousSourceCommit:predecessor,
    review:'The actual checked-program validator now admits only absent or explicit Boolean utf16Strings metadata. Complete semantic rechecking retains that exact present flag before the unchanged whole-IR comparison. Every other provider byte remains unchanged. Both complete changed modules and all five independently certified current and preceding sources are retained; two exact whole-module inverse actions restore the original source profile. This is source conservation and an admitted-domain extension, not proof that changed operations have identical meaning or that native execution passed.',
    providers,inverse,certificates};
  const output=new URL(file,root);if(check)checkProjectData(output,value);else writeProjectData(output,value);
  const helper=new URL('./overlay-provider-revision.mjs',import.meta.url),source=readFileSync(helper,'utf8');
  const expected=source.replace(/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u,"const packetSha256='"+hash(readFileSync(output))+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helper,expected);
  return value;
}

/** Refresh only current source-derived receipts. Historical overlay documents,
 * every decorator action and all original behavior expectations stay exact. */
export function generateProgramStringOverlayRecords({check=false}={}) {
  const results=[];
  for(const [experiment,author]of [['rust-text-certificate-adapter',rustTextCertificateOverlayRecords],
    ['rust-raw-binding-certificate-adapter',rawBindingCertificateOverlayRecords]]){
    const path='experiments/'+experiment+'/data/overlays-current.lino';
    const original=captureGitArtifactCertificate('abde6ee604b914a895f23525946fab2234109e86',path);
    verifyGitArtifactCertificate(original.certificate,original.blob,{revision:'abde6ee604b914a895f23525946fab2234109e86',path});
    const previous=readStructuredDataLino(original.blob.toString()),current=author(),inverse=structuredClone(current);
    assert.equal(current.length,5);assert.equal(previous.length,5);
    const modules=previous.map(row=>row.module);
    const oldSources=restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit:predecessor}).sources;
    for(const [index,row]of current.entries()){
      const old=previous[index];assert.equal(row.module,old.module);assert.equal(old.providerRevision,predecessor);
      assert.equal(row.providerRevision,revision);assert.equal(row.decorators,old.decorators);assert.equal(row.changes,old.changes);
      const source=readFileSync(new URL('js/src/'+row.module,root),'utf8'),before=oldSources.get(row.module);
      const decorate=text=>DecoratorSet.fromLino(row.decorators).decorate('translation-rule',{module:row.module,source:text}).source;
      assert.equal(hash(source),row.sourceSha256);assert.equal(hash(decorate(source)),row.overlaySha256);
      assert.equal(hash(before),old.sourceSha256);assert.equal(hash(decorate(before)),old.overlaySha256);
      for(const key of ['sourceSha256','overlaySha256','providerRevision'])inverse[index][key]=old[key];
    }
    assert.deepEqual(inverse,previous);assert.equal(serializeProjectData(inverse,path),original.blob.toString());
    const output=new URL(path,root);if(check)checkProjectData(output,current);else writeProjectData(output,current);
    results.push({file:path,providers:current.length,originalDecoratorActionsPreserved:true});
  }
  const original=captureGitArtifactCertificate('abde6ee604b914a895f23525946fab2234109e86','experiments/rust-text-certificate-adapter/data/provider-revisions.lino');
  const policy=structuredClone(reviewedOverlayProviderPolicy);assert.equal(policy.revisions.pop().sourceCommit,revision);
  assert.deepEqual(policy,readStructuredDataLino(original.blob.toString()),'all preceding provider policy fields and revisions remain exact');
  return results;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  console.log('Registered '+generateProgramStringOverlayProviderRevision({check:process.argv.includes('--check')}).providers.length+' complete current overlay providers.');
  console.log(generateProgramStringOverlayRecords({check:process.argv.includes('--check')}));
}
