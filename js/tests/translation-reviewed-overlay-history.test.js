import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {restoreReviewedOverlayProviderModuleSource} from '../../experiments/rust-text-certificate-adapter/provider-succession.mjs';
import {readReviewedOverlayProviderSources,restoreReviewedOverlayProviderSources,reviewedOverlayProviderPolicy}
  from '../../experiments/rust-text-certificate-adapter/reviewed-providers.mjs';

test('reviewed current overlays retain all five actual providers and complete historical inverses',()=>{
  const current=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='9f867bfac32f59a3341dabf54e8c0c02fbd83314');
  assert.equal(current.sourceCommit,'9f867bfac32f59a3341dabf54e8c0c02fbd83314');
  assert.equal(current.inverse.length,6);
  const modules=current.providers.map(row=>row.module);
  const root=new URL('../../',import.meta.url);
  for(const experiment of ['rust-text-certificate-adapter','rust-raw-binding-certificate-adapter']){
    const readSource=module=>restoreReviewedOverlayProviderModuleSource(restoreCurrentCiRepairSource(readFileSync(new URL('js/src/'+module,root),'utf8'),'js/src/'+module),'js/src/'+module);
    const actual=readReviewedOverlayProviderSources({experiment,modules,readSource});assert.equal(actual.revision,current);
    const previous=restoreReviewedOverlayProviderSources({experiment,modules,readSource,sourceCommit:'113a7747f799cbce5167167a783eb88427bd2825'});
    assert.equal(previous.sources.size,5);
    for(const row of current.providers){
      const source=actual.sources.get(row.module);
      assert.equal(source,readSource(row.module));
      assert.equal(createHash('sha256').update(source).digest('hex'),row.sourceSha256);
      assert.equal(previous.sources.get(row.module),execFileSync('git',['show',previous.revision.sourceCommit+':js/src/'+row.module],
        {cwd:root,encoding:'utf8',maxBuffer:1024*1024}));
    }
    assert.throws(()=>restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit:'0'.repeat(40)}),/one reviewed historical/u);
    for(const changed of modules)assert.throws(()=>restoreReviewedOverlayProviderSources({experiment,modules,
      sourceCommit:previous.revision.sourceCommit,readSource:module=>actual.sources.get(module)+(module===changed?'\n':'')}),/no unique reviewed revision/u);
  }
});

import {restoreCurrentCiRepairSource} from '../scripts/current-ci-repair-source-history.mjs';

test('actual current program-string overlay providers retain both complete earlier source profiles',()=>{
  generateProgramStringOverlayRecords({check:true});
  const current=reviewedOverlayProviderPolicy.revisions.find(row=>row.sourceCommit==='478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8');
  assert.equal(current.sourceCommit,'478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8');
  assert.equal(current.inverse.length,2);assert.equal(current.certificates.length,5);
  const modules=current.providers.map(row=>row.module);
  for(const experiment of ['rust-text-certificate-adapter','rust-raw-binding-certificate-adapter']){
    const actual=restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit:current.sourceCommit});assert.equal(actual.revision,current);
    for(const row of current.providers)assert.equal(createHash('sha256').update(actual.sources.get(row.module)).digest('hex'),row.sourceSha256);
    for(const sourceCommit of [current.previousSourceCommit,'113a7747f799cbce5167167a783eb88427bd2825']){
      const previous=restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit});
      for(const row of previous.revision.providers)assert.equal(previous.sources.get(row.module),execFileSync('git',['show',sourceCommit+':js/src/'+row.module],{encoding:'utf8',maxBuffer:1024*1024}));
    }
    for(const changed of modules)assert.throws(()=>readReviewedOverlayProviderSources({experiment,modules,
      readSource:module=>actual.sources.get(module)+(module===changed?'\n':'')}),/no unique reviewed revision/u);
  }
});

import {generateProgramStringOverlayRecords} from '../../experiments/grammar-context-source-metadata/generate-overlay-provider-revision.mjs';
