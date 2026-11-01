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
 let sourceFields=null,sourceSyntax=null;
 function sourceField(node,label){
  if(sourceFields===null){const fields=program.network.fieldRelations();if(fields.length>100000||program.sourceMappings.length>100000)fail('source field witnesses exceed their explicit graph budget');sourceFields=new Map();sourceSyntax=new Map(program.sourceMappings.map(row=>[row.linkId,row]));for(const field of fields){const key=field.parent.asU64()+':'+field.label,child=field.child.asU64();const prior=sourceFields.get(key);if(prior!==undefined&&prior!==child&&['value','left','argument','parameter','parameters'].includes(field.label))fail('a source operand must retain its unique actual CST field');sourceFields.set(key,child);}}
  return sourceSyntax.get(sourceFields.get(node.linkId+':'+label));
 }
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
 let activeClass=null,activeFunction=null,initializingParameters=null,initializingParameterOwners=null,controlTargets=[];
 const arrowNodes=[...new Map(program.sourceMappings.filter(row=>row.term==='arrow_function').map(row=>[row.start+':'+row.end,row])).values()].sort((a,b)=>a.start-b.start||b.end-a.end),templates=[];
 const methodNodes=[...new Map(program.sourceMappings.filter(row=>row.term==='method_definition').map(row=>[row.start+':'+row.end,row])).values()].sort((a,b)=>a.start-b.start||b.end-a.end);
 if(arrowNodes.length+methodNodes.length>4096)fail('source lexical templates exceed their explicit site budget');
 // A shared lexical cell is private source storage, using the same null-
 // prototype value holder as module cells. Its identity, not its mutable value,
 // is captured. Loop-head cloning and TDZ initialization are separate profiles.
 const sharedBindingCells=new Set();
 if(classProfile)for(const binding of bindings){
  if(!['let','parameter'].includes(binding.kind)||binding.scope==='scope:0')continue;
  if(binding.kind==='let'){
   const declaration=containing(binding,['variable_declarator']);if(!declaration||declaration.start!==binding.declaration.start)continue;
   const loop=containing(binding,['for_statement','for_in_statement','for_of_statement']);if(loop){const body=sourceField(loop,'body');if(!body||binding.declaration.start<body.start)continue;}
  }else{
   const position=tokens.findIndex(token=>token.start===binding.declaration.start&&token.end===binding.declaration.end),formal=containing(binding,['formal_parameters']),singleArrow=arrowNodes.some(node=>node.start===binding.declaration.start&&values[position+1]==='=>');
   if(position<0||!singleArrow&&(!formal||!['(',',','...'].includes(values[position-1])||![')',',','='].includes(values[position+1])))continue;
  }
  if([...arrowNodes,...methodNodes].some(node=>!(binding.declaration.start>=node.start&&binding.declaration.end<=node.end)&&binding.references.some(row=>row.start>=node.start&&row.end<=node.end))){sharedBindingCells.add(binding.identity);cellIDs.add(binding.identity);}
 }
 function inferArrowName(value,name){if(value.$==='errorClosure'){const template=templates.find(row=>row.owner===value.owner&&row.ordinal===value.ordinal);if(template&&template.name==='')template.name=name;}return value;}
 let writeCoordinates=null,writeRanges=[],writeComparisonWork=0;
 function capturedBindingWrites(){
  if(writeCoordinates!==null)return writeCoordinates;
  if(program.sourceMappings.length>100000)fail('capture write proofs exceed their actual syntax budget');
  const syntax=new Map(program.sourceMappings.map(row=>[row.linkId,row])),fields=program.network.fieldRelations();if(fields.length>100000)fail('capture write proofs exceed their actual field budget');
  let work=0;const operands=new Map(fields.map(row=>[row.parent.asU64()+':'+row.label,row.child.asU64()]));writeCoordinates=new Set();
  for(const assignment of program.sourceMappings.filter(row=>['assignment_expression','augmented_assignment_expression','update_expression'].includes(row.term))){
   const label=assignment.term==='update_expression'?'argument':'left';let target=syntax.get(operands.get(assignment.linkId+':'+label)),depth=0;
   if(!target)fail('capture writes require the actual CST assignment operand field');
   while(target.term==='parenthesized_expression'){
    if(++depth>128)fail('capture write targets exceed their actual parenthesis depth');
    let child=null;
    for(const candidate of program.sourceMappings){if(++work>1000000)fail('capture write proof exceeds its bounded syntax traversal work');if(candidate.start<=target.start||candidate.end>=target.end)continue;const metadata=program.network.link(candidate.linkId).metadata();if(!metadata.named||metadata.flags.isExtra)continue;if(child===null||candidate.end-candidate.start>child.end-child.start)child=candidate;}
    if(child===null)fail('parenthesized capture writes require their actual expression child');target=child;
   }
   if(['member_expression','subscript_expression'].includes(target.term))continue;
   if(target.term!=='identifier'){writeRanges.push([target.start,target.end]);continue;}
   writeCoordinates.add(target.start+':'+target.end);
  }
  return writeCoordinates;
 }
 function readonlyCapture(binding,node){
  if(!['parameter','const','let'].includes(binding.kind)||binding.declaration.end>node.start)fail('arrow capture requires its actual already initialized lexical source binding; later bindings need shared cells');
  const declaration=binding.kind==='parameter'?null:containing(binding,['variable_declarator']);if(declaration&&declaration.end>node.start)fail('capturing a binding during its initializer requires its shared lexical cell');
  if(sharedBindingCells.has(binding.identity))return;
  const writes=capturedBindingWrites();for(const occurrence of binding.references)if(writes.has(occurrence.start+':'+occurrence.end)||writeRanges.some(([start,end])=>{if(++writeComparisonWork>1000000)fail('capture write ranges exceed their bounded comparison work');return occurrence.start>=start&&occurrence.end<=end;}))fail('mutable captured bindings require their actual shared lexical cells');
 }
 function objectMethod(input,first,keyAt,kind,offset,computedKeyUnits=null){
  if(!classProfile||offset!==0||activeFunction===null)fail('object methods require an actual closed source function owner');
  const node=methodNodes.find(row=>row.start===input[first]?.start),openToken=input[keyAt+1];
  if(!node||openToken?.value!=='(')fail('object methods retain their actual CST site and complete parameter head');
  const open=tokens.findIndex(token=>token.start===openToken.start),close=findJavaScriptClassGroupEnd(values,open),body=close+1,last=findJavaScriptClassGroupEnd(values,body);
  if(open<0||close<open||values[body]!=='{'||last<body||tokens[last].end!==node.end)fail('object methods require a complete synchronous ordinary body; async/generator/super semantics are separate');
  const captures=[];for(const binding of bindings){if(binding.scope==='scope:0'||binding.declaration.start>=node.start&&binding.declaration.end<=node.end)continue;if(!binding.references.some(row=>row.start>=node.start&&row.end<=node.end))continue;readonlyCapture(binding,node);captures.push(binding.identity);}
  if(captures.length>4096)fail('method captures exceed their actual immutable value budget');
  const previous=initializingParameters,previousOwners=initializingParameterOwners,previousTargets=controlTargets;
  if(previous!==null&&captures.some(identity=>previousOwners?.has(identity)&&!previous.has(identity)))fail('capturing an uninitialized parameter requires its shared lexical cell');
  initializingParameters=null;initializingParameterOwners=null;controlTargets=[];let parameters,statements_;
  try{parameters=readSourceParameters({open,close});statements_=statements(body+1,last);}finally{initializingParameters=previous;initializingParameterOwners=previousOwners;controlTargets=previousTargets;}
  if(kind==='get'&&parameters.length!==0||kind==='set'&&(parameters.length!==1||parameters[0].$==='errorRestParameter'))fail('literal accessors retain their required zero/one non-rest formal parameters');
  const key=input[keyAt],keyUnits=computedKeyUnits??key.stringUnits??units(key.kind==='number'?key.suffix==='n'?String(BigInt(key.value)):String(Number(key.value)):key.value),name=keyUnits.map(unit=>String.fromCharCode(unit)).join(''),firstDefault=parameters.findIndex(parameter=>parameter.$==='errorDefaultParameter'||parameter.$==='errorRestParameter'||(parameter.$==='errorObjectParameter'||parameter.$==='errorArrayParameter')&&parameter.hasDefault),ordinal=arrowNodes.length+methodNodes.indexOf(node),owner=activeFunction;
  templates.push({$:'errorArrowTemplatePlan',owner,ordinal,name:(kind===''?'':kind+' ')+name,length:firstDefault<0?parameters.length:firstDefault,parameters,captures,statements:statements_,receiverMode:'strict'});
  let end=keyAt+1;while(end<input.length&&input[end].start<node.end)end++;if(end<=keyAt+1||input[end-1]?.end!==node.end)fail('literal methods retain their complete token interval');
  return {end,key:keyUnits,value:{$:'errorClosure',owner,ordinal,captures,receiverMode:'strict'}};
 }
 function arrow(input,first,offset){
  if(!classProfile||offset!==0||activeFunction===null)return null;
  const node=arrowNodes.find(row=>row.start===input[first]?.start);if(!node)return null;
  if(input[first].value==='async')fail('async arrow continuations require their separate promise semantics');
  let separator,parameters;
  if(input[first].value==='('){
   const close=findJavaScriptClassGroupEnd(input.map(token=>token.value),first);if(close<0||input[close+1]?.value!=='=>')fail('arrow parameters require their complete actual lexical head');separator=close+1;
   const openAt=tokens.findIndex(token=>token.start===input[first].start),closeAt=tokens.findIndex(token=>token.start===input[close].start);if(openAt<0||closeAt<openAt)fail('arrow parameter head retains its actual source coordinates');
   parameters=readSourceParameters({open:openAt,close:closeAt});
  }else{
   const token=input[first],binding=declared(token);if(token.kind!=='identifier'||input[first+1]?.value!=='=>'||!binding||binding.kind!=='parameter')fail('arrow parameters need their actual binding head');separator=first+1;parameters=[{$:'errorRequiredParameter',identity:binding.identity,...(sharedBindingCells.has(binding.identity)?{shared:true}:{})}];
  }
  let end=separator+1;while(end<input.length&&input[end].start<node.end)end++;
  if(end<=separator+1||input[end-1].end>node.end)fail('arrow body retains its complete actual CST interval');
  const captures=[];for(const binding of bindings){if(binding.scope==='scope:0'||binding.declaration.start>=node.start&&binding.declaration.end<=node.end)continue;if(!binding.references.some(row=>row.start>=node.start&&row.end<=node.end))continue;readonlyCapture(binding,node);captures.push(binding.identity);}
  if(captures.length>4096)fail('arrow lexical captures exceed their actual value budget');
  const ordinal=arrowNodes.indexOf(node),owner=activeFunction,previous=initializingParameters,previousOwners=initializingParameterOwners,previousTargets=controlTargets;let body;
  // Parameter default TDZ selection belongs to creation; body-local parameter
  // references belong to the deferred function body, not that outer phase.
  if(previous!==null&&captures.some(identity=>previousOwners?.has(identity)&&!previous.has(identity)))fail('capturing an uninitialized parameter requires its shared lexical cell');
  initializingParameters=null;initializingParameterOwners=null;controlTargets=[];
  try{if(input[separator+1].value==='{'){const block=sourceField(node,'body'),begin=tokens.findIndex(token=>token.start===input[separator+1].start),last=begin<0?-1:findJavaScriptClassGroupEnd(values,begin);if(!block||block.term!=='statement_block'||begin<0||last<begin||tokens[last].end>block.end)fail('arrow block body retains its actual CST field and source token interval');body=statements(begin+1,last);}else body=[{$:'sourceReturnStatement',value:expression(input.slice(separator+1,end))}];}finally{initializingParameters=previous;initializingParameterOwners=previousOwners;controlTargets=previousTargets;}
  const firstDefault=parameters.findIndex(parameter=>parameter.$==='errorDefaultParameter'||parameter.$==='errorRestParameter'||(parameter.$==='errorObjectParameter'||parameter.$==='errorArrayParameter')&&parameter.hasDefault);
  templates.push({$:'errorArrowTemplatePlan',owner,ordinal,name:'',length:firstDefault<0?parameters.length:firstDefault,parameters,captures,statements:body});
  return {end,value:{$:'errorClosure',owner,ordinal,captures}};
 }
 function expression(input,offset=0){let at=0;const peek=()=>input[at],take=value=>{if(peek()?.value===value){at++;return true;}return false;};
  function variable(token){const binding=reference(token,offset);if(binding){if(initializingParameters!==null&&initializingParameterOwners?.has(binding.identity)&&!initializingParameters.has(binding.identity))fail('parameter defaults require actual earlier initialized bindings; later/self parameter TDZ needs its explicit source continuation');const actualClass=classBinding(binding);if(actualClass)return {$:'errorFunctionReference',identity:actualClass.binding};if(cellIDs.has(binding.identity))return {$:'errorModuleCell',identity:binding.identity};if(functionIDs.has(binding.identity))return {$:'errorFunctionReference',identity:binding.identity};if(importByID.has(binding.identity)){const owner=importByID.get(binding.identity);return port('binding:'+binding.identity,{kind:'import',binding:binding.identity,specifier:owner.specifier,imported:owner.imported});}if(binding.scope!=='scope:0')return {$:'errorBinding',identity:binding.identity};fail('a selected source function refers to an unmodeled module binding');}if(token.value==='undefined'&&unresolved(token,offset))return absent();if(classProfile&&unresolved(token,offset)&&['NaN','Infinity'].includes(token.value))return {$:'errorSpecialNumber',form:token.value};if(!unresolved(token,offset)||!['URL','Buffer','JSON','structuredClone','TypeError','Error',...(classProfile?['Set','Map','Array','Symbol']:[])].includes(token.value))fail('source globals need their actual unshadowed bounded operation owner');return port('global:'+token.value,{kind:'global',name:token.value});}
  function arguments_(end){const result=[];if(take(end))return result;while(at<input.length){if(take('...')){if(!classProfile)fail('iterable argument spread requires its closed source continuation');const value=parse(1),iteratorKey={$:'errorGet',object:port('intrinsic:iterator namespace',{kind:'global',name:'Symbol'}),key:constant({$:'text',units:units('iterator')})};result.push({$:'errorArgumentSpread',value,iteratorKey});}else result.push(parse(1));if(take(end))return result;if(!take(','))fail('source arguments require complete separators');}fail('source arguments require their closing group');}
  function update(target,operator,prefix){
   if(!classProfile||!['errorBinding','errorModuleCell','errorGet','errorComputedGet'].includes(target.$))fail('updates require their actual admitted mutable source reference');
   if(target.$==='errorBinding'&&!['parameter','let'].includes(bindings.find(row=>row.identity===target.identity)?.kind))fail('constant binding updates retain their explicit abrupt PutValue obligation');
   return {$:'errorUpdate',target,operator,prefix};
  }
  function primary(){const token=peek();if(!token)fail('source expressions require a complete operand');let value,member=null;const closure=arrow(input,at,offset);
   if(closure){at=closure.end;value=closure.value;}
   else if(['++','--'].includes(token.value)){at++;return update(parse(9),token.value,true);}
   else if(['!','typeof','-','+'].includes(token.value)){at++;return {$:'errorUnary',operator:token.value,value:parse(9)};}
   else if(take('(')){value=parse(0);if(!take(')'))fail('source parentheses must close');if(value.$==='errorGet'||value.$==='errorComputedGet')member={object:value.object,key:value.key,computed:value.$==='errorComputedGet'};}
   else if(take('[')){
    if(classProfile){
     let length=0,spread=false;const elements=[],parts=[];
     if(!take(']'))while(at<input.length){
      if(take(']'))break;if(take(',')){length++;parts.push({$:'errorArrayHole'});continue;}
      if(take('...')){spread=true;parts.push({$:'errorArraySpread',value:parse(1)});}else{const value=parse(1);elements.push({$:'errorArrayElement',index:length++,value});parts.push({$:'errorArrayValue',value});}
      if(take(']'))break;if(!take(','))fail('Array literal elements need complete separators');if(take(']'))break;
     }
     if(input[at-1]?.value!==']')fail('Array literal needs its closing bracket');
     const iteratorKey=spread?{$:'errorGet',object:port('intrinsic:iterator namespace',{kind:'global',name:'Symbol'}),key:constant({$:'text',units:units('iterator')})}:absent();
     value=spread?{$:'errorArrayAccumulation',parts,iteratorKey}:{$:'errorArrayLiteral',length,elements};
    }
    else{port('syntax:array',{kind:'syntax',operation:'array literal'});value={$:'errorArray',factory:'syntax:array',items:arguments_(']')};}
   }
   else if(take('{')){
    const properties=[];let prototypeSetter=false;if(!take('}'))while(at<input.length){
     if(take('...')){if(!classProfile)fail('object spread requires its actual closed source continuation');properties.push({$:'errorObjectSpread',input:parse(1)});}
     else if(take('[')){
      if(!classProfile)fail('computed object fields require the closed source profile');
      const start=at-1,key=parse(0);if(!take(']'))fail('computed object fields retain their complete actual key');
      if(peek()?.value==='('){const method=objectMethod(input,start,at-1,'',offset,[]);at=method.end;properties.push({$:'errorComputedObjectProperty',key,value:method.value,inferName:true});}
      else{if(!take(':'))fail('computed object fields retain their complete key and value');const field=parse(1);properties.push({$:'errorComputedObjectProperty',key,value:field,...(field.$==='errorClosure'?{inferName:true}:{})});}
     }else{
      const start=at,key=peek();if(!key||!['identifier','string','number'].includes(key.kind))fail('source object fields require their actual fixed or computed data key');
      if(['get','set'].includes(key.value)&&input[at+1]?.value==='['){at+=2;const computed=parse(0);if(!take(']')||peek()?.value!=='(')fail('computed accessor retains its complete actual key and parameter head');const method=objectMethod(input,start,at-1,key.value,offset,[]);at=method.end;properties.push({$:'errorComputedObjectProperty',key:computed,value:method.value,inferName:true,kind:key.value});}
      else{
      const accessor=['get','set'].includes(key.value)&&['identifier','string','number'].includes(input[at+1]?.kind)&&input[at+2]?.value==='(',method=input[at+1]?.value==='(';
      if(accessor||method){const result=objectMethod(input,start,accessor?start+1:start,accessor?key.value:'',offset);at=result.end;properties.push(accessor?{$:'errorObjectAccessor',key:result.key,value:result.value,kind:key.value}:{$:'errorObjectProperty',key:result.key,value:result.value});}
      else{at++;
      const pair=take(':');if(!pair&&key.kind!=='identifier')fail('only actual identifier bindings admit object shorthand');
      const name=key.kind==='number'?key.suffix==='n'?String(BigInt(key.value)):String(Number(key.value)):key.value;if(key.kind==='number'&&key.suffix!=='n'&&!Number.isFinite(Number(key.value)))fail('source numeric property names require their actual finite literal Number');
      const keyUnits=key.stringUnits??units(name),field=pair?parse(1):variable(key),prototype=pair&&keyUnits.length===9&&keyUnits.every((unit,index)=>unit===[95,95,112,114,111,116,111,95,95][index]);
      if(prototype){if(!classProfile)fail('literal prototype installation requires its closed source operation');if(prototypeSetter)fail('duplicate literal prototype setters are an early SyntaxError');prototypeSetter=true;properties.push({$:'errorLiteralPrototype',value:field});}
      else{if(pair)inferArrowName(field,keyUnits.map(unit=>String.fromCharCode(unit)).join(''));properties.push({$:'errorObjectProperty',key:keyUnits,value:field});}
      }
      }
     }
     if(take('}'))break;if(!take(','))fail('source object fields need complete separators');if(take('}'))break;
    }
    if(input[at-1]?.value!=='}')fail('source object fields retain their actual closing group');value={$:'errorObject',properties};
   }
   else if(take('this')){if(activeClass===null&&!classProfile)fail('this requires its actual captured strict function or class receiver');value={$:'errorReceiver'};}
   else if(take('new')){const name=peek(),owner=name&&reference(name,offset),classOwner=classBinding(owner);
    if(name?.kind!=='identifier')fail('construction needs its actual fixed lexical callee');at++;if(!take('('))fail('source construction needs complete arguments');
    if(classOwner)value={$:'errorOrdinaryConstruction',identity:classOwner.binding,arguments:arguments_(')')};
    else{if(owner||!unresolved(name,offset)||!['URL','Error','TypeError',...(classProfile?['Set','Map']:[])].includes(name.value))fail('native construction needs its actual unshadowed source capability');const callee=port('construct:'+name.value,{kind:'construct',name:name.value});value=classProfile&&['Set','Map'].includes(name.value)?{$:'errorCollectionConstruction',source:'construct:'+name.value,arguments:arguments_(')')}:{$:'errorCall',callee,receiver:absent(),arguments:arguments_(')'),construct:false};}
   }
   else if(token.kind==='number'&&token.suffix==='n'){if(!classProfile||token.value.length>4096)fail('BigInt literals retain their explicit source profile and digit budget');at++;value=constant({$:'integer',value:BigInt(token.value)});}
   else if(token.kind==='number'){at++;const value_=Number(token.value);if(!Number.isFinite(value_))fail('source numeric literals need a finite actual Number');value=constant({$:'number',value:value_});}
   else if(token.kind==='string'){at++;value=constant({$:'text',units:token.stringUnits??units(token.value)});}
   else if(token.kind==='template'){at++;const parts=[];for(const part of token.parts){parts.push({$:'errorTemplateText',units:part.stringUnits??units(part.text)});if(part.expression)parts.push({$:'errorTemplateValue',value:expression(tokenize(part.expression.source,'JavaScript').tokens.filter(row=>row.kind!=='eof'),offset+part.expression.offset)});}value={$:'errorTemplate',parts};}
   else if(take('null'))value=constant({$:'null'});else if(take('true'))value=constant({$:'boolean',value:true});else if(take('false'))value=constant({$:'boolean',value:false});
   else if(token.kind==='identifier'){at++;value=variable(token);}else fail('this source operand requires an additional actual semantic operation');
   while(at<input.length){if(peek()?.value==='.'||peek()?.value==='?.'){const optional=take('?.');if(!optional)take('.');const key=peek();if(key?.kind!=='identifier')fail('source member reads require a complete fixed key');at++;const property=constant({$:'text',units:units(key.value)});member=optional?null:{object:value,key:property};value={$:optional?'errorOptionalGet':'errorGet',object:value,key:property};}else if(classProfile&&take('[')){const key=parse(0);if(!take(']'))fail('computed source member access needs its complete key');member={object:value,key,computed:true};value={$:'errorComputedGet',object:value,key};}else if(take('(')){const args=arguments_(')');value=member?{$:member.computed?'errorComputedMethodCall':'errorMethodCall',object:member.object,key:member.key,arguments:args}:{$:'errorCall',callee:value,receiver:absent(),arguments:args,construct:false};member=null;}else break;}if(['++','--'].includes(peek()?.value)&&!/[\r\n\u2028\u2029]/u.test(source.slice(input[at-1].end+offset,peek().start+offset))){const operator=peek().value;at++;value=update(value,operator,false);}return value;
  }
  const assignments=['=','+=','-=','*=','/=','%=','**=','&&=','||=','??='];
  const levels={',':0,'=':1,'+=':1,'-=':1,'*=':1,'/=':1,'%=':1,'**=':1,'&&=':1,'||=':1,'??=':1,'?':2,'||':3,'??':3,'&&':4,'===':5,'!==':5,'<':6,'<=':6,'>':6,'>=':6,'instanceof':6,'+':7,'-':7,'*':8,'/':8,'%':8,'**':9};
  function parse(minimum){if(++expressionDepth>128)fail('project data expressions exceed their explicit source depth');let left=primary();while(at<input.length){const operator=peek().value,level=levels[operator];if(level===undefined||level<minimum)break;at++;
    if(operator==='?'){const yes=parse(1);if(!take(':'))fail('source conditional needs both branches');left={$:'errorConditional',condition:left,yes,no:parse(level)};continue;}
    let right=parse(assignments.includes(operator)||operator==='**'?level:level+1);if(assignments.includes(operator)){
     if(operator!=='='){
      if(!classProfile||!['errorBinding','errorModuleCell','errorGet','errorComputedGet'].includes(left.$)||left.$==='errorBinding'&&!['parameter','let'].includes(bindings.find(row=>row.identity===left.identity)?.kind))fail('compound assignment requires its actual admitted mutable reference');
      if(['&&=','||=','??='].includes(operator)&&(left.$==='errorBinding'||left.$==='errorModuleCell'))inferArrowName(right,bindings.find(row=>row.identity===left.identity).name);
      left={$:'errorCompoundAssignment',operator,target:left,value:right};continue;
     }
     if(left.$==='errorModuleCell'){inferArrowName(right,bindings.find(row=>row.identity===left.identity).name);left={$:'errorModuleCellSet',identity:left.identity,value:right};}else if(left.$==='errorGet'||left.$==='errorComputedGet')left={$:left.$==='errorComputedGet'?'errorComputedSet':'errorSet',object:left.object,key:left.key,value:right};
     else if(classProfile&&left.$==='errorBinding'&&['parameter','let'].includes(bindings.find(row=>row.identity===left.identity)?.kind)){inferArrowName(right,bindings.find(row=>row.identity===left.identity).name);left={$:'errorBindingSet',identity:left.identity,value:right};}else fail('source assignment requires its actual admitted mutable cell or property');
    }
    else if(operator==='==='||operator==='!==')left={$:'errorStrictEqual',left,right,negate:operator==='!=='};
    else {let key='';if(operator==='instanceof'){key='syntax:instanceof';port(key,{kind:'syntax',operation:'instanceof'});}left={$:'errorBinary',operator,left,right,port:key};}
   }expressionDepth--;return left;}
  const result=parse(0);if(at!==input.length)fail('source expression cannot discard unconsumed syntax');return result;
 }
 function split(first,last,separator,preserveEmpty=false){const result=[];let start=first;for(let at=first;at<last;at++){if(['(','{','['].includes(values[at])){const end=findJavaScriptClassGroupEnd(values,at);if(end<0||end>=last)fail('source groups must remain complete');at=end;}else if(values[at]===separator){result.push([start,at]);start=at+1;}}if(start<last||preserveEmpty)result.push([start,last]);return result;}
 // Shared fixed object binding shape; evaluation is lowered once for
 // parameters and lexical declarations, including coercibility and rest copy.
 function readObjectBinding(first,close,kind,depth=0){
  if(depth>128)fail('object binding patterns exceed their explicit source nesting budget');
  const fields=[];let rest;
  for(const [a,b]of split(first+1,close,',')){
   if(values[a]==='...'){
    const owner=declared(tokens[a+1]);if(!classProfile||rest!==undefined||b!==close||values[close-1]===','||b!==a+2||tokens[a+1]?.kind!=='identifier'||!owner||owner.kind!==kind)fail('object rest retains one final actual identifier binding without an initializer');
    rest=owner.identity;if(kind==='parameter')initializingParameters.add(rest);continue;
   }
   const key=tokens[a],computed=key.value==='[',keyEnd=computed?findJavaScriptClassGroupEnd(values,a):-1;
   if(computed&&(!classProfile||keyEnd<0||keyEnd+2>=b||values[keyEnd+1]!==':'))fail('computed object binding keys retain their complete key expression and actual alias');
   const position=computed?keyEnd+2:values[a+1]===':'?a+2:a,nested=values[position]==='{'||values[position]==='[',targetEnd=nested?findJavaScriptClassGroupEnd(values,position):position,owner=nested?undefined:declared(tokens[position]);
   if(!computed&&!['identifier','string'].includes(key.kind)||position>=b||targetEnd<position||targetEnd>=b||!nested&&(!owner||owner.kind!==kind)||b>targetEnd+1&&values[targetEnd+1]!=='='||nested&&!classProfile)fail('source object bindings retain actual keys, nested patterns or binding aliases and optional defaults');
   const keyExpression=computed?expression(tokens.slice(a+1,keyEnd)):undefined,hasDefault=b>targetEnd+1,defaultExpression=hasDefault?expression(tokens.slice(targetEnd+2,b)):absent(),defaultValue=hasDefault&&owner?inferArrowName(defaultExpression,owner.name):defaultExpression,pattern=nested?(values[position]==='['?readArrayBinding(position,targetEnd,kind,depth+1):readObjectBinding(position,targetEnd,kind,depth+1)):undefined,identity=pattern?.identity??owner.identity;
   fields.push({$:'errorParameterField',identity,key:computed?[]:key.stringUnits??units(key.value),hasDefault,value:defaultValue,...(computed?{keyExpression}:{}),...(pattern===undefined?{}:{pattern})});if(kind==='parameter'&&!nested)initializingParameters.add(identity);
  }
  if(fields.length===0&&rest===undefined&&!classProfile)fail('empty object binding patterns require their explicit coercibility operation');
  return {$:'errorObjectParameter',identity:nextBinding++,fields,hasDefault:false,value:absent(),...(rest===undefined?{}:{rest})};
 }
 function readArrayBindingTarget(first,last,kind,depth,rest=false){
  const nested=['{','['].includes(values[first]),end=nested?findJavaScriptClassGroupEnd(values,first):first,owner=nested?undefined:declared(tokens[first]);
  if(first>=last||end<first||end>=last||!nested&&(tokens[first]?.kind!=='identifier'||!owner||owner.kind!==kind)||last>end+1&&(rest||values[end+1]!=='='))fail('Array binding elements retain actual lexical targets, nested patterns and defaults');
  const hasDefault=last>end+1,raw=hasDefault?expression(tokens.slice(end+2,last)):absent(),value=hasDefault&&owner?inferArrowName(raw,owner.name):raw,pattern=nested?(values[first]==='['?readArrayBinding(first,end,kind,depth+1):readObjectBinding(first,end,kind,depth+1)):undefined,identity=pattern?.identity??owner.identity;
  if(kind==='parameter'&&!nested)initializingParameters.add(identity);return {$:'errorArrayBindingElement',identity,hasDefault,value,...(pattern===undefined?{}:{pattern})};
 }
 function readArrayBinding(first,close,kind,depth=0){
  if(!classProfile||depth>128)fail('Array binding patterns retain their actual iterator-capable source profile and nesting budget');const elements=[],parts=split(first+1,close,',',true);let rest;
  if(parts.length&&parts[parts.length-1][0]===parts[parts.length-1][1])parts.pop();
  for(let at=0;at<parts.length;at++){const [a,b]=parts[at];if(a===b){elements.push({$:'errorArrayElision'});continue;}
   if(values[a]==='...'){if(at!==parts.length-1||values[close-1]===',')fail('Array rest retains its actual final target without a trailing comma');rest=readArrayBindingTarget(a+1,b,kind,depth,true);}
   else elements.push(readArrayBindingTarget(a,b,kind,depth));
  }
  const iteratorKey={$:'errorGet',object:port('intrinsic:iterator namespace',{kind:'global',name:'Symbol'}),key:constant({$:'text',units:units('iterator')})};return {$:'errorArrayParameter',identity:nextBinding++,hasDefault:false,value:absent(),iteratorKey,elements,...(rest===undefined?{}:{rest})};
 }
 function readSourceParameters(header){
  if(header.implicitConstructor)return [];
  const previous=initializingParameters,previousOwners=initializingParameterOwners;initializingParameters=new Set();
  initializingParameterOwners=new Set(bindings.filter(binding=>binding.kind==='parameter'&&binding.declaration.start>=tokens[header.open].start&&binding.declaration.end<=tokens[header.close].end).map(binding=>binding.identity));
  try{return split(header.open+1,header.close,',').map(([first,last])=>{
   const token=tokens[first],binding=declared(token);
   if(token.value==='...'){
    if(!classProfile||last!==header.close)fail('rest parameters require their actual final source formal without a trailing comma');const target=tokens[first+1],nested=target?.value==='{'||target?.value==='[',end=nested?findJavaScriptClassGroupEnd(values,first+1):first+1,owner=nested?undefined:declared(target);if(end+1!==last||!nested&&(target?.kind!=='identifier'||!owner||owner.kind!=='parameter'))fail('rest parameters retain their actual identifier or recursive binding target');
    const pattern=nested?(target.value==='['?readArrayBinding(first+1,end,'parameter'):readObjectBinding(first+1,end,'parameter')):undefined,identity=pattern?.identity??owner.identity;if(!nested)initializingParameters.add(identity);return {$:'errorRestParameter',identity,...(pattern===undefined?{}:{pattern}),...(sharedBindingCells.has(identity)?{shared:true}:{})};
   }
   if(token.value==='{'||token.value==='['){
    const close=findJavaScriptClassGroupEnd(values,first);if(close<0||close>=last)fail('source object parameter retains its complete binding pattern');
    const hasDefault=close+1<last;if(hasDefault&&values[close+1]!=='=')fail('object parameter default needs its actual source expression');const value=hasDefault?expression(tokens.slice(close+2,last)):absent();
    return {...(token.value==='['?readArrayBinding(first,close,'parameter'):readObjectBinding(first,close,'parameter')),hasDefault,value};
   }
   if(!binding||binding.kind!=='parameter'||last>first+1&&values[first+1]!=='=')fail('source parameters need actual fixed binding identities; rest and other patterns need their own lowering');
   const parameter=last===first+1?{$:'errorRequiredParameter',identity:binding.identity}:{$:'errorDefaultParameter',identity:binding.identity,value:inferArrowName(expression(tokens.slice(first+2,last)),binding.name)};initializingParameters.add(binding.identity);return {...parameter,...(sharedBindingCells.has(binding.identity)?{shared:true}:{})};
  });}finally{initializingParameters=previous;initializingParameterOwners=previousOwners;}
 }
 function lexicalDeclarations(first,last,kind){
  if(!['let','const'].includes(kind)||kind==='let'&&!classProfile)fail('lexical declaration groups require their actual source initialization profile');
  const result=[];
  for(const [start,end]of split(first,last,',')){
   if(classProfile&&['{','['].includes(values[start])){
    const close=findJavaScriptClassGroupEnd(values,start);if(close<0||close+2>=end||values[close+1]!=='=')fail('lexical object bindings retain their complete required source initializer');
    const value=expression(tokens.slice(close+2,end)),pattern=values[start]==='['?readArrayBinding(start,close,kind):readObjectBinding(start,close,kind);result.push({$:pattern.$==='errorArrayParameter'?'sourceArrayBindingStatement':'sourceObjectBindingStatement',pattern,value,mutable:kind==='let'});continue;
   }
   const token=tokens[start],owner=declared(token),hasInitializer=start+1<end;
   if(token?.kind!=='identifier'||!owner||owner.kind!==kind||hasInitializer&&values[start+1]!=='='||kind==='const'&&!hasInitializer)fail('each lexical declarator retains its actual binding and complete initializer; patterns need their own binding protocol');
   result.push({$:kind==='let'?'sourceMutableStatement':'sourceConstStatement',identity:owner.identity,value:hasInitializer?inferArrowName(expression(tokens.slice(start+2,end)),owner.name):absent(),...(sharedBindingCells.has(owner.identity)?{shared:true}:{})});
  }
  if(result.length===0)fail('a lexical declaration group requires its actual declarators');return result;
 }
 function statements(first,last){if(++statementDepth>128)fail('project data statements exceed their explicit source depth');const result=[];let at=first;
  function one(){if(values[at]===';'){at++;return [];}if(values[at]==='{'){const end=findJavaScriptClassGroupEnd(values,at);if(end<0||end>last)fail('source blocks require complete groups');const body=statements(at+1,end);at=end+1;return body;}return [statement()];}
  function statement(){
   const labels=[];
   while(classProfile&&tokens[at]?.kind==='identifier'&&values[at+1]===':'){
    const label=tokens[at],node=program.sourceMappings.find(row=>row.term==='labeled_statement'&&row.start===label.start),owned=node&&sourceField(node,'label'),body=node&&sourceField(node,'body');
    if(!node||!owned||owned.start!==label.start||owned.end!==label.end||!body||body.start!==tokens[at+2]?.start)fail('labels retain their actual CST label and body fields');
    if(labels.length>=128||labels.includes(label.value)||controlTargets.some(row=>row.labels?.includes(label.value)))fail('active source labels retain distinct bounded lexical owners');
    labels.push(label.value);at+=2;
   }
   if(labels.length&&!['while','do','for','switch'].includes(values[at])){
    const target=nextBinding++;controlTargets.push({identity:target,kind:'block',labels});let body;
    try{body=one();}finally{controlTargets.pop();}return {$:'sourceControlBlockStatement',target,body};
   }
   const word=values[at++];if(word==='if'){const open=at,close=findJavaScriptClassGroupEnd(values,open);if(values[open]!=='('||close<0)fail('source if requires its complete condition');const condition=expression(tokens.slice(open+1,close));at=close+1;const yes=one(),no=values[at]==='else'?(at++,one()):[];return {$:'sourceIfStatement',condition,yes,no};}
   if(word==='try'&&classProfile){
    const bodyOpen=at,bodyClose=findJavaScriptClassGroupEnd(values,bodyOpen);if(values[bodyOpen]!=='{'||bodyClose<0||bodyClose>=last)fail('source try retains its actual complete protected block');const body=statements(bodyOpen+1,bodyClose);at=bodyClose+1;let protectedBody=body;
    if(values[at]==='catch'){const catchStart=tokens[at].start;at++;let identity=nextBinding++,pattern;
    if(values[at]==='('){
     const open=at,close=findJavaScriptClassGroupEnd(values,open),token=tokens[open+1],owner=declared(token);if(close<0)fail('catch retains its complete binding head');
     if(token?.value==='{'||token?.value==='['){
      const end=findJavaScriptClassGroupEnd(values,open+1);if(end+1!==close)fail('catch bindings retain their actual complete pattern without an outer initializer');
      const previous=initializingParameters,previousOwners=initializingParameterOwners;initializingParameters=new Set();initializingParameterOwners=new Set(bindings.filter(binding=>binding.kind==='parameter'&&binding.declaration.start>=token.start&&binding.declaration.end<=tokens[end].end).map(binding=>binding.identity));
      try{pattern=token.value==='['?readArrayBinding(open+1,end,'parameter'):readObjectBinding(open+1,end,'parameter');identity=pattern.identity;}finally{initializingParameters=previous;initializingParameterOwners=previousOwners;}
     }else{if(close!==open+2||token?.kind!=='identifier'||!owner||owner.kind!=='parameter')fail('catch array bindings require their actual iterator initialization protocol');identity=owner.identity;}
     at=close+1;
    }
    const handlerOpen=at,handlerClose=findJavaScriptClassGroupEnd(values,handlerOpen),clause=program.sourceMappings.find(row=>row.term==='catch_clause'&&row.start===catchStart&&row.end===tokens[handlerClose]?.end);
    if(values[handlerOpen]!=='{'||handlerClose<0||handlerClose>=last||!clause)fail('source catch retains its actual complete lexical clause');const handler=statements(handlerOpen+1,handlerClose);at=handlerClose+1;protectedBody=[{$:'sourceTryStatement',identity,body,handler,...(pattern===undefined?{}:{pattern})}];
    }
    if(values[at]==='finally'){
     at++;const open=at,close=findJavaScriptClassGroupEnd(values,open);if(values[open]!=='{'||close<0||close>=last)fail('finally retains its actual complete source block');const finalizer=statements(open+1,close);at=close+1;return {$:'sourceFinallyStatement',body:protectedBody,finalizer};
    }
    if(protectedBody===body)fail('try requires its actual catch or finally clause');return protectedBody[0];
   }
   if((word==='return'||word==='throw')&&tokens[at]&&/[\r\n\u2028\u2029]/u.test(source.slice(tokens[at-1].end,tokens[at].start))){if(word==='throw')fail('throw requires its actual same-line source operand');return {$:'sourceReturnStatement',value:absent()};}
   if((word==='break'||word==='continue')&&classProfile){
    const continuing=word==='continue',sameLine=tokens[at]&&!/[\r\n\u2028\u2029]/u.test(source.slice(tokens[at-1].end,tokens[at].start)),label=sameLine&&tokens[at]?.kind==='identifier'?tokens[at++].value:null;
    const target=[...controlTargets].reverse().find(row=>label!==null?row.labels?.includes(label):continuing?row.kind==='loop':row.kind==='loop'||row.kind==='switch');
    if(!target||continuing&&target.kind!=='loop')fail('source break/continue requires its actual enclosing labelled or iteration target');
    if(values[at]===';')at++;else if(at<last&&values[at]!=='}'&&!/[\r\n\u2028\u2029]/u.test(source.slice(tokens[at-1].end,tokens[at].start)))fail('control transfers retain their complete actual statement boundary');
    return {$:'sourceControlJumpStatement',target:target.identity,continuing};
   }
   if(word==='do'&&classProfile){
    const start=tokens[at-1].start,node=program.sourceMappings.find(row=>row.term==='do_statement'&&row.start===start),target=nextBinding++;let body;
    if(!node)fail('post-test loops retain their complete actual CST statement');
    controlTargets.push({identity:target,kind:'loop',labels});try{body=one();}finally{controlTargets.pop();}
    if(values[at++]!=='while')fail('post-test loops retain their actual trailing while condition');const open=at,close=findJavaScriptClassGroupEnd(values,open);
    if(values[open]!=='('||close<0||tokens[close].end>node.end)fail('post-test loop conditions retain their complete actual group');const condition=expression(tokens.slice(open+1,close));at=close+1;if(values[at]===';')at++;
    if(tokens[at]?.start<node.end)fail('post-test loops cannot discard trailing source syntax');
    return {$:'sourceWhileStatement',target,condition,body,postTest:true};
   }
   if(word==='while'&&classProfile){
    const open=at,close=findJavaScriptClassGroupEnd(values,open);if(values[open]!=='('||close<0)fail('source while needs its actual complete condition');const condition=expression(tokens.slice(open+1,close)),target=nextBinding++;at=close+1;let body;
    controlTargets.push({identity:target,kind:'loop',labels});try{body=one();}finally{controlTargets.pop();}
    return {$:'sourceWhileStatement',target,condition,body};
   }
   if(word==='switch'&&classProfile){
    const start=tokens[at-1].start,open=at,close=findJavaScriptClassGroupEnd(values,open),body=close+1,end=findJavaScriptClassGroupEnd(values,body);
    if(values[open]!=='('||close<0||values[body]!=='{'||end<0||!program.sourceMappings.some(row=>row.term==='switch_statement'&&row.start===start&&row.end===tokens[end].end))fail('source switch needs its actual complete CST condition and case block');
    const value=expression(tokens.slice(open+1,close)),target=nextBinding++,clauses=[];let cursor=body+1,defaults=0;
    controlTargets.push({identity:target,kind:'switch',labels});
    try{while(cursor<end){
     const term=values[cursor]==='case'?'switch_case':values[cursor]==='default'?'switch_default':'';const node=program.sourceMappings.find(row=>row.term===term&&row.start===tokens[cursor].start);if(!term||!node)fail('switch clauses require actual source CST boundaries');
     let next=cursor;while(next<end&&tokens[next].start<node.end)next++;
     if(next<=cursor||tokens[next-1].end>node.end)fail('switch clause cannot discard source tokens');
     let colon=cursor+1,test=null;
     if(term==='switch_case'){
      const selector=sourceField(node,'value');if(!selector)fail('a case selector requires its actual CST value field');
      while(colon<next&&tokens[colon].start<selector.end)colon++;test=expression(tokens.slice(cursor+1,colon));
     }else if(++defaults>1)fail('switch cannot contain duplicate default clauses');
     if(values[colon]!==':')fail('switch selector requires its actual clause separator');
     const statements_=statements(colon+1,next);if(statements_.some(row=>row.$==='sourceConstStatement'||row.$==='sourceMutableStatement'||row.$==='sourceDeclarationGroupStatement'||row.$==='sourceObjectBindingStatement'||row.$==='sourceArrayBindingStatement'))fail('unbraced switch lexical declarations require their shared TDZ environment');
     if(clauses.length>=4096)fail('switch clause source witnesses exceed their explicit directory budget');
     clauses.push(test===null?{$:'sourceDefaultStatement',body:statements_}:{$:'sourceCaseStatement',value:test,body:statements_});cursor=next;
    }}finally{controlTargets.pop();}
    at=end+1;return {$:'sourceSwitchStatement',target,value,clauses};
   }
   if(word==='for'&&classProfile){
    const open=at,close=findJavaScriptClassGroupEnd(values,open),parts=close>=0?split(open+1,close,';',true):[];
    if(values[open]==='('&&close>=0&&parts.length===1&&['const','let'].includes(values[open+1])){
     const kind=values[open+1],first=open+2,end=['{','['].includes(values[first])?findJavaScriptClassGroupEnd(values,first):first;
     if(values[end+1]==='of'){
      if(!program.sourceMappings.some(row=>['for_in_statement','for_of_statement'].includes(row.term)&&row.start===tokens[open-1].start))fail('for-of retains its actual complete source CST');
      const pattern=values[first]==='{'?readObjectBinding(first,end,kind):values[first]==='['?readArrayBinding(first,end,kind):undefined,owner=pattern===undefined?declared(tokens[first]):undefined;if(pattern===undefined&&(tokens[first]?.kind!=='identifier'||!owner||owner.kind!==kind))fail('for-of array bindings and assignment heads require their actual initialization protocol');
      const input=expression(tokens.slice(end+2,close)),target=nextBinding++,iteratorKey={$:'errorGet',object:port('intrinsic:iterator namespace',{kind:'global',name:'Symbol'}),key:constant({$:'text',units:units('iterator')})};at=close+1;let body;
      controlTargets.push({identity:target,kind:'loop',labels});try{body=one();}finally{controlTargets.pop();}
      return {$:'sourceForOfStatement',target,binding:pattern?.identity??owner.identity,mutable:kind==='let',input,iteratorKey,body,...(pattern===undefined?{}:{pattern})};
     }
    }
    if(values[open]!=='('||parts.length!==3)fail('ordinary for loops retain their actual initialization, test and update clauses; iterable loops need their separate protocol');
    const initializers=[],[first,lastInitial]=parts[0];
    if(first<lastInitial){
     if(values[first]==='let'||values[first]==='const'){
      initializers.push(...lexicalDeclarations(first+1,lastInitial,values[first]));
     }else if(values[first]==='var')fail('function-scoped var initialization requires its actual hoisting environment');
     else initializers.push({$:'sourceExpressionStatement',value:expression(tokens.slice(first,lastInitial))});
    }
    const condition=parts[1][0]===parts[1][1]?constant({$:'boolean',value:true}):expression(tokens.slice(...parts[1])),update=parts[2][0]===parts[2][1]?null:expression(tokens.slice(...parts[2])),target=nextBinding++;at=close+1;let body;
    controlTargets.push({identity:target,kind:'loop',labels});try{body=one();}finally{controlTargets.pop();}
    return {$:'sourceBlockStatement',statements:initializers.concat([{$:'sourceWhileStatement',target,condition,...(update!==null?{update}:{}),body}])};
   }
   if(word==='{'){at--;return {$:'sourceBlockStatement',statements:one()};}
   const begin=at-1;let end=at;for(;end<last&&values[end]!==';';end++)if(['(','{','['].includes(values[end])){const close=findJavaScriptClassGroupEnd(values,end);if(close<0||close>=last)fail('source statement groups must close');end=close;}if(end>=last)fail('this source statement profile requires explicit actual semicolons');at=end+1;
   if(word==='const'||word==='let'&&classProfile){const declarations=lexicalDeclarations(begin+1,end,word);return declarations.length===1?declarations[0]:{$:'sourceDeclarationGroupStatement',statements:declarations};}
   if(word==='return'||word==='throw')return {$:word==='return'?'sourceReturnStatement':'sourceThrowStatement',value:end===begin+1?absent():expression(tokens.slice(begin+1,end))};
   return {$:'sourceExpressionStatement',value:expression(tokens.slice(begin,end))};
  }
  while(at<last){if(values[at]===';'){at++;continue;}result.push(statement());}statementDepth--;return result;
 }
 const functions=[],origins=[];for(const header of headers.filter(row=>selected.has(row.binding.identity))){
  for(const member of [header,...(header.methods??[])]){
   activeClass=member.classBinding??null;activeFunction=member.binding.identity;controlTargets=[];const params=readSourceParameters(member),body=member.implicitConstructor?[]:statements(member.body+1,member.last);
   functions.push({$:member.isConstructor?'errorOrdinaryConstructorPlan':member.classBinding!==undefined?'errorMethodFunctionPlan':classProfile?'errorMutableStatementFunctionPlan':'errorStatementFunctionPlan',identity:member.binding.identity,name:member.binding.name,parameters:params,statements:body});origins.push({identity:member.binding.identity,name:member.binding.name,start:member.node.start,end:member.node.end,coordinateUnit:'UTF-16 code units',...(member.classBinding!==undefined?{classBinding:member.classBinding,constructor:member.isConstructor}: {})});
  }
 }activeClass=null;activeFunction=null;
 return {schemaVersion:1,artifactKind:'authenticated semantic source plan/input 1',model:profile,identifier,source,sourceBytes:Buffer.byteLength(source),sourceSha256:createHash('sha256').update(source).digest('hex'),entries:[...entries],origins,imports:imports.filter(row=>capabilities.has('binding:'+row.binding)),cells,capabilities:[...capabilities.values()],...(classProfile?{classes:classes.filter(row=>selected.has(row.binding))}:{}),qualification:'owned selected source functions/classes; imported operations require their actual qualified implementations; not a checked portable Core program',plan:{$:'errorModulePlan',functions,...(templates.length?{templates:templates.toSorted((a,b)=>a.owner-b.owner||a.ordinal-b.ordinal)}:{}),exports:entries.map(name=>headers.find(row=>row.binding.name===name).binding.identity),nextBinding}};
}

export function checkJavaScriptProjectDataSource(capture){assertPlainSourceReceipt(capture);const actual=captureJavaScriptProjectDataSource(capture.source,{identifier:capture.identifier,entries:capture.entries,profile:capture.model});if(JSON.stringify(actual)!==JSON.stringify(capture))throw new TypeError('project data source plan must retain its complete CST, selected binding closure and operation owners');return actual;}
