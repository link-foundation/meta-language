// Historical inputs, current producers and their exact inverse stay explicit.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {createHash} from 'node:crypto';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {serializeProjectData} from '../../../../js/scripts/project-data.mjs';
import {outcomeWirePolicy} from '../wire/wire-policy.mjs';
import {deriveDiagnosticOrderDecorator,applyDiagnosticOrderDecorator,nativePath,javascriptPath} from './generate.mjs';

const digest=value=>createHash('sha256').update(value).digest('hex');
const canonical=value=>serializeProjectData(value,'source-view.lino');
const copy=value=>structuredClone(value);

// Only syntax derived from these exact whole source bytes is retained. The
// private entry never contains caller imports, declarations, proofs or returns.
let currentDeclarationSource;
function readCurrentDeclarationSource(source) {
  assert.equal(typeof source,'string');
  if(currentDeclarationSource?.source!==source) {
    const bytes=Buffer.from(source);
    currentDeclarationSource={source,bytes,parsed:readRustSourceItems(bytes)};
  }
  return currentDeclarationSource;
}

function authenticatedDecorator(decorator) {
  const originalJavaScript=decorator.inputs.find(input=>input.path===javascriptPath)?.source;
  const originalNative=decorator.inputs.find(input=>input.path===nativePath)?.source;
  const result=deriveDiagnosticOrderDecorator(originalJavaScript,originalNative);
  assert.equal(canonical(result.decorator),canonical(decorator),'complete actual source-derived decorator');return result;
}

/** A file read is authenticated before its historical view is returned. */
export function captureOutcomeSourceViews(root,decorator) {
  const checked=authenticatedDecorator(decorator),historical=new Map(),pins=[],currentPins=[],adaptations=[];
  for(const [file,expected]of Object.entries(outcomeWirePolicy.pins)) {
    const current=readFileSync(resolve(root,file));let original=current;
    const currentSha256=digest(current);
    if(currentSha256!==expected) {
      assert.equal(file,nativePath,'only the separately declared producer permutation may differ from a frozen pin');
      assert.equal(current.toString('utf8'),checked.corrected,'exact reviewed current producer');
      const inverse=applyDiagnosticOrderDecorator(current.toString('utf8'),decorator,{direction:'inverse',role:'production'});
      original=Buffer.from(inverse.source);
      adaptations.push({file,sourceView:'historical baseline plus separately authenticated current producer',
        historicalSource:original.toString('utf8'),currentSource:current.toString('utf8'),inverseReceipt:inverse.receipt});
    }
    assert.equal(digest(original),expected,'unchanged historical whole-source pin');historical.set(file,original);
    pins.push({file,bytes:original.length,sha256:expected});
    currentPins.push({file,bytes:current.length,sha256:currentSha256});
  }
  return {historical,pins,currentPins,adaptations};
}

function currentDeclarations(adaptation,inputs,declarations) {
  const input=inputs.find(item=>item.file===adaptation.file);assert.ok(input,'changed producer is also an actual selected input owner');
  assert.equal(input.source,adaptation.historicalSource,'the input source is explicitly the historical view');
  const {bytes,parsed}=readCurrentDeclarationSource(adaptation.currentSource);
  const imports=parsed.items.filter(item=>item.kind==='use_declaration').map(item=>({
    source:parsed.text(item.node),span:{start:item.node.start,end:item.node.end}
  }));
  assert.deepEqual(imports,input.imports,'every actual current import, order and byte offset is unchanged');
  const selected=declarations.filter(item=>item.file===adaptation.file).map(declaration=>{
    const matches=parsed.items.filter(item=>item.name===declaration.name&&item.kind===declaration.kind);
    assert.equal(matches.length,1,'current declaration owner stays unique');const actual=matches[0];
    assert.deepEqual({start:actual.start,end:actual.end},declaration.span,'current declaration byte offsets remain exact');
    const source=bytes.subarray(actual.start,actual.end).toString('utf8');
    assert.equal(source,declaration.source,'the entire current declaration, including every nested type and field, remains exact');
    assert.equal(digest(source),declaration.sha256);
    return {file:declaration.file,name:declaration.name,kind:declaration.kind,
      span:copy(declaration.span),bytes:Buffer.byteLength(source),sha256:declaration.sha256};
  });
  assert.ok(selected.length>0,'at least one selected current owner is proved');
  return {imports,selectedDeclarations:selected,
    conclusion:'All selected current declarations and nested pre-erasure types are byte-identical at their original offsets; only the explicit producer macro pair order differs.'};
}

export function finalizeOutcomeSourceViews(capture,inputs,declarations) {
  return {schemaVersion:1,historicalPins:copy(capture.pins),currentPins:copy(capture.currentPins),
    adaptations:capture.adaptations.map(adaptation=>({...copy(adaptation),currentSourceProof:currentDeclarations(adaptation,inputs,declarations)})),
    boundary:'inventory.inputs/source use the authenticated historical view; actual current bytes/pins and every permitted inverse remain explicit here.'};
}

/** Validate serialized provenance, including direct current CST proofs. A
 * changed historical/current label or omitted current owner never authenticates.
 */
export function checkOutcomeSourceViews(inventory,decorator) {
  const checked=authenticatedDecorator(decorator),view=inventory.sourceViews;
  assert.ok(view&&view.schemaVersion===1,'explicit original/current source provenance is required');
  assert.deepEqual(view.historicalPins,inventory.pins,'all original thirteen recorded pins remain historical');
  assert.deepEqual(inventory.pins.map(item=>[item.file,item.sha256]),Object.entries(outcomeWirePolicy.pins));
  const currentPins=copy(inventory.pins),adaptations=[];
  assert.ok(Array.isArray(view.adaptations)&&view.adaptations.length<=1,'one explicit finite producer adaptation only');
  for(const adaptation of view.adaptations) {
    assert.equal(adaptation.file,nativePath);assert.equal(adaptation.currentSource,checked.corrected);
    const inverse=applyDiagnosticOrderDecorator(adaptation.currentSource,decorator,{direction:'inverse',role:'production'});
    assert.equal(adaptation.historicalSource,inverse.source);assert.deepEqual(adaptation.inverseReceipt,inverse.receipt);
    const expected={file:nativePath,sourceView:'historical baseline plus separately authenticated current producer',
      historicalSource:inverse.source,currentSource:checked.corrected,inverseReceipt:inverse.receipt};
    adaptations.push(expected);
    const pin=currentPins.find(item=>item.file===nativePath);pin.bytes=Buffer.byteLength(checked.corrected);pin.sha256=digest(checked.corrected);
  }
  const expected=finalizeOutcomeSourceViews({pins:inventory.pins,currentPins,adaptations:adaptations.map(({currentSourceProof,...item})=>item)},inventory.inputs,inventory.declarations);
  assert.equal(canonical(view),canonical(expected),'complete historical/current source-view conservation');
  return copy(view);
}

/** The frozen overlay is a distinct role. The frozen source hash never changes;
 * its exact delta and separately named current output are returned explicitly.
 */
export function applyFrozenOutcomeProducerCorrection(source,decorator) {
  const result=applyDiagnosticOrderDecorator(source,decorator,{role:'frozen CI overlay'});
  assert.equal(digest(source),outcomeWirePolicy.pins[nativePath]);
  return {...result,original:source};
}
