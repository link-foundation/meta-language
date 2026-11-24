// Actual generic/header/body token origins. This controller never substitutes
// identifiers by spelling inside a body, and never supplies a replacement body.
import {unsupported} from './diagnostics.js';
import {readRustFnMutSourceModelError} from './rust-fnmut-source-declaration-rules.js';
import {readRustFnMutSourceTokenPlan,readRustFnMutSourceTokenError} from './rust-fnmut-source-token-rules.js';
import {testRustFnMutSourceCallbackBinding} from './rust-fnmut-source-callbacks.js';
import {markRustFnMutSourceGenericType,guardRustFnMutSourceBodyTokens} from './rust-fnmut-source-opacity.js';
const inventories=new WeakMap(),handles=new WeakMap();
const fail=(reason,span)=>{throw unsupported('Rust source FnMut specialization',reason,span);};
function inventory(parser){let result=inventories.get(parser);if(!result){result={declarations:new Map(),stack:[],instances:[],next:0};inventories.set(parser,result);}return result;}
function sourceTokens(parser){
 if(typeof parser?.source!=='string'||parser.source.length>16777216||!Array.isArray(parser.sourceConstantTokens)||!parser.origins)fail('the bounded actual original source and retained lexer inventory are required');
 const error=readRustFnMutSourceModelError(parser.fnMutSourceModel??'',parser.sourceVersionMinor);if(error)fail(error);
 return parser.sourceConstantTokens;
}
function capture(parser,path,start,optional){
 const tokens=sourceTokens(parser),arena=inventory(parser);
 if(arena.tokens!==tokens){
  arena.tokens=tokens;
  arena.tokenKinds=tokens.map(token=>token.kind);
  arena.tokenValues=tokens.map(token=>token.value);
  arena.tokenIndices=new Map(tokens.map((token,index)=>[token.start,index]));
 }
 const from=arena.tokenIndices.get(start.start)??-1;
 if(!Array.isArray(path)||path.some(part=>typeof part!=='string'||!part))fail(readRustFnMutSourceTokenError(1),start);
 const plan=readRustFnMutSourceTokenPlan(arena.tokenKinds,arena.tokenValues,from);
 if(plan.length===0){
  if(optional)return null;
  fail(readRustFnMutSourceTokenError(7),start);
 }
 if(plan[0]===-1)fail(readRustFnMutSourceTokenError(plan[1]),tokens[plan[2]]??start);
 const [bodyFrom,bodyTo,resultFrom,resultTo,formalCount,callbackIndex,genericCount]=plan;
 const span=token=>({start:token.start,end:token.end});
 const range=(begin,end)=>({start:tokens[begin].start,end:tokens[end-1].end});
 const generics=plan.slice(7,7+genericCount).map(index=>({name:tokens[index].value,...span(tokens[index])}));
 const [binder,exclusive,mutable,typeFrom,typeTo,callbackResultFrom,callbackResultTo,...parameters]=plan.slice(7+genericCount);
 const parameterRanges=[];
 for(let at=0;at<parameters.length;at+=2)parameterRanges.push(range(parameters[at],parameters[at+1]));
 const callback={index:callbackIndex,name:tokens[binder].value,binder:span(tokens[binder]),
  mode:exclusive?'exclusive':'value',mutable:mutable===1,typeRange:range(typeFrom,typeTo),
  parameterRanges,resultRange:range(callbackResultFrom,callbackResultTo)};
 for(let index=from;index<=bodyTo;index++){
  const token=tokens[index];
  if(!Number.isSafeInteger(token.start)||!Number.isSafeInteger(token.end)||token.start<start.start||
     token.end>tokens[bodyTo].end||parser.source.slice(token.start,token.end)!==token.raw)
   fail('each generic body token must retain its real original source spelling and range',token);
 }
 const name=tokens[from+1].value,sourceRange=range(from,bodyTo+1);
 const key=[...path,name].join('.')+'@'+sourceRange.start+':'+sourceRange.end,existing=arena.declarations.get(key);
 if(existing)return existing;
 const handle=Object.freeze({identity:arena.next++});
 handles.set(handle,{parser,name,path:[...path],range:sourceRange,generics,callback,formalCount,
  resultRange:resultFrom<0?null:range(resultFrom,resultTo),bodyRange:range(bodyFrom,bodyTo+1),tokenFrom:from,tokenTo:bodyTo});
 arena.declarations.set(key,handle);
 return handle;
}
/** Obtain the handle at an actual original fn token, with no stub declaration. */
export function captureRustFnMutSourceDeclaration(parser,path,start){return capture(parser,path,start,false);}
export function tryCaptureRustFnMutSourceDeclaration(parser,path,start){return capture(parser,path,start,true);}
export function readRustFnMutSourceDeclaration(parser,handle){
 const declaration=handles.get(handle);if(!declaration||declaration.parser!==parser)fail('a generic source handle must originate from this actual parser');
 const {name,path,range,generics,callback,formalCount,bodyRange,resultRange}=declaration;return structuredClone({name,path,range,generics,callback,formalCount,bodyRange,resultRange});
}
export function previewRustFnMutSourceDeclaration(parser,handle,typeBindings,readActualHeader){
 const declaration=handles.get(handle);if(!declaration||declaration.parser!==parser||!(typeBindings instanceof Map)||typeof readActualHeader!=='function'||typeBindings.size!==declaration.generics.length||declaration.generics.some(row=>!typeBindings.has(row.name)))fail('a source header preview retains every actual concrete generic type binder');
 const activeIndex=parser.cursor.tokens.findIndex(token=>token.start===declaration.range.start&&token.value==='fn');if(activeIndex<0)fail('the actual function header remains in this source token stream');
 const prior={index:parser.cursor.index,modulePath:parser.modulePath,context:parser.fnMutSourceSpecialization,discovery:parser.headerDiscovery},context={handle,declaration,typeBindings:new Map(typeBindings),phase:'preview',headerConsumed:false,callbackConsumed:false};parser.cursor.index=activeIndex;parser.modulePath=[...declaration.path];parser.headerDiscovery=false;parser.fnMutSourceSpecialization=context;
 try{readActualHeader(parser,context);if(!context.headerConsumed||!context.callbackConsumed||!context.resolvedHeader)fail('each actual generic/callback source header must resolve before argument parsing',declaration.range);return context.resolvedHeader;}finally{parser.cursor.index=prior.index;parser.modulePath=prior.modulePath;parser.headerDiscovery=prior.discovery;parser.fnMutSourceSpecialization=prior.context;}
}
/** Scope concrete type binders around reparsing the original complete body.
 * The normal fn/type/callback readers must mark the actual header and callback
 * consumption; an unintegrated reader fails rather than releasing a body.
 */
