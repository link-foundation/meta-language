// Actual source facts and existing AST vocabulary into shared complete plans.
import {rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {acceptRustWrappingShiftMethod,acceptRustWrappingShiftReceiver,readRustWrappingShiftOperation,planRustWrappingShift} from './rust-wrapping-shift-rules.js';
import {acceptRustPrimitiveInherentCopy} from './rust-primitive-inherent-copy-rules.js';
import {acceptRustPrimitiveSelectionReceiver,acceptRustPrimitiveSelectionArgument} from './rust-primitive-selection-rules.js';
import {readRustPrimitiveConversionFacts} from './rust-primitive-try-from.js';
export function readRustWrappingShifts(parser,receiver,method,args,range,path){
 if(!parser.ordinaryCompletionContext()||!acceptRustWrappingShiftMethod(method.value))return null;
 if(args.length!==1)throw new TranslationError('type',method.value+' needs one actual u32 count argument',range);
 if(parser.origins.collecting)return{k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver,...args],span:range};
 const source=readRustPrimitiveConversionFacts(parser,receiver,path),old=source.origin,owner=source.owner?.path.join('::')??'';
 if(acceptRustPrimitiveInherentCopy(method.value,source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,old.proven===true,old.wrappers.length,old.wrappers.every(layer=>layer==='reference'),old.copyEvidence===true))source.origin={...old,wrappers:[],sourceLifetimes:[],copyEvidence:true};
 if(!acceptRustWrappingShiftReceiver(source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,source.origin.proven===true,source.origin.wrappers.length))throw unsupported('Rust wrapping shift','a proven standard fixed integer receiver with its actual source owner is required',range);
 const count=readRustPrimitiveConversionFacts(parser,args[0],path),countOwner=count.owner?.path.join('::')??'',known=acceptRustPrimitiveSelectionReceiver(count.kind,count.bits,count.signed,count.owner?.kind??'',countOwner,count.owner?.arguments.length??-1,count.origin.proven===true,count.origin.wrappers.length),literal=args[0].k==='num'&&!args[0].type||args[0].k==='unary'&&args[0].op==='neg'&&args[0].arg.k==='num'&&!args[0].arg.type;
 if(!acceptRustPrimitiveSelectionArgument('fixed','32',false,'u32',count.kind,count.bits,count.signed,countOwner,count.origin.wrappers.length,count.origin.proven===true,known,Boolean(literal)))throw unsupported('Rust wrapping shift count','the actual count must retain standard u32 ownership',range);
 const countType=rustFixedType('u32'),unsigned={kind:'fixed',bits:source.type.bits,signed:false},captures=[{name:parser.freshProductBinding(),type:source.type,value:receiver},{name:parser.freshProductBinding(),type:countType,value:args[0]}],stack=[];
 const owned=(type,name)=>({...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[name],arguments:[]}}),local=index=>parser.origins.mark({k:'name',path:[captures[index].name],span:captures[index].value.span??range},owned(captures[index].type,index===0?owner:'u32'));
 const pop=()=>{if(!stack.length)throw new TypeError('the wrapping shift source plan lost an earlier term');return stack.pop();};
 for(const term of planRustWrappingShift(method.value)){
  if(term==='operand')stack.push(local(0));
  else if(term==='unsigned operand')stack.push({k:'cast',arg:local(0),to:unsigned,flavor:'wrapping',span:range});
  else if(term==='count')stack.push(local(1));
  else if(term==='width')stack.push({k:'num',value:String(source.type.bits),type:countType,span:range});
  else if(term==='rem'||term==='sub'){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}
  else if(readRustWrappingShiftOperation(term)){const right=pop(),left=pop();stack.push({k:'math',op:readRustWrappingShiftOperation(term),name:'Rust primitive wrapping shift',args:[{value:left,spread:false},{value:right,spread:false}],span:range});}
  else if(term==='source cast')stack.push({k:'cast',arg:pop(),to:source.type,flavor:'wrapping',span:range});
  else throw new TypeError('unknown wrapping shift source term');
 }
 if(stack.length!==1)throw new TypeError('the wrapping shift source plan needs one complete result');let body=stack[0];
 for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};}
 return parser.origins.mark(body,owned(source.type,owner));
}
