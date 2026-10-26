// Tokens and existing source facts into the existing owned copy projections.
import {unsupported,typeError} from './diagnostics.js';
import {STRING} from './types.js';
import {readRustSourceTypeOwner} from './rust-source-signatures.js';
import {acceptRustReadOnlyQueryOwner} from './rust-read-only-query-rules.js';
import {acceptRustScalarVectorCopyOwner,acceptRustScalarVectorCopyReceiver} from './rust-scalar-vector-copy-rules.js';
import {createRustScalarVectorCopy} from './rust-scalar-vector-copy.js';
import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
import {readRustStandardOwnedCallRole,acceptRustStandardOwnedTextCall,acceptRustStandardOwnedArrayCall,acceptRustStandardOwnedScalarCall} from './rust-standard-owned-call-rules.js';
export function readRustStandardOwnedCall(parser,path){
 const c=parser.cursor,start=c.peek();let offset=c.is('::')?1:0;
 if(c.peek(offset).value!=='std')return null;
 const segments=[];
 while(c.peek(offset).kind==='identifier'&&segments.length<16){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;offset+=1;}
 const shadowed=Boolean(parser.origins.lookup(['std'],path)||parser.origins.scope.has('std')||[...parser.aliases.values()].some(aliases=>aliases.has('std')));
 const role=readRustStandardOwnedCallRole(segments.join('::'),parser.ordinaryCompletionContext(),shadowed);
 if(!role)return null;
 if(!c.is('(',offset))throw unsupported('Rust standard owned call','a standard trait function value needs its own callable interpretation',{start:start.start,end:c.peek(offset).end});
 for(let index=0;index<=offset;index+=1)c.next();
 const args=[];while(!c.is(')')){args.push(parser.expr(path));if(!c.eat(','))break;}
 const close=c.expect(')','standard owned call'),range={start:start.start,end:close.end};
 if(args.length!==1)throw typeError('a standard owned call needs one actual borrowed argument',range);
 if(parser.origins.collecting)return{k:'app',fn:{k:'name',path:segments,span:range},args,span:range};
 const receiver=args[0],origin=parser.origins.origin(receiver,path);
 const owner=origin.sourceOwner??(origin.type?.kind==='string'?readRustSourceTypeOwner(parser,origin.textSourceModel==='string slice'?['str']:['String'],[],STRING):null);
 const knownText=Boolean(owner&&acceptRustReadOnlyQueryOwner('string empty',owner.kind,owner.path.join('::')));
 if(acceptRustStandardOwnedTextCall(role,origin.type?.kind??'',origin.textSourceModel??'',origin.wrappers.length,origin.proven===true,origin.wrappers.every(layer=>layer==='reference'),knownText)){
  return parser.origins.mark({...receiver,span:range},{...origin,type:STRING,path:[...path],wrappers:[],sourceLifetimes:[],textSourceModel:'owned string',sourceOwner:{kind:'library',path:['std','string','String'],arguments:[]},copyEvidence:false});
 }
 const knownScalar=Boolean(owner&&owner.arguments.length===0&&(owner.kind==='unit'&&owner.path.length===0&&origin.type?.kind==='unit'||owner.kind==='primitive'&&owner.path.length===1&&acceptRustSourceSignaturePrimitive(origin.type?.kind??'',owner.path[0],String(origin.type?.bits??''),Boolean(origin.type?.signed))));
 if(acceptRustStandardOwnedScalarCall(role,origin.type?.kind??'',origin.wrappers.length,origin.proven===true,origin.wrappers.every(layer=>layer==='reference'),knownScalar))return parser.origins.mark({...receiver,span:range},{...origin,path:[...path],wrappers:[],sourceLifetimes:[],copyEvidence:true});
 const knownArray=Boolean(owner&&acceptRustScalarVectorCopyOwner(owner.kind,owner.path.join('::'),owner.path.length,owner.arguments.length));
 const type=origin.type;
 if(acceptRustStandardOwnedArrayCall(role,origin.arraySourceModel??'',origin.wrappers.length,knownArray)&&acceptRustScalarVectorCopyReceiver(type?.kind??'',type?.kind==='array'?type.element.kind:'',origin.proven===true,origin.wrappers.every(layer=>layer==='reference'),origin.arrayElementCopyEvidence===true,0))return createRustScalarVectorCopy(parser,receiver,origin,range,path);
 throw unsupported('Rust standard owned call','the actual borrowed standard scalar, String, str or direct scalar vector receiver is required',range);
}
