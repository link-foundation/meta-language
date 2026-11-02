// Genuine built-in primitive RHS-before-place ordering, transported through
// the existing ordered lexical state effects and ordinary checked arithmetic.
import {unsupported} from './diagnostics.js';
import {readRustOwnedRecordPlaceTokenCount} from './rust-owned-record-place-rules.js';
import {readRustOwnedRecordStatePlace,projectRustOwnedRecordStatePlace} from './rust-owned-record-state.js';
import {readRustOwnedRecordStateFieldIdentity} from './rust-owned-record-state-rules.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustRecordSourceType} from './rust-records.js';
import {recordRustExclusiveSourcePlaceWrite} from './rust-exclusive-source-calls.js';
import {readRustExclusiveSourceCompoundOperation,acceptRustExclusiveSourceCompoundPrimitive} from './rust-exclusive-source-compound-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust exclusive primitive compound assignment',reason,span);};
export function readRustExclusiveSourceCompoundAssignment(parser,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 const c=parser.cursor,tokens=c.tokens.slice(c.index,c.index+1025),star=tokens[0]?.value==='*',forms=tokens.map(token=>readRustExclusiveSourceCompoundOperation(token.value)?'}':['(',')','.'].includes(token.value)?token.value:token.kind==='identifier'?'name':token.kind==='number'&&/^(0|[1-9][0-9]*)$/u.test(token.raw)?'number':'other'),count=star&&tokens[1]?.kind==='identifier'&&readRustExclusiveSourceCompoundOperation(tokens[2]?.value)?2:readRustOwnedRecordPlaceTokenCount(forms);
 if(count<0)return null;const operation=readRustExclusiveSourceCompoundOperation(tokens[count]?.value);if(!operation)return null;
 const used=tokens.slice(0,count),rootToken=star?used[1]:used.find(token=>token.value!=='('),source=parser.origins.scope.get(rootToken?.value),fields=[];
 if(star&&source?.sourceExclusiveMode!=='exclusive')fail('a dereferenced compound place requires its actual exclusive parameter',tokens[0]);
 for(let index=0;index<used.length;index++)if(used[index].value==='.')fields.push(used[++index].kind==='number'?used[index].raw:used[index].value);
 const place=readRustOwnedRecordStatePlace(parser,rootToken?.value,fields,tokens[0]);if(!place)fail('compound assignment needs its actual mutable lexical binder and field path',tokens[0]);
 const position=place.field,owner=position.expandedSourceOwner??position.sourceOwner,type=position.type;
 if(!parser.origins.collecting&&!acceptRustExclusiveSourceCompoundPrimitive(type?.kind??'',String(type?.bits??''),Boolean(type?.signed),owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,position.proven===true,position.wrappers.length))fail('only actual known primitive integer/f64 operands use this built-in source assignment interpretation',tokens[count]);
 for(let index=0;index<=count;index++)c.next();
 const rhs=withRustStandardSumSourceContext(parser,position,()=>readRustExpectedCopyClosure(parser,path,position)),span={start:tokens[0].start,end:rhs.span?.end??c.peek(-1).end};
 if(!parser.origins.collecting){parser.origins.argument(parser.origins.origin(rhs,path),position,rhs.span??span);for(let depth=0;depth<=place.fields.length;depth++)if((parser.origins.sourceBindingStates.get(readRustOwnedRecordStateFieldIdentity(place.slot.root,place.fields.slice(0,depth)))??'available')!=='available')fail('compound assignment reads an initialized place after its actual RHS effects',span);}
 parser.exclusiveSourceReadGuard?.(place.slot,place.fields,span);
 const transport=parser.ownedRecordState?.effectTransport;if(typeof transport?.read!=='function')fail('compound assignment needs its actual ordered state continuation',span);
 const held=parser.freshProductBinding(),rhsName=parser.origins.mark({k:'name',path:[held],span},position),root=transport.read(place.slot,{k:'name',path:[rootToken.value],span}),left=parser.origins.mark(projectRustOwnedRecordStatePlace(parser,place.layouts,root,span),position),binary=parser.origins.mark({k:'binary',op:operation,left,right:rhsName,span},position),value=parser.origins.mark({k:'let',name:held,type:readRustRecordSourceType(parser,type,position.path),value:rhs,body:binary,span},position);
 return recordRustExclusiveSourcePlaceWrite(parser,place,value,path,span);
}
