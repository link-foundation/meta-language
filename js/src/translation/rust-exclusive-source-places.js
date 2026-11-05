// Source token selection and actual mutable storage admission share one place
// resolver. Inspecting a type/place never performs its eventual read or write.
import {unsupported} from './diagnostics.js';
import {readRustExclusiveSourcePlacePlan,readRustExclusiveSourceBorrowPlacePlan,readRustExclusiveSourceCompoundOperation} from './rust-exclusive-source-compound-rules.js';
import {readRustOwnedRecordStatePlace} from './rust-owned-record-state.js';
import {readRustOwnedBoxPlaceProjection} from './rust-owned-containers.js';
import {noteRustBorrowedExpressionSourcePosition} from './rust-borrowed-expression-positions.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readRustRecordPlaceBoxProjectionDepth} from './rust-record-place-owner-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust exclusive source place',reason,span);};
export function readRustExclusiveSourcePlaceTokens(parser,operation){
 const tokens=parser.cursor.tokens.slice(parser.cursor.index,parser.cursor.index+1025),forms=tokens.map(token=>(operation==='compound'?Boolean(readRustExclusiveSourceCompoundOperation(token.value)):operation!=='borrow'&&token.value===operation)?'}':['(',')','.','*',','].includes(token.value)?token.value:token.kind==='identifier'?'name':token.kind==='number'&&/^(0|[1-9][0-9]*)$/u.test(token.raw)?'number':'other'),plan=operation==='borrow'?readRustExclusiveSourceBorrowPlacePlan(forms):readRustExclusiveSourcePlacePlan(forms);
 return plan.length>=2?{tokens,plan}:null;
}
export function resolveRustExclusiveSourcePlannedPlace(parser,{tokens,plan}){
 const rootToken=tokens[plan[1]],source=parser.origins.scope.get(rootToken.value),fields=[];
 let place=readRustOwnedRecordStatePlace(parser,rootToken.value,fields,tokens[0]);if(!place)return null;
 let position=place.field,exclusiveReference=source?.sourceExclusiveMode==='exclusive'&&source.wrappers[0]==='reference';
 for(const step of plan.slice(2)){
  if(step===-1){if(exclusiveReference){exclusiveReference=false;continue;}const payload=readRustOwnedBoxPlaceProjection(parser.origins,position,1);if(!payload)fail('each dereference retains its actual exclusive reference or canonical owning Box layer',tokens[0]);position=payload;}
  else{exclusiveReference=false;fields.push(tokens[step].kind==='number'?tokens[step].raw:tokens[step].value);place=readRustOwnedRecordStatePlace(parser,rootToken.value,fields,tokens[0]);position=place.field;}
 }
 if(exclusiveReference)position=source;
 return {place,position,rootToken,count:plan[0],exclusiveReference,payloadLayers:exclusiveReference?0:place.field.wrappers.length-position.wrappers.length};
}
export function restoreRustExclusiveSourcePlaceValue(parser,selected,value,path,span){
 if(selected.payloadLayers===0)return value;
 if(selected.payloadLayers<0||selected.exclusiveReference)fail('a payload replacement retains its actual complete original Box field',span);
 const table=parser.origins,name=parser.freshProductBinding(),body=table.mark({k:'name',path:[name],span},selected.position),origin={...selected.place.field,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceExclusiveParameter:false,sourceStandardSumPatternMove:false,sourceStandardSumBorrowedField:false,sourceNominalPatternMove:false,sourceNominalBorrowedField:false,sourceOwnedResultProof:true};
 noteRustBorrowedExpressionSourcePosition(parser,body,path);
 const restored=table.mark({k:'let',name,type:readRustRecordSourceType(parser,selected.position.type,selected.position.path),value,body,span},origin);
 noteRustBorrowedExpressionSourcePosition(parser,restored,path);return restored;
}
// Only the selected mutable call formal supplies a coercion target. Ordinary
// value transfers and assignment places never gain this call-site projection.
export function selectRustExclusiveSourceCallReferent(parser,place,parameter,span){
 if(!parameter)return place;
 const source=place.referent??place.field,reference=parameter.wrappers[0]==='reference'?1:0,target={...parameter,wrappers:parameter.wrappers.slice(reference),sourceLifetimes:(parameter.sourceLifetimes??[]).slice(reference)},position=origin=>({owner:origin.expandedSourceOwner??origin.sourceOwner,wrappers:origin.wrappers,lifetimes:origin.sourceLifetimes??[],arrayElementPosition:origin.arrayElementPosition}),depth=readRustRecordPlaceBoxProjectionDepth(position(source),position(target));
 if(depth<0&&!parser.origins.collecting&&(place.payloadLayers>0||source.wrappers.includes('box')))fail('a Box call projection preserves the actual selected formal owner and every remaining canonical layer',span);
 if(depth<=0)return place;
 const referent=readRustOwnedBoxPlaceProjection(parser.origins,source,depth);if(!referent)fail('a selected mutable formal retains each actual canonical Box coercion layer',span);
 return {...place,referent,payloadLayers:(place.payloadLayers??0)+depth};
}
export function requireRustExclusiveSourceReceiverRoot(parser,place,receiver){
 const tokens=parser.sourceConstantTokens,span=receiver.span;if(!Array.isArray(tokens)||tokens.length>1000000||!span)fail('a mutable receiver retains its bounded actual source token inventory and span',span);
 let first=0,last=tokens.length;while(first<last){const middle=Math.floor((first+last)/2);if(tokens[middle].start<span.start)first=middle+1;else last=middle;}
 let steps=0;while(first<tokens.length&&tokens[first].start<span.end&&['(','*'].includes(tokens[first].value)){if(++steps>256)fail('a mutable receiver exceeds its bounded original place syntax',span);first++;}
 const token=tokens[first],source=token?.kind==='identifier'?parser.origins.scope.get(token.value):null;
 if(!source||token.start>=span.end||token.value!==place.slot.name||source.sourceLexicalBindingIdentity&&source.sourceLexicalBindingIdentity!==place.slot.lexical||source.wrappers[0]==='reference'&&source.sourceExclusiveMode!=='exclusive')fail('a mutable receiver selects its actual mutable lexical root, never a shared alias into that root',span);
 return place;
}
