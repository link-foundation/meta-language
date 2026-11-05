// Full source/CST capture only. This module never reads policy contents,
// imports represented modules, or executes a represented initializer/body.
import {createHash} from 'node:crypto';
import {analyzeProgram} from '../program-representation.js';
import {tokenize} from './lexer.js';
import {sourceLibraryLogicalModuleURL,sourceLibraryLogicalModulePath} from './source-library-closure.js';
import {readJavaScriptRuntimeDataVersionError,readJavaScriptModuleInitializationProfileError} from './javascript-module-initialization-rules.js';
import {assertPlainSourceReceipt} from './source-receipt-data.js';
import {readJavaScriptModuleInitializationPlanError} from './javascript-module-initialization.js';
const digest=source=>createHash('sha256').update(source).digest('hex'),profile='actual runtime Links policy initializers 1';
function syntax(source){if(typeof source!=='string'||Buffer.byteLength(source)>131072)throw new TypeError('runtime module source must retain its bounded complete bytes');const program=analyzeProgram(source,'JavaScript');if(!program.network.verifyFullMatch().isClean())throw new TypeError('runtime module source requires complete actual CST without recovery');return program;}
function declaration(program,binding,terms){return program.sourceMappings.filter(node=>terms.includes(node.term)&&node.start<=binding.declaration.start&&node.end>=binding.declaration.end).sort((left,right)=>(left.end-left.start)-(right.end-right.start))[0];}
function exported(program,node){return program.sourceMappings.find(row=>row.term==='export_statement'&&row.start<=node.start&&row.end>=node.end);}
function functionSource(source,name){const program=syntax(source),binding=program.bindings.find(row=>row.scope==='scope:0'&&row.kind==='function'&&row.name===name),node=binding&&declaration(program,binding,['function_declaration']);if(!node||!exported(program,node))throw new TypeError('the qualified adapter needs its actual exported source function');return {source:source.slice(node.start,node.end),start:node.start,end:node.end,coordinateUnit:'UTF-16 code units'};}

export function captureRuntimeDataAdapterSource({loaderPath,loaderSource,delegatePath,delegateSource,nodeMajor,nodeMinor,linksVersion}){
 const version=readJavaScriptRuntimeDataVersionError(nodeMajor,nodeMinor,linksVersion);if(version)throw new TypeError(version);
 return constructRuntimeDataAdapterSource({loaderPath,loaderSource,delegatePath,delegateSource});
}
function constructRuntimeDataAdapterSource({loaderPath,loaderSource,delegatePath,delegateSource}){
 sourceLibraryLogicalModuleURL(loaderPath);sourceLibraryLogicalModuleURL(delegatePath);
 const loader=syntax(loaderSource),fn=functionSource(loaderSource,'readRuntimeProjectData'),delegate=functionSource(delegateSource,'readProjectData');
 const tokens=tokenize(fn.source,'JavaScript').tokens.filter(token=>token.kind!=='eof').map(token=>token.value);
 if(JSON.stringify(tokens)!==JSON.stringify(['function','readRuntimeProjectData','(','file',',','options',')','{','return','readProjectData','(','file',',','options',')',';','}']))throw new TypeError('the qualified runtime adapter must retain its actual direct readProjectData delegation');
 const imported=loader.bindings.find(row=>row.scope==='scope:0'&&row.kind==='import'&&row.name==='readProjectData'),statement=imported&&declaration(loader,imported,['import_statement']);
 const importTokens=statement?tokenize(loaderSource.slice(statement.start,statement.end),'JavaScript').tokens.filter(token=>token.kind!=='eof'):[];
 if(importTokens.length!==7||JSON.stringify(importTokens.slice(0,5).map(token=>token.value))!==JSON.stringify(['import','{','readProjectData','}','from'])||importTokens[5].kind!=='string'||importTokens[6].value!==';'||sourceLibraryLogicalModulePath(new URL(importTokens[5].value,sourceLibraryLogicalModuleURL(loaderPath)))!==delegatePath)throw new TypeError('the adapter delegation needs its actual owned project-data-files import');
 for(const token of tokenize(loaderSource,'JavaScript').tokens)if(token.kind!=='eof'&&token.value!==';'&&!(token.start>=statement.start&&token.end<=statement.end)&&!(token.start>=(exported(loader,{start:fn.start,end:fn.end})?.start??fn.start)&&token.end<=fn.end))throw new TypeError('the qualified forwarding module cannot hide initialization effects');
 const contract={schemaVersion:1,model:'qualified runtime Links data IO adapter 1',qualification:'external IO/decoder boundary; owned loader/decode bodies are not claimed self-translated',versions:{node:'>=22.15 <23 || >=24 <25',linksNotation:'0.22.0'},loader:{path:loaderPath,source:loaderSource,bytes:Buffer.byteLength(loaderSource),sourceSha256:digest(loaderSource),declaration:fn},delegate:{path:delegatePath,source:delegateSource,bytes:Buffer.byteLength(delegateSource),sourceSha256:digest(delegateSource),declaration:delegate}};
 return {...contract,identity:digest(JSON.stringify(contract))};
}

