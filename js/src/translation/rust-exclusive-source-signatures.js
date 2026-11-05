// The exclusive source interface is distinct from immutable sourceSignature.
import {createRustSourceSignature,checkRustSourceSignature} from './rust-source-signatures.js';
import {readRustSourceVersion} from './rust-source-versions.js';
import {readRustExclusiveSourceLoanVersionError,rustExclusiveSourceLoanProfile} from './rust-exclusive-source-loan-rules.js';
import {typeError,unsupported} from './diagnostics.js';
import {typeKey} from './types.js';
import {checkRustStandardCollectionCarrier} from './rust-standard-collection-signatures.js';
import {readRustRecordPlaceOwner} from './rust-record-place-owner-rules.js';
export function requireRustExclusiveSourceModel(model,version){
 if(model==='')return;
 const facts=readRustSourceVersion(version),error=readRustExclusiveSourceLoanVersionError(facts.major,facts.minor);
 if(model!==rustExclusiveSourceLoanProfile||error)throw unsupported('Rust exclusive source model',error||'select Rust exclusive source loans 1');
}
export function createRustExclusiveSourceSignature(parser,parameters,result){
 if(parameters.length>256)throw unsupported('Rust exclusive source signature','the actual formal list exceeds the bounded source carrier profile');
 const mode=origin=>origin.sourceExclusiveMode==='exclusive'?'exclusive':origin.wrappers[0]==='reference'?'shared':'value';
 const referent=origin=>{
  const owner=origin.expandedSourceOwner??origin.sourceOwner,text=mode(origin)==='shared'&&origin.wrappers.length===1&&owner?.kind==='primitive'&&owner.path.join('::')==='str';if(text)return origin;
  const borrowed=mode(origin)!=='value',wrappers=origin.wrappers.slice(borrowed?1:0),lifetimes=(origin.sourceLifetimes??[]).slice(borrowed?1:0);
  if(origin.arrayElementPosition||borrowed&&origin.wrappers[0]!=='reference'||!readRustRecordPlaceOwner({type:origin.type,owner,wrappers,lifetimes},false))throw unsupported('Rust exclusive source position','nonescaping call referents retain actual direct or canonical owned Box positions without stored references');
  return {...origin,wrappers,sourceLifetimes:lifetimes};
 };
 if(result.wrappers.includes('reference'))throw unsupported('Rust exclusive source result','returned references need their actual escape and lifetime interpretation');
 const positions=createRustSourceSignature('Rust source function signature 2',parameters.map(referent),referent(result),new Set());
 return {model:rustExclusiveSourceLoanProfile,language:'Rust',sourceVersion:parser.exclusiveSourceVersion,parameters:positions.parameters.map((position,index)=>({mode:mode(parameters[index]),position})),result:positions.result,packet:parameters.flatMap((origin,index)=>mode(origin)==='exclusive'?[index]:[])};
}
export function checkRustExclusiveSourceSignature(signature,parameters,result,language,resolveType=value=>value,resolveData=()=>null){
 const exact=(value,keys)=>value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===keys.length&&keys.every(key=>Object.hasOwn(value,key));
 if(!exact(signature,['model','language','sourceVersion','parameters','result','packet'])||signature.model!==rustExclusiveSourceLoanProfile||signature.language!=='Rust'||language!=='Rust')throw typeError('an exclusive Rust source signature requires its actual distinct model and fields');
 const version=readRustSourceVersion(signature.sourceVersion),versionError=readRustExclusiveSourceLoanVersionError(version.major,version.minor);if(versionError)throw typeError(versionError);
 if(typeof signature.sourceVersion!=='string'||signature.sourceVersion!==version.spelling)throw typeError('an exclusive Rust source signature retains its explicit canonical selected source version');
 if(!Array.isArray(signature.parameters)||signature.parameters.length!==parameters.length||signature.parameters.length>256||!Array.isArray(signature.packet))throw typeError('an exclusive Rust source signature retains its complete bounded formal inventory');
 const packet=[];for(let index=0;index<signature.parameters.length;index++){const formal=signature.parameters[index];if(!exact(formal,['mode','position'])||!['value','shared','exclusive'].includes(formal.mode))throw typeError('exclusive source formals retain actual value, shared or exclusive modes');if(formal.mode==='exclusive')packet.push(index);}
 if(packet.length!==signature.packet.length||packet.some((value,index)=>value!==signature.packet[index]))throw typeError('exclusive source packet slots retain exactly the actual exclusive formal order');
 if(packet.length&&(result.kind!=='tuple'||result.elements.length!==packet.length+1))throw typeError('an exclusive function returns the actual result and every updated referent');
 const originalResult=packet.length?result.elements[0]:result;
 const positional=checkRustSourceSignature({model:'Rust source function signature 2',language:'Rust',parameters:signature.parameters.map(row=>row.position),result:signature.result,lifetimeBindings:[]},parameters,originalResult,language,resolveType,resolveData);
 let work=0;
 function storage(position,active=[]){
  if(++work>65536||active.length>128||!position||!Array.isArray(position.wrappers)||!Array.isArray(position.lifetimes)||position.lifetimes.length||position.arrayElementPosition)throw typeError('exclusive source storage requires bounded owned referent positions without stored references');
  const peeled=readRustRecordPlaceOwner(position,false);if(!peeled)throw typeError('exclusive Box storage retains every actual canonical owner and wrapper');if(peeled.boxed){storage(peeled.position,active);return;}
  const type=position.type,owner=position.owner;
  if(type.kind==='array'){if(owner.kind!=='library'||owner.path.join('::')!=='std::vec::Vec')throw typeError('exclusive vector storage retains its actual standard owner');const child=owner.arguments[0],wrappers=[];let leaf=child;while(leaf?.kind==='library'&&['std::boxed::Box','alloc::boxed::Box'].includes(leaf.path.join('::'))){if(wrappers.length>=128||leaf.arguments.length!==1)throw typeError('vector Box storage retains its bounded complete element owner');wrappers.push('box');leaf=leaf.arguments[0];}storage({type:type.element,owner:child,wrappers,lifetimes:[]},active);return;}
  if(type.kind==='data'){
   const data=resolveData(type),carrier=data&&checkRustStandardCollectionCarrier(data,language,resolveType,name=>resolveData({kind:'data',name}));if(carrier)return;
   const fields=data?.sourceFieldSignatures,sum=owner.kind==='library'&&['std::option::Option','std::result::Result'].includes(owner.path.join('::'));
   if(!data||!fields||fields.ctors.length!==data.ctors.length||!sum&&data.ctors.length!==1||active.includes(type.name))throw typeError('exclusive nominal storage needs its actual complete acyclic record or standard sum declaration');
   for(const constructor of fields.ctors)for(const field of constructor.fields)storage(field,[...active,type.name]);return;
  }
  if(!['unit','bool','fixed','int','nat','float','string'].includes(type.kind)||owner.kind==='library'&&owner.path.join('::')!=='std::string::String')throw typeError('exclusive source storage needs closed scalar, String, record or Vec values');
 }
 for(let index=0;index<positional.parameters.length;index++){const position=positional.parameters[index],text=signature.parameters[index].mode==='shared'&&position.owner.kind==='primitive'&&position.owner.path.join('::')==='str'&&position.wrappers.length===1&&position.wrappers[0]==='reference'&&position.lifetimes.length===1&&position.lifetimes[0]===null;if(!text)storage(position);}
 storage(positional.result);
 if(packet.length)for(let index=0;index<packet.length;index++)if(typeKey(result.elements[index+1])!==typeKey(parameters[packet[index]]))throw typeError('exclusive packet writeback type differs from its actual formal referent');
 return {...signature,parameters:signature.parameters.map((row,index)=>({...row,position:positional.parameters[index]})),result:positional.result};
}
