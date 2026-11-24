// Deferred controls only: do not run during the source-drafting phase.
import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {readProjectData,resolveProjectDataPath} from '../src/project-data-files.js';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {captureJavaScriptProjectDataSource,checkJavaScriptProjectDataSource} from '../src/translation/javascript-project-data-source.js';
import {readShippedJavaScriptProjectDataSource,prepareJavaScriptProjectDataModule} from '../src/translation/javascript-project-data-module.js';
import {projectDataSourceOriginals,projectDataSourceRefusals,projectDataSourceCacheOriginals} from './fixtures/project-data-source-execution.mjs';
import {runtimeDataModuleOriginals} from './fixtures/runtime-data-module-initialization.mjs';
import {readRuntimeDataOriginalDiagnostic} from './support/runtime-data-module-controls.js';
import {finishControlDiagnostics} from './support/control-diagnostics.js';

for(const row of projectDataSourceOriginals)test(row.name,()=>{
 const directory=mkdtempSync(join(tmpdir(),'meta-project-data-source-'));
 try{
  for(const file of row.files)writeFileSync(join(directory,file.name),Object.hasOwn(file,'text')?file.text:structuredDataToLino(file.data),'utf8');
  const capture=readShippedJavaScriptProjectDataSource(),instance=prepareJavaScriptProjectDataModule(capture,{realmIdentity:13n,trace:true});
  assert.deepEqual(checkJavaScriptProjectDataSource(capture),capture);
  assert.equal(capture.plan.functions.some(fn=>fn.$==='errorStatementFunctionPlan'),true);
  assert.equal(instance.qualification.providerSourceSha256,capture.sourceSha256);
  const path=join(directory,row.file),file=row.asURL?pathToFileURL(path):path,args=[instance.encode(file)];
  if(Object.hasOwn(row,'options'))args.push(instance.encode(row.options));
  const transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,args),{steps:1048576}).transition;
  const original=row.entry==='readProjectData'?readProjectData:resolveProjectDataPath;
  if(row.expectedError){
   assert.equal(transition.$,'machineThrow',transition.reason);const error=instance.embedding.decode(transition.value);
   assert.equal(error.name,row.expectedError.name);if(row.expectedError.message!==undefined)assert.equal(error.message,row.expectedError.message);else assert.equal(error.message.startsWith(row.expectedError.messagePrefix),true);
   assert.throws(()=>original(file,row.options),actual=>actual.name===row.expectedError.name&&(row.expectedError.message!==undefined?actual.message===row.expectedError.message:actual.message.startsWith(row.expectedError.messagePrefix)));
  }else{
   assert.equal(transition.$,'machineCompleted',transition.reason);const actual=instance.embedding.materializeData(transition.value),expected=Object.hasOwn(row,'expectedPath')?join(directory,row.expectedPath):row.expected;
   assert.deepEqual(actual,expected);assert.deepEqual(original(file,row.options),expected);
  }
  assert.ok(instance.events().length>0);
  const forged=structuredClone(capture);forged.plan.functions[0].name+='Changed';assert.throws(()=>checkJavaScriptProjectDataSource(forged),/complete CST/);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
for(const row of projectDataSourceRefusals)test(row.name,()=>{
 assert.throws(()=>captureJavaScriptProjectDataSource(row.source,{identifier:'literal-fixture.js',entries:row.entries}),error=>error instanceof TypeError&&error.message.includes(row.reason));
});
for(const row of runtimeDataModuleOriginals)test('actual loader/provider source importer: '+row.name,async()=>finishControlDiagnostics({name:row.name,...await readRuntimeDataOriginalDiagnostic(row,{readerProfile:'actual project data source functions 1',readerRealmIdentity:2n})}));
test('opaque native URL ownership cannot be retagged by an edited heap',()=>{
 const instance=prepareJavaScriptProjectDataModule(readShippedJavaScriptProjectDataSource(),{realmIdentity:37n,trace:true}),url=instance.encode(new URL('file:///unread-project-data.json'));
 const heap=structuredClone(instance.embedding.heap()),row=heap.objects.find(row=>row.identity===url.identity);assert.equal(row.$,'hostURLObject');row.$='hostArrayObject';instance.embedding.adoptHeap(heap);
 assert.throws(()=>runJavaScriptSourceTransition(instance.embedding,instance.begin('readProjectData',[url]),{steps:1048576}),/cannot replace an actual opaque host object/u);assert.deepEqual(instance.events(),[]);
});
for(const row of projectDataSourceCacheOriginals)test(row.name,()=>{
 assert.equal(process.env.CI,'true','large source-reader controls run only in the CI resource profile');
 const directory=mkdtempSync(join(tmpdir(),'meta-project-data-cache-'));
 try{
  const path=join(directory,'large.lino'),payload=value=>row.prefixUnit.repeat(row.prefixCount)+structuredDataToLino(value);
  writeFileSync(path,payload(row.first));const instance=prepareJavaScriptProjectDataModule(readShippedJavaScriptProjectDataSource(),{realmIdentity:31n,trace:true}),values=[],handles=[],nested=[];
  for(let index=0;index<3;index++){
   if(index===2)writeFileSync(path,payload(row.afterEdit));
   const transition=runJavaScriptSourceTransition(instance.embedding,instance.begin('readProjectData',[instance.encode(path)]),{steps:1048576}).transition;
   assert.equal(transition.$,'machineCompleted',transition.reason);handles.push(transition.value.identity);
   const record=instance.embedding.decode(transition.value);values.push(record.nested.value);nested.push(instance.embedding.encode(record.nested).identity);
   if(index===0)record.nested.value='caller mutation';
  }
  assert.deepEqual(values,row.expectedValues);assert.equal(new Set(handles).size===handles.length,row.expectedDistinctRootHandles);assert.equal(new Set(nested).size===nested.length,row.expectedDistinctNestedHandles);
  const events=instance.events();assert.equal(events.filter(event=>event.operation==='import node:fs readFileSync').length,row.expectedReads);assert.equal(events.filter(event=>event.operation==='import ./structured-data-links.js readStructuredDataLino').length,row.expectedDecodes);
  writeFileSync(path,payload(row.first));const original=[];for(let index=0;index<3;index++){if(index===2)writeFileSync(path,payload(row.afterEdit));const result=readProjectData(path);original.push(result.nested.value);if(index===0)result.nested.value='caller mutation';}assert.deepEqual(original,row.expectedValues);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
