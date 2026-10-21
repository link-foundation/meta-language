// Explicit current-source admission; the frozen diagnostic producer stays exact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {restoreCiLintSource,readCiLintSourceReceipt} from '../../js/scripts/ci-39496927203-lint-source-history.mjs';

export const GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE=Object.freeze({
  revision:'bd939f8df9dd6156b7208a4d22843ba7e59a3c46',
  path:'parity/self-translation/global-grammar-context-reuse.lino',
  sha256:'3ab63b29e01be1e18fe2b08f86e3e73cbda28b34728a19ee69fc6f280f79e63c',
  files:Object.freeze(['js/src/grammar-runtime/grammar-selection.js',
    'rust/src/grammar/feature_runtime/grammar_selection.rs',
    'js/tests/global-concept-archive-parser.test.js',
    'rust/tests/unit/global_concept_archive_parser.rs']),
});
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let sourcePairs;

/** Derive exact successor sources from one independently pinned Links recipe.
 * Whole original Git membership and the inverse of every action are checked.
 * No current file, caller proof or mutable returned object is an authority. */
export function readGlobalGrammarContextSourcePair(file) {
  const profile=GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE;
  assert.ok(profile.files.includes(file),'an exact registered context source owner is required');
  if(!sourcePairs){
    const bytes=readFileSync(new URL(profile.path,root));assert.equal(pin(bytes).sha256,profile.sha256,'complete independently pinned context source recipe');
    const packet=readProjectData(new URL(profile.path,root));
    assert.equal(packet.schemaVersion,1);assert.equal(packet.revision,profile.revision);
    assert.deepEqual(packet.sources.map(row=>row.file),profile.files);
    const pairs=new Map();
    for(const row of packet.sources){
      const original=Buffer.from(row.sourceText);assert.ok(original.length<=131072,'bounded complete context source');
      assert.equal(row.commit,profile.revision);
      verifyGitArtifactCertificate(row.certificate,original,{revision:profile.revision,path:row.file});
      assert.deepEqual(pin(original),{bytes:row.bytes,sha256:row.sha256});
      let source=row.sourceText;
      for(const action of row.actions){assert.equal(source.split(action.before).length,2);source=source.replace(action.before,action.after);}
      assert.deepEqual(pin(source),row.proposed);assert.ok(Buffer.byteLength(source)<=131072);
      let inverse=source;
      for(const action of row.actions.toReversed()){assert.equal(inverse.split(action.after).length,2);inverse=inverse.replace(action.after,action.before);}
      assert.equal(inverse,row.sourceText,'the entire original context source stays exact');
      pairs.set(row.file,{file:row.file,revision:profile.revision,
        predecessor:{source:row.sourceText,...pin(original)},successor:{source,...pin(source)},actions:row.actions});
    }
    sourcePairs=pairs;
  }
  return structuredClone(sourcePairs.get(file));
}

/** Admit only a complete original or its exact context-reuse successor.
 * Historical wire pins and frozen-overlay roles are deliberately not inputs. */
export function restoreGlobalGrammarContextSource(file,source) {
  assert.equal(typeof source,'string');assert.ok(Buffer.byteLength(source)<=131072);
  const pair=readGlobalGrammarContextSourcePair(file),current=pin(source);
  const prior=restoreCiLintSource(source,file);
  if(source===pair.predecessor.source)return{file,predecessor:source,current:source,changed:false,receipt:null};
  assert.equal(prior,pair.successor.source,'unknown current context source edits refuse');
  let inverse=prior;
  for(const action of pair.actions.toReversed()){assert.equal(inverse.split(action.after).length,2);inverse=inverse.replace(action.after,action.before);}
  assert.equal(inverse,pair.predecessor.source);
  return{file,predecessor:inverse,current:source,changed:true,receipt:{
    schemaVersion:1,profile:'complete global grammar context source successor 1',file,revision:pair.revision,
    recipe:{path:GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE.path,sha256:GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE.sha256},
    predecessor:pin(inverse),current,inverseVerified:true,
    ...(prior!==source?{sourceSuccessor:readCiLintSourceReceipt(source,file)}:{}),
    boundary:'Exact current-to-predecessor source conservation only; no frozen producer repin, predicate equivalence or native execution observation.'}};
}

/** Serialized current-source receipts are checked against the complete recipe. */
export function checkGlobalGrammarContextSourceReceipt(file,source,receipt) {
  const actual=restoreGlobalGrammarContextSource(file,source);assert.equal(actual.changed,true,'a successor receipt needs actual successor bytes');
  assert.equal(serializeProjectData(receipt,'receipt.lino'),serializeProjectData(actual.receipt,'receipt.lino'),'complete current-source receipt conservation');
  return structuredClone(actual.receipt);
}
