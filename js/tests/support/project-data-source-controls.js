// Future bounded diagnostic producer. Missing peer/codecs products fail their
// own cells; every original input and every assertion remains represented.
import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {readProjectData,resolveProjectDataPath} from '../../src/project-data-files.js';
import {structuredDataToLino,readStructuredDataLino,encodeStructuredDataLinks,decodeStructuredDataLinks} from '../../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
import {runJavaScriptSourceTransition} from '../../src/node-source-modules.js';
import {captureJavaScriptProjectDataSource,checkJavaScriptProjectDataSource} from '../../src/translation/javascript-project-data-source.js';
import {readShippedJavaScriptProjectDataSource,prepareJavaScriptProjectDataModule} from '../../src/translation/javascript-project-data-module.js';
import {createControlDiagnostics,requireControlProduct as product} from './control-diagnostics.js';
import {encodeSourceModuleCompilation} from './source-module-compilation.js';
import {readCollectionCopyConcreteControl} from './collection-copy-concrete.js';
import * as codecs from '../../../experiments/javascript-properties/error-class-source-codecs.mjs';

export function readProjectDataSourceOriginalDiagnostic(definition){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 const capture=attempt('complete actual provider source capture',readShippedJavaScriptProjectDataSource);
 const concrete=attempt('complete provider default CST/text/binary/AST projection',()=>readCollectionCopyConcreteControl(product(capture,'complete actual provider source capture').source));
 const directory=attempt('owned literal filesystem input',()=>{const directory=mkdtempSync(join(tmpdir(),'meta-project-data-control-'));try{for(const file of definition.files)writeFileSync(join(directory,file.name),Object.hasOwn(file,'text')?file.text:structuredDataToLino(file.data),'utf8');return directory;}catch(error){rmSync(directory,{recursive:true,force:true});throw error;}});
 let originalResult,originalError;const transports=[];
 try{
  const file=attempt('actual filename or native URL',()=>{const path=join(product(directory,'owned literal filesystem input'),definition.file);return definition.asURL?pathToFileURL(path):path;});
  attempt('original complete provider completion',()=>{const read=definition.entry==='readProjectData'?readProjectData:resolveProjectDataPath;try{originalResult=read(product(file,'actual filename or native URL'),definition.options);}catch(error){originalError=error;}assert.equal(Boolean(originalError),Boolean(definition.expectedError));});
  attempt('original literal value/throw oracle',()=>assertResult(definition,originalResult,originalError,product(directory,'owned literal filesystem input')));
  for(const transport of ['original','text','binary']){
   const label='source input '+transport;
   const restored=attempt(label+'/complete source receipt restoration',()=>{
    const actual=product(capture,'complete actual provider source capture');if(transport==='original')return actual;
    // These are source-plan INPUT archives, not checked Core programs.
    if(transport==='text')return readStructuredDataLino(structuredDataToLino(actual));
    const packet=encodeStructuredDataLinks(actual);return decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root);
   });
   attempt(label+'/actual complete CST recapture',()=>assert.deepEqual(checkJavaScriptProjectDataSource(product(restored,label+'/complete source receipt restoration')),capture));
   const instance=attempt(label+'/qualified VM/provider publication',()=>prepareJavaScriptProjectDataModule(product(restored,label+'/complete source receipt restoration'),{realmIdentity:19n,trace:true}));
   const compilation=attempt(label+'/actual capability source compilation transport',()=>{const module=product(instance,label+'/qualified VM/provider publication').embedding.module;return encodeSourceModuleCompilation(module);});
   const completion=attempt(label+'/product source runner',()=>{const actual=product(instance,label+'/qualified VM/provider publication'),args=[actual.encode(product(file,'actual filename or native URL'))];if(Object.hasOwn(definition,'options'))args.push(actual.encode(definition.options));const transition=runJavaScriptSourceTransition(actual.embedding,actual.begin(definition.entry,args),{steps:1048576}).transition;assert.ok(['machineCompleted','machineThrow'].includes(transition.$),transition.reason??transition.$);return transition;});
   let result,error;
   attempt(label+'/literal value/throw oracle',()=>{const actual=product(instance,label+'/qualified VM/provider publication'),transition=product(completion,label+'/product source runner');if(transition.$==='machineThrow')error=actual.embedding.decode(transition.value);else result=actual.embedding.materializeData(transition.value);assertResult(definition,result,error,product(directory,'owned literal filesystem input'));});
   const events=attempt(label+'/actual capability requests retained',()=>product(instance,label+'/qualified VM/provider publication').events().map(row=>({...row,arguments:row.arguments.map(codecs.encodeJavaScriptSemanticValue)})));
   attempt(label+'/original and portable completion parity',()=>{assert.equal(error?.name,originalError?.name);assert.equal(error?.message,originalError?.message);assert.deepEqual(result,originalResult);});
   transports.push({transport,capture:restored,compilation,events,result:result??null,error:error?{name:error.name,message:error.message}:null});
  }
 }finally{attempt('owned input cleanup',()=>{if(directory!==undefined)rmSync(directory,{recursive:true,force:true});});}
 return {definition,capture,concrete,originalResult:originalResult??null,originalError:originalError?{name:originalError.name,message:originalError.message}:null,transports,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};
}
function assertResult(row,result,error,directory){
 if(row.expectedError){assert.ok(error);assert.equal(error.name,row.expectedError.name);if(Object.hasOwn(row.expectedError,'message'))assert.equal(error.message,row.expectedError.message);else assert.equal(error.message.startsWith(row.expectedError.messagePrefix),true);}
 else{assert.equal(error,undefined);assert.deepEqual(result,Object.hasOwn(row,'expectedPath')?join(directory,row.expectedPath):row.expected);}
}
export function readProjectDataSourceRefusalDiagnostic(definition){const diagnostic=createControlDiagnostics();diagnostic.attempt('complete source refusal',()=>assert.throws(()=>captureJavaScriptProjectDataSource(definition.source,{identifier:'literal-fixture.js',entries:definition.entries}),error=>error instanceof TypeError&&error.message.includes(definition.reason)));const concrete=diagnostic.attempt('refused input default CST exact bytes',()=>readCollectionCopyConcreteControl(definition.source));return {definition,concrete,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};}

