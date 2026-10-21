import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import test from 'node:test';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitJavaScript} from '../../../../js/src/translation/emit-javascript.js';
import {selfTranslate} from '../../../../js/src/self-translation.js';
import {encodeStructuredDataLinks,structuredDataToLino,readStructuredDataLino} from '../../../../js/src/structured-data-links.js';
import {captureOrdinaryInversePlan} from './plan.mjs';
import {reconstructOrdinaryOutcome} from './owned.mjs';
import {acceptOwnedOutcomeStaticBorrowResources,findOwnedOutcomeStaticBorrowIndex,readOwnedOutcomeStaticBorrowFailure} from './static-borrow-rules.mjs';
import {generateOwnedOutcomeStaticBorrowRules} from './generate-static-borrows.mjs';
import {planOwnedOutcomeStaticBorrows,checkOwnedOutcomeStaticBorrowPlan} from './static-borrows.mjs';

const source=readFileSync(new URL('./static-borrow-rules.mjs',import.meta.url),'utf8');

test('complete shared static reference selection agrees with all independent registry cases and exact resource boundaries',()=>{
  const {fixture}=generateOwnedOutcomeStaticBorrowRules({check:true});assert.equal(fixture.cases.length,256);
  const operations=runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source,{referenceSemantics:false}))).text+';({findOwnedOutcomeStaticBorrowIndex});');
  for(const row of fixture.cases){assert.equal(findOwnedOutcomeStaticBorrowIndex(row.text,row.registry,row.bytes),row.expected);assert.equal(operations.findOwnedOutcomeStaticBorrowIndex(row.text,row.registry,row.bytes),row.expected);}
  for(const [entries,bytes,expected]of [[0,0,true],[128,1048576,true],[129,0,false],[0,1048577,false],[-1,0,false],[0,-1,false],[0.5,0,false],[0,0.5,false],[NaN,0,false],[0,Infinity,false]])assert.equal(acceptOwnedOutcomeStaticBorrowResources(entries,bytes),expected);
  assert.match(readOwnedOutcomeStaticBorrowFailure(-1),/no existing/u);assert.match(readOwnedOutcomeStaticBorrowFailure(-2),/ambiguous/u);assert.match(readOwnedOutcomeStaticBorrowFailure(-3),/resource/u);
  const translated=selfTranslate(source,'JavaScript','Rust');assert.equal(translated.items.filter(row=>row.status==='carried').length,0);assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});

test('source-owned static reference plans retain actual type owners complete model validation and source-free selection data',()=>{
  const authority=captureOrdinaryInversePlan(),plan=authority.plan();
  const reference=plan.graph.nodes.find(node=>node.kind==='reference'),rootReference=plan.graph.roots.find(id=>id.endsWith('#ParseRejection'));
  assert.ok(reference&&rootReference);const options={usizeBits:64,rootReference};
  const encoded=encodeStructuredDataLinks({reason:'syntax'}),model=reconstructOrdinaryOutcome(authority,encoded.network,encoded.root,options);
  const registry={typeReference:reference.identity,values:['ambiguity','syntax','😀']};
  const selected=planOwnedOutcomeStaticBorrows(authority,model,registry,options);assert.equal(selected.selections.length,1);assert.equal(selected.selections[0].registryIndex,1);assert.equal(selected.selections[0].typeReference,reference.identity);
  assert.deepEqual(checkOwnedOutcomeStaticBorrowPlan(authority,model,registry,readStructuredDataLino(structuredDataToLino(selected)),options),selected);
  const changed=structuredClone(selected);changed.selections[0].registryIndex=0;assert.throws(()=>checkOwnedOutcomeStaticBorrowPlan(authority,model,registry,changed,options));
  for(const values of [[],['syntax','syntax'],['missing','missing','syntax'],['x'.repeat(1048577)],['\ud800']])assert.throws(()=>planOwnedOutcomeStaticBorrows(authority,model,{typeReference:reference.identity,values},options));
  assert.throws(()=>planOwnedOutcomeStaticBorrows(authority,model,{typeReference:'forged owner',values:['syntax']},options));
  const altered=structuredClone(model);altered.borrowRequirements=[];assert.throws(()=>planOwnedOutcomeStaticBorrows(authority,altered,registry,options),/manifest/u);
  let invoked=false;const active={typeReference:reference.identity,get values(){invoked=true;return['syntax'];}};
  assert.throws(()=>planOwnedOutcomeStaticBorrows(authority,model,active,options));assert.equal(invoked,false);
  const activeSelection={get model(){invoked=true;return selected.model;}};
  assert.throws(()=>checkOwnedOutcomeStaticBorrowPlan(authority,model,registry,activeSelection,options));assert.equal(invoked,false);
});
