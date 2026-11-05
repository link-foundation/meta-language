import assert from 'node:assert/strict';
import {checkProgram} from '../../src/translation/check.js';
import {parseJavaScript} from '../../src/translation/javascript.js';
import {checkJavaScriptClassOwners} from '../../src/translation/javascript-class-owners.js';
import {emitJavaScript} from '../../src/translation/emit-javascript.js';
import {emitRust} from '../../src/translation/emit-rust.js';
import {emitLean} from '../../src/translation/emit-lean.js';
import {emitRocq} from '../../src/translation/emit-rocq.js';
import {createTypedProgramLinks,encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../src/typed-program-links.js';
import {LinkNetwork} from '../../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
import {closedClassOwnerOriginals,closedClassOwnerArchiveEdits,closedClassOwnerGraphEdits} from '../fixtures/closed-class-owner-witnesses.mjs';
import {createControlDiagnostics,finishControlDiagnostics,requireControlProduct as product} from './control-diagnostics.js';
function objects(root){const result=[],pending=[root],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<100000);seen.add(value);result.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return result;}
// Literal edits target actual declaration/type references, never private name
// patterns. The complete unmodified base is retained alongside every edit.
export function readClosedClassOwnerArchiveEdit(name){
 assert.ok(closedClassOwnerArchiveEdits.includes(name));
 const source=closedClassOwnerOriginals.at(-1).source,program=structuredClone(checkProgram(parseJavaScript(source)));
 const witness=program.sourceClassOwners,owner=witness.owners.find(owner=>owner.constructor==='Box');assert.ok(owner);
 const nodes=objects(program),types=nodes.filter(node=>node.kind==='record'&&node.sourceClassOwner===owner.constructor);
 const call=nodes.find(node=>node.k==='call'&&node.fn==='Box'&&node.args.length===1&&node.args[0].k==='lit');assert.ok(call);assert.ok(types.length);
 if(name==='dropped witness')delete program.sourceClassOwners;
 else if(name==='null witness')program.sourceClassOwners=null;
 else if(name==='empty witness')witness.owners=[];
 else if(name==='wrong model')witness.model='unregistered model';
 else if(name==='wrong version')witness.schemaVersion=2;
 else if(name==='wrong constructor reference')owner.constructor='plain';
 else if(name==='wrong helper reference')owner.methods[0].helper='plain';
 else if(name==='wrong brand')owner.brand='not the actual brand';
 else if(name==='empty owner')types[0].sourceClassOwner='';
 else if(name==='null owner')types[0].sourceClassOwner=null;
 else if(name==='numeric owner')types[0].sourceClassOwner=3;
 else if(name==='erased owner')delete types[0].sourceClassOwner;
 else if(name==='retagged plain record'){const plain=program.declarations.get('plain');assert.ok(plain);plain.ret.sourceClassOwner='Box';}
 else {
  const text=nodes.find(node=>node.k==='toString'&&node.arg?.k==='call'&&node.arg.fn==='compute');assert.ok(text);
  if(name==='direct debug observation')text.arg=structuredClone(call);
  else if(name==='nested debug observation'){const nested=nodes.find(node=>node.k==='call'&&node.fn==='wrapper');assert.ok(nested);text.arg=structuredClone(nested);}
  else if(name==='prototype identity observation')text.arg={k:'binary',op:'eq',left:structuredClone(call),right:structuredClone(call),type:{kind:'bool'}};
 }
 return {name,source,program};
}
export function readClosedClassOwnerArchiveControl(name){
 const row=readClosedClassOwnerArchiveEdit(name),consumers=[checkJavaScriptClassOwners,encodeTypedProgramLinks,emitJavaScript,emitRust,emitLean,emitRocq];
 const diagnostic=createControlDiagnostics();
 for(const consume of consumers)diagnostic.attempt(consume.name,()=>assert.throws(()=>consume(row.program),name+': '+consume.name));
 return finishControlDiagnostics({...row,program:{...row.program,declarations:[...row.program.declarations.values()]},assertions:consumers.map(consume=>consume.name),diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
}
export function readClosedClassOwnerGraphControl(name){
 assert.ok(closedClassOwnerGraphEdits.includes(name));
 const source=closedClassOwnerOriginals.at(-1).source,packet=createTypedProgramLinks(source,'JavaScript'),network=packet.network;
 const relations=network.fieldRelations(),label=name=>schema.fields[name]??name;
 const field=(parent,name)=>relations.find(row=>row.parent.equals(parent)&&row.label===name)?.child;
 const kind=parent=>{const node=field(parent,'construct kind');return node&&network.link(node).metadata().definition;};
 const declarationName=parent=>{const node=field(parent,label('fullName'));return node&&network.link(node).metadata().definition;};
 const replace=(parent,key,child)=>{const relation=relations.find(row=>row.parent.equals(parent)&&row.label===key);assert.ok(relation);relation.link.setReferences([parent,relation.link.references()[1],child]);};
 if(name==='retargeted constructor edge'){
  const edge=relations.find(row=>row.label===schema.references.classConstructor);assert.ok(edge);
  const plain=relations.find(row=>row.label==='construct kind'&&kind(row.parent)==='fn'&&declarationName(row.parent)==='plain')?.parent;assert.ok(plain);replace(edge.parent,edge.label,plain);
 }else{
  const selected=name==='restored direct debug observation'?'Box':'wrapper';
  const call=relations.find(row=>{if(row.label!=='construct kind'||kind(row.parent)!=='call')return false;const target=field(row.parent,schema.references.declaration);if(!target||declarationName(target)!==selected)return false;if(selected==='wrapper')return true;const args=field(row.parent,label('args'));const first=args&&network.link(args).references()[0];return first&&kind(first)==='lit';})?.parent;assert.ok(call);
  const conversion=relations.find(row=>row.label==='construct kind'&&kind(row.parent)==='toString')?.parent;assert.ok(conversion);replace(conversion,label('arg'),call);
 }
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 const links=attempt('edited text encoding',()=>network.toLino()),binary=attempt('edited binary encoding',()=>encodeLinkNetworkBinary(network));
 const variants=[network,attempt('edited text reload',()=>LinkNetwork.fromLino(product(links,'edited text encoding'))),attempt('edited binary reload',()=>decodeLinkNetworkBinary(product(binary,'edited binary encoding')))];
 for(let at=0;at<variants.length;at++){
  const restored=variants[at],transport=['original','text','binary'][at];
  attempt(transport+'/exact text graph',()=>assert.equal(product(restored,transport+' graph').toLino(),product(links,'edited text encoding')));
  attempt(transport+'/typed decode refusal',()=>{product(restored,transport+' graph');assert.throws(()=>decodeTypedProgramLinks(restored,packet.root),name);});
  for(const target of ['JavaScript','Rust','Lean','Rocq'])attempt(transport+'/'+target+'/emitter refusal',()=>{product(restored,transport+' graph');assert.throws(()=>emitTypedProgramLinks(restored,packet.root,target),name+'/'+target);});
 }
 return finishControlDiagnostics({name,source,root:packet.root.asU64(),links,...(binary!==undefined?{binary:[...binary]}:{}),targets:['JavaScript','Rust','Lean','Rocq'],transports:['original','text','binary'],diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
}