export function specializeRustFnMutSourceDeclaration(parser,handle,typeBindings,callbackBinding,readActualDeclaration){
 const declaration=handles.get(handle),arena=inventory(parser);if(!declaration||declaration.parser!==parser||!(typeBindings instanceof Map)||typeof readActualDeclaration!=='function'||!testRustFnMutSourceCallbackBinding(parser,handle,callbackBinding))fail('specialization needs its actual parser declaration, concrete type binders and authenticated callback selection');
 if(typeBindings.size!==declaration.generics.length||declaration.generics.some(binder=>!typeBindings.has(binder.name)))fail('every actual generic type binder needs exactly one selected concrete source origin',declaration.range);
 guardRustFnMutSourceBodyTokens(parser,declaration);
 if(arena.stack.length>=128||arena.instances.length>=65536)fail('actual source specialization exceeds its bounded nesting or declaration budget',declaration.range);
 const activeIndex=parser.cursor.tokens.findIndex(token=>token.start===declaration.range.start&&token.value==='fn');if(activeIndex<0)fail('the original generic function token must remain in the active source stream',declaration.range);
 const prior={cursor:parser.cursor.index,modulePath:parser.modulePath,context:parser.fnMutSourceSpecialization},context={handle,declaration,typeBindings:new Map(typeBindings),callbackBinding,headerConsumed:false,callbackConsumed:false,bodyConsumed:false};arena.stack.push(context);parser.fnMutSourceSpecialization=context;parser.modulePath=[...declaration.path];parser.cursor.index=activeIndex;
 try{
  const result=readActualDeclaration(parser,context);if(!context.headerConsumed||!context.callbackConsumed||!context.bodyConsumed||parser.cursor.peek(-1).end!==declaration.range.end||result?.k!=='fn'||!result.body)fail('the complete original generic declaration/header/callback/body must finish actual admission before specialization is released',declaration.range);
  arena.instances.push(result);return result;
 }finally{arena.stack.pop();parser.cursor.index=prior.cursor;parser.modulePath=prior.modulePath;parser.fnMutSourceSpecialization=prior.context;}
}
/** Only the ordinary type-position reader invokes this operation. A value
 * binder with the same spelling is never rewritten in the actual body. */
export function readRustFnMutSourceConcreteType(parser){
 const context=parser.fnMutSourceSpecialization,token=parser.cursor.peek();if(!context||token.kind!=='identifier'||!context.typeBindings.has(token.value))return null;
 const origin=context.typeBindings.get(token.value);if(!origin?.proven||!origin.type||!origin.sourceOwner||!Array.isArray(origin.wrappers)||origin.wrappers.length)fail('a concrete source generic argument needs its actual complete direct source origin',token);
 parser.cursor.next();parser.origins.type=markRustFnMutSourceGenericType(parser,token,{...structuredClone(origin),expandedSourceOwner:structuredClone(origin.expandedSourceOwner??origin.sourceOwner)});return parser.origins.type.type;
}
