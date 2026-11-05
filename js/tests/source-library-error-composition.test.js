// Deferred original modules/semantic frontend controls. No controls ran while
// authoring this file; all failures remain distinct from source declarations.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {join,relative} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import * as rules from '../src/translation/source-library-error-rules.js';
import {readSourceLibraryClosure,readSuppliedSourceLibraryClosure} from '../scripts/read-source-library-closure.mjs';
import {captureSourceLibraryErrorClosure,checkSourceLibraryErrorClosure} from '../scripts/source-library-errors.mjs';
import {prepareJavaScriptErrorSourceClosure,beginJavaScriptErrorSourceClosure} from '../src/translation/javascript-error-closure.js';
import {createJavaScriptErrorRealm} from '../src/translation/javascript-error-realm.js';
import {structuredDataToLino,readStructuredDataLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readCollectionCopyConcreteControl} from './support/collection-copy-concrete.js';
import {createControlDiagnostics,finishControlDiagnostics,requireControlProduct as product} from './support/control-diagnostics.js';
import {createActualErrorSourceDeclarations,executeErrorSourceClosure} from './support/error-source-module-controls.js';
import {sourceLibraryErrorDecisions as decisions,sourceLibraryErrorOriginals as originals,sourceLibraryErrorRefusals as refusals,sourceLibraryErrorSourceAnchors as anchors} from './fixtures/source-library-error-composition.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url)),model='resolved supplied Error source modules 1',digest=source=>createHash('sha256').update(source).digest('hex');
function versions(){const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node);}
function supplied(row,context){
 const parent=join(root,'issue-195-results/work/source-library-error-composition');mkdirSync(parent,{recursive:true});const directory=mkdtempSync(join(parent,'case-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));writeFileSync(join(directory,'package.json'),'{"type":"module"}');
 for(const file of row.files){assert.match(file.path,/^[a-z][a-z-]*\.js$/u);writeFileSync(join(directory,file.path),file.source);}return {directory,entry:relative(root,join(directory,row.entry)).replaceAll('\\','/')};
}
function inspect(value,expected){const result={};for(const key of Object.keys(expected))result[key]=value[key];assert.deepEqual(result,expected);}
test('Error linkage source rules retain explicit actual module/entry owners',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);});
test('whole actual diagnostics and ordinary frontend import anchors remain exact',()=>{for(const row of anchors)assert.ok(readFileSync(join(root,row.path),'utf8').includes(row.source),row.path);});
for(const row of originals)test(row.name,async context=>{
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;attempt('active version coverage',versions);for(const file of row.files)attempt(file.path+'/default CST and exact bytes/codecs',()=>readCollectionCopyConcreteControl(file.source));
 const input=supplied(row,context),original=await import(pathToFileURL(join(input.directory,row.entry)).href);
 const before=attempt('original importer actual completion',()=>{if(row.abrupt){let error;try{original[row.exportName](...row.args);}catch(value){error=value;}assert.ok(error);return error;}return original[row.exportName](...row.args);});
 attempt('original literal result',()=>{const value=product(before,'original importer actual completion');if(row.result==='function'){assert.equal(typeof value,'function');assert.equal(value.name,row.expectedName);}else if(Object.hasOwn(row,'expectedScalar'))assert.equal(value,row.expectedScalar);else inspect(value,row.expected);});
 if(row.followup)attempt('original source value/callback followup',()=>{const value=original[row.followup.exportName](product(before,'original importer actual completion'),...row.followup.args);if(typeof row.followup.expected==='string')assert.equal(value,row.followup.expected);else inspect(value,row.followup.expected);});
 if(row.identityFollowup)attempt('original constructor identity',()=>assert.equal(original[row.identityFollowup](product(before,'original importer actual completion')),true));
 attempt('old function-only closure gate',()=>assert.throws(()=>readSourceLibraryClosure(input.entry,{aliases:true})));
 const closure=attempt('actual supplied Error source closure',()=>readSourceLibraryClosure(input.entry,{aliases:true,errorClasses:model,reExports:row.reExports??''})),archive=attempt('actual Error module/entry ownership',()=>captureSourceLibraryErrorClosure(product(closure,'actual supplied Error source closure')));
 attempt('same supplied-CST alias pipeline without file IO',()=>{const value=product(closure,'actual supplied Error source closure');assert.deepEqual(readSuppliedSourceLibraryClosure(value.entryPath,value.inputs.map(input=>({path:input.path,source:input.source})),{aliases:true,errorClasses:model,reExports:row.reExports??''}),value);});
 attempt('complete original source receipts',()=>{const value=product(archive,'actual Error module/entry ownership');assert.equal(value.completeSourceReceipts.length,row.files.length);for(const receipt of value.completeSourceReceipts){const file=row.files.find(file=>receipt.path.endsWith('/'+file.path));assert.equal(receipt.source,file.source);assert.equal(receipt.sourceSha256,digest(file.source));assert.equal(receipt.bytes,Buffer.byteLength(file.source));}});
 const packet=attempt('semantic closure Links construction',()=>encodeStructuredDataLinks(product(archive,'actual Error module/entry ownership'))),text=attempt('semantic closure Links text',()=>structuredDataToLino(product(archive,'actual Error module/entry ownership')));
 const variants=[archive,attempt('semantic closure text reload',()=>readStructuredDataLino(product(text,'semantic closure Links text'))),attempt('semantic closure binary reload',()=>{const value=product(packet,'semantic closure Links construction');return decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(value.network)),value.root);})];
 for(let index=0;index<variants.length;index++){
  const label=['original','text','binary'][index],restored=variants[index];attempt(label+'/actual source ownership revalidation',()=>assert.deepEqual(checkSourceLibraryErrorClosure(product(restored,label+' source archive')),product(archive,'actual Error module/entry ownership')));
  const actual=attempt(label+'/real source declarations',()=>createActualErrorSourceDeclarations(product(restored,label+' source archive').capture)),linked=attempt(label+'/linked semantic frontend',()=>prepareJavaScriptErrorSourceClosure(product(restored,label+' source archive'),product(actual,label+'/real source declarations'))),result=attempt(label+'/portable importer completion',()=>executeErrorSourceClosure(product(linked,label+'/linked semantic frontend'),row.exportName,row.args,{abrupt:row.abrupt??false}));
  attempt(label+'/literal result',()=>{const value=product(result,label+'/portable importer completion').value;if(row.result==='function'){assert.equal(typeof value,'function');assert.equal(value.name,row.expectedName);assert.throws(()=>value(),/portable source-call continuation/u);assert.throws(()=>value.toString(),/original source mapping/u);}else if(Object.hasOwn(row,'expectedScalar'))assert.equal(value,row.expectedScalar);else inspect(value,row.expected);});
  if(row.provider&&row.result!=='function')attempt(label+'/actual distinct source class provider',()=>{const value=product(restored,label+' source archive'),owner=value.owners.find(owner=>owner.kind==='class'&&owner.path.endsWith('/'+row.provider));assert.ok(owner);assert.equal(Object.getPrototypeOf(product(result,label+'/portable importer completion').value),product(actual,label+'/real source declarations').definitions[owner.compiledName].prototype);});
  if(row.followup)attempt(label+'/ordinary source value/callback followup',()=>{const value=executeErrorSourceClosure(product(linked,label+'/linked semantic frontend'),row.followup.exportName,[product(result,label+'/portable importer completion').value,...row.followup.args]).value;if(typeof row.followup.expected==='string')assert.equal(value,row.followup.expected);else inspect(value,row.followup.expected);});
  if(row.identityFollowup)attempt(label+'/actual semantic constructor identity',()=>assert.equal(executeErrorSourceClosure(product(linked,label+'/linked semantic frontend'),row.identityFollowup,[product(result,label+'/portable importer completion').value]).value,true));
  attempt(label+'/dependency export remains private at entry boundary',()=>assert.throws(()=>beginJavaScriptErrorSourceClosure(product(linked,label+'/linked semantic frontend'),'unsupported',[]),/actual public entry owner/u));
  attempt(label+'/authenticated original declaration names',()=>{const value=product(restored,label+' source archive');for(const owner of value.owners)assert.equal(product(actual,label+'/real source declarations').definitions[owner.compiledName].name,owner.originalName);});
 }
 finishControlDiagnostics({name:row.name,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
});
for(const row of refusals)test(row.name,context=>{versions();const input=supplied(row,context);assert.throws(()=>captureSourceLibraryErrorClosure(readSourceLibraryClosure(input.entry,{aliases:true,errorClasses:model})));});
test('edited owner/entry/source archives cannot certify forged declaration names',context=>{
 const row=originals[2],input=supplied(row,context),archive=captureSourceLibraryErrorClosure(readSourceLibraryClosure(input.entry,{aliases:true,errorClasses:model}));
 for(const edit of [value=>{value.owners[0].originalName='forged';},value=>{value.entries.push({...value.entries[0],exported:'unsupported'});},value=>{value.completeSourceReceipts.pop();},value=>{value.sourceClosure.source+='\nfunction forged() { return 1; }';}]){const value=structuredClone(archive);edit(value);assert.throws(()=>checkSourceLibraryErrorClosure(value));}
 const actual=createActualErrorSourceDeclarations(archive.capture);assert.throws(()=>createJavaScriptErrorRealm(archive.capture,{...actual,declarationWitness:{sourceSha256:archive.capture.sourceSha256,owners:archive.owners}}),/actual recomposed source witness/u);
 let calls=0;const hostile={};Object.defineProperty(hostile,'sourceClosure',{get(){calls++;throw new Error('metadata must not execute');},enumerable:true});assert.throws(()=>checkSourceLibraryErrorClosure(hostile),/accessor/u);assert.equal(calls,0);
});
