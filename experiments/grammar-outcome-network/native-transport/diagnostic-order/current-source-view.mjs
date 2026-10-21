// A successor profile composes with, and retains, the complete frozen profile.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {serializeProjectData} from '../../../../js/scripts/project-data.mjs';
import {restoreGlobalGrammarContextSource,checkGlobalGrammarContextSourceReceipt} from '../../../global-grammar-context-reuse/current-source.mjs';
import {outcomeWirePolicy} from '../wire/wire-policy.mjs';
import {deriveDiagnosticOrderDecorator,applyDiagnosticOrderDecorator,readJavaScriptDiagnosticOrders,javascriptPath,nativePath} from './generate.mjs';
import {finalizeOutcomeSourceViews as finalizePredecessorSourceViews,checkOutcomeSourceViews as checkPredecessorSourceViews,applyFrozenOutcomeProducerCorrection} from './source-view.mjs';
import {outcomeCrateRootPath,restoreOutcomeCrateRootSource,finalizeOutcomeCrateRootSourceView} from './crate-root-source-view.mjs';
export {applyFrozenOutcomeProducerCorrection};

const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const canonical=value=>serializeProjectData(value,'current-source-view.lino');
const copy=value=>structuredClone(value);
const diagnosticRows=new Map();
const nativeOwnerProofs=new Map();

function readCurrentContextDiagnosticRows(source) {
  if(!diagnosticRows.has(source))diagnosticRows.set(source,readJavaScriptDiagnosticOrders(source));
  return copy(diagnosticRows.get(source));
}

function authenticateCurrentContextDecorator(decorator) {
  const javascript=decorator.inputs.find(row=>row.path===javascriptPath)?.source;
  const native=decorator.inputs.find(row=>row.path===nativePath)?.source;
  const derived=deriveDiagnosticOrderDecorator(javascript,native);
  assert.equal(canonical(decorator),canonical(derived.decorator));return derived;
}

/** Capture current successors while supplying precisely the old inputs to the
 * original source profile. Every other historical source pin stays exact. */
export function captureOutcomeSourceViews(root,decorator) {
  const checked=authenticateCurrentContextDecorator(decorator),historical=new Map();
  const pins=[],currentPins=[],adaptations=[],contextSuccessors=[],crateRootSuccessors=[];
  for(const[file,expected]of Object.entries(outcomeWirePolicy.pins)){
    const actual=readFileSync(resolve(root,file)),actualPin=pin(actual);let predecessor=actual;
    if(actualPin.sha256!==expected&&file===outcomeCrateRootPath){
      const current=restoreOutcomeCrateRootSource(file,actual.toString('utf8'));
      predecessor=Buffer.from(current.predecessor);
      if(current.changed)crateRootSuccessors.push({file,predecessorSource:current.predecessor,currentSource:current.current,receipt:current.receipt});
    }
    if(actualPin.sha256!==expected&&[javascriptPath,nativePath].includes(file)){
      const context=restoreGlobalGrammarContextSource(file,actual.toString('utf8'));
      predecessor=Buffer.from(context.predecessor);
      if(context.changed)contextSuccessors.push({file,predecessorSource:context.predecessor,currentSource:context.current,receipt:context.receipt});
    }
    let original=predecessor;
    if(pin(predecessor).sha256!==expected){
      assert.equal(file,nativePath,'only the original native diagnostic permutation changes the predecessor');
      assert.equal(predecessor.toString('utf8'),checked.corrected);
      const inverse=applyDiagnosticOrderDecorator(checked.corrected,decorator,{direction:'inverse',role:'production'});
      original=Buffer.from(inverse.source);
      adaptations.push({file,sourceView:'historical baseline plus separately authenticated current producer',
        historicalSource:inverse.source,currentSource:checked.corrected,inverseReceipt:inverse.receipt});
    }
    assert.equal(pin(original).sha256,expected,'all thirteen original source pins stay exact');
    historical.set(file,original);pins.push({file,...pin(original)});currentPins.push({file,...pin(predecessor)});
  }
  return{historical,pins,currentPins,adaptations,contextSuccessors,crateRootSuccessors};
}

