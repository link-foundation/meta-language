import {projectRustIntegerFloatPlan} from './rust-integer-float-plan.js';
// Actual source facts and existing capture/conditional/cast AST transport only.
import {unsupported} from './diagnostics.js';
import {rustFixedType,BOOL,FLOAT} from './types.js';
import {readRustPrimitiveFromLiteralOwner,acceptRustPrimitiveFromSource,planRustPrimitiveFrom} from './rust-primitive-from-rules.js';
export function projectRustPrimitiveFrom(parser,receiver,target,range,path){
 const negative=receiver.k==='unary'&&receiver.op==='neg'&&receiver.arg.k==='num',number=negative?receiver.arg:receiver;
 const literalOwner=readRustPrimitiveFromLiteralOwner(negative?'negative numeral':receiver.k==='num'?'numeral':'',Boolean(number.type),number.type?.kind??'');
 if(literalOwner){const type=literalOwner==='f64'?FLOAT:rustFixedType(literalOwner),literal={...number,type};receiver=negative?{...receiver,arg:literal}:literal;parser.origins.mark(receiver,{...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[literalOwner],arguments:[]}});}
 const origin=parser.origins.origin(receiver,path),type=origin.type,owner=origin.sourceOwner??(receiver.k==='bool'?{kind:'primitive',path:['bool'],arguments:[]}:receiver.k==='num'&&type?.kind==='float'?{kind:'primitive',path:['f64'],arguments:[]}:null);
 if(!acceptRustPrimitiveFromSource(type?.kind??'',String(type?.bits??''),type?.signed===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.proven===true,origin.wrappers.length))throw unsupported('Rust primitive From','the actual argument must retain a proven direct standard primitive source type',range);
 const plan=planRustPrimitiveFrom(target,owner.path.join('::'));
 if(!plan)throw unsupported('Rust primitive From','the selected standard primitive From implementation must exist for the actual source owner',range);
 const to=target==='bool'?BOOL:target==='f64'?FLOAT:rustFixedType(target),name=parser.freshProductBinding(),local=()=>parser.origins.mark({k:'name',path:[name],span:receiver.span??range},origin);
 const body=plan==='floating exact'?projectRustIntegerFloatPlan(parser,local(),type,range):plan==='identity'?local():plan==='boolean integer'?{k:'if',cond:local(),then:{k:'num',value:'1',type:to,span:range},else:{k:'num',value:'0',type:to,span:range},span:range}:{k:'cast',arg:local(),to,flavor:'exact',span:range};
 const result={k:'let',name,type,value:receiver,body,span:range};
 return parser.origins.mark(result,{...parser.origins.typeOrigin(to,path),sourceOwner:{kind:'primitive',path:[target],arguments:[]}});
}
