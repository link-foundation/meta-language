import assert from 'node:assert/strict';
import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,dirname} from 'node:path';
import {pathToFileURL} from 'node:url';
import {analyzeProgram} from '../../src/program-representation.js';
import {readSuppliedSourceLibraryClosure,sourceLibraryLogicalModuleURL} from '../../src/translation/source-library-closure.js';
import {captureJavaScriptOrdinaryClassSourceClosure} from '../../src/translation/javascript-ordinary-class-closure-source.js';
import {prepareJavaScriptOrdinaryClassSourceClosure} from '../../src/translation/javascript-ordinary-class-closure.js';
import {createControlDiagnostics,requireControlProduct as product} from './control-diagnostics.js';
import {readCollectionCopyConcreteControl} from './collection-copy-concrete.js';
import {readOrdinaryClassSourceOriginalDiagnostic} from './ordinary-class-source-controls.js';
const root=new URL('../../../',import.meta.url),options={aliases:true,classes:'closed local class declarations 1',reExports:'resolved named module providers 1'};

export async function readOrdinaryClassClosureOriginalDiagnostic(definition){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt,attemptAsync=diagnostic.attemptAsync;
 const modules=attempt('complete supplied original module bytes',()=>definition.modules??definition.paths.map(path=>({path,source:readFileSync(new URL(path,root),'utf8')})));
 const moduleConcrete=(modules??definition.modules??[]).map(module=>({path:module.path,concrete:attempt('original module '+module.path+'/default CST exact bytes',()=>readCollectionCopyConcreteControl(module.source)))}));
 const archive=attempt('actual complete supplied CST alias/re-export composition',()=>captureJavaScriptOrdinaryClassSourceClosure(readSuppliedSourceLibraryClosure(definition.entryPath,product(modules,'complete supplied original module bytes'),options),{entryPath:definition.entryPath}));
 const sourceAnchor=definition.scopeSource?attempt('exact whole original Scope declaration anchor',()=>{
  const selected=product(modules,'complete supplied original module bytes').find(module=>module.path===definition.scopeSource),original=readFileSync(new URL(definition.completeOriginalScopeModule,root),'utf8'),program=analyzeProgram(selected.source,'JavaScript'),binding=program.bindings.find(row=>row.kind==='class'&&row.name===definition.className),syntax=binding&&program.sourceMappings.find(node=>node.term==='class_declaration'&&node.start<=binding.declaration.start&&node.end>=binding.declaration.end);assert.ok(syntax);const declaration=selected.source.slice(syntax.start,syntax.end);assert.ok(original.includes(declaration));return {path:definition.completeOriginalScopeModule,source:original,declaration,concrete:readCollectionCopyConcreteControl(original)};
 }):null;
 let directory;
 const original=await attemptAsync('genuine original complete ESM module completion',async()=>{
  directory=mkdtempSync(join(tmpdir(),'meta-ordinary-class-modules-'));writeFileSync(join(directory,'package.json'),'{"type":"module"}');
  for(const module of product(modules,'complete supplied original module bytes')){sourceLibraryLogicalModuleURL(module.path);const target=join(directory,module.path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,module.source,'utf8');}
  let namespace,result,error;try{namespace=await import(pathToFileURL(join(directory,definition.entryPath)).href);result=namespace[definition.entry](...structuredClone(definition.arguments));}catch(value){error=value;}
  if(definition.expectedError){assert.ok(error);for(const [key,value]of Object.entries(definition.expectedError))assert.equal(error[key],value);}else{assert.equal(error,undefined);assert.deepEqual(result,definition.expected);}return {result,error};
 });
 let record;
 try{
  const actual=product(archive,'actual complete supplied CST alias/re-export composition'),entry=actual.entries.find(row=>row.exported===definition.entry);if(!entry)throw new TypeError('the literal public entry needs its actual composed source owner');
  const sourceDefinition={...definition,source:actual.capture.source,entries:actual.capture.entries,entry:entry.compiledName,publicEntry:definition.entry};
  record=readOrdinaryClassSourceOriginalDiagnostic(sourceDefinition,{originalCompletion:original??{unavailable:true},prepare(restored,{realmIdentity}){assert.deepEqual(restored,actual.capture);const linked=prepareJavaScriptOrdinaryClassSourceClosure({...actual,capture:restored},{realmIdentity});return {...linked,begin(compiled,args){const selected=actual.entries.find(row=>row.compiledName===compiled);if(!selected)throw new TypeError('a composed class call retains its actual public export identity');return linked.begin(selected.exported,args);}};}});
  record={...record,definition,sourceDefinition,sourceClosureArchive:actual,moduleConcrete,sourceAnchor};
 }catch(error){diagnostic.attempt('complete composed class control construction',()=>{throw error;});record={definition,sourceClosureArchive:archive,moduleConcrete,sourceAnchor,transports:[]};}
 finally{attempt('owned original ESM input cleanup',()=>{if(directory!==undefined)rmSync(directory,{recursive:true,force:true});});}
 record.diagnostics={assertions:[...diagnostic.assertions,...(record.diagnostics?.assertions??[])],failures:[...diagnostic.failures,...(record.diagnostics?.failures??[])]};return record;
}
