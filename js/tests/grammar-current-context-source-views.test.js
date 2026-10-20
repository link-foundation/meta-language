import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {tmpdir} from 'node:os';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {GLOBAL_GRAMMAR_CONTEXT_SOURCE_PROFILE,readGlobalGrammarContextSourcePair,restoreGlobalGrammarContextSource,checkGlobalGrammarContextSourceReceipt} from '../../experiments/global-grammar-context-reuse/current-source.mjs';
import {captureOutcomeSourceViews,finalizeOutcomeSourceViews,checkOutcomeSourceViews,applyFrozenOutcomeProducerCorrection} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/current-source-view.mjs';
import {diagnosticOrderDecorator} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/decorator.mjs';
import {javascriptPath,nativePath} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs';
import {readOutcomeInventory} from '../../experiments/grammar-outcome-network/native-transport/wire/generate.mjs';
import {structuredDataToLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const root=fileURLToPath(new URL('../../',import.meta.url));
let candidate;

function currentContextInventory() {
  if(candidate)return structuredClone(candidate);
  const inventory=readOutcomeInventory(root),scratch=mkdtempSync(join(tmpdir(),'meta-language-current-context-'));
  try{
    let size=0;
    for(const file of Object.keys(diagnosticOrderDecorator.baselinePins)){
      const bytes=[javascriptPath,nativePath].includes(file)?Buffer.from(readGlobalGrammarContextSourcePair(file).successor.source):readFileSync(join(root,file));
      size+=bytes.length;assert.ok(size<=2*1024*1024);
      mkdirSync(dirname(join(scratch,file)),{recursive:true});writeFileSync(join(scratch,file),bytes);
    }
    const capture=captureOutcomeSourceViews(scratch,diagnosticOrderDecorator);
    candidate={...inventory,sourceViews:finalizeOutcomeSourceViews(capture,inventory.inputs,inventory.declarations)};
    return structuredClone(candidate);
  }finally{rmSync(scratch,{recursive:true,force:true});}
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
