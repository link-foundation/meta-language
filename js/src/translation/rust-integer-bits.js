// Actual source facts and generic typed source-term projection only.
import {BOOL,rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {acceptRustIntegerBitMethod,acceptRustIntegerBitReceiver,readRustIntegerBitResultOwner,readRustIntegerBitPlanOperation,planRustIntegerBits} from './rust-integer-bit-rules.js';
import {acceptRustPrimitiveInherentCopy} from './rust-primitive-inherent-copy-rules.js';
import {readRustPrimitiveConversionFacts} from './rust-primitive-try-from.js';
export function readRustIntegerBits(parser,receiver,method,args,range,path){
 if(!parser.ordinaryCompletionContext()||!acceptRustIntegerBitMethod(method.value))return null;
 if(args.length!==0)throw new TranslationError('type',method.value+' needs no actual source arguments',range);
 const source=readRustPrimitiveConversionFacts(parser,receiver,path),old=source.origin,owner=source.owner?.path.join('::')??'';
 if(acceptRustPrimitiveInherentCopy(method.value,source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,old.proven===true,old.wrappers.length,old.wrappers.every(layer=>layer==='reference'),old.copyEvidence===true))source.origin={...old,wrappers:[],sourceLifetimes:[],copyEvidence:true};
 if(!acceptRustIntegerBitReceiver(source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,source.origin.proven===true,source.origin.wrappers.length))throw unsupported('Rust integer bit operation','a proven standard fixed integer receiver with its actual source owner is required',range);
 const unsigned={kind:'fixed',bits:source.type.bits,signed:false},countType=rustFixedType('u32'),plan=planRustIntegerBits(method.value,source.type.bits),values=[],names=[];
 if(!plan.length||plan.length%5||plan.length>16384)throw new TypeError('the shared integer bit source plan exceeds its declared domain');
 const name=slot=>{if(!Number.isSafeInteger(slot)||slot<0||slot>512)throw new TypeError('invalid shared integer bit binding slot');return names[slot]??=parser.freshProductBinding();};
 const read=index=>{if(!Number.isSafeInteger(index)||index<0||index>=values.length)throw new TypeError('the shared integer bit plan lost an earlier term');return values[index];};
 for(let at=0;at<plan.length;at+=5){const operation=readRustIntegerBitPlanOperation(plan[at]),one=plan[at+1],two=plan[at+2],three=plan[at+3];let value;
  if(operation==='unsigned input')value={node:{k:'cast',arg:receiver,to:unsigned,flavor:'wrapping',span:range},type:unsigned};
  else if(['fixed zero','fixed one','fixed two','count zero','count one'].includes(operation)){const type=operation.startsWith('fixed ')?unsigned:countType,magnitude=operation.endsWith('zero')?'0':operation.endsWith('one')?'1':'2';value={node:{k:'num',value:magnitude,type,span:range},type};}
  else if(operation==='false'||operation==='true')value={node:{k:'bool',value:operation==='true',span:range},type:BOOL};
  else if(['fixed reference','count reference','boolean reference'].includes(operation))value={node:{k:'name',path:[name(one)],span:range},type:operation==='fixed reference'?unsigned:operation==='count reference'?countType:BOOL};
  else if(['equal','remainder','division','count addition','fixed addition','fixed multiplication'].includes(operation)){const op={equal:'eq',remainder:'rem',division:'div','count addition':'add','fixed addition':'add','fixed multiplication':'mul'}[operation],type=operation==='equal'?BOOL:operation==='count addition'?countType:unsigned;value={node:{k:'binary',op,left:read(one).node,right:read(two).node,span:range},type};}
  else if(operation==='conditional'){const then=read(two);value={node:{k:'if',cond:read(one).node,then:then.node,else:read(three).node,span:range},type:then.type};}
  else if(operation==='binding'){const item=read(two),body=read(three);value={node:{k:'let',name:name(one),type:item.type,value:item.node,body:body.node,span:range},type:body.type};}
  else if(operation==='source cast')value={node:{k:'cast',arg:read(one).node,to:source.type,flavor:'wrapping',span:range},type:source.type};
  else throw new TypeError('unknown shared integer bit source operation');values.push(value);
 }
 const resultOwner=readRustIntegerBitResultOwner(method.value,owner),resultType=rustFixedType(resultOwner),result=values.at(-1);
 if(result.type.kind!=='fixed'||result.type.bits!==resultType.bits||result.type.signed!==resultType.signed)throw new TypeError('the shared integer bit plan lost its actual result type');
 return parser.origins.mark(result.node,{...parser.origins.typeOrigin(resultType,path),sourceOwner:{kind:'primitive',path:[resultOwner],arguments:[]}});
}
