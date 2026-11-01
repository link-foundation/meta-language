// Capture complete providers without evaluating any represented declaration.
// The dependency closure is selected by CST binding edges, never helper names.
import {createHash} from 'node:crypto';
import {analyzeProgram} from '../program-representation.js';
import {tokenize} from './lexer.js';
import {findJavaScriptClassGroupEnd} from './javascript-class-source-rules.js';
import {assertPlainSourceReceipt} from './source-receipt-data.js';
const model='actual project data source functions 1',classModel='actual closed source classes 1';
const units=text=>Array.from({length:text.length},(_,at)=>text.charCodeAt(at));
const constant=value=>({$:'errorConstant',value});
const absent=()=>constant({$:'undefined'});

export function captureJavaScriptProjectDataSource(source,{identifier,entries=['readProjectData','resolveProjectDataPath'],profile=model}={}){
 const classProfile=profile===classModel;
 if((profile!==model&&!classProfile)||typeof source!=='string'||Buffer.byteLength(source)>131072||typeof identifier!=='string'||!identifier||!Array.isArray(entries)||entries.length===0||entries.length>32||entries.some(name=>typeof name!=='string')||new Set(entries).size!==entries.length)throw new TypeError('source functions/classes require their explicit bounded source profile and entry names');
 const program=analyzeProgram(source,'JavaScript');if(!program.network.verifyFullMatch().isClean())throw new TypeError('project data source requires its complete actual CST without recovery');
 const tokens=tokenize(source,'JavaScript').tokens.filter(token=>token.kind!=='eof'),values=tokens.map(token=>token.value),bindings=program.bindings.map((binding,identity)=>({...binding,identity}));let nextBinding=bindings.length,expressionDepth=0,statementDepth=0;
 if(tokens.length>16384||bindings.length>4096)throw new TypeError('project data source exceeds its explicit token or binding budget');
 const fail=message=>{throw new TypeError(message);};
 const declared=token=>bindings.find(binding=>binding.declaration.start===token?.start&&binding.declaration.end===token?.end);
 const reference=(token,offset=0)=>bindings.find(binding=>binding.references.some(row=>row.start===token.start+offset&&row.end===token.end+offset));
 const unresolved=(token,offset=0)=>program.unresolvedReferences.some(row=>row.name===token.value&&row.start===token.start+offset&&row.end===token.end+offset);
 const containing=(binding,terms)=>program.sourceMappings.filter(row=>terms.includes(row.term)&&row.start<=binding.declaration.start&&row.end>=binding.declaration.end).sort((a,b)=>(a.end-a.start)-(b.end-b.start))[0];
 const headers=[],imports=[],cells=[],covered=[],classes=[];
 for(const binding of bindings.filter(row=>row.scope==='scope:0')){
  if(binding.kind==='function'){
   const node=containing(binding,['function_declaration']);if(!node)fail('source functions require their actual complete declaration');
   const first=tokens.findIndex(token=>token.start===node.start),open=first+2,close=findJavaScriptClassGroupEnd(values,open),body=close+1,last=findJavaScriptClassGroupEnd(values,body);
   if(first<0||tokens[first]?.value!=='function'||tokens[open]?.value!=='('||close<0||tokens[body]?.value!=='{'||last<0||tokens[last].end!==node.end)fail('only complete ordinary fixed named function declarations belong to this source profile');
   headers.push({binding,node,open,close,body,last});covered.push(node);
  }else if(binding.kind==='class'&&classProfile){
   const node=containing(binding,['class_declaration']),first=node&&tokens.findIndex(token=>token.start===node.start),body=first+2,last=findJavaScriptClassGroupEnd(values,body);
   if(!node||first<0||values[first]!=='class'||values[body]!=='{'||last<0||tokens[last].end!==node.end)fail('the closed source class profile requires its complete ordinary base declaration; inheritance needs its own source semantics');
   const methods=[];let constructor=null;
   for(let at=body+1;at<last;){
    if(values[at]===';'){at++;continue;}
    const name=tokens[at],open=at+1,close=findJavaScriptClassGroupEnd(values,open),start=close+1,end=findJavaScriptClassGroupEnd(values,start);
    if(name?.kind!=='identifier'||['__proto__','prototype'].includes(name.value)||values[open]!=='('||close<0||values[start]!=='{'||end<0||end>=last)fail('class members require complete fixed ordinary instance methods; fields, accessors and computed/static members retain their separate source obligations');
    if(methods.some(row=>row.binding.name===name.value)||name.value==='constructor'&&constructor!==null)fail('duplicate source class members require their complete installation-order model');
    const member={binding:{identity:name.value==='constructor'?binding.identity:nextBinding++,name:name.value},node:{start:name.start,end:tokens[end].end},open,close,body:start,last:end,classBinding:binding.identity,isConstructor:name.value==='constructor'};
    if(member.isConstructor)constructor=member;else methods.push(member);at=end+1;
   }
   const header={binding,node,open:constructor?.open??-1,close:constructor?.close??-1,body:constructor?.body??body,last:constructor?.last??body+1,classBinding:binding.identity,isConstructor:true,implicitConstructor:constructor===null,methods};
   headers.push(header);classes.push({binding:binding.identity,name:binding.name,start:node.start,end:node.end,constructorStart:constructor?.node.start??null,methods:methods.map(row=>({identity:row.binding.identity,name:row.binding.name,start:row.node.start,end:row.node.end}))});covered.push(node);
  }else if(binding.kind==='import'){
   const node=containing(binding,['import_statement']);if(!node)fail('source imports need actual declaration ownership');
   const row=tokens.filter(token=>token.start>=node.start&&token.end<=node.end),from=row.findIndex(token=>token.value==='from');
   if(row[0]?.value!=='import'||row[1]?.value!=='{'||from<0||row[from-1]?.value!=='}'||row[from+1]?.kind!=='string'||row[from+2]?.value!==';'||from+3!==row.length)fail('only actual named source imports belong to this provider profile');
   const at=row.findIndex(token=>token.start===binding.declaration.start);if(at<2)fail('an import needs its actual local binding token');
   const imported=row[at-1]?.value==='as'?row[at-2]?.value:row[at]?.value;
   imports.push({binding:binding.identity,localName:binding.name,imported,specifier:row[from+1].value,start:node.start,end:node.end});covered.push(node);
  }else if(binding.kind==='let'){
   const node=containing(binding,['lexical_declaration']),row=node&&tokens.filter(token=>token.start>=node.start&&token.end<=node.end);
   if(!row||row.length!==3||row[0].value!=='let'||row[1].start!==binding.declaration.start||row[2].value!==';')fail('mutable module cells currently require their actual uninitialized single let declaration');
   cells.push({binding:binding.identity,name:binding.name,start:node.start,end:node.end,initialValue:{$:'undefined'}});covered.push(node);
  }else fail('this provider declaration kind needs its own source initialization path');
 }
 for(const token of tokens){if(covered.some(row=>token.start>=row.start&&token.end<=row.end)||token.value===';')continue;if(token.value==='export'&&headers.some(row=>row.node.start===tokens[tokens.indexOf(token)+1]?.start))continue;fail('provider source cannot omit an actual top-level effect or export form');}
 const selected=new Set(),queue=entries.map(name=>{const header=headers.find(row=>row.binding.name===name);if(!header||!program.sourceMappings.some(row=>row.term==='export_statement'&&row.start<=header.node.start&&row.end>=header.node.end))fail('every selected entry must be its actual exported source function');return header.binding.identity;});
 while(queue.length){const identity=queue.shift();if(selected.has(identity))continue;selected.add(identity);const header=headers.find(row=>row.binding.identity===identity);for(const candidate of headers)if(candidate.binding.references.some(row=>row.start>=header.node.start&&row.end<=header.node.end))queue.push(candidate.binding.identity);}
 const capabilities=new Map();
 function port(key,details){if(!capabilities.has(key))capabilities.set(key,{source:key,...details});return {$:'errorNativeReference',source:key};}
 const cellIDs=new Set(cells.map(row=>row.binding)),functionIDs=new Set([...selected]),importByID=new Map(imports.map(row=>[row.binding,row]));
 const classBinding=binding=>binding&&classes.find(row=>{const owner=bindings[row.binding];return selected.has(row.binding)&&(row.binding===binding.identity||owner&&owner.declaration.start===binding.declaration.start&&owner.declaration.end===binding.declaration.end);});
 let activeClass=null,initializingParameters=null;
 function expression(input,offset=0){let at=0;const peek=()=>input[at],take=value=>{if(peek()?.value===value){at++;return true;}return false;};
  function variable(token){const binding=reference(token,offset);if(binding){if(initializingParameters!==null&&binding.kind==='parameter'&&!initializingParameters.has(binding.identity))fail('parameter defaults require actual earlier initialized bindings; later/self parameter TDZ needs its explicit source continuation');const actualClass=classBinding(binding);if(actualClass)return {$:'errorFunctionReference',identity:actualClass.binding};if(cellIDs.has(binding.identity))return {$:'errorModuleCell',identity:binding.identity};if(functionIDs.has(binding.identity))return {$:'errorFunctionReference',identity:binding.identity};if(importByID.has(binding.identity)){const owner=importByID.get(binding.identity);return port('binding:'+binding.identity,{kind:'import',binding:binding.identity,specifier:owner.specifier,imported:owner.imported});}if(binding.scope!=='scope:0')return {$:'errorBinding',identity:binding.identity};fail('a selected source function refers to an unmodeled module binding');}if(token.value==='undefined'&&unresolved(token,offset))return absent();if(classProfile&&unresolved(token,offset)&&['NaN','Infinity'].includes(token.value))return {$:'errorSpecialNumber',form:token.value};if(!unresolved(token,offset)||!['URL','Buffer','JSON','structuredClone','TypeError','Error',...(classProfile?['Set','Map','Array']:[])].includes(token.value))fail('source globals need their actual unshadowed bounded operation owner');return port('global:'+token.value,{kind:'global',name:token.value});}
  function arguments_(end){const result=[];if(take(end))return result;while(at<input.length){result.push(parse(0));if(take(end))return result;if(!take(','))fail('source arguments require complete separators');}fail('source arguments require their closing group');}
  function primary(){const token=peek();if(!token)fail('source expressions require a complete operand');let value,member=null;
   if(['!','typeof','-'].includes(token.value)){at++;return {$:'errorUnary',operator:token.value,value:parse(9)};}
   if(take('(')){value=parse(0);if(!take(')'))fail('source parentheses must close');}
   else if(take('[')){
    if(classProfile){let length=0;const elements=[];if(!take(']'))while(at<input.length){if(take(']'))break;if(take(',')){length++;continue;}if(peek()?.value==='...')fail('Array literal spread requires its actual iterator and close continuations');elements.push({$:'errorArrayElement',index:length++,value:parse(0)});if(take(']'))break;if(!take(','))fail('Array literal elements need complete separators');if(take(']'))break;}if(input[at-1]?.value!==']')fail('Array literal needs its closing bracket');value={$:'errorArrayLiteral',length,elements};}
    else{port('syntax:array',{kind:'syntax',operation:'array literal'});value={$:'errorArray',factory:'syntax:array',items:arguments_(']')};}
   }
   else if(take('{')){const properties=[];if(!take('}'))while(at<input.length){const key=peek();if(!key||!['identifier','string'].includes(key.kind)||key.value==='__proto__')fail('source object fields require fixed own data keys');at++;const field=take(':')?parse(0):variable(key);properties.push({$:'errorObjectProperty',key:key.stringUnits??units(key.value),value:field});if(take('}'))break;if(!take(','))fail('source object fields need complete separators');}value={$:'errorObject',properties};}
   else if(take('this')){if(activeClass===null)fail('this requires its actual captured class constructor or method owner');value={$:'errorReceiver'};}
   else if(take('new')){const name=peek(),owner=name&&reference(name,offset),classOwner=classBinding(owner);
    if(name?.kind!=='identifier')fail('construction needs its actual fixed lexical callee');at++;if(!take('('))fail('source construction needs complete arguments');
    if(classOwner)value={$:'errorOrdinaryConstruction',identity:classOwner.binding,arguments:arguments_(')')};
    else{if(owner||!unresolved(name,offset)||!['URL','Error','TypeError',...(classProfile?['Set','Map']:[])].includes(name.value))fail('native construction needs its actual unshadowed source capability');const callee=port('construct:'+name.value,{kind:'construct',name:name.value});value=classProfile&&['Set','Map'].includes(name.value)?{$:'errorCollectionConstruction',source:'construct:'+name.value,arguments:arguments_(')')}:{$:'errorCall',callee,receiver:absent(),arguments:arguments_(')'),construct:false};}
   }
   else if(token.kind==='number'){at++;const value_=Number(token.value);if(!Number.isFinite(value_))fail('source numeric literals need a finite actual Number');value=constant({$:'number',value:value_});}
   else if(token.kind==='string'){at++;value=constant({$:'text',units:token.stringUnits??units(token.value)});}
   else if(token.kind==='template'){at++;const parts=[];for(const part of token.parts){parts.push({$:'errorTemplateText',units:part.stringUnits??units(part.text)});if(part.expression)parts.push({$:'errorTemplateValue',value:expression(tokenize(part.expression.source,'JavaScript').tokens.filter(row=>row.kind!=='eof'),offset+part.expression.offset)});}value={$:'errorTemplate',parts};}
   else if(take('null'))value=constant({$:'null'});else if(take('true'))value=constant({$:'boolean',value:true});else if(take('false'))value=constant({$:'boolean',value:false});
   else if(token.kind==='identifier'){at++;value=variable(token);}else fail('this source operand requires an additional actual semantic operation');
   while(at<input.length){if(peek()?.value==='.'||peek()?.value==='?.'){const optional=take('?.');if(!optional)take('.');const key=peek();if(key?.kind!=='identifier')fail('source member reads require a complete fixed key');at++;const property=constant({$:'text',units:units(key.value)});member=optional?null:{object:value,key:property};value={$:optional?'errorOptionalGet':'errorGet',object:value,key:property};}else if(classProfile&&take('[')){const key=parse(0);if(!take(']'))fail('computed source member access needs its complete key');member={object:value,key,computed:true};value={$:'errorComputedGet',object:value,key};}else if(take('(')){const args=arguments_(')');value=member?{$:member.computed?'errorComputedMethodCall':'errorMethodCall',object:member.object,key:member.key,arguments:args}:{$:'errorCall',callee:value,receiver:absent(),arguments:args,construct:false};member=null;}else break;}return value;
  }
  const levels={'=':1,'+=':1,'?':2,'||':3,'&&':4,'===':5,'!==':5,'<':6,'<=':6,'>':6,'>=':6,'instanceof':6,'+':7,'-':7,'*':8,'/':8,'%':8};
  function parse(minimum){if(++expressionDepth>128)fail('project data expressions exceed their explicit source depth');let left=primary();while(at<input.length){const operator=peek().value,level=levels[operator];if(level===undefined||level<minimum)break;at++;
    if(operator==='?'){const yes=parse(0);if(!take(':'))fail('source conditional needs both branches');left={$:'errorConditional',condition:left,yes,no:parse(level)};continue;}
    let right=parse(operator==='='||operator==='+='?level:level+1);if(operator==='='||operator==='+='){
     if(operator==='+='){if(!classProfile||left.$!=='errorBinding')fail('compound assignment currently requires its actual class-local Number binding');right={$:'errorBinary',operator:'+',left,right,port:''};}
     if(left.$==='errorModuleCell')left={$:'errorModuleCellSet',identity:left.identity,value:right};else if(left.$==='errorGet'||left.$==='errorComputedGet')left={$:left.$==='errorComputedGet'?'errorComputedSet':'errorSet',object:left.object,key:left.key,value:right};
     else if(classProfile&&left.$==='errorBinding'&&['parameter','let'].includes(bindings.find(row=>row.identity===left.identity)?.kind))left={$:'errorBindingSet',identity:left.identity,value:right};else fail('source assignment requires its actual admitted mutable cell or property');
    }
    else if(operator==='==='||operator==='!==')left={$:'errorStrictEqual',left,right,negate:operator==='!=='};
    else {let key='';if(operator==='instanceof'){key='syntax:instanceof';port(key,{kind:'syntax',operation:'instanceof'});}left={$:'errorBinary',operator,left,right,port:key};}
   }expressionDepth--;return left;}
  const result=parse(0);if(at!==input.length)fail('source expression cannot discard unconsumed syntax');return result;
 }
 function split(first,last,separator){const result=[];let start=first;for(let at=first;at<last;at++){if(['(','{','['].includes(values[at])){const end=findJavaScriptClassGroupEnd(values,at);if(end<0||end>=last)fail('source groups must remain complete');at=end;}else if(values[at]===separator){result.push([start,at]);start=at+1;}}if(start<last)result.push([start,last]);return result;}
 function parameters(header){if(header.implicitConstructor)return [];initializingParameters=new Set();const result=split(header.open+1,header.close,',').map(([first,last])=>{
  const token=tokens[first],binding=declared(token);if(token.value==='{'){
   const close=findJavaScriptClassGroupEnd(values,first),fields=[];if(close<0||close>=last)fail('source object parameter must retain its complete binding pattern');
   const hasDefault=close+1<last;if(hasDefault&&values[close+1]!=='=')fail('object parameter default needs its actual source expression');const value=hasDefault?expression(tokens.slice(close+2,last)):absent();
   for(const [a,b]of split(first+1,close,',')){const key=tokens[a],owner=declared(key);if(!owner||owner.kind!=='parameter'||(b>a+1&&values[a+1]!=='='))fail('source object parameters currently require actual shorthand fields and optional defaults');fields.push({$:'errorParameterField',identity:owner.identity,key:units(key.value),hasDefault:b>a+1,value:b>a+1?expression(tokens.slice(a+2,b)):absent()});initializingParameters.add(owner.identity);}
   return {$:'errorObjectParameter',identity:nextBinding++,fields,hasDefault,value};
  }
  if(!binding||binding.kind!=='parameter'||(last>first+1&&values[first+1]!=='='))fail('source parameters need actual fixed binding identities');const parameter=last===first+1?{$:'errorRequiredParameter',identity:binding.identity}:{$:'errorDefaultParameter',identity:binding.identity,value:expression(tokens.slice(first+2,last))};initializingParameters.add(binding.identity);return parameter;
 });initializingParameters=null;return result;}
 function statements(first,last){if(++statementDepth>128)fail('project data statements exceed their explicit source depth');const result=[];let at=first;
  function one(){if(values[at]==='{'){const end=findJavaScriptClassGroupEnd(values,at);if(end<0||end>last)fail('source blocks require complete groups');const body=statements(at+1,end);at=end+1;return body;}return [statement()];}
  function statement(){const word=values[at++];if(word==='if'){const open=at,close=findJavaScriptClassGroupEnd(values,open);if(values[open]!=='('||close<0)fail('source if requires its complete condition');const condition=expression(tokens.slice(open+1,close));at=close+1;const yes=one(),no=values[at]==='else'?(at++,one()):[];return {$:'sourceIfStatement',condition,yes,no};}
   if((word==='return'||word==='throw')&&tokens[at]&&/[\r\n\u2028\u2029]/u.test(source.slice(tokens[at-1].end,tokens[at].start))){if(word==='throw')fail('throw requires its actual same-line source operand');return {$:'sourceReturnStatement',value:absent()};}
   if(word==='for'&&classProfile){const open=at,close=findJavaScriptClassGroupEnd(values,open),parts=close>=0?split(open+1,close,';'):[];
    if(values[open]!=='('||parts.length!==3||values[parts[0][0]]!=='let'||values[parts[0][0]+2]!=='=')fail('source class loops require their actual fixed let/condition/update operands');
    const owner=declared(tokens[parts[0][0]+1]);if(!owner||owner.kind!=='let')fail('source loop initialization requires its actual mutable lexical owner');
    const initial=expression(tokens.slice(parts[0][0]+3,parts[0][1])),condition=expression(tokens.slice(...parts[1])),update=expression(tokens.slice(...parts[2]));at=close+1;
    return {$:'sourceBlockStatement',statements:[{$:'sourceMutableStatement',identity:owner.identity,value:initial},{$:'sourceWhileStatement',condition,body:[...one(),{$:'sourceExpressionStatement',value:update}]}]};
   }
   if(word==='{'){at--;return {$:'sourceBlockStatement',statements:one()};}
   const begin=at-1;let end=at;for(;end<last&&values[end]!==';';end++)if(['(','{','['].includes(values[end])){const close=findJavaScriptClassGroupEnd(values,end);if(close<0||close>=last)fail('source statement groups must close');end=close;}if(end>=last)fail('this source statement profile requires explicit actual semicolons');at=end+1;
   if(word==='const'||word==='let'&&classProfile){const owner=declared(tokens[begin+1]);if(!owner||owner.kind!==word||values[begin+2]!=='=')fail('source local declarations need one actual ordinary binding');return {$:word==='let'?'sourceMutableStatement':'sourceConstStatement',identity:owner.identity,value:expression(tokens.slice(begin+3,end))};}
   if(word==='return'||word==='throw')return {$:word==='return'?'sourceReturnStatement':'sourceThrowStatement',value:end===begin+1?absent():expression(tokens.slice(begin+1,end))};
   return {$:'sourceExpressionStatement',value:expression(tokens.slice(begin,end))};
  }
  while(at<last){if(values[at]===';'){at++;continue;}result.push(statement());}statementDepth--;return result;
 }
 const functions=[],origins=[];for(const header of headers.filter(row=>selected.has(row.binding.identity))){
  for(const member of [header,...(header.methods??[])]){
   activeClass=member.classBinding??null;const params=parameters(member),body=member.implicitConstructor?[]:statements(member.body+1,member.last);
   functions.push({$:member.isConstructor?'errorOrdinaryConstructorPlan':member.classBinding!==undefined?'errorMethodFunctionPlan':classProfile?'errorMutableStatementFunctionPlan':'errorStatementFunctionPlan',identity:member.binding.identity,name:member.binding.name,parameters:params,statements:body});origins.push({identity:member.binding.identity,name:member.binding.name,start:member.node.start,end:member.node.end,coordinateUnit:'UTF-16 code units',...(member.classBinding!==undefined?{classBinding:member.classBinding,constructor:member.isConstructor}: {})});
  }
 }activeClass=null;
 return {schemaVersion:1,artifactKind:'authenticated semantic source plan/input 1',model:profile,identifier,source,sourceBytes:Buffer.byteLength(source),sourceSha256:createHash('sha256').update(source).digest('hex'),entries:[...entries],origins,imports:imports.filter(row=>capabilities.has('binding:'+row.binding)),cells,capabilities:[...capabilities.values()],...(classProfile?{classes:classes.filter(row=>selected.has(row.binding))}:{}),qualification:'owned selected source functions/classes; imported operations require their actual qualified implementations; not a checked portable Core program',plan:{$:'errorModulePlan',functions,exports:entries.map(name=>headers.find(row=>row.binding.name===name).binding.identity),nextBinding}};
}

export function checkJavaScriptProjectDataSource(capture){assertPlainSourceReceipt(capture);const actual=captureJavaScriptProjectDataSource(capture.source,{identifier:capture.identifier,entries:capture.entries,profile:capture.model});if(JSON.stringify(actual)!==JSON.stringify(capture))throw new TypeError('project data source plan must retain its complete CST, selected binding closure and operation owners');return actual;}
