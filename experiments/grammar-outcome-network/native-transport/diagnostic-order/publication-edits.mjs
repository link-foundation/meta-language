// Mechanical additions to the retained source publication and archive replay.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';

const digest=source=>createHash('sha256').update(source).digest('hex');

const unique=(source,from)=>assert.equal(source.split(from).length,2,'one actual whole source occurrence');
const author=(original,changes)=>{
  const present=changes.map(([,to])=>original.split(to).length===2);
  const alreadyApplied=present.every(Boolean);assert.ok(alreadyApplied||present.every(value=>!value),'a partial publication addition must be reviewed explicitly');
  let predecessor=original;
  if(alreadyApplied)for(const [from,to]of [...changes].reverse()){unique(predecessor,to);predecessor=predecessor.replace(to,from);}
  let source=predecessor;const edits=[];
  for(const [from,to]of changes){unique(source,from);source=source.replace(from,to);edits.push({from,to,count:1});}
  let inverse=source;for(const {from,to}of [...edits].reverse()){unique(inverse,to);inverse=inverse.replace(to,from);}
  assert.equal(inverse,predecessor,'whole published predecessor is exactly reconstructible');
  if(alreadyApplied)assert.equal(source,original,'installed publication replays the exact same reviewed changes');
  return {source,edits,original:predecessor,alreadyApplied};
};

export function authorWireSourceViewEdits(original,{adapterImport='../diagnostic-order/source-view.mjs',decoratorImport='../diagnostic-order/decorator.mjs'}={}) {
  const additions="import {captureOutcomeSourceViews,finalizeOutcomeSourceViews,checkOutcomeSourceViews} from '"+adapterImport+"';\n"
    +"import {diagnosticOrderDecorator} from '"+decoratorImport+"';\n";
  return author(original,[
    ["import {explicitOutputDirectory} from './output.mjs';\n","import {explicitOutputDirectory} from './output.mjs';\n"+additions],
    ["  const pins=[];\n  for(const [file,sha256] of Object.entries(policy.pins)) {\n    const bytes=readFileSync(resolve(root,file));\n    assert.equal(digest(bytes),sha256,'reviewed complete input changed: '+file);\n    pins.push({file,bytes:bytes.length,sha256});\n  }",
      '  const sourceCapture=captureOutcomeSourceViews(root,diagnosticOrderDecorator);\n  const pins=sourceCapture.pins;'],
    ["    const bytes=readFileSync(resolve(root,source.file));","    const bytes=sourceCapture.historical.get(source.file);"],
    ["  const rootBytes=readFileSync(resolve(root,'rust/src/lib.rs'));","  const rootBytes=sourceCapture.historical.get('rust/src/lib.rs');"],
    ["  return {schemaVersion:1,model:policy.model,pins,inputs,declarations,\n    rootReexport:",
      "  return {schemaVersion:1,model:policy.model,pins,inputs,declarations,\n    sourceViews:finalizeOutcomeSourceViews(sourceCapture,inputs,declarations),\n    rootReexport:"],
    ["export function checkOutcomeInventory(inventory) {\n","export function checkOutcomeInventory(inventory) {\n  checkOutcomeSourceViews(inventory,diagnosticOrderDecorator);\n"],
    ["    inputs:inventory.pins,sourceBytes:","    inputs:inventory.pins,sourceViews:inventory.sourceViews,sourceBytes:"]
  ]);
}

export function authorBridgeSourceViewEdits(original) {
  return author(original,[[
    '      originalParentPin:outcomeWirePolicy.pins[parentPath],pins:packet.receipt.inputs},',
    '      originalParentPin:outcomeWirePolicy.pins[parentPath],pins:packet.receipt.inputs,sourceViews:packet.receipt.sourceViews},'
  ]]);
}

