// Actual type/origin facts and generic existing source AST projection only.
import {rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustIntegerMagnitudeArity,acceptRustIntegerMagnitudeReceiver,readRustIntegerMagnitudeResultOwner,planRustIntegerMagnitude} from './rust-integer-magnitude-rules.js';
import {acceptRustPrimitiveInherentCopy} from './rust-primitive-inherent-copy-rules.js';
import {acceptRustPrimitiveSelectionReceiver,acceptRustPrimitiveSelectionArgument} from './rust-primitive-selection-rules.js';
import {readRustPrimitiveConversionFacts} from './rust-primitive-try-from.js';
import {readRustSaturatingIntegerBoundMagnitude} from './rust-saturating-integer-rules.js';
export function readRustIntegerMagnitudes(parser,receiver,method,args,range,path){
 const arity=readRustIntegerMagnitudeArity(method.value,parser.ordinaryCompletionContext());if(arity<0)return null;
 if(args.length!==arity)throw new TranslationError('type',method.value+' needs '+arity+' actual source arguments',range);
 const source=readRustPrimitiveConversionFacts(parser,receiver,path),old=source.origin,owner=source.owner?.path.join('::')??'';
 if(acceptRustPrimitiveInherentCopy(method.value,source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,old.proven===true,old.wrappers.length,old.wrappers.every(layer=>layer==='reference'),old.copyEvidence===true))source.origin={...old,wrappers:[],sourceLifetimes:[],copyEvidence:true};
 if(!acceptRustIntegerMagnitudeReceiver(method.value,source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,source.origin.proven===true,source.origin.wrappers.length))throw unsupported('Rust integer magnitude','a proven standard integer receiver with the actual signed method is required',range);
 if(arity>0){const right=readRustPrimitiveConversionFacts(parser,args[0],path),actual=right.owner?.path.join('::')??'',known=acceptRustPrimitiveSelectionReceiver(right.kind,right.bits,right.signed,right.owner?.kind??'',actual,right.owner?.arguments.length??-1,right.origin.proven===true,right.origin.wrappers.length),literal=args[0].k==='num'&&!args[0].type||args[0].k==='unary'&&args[0].op==='neg'&&args[0].arg.k==='num'&&!args[0].arg.type;if(!acceptRustPrimitiveSelectionArgument(source.kind,source.bits,source.signed,owner,right.kind,right.bits,right.signed,actual,right.origin.wrappers.length,right.origin.proven===true,known,Boolean(literal)))throw unsupported('Rust integer magnitude argument','the actual argument must retain the receiver type and source owner',range);}
 const resultOwner=readRustIntegerMagnitudeResultOwner(owner),unsigned=rustFixedType(resultOwner),captures=[receiver,...args].map(value=>({name:parser.freshProductBinding(),type:source.type,value})),stack=[];
 const owned=(type,name)=>({...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[name],arguments:[]}}),local=index=>parser.origins.mark({k:'name',path:[captures[index].name],span:captures[index].value.span??range},owned(source.type,owner)),cast=value=>({k:'cast',arg:value,to:unsigned,flavor:'wrapping',span:range});
 const pop=()=>{if(!stack.length)throw new TypeError('the integer magnitude source plan lost an earlier term');return stack.pop();};
 for(const term of planRustIntegerMagnitude(method.value)){
  if(term==='operand'||term==='right')stack.push(local(term==='operand'?0:1));
  else if(term==='unsigned operand'||term==='unsigned right')stack.push(cast(local(term==='unsigned operand'?0:1)));
  else if(term==='unsigned cast')stack.push(cast(pop()));
  else if(term==='zero'||term==='positive unit'||term==='unsigned unit'||term==='unsigned maximum')stack.push({k:'num',value:term==='zero'?'0':term==='unsigned maximum'?readRustSaturatingIntegerBoundMagnitude('maximum',unsigned.bits,false):'1',type:term.startsWith('unsigned ')?unsigned:source.type,span:range});
  else if(['lt','ge','add','sub'].includes(term)){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}
  else if(term==='neg')stack.push({k:'unary',op:'neg',arg:pop(),span:range});
  else if(term==='if'){const otherwise=pop(),then=pop(),cond=pop();stack.push({k:'if',cond,then,else:otherwise,span:range});}
  else throw new TypeError('unknown integer magnitude source term');
 }
 if(stack.length!==1)throw new TypeError('the integer magnitude source plan needs one complete result');let body=stack[0];for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};}
 return parser.origins.mark(body,owned(unsigned,resultOwner));
}
