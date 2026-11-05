// Exact crate-root source succession; complete earlier transport profiles remain mandatory.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,serializeProjectData} from '../../../../js/scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {outcomeWirePolicy} from '../wire/wire-policy.mjs';
import {readNativeCompilerHostSourceHistory,restoreNativeCompilerHostSource} from '../../../../js/scripts/native-compiler-host-source-history.mjs';

export const outcomeCrateRootPath='rust/src/lib.rs';
export const OUTCOME_CRATE_ROOT_SOURCE_PROFILE=Object.freeze({
  revision:'897bfc8b2dcc5ec561feffa455ddeb9f95d24335',
  file:outcomeCrateRootPath,
  current:Object.freeze({bytes:20851,sha256:'9f4f452ae9ebcf5165506c47d67585af423bfd99d5ccd8640e8226a936dbbc46'}),
  origin:Object.freeze({path:'parity/naming/own-codebase-baseline-predecessor-source-history.lino',sha256:'2aab89c9bf21a9e6f3addf28776bed84ccfdb192e6bb69b59c3fa546e1734abc'}),
  recipe:'experiments/grammar-outcome-network/native-transport/diagnostic-order/crate-root-source-successor.lino',
});
const recipeSha256='c4048cdefd81b07b4ac6130a96476e143b3c40c0426d642dfb8d46c5896c6d13';
const root=new URL('../../../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const canonical=value=>serializeProjectData(value,'crate-root-source-view.lino');
const tokenText=source=>tokenize(source,'Rust').tokens.filter(token=>token.kind!=='eof').map(token=>token.value).join('');
let sourcePair;
const crateRootOwnerProofs=new Map();

/** Check the original complete Git source and derive one bounded exact byte successor.
 * This certificate proves source membership and inversion, never runtime semantics. */
export function verifyOutcomeCrateRootSourcePacket(packet) {
  const profile=OUTCOME_CRATE_ROOT_SOURCE_PROFILE,row=packet.source;
  assert.deepEqual(Object.keys(packet).sort(),['boundary','origin','revision','schemaVersion','source']);
  assert.deepEqual(packet.origin,profile.origin);
  assert.deepEqual(Object.keys(row).sort(),['bytes','certificate','commit','current','edit','file','sha256','sourceText']);
  assert.deepEqual(row.current,profile.current,'a different successor cannot certify its own current source pin');
  assert.equal(packet.schemaVersion,1);assert.equal(packet.revision,profile.revision);
  assert.equal(row.file,profile.file);assert.equal(row.commit,profile.revision);
  assert.equal(row.sha256,outcomeWirePolicy.pins[profile.file],'the independently frozen original crate root stays exact');
  const before=Buffer.from(row.sourceText,'utf8');assert.equal(before.toString('utf8'),row.sourceText);
  assert.ok(before.length<=131072);assert.deepEqual(pin(before),{bytes:row.bytes,sha256:row.sha256});
  verifyGitArtifactCertificate(row.certificate,before,{revision:profile.revision,path:profile.file});
  const edit=row.edit;assert.deepEqual(Object.keys(edit).sort(),['deleteBytes','insertText','start']);
  assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=before.length);
  assert.ok(Number.isSafeInteger(edit.deleteBytes)&&edit.deleteBytes>=0&&edit.start+edit.deleteBytes<=before.length);
  assert.equal(typeof edit.insertText,'string');const insertion=Buffer.from(edit.insertText,'utf8');
  assert.equal(insertion.toString('utf8'),edit.insertText);
  const after=Buffer.concat([before.subarray(0,edit.start),insertion,before.subarray(edit.start+edit.deleteBytes)]);
  assert.ok(after.length<=131072);assert.equal(Buffer.from(after.toString('utf8'),'utf8').equals(after),true);
  assert.deepEqual(pin(after),row.current);assert.notDeepEqual(pin(after),pin(before));
  const inverse=Buffer.concat([after.subarray(0,edit.start),before.subarray(edit.start,edit.start+edit.deleteBytes),after.subarray(edit.start+insertion.length)]);
  assert.equal(inverse.equals(before),true,'the entire independently committed crate root is restored');
  return{file:row.file,revision:row.commit,predecessor:{source:row.sourceText,...pin(before)},successor:{source:after.toString('utf8'),...pin(after)},edit:structuredClone(edit)};
}

export function readOutcomeCrateRootSourcePair() {
  if(!sourcePair){
    const path=new URL(OUTCOME_CRATE_ROOT_SOURCE_PROFILE.recipe,root),bytes=readFileSync(path);
    assert.equal(pin(bytes).sha256,recipeSha256,'complete independently pinned crate-root successor recipe');
    sourcePair=verifyOutcomeCrateRootSourcePacket(readProjectData(path));
  }
  return structuredClone(sourcePair);
}

