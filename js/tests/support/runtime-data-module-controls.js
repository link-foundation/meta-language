// Future controls over real source modules and the shared portable machine.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {syncBuiltinESMExports} from 'node:module';
import {join,relative,basename,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {structuredDataToLino,readStructuredDataLino,encodeStructuredDataLinks} from '../../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
import * as machine from '../../src/translation/javascript-module-initialization.js';
import {captureJavaScriptRuntimeDataModule} from '../../src/translation/runtime-data-module-source.js';
import {readShippedRuntimeDataAdapterContract,readActiveRuntimeDataVersions,prepareJavaScriptRuntimeDataModule,createRuntimeDataModuleScope} from '../../src/translation/runtime-data-module.js';
import {captureJavaScriptRuntimeDataFunctions} from '../../src/translation/javascript-error-classes.js';
import {runJavaScriptSourceTransition} from '../../src/node-source-modules.js';
import {createActualErrorSourceDeclarations} from './error-source-module-controls.js';
import {readCollectionCopyConcreteControl} from './collection-copy-concrete.js';
import {createControlDiagnostics,requireControlProduct as product} from './control-diagnostics.js';
import * as codecs from '../../../experiments/javascript-properties/module-initialization-codecs.mjs';
import * as errorCodecs from '../../../experiments/javascript-properties/error-class-source-codecs.mjs';
const root=fileURLToPath(new URL('../../../',import.meta.url));
const diagnostics=value=>({assertions:value.assertions,failures:value.failures});

export function readRuntimeDataModelDiagnostic(definition){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 const plan=attempt('actual model plan decode',()=>codecs.decodeJavaScriptModuleInitializationPlan(definition.plan));
 attempt('actual model plan codec replay',()=>assert.deepEqual(codecs.encodeJavaScriptModuleInitializationPlan(product(plan,'actual model plan decode')),definition.plan));
 attempt('actual model plan semantic validation',()=>assert.equal(machine.readJavaScriptModuleInitializationPlanError(product(plan,'actual model plan decode')),''));
 let state=attempt('initial state creation',()=>machine.createJavaScriptModuleInitialization(product(plan,'actual model plan decode'),definition.moduleURL,BigInt(definition.realm))),response=0;const events=[],requests=[];
 const initialState=attempt('initial state encoding',()=>codecs.encodeJavaScriptModuleInitializationState(product(state,'initial state creation')));
 attempt('complete literal model execution',()=>{
  for(let step=0;step<8192;step++){
   let transition=machine.stepJavaScriptModuleInitialization(product(plan,'actual model plan decode'),product(state,'initial state creation'));events.push({operation:'step',transition:codecs.encodeJavaScriptModuleInitializationTransition(transition),value:null,abrupt:false});state=transition.state;
   if(transition.$==='moduleContinue')continue;
   if(transition.$==='moduleReadData'||transition.$==='moduleTemporalDeadZone'){
    requests.push(transition.$==='moduleReadData'?['load',transition.binding,transition.specifier]:['tdz',transition.binding,transition.name]);const input=definition.responses[response++];assert.ok(input,'every actual request requires its declared response');const value=codecs.decodeJavaScriptSemanticValue(input.value);
    transition=machine.resumeJavaScriptModuleInitialization(plan,state,value,input.abrupt);events.push({operation:'resume',transition:codecs.encodeJavaScriptModuleInitializationTransition(transition),value:codecs.encodeJavaScriptSemanticValue(value),abrupt:input.abrupt});state=transition.state;if(transition.$==='moduleContinue')continue;
   }
   assert.ok(['moduleEvaluated','moduleAbrupt'].includes(transition.$));return;
  }assert.fail('finite literal model exceeded its explicit step budget');
 });
 const finalState=attempt('final model state encoding',()=>codecs.encodeJavaScriptModuleInitializationState(product(state,'initial state creation')));
 attempt('every declared response consumed',()=>assert.equal(response,definition.responses.length));attempt('literal final module phase',()=>assert.equal(product(state,'initial state creation').phase,definition.expectedPhase));attempt('literal request order',()=>assert.deepEqual(requests,definition.expectedRequests));
 if(definition.expectedFailure)attempt('literal cached error identity',()=>assert.deepEqual(codecs.encodeJavaScriptSemanticValue(product(state,'initial state creation').failure),definition.expectedFailure));
 for(const [first,second,expected]of definition.aliases)attempt('actual const alias '+first+'/'+second,()=>assert.equal(machine.testJavaScriptModuleBindingAlias(product(state,'initial state creation'),first,second),expected));
 const bindingReads=(plan?.bindings??definition.plan.bindings).map(binding=>({binding,result:attempt('binding read '+binding,()=>codecs.encodeJavaScriptModuleBindingRead(machine.readJavaScriptModuleBinding(product(plan,'actual model plan decode'),product(state,'initial state creation'),binding)))}));
 attempt('cached evaluation/error step performs no further IO',()=>{const transition=machine.stepJavaScriptModuleInitialization(product(plan,'actual model plan decode'),product(state,'initial state creation'));assert.equal(transition.$,definition.expectedPhase==='errored'?'moduleAbrupt':'moduleEvaluated');assert.deepEqual(codecs.encodeJavaScriptModuleInitializationState(transition.state),finalState);});
 return {definition,plan:definition.plan,initialState,events,finalState,requests,bindingReads,diagnostics:diagnostics(diagnostic)};
}

function files(directory,rows){for(const row of rows??[]){assert.match(row.path,/^[a-z][a-z-]*\.lino$/u);fs.writeFileSync(join(directory,row.path),Object.hasOwn(row,'text')?row.text:structuredDataToLino(row.data));}}
function input(definition){const parent=join(root,'issue-195-results/work/runtime-data-initialization');fs.mkdirSync(parent,{recursive:true});const directory=fs.mkdtempSync(join(parent,'case-'));fs.writeFileSync(join(directory,'package.json'),'{"type":"module"}');fs.writeFileSync(join(directory,'entry.js'),definition.source);files(directory,definition.files);return {directory,identifier:relative(root,join(directory,'entry.js')).replaceAll('\\','/'),url:pathToFileURL(join(directory,'entry.js')).href};}
async function realReads(directory,definition,read){
 const allowed=new Set(definition.files.map(row=>resolve(directory,row.path))),requests=[],original=fs.readFileSync;
 fs.readFileSync=function(file,...args){const value=Reflect.apply(original,this,[file,...args]),path=typeof file==='string'?resolve(file):file instanceof URL?fileURLToPath(file):'';if(allowed.has(path)){requests.push(basename(path));for(const update of definition.afterReads??[])if(update.after===requests.length)files(directory,[update]);}return value;};syncBuiltinESMExports();
 try{return {value:await read(),requests};}finally{fs.readFileSync=original;syncBuiltinESMExports();}
}
function runEntry(instance,name,args=[]){
 const execution=runJavaScriptSourceTransition(instance.embedding,instance.begin(name,args.map(instance.embedding.encode)),{steps:65536}),current=execution.transition;
 assert.equal(current.$,'machineCompleted',current.reason);return instance.embedding.decode(current.value);
}

export async function readRuntimeDataOriginalDiagnostic(definition){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt,attemptAsync=diagnostic.attemptAsync;
 const version=attempt('active Node/dependency qualification',readActiveRuntimeDataVersions),concrete=attempt('default CST exact source bytes/text/binary/AST projection',()=>readCollectionCopyConcreteControl(definition.source)),context=attempt('complete owned source/file setup',()=>input(definition));
 let capture,plan,initialState,events=[],finalState,bindingReads=[],ioRequests=[],functionCompilation=null,originalResult,originalRequests=[];const transports=[];
 try{
  const contract=attempt('actual owned IO adapter source/hash qualification',()=>{product(version,'active Node/dependency qualification');return readShippedRuntimeDataAdapterContract();});
  capture=attempt('complete actual runtime module source capture',()=>captureJavaScriptRuntimeDataModule(definition.source,{identifier:product(context,'complete owned source/file setup').identifier,readerContract:product(contract,'actual owned IO adapter source/hash qualification')}));
  const original=await attemptAsync('actual original module completion',async()=>{product(version,'active Node/dependency qualification');const input=product(context,'complete owned source/file setup');files(input.directory,definition.beforeInitialization);let namespace,error;const read=await realReads(input.directory,definition,async()=>{try{namespace=await import(input.url);}catch(value){error=value;}return namespace;});originalRequests=read.requests;
   if(definition.expectedError){assert.ok(error);for(const [key,value]of Object.entries(definition.expectedError))assert.equal(error[key],value);files(input.directory,definition.afterEvaluation);let repeated;try{await import(input.url);}catch(value){repeated=value;}assert.equal(repeated,error,'real ESM failure identity is cached');return {error};}
   assert.ok(!error,error?.message);const value=namespace.read();assert.equal(value,definition.expected);if(definition.alias)assert.equal(namespace.same(),true);if(definition.mutate){const loaded=namespace.value();loaded[definition.mutate.field]=definition.mutate.value;assert.equal(namespace.read(),definition.mutate.expected);}files(input.directory,definition.afterEvaluation);assert.equal(await import(input.url),namespace);return {value};});
  originalResult=original?.error?{error:Object.fromEntries(Object.keys(definition.expectedError).map(key=>[key,original.error[key]]))}:original&&Object.hasOwn(original,'value')?{value:original.value}:null;
  attempt('original literal IO order/count',()=>assert.deepEqual(originalRequests,definition.requests));
  const packet=attempt('semantic input Links encoding',()=>encodeStructuredDataLinks(product(capture,'complete actual runtime module source capture'))),text=attempt('semantic input Links text',()=>structuredDataToLino(product(capture,'complete actual runtime module source capture')));
  const variants=[capture,attempt('semantic source text reload',()=>readStructuredDataLino(product(text,'semantic input Links text'))),attempt('semantic source binary reload',()=>{const value=product(packet,'semantic input Links encoding');return decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(value.network)),value.root);})];
  for(let index=0;index<variants.length;index++){
   const label=['original','text','binary'][index],restored=variants[index];attempt(label+'/reset literal file inputs',()=>files(product(context,'complete owned source/file setup').directory,definition.files));
   const functions=attempt(label+'/actual captured source functions',()=>captureJavaScriptRuntimeDataFunctions(product(restored,label+' source input'))),authority=attempt(label+'/real source realm declarations',()=>({...createActualErrorSourceDeclarations(product(functions,label+'/actual captured source functions')),identity:1n})),scope=createRuntimeDataModuleScope();
   const instance=attempt(label+'/actual qualified module instance',()=>prepareJavaScriptRuntimeDataModule(product(restored,label+' source input'),product(authority,label+'/real source realm declarations'),{moduleURL:product(context,'complete owned source/file setup').url,scope,trace:true}));
   const before=attempt(label+'/actual initial state',()=>codecs.encodeJavaScriptModuleInitializationState(product(instance,label+'/actual qualified module instance').state()));
   if(index===0){plan=attempt('wire actual source plan',()=>codecs.encodeJavaScriptModuleInitializationPlan(product(restored,label+' source input').plan));initialState=before;}
   attempt(label+'/actual exported binding starts in TDZ',()=>{const item=product(restored,label+' source input').exports.find(row=>row.kind==='data');if(item)assert.equal(product(instance,label+'/actual qualified module instance').readExport(item.name).$,'moduleBindingUninitialized');});
   const initialized=await attemptAsync(label+'/real module IO evaluation',async()=>{const value=product(instance,label+'/actual qualified module instance'),input=product(context,'complete owned source/file setup');files(input.directory,definition.beforeInitialization);return realReads(input.directory,definition,()=>value.initialize());});
   attempt(label+'/literal actual IO order/count',()=>assert.deepEqual(product(initialized,label+'/real module IO evaluation').requests,definition.requests));
   const result=attempt(label+'/source completion and literal result',()=>{const value=product(instance,label+'/actual qualified module instance'),transition=product(initialized,label+'/real module IO evaluation').value;
    if(definition.expectedError){assert.equal(transition.$,'moduleAbrupt');const error=value.embedding.decode(transition.value);for(const [key,expected]of Object.entries(definition.expectedError))assert.equal(error[key],expected);return {error};}
    assert.equal(transition.$,'moduleEvaluated');const result=runEntry(value,'read');assert.equal(result,definition.expected);if(definition.alias)assert.equal(runEntry(value,'same'),true);if(definition.mutate){const loaded=runEntry(value,'value');loaded[definition.mutate.field]=definition.mutate.value;assert.equal(runEntry(value,'read'),definition.mutate.expected);}return {value:result};});
   if(!definition.expectedError){
    const compiledPlan=attempt(label+'/wire function capture plan',()=>errorCodecs.encodeJavaScriptErrorModulePlan(product(instance,label+'/actual qualified module instance').embedding.module.capture.plan));
    const registry=attempt(label+'/wire actual callable registry',()=>errorCodecs.encodeJavaScriptErrorModuleRegistry(product(instance,label+'/actual qualified module instance').embedding.module.registry));
    const bindings=attempt(label+'/wire actual initialized binding values',()=>product(instance,label+'/actual qualified module instance').state().slots.map(slot=>errorCodecs.encodeJavaScriptErrorModuleBinding({$:'errorModuleBindingValue',identity:slot.binding,value:slot.value})));
    const expected=attempt(label+'/wire actual source compilation',()=>errorCodecs.encodeJavaScriptErrorModuleCompilation({$:'errorModuleCompilation',program:product(instance,label+'/actual qualified module instance').embedding.module.program,error:''}));
    if(index===0)functionCompilation={plan:compiledPlan,registry,bindings,expected};
   }
   attempt(label+'/later file writes do not reinitialize a module',()=>{files(product(context,'complete owned source/file setup').directory,definition.afterEvaluation);const value=product(instance,label+'/actual qualified module instance'),count=value.requests().length,repeated=value.initialize();assert.equal(value.requests().length,count);if(definition.expectedError){assert.equal(repeated.$,'moduleAbrupt');assert.equal(value.embedding.decode(repeated.value),product(result,label+'/source completion and literal result').error);}else assert.equal(runEntry(value,'read'),definition.mutate?definition.mutate.expected:definition.expected);});
   attempt(label+'/same actual module and realm reuse one instance',()=>assert.equal(prepareJavaScriptRuntimeDataModule(product(restored,label+' source input'),product(authority,label+'/real source realm declarations'),{moduleURL:product(context,'complete owned source/file setup').url,scope}),product(instance,label+'/actual qualified module instance')));
   const state=attempt(label+'/wire final state',()=>codecs.encodeJavaScriptModuleInitializationState(product(instance,label+'/actual qualified module instance').state()));
   const history=attempt(label+'/every actual initialization event',()=>product(instance,label+'/actual qualified module instance').trace().map(event=>({operation:event.operation,transition:codecs.encodeJavaScriptModuleInitializationTransition(event.transition),value:event.value===null?null:codecs.encodeJavaScriptSemanticValue(event.value),abrupt:event.abrupt})));
   const reads=(restored?.plan.bindings??[]).map(binding=>({binding,result:attempt(label+'/actual binding read '+binding,()=>{const value=product(instance,label+'/actual qualified module instance');return codecs.encodeJavaScriptModuleBindingRead(machine.readJavaScriptModuleBinding(product(restored,label+' source input').plan,value.state(),binding));})}));
   if(index===0){finalState=state;events=history??[];bindingReads=reads;ioRequests=attempt('actual qualified IO requests',()=>product(instance,label+'/actual qualified module instance').requests())??[];}
   transports.push({transport:label,finalState:state,result:result?.error?{error:Object.fromEntries(Object.keys(definition.expectedError).map(key=>[key,result.error[key]]))}:result&&Object.hasOwn(result,'value')?{value:result.value}:null});
  }
 }finally{if(context)attempt('release owned source control scratch',()=>fs.rmSync(context.directory,{recursive:true,force:true}));}
 return {definition,concrete,capture,plan,initialState,events,finalState,bindingReads,ioRequests,functionCompilation,originalResult,originalRequests,transports,diagnostics:diagnostics(diagnostic)};
}

export function readRuntimeDataRefusalDiagnostic(definition){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt,version=attempt('active Node/dependency qualification',readActiveRuntimeDataVersions),concrete=attempt('default CST exact source bytes/text/binary/AST projection',()=>readCollectionCopyConcreteControl(definition.source));
 attempt('explicit complete source refusal',()=>{product(version,'active Node/dependency qualification');const contract=readShippedRuntimeDataAdapterContract();assert.throws(()=>captureJavaScriptRuntimeDataFunctions(captureJavaScriptRuntimeDataModule(definition.source,{identifier:'issue-195-results/work/runtime-data-initialization/case-refusal/entry.js',readerContract:contract})));});
 return {definition,concrete,diagnostics:diagnostics(diagnostic)};
}
