import {unsupported} from './diagnostics.js';
import {beginRustOwnedRecordStateBlock,endRustOwnedRecordStateBlock,registerRustExclusiveSourceStateRoot} from './rust-owned-record-state.js';
import {createRustExclusiveSourceSignature} from './rust-exclusive-source-signatures.js';
import {lowerRustExclusiveSourceParsedCallable} from './rust-exclusive-source-call-adapter.js';
import {initializeRustOwnedRecordEffects} from './rust-owned-record-effects.js';
import {readRustSourceRegionBodyMode} from './rust-source-region-composition.js';
import {readRustRecordPlaceOwner} from './rust-record-place-owner-rules.js';
export function readRustExclusiveSourceReferenceType(parser,lifetime){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'||!parser.exclusiveParameterType)return null;
 if(lifetime!==null)throw unsupported('Rust exclusive source lifetime','this nonescaping formal profile uses actual elided parameter regions');
 const token=parser.cursor.next();
 if(parser.cursor.is('[')||parser.cursor.is('str'))throw unsupported('Rust exclusive source referent','mutable slices and str need their actual indexed alias and growth protocols',token);
 const type=parser.type(),origin=parser.origins.type;
 if(origin.arrayElementPosition||(!parser.origins.collecting||origin.wrappers.length)&&!readRustRecordPlaceOwner({type:origin.type,owner:origin.expandedSourceOwner??origin.sourceOwner,wrappers:origin.wrappers,lifetimes:origin.sourceLifetimes??[]},false))throw unsupported('Rust exclusive source referent','mutable formals retain actual direct or canonical Box referents without stored references',token);
 parser.origins.type={...origin,wrappers:['reference',...origin.wrappers],sourceLifetimes:[null,...(origin.sourceLifetimes??[])],sourceExclusiveMode:'exclusive'};return type;
}
export function beginRustExclusiveSourceFunction(parser,name,path,parameters,sourceParameters,result,mutableParameters){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 initializeRustOwnedRecordEffects(parser);
 const readonlyRegionComposition=readRustSourceRegionBodyMode(parser,sourceParameters,result)==='readonly',signature=readonlyRegionComposition?null:createRustExclusiveSourceSignature(parser,sourceParameters,result),frame=beginRustOwnedRecordStateBlock(parser,true),slots=[];
 for(let index=0;index<parameters.length;index++)if(sourceParameters[index].sourceExclusiveMode==='exclusive'||mutableParameters.some(row=>row.name===parameters[index].name))slots.push({parameter:index,slot:registerRustExclusiveSourceStateRoot(parser,parameters[index].name,sourceParameters[index].sourceExclusiveMode==='exclusive'?'exclusive':'owned',parameters[index].span)});
 return {frame,name,path:[...path],signature,slots,readonlyRegionComposition};
}
export function finishRustExclusiveSourceFunction(parser,frame,declaration){return frame?lowerRustExclusiveSourceParsedCallable(parser,frame,declaration):declaration;}
export function endRustExclusiveSourceFunction(parser,frame){if(frame)endRustOwnedRecordStateBlock(parser,frame.frame);}
