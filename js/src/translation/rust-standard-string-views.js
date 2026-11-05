// Actual token/source projections. Shared rules own every borrow admission.
import {unsupported,typeError} from './diagnostics.js';
import {STRING} from './types.js';
import {readRustSourceTypeOwner} from './rust-source-signatures.js';
import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
import {acceptRustStandardStringViewPath,acceptRustStandardStringViewLayers,acceptRustStandardStringViewConsumer,acceptRustStandardStringViewReceiver,acceptRustStandardStringViewArgument} from './rust-standard-string-view-rules.js';
const fail=(reason,range)=>{throw unsupported('Rust standard string view',reason,range);};
function viewPath(parser,path,prefix=false){
 const c=parser.cursor;let offset=0;
 if(prefix)while(['&','&&','('].includes(c.peek(offset).value)){offset+=1;if(offset>256)return null;}
 if(c.is('::',offset))offset+=1;
 if(c.peek(offset).value!=='std')return null;
 const segments=[];while(c.peek(offset).kind==='identifier'&&segments.length<16){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;offset+=1;}
 const shadowed=Boolean(parser.origins.lookup(['std'],path)||parser.origins.scope.has('std')||[...parser.aliases.values()].some(aliases=>aliases.has('std')));
 return acceptRustStandardStringViewPath(segments.join('::'),parser.ordinaryCompletionContext(),shadowed)?{segments,offset}:null;
}
function knownResult(origin){
 if(!origin?.proven||origin.wrappers.length)return false;
 const type=origin.type,owner=origin.sourceOwner;
 if(type?.kind==='unit')return !owner||owner.kind==='unit'&&owner.path.length===0&&owner.arguments.length===0;
 if(!owner||owner.arguments.length)return false;
 if(type?.kind==='string')return owner.kind==='library'&&owner.path.join('::')==='std::string::String'&&origin.textSourceModel==='owned string';
 return owner.kind==='primitive'&&owner.path.length===1&&acceptRustSourceSignaturePrimitive(type?.kind??'',owner.path[0],String(type?.bits??''),Boolean(type?.signed));
}
export function beginRustStandardStringViewArgument(parser,path,parameter,result){
 const owner=parameter?.sourceOwner,knownSlice=Boolean(parameter?.proven&&parameter.textSourceModel==='string slice'&&parameter.wrappers.every(layer=>layer==='reference')&&owner?.kind==='primitive'&&owner.path.join('::')==='str'&&owner.arguments.length===0);
 const enabled=acceptRustStandardStringViewConsumer(parameter?.type?.kind??'',parameter?.wrappers.length??0,knownSlice,parameter?.sourceLifetimes?.includes('static')??false,result?.type?.kind??'',result?.wrappers.length??-1,knownResult(result))&&Boolean(viewPath(parser,path,true));
 const context={enabled,layers:parameter?.wrappers.length??0,sites:[]},frame={previous:parser.standardStringBorrowContext,context};parser.standardStringBorrowContext=context;return frame;
}
export function endRustStandardStringViewArgument(parser,frame){parser.standardStringBorrowContext=frame.previous;}
export function checkRustStandardStringViewArgument(parser,frame,value,path){
 const context=frame.context;if(!context.sites.length)return value;
 const origin=parser.origins.origin(value,path),site=context.sites[0],root=Boolean(value.span&&value.span.start===site.start&&value.span.end===site.end&&!value.block);
 if(!acceptRustStandardStringViewArgument(context.sites.length,root,origin.type?.kind??'',origin.wrappers.length,context.layers,origin.wrappers.every(layer=>layer==='reference')))fail('the complete direct borrowed argument must reach its checked consumer',value.span);
 return value;
}
export function readRustStandardStringView(parser,path){
 const selected=viewPath(parser,path);if(!selected)return null;
 const c=parser.cursor,start=c.peek();
 if(!c.is('(',selected.offset))fail('a standard string view function value needs its own callable interpretation',{start:start.start,end:c.peek(selected.offset).end});
 const context=parser.standardStringBorrowContext;
 if(!context?.enabled)fail('the view needs an immediate checked nonstatic string argument consumer',{start:start.start,end:c.peek(selected.offset).end});
 for(let index=0;index<=selected.offset;index+=1)c.next();
 const args=[];while(!c.is(')')){args.push(parser.expr(path));if(!c.eat(','))break;}
 const close=c.expect(')','standard string view'),range={start:start.start,end:close.end};
 if(args.length!==1)throw typeError('a standard string view needs one actual borrowed String argument',range);
 const receiver=args[0],origin=parser.origins.origin(receiver,path),owner=origin.sourceOwner??readRustSourceTypeOwner(parser,['String'],[],STRING);
 const known=owner.kind==='library'&&owner.path.join('::')==='std::string::String'&&owner.arguments.length===0;
 if(!acceptRustStandardStringViewReceiver(origin.type?.kind??'',origin.textSourceModel??'',origin.wrappers.length,origin.proven===true,origin.wrappers.every(layer=>layer==='reference'),known,args.length))fail('the receiver needs its actual standard borrowed String owner',range);
 if(!acceptRustStandardStringViewLayers(context.sites.length+1))fail('the string view argument exceeds its source site budget',range);
 context.sites.push(range);
 return parser.origins.mark({...receiver,span:range},{...origin,type:STRING,path:[...path],textSourceModel:'string slice',wrappers:['reference'],sourceLifetimes:[origin.sourceLifetimes?.[0]??null],sourceOwner:{kind:'primitive',path:['str'],arguments:[]},copyEvidence:false});
}