export function checkRuntimeDataAdapterSource(contract){
 assertPlainSourceReceipt(contract);
 const actual=constructRuntimeDataAdapterSource({loaderPath:contract.loader.path,loaderSource:contract.loader.source,delegatePath:contract.delegate.path,delegateSource:contract.delegate.source});if(JSON.stringify(actual)!==JSON.stringify(contract))throw new TypeError('runtime data adapter qualification must retain complete actual provider sources and hashes');return actual;
}

export function captureJavaScriptRuntimeDataModule(source,{identifier,readerContract,model=profile}={}){
 const selected=readJavaScriptModuleInitializationProfileError(model);if(selected)throw new TypeError(selected);sourceLibraryLogicalModuleURL(identifier);const contract=checkRuntimeDataAdapterSource(readerContract),program=syntax(source),tokens=tokenize(source,'JavaScript').tokens.filter(token=>token.kind!=='eof'),bindings=program.bindings.filter(row=>row.scope==='scope:0'),ids=new Map(program.bindings.map((binding,index)=>[binding.id,index]));
 const initializers=[],dataBindings=[],names=[],ignoredRanges=[],functions=[],ranges=[],exports=[];let reader=null,readerImport=null;
 for(const binding of bindings.filter(row=>row.kind==='import')){
  const node=declaration(program,binding,['import_statement']);if(!node)throw new TypeError('a runtime loader import needs its complete actual CST statement');const row=tokens.filter(token=>token.start>=node.start&&token.end<=node.end);let at=0;
  if(row[at++]?.value!=='import'||row[at++]?.value!=='{'||row[at++]?.value!=='readRuntimeProjectData')throw new TypeError('this module initializer profile imports only its actual qualified named reader');
  let local='readRuntimeProjectData';if(row[at]?.value==='as'){at++;local=row[at++]?.value;}
  if(row[at++]?.value!=='}'||row[at++]?.value!=='from'||row[at]?.kind!=='string')throw new TypeError('the reader import needs exactly one actual qualified provider and lexical binding');
  const specifier=row[at++].value;if(sourceLibraryLogicalModulePath(new URL(specifier,sourceLibraryLogicalModuleURL(identifier)))!==contract.loader.path||row[at++]?.value!==';'||at!==row.length||binding.name!==local||reader!==null)throw new TypeError('the reader import needs exactly one actual qualified provider and lexical binding');
  reader=binding;readerImport={binding:ids.get(binding.id),localName:binding.name,specifier,start:node.start,end:node.end,coordinateUnit:'UTF-16 code units'};ignoredRanges.push({start:node.start,end:node.end,coordinateUnit:'UTF-16 code units'});ranges.push(node);
 }
 const constants=bindings.filter(row=>row.kind==='const').map(binding=>{const node=declaration(program,binding,['lexical_declaration']);if(!node)throw new TypeError('a policy initializer needs its actual complete const declaration');return {binding,node};}).sort((left,right)=>left.node.start-right.node.start);
 for(const {binding,node}of constants){
  if(!node)throw new TypeError('a policy initializer needs its actual complete const declaration');const row=tokens.filter(token=>token.start>=node.start&&token.end<=node.end),owner=ids.get(binding.id);if(row[0]?.value!=='const'||row[1]?.start!==binding.declaration.start||row[2]?.value!=='='||row.at(-1)?.value!==';')throw new TypeError('runtime data bindings require one complete ordinary const initializer');
  const expression=row.slice(3,-1),reference=token=>program.bindings.find(binding=>binding.references.some(row=>row.start===token.start&&row.end===token.end));let initializer;
  if(expression.length===1&&expression[0].kind==='identifier'){
   const target=reference(expression[0]);if(!target||target.kind!=='const'||target.scope!=='scope:0')throw new TypeError('runtime aliases retain actual owned module const bindings');initializer={$:'moduleConstAlias',binding:owner,target:ids.get(target.id)};
  }else{
   const words=expression.map((token,index)=>index===5?'#specifier':token.value);
   if(!reader||expression.length!==14||reference(expression[0])?.id!==reader.id||JSON.stringify(words)!==JSON.stringify([reader.name,'(','new','URL','(','#specifier',',','import','.','meta','.','url',')',')']))throw new TypeError('policy loads require the exact qualified reader(new URL(literal, import.meta.url)) source form');
   const url=expression[3],specifier=expression[5];if(specifier.kind!=='string'||specifier.stringUnits||reference(url)!==undefined||!program.unresolvedReferences.some(row=>row.name==='URL'&&row.start===url.start&&row.end===url.end))throw new TypeError('URL construction needs its actual unshadowed standard global binding and literal source operand');
   initializer={$:'moduleDataLoad',binding:owner,specifier:specifier.value,readerOwner:contract.identity};
  }
  initializers.push(initializer);dataBindings.push(owner);names.push(binding.name);ignoredRanges.push({start:(exported(program,node)?.start??node.start),end:node.end,coordinateUnit:'UTF-16 code units'});ranges.push(exported(program,node)??node);if(exported(program,node))exports.push({name:binding.name,binding:owner,kind:'data'});
 }
 for(const binding of bindings.filter(row=>row.kind==='function'||row.kind==='class')){const node=declaration(program,binding,[binding.kind==='class'?'class_declaration':'function_declaration']);if(!node)throw new TypeError('module functions/classes retain their actual complete source declarations');functions.push({name:binding.name,binding:ids.get(binding.id),start:node.start,end:node.end,kind:binding.kind,coordinateUnit:'UTF-16 code units'});ranges.push(exported(program,node)??node);if(exported(program,node))exports.push({name:binding.name,binding:ids.get(binding.id),kind:'function'});}
 for(const binding of bindings)if(!['import','const','function','class'].includes(binding.kind))throw new TypeError('mutable/reassigned module bindings require their actual cell initialization model');
 for(const token of tokens)if(token.value!==';'&&!ranges.some(range=>token.start>=range.start&&token.end<=range.end))throw new TypeError('module initialization cannot omit an actual top-level source effect');
 if(initializers.some(row=>row.$==='moduleConstAlias'&&!dataBindings.includes(row.target)))throw new TypeError('a const alias target must retain an actual modeled data binding');
 const plan={$:'moduleInitializationPlan',profile,moduleIdentity:identifier,sourceSha256:digest(source),bindings:dataBindings,names,initializers},invalid=readJavaScriptModuleInitializationPlanError(plan);if(invalid)throw new TypeError(invalid);
 return {schemaVersion:1,artifactKind:'authenticated semantic source plan/input 1',model:profile,identifier,source,sourceBytes:Buffer.byteLength(source),sourceSha256:digest(source),readerContract:contract,readerImport,ignoredRanges,functions,exports,plan};
}

export function checkJavaScriptRuntimeDataModule(capture){assertPlainSourceReceipt(capture);const actual=captureJavaScriptRuntimeDataModule(capture.source,{identifier:capture.identifier,readerContract:capture.readerContract,model:capture.model});if(JSON.stringify(actual)!==JSON.stringify(capture))throw new TypeError('module initializer bindings and source receipts must match their complete actual CST');return actual;}
