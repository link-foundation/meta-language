// Genuine built-in primitive RHS-before-place ordering, transported through
// the existing ordered lexical state effects and ordinary checked arithmetic.
import {unsupported} from './diagnostics.js';
import {noteRustBorrowedExpressionSourcePosition} from './rust-borrowed-expression-positions.js';
import {projectRustOwnedRecordStatePlace} from './rust-owned-record-state.js';
import {readRustExclusiveSourcePlaceTokens,resolveRustExclusiveSourcePlannedPlace,restoreRustExclusiveSourcePlaceValue} from './rust-exclusive-source-places.js';
import {readRustOwnedRecordStateFieldIdentity} from './rust-owned-record-state-rules.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustRecordSourceType} from './rust-records.js';
import {recordRustExclusiveSourcePlaceWrite} from './rust-exclusive-source-calls.js';
import {readRustExclusiveSourceCompoundOperation,acceptRustExclusiveSourceCompoundPrimitive} from './rust-exclusive-source-compound-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust exclusive primitive compound assignment',reason,span);};
export function readRustExclusiveSourceCompoundAssignment(parser,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 const c=parser.cursor,syntax=readRustExclusiveSourcePlaceTokens(parser,'compound');if(!syntax)return null;
 const {tokens}=syntax,selected=resolveRustExclusiveSourcePlannedPlace(parser,syntax);if(!selected)fail('compound assignment needs its actual mutable lexical binder and field path',tokens[0]);
 const {count,rootToken,place,position}=selected,operation=readRustExclusiveSourceCompoundOperation(tokens[count]?.value);
 const owner=position.expandedSourceOwner??position.sourceOwner,type=position.type;
 if(!parser.origins.collecting&&!acceptRustExclusiveSourceCompoundPrimitive(type?.kind??'',String(type?.bits??''),Boolean(type?.signed),owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,position.proven===true,position.wrappers.length))fail('only actual known primitive integer/f64 operands use this built-in source assignment interpretation',tokens[count]);
 for(let index=0;index<=count;index++)c.next();
 const rhs=withRustStandardSumSourceContext(parser,position,()=>readRustExpectedCopyClosure(parser,path,position)),span={start:tokens[0].start,end:rhs.span?.end??c.peek(-1).end};
 if(!parser.origins.collecting){parser.origins.argument(parser.origins.origin(rhs,path),position,rhs.span??span);for(let depth=0;depth<=place.fields.length;depth++)if((parser.origins.sourceBindingStates.get(readRustOwnedRecordStateFieldIdentity(place.slot.root,place.fields.slice(0,depth)))??'available')!=='available')fail('compound assignment reads an initialized place after its actual RHS effects',span);}
 parser.exclusiveSourceReadGuard?.(place.slot,place.fields,span);
 const transport=parser.ownedRecordState?.effectTransport;if(typeof transport?.read!=='function')fail('compound assignment needs its actual ordered state continuation',span);
 const held=parser.freshProductBinding(),rhsName=parser.origins.mark({k:'name',path:[held],span},position),root=transport.read(place.slot,{k:'name',path:[rootToken.value],span}),left=parser.origins.mark(projectRustOwnedRecordStatePlace(parser,place.layouts,root,span),position),binary=parser.origins.mark({k:'binary',op:operation,left,right:rhsName,span},position);
 noteRustBorrowedExpressionSourcePosition(parser,left,path);noteRustBorrowedExpressionSourcePosition(parser,binary,path);
 const value=parser.origins.mark({k:'let',name:held,type:readRustRecordSourceType(parser,type,position.path),value:rhs,body:binary,span},position);
 noteRustBorrowedExpressionSourcePosition(parser,value,path);
 return recordRustExclusiveSourcePlaceWrite(parser,place,restoreRustExclusiveSourcePlaceValue(parser,selected,value,path,span),path,span);
}