/** Admit only the complete original crate root or its independently registered exact successor. */
export function restoreOutcomeCrateRootSource(file,source) {
  assert.equal(file,outcomeCrateRootPath);assert.equal(typeof source,'string');assert.ok(Buffer.byteLength(source)<=131072);
  const pair=readOutcomeCrateRootSourcePair();
  if(source===pair.predecessor.source)return{file,predecessor:source,current:source,changed:false,receipt:null};
  const preceding=restoreNativeCompilerHostSource(source,file);
  if(preceding!==source){
    assert.equal(preceding,pair.successor.source,'the complete earlier root recipe remains exact');
    const compilerSourceSuccession=readNativeCompilerHostSourceHistory().sources.find(row=>row.file===file);assert.ok(compilerSourceSuccession);
    return{file,predecessor:pair.predecessor.source,current:source,changed:true,receipt:{
      schemaVersion:1,profile:'complete grammar transport crate root source successor 2',file,revision:pair.revision,
      recipe:{path:OUTCOME_CRATE_ROOT_SOURCE_PROFILE.recipe,sha256:recipeSha256},
      predecessor:pin(pair.predecessor.source),current:pin(source),inverseVerified:true,compilerSourceSuccession,
      boundary:'The complete earlier root recipe and its original Git source remain exact; independently certified compiler succession retains the whole actual current root and every current CST export owner. Source conservation is not native execution evidence.'}};
  }
  assert.equal(source,pair.successor.source,'unknown complete crate-root source edits refuse');
  return{file,predecessor:pair.predecessor.source,current:source,changed:true,receipt:{
    schemaVersion:1,profile:'complete grammar transport crate root source successor 1',file,revision:pair.revision,
    recipe:{path:OUTCOME_CRATE_ROOT_SOURCE_PROFILE.recipe,sha256:recipeSha256},
    predecessor:pin(pair.predecessor.source),current:pin(source),inverseVerified:true,
    boundary:'Exact source succession and original Git membership only; complete current reexport proofs are separate and native execution remains a CI obligation.'}};
}

export function checkOutcomeCrateRootSourceReceipt(file,source,receipt) {
  const actual=restoreOutcomeCrateRootSource(file,source);assert.equal(actual.changed,true);
  assert.equal(canonical(receipt),canonical(actual.receipt),'complete crate-root receipt conservation');return structuredClone(actual.receipt);
}

function readCrateRootOwnerProof(source) {
  // The entire source and every consumed policy field participate in this key.
  // Keep only the complete predecessor/current pair, and never share returned objects.
  const key=canonical({source,rootReexport:outcomeWirePolicy.rootReexport,sources:outcomeWirePolicy.sources});
  if(crateRootOwnerProofs.has(key))return structuredClone(crateRootOwnerProofs.get(key));
  const parsed=readRustSourceItems(Buffer.from(source)),exports=parsed.items.filter(item=>item.kind==='use_declaration');
  const describe=item=>({source:parsed.text(item.node),span:{start:item.node.start,end:item.node.end}});
  const roots=exports.filter(item=>tokenText(parsed.text(item.node))===tokenText(outcomeWirePolicy.rootReexport));
  assert.equal(roots.length,1,'the actual complete grammar runtime crate-root reexport stays owned');
  const selectedOwners=outcomeWirePolicy.sources.flatMap(row=>row.names).map(name=>{
    const owners=exports.filter(item=>tokenize(parsed.text(item.node),'Rust').tokens.some(token=>token.value===name));
    assert.equal(owners.length,1,'every actual grammar runtime crate-root owner stays unique: '+name);
    return{name,...describe(owners[0])};
  });
  const result={reexports:exports.map(describe),rootReexport:describe(roots[0]),selectedOwners};
  if(crateRootOwnerProofs.size===2)crateRootOwnerProofs.delete(crateRootOwnerProofs.keys().next().value);
  crateRootOwnerProofs.set(key,structuredClone(result));return structuredClone(result);
}

/** Schema three retains the whole original context profile and records actual current crate-root owners. */
export function finalizeOutcomeCrateRootSourceView(predecessorView,successor) {
  assert.ok([1,2].includes(predecessorView.schemaVersion));
  const actual=restoreOutcomeCrateRootSource(successor.file,successor.currentSource);assert.equal(actual.changed,true);
  assert.equal(actual.predecessor,successor.predecessorSource);
  checkOutcomeCrateRootSourceReceipt(successor.file,successor.currentSource,successor.receipt);
  const before=readCrateRootOwnerProof(actual.predecessor),after=readCrateRootOwnerProof(actual.current);
  assert.equal(after.rootReexport.source,before.rootReexport.source,'the complete selected root reexport is unchanged');
  assert.deepEqual(after.selectedOwners.map(({name,source})=>({name,source})),before.selectedOwners.map(({name,source})=>({name,source})),
    'all complete selected transport export owners remain unchanged');
  const currentPins=structuredClone(predecessorView.currentPins),row=currentPins.find(item=>item.file===outcomeCrateRootPath);assert.ok(row);
  assert.deepEqual({bytes:row.bytes,sha256:row.sha256},pin(actual.predecessor));Object.assign(row,pin(actual.current));
  return{schemaVersion:3,profile:'complete current grammar transport crate root source views 1',
    historicalPins:structuredClone(predecessorView.historicalPins),currentPins,predecessorView:structuredClone(predecessorView),
    crateRootSuccessor:{file:actual.file,predecessorSource:actual.predecessor,currentSource:actual.current,receipt:actual.receipt,
      currentSourceProof:{predecessor:before,current:after}},
    boundary:'The complete original diagnostic and context source profiles stay mandatory. The exact current crate root and all its actual reexports have separate whole-source, Git and CST proofs; no semantic or native execution observation is implied.'};
}