export function authorRunnerPlanSourceViewEdits(original,{adapterImport='../diagnostic-order/source-view.mjs',decoratorImport='../diagnostic-order/decorator.mjs'}={}) {
  const additions="import {applyFrozenOutcomeProducerCorrection} from '"+adapterImport+"';\n"
    +"import {diagnosticOrderDecorator} from '"+decoratorImport+"';\n";
  return author(original,[
    ["import {verifyPublication} from '../wire/publication.mjs';\n","import {verifyPublication} from '../wire/publication.mjs';\n"+additions],
    ["  for(const input of receipt.predecessor.pins) assert.equal(sha256(bytesAt(repository,input.file)),input.sha256,'predecessor source: '+input.file);",
      "  assert.deepEqual(receipt.predecessor.pins,receipt.predecessor.sourceViews.historicalPins,'frozen predecessors remain separate');\n"
      +"  for(const input of receipt.predecessor.sourceViews.currentPins) assert.equal(sha256(bytesAt(repository,input.file)),input.sha256,'current authenticated source: '+input.file);\n"
      +"  const originalProducer=diagnosticOrderDecorator.inputs.find(input=>input.path==='rust/src/grammar/feature_runtime/grammar_selection.rs').source;\n"
      +"  const producerCorrection=applyFrozenOutcomeProducerCorrection(originalProducer,diagnosticOrderDecorator);"],
    ['    nativeOraclePolicy:runnerPublication.nativeOraclePolicy};',
      '    nativeOraclePolicy:runnerPublication.nativeOraclePolicy,producerCorrection};']
  ]);
}

export function authorRunnerOverlaySourceViewEdits(original) {
  return author(original,[
    ["  writeFileSync(join(overlay,'rust/tests',policy.testName+'.rs'),plan.probe,{flag:'wx'});",
      "  writeFileSync(join(overlay,'rust/tests',policy.testName+'.rs'),plan.probe,{flag:'wx'});\n"
      +"  const producerPath='rust/src/grammar/feature_runtime/grammar_selection.rs';\n"
      +"  assert.equal(readFileSync(join(overlay,producerPath),'utf8'),plan.producerCorrection.original,'actual frozen producer matches the explicit adapter input');\n"
      +"  writeFileSync(join(overlay,producerPath),plan.producerCorrection.source);"],
    ['    bridgeReceipt:plan.receipt,nativeOraclePolicy:plan.nativeOraclePolicy,processes:[]};',
      '    bridgeReceipt:plan.receipt,producerCorrection:plan.producerCorrection.receipt,nativeOraclePolicy:plan.nativeOraclePolicy,processes:[]};']
  ]);
}

/** Append to a retained publisher record; original whole-source bytes, hashes,
 * prior edits and archival candidate inputs stay intact. Root's existing normal
 * Links writer serializes the new record; never hand-transcribe archive rows.
 */
export function appendWirePublicationEdits(record,addition) {
  assert.equal(digest(record.original),record.originalSha256);assert.equal(Buffer.byteLength(record.original),record.originalBytes);
  assert.equal(Boolean(record.formatting),false,'only exact JS source records use this addition');
  let current=record.original;
  for(const {from,to,count}of record.edits){assert.equal(current.split(from).length-1,count);current=current.split(from).join(to);}
  assert.equal(digest(current),record.publishedSha256);assert.equal(Buffer.byteLength(current),record.publishedBytes);
  const result=addition(current);
  if(result.alreadyApplied)assert.deepEqual(record.edits.slice(-result.edits.length),result.edits,'installed archive retains the complete appended source author');
  const updated={...structuredClone(record),edits:result.alreadyApplied?structuredClone(record.edits):[...structuredClone(record.edits),...result.edits],
    publishedSha256:digest(result.source),publishedBytes:Buffer.byteLength(result.source)};
  let inverse=result.source;
  for(const {from,to,count}of [...updated.edits].reverse()){assert.equal(inverse.split(to).length-1,count);inverse=inverse.split(to).join(from);}
  assert.equal(inverse,record.original,'all historical and new publication changes have one complete source inverse');
  return {record:updated,source:result.source};
}

/** Reuse the complete original recipe while retaining independently published
 * receipt fields added after it. The old author remains available unchanged.
 */
export function authorRunnerOverlayReceiptSourceViewEdits(original,historicalRunner) {
  assert.equal(digest(historicalRunner),'a584080a3b4f281c1b239ce81ffa8cd5ab135f9449700aa26d88c4e164b7415e','the complete original compiler-profile predecessor');
  const previous=authorRunnerOverlaySourceViewEdits(historicalRunner);
  assert.equal(previous.edits.length,2,'the complete historical producer correction recipe');
  const tail='processes:[]};';
  const changes=previous.edits.map(({from,to,count},index)=>{
    assert.equal(count,1);
    if(index===0)return[from,to];
    assert.ok(from.endsWith(tail)&&to.endsWith(tail),'only the unchanged terminal receipt field is outside the member insertion');
    assert.equal(from.split(tail).length,2);assert.equal(to.split(tail).length,2);
    return[from.slice(0,-tail.length),to.slice(0,-tail.length)];
  });
  return author(original,changes);
}