function currentContextOwnerProof(successor,predecessorCapture,inputs,declarations) {
  if(successor.file===javascriptPath){
    const before=readCurrentContextDiagnosticRows(successor.predecessorSource),after=readCurrentContextDiagnosticRows(successor.currentSource);
    assert.equal(before.length,9);assert.equal(after.length,before.length);
    const calls=before.map((row,index)=>{
      const current=after[index],source=Buffer.from(successor.predecessorSource).subarray(row.call.span.start,row.call.span.end).toString();
      assert.equal(Buffer.from(successor.currentSource).subarray(current.call.span.start,current.call.span.end).toString(),source);
      assert.deepEqual(current.keys,row.keys);assert.equal(current.reason,row.reason);assert.equal(current.detail,row.detail);
      return{source,keys:row.keys,reason:row.reason,detail:row.detail,historicalCall:row.call,currentCall:current.call};
    });
    return{diagnosticCalls:calls,conclusion:'All nine complete current diagnostic calls preserve every field value spelling and its order.'};
  }
  assert.equal(successor.file,nativePath);
  // Cache only complete immutable inputs. Every caller field participates in
  // the key, and returned proofs are copies. Retain at most two input versions.
  const key=canonical({source:successor.currentSource,inputs:inputs.filter(row=>row.file===nativePath),declarations:declarations.filter(row=>row.file===nativePath)});
  if(nativeOwnerProofs.has(key))return copy(nativeOwnerProofs.get(key));
  const capture={...predecessorCapture,adaptations:predecessorCapture.adaptations.map(row=>row.file===nativePath?{...row,currentSource:successor.currentSource}:row)};
  const actual=finalizePredecessorSourceViews(capture,inputs,declarations).adaptations.find(row=>row.file===nativePath);
  assert.ok(actual,'a native successor requires its exact original diagnostic predecessor');
  const{imports,selectedDeclarations}=actual.currentSourceProof;
  const result={imports,selectedDeclarations,conclusion:'Every actual current native import, selected complete type declaration and original byte interval remains exact after context reuse.'};
  if(nativeOwnerProofs.size===2)nativeOwnerProofs.delete(nativeOwnerProofs.keys().next().value);
  nativeOwnerProofs.set(key,copy(result));return copy(result);
}

/** Schema two retains the complete schema-one predecessor and adds distinct
 * current bytes, receipts and actual CST owner proofs. Frozen overlay output
 * remains the original diagnostic permutation rather than the new producer. */
function finalizeContextSourceViews(capture,inputs,declarations) {
  const predecessorView=finalizePredecessorSourceViews(capture,inputs,declarations);
  if(!capture.contextSuccessors?.length)return predecessorView;
  const currentPins=copy(predecessorView.currentPins),successors=[];
  for(const successor of capture.contextSuccessors){
    const context=restoreGlobalGrammarContextSource(successor.file,successor.currentSource);assert.equal(context.changed,true);
    assert.equal(context.predecessor,successor.predecessorSource);checkGlobalGrammarContextSourceReceipt(successor.file,successor.currentSource,successor.receipt);
    const current=currentPins.find(row=>row.file===successor.file);assert.ok(current);
    assert.deepEqual({bytes:current.bytes,sha256:current.sha256},pin(context.predecessor));
    Object.assign(current,pin(context.current));
    successors.push({...copy(successor),currentSourceProof:currentContextOwnerProof(successor,capture,inputs,declarations)});
  }
  assert.ok(successors.length<=2);assert.equal(new Set(successors.map(row=>row.file)).size,successors.length);
  return{schemaVersion:2,profile:'complete current grammar context source views 1',historicalPins:copy(predecessorView.historicalPins),currentPins,
    predecessorView,successors,boundary:'The complete original source profile remains mandatory. Actual successors have separate whole-source receipts and current CST proofs; frozen overlay identities and observations remain separate.'};
}

/** Historical archives keep their exact original validator. Current archives
 * must reproduce both complete profiles and every current owner proof. */
function checkContextSourceViews(inventory,decorator) {
  const view=inventory.sourceViews;
  if(view?.schemaVersion===1)return checkPredecessorSourceViews(inventory,decorator);
  assert.equal(view?.schemaVersion,2);authenticateCurrentContextDecorator(decorator);
  const predecessor=checkPredecessorSourceViews({...inventory,sourceViews:view.predecessorView},decorator);
  assert.deepEqual(view.historicalPins,predecessor.historicalPins);
  assert.ok(Array.isArray(view.successors)&&view.successors.length>0&&view.successors.length<=2);
  assert.equal(new Set(view.successors.map(row=>row.file)).size,view.successors.length);
  const capture={pins:predecessor.historicalPins,currentPins:predecessor.currentPins,adaptations:predecessor.adaptations,
    contextSuccessors:view.successors.map(({file,predecessorSource,currentSource,receipt})=>({file,predecessorSource,currentSource,receipt}))};
  const expected=finalizeContextSourceViews(capture,inventory.inputs,inventory.declarations);
  assert.equal(canonical(view),canonical(expected),'complete original and current context source conservation');
  return copy(expected);
}


/** The crate-root successor wraps the complete preceding context profile; no original gate is broadened. */
export function finalizeOutcomeSourceViews(capture,inputs,declarations) {
  const predecessor=finalizeContextSourceViews(capture,inputs,declarations),successors=capture.crateRootSuccessors??[];
  assert.ok(Array.isArray(successors)&&successors.length<=1);
  return successors.length?finalizeOutcomeCrateRootSourceView(predecessor,successors[0]):predecessor;
}

export function checkOutcomeSourceViews(inventory,decorator) {
  const view=inventory.sourceViews;
  if(view?.schemaVersion!==3)return checkContextSourceViews(inventory,decorator);
  const predecessor=checkContextSourceViews({...inventory,sourceViews:view.predecessorView},decorator);
  const expected=finalizeOutcomeCrateRootSourceView(predecessor,view.crateRootSuccessor);
  assert.equal(canonical(view),canonical(expected),'complete original context and current crate-root source conservation');
  return copy(expected);
}