export function readProjectDataSourceCacheDiagnostic(definition){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 const resource=attempt('actual CI large-data resource profile',()=>{assert.equal(process.env.CI,'true');return true;});
 const directory=attempt('owned large-data input directory',()=>{product(resource,'actual CI large-data resource profile');return mkdtempSync(join(tmpdir(),'meta-project-data-cache-control-'));});
 const instance=attempt('actual source cache provider',()=>prepareJavaScriptProjectDataModule(readShippedJavaScriptProjectDataSource(),{realmIdentity:31n,trace:true}));
 const values=[],roots=[],nested=[],originalValues=[];
 const payload=value=>definition.prefixUnit.repeat(definition.prefixCount)+structuredDataToLino(value);
 try{
  attempt('exact independently declared first bytes',()=>writeFileSync(join(product(directory,'owned large-data input directory'),'large.lino'),payload(definition.first)));
  for(let index=0;index<3;index++)attempt('portable cache call '+index,()=>{
   const path=join(product(directory,'owned large-data input directory'),'large.lino');if(index===2)writeFileSync(path,payload(definition.afterEdit));const actual=product(instance,'actual source cache provider');
   const transition=runJavaScriptSourceTransition(actual.embedding,actual.begin('readProjectData',[actual.encode(path)]),{steps:1048576}).transition;assert.equal(transition.$,'machineCompleted',transition.reason);roots.push(transition.value.identity.toString());
   const value=actual.embedding.decode(transition.value);values.push(value.nested.value);nested.push(actual.embedding.encode(value.nested).identity.toString());if(index===0)value.nested.value='caller mutation';
  });
  attempt('portable literal values across caller mutation and external edit',()=>assert.deepEqual(values,definition.expectedValues));
  attempt('fresh actual root handles across cache reuse',()=>assert.equal(roots.length===3&&new Set(roots).size===roots.length,definition.expectedDistinctRootHandles));
  attempt('fresh actual nested handles across cache reuse',()=>assert.equal(nested.length===3&&new Set(nested).size===nested.length,definition.expectedDistinctNestedHandles));
  attempt('current text read on every source call',()=>assert.equal(product(instance,'actual source cache provider').events().filter(row=>row.operation==='import node:fs readFileSync').length,definition.expectedReads));
  attempt('decoder called only for first data and changed current text',()=>assert.equal(product(instance,'actual source cache provider').events().filter(row=>row.operation==='import ./structured-data-links.js readStructuredDataLino').length,definition.expectedDecodes));
  attempt('original independently reset complete bytes',()=>writeFileSync(join(product(directory,'owned large-data input directory'),'large.lino'),payload(definition.first)));
  for(let index=0;index<3;index++)attempt('original cache call '+index,()=>{const path=join(product(directory,'owned large-data input directory'),'large.lino');if(index===2)writeFileSync(path,payload(definition.afterEdit));const value=readProjectData(path);originalValues.push(value.nested.value);if(index===0)value.nested.value='caller mutation';});
  attempt('original literal values across caller mutation and external edit',()=>assert.deepEqual(originalValues,definition.expectedValues));
 }finally{attempt('owned large-data cleanup',()=>{if(directory!==undefined)rmSync(directory,{recursive:true,force:true});});}
 return {definition,values,roots,nested,originalValues,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};
}
