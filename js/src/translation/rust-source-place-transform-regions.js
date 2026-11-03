// Symbolic state transport for the independently checked actual extraction,
// selected call and reconstruction graph. No archived epochs are trusted.
import {unsupported} from './diagnostics.js';
import {readRustSourcePlaceTransformPlan} from './rust-source-place-transforms.js';
import * as rules from './rust-borrowed-source-region-rules.js';
import {checkRustClosedSourceCallback} from './rust-closed-callback-regions.js';
const fail=(reason,node)=>{throw unsupported('Rust source place regions',reason,node?.span);};
const callPosition=position=>({...position,lifetimes:position.lifetimes.map(value=>value==='static'?'static':null)});
export function readRustSourcePlaceTransformRegions(visitor,node,env,bindings,depth){
 const plan=readRustSourcePlaceTransformPlan(node.sourcePlaceTransform,node,visitor.program.sourceLanguage,value=>value,value=>visitor.program.declarations.get(value.name),name=>visitor.program.declarations.get(name)),{facet,interface:interface_}=plan;
 if(!visitor.composed)fail('source place calls retain the explicit closed-call region composition',node);
 if(plan.kind==='replacement')for(const operand of facet.operands)visitor.scope(operand.position,bindings,node);
 const roots=facet.slots.map(slot=>{visitor.scope(slot.position,bindings,node);visitor.closedOutput(slot.position,node);const actual=env.get(slot.input);if(!actual)fail('a source place input retains its actual enclosing lexical binder',node);visitor.compatible(visitor.read(actual,'read',node),slot.position,node);const storage=visitor.storages.get(slot.binder.identity);if(!storage||actual.storageIdentity!==slot.binder.identity||actual.storageRoot!==storage.root||actual.regions.length||storage.binder.name!==slot.binder.name||storage.binder.start!==slot.binder.start||storage.binder.end!==slot.binder.end)fail('a source place input retains its actual initialized original binder and current closed storage revision',node);return {slot,actual,storage};});
 for(const position of interface_.outputs)visitor.closedOutput(position,node);const ordinary=[];
 for(let index=0;index<plan.arguments.length;index++){const place=facet.places.find(place=>place.parameter===index);if(place){visitor.read(roots[place.slot].actual,'read',node);continue;}const argument=plan.arguments[index],parameter=interface_.parameters[index],mode=interface_.modes[index];if(mode==='callback'){checkRustClosedSourceCallback(visitor,argument,interface_,env);continue;}visitor.closedTarget(parameter,argument);const shared=mode==='shared'&&parameter.wrappers.length===0?{...parameter,wrappers:['reference'],lifetimes:[null]}:parameter,expected=callPosition(visitor.position(shared)),actual=visitor.compatible(visitor.expression(argument,env,bindings,expected,depth+1),expected,argument);visitor.read(actual,mode==='shared'?'read':'move',argument);ordinary.push(actual);}
 // Acquire distinct real whole-root epochs once, even when several disjoint
 // fields of the same record are selected by separate mutable formals.
 for(const root of roots){visitor.read(root.actual,'read',node);visitor.states[root.storage.root]=rules.consumeRustBorrowedSourceRegion(visitor.states[root.storage.root]);if(root.actual.root!==null&&root.actual.root!==root.storage.root)visitor.states[root.actual.root]=rules.consumeRustBorrowedSourceRegion(visitor.states[root.actual.root]);}
 for(const actual of ordinary)if(['reference','borrowed vector'].includes(actual.role))visitor.read({...actual,root:null,producer:true},'read',node);
 const fields=[visitor.value(visitor.position(interface_.result),visitor.root(),[],true),...roots.map(({slot,storage})=>{const root=visitor.root();visitor.storages.set(slot.binder.identity,{...storage,root});return {...visitor.value(visitor.position(slot.position),null,[],true),storageRoot:root,storageIdentity:slot.binder.identity,packetState:true};})];return visitor.packet(fields,node);
}
