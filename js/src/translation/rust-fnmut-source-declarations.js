// Actual generic/header/body token origins. This controller never substitutes
// identifiers by spelling inside a body, and never supplies a replacement body.
import {unsupported} from './diagnostics.js';
import {readRustFnMutSourceModelError,readRustFnMutSourceDeclarationBudgetError} from './rust-fnmut-source-declaration-rules.js';
import {testRustFnMutSourceCallbackBinding} from './rust-fnmut-source-callbacks.js';
import {markRustFnMutSourceGenericType,guardRustFnMutSourceBodyTokens} from './rust-fnmut-source-opacity.js';
const inventories=new WeakMap(),handles=new WeakMap();
const fail=(reason,span)=>{throw unsupported('Rust source FnMut specialization',reason,span);};
const punct=(token,value)=>token?.kind==='punct'&&token.value===value;
function inventory(parser){let result=inventories.get(parser);if(!result){result={declarations:new Map(),stack:[],instances:[],next:0};inventories.set(parser,result);}return result;}
function sourceTokens(parser){
 if(typeof parser?.source!=='string'||parser.source.length>16777216||!Array.isArray(parser.sourceConstantTokens)||!parser.origins)fail('the bounded actual original source and retained lexer inventory are required');
 const error=readRustFnMutSourceModelError(parser.fnMutSourceModel??'',parser.sourceVersionMinor);if(error)fail(error);
 return parser.sourceConstantTokens;
}
function close(tokens,from,opening,closing){
 if(!punct(tokens[from],opening))fail('the complete actual source delimiter is unavailable',tokens[from]);let depth=0;
 for(let index=from;index<tokens.length;index++){if(index-from>1000000)fail('the actual source body exceeds its token budget',tokens[from]);if(punct(tokens[index],opening))depth++;else if(punct(tokens[index],closing)&&--depth===0)return index;}
 fail('the actual generic/callback declaration is not completely delimited',tokens[from]);
}
function parameters(tokens,from,to){
 const result=[];let begin=from,round=0,square=0,angle=0;
 for(let at=from;at<=to;at++){
  const token=tokens[at];if(at===to||punct(token,',')&&round===0&&square===0&&angle===0){if(at>begin)result.push({from:begin,to:at});begin=at+1;continue;}
  if(punct(token,'('))round++;else if(punct(token,')'))round--;else if(punct(token,'['))square++;else if(punct(token,']'))square--;else if(punct(token,'<'))angle++;else if(punct(token,'>'))angle--;
  if(round<0||square<0||angle<0)fail('an actual source formal has an inconsistent complete delimiter inventory',token);
 }
 return result;
}
function callback(tokens,formal,index){
 let at=formal.from;const mutable=tokens[at]?.value==='mut';if(mutable)at++;
 const binder=tokens[at];if(binder?.kind!=='identifier'||!punct(tokens[at+1],':'))return null;at+=2;const typeStart=tokens[at];let mode='value';
 if(punct(tokens[at],'&')){if(tokens[at+1]?.value!=='mut')return null;mode='exclusive';at+=2;}
 if(tokens[at]?.value!=='impl'||tokens[at+1]?.value!=='FnMut')return null;at+=2;
 const argumentsFrom=at,argumentsTo=close(tokens,at,'(',')');at=argumentsTo+1;
 if(!punct(tokens[at],'->')||at+1>=formal.to)fail('the actual FnMut bound needs its complete explicit result source position',tokens[at]);
 return {index,name:binder.value,binder:{start:binder.start,end:binder.end},mode,mutable,typeRange:{start:typeStart.start,end:tokens[formal.to-1].end},parameterRanges:parameters(tokens,argumentsFrom+1,argumentsTo).map(row=>({start:tokens[row.from].start,end:tokens[row.to-1].end})),resultRange:{start:tokens[at+1].start,end:tokens[formal.to-1].end}};
}
/** Capture a real declaration at its actual fn token before header admission.
 * The returned handle is authenticated by this parser, not a copied descriptor.
 */
export function captureRustFnMutSourceDeclaration(parser,path,start){
 const tokens=sourceTokens(parser),from=tokens.findIndex(token=>token.start===start.start&&token.value==='fn');if(from<0||tokens[from+1]?.kind!=='identifier'||!Array.isArray(path)||path.some(part=>typeof part!=='string'||!part))fail('the selected source function needs its actual lexical token and owner path',start);
 const name=tokens[from+1].value,generics=[];let at=from+2;
 if(punct(tokens[at],'<')){const end=close(tokens,at,'<','>');for(const row of parameters(tokens,at+1,end)){if(row.to-row.from!==1||tokens[row.from].kind!=='identifier'||generics.some(prior=>prior.name===tokens[row.from].value))fail('this closed specialization requires distinct unconstrained actual type binders',tokens[row.from]);const token=tokens[row.from];generics.push({name:token.value,start:token.start,end:token.end});}at=end+1;}
 const formalEnd=close(tokens,at,'(',')'),formals=parameters(tokens,at+1,formalEnd),callbacks=formals.map((formal,index)=>callback(tokens,formal,index)).filter(Boolean);at=formalEnd+1;
 while(at<tokens.length&&!punct(tokens[at],'{')){if(tokens[at].value==='where'||punct(tokens[at],';'))fail('where predicates and absent generic bodies need their actual separate source interpretation',tokens[at]);at++;}
 const bodyFrom=at,bodyTo=close(tokens,bodyFrom,'{','}'),resultRange=punct(tokens[formalEnd+1],'->')?{start:tokens[formalEnd+2].start,end:tokens[bodyFrom-1].end}:null,error=readRustFnMutSourceDeclarationBudgetError(bodyTo-from+1,formals.length,generics.length,callbacks.length);if(error)fail(error,start);
 for(let index=from;index<=bodyTo;index++){const token=tokens[index];if(!Number.isSafeInteger(token.start)||!Number.isSafeInteger(token.end)||token.start<start.start||token.end>tokens[bodyTo].end||parser.source.slice(token.start,token.end)!==token.raw)fail('each generic body token must retain its real original source spelling and range',token);}
 const range={start:tokens[from].start,end:tokens[bodyTo].end},key=[...path,name].join('.')+'@'+range.start+':'+range.end,arena=inventory(parser),existing=arena.declarations.get(key);if(existing)return existing;
 const handle=Object.freeze({identity:arena.next++}),declaration={parser,name,path:[...path],range,generics,callback:callbacks[0],formalCount:formals.length,resultRange,bodyRange:{start:tokens[bodyFrom].start,end:tokens[bodyTo].end},tokenFrom:from,tokenTo:bodyTo};handles.set(handle,declaration);arena.declarations.set(key,handle);return handle;
}
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
