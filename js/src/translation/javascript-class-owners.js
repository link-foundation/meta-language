// Mechanical actual graph/type transport. No checker or emitter import: the
// shared policy validates provenance before either consumer can execute it.
import {createSurfaceReferenceFacts} from './surface-reference-facts.js';
import {typeKey} from './types.js';
import {typeError} from './diagnostics.js';
import {readJavaScriptClosedClassOwnerRow} from './javascript-class-source-rules.js';
import {readJavaScriptClassOwnerModel,readJavaScriptClassRecordOwner,readJavaScriptClassOwnerError,acceptJavaScriptClassOwnerObservation} from './javascript-class-owner-rules.js';
function fail(message){throw typeError('JavaScript class ownership: '+message);}
function plain(value,keys){
 if(!value||typeof value!=='object'||Array.isArray(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))fail('an actual owned record is required');
 const descriptors=Object.getOwnPropertyDescriptors(value),names=Reflect.ownKeys(descriptors);
 if(names.length!==keys.length||names.some(key=>typeof key!=='string'||!keys.includes(key)||!descriptors[key].enumerable||!Object.hasOwn(descriptors[key],'value')||descriptors[key].value===undefined))fail('unrepresented class witness fields');
}
function dense(value){if(!Array.isArray(value)||Object.getPrototypeOf(value)!==Array.prototype||value.length>100000)fail('a bounded dense class witness sequence is required');const keys=Reflect.ownKeys(value);if(keys.length!==value.length+1)fail('a dense class witness sequence is required');for(let at=0;at<value.length;at+=1){const descriptor=Object.getOwnPropertyDescriptor(value,String(at));if(!descriptor||!descriptor.enumerable||!Object.hasOwn(descriptor,'value'))fail('unrepresented class witness array property');}}
export function readJavaScriptClassOwnerRecords(value){
 if(value===undefined)return [];
 plain(value,['schemaVersion','language','model','owners']);dense(value.owners);
 if(value.schemaVersion!==1||value.language!=='JavaScript'||value.model!==readJavaScriptClassOwnerModel()||value.owners.length<1||value.owners.length>64)fail('unrecognized owned class witness header');
 for(const owner of value.owners){plain(owner,['constructor','brand','representation','fields','methods']);dense(owner.fields);dense(owner.methods);if(owner.fields.length>8192||owner.methods.length>8192)fail('actual class members exceed their source budget');if([owner.constructor,owner.brand,owner.representation,...owner.fields].some(value=>typeof value!=='string'||value.length===0))fail('class ownership uses actual nonempty source names');for(const method of owner.methods){plain(method,['name','helper']);if(typeof method.name!=='string'||!method.name||typeof method.helper!=='string'||!method.helper)fail('class methods require actual source owners');}}
 return value.owners;
}
export function decodeJavaScriptClassOwnerRows(rows){
 const owners=[];
 for(const row of rows){const plan=readJavaScriptClosedClassOwnerRow(row);if(plan[0]!==0)fail('the source-owned class row is malformed');const methods=[];for(let at=0;at<plan[4];at+=1)methods.push({name:row[plan[3]+2*at],helper:row[plan[3]+2*at+1]});owners.push({constructor:row[0],brand:row[1],representation:row[2],fields:row.slice(plan[1],plan[1]+plan[2]),methods});}
 return {schemaVersion:1,language:'JavaScript',model:readJavaScriptClassOwnerModel(),owners};
}
export function readJavaScriptSourceClassOwner(value,fields){const owners=readJavaScriptClassOwnerRecords(value),owner=readJavaScriptClassRecordOwner(owners.map(owner=>owner.constructor),owners.map(owner=>owner.brand),fields);if(owner==='#ambiguous')fail('a record cannot combine different actual class brands');return owner||undefined;}
export function acceptJavaScriptClassOwnerTypeObservation(type,witness,operation){if(type.kind!=='record'||type.reference!==true||typeof type.sourceClassOwner!=='string')return false;const owner=readJavaScriptClassOwnerRecords(witness).find(owner=>owner.constructor===type.sourceClassOwner);return Boolean(owner)&&acceptJavaScriptClassOwnerObservation(owner.representation,operation==='comparison'?'identity':operation);}
export function checkJavaScriptClassOwners(program){
 if(Object.hasOwn(program,'sourceClassOwners'))readJavaScriptClassOwnerRecords(program.sourceClassOwners);
 const normalized={...program,declarations:[...program.declarations.values()]},facts=createSurfaceReferenceFacts(normalized,{retainValues:true});
 const keys=facts.values.map(value=>value&&typeof value==='object'&&typeof value.kind==='string'?typeKey(value):'');
 const error=readJavaScriptClassOwnerError(facts.forms,facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,keys,facts.root);if(error)fail(error);
}
