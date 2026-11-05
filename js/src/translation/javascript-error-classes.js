// Actual complete source/CST transport to the shared Error constructor model.
// No represented class/function is evaluated, imported or replaced by a stub.
import {createHash} from 'node:crypto';
import {analyzeProgram} from '../program-representation.js';
import {tokenize} from './lexer.js';
import {findJavaScriptClassGroupEnd} from './javascript-class-source-rules.js';
import {readJavaScriptErrorClassProfileError,readJavaScriptErrorClassDirectory,readJavaScriptErrorClassRefusal} from './javascript-error-class-rules.js';
import {compileJavaScriptErrorModulePlan,compileJavaScriptErrorModulePlanWithBindings} from './javascript-error-class-source.js';
import {checkJavaScriptRuntimeDataModule} from './runtime-data-module-source.js';
import {readJavaScriptModuleInitializationStateError} from './javascript-module-initialization.js';
import {unsupported} from './diagnostics.js';
const units=value=>Array.from({length:value.length},(_,at)=>value.charCodeAt(at));
const constant=value=>({$:'errorConstant',value});
const undefinedExpression=()=>constant({$:'undefined'});
export function captureJavaScriptErrorClassModule(source,{profile='',identifier=''}={}){
 return captureJavaScriptErrorSourceModule(source,{profile,identifier},null);
}
export function captureJavaScriptRuntimeDataFunctions(moduleCapture){
 const actual=checkJavaScriptRuntimeDataModule(moduleCapture);
 return captureJavaScriptErrorSourceModule(actual.source,{profile:'actual intrinsic Error subclasses 1',identifier:actual.identifier},actual);
}
function captureJavaScriptErrorSourceModule(source,{profile='',identifier=''}={},initialization){
 const profileError=readJavaScriptErrorClassProfileError(profile);if(profileError)throw unsupported('Error class source',profileError);
 if(typeof source!=='string'||source.length>131072||typeof identifier!=='string')throw unsupported('Error class source','actual bounded complete source and identity are required');
 const program=analyzeProgram(source,'JavaScript');if(!program.network.verifyFullMatch().isClean())throw unsupported('Error class source','the complete actual source CST must be represented without recovery');
 const tokens=tokenize(source,'JavaScript').tokens,values=tokens.map(token=>['identifier','punct'].includes(token.kind)?token.value:'#'+(token.value??'')),kinds=tokens.map(token=>token.kind);
 const bindingRows=program.bindings.map((binding,identity)=>({...binding,identity})),definitions=[],exports=[],origins=[];let nextBinding=bindingRows.length;
 const moduleConstants=new Set(initialization?.plan.bindings??[]);
 const fail=(message,token)=>{throw unsupported('Error class source',message,token?{start:token.start,end:token.end}:undefined);};
 const declared=token=>bindingRows.find(binding=>binding.declaration.start===token.start&&binding.declaration.end===token.end);
 function reference(token,offset){const start=token.start+offset,end=token.end+offset;return bindingRows.find(binding=>binding.references.some(row=>row.start===start&&row.end===end));}
 // Collect actual top-level declarations first, so forward source calls retain
 // real owners and no function is selected merely by equal spelling.
 const headers=[];
 for(let at=0;at<tokens.length&&tokens[at].kind!=='eof';){
  const ignored=initialization?.ignoredRanges.find(range=>tokens[at].start>=range.start&&tokens[at].end<=range.end);
  if(ignored){while(at<tokens.length&&tokens[at].start<ignored.end)at++;continue;}
  const first=at,exported=tokens[at].value==='export';if(exported)at+=1;
  if(tokens[at].value==='class'){
   const row=readJavaScriptErrorClassDirectory(values,kinds,at);if(row[0]<0)fail(readJavaScriptErrorClassRefusal(row[0]),tokens[at]);
   const name=tokens[at+1],base=tokens[at+3],binding=declared(name);if(!binding||binding.kind!=='class'||binding.scope!=='scope:0')fail('the class needs its actual complete module declaration binding',name);
   if(reference(base,0)!==undefined||!program.unresolvedReferences.some(row=>row.name===base.value&&row.start===base.start&&row.end===base.end))fail('the Error base must be the actual unresolved global intrinsic binding rather than a local/imported shadow',base);
   headers.push({kind:'constructor',binding,first:at,last:row[0],row,base:base.value,exported});at=row[0]+1;
  }else if(tokens[at].value==='function'){
   const name=tokens[at+1],binding=name&&declared(name),open=at+2,close=findJavaScriptClassGroupEnd(values,open),body=close+1,last=findJavaScriptClassGroupEnd(values,body);
   if(!binding||binding.kind!=='function'||binding.scope!=='scope:0'||tokens[open]?.value!=='('||close<0||tokens[body]?.value!=='{'||last<0)fail('ordinary helpers require their actual complete named function declarations',tokens[at]);
   headers.push({kind:'function',binding,first:at,last,open,close,body,exported});at=last+1;
  }else if(tokens[at].value===';'){at+=1;continue;}else fail('module state, imports and other declaration forms need their own complete source closure before Error lowering',tokens[at]);
  if(at<=first)fail('source declaration scanning must make progress',tokens[first]);
 }
 if(headers.length>4096)fail('Error module declaration count exceeds its explicit source profile');
 const moduleBindings=new Map(headers.map(header=>[header.binding.identity,header]));
 function expressionRange(first,last,context,offset=0,expressionTokens=tokens){
  let at=first;
  const peek=()=>expressionTokens[at],take=value=>{if(peek()?.value===value){at+=1;return true;}return false;};
  function node(depth=0){if(depth>128)fail('constructor expressions exceed their complete source depth',peek());let value,token=peek();if(!token||at>=last)fail('an actual complete expression operand is required',token);
   if(take('(')){value=conditional(depth+1);if(!take(')'))fail('parenthesized operands require their actual closing group',peek());}
   else if(take('{')){const properties=[];while(!take('}')){const key=peek();if(!key||!['identifier','string'].includes(key.kind)||key.value==='__proto__')fail('object fields need actual fixed own keys; prototype setters and methods require separate source lowering',key);at+=1;let field;if(take(':'))field=conditional(depth+1);else field=variable(key);properties.push({$:'errorObjectProperty',key:key.stringUnits??units(key.value),value:field});if(take('}'))break;if(!take(','))fail('object fields require complete source separators',peek());}value={$:'errorObject',properties};}
   else if(token.kind==='string'){at+=1;value=constant({$:'text',units:token.stringUnits??units(token.value)});}
   else if(token.kind==='number'){at+=1;const numeric=Number(token.value);if(!Number.isFinite(numeric))fail('this constructor numeric literal needs its explicit nonfinite source definition',token);value=constant({$:'number',value:numeric});}
   else if(token.kind==='template'){at+=1;const parts=[];for(const part of token.parts){parts.push({$:'errorTemplateText',units:part.stringUnits??units(part.text)});if(part.expression){const nested=tokenize(part.expression.source,'JavaScript').tokens;parts.push({$:'errorTemplateValue',value:expressionRange(0,nested.length-1,context,offset+part.expression.offset,nested)});}}value={$:'errorTemplate',parts};}
   else if(take('null'))value=constant({$:'null'});
   else if(take('true'))value=constant({$:'boolean',value:true});
   else if(take('false'))value=constant({$:'boolean',value:false});
   else if(take('this')){if(!context.constructor||!context.afterSuper)fail('this/defaults before the dominant super call need their explicit uninitialized-receiver source path',token);value={$:'errorThis'};}
   else if(take('new')){const name=peek();if(name?.kind!=='identifier')fail('constructor selection requires its actual resolved lexical identifier',name);at+=1;const binding=reference(name,offset),header=binding&&moduleBindings.get(binding.identity);if(!header||header.kind!=='constructor')fail('new requires an actual supplied derived Error class; standard/unknown constructors need their separate value source path',name);if(!take('('))fail('new requires its complete actual argument list',peek());value={$:'errorCall',callee:{$:'errorFunctionReference',identity:binding.identity},receiver:undefinedExpression(),arguments:arguments_(depth+1),construct:true};}
   else if(token.kind==='identifier'){at+=1;value=variable(token);}
   else fail('this expression operation needs its complete source definition',token);
   let member=null;
   while(at<last){if(take('.')){const key=peek();if(key?.kind!=='identifier')fail('property access requires its actual source key',key);at+=1;const property=constant({$:'text',units:units(key.value)});member={object:value,key:property};value={$:'errorGet',object:value,key:property};}else if(take('(')){if(member)value={$:'errorMethodCall',object:member.object,key:member.key,arguments:arguments_(depth+1)};else{if(value.$==='errorFunctionReference'&&moduleBindings.get(value.identity)?.kind==='constructor')fail('ordinary class calls retain their explicit new/source-error event obligation',token);value={$:'errorCall',callee:value,receiver:undefinedExpression(),arguments:arguments_(depth+1),construct:false};}member=null;}else break;}
   return value;
  }
  function variable(token){const binding=reference(token,offset);if(binding){if(context.parameters.has(binding.identity))return {$:'errorBinding',identity:binding.identity};if(moduleConstants.has(binding.identity))return {$:'errorModuleBinding',identity:binding.identity};if(moduleBindings.has(binding.identity))return {$:'errorFunctionReference',identity:binding.identity};fail('a variable/default reference needs its initialized actual lexical owner',token);}if(token.value==='undefined'&&program.unresolvedReferences.some(row=>row.name==='undefined'&&row.start===token.start+offset&&row.end===token.end+offset))return undefinedExpression();fail('an unresolved source reference requires its actual global operation owner',token);}
  function arguments_(depth){const result=[];if(take(')'))return result;while(at<last){if(peek()?.value==='...')fail('spread arguments need their actual iterator source operation',peek());result.push(conditional(depth+1));if(take(')'))return result;if(!take(','))fail('arguments require their complete ordered source operands',peek());}fail('an argument group must retain its actual closing token',peek());}
  function equality(depth){let value=node(depth+1);while(at<last&&['===','!=='].includes(peek()?.value)){const negate=peek().value==='!==';at+=1;value={$:'errorStrictEqual',left:value,right:node(depth+1),negate};}return value;}
  function conditional(depth){const condition=equality(depth+1);if(!take('?'))return condition;const yes=conditional(depth+1);if(!take(':'))fail('a conditional needs both actual branch expressions',peek());return {$:'errorConditional',condition,yes,no:conditional(depth+1)};}
  if(first===last)return undefinedExpression();const result=conditional(0);if(at!==last)fail('unconsumed constructor expression syntax retains its source obligation',peek());return result;
 }
 function parameters(open,close,constructor){const result=[],available=new Set();let at=open+1;while(at<close){const token=tokens[at++],binding=token&&declared(token);if(token?.kind!=='identifier'||!binding||binding.kind!=='parameter')fail('fixed parameters need their actual distinct lexical declaration identities',token);if(result.some(row=>row.identity===binding.identity))fail('parameters must retain distinct actual source owners',token);let value;if(tokens[at]?.value==='='){const first=++at;for(;at<close&&tokens[at].value!==',';at+=1)if(['(','{','['].includes(tokens[at].value)){const end=findJavaScriptClassGroupEnd(values,at);if(end<0||end>=close)fail('parameter defaults need complete actual expression groups',tokens[at]);at=end;}value=expressionRange(first,at,{constructor,afterSuper:false,parameters:available});}result.push(value?{$:'errorDefaultParameter',identity:binding.identity,value}:{$:'errorRequiredParameter',identity:binding.identity});available.add(binding.identity);if(at<close&&tokens[at++].value!==',')fail('fixed parameter lists need actual separators',tokens[at-1]);}return result;}
 for(const header of headers){const constructor=header.kind==='constructor',open=constructor?header.row[1]:header.open,close=constructor?header.row[2]:header.close,params=parameters(open,close,constructor),context={constructor,afterSuper:constructor,parameters:new Set(params.map(row=>row.identity))};
  if(constructor){const message=expressionRange(header.row[3],header.row[4],{...context,afterSuper:false}),fields=[];for(let at=5;at<header.row.length;at+=3)fields.push({$:'errorFieldAssignment',name:units(tokens[header.row[at]].value),value:expressionRange(header.row[at+1],header.row[at+2],context)});definitions.push({$:'errorConstructorPlan',identity:header.binding.identity,name:header.binding.name,baseClass:header.base,parameters:params,message,fields});}
  else{const first=header.body+1,abrupt=tokens[first]?.value==='throw';if(!abrupt&&tokens[first]?.value!=='return')fail('ordinary Error helpers need their complete return or throw expression',tokens[first]);let last=header.last;if(tokens[last-1]?.value===';')last-=1;definitions.push({$:'errorFunctionPlan',identity:header.binding.identity,name:header.binding.name,parameters:params,body:expressionRange(first+1,last,{...context,afterSuper:false}),abrupt});}
  if(header.exported)exports.push(header.binding.identity);origins.push({identity:header.binding.identity,name:header.binding.name,start:tokens[header.first].start,end:tokens[header.last].end,...(constructor?{superStart:tokens[header.row[3]-2].start,superEnd:tokens[header.row[4]].end}:{}),coordinateUnit:'UTF-16 code units'});
 }
 return {schemaVersion:1,artifactKind:'authenticated semantic source plan/input 1',model:'actual intrinsic Error subclasses 1',identifier,source,sourceSha256:createHash('sha256').update(source).digest('hex'),sourceBytes:Buffer.byteLength(source),origins,plan:{$:'errorModulePlan',functions:definitions,exports,nextBinding},...(initialization?{runtimeInitialization:initialization}:{})};
}
export function validateJavaScriptErrorClassModule(capture){
 const actual=capture.runtimeInitialization?captureJavaScriptRuntimeDataFunctions(capture.runtimeInitialization):captureJavaScriptErrorClassModule(capture.source,{profile:capture.model,identifier:capture.identifier});if(JSON.stringify(actual)!==JSON.stringify(capture))throw unsupported('Error class source','the captured constructor plan must retain its complete actual source and lexical owners');
 return actual;
}
export function compileJavaScriptErrorClassModule(capture,registry){
 const actual=validateJavaScriptErrorClassModule(capture);
 if(actual.runtimeInitialization)throw unsupported('runtime data source','the actual module must initialize before its function captures are compiled');
 const compiled=compileJavaScriptErrorModulePlan(actual.plan,registry);if(compiled.error)throw unsupported('Error class source',compiled.error);return {...compiled,capture:actual};
}
export function compileJavaScriptInitializedErrorClassModule(capture,registry,state){
 const actual=validateJavaScriptErrorClassModule(capture);if(!actual.runtimeInitialization)throw unsupported('runtime data source','actual source initialization ownership is required');
 const invalid=readJavaScriptModuleInitializationStateError(actual.runtimeInitialization.plan,state);if(invalid||state.phase!=='evaluated'||registry.owners.some(owner=>owner.realm!==state.realm))throw unsupported('runtime data source',invalid||'source functions require their actual successfully evaluated module and realm');
 const bindings=state.slots.map(slot=>({$:'errorModuleBindingValue',identity:slot.binding,value:slot.value}));
 const compiled=compileJavaScriptErrorModulePlanWithBindings(actual.plan,registry,bindings);if(compiled.error)throw unsupported('runtime data source',compiled.error);return {...compiled,capture:actual};
}
