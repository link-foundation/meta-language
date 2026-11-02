// Future originals and native replay inputs. Failures are accumulated per
// assertion/transport; declarative source plans remain INPUT archives.
import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from '../../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../../src/translation/javascript-ordinary-class-module.js';
import {structuredDataToLino,readStructuredDataLino,encodeStructuredDataLinks,decodeStructuredDataLinks} from '../../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
import {createControlDiagnostics,requireControlProduct as product} from './control-diagnostics.js';
import {readCollectionCopyConcreteControl} from './collection-copy-concrete.js';
import {readSourceMachineTrace} from './source-machine-trace.js';
import {encodeSourceModuleCompilation} from './source-module-compilation.js';
import * as codecs from '../../../experiments/javascript-properties/error-class-source-codecs.mjs';
import * as collectionCodecs from '../../../experiments/javascript-properties/collection-call-source-codecs.mjs';

export function readOrdinaryClassSourceOriginalDiagnostic(definition,{prepare=prepareJavaScriptOrdinaryClassModule,originalCompletion,observeResult}={}){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 const concrete=attempt('original full source default CST/text/binary/AST projection',()=>readCollectionCopyConcreteControl(definition.source));
 const capture=attempt('complete original class/member/lexical source ownership',()=>captureJavaScriptOrdinaryClassModule(definition.source,{identifier:'literal-ordinary-class.js',entries:definition.entries}));
 let originalResult,originalError,originalArgumentState;const transports=[];
 attempt('original engine source completion',()=>{
  const actual=product(capture,'complete original class/member/lexical source ownership'),args=structuredClone(definition.arguments);
  if(originalCompletion){if(originalCompletion.unavailable)throw new TypeError('required original ESM completion unavailable');originalResult=originalCompletion.result;originalError=originalCompletion.error;originalArgumentState=originalCompletion.argumentState;}
  else{const source=actual.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>actual.source.slice(origin.start,origin.end)).join('\n'),entry=compileFunction('"use strict";'+source+'\nreturn '+definition.entry+';',[])();try{originalResult=entry(...args);}catch(error){originalError=error;}}
  if(originalError===undefined&&observeResult!==undefined)originalResult=observeResult(originalResult);
  if(!originalCompletion)originalArgumentState=args;
  assertResult(definition,originalResult,originalError);if(definition.expectedUnchangedArgument)assert.deepEqual(originalArgumentState[0],definition.expectedUnchangedArgument);
  if(Object.hasOwn(definition,'expectedArgumentState'))assert.deepEqual(originalArgumentState,definition.expectedArgumentState);
 });
 for(const transport of ['original','text','binary']){
  const label=transport,restored=attempt(label+'/complete source input restoration',()=>{
   const actual=product(capture,'complete original class/member/lexical source ownership');if(transport==='original')return actual;if(transport==='text')return readStructuredDataLino(structuredDataToLino(actual));const packet=encodeStructuredDataLinks(actual);return decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root);
  });
  attempt(label+'/actual source/member recapture',()=>assert.deepEqual(checkJavaScriptOrdinaryClassModule(product(restored,label+'/complete source input restoration')),capture));
  const instance=attempt(label+'/actual class/prototype/standard-collection publication',()=>prepare(product(restored,label+'/complete source input restoration'),{realmIdentity:53n}));
  const sourceCompilation=attempt(label+'/actual source compiler roots',()=>encodeSourceModuleCompilation(product(instance,label+'/actual class/prototype/standard-collection publication').embedding.module));
  const collectionCompilation=attempt(label+'/actual standard operation compiler roots',()=>{const value=product(instance,label+'/actual class/prototype/standard-collection publication').collection;return {directory:collectionCodecs.encodeJavaScriptCollectionCallableDirectory(value.directory),expected:collectionCodecs.encodeJavaScriptCollectionCallCompilation({$:'collectionCallCompilation',program:value.program,error:''})};});
  const combinedProgram=attempt(label+'/actual complete joined portable program',()=>codecs.encodeJavaScriptCallProgram(product(instance,label+'/actual class/prototype/standard-collection publication').embedding.module.program));
  const arguments_=attempt(label+'/exact original literal argument values',()=>{const value=product(instance,label+'/actual class/prototype/standard-collection publication');return structuredClone(definition.arguments).map(value.embedding.encode);});
  const trace=attempt(label+'/actual portable source run and every external response',()=>{const value=product(instance,label+'/actual class/prototype/standard-collection publication');return readSourceMachineTrace(value.embedding,value.begin(definition.entry,product(arguments_,label+'/exact original literal argument values')),{steps:65536});});
  if(definition.expectedBatchResumption)attempt(label+'/declared bounded pure batch resumption',()=>assert.equal(product(trace,label+'/actual portable source run and every external response').events.some(event=>event.operation==='resumeBatch'),true));
  let result,error,argumentState;
  attempt(label+'/independent literal completion oracle',()=>{const value=product(instance,label+'/actual class/prototype/standard-collection publication'),execution=product(trace,label+'/actual portable source run and every external response');assert.equal(execution.budgetExhausted,false);const transition=execution.transition;assert.equal(transition.$,definition.expectedError?'machineThrow':'machineCompleted',transition.reason??transition.$);if(transition.$==='machineThrow')error=value.embedding.decode(transition.value);else{result=value.embedding.materializeData(transition.value);if(observeResult!==undefined)result=observeResult(result);}assertResult(definition,result,error);});
  attempt(label+'/original and portable completion parity',()=>{assert.deepEqual(result,originalResult);assert.equal(error?.name,originalError?.name);assert.equal(error?.message,originalError?.message);});
  if(definition.expectedUnchangedArgument)attempt(label+'/failed ordinary class call has no constructor/default effect',()=>assert.deepEqual(product(instance,label+'/actual class/prototype/standard-collection publication').embedding.materializeData(product(arguments_,label+'/exact original literal argument values')[0]),definition.expectedUnchangedArgument));
  if(Object.hasOwn(definition,'expectedArgumentState'))attempt(label+'/all actual argument effects after completion',()=>{const actual=product(instance,label+'/actual class/prototype/standard-collection publication');argumentState=product(arguments_,label+'/exact original literal argument values').map(value=>actual.embedding.materializeData(value));assert.deepEqual(argumentState,definition.expectedArgumentState);assert.deepEqual(argumentState,originalArgumentState);});
  transports.push({transport,capture:restored,sourceCompilation,collectionCompilation,combinedProgram,trace:trace?{initial:trace.initial,events:trace.events,budgetExhausted:trace.budgetExhausted,budget:trace.budget,used:trace.used,final:trace.final}:undefined,...(Object.hasOwn(definition,'expectedArgumentState')?{argumentState}:{}),result:result??null,error:error?{name:error.name,message:error.message}:null});
 }
 return {definition,capture,concrete,...(Object.hasOwn(definition,'expectedArgumentState')?{originalArgumentState}:{}),originalResult:originalResult??null,originalError:originalError?{name:originalError.name,message:originalError.message}:null,transports,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};
}
function assertResult(definition,result,error){if(definition.expectedError){assert.ok(error);for(const [key,value]of Object.entries(definition.expectedError))assert.equal(error[key],value);}else{assert.equal(error,undefined);assert.deepEqual(result,definition.expected);}}
export function readOrdinaryClassSourceRefusalDiagnostic(definition){const diagnostic=createControlDiagnostics();diagnostic.attempt('complete original source refusal',()=>assert.throws(()=>captureJavaScriptOrdinaryClassModule(definition.source,{identifier:'literal-ordinary-class-refusal.js',entries:definition.entries})));const concrete=diagnostic.attempt('refused source default CST/text/binary/AST projection',()=>readCollectionCopyConcreteControl(definition.source));return {definition,concrete,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};}
