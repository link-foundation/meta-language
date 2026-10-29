// Actual lexical tokens and source ownership into shared byte-array decisions.
import {array,rustFixedType} from './types.js';
import {unsupported} from './diagnostics.js';
import {readRustByteStringExpressionBytes,readRustByteStringExpressionError,acceptRustStaticByteArraySliceView} from './rust-byte-string-expression-rules.js';
export function readRustByteStringExpression(parser,token,path){
 const bytes=readRustByteStringExpressionBytes(token.value,token.suffix??''),range={start:token.start,end:token.end};if(bytes[0]!==1)throw unsupported('Rust byte string expression',readRustByteStringExpressionError(bytes[0]),range);
 const element=rustFixedType('u8'),owner={kind:'primitive',path:['u8'],arguments:[]},items=bytes.slice(1).map(byte=>({spread:false,value:parser.origins.mark({k:'num',value:String(byte),type:element,span:range},{...parser.origins.typeOrigin(element,path),sourceOwner:owner})}));
 parser.cursor.next();
 return parser.origins.mark({k:'array',items,element,span:range},{...parser.origins.typeOrigin(array(element),path),wrappers:['reference'],sourceLifetimes:['static'],arraySourceModel:'static byte array',arrayElementCopyEvidence:true,sourceOwner:{kind:'array',path:[String(items.length)],arguments:[owner]}});
}
export function readRustByteStringSliceView(parser,receiver,method,args,range,path){
 const origin=parser.origins.origin(receiver,path);
 if(!acceptRustStaticByteArraySliceView(method.value,args.length,origin.arraySourceModel??'',origin.proven===true,origin.wrappers.length,origin.wrappers.every(layer=>layer==='reference')))return null;
 return parser.origins.mark({...receiver,span:range},{...origin,wrappers:['reference'],sourceLifetimes:['static'],arraySourceModel:'immutable slice',sourceOwner:{kind:'slice',path:[],arguments:[structuredClone(origin.sourceOwner.arguments[0])]}});
}
