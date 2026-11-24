import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,mkdtempSync,mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {tmpdir} from 'node:os';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE,readGlobalGrammarContextSourcePair,restoreGlobalGrammarContextSource,checkGlobalGrammarContextSourceReceipt} from '../../experiments/global-grammar-context-reuse/current-source.mjs';
import {captureOutcomeSourceViews,finalizeOutcomeSourceViews,checkOutcomeSourceViews,applyFrozenOutcomeProducerCorrection} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/current-source-view.mjs';
import {OUTCOME_CRATE_ROOT_SOURCE_PROFILE,verifyOutcomeCrateRootSourcePacket,finalizeOutcomeCrateRootSourceView,outcomeCrateRootPath,readOutcomeCrateRootSourcePair,restoreOutcomeCrateRootSource,checkOutcomeCrateRootSourceReceipt} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/crate-root-source-view.mjs';
import {diagnosticOrderDecorator} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/decorator.mjs';
import {javascriptPath,nativePath} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs';
import {readOutcomeInventory} from '../../experiments/grammar-outcome-network/native-transport/wire/generate.mjs';
import {structuredDataToLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {outcomeWirePolicy} from '../../experiments/grammar-outcome-network/native-transport/wire/wire-policy.mjs';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {javascriptRuntimeSource,runtimeSourceFiles,readIssue245SourcePair} from '../../experiments/issue-245/source-history.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
let candidate;

function currentContextInventory() {
  if(candidate)return structuredClone(candidate);
  const inventory=readOutcomeInventory(root),scratch=mkdtempSync(join(tmpdir(),'meta-language-current-context-'));
  try{
    let size=0;
    for(const file of Object.keys(diagnosticOrderDecorator.baselinePins)){
      // This fixture retains the complete original two-successor source profile.
      const bytes=file===outcomeCrateRootPath?Buffer.from(readOutcomeCrateRootSourcePair().predecessor.source):
        [javascriptPath,nativePath].includes(file)?Buffer.from(readGlobalGrammarContextSourcePair(file).successor.source):
        runtimeSourceFiles.includes(file)?Buffer.from(readIssue245SourcePair(file).predecessor):readFileSync(join(root,file));
      size+=bytes.length;assert.ok(size<=2*1024*1024);
      mkdirSync(dirname(join(scratch,file)),{recursive:true});writeFileSync(join(scratch,file),bytes);
    }
    const capture=captureOutcomeSourceViews(scratch,diagnosticOrderDecorator);
    candidate={...inventory,sourceViews:finalizeOutcomeSourceViews(capture,inventory.inputs,inventory.declarations)};
    return structuredClone(candidate);
  }finally{rmSync(scratch,{recursive:true,force:true});}
}

function currentCrateRootInventory(){
  const inventory=readOutcomeInventory(root);
  assert.equal(inventory.sourceViews.schemaVersion,4);
  return {...inventory,sourceViews:inventory.sourceViews.predecessorView};
}

test('all exact context successor sources retain independent original membership and unchanged complete predecessors',()=>{
  assert.equal(GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE.files.length,4);
  for(const file of GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE.files){
    const pair=readGlobalGrammarContextSourcePair(file),old=restoreGlobalGrammarContextSource(file,pair.predecessor.source),current=restoreGlobalGrammarContextSource(file,pair.successor.source);
    assert.equal(old.changed,false);assert.equal(old.receipt,null);
    assert.equal(current.changed,true);assert.equal(current.predecessor,pair.predecessor.source);
    assert.deepEqual(checkGlobalGrammarContextSourceReceipt(file,pair.successor.source,current.receipt),current.receipt);
    assert.notEqual(pair.predecessor.sha256,pair.successor.sha256);
    pair.actions[0].after+=' changed';assert.notDeepEqual(readGlobalGrammarContextSourcePair(file),pair);
    assert.throws(()=>restoreGlobalGrammarContextSource(file,current.current+'\n'));
    const changed=structuredClone(current.receipt);changed.predecessor.sha256=changed.current.sha256;
    assert.throws(()=>checkGlobalGrammarContextSourceReceipt(file,current.current,changed));
    assert.throws(()=>checkGlobalGrammarContextSourceReceipt(file,old.current,current.receipt));
  }
  assert.throws(()=>readGlobalGrammarContextSourcePair('unknown/source.js'));
  assert.throws(()=>restoreGlobalGrammarContextSource(javascriptPath,'x'.repeat(131073)));
});

test('actual current source views preserve all frozen pins legacy validation complete native owners and diagnostic calls through both archives',()=>{
  const inventory=currentContextInventory(),view=inventory.sourceViews;
  assert.equal(view.schemaVersion,2);assert.equal(view.historicalPins.length,13);assert.equal(view.currentPins.length,13);
  assert.deepEqual(view.historicalPins,inventory.pins);assert.equal(view.successors.length,2);
  assert.equal(view.predecessorView.schemaVersion,1);assert.equal(view.predecessorView.adaptations.length,1);
  const javascript=view.successors.find(row=>row.file===javascriptPath),native=view.successors.find(row=>row.file===nativePath);
  assert.equal(javascript.currentSourceProof.diagnosticCalls.length,9);
  assert.ok(native.currentSourceProof.imports.length>0);assert.ok(native.currentSourceProof.selectedDeclarations.length>0);
  const frozen=applyFrozenOutcomeProducerCorrection(diagnosticOrderDecorator.inputs.find(row=>row.path===nativePath).source,diagnosticOrderDecorator);
  assert.equal(frozen.source,native.predecessorSource);assert.notEqual(frozen.source,native.currentSource);
  assert.equal(frozen.receipt.role,'frozen CI overlay');
  assert.deepEqual(checkOutcomeSourceViews(inventory,diagnosticOrderDecorator),view);
  const encoded=encodeStructuredDataLinks(view),network=LinkNetwork.fromLino(structuredDataToLino(view));
  for(const graph of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
    const restored=decodeStructuredDataLinks(graph,encoded.root);assert.deepEqual(restored,view);
    assert.deepEqual(checkOutcomeSourceViews({...inventory,sourceViews:restored},diagnosticOrderDecorator),view);
  }
  const legacy={...inventory,sourceViews:view.predecessorView};
  assert.deepEqual(checkOutcomeSourceViews(legacy,diagnosticOrderDecorator),legacy.sourceViews);
});

test('current source views refuse changed frozen labels missing owners forged receipts duplicate successors and caller mutations',()=>{
  const inventory=currentContextInventory();
  for(const mutate of [
    value=>{value.sourceViews.currentPins.find(row=>row.file===nativePath).sha256=value.pins.find(row=>row.file===nativePath).sha256;},
    value=>{value.sourceViews.historicalPins[0].sha256='0'.repeat(64);},
    value=>{value.sourceViews.predecessorView.adaptations[0].inverseReceipt.role='frozen CI overlay';},
    value=>{value.sourceViews.successors.pop();},
    value=>{value.sourceViews.successors[1]=structuredClone(value.sourceViews.successors[0]);},
    value=>{value.sourceViews.successors[0].currentSource+='\n';},
    value=>{value.sourceViews.successors[0].predecessorSource+='\n';},
    value=>{value.sourceViews.successors[0].receipt.recipe.sha256='0'.repeat(64);},
    value=>{value.sourceViews.successors.find(row=>row.file===javascriptPath).currentSourceProof.diagnosticCalls[0].source+=' changed';},
    value=>{value.sourceViews.successors.find(row=>row.file===nativePath).currentSourceProof.selectedDeclarations=[];},
    value=>{value.sourceViews.successors.find(row=>row.file===nativePath).currentSourceProof.imports[0].source+=' changed';},
    value=>{value.declarations.find(row=>row.file===nativePath).span.start++;},
    value=>{value.inputs.find(row=>row.file===nativePath).imports[0].source+=' changed';},
  ]){const value=structuredClone(inventory);mutate(value);assert.throws(()=>checkOutcomeSourceViews(value,diagnosticOrderDecorator));}
  const returned=checkOutcomeSourceViews(inventory,diagnosticOrderDecorator);returned.successors[0].receipt.current.sha256='0'.repeat(64);
  assert.deepEqual(checkOutcomeSourceViews(inventory,diagnosticOrderDecorator),inventory.sourceViews);
  assert.throws(()=>checkOutcomeSourceViews({...inventory,sourceViews:returned},diagnosticOrderDecorator));
});


test('current crate root retains complete certified predecessors actual export owners and original context profiles',()=>{
  const pair=readOutcomeCrateRootSourcePair(),original=restoreOutcomeCrateRootSource(outcomeCrateRootPath,pair.predecessor.source);
  assert.equal(original.changed,false);assert.equal(original.receipt,null);
  const current=restoreOutcomeCrateRootSource(outcomeCrateRootPath,readFileSync(join(root,outcomeCrateRootPath),'utf8'));
  assert.equal(current.changed,true);assert.equal(current.predecessor,pair.predecessor.source);assert.equal(current.current,pair.successor.source);
  assert.deepEqual(checkOutcomeCrateRootSourceReceipt(outcomeCrateRootPath,current.current,current.receipt),current.receipt);
  const inventory=currentCrateRootInventory(),view=inventory.sourceViews;
  assert.equal(view.schemaVersion,3);assert.equal(view.predecessorView.schemaVersion,2);
  assert.equal(view.predecessorView.successors.length,2);assert.equal(view.predecessorView.predecessorView.schemaVersion,1);
  assert.equal(view.historicalPins.length,13);assert.equal(view.currentPins.length,13);assert.deepEqual(view.historicalPins,inventory.pins);
  const before=view.predecessorView.currentPins.find(row=>row.file===outcomeCrateRootPath),after=view.currentPins.find(row=>row.file===outcomeCrateRootPath);
  assert.deepEqual({bytes:before.bytes,sha256:before.sha256},{bytes:pair.predecessor.bytes,sha256:pair.predecessor.sha256});
  assert.deepEqual({bytes:after.bytes,sha256:after.sha256},{bytes:pair.successor.bytes,sha256:pair.successor.sha256});
  const proof=view.crateRootSuccessor.currentSourceProof;
  assert.equal(proof.predecessor.rootReexport.source,proof.current.rootReexport.source);
  assert.deepEqual(proof.current.selectedOwners.map(({name,source})=>({name,source})),proof.predecessor.selectedOwners.map(({name,source})=>({name,source})));
  assert.ok(proof.current.reexports.some(row=>row.source.includes('encode_formal_function_declaration_value')));
  assert.equal(proof.predecessor.reexports.some(row=>row.source.includes('encode_formal_function_declaration_value')),false);
  assert.deepEqual(checkOutcomeSourceViews(inventory,diagnosticOrderDecorator),view);
  assert.deepEqual(checkOutcomeSourceViews({...inventory,sourceViews:view.predecessorView},diagnosticOrderDecorator),view.predecessorView);
  const encoded=encodeStructuredDataLinks(view),network=LinkNetwork.fromLino(structuredDataToLino(view));
  for(const graph of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
    const restored=decodeStructuredDataLinks(graph,encoded.root);assert.deepEqual(restored,view);
    assert.deepEqual(checkOutcomeSourceViews({...inventory,sourceViews:restored},diagnosticOrderDecorator),view);
  }
});

test('crate-root source succession refuses arbitrary files edits forged whole pins lost owners and caller mutations',()=>{
  const packet=readProjectData(join(root,OUTCOME_CRATE_ROOT_SOURCE_PROFILE.recipe));
  assert.equal(verifyOutcomeCrateRootSourcePacket(packet).successor.source,readOutcomeCrateRootSourcePair().successor.source);
  for(const mutate of [
    value=>{value.source.current.sha256='0'.repeat(64);},
    value=>{value.source.certificate.revision='0'.repeat(40);},
    value=>{value.source.bytes++;},
    value=>{value.source.edit.insertText+=' changed';},
    value=>{value.unregistered=true;},
  ]){const value=structuredClone(packet);mutate(value);assert.throws(()=>verifyOutcomeCrateRootSourcePacket(value));}
  const pair=readOutcomeCrateRootSourcePair();
  assert.throws(()=>restoreOutcomeCrateRootSource('unknown/source.rs',pair.successor.source));
  assert.throws(()=>restoreOutcomeCrateRootSource(outcomeCrateRootPath,pair.successor.source+'\n'));
  assert.throws(()=>restoreOutcomeCrateRootSource(outcomeCrateRootPath,'x'.repeat(131073)));
  pair.predecessor.source+=' changed';assert.notDeepEqual(readOutcomeCrateRootSourcePair(),pair);
  const inventory=currentCrateRootInventory();
  for(const mutate of [
    value=>{value.sourceViews.currentPins.find(row=>row.file===outcomeCrateRootPath).sha256='0'.repeat(64);},
    value=>{value.sourceViews.historicalPins[0].sha256='0'.repeat(64);},
    value=>{value.sourceViews.predecessorView.successors.pop();},
    value=>{value.sourceViews.crateRootSuccessor.file='unregistered/source.rs';},
    value=>{value.sourceViews.crateRootSuccessor.currentSource+='\n';},
    value=>{value.sourceViews.crateRootSuccessor.predecessorSource+='\n';},
    value=>{value.sourceViews.crateRootSuccessor.receipt.recipe.sha256='0'.repeat(64);},
    value=>{value.sourceViews.crateRootSuccessor.receipt.current.bytes++;},
    value=>{value.sourceViews.crateRootSuccessor.currentSourceProof.current.rootReexport.span.start++;},
    value=>{value.sourceViews.crateRootSuccessor.currentSourceProof.current.selectedOwners.pop();},
    value=>{value.sourceViews.crateRootSuccessor.currentSourceProof.current.reexports.pop();},
    value=>{value.sourceViews.crateRootSuccessor.unregistered=true;},
  ]){const value=structuredClone(inventory);mutate(value);assert.throws(()=>checkOutcomeSourceViews(value,diagnosticOrderDecorator));}
  const returned=checkOutcomeSourceViews(inventory,diagnosticOrderDecorator);returned.crateRootSuccessor.currentSourceProof.current.reexports=[];
  assert.deepEqual(checkOutcomeSourceViews(inventory,diagnosticOrderDecorator),inventory.sourceViews);
  assert.throws(()=>checkOutcomeSourceViews({...inventory,sourceViews:returned},diagnosticOrderDecorator));
});


test('complete crate-root proof reuse remains detached and authenticates every consumed policy owner',()=>{
  const inventory=currentCrateRootInventory(),view=inventory.sourceViews;
  const derive=()=>finalizeOutcomeCrateRootSourceView(view.predecessorView,view.crateRootSuccessor);
  assert.deepEqual(derive(),view);
  const changed=derive();changed.crateRootSuccessor.currentSourceProof.current.reexports=[];
  changed.crateRootSuccessor.currentSourceProof.predecessor.selectedOwners[0].source+=' changed';
  assert.deepEqual(derive(),view,'caller-owned current and predecessor proof mutations never change a reused proof');
  const originalRoot=outcomeWirePolicy.rootReexport;
  try{
    outcomeWirePolicy.rootReexport='pub use unregistered_grammar_transport_owner::UnknownOwner;';
    assert.throws(derive,'complete unchanged source cannot reuse a proof under a different selected root owner');
  }finally{outcomeWirePolicy.rootReexport=originalRoot;}
  const originalSources=outcomeWirePolicy.sources;
  try{
    outcomeWirePolicy.sources=structuredClone(originalSources);
    outcomeWirePolicy.sources[0].names.push('UnregisteredGrammarTransportOwner');
    assert.throws(derive,'complete unchanged source cannot reuse a proof with an additional consumed policy owner');
  }finally{outcomeWirePolicy.sources=originalSources;}
  assert.deepEqual(derive(),view);assert.deepEqual(checkOutcomeSourceViews(inventory,diagnosticOrderDecorator),view);
});

test('grammar search retains the original transport source and authenticates the current runtime sources',()=>{
  const inventory=readOutcomeInventory(root),view=inventory.sourceViews;
  assert.equal(view.schemaVersion,4);
  assert.equal(view.predecessorView.schemaVersion,3);
  assert.deepEqual(view.historicalPins,inventory.pins);
  assert.deepEqual(view.searchSuccessors.map(row=>row.file),runtimeSourceFiles);
  for(const proof of view.searchSuccessors){
    const current=readFileSync(join(root,proof.file),'utf8'),pair=readIssue245SourcePair(proof.file);
    assert.equal(current,proof.currentSource);
    assert.equal(pair.current,current);
    assert.equal(pair.predecessor,proof.predecessorSource);
    const [predecessor,...successors]=pair.versions;
    assert.equal(predecessor,proof.predecessorSource);
    assert.equal(successors.pop(),current);
    assert.deepEqual(successors.map(source=>createHash('sha256').update(source).digest('hex')),
      proof.file===javascriptRuntimeSource?['adbd70d41959110356341de44370a94a9b866ab5a0de906ea41fac1b13c5c2c5']:[]);
    const declarations=inventory.declarations.filter(row=>row.file===proof.file);
    assert.deepEqual(proof.selectedDeclarations.map(row=>row.name),declarations.map(row=>row.name));
    for(const [index,row]of proof.selectedDeclarations.entries()){
      assert.equal(Buffer.from(current).subarray(row.span.start,row.span.end).toString('utf8'),declarations[index].source);
    }
  }
  assert.deepEqual(checkOutcomeSourceViews(inventory,diagnosticOrderDecorator),view);
  for(const mutate of [
    value=>{value.searchSuccessors[0].currentSource+='\n';},
    value=>{value.searchSuccessors[0].selectedDeclarations.pop();},
    value=>{value.searchSuccessors[1].predecessorSource+='\n';},
    value=>{value.searchSuccessors.push(value.searchSuccessors[0]);},
    value=>{value.currentPins[0].bytes++;},
  ]){
    const changed=structuredClone(view);mutate(changed);
    assert.throws(()=>checkOutcomeSourceViews({...inventory,sourceViews:changed},diagnosticOrderDecorator));
  }
});
