// Owned constructor/field projection. The existing source-signature validator
// supplies all recursive type, wrapper, owner and lifetime interpretations.
import {UNIT} from './types.js';
import {typeError} from './diagnostics.js';
import {createRustSourceSignature,checkRustSourceSignature} from './rust-source-signatures.js';
import {checkOwnedProgramContainers} from './owned-program-containers.js';
import {acceptRustDataTypeFieldSignature,acceptRustConstructorFieldSignature,acceptRustDataTypeFieldSignatureBudget,acceptRustDataTypeFieldSignatureContainer,acceptRustDataTypeFieldSignatureProgramContainer} from './rust-data-type-field-signature-rules.js';
const model='Rust data type field signatures 1',typeModel='Rust source function signature 2';
const unitOwner={kind:'unit',path:[],arguments:[]};
const unitPosition={type:UNIT,wrappers:[],lifetimes:[],owner:unitOwner};
const fail=reason=>{throw typeError('Rust data type field signatures: '+reason);};
const exact=(value,fields)=>{if(!value||typeof value!=='object'||Array.isArray(value)||Object.keys(value).length!==fields.length||fields.some(field=>!Object.hasOwn(value,field)))fail('an owned field-signature record has missing or unrepresented fields');};
export function checkRustDataTypeFieldSignatureContainers(value){return checkOwnedProgramContainers(value,{selectedFields:['sourceFieldSignatures'],acceptContainer:acceptRustDataTypeFieldSignatureContainer,fail});}
export function checkRustDataTypeFieldSignatureProgramContainers(value){return checkOwnedProgramContainers(value,{selectedFields:['sourceFieldSignatures'],acceptContainer:acceptRustDataTypeFieldSignatureProgramContainer,fail});}
/** Capture every original position while its actual source origin is live. */
export function retainRustDataTypeFieldSignatures(parser,declaration,path,groups){
 if(!parser.retainSourceTypeOwners)return declaration;
 const unit={type:UNIT,wrappers:[],sourceLifetimes:[],sourceOwner:unitOwner};
 declaration.sourceFieldSignatures={model,language:'Rust',dataTypeName:[...path,declaration.name].join('.'),ctors:groups.map((origins,index)=>({name:declaration.ctors[index].name,fields:createRustSourceSignature(typeModel,origins,unit,[]).parameters}))};
 return declaration;
}
/** Validate complete actual constructor identities and every resolved field. */
export function checkRustDataTypeFieldSignatures(signature,constructors,language,dataTypeName,resolveType=value=>value){
 checkRustDataTypeFieldSignatureContainers({sourceFieldSignatures:signature});
 let visits=0;const active=new WeakSet();
 const inspect=(value,depth)=>{visits+=1;if(!acceptRustDataTypeFieldSignatureBudget(visits,depth))fail('the complete field-signature tree exceeds its resource limit');if(value&&typeof value==='object'){if(active.has(value))fail('owned field-signature records cannot contain a structural cycle');active.add(value);for(const child of Object.values(value))inspect(child,depth+1);active.delete(value);}};inspect(signature,0);
 exact(signature,['model','language','dataTypeName','ctors']);
 if(!Array.isArray(signature.ctors)||!acceptRustDataTypeFieldSignature(signature.model,language,signature.dataTypeName,dataTypeName,signature.ctors.length,constructors.length)||signature.language!==language)fail('the model, source language or actual datatype/constructor inventory differs');
 const ctors=signature.ctors.map((ctor,index)=>{
  exact(ctor,['name','fields']);const actual=constructors[index];
  if(!Array.isArray(ctor.fields)||!acceptRustConstructorFieldSignature(ctor.name,actual.name,ctor.fields.length,actual.fields.length))fail('the actual constructor name or complete field inventory differs');
  const checked=checkRustSourceSignature({model:typeModel,language,parameters:ctor.fields,result:unitPosition,lifetimeBindings:[]},actual.fields.map(field=>field.type),UNIT,language,resolveType);
  return{name:ctor.name,fields:checked.parameters};
 });
 return{...signature,ctors};
}
