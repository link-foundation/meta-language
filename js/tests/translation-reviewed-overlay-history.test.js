import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readReviewedOverlayProviderSources,restoreReviewedOverlayProviderSources,reviewedOverlayProviderPolicy}
  from '../../experiments/rust-text-certificate-adapter/reviewed-providers.mjs';

test('reviewed current overlays retain all five actual providers and complete historical inverses',()=>{
  const current=reviewedOverlayProviderPolicy.revisions.at(-1);
  assert.equal(current.sourceCommit,'9f867bfac32f59a3341dabf54e8c0c02fbd83314');
  assert.equal(current.inverse.length,6);
  const modules=current.providers.map(row=>row.module);
  const root=new URL('../../',import.meta.url);
  for(const experiment of ['rust-text-certificate-adapter','rust-raw-binding-certificate-adapter']){
    const actual=readReviewedOverlayProviderSources({experiment,modules});assert.equal(actual.revision,current);
    const previous=restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit:'113a7747f799cbce5167167a783eb88427bd2825'});
    assert.equal(previous.sources.size,5);
    for(const row of current.providers){
      const source=actual.sources.get(row.module);
      assert.equal(source,readFileSync(new URL('js/src/'+row.module,root),'utf8'));
      assert.equal(createHash('sha256').update(source).digest('hex'),row.sourceSha256);
      assert.equal(previous.sources.get(row.module),execFileSync('git',['show',previous.revision.sourceCommit+':js/src/'+row.module],
        {cwd:root,encoding:'utf8',maxBuffer:1024*1024}));
    }
    assert.throws(()=>restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit:'0'.repeat(40)}),/one reviewed historical/u);
    for(const changed of modules)assert.throws(()=>restoreReviewedOverlayProviderSources({experiment,modules,
      sourceCommit:previous.revision.sourceCommit,readSource:module=>actual.sources.get(module)+(module===changed?'\n':'')}),/no unique reviewed revision/u);
  }
});
