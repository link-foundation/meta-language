// Future supplied-closure controls. No capture, source execution or product
// generation ran while authoring this test.
import assert from 'node:assert/strict';
import test from 'node:test';
import {mkdirSync,mkdtempSync,writeFileSync,rmSync,readFileSync} from 'node:fs';
import {join,relative} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {runInNewContext} from 'node:vm';
import * as rules from '../src/translation/source-library-class-rules.js';
import {acceptSourceLibraryFunctionExport} from '../src/translation/source-library-export-rules.js';
import {readSourceLibraryDeclarationBindingKind} from '../src/translation/source-library-binding-rules.js';
import {readSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from '../scripts/read-source-library-closure.mjs';
import {projectSourceLibraryEntryPoints} from '../scripts/source-library-entry-points.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readCollectionCopyConcreteControl} from './support/collection-copy-concrete.js';
import {createControlDiagnostics,finishControlDiagnostics,requireControlProduct as product} from './support/control-diagnostics.js';
import {sourceLibraryClassDecisions as decisions,sourceLibraryClassOriginals as originals,sourceLibraryClassRefusals as refusals,sourceLibraryClassSourceAnchors as anchors} from './fixtures/source-library-class-composition.mjs';
import {sourceLibraryClassReExportDecisions as reExportDecisions,sourceLibraryClassReExportOriginals as reExportOriginals,sourceLibraryClassReExportRefusals as reExportRefusals,sourceLibraryClassReExportSourceAnchors as reExportAnchors} from './fixtures/source-library-class-re-exports.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url)),digest=source=>createHash('sha256').update(source).digest('hex');
function supplied(row,context){const parent=join(root,'issue-195-results/work/source-library-class-composition');mkdirSync(parent,{recursive:true});const directory=mkdtempSync(join(parent,'case-'));context.after(()=>rmSync(directory,{recursive:true,force:true}));writeFileSync(join(directory,'package.json'),'{"type":"module"}');for(const file of row.files){assert.match(file.path,/^[a-z][a-z-]*\.js$/u);writeFileSync(join(directory,file.path),file.source);readCollectionCopyConcreteControl(file.source);}return {directory,entry:relative(root,join(directory,row.entry)).replaceAll('\\','/')};}
test('the function-only declaration and export profiles remain exact',()=>{assert.equal(readSourceLibraryDeclarationBindingKind('class','identifier','Scope',false),'');assert.equal(acceptSourceLibraryFunctionExport('class','identifier','Scope',false),false);});
test('class closure source decisions retain actual owner and interval facts',()=>{for(const row of [...decisions,...reExportDecisions])assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);});
test('selected Scope module body stays exact to its actual source owner',()=>{for(const row of [...anchors,...reExportAnchors])assert.ok(readFileSync(join(root,row.path),'utf8').includes(row.source),row.path);});
for(const row of [...originals,...reExportOriginals])test(row.name,async context=>{
 const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered '+process.versions.node+'; '+row.versions.node);
 const {directory,entry}=supplied(row,context),original=await import(pathToFileURL(join(directory,row.entry)).href);assert.equal(await original.run(...row.arguments),row.expected);
 assert.throws(()=>readSourceLibraryClosure(entry,{aliases:true}),/exports need/u);
 const closure=readSourceLibraryClosure(entry,{aliases:true,classes:'closed local class declarations 1',reExports:row.reExports??''}),checked=checkProgram(parseJavaScript(closure.source,{referenceSemantics:true}));
 const owners=readSourceLibraryDeclarationOwnership(closure,checked);assert.equal(owners.length,checked.declarations.size);assert.equal(owners.some(owner=>owner.sourceClassOwner),row.classOwnersExpected??true);
 for(const input of closure.inputs){assert.equal(input.source,row.files.find(file=>input.path.endsWith('/'+file.path)).source);assert.equal(digest(input.source),input.sourceSha256);let source=input.source;for(const step of input.sourceCompositionSteps){assert.equal(digest(source),step.beforeSourceSha256);for(const edit of step.edits.toReversed())source=source.slice(0,edit.start)+edit.text+source.slice(edit.end);assert.equal(digest(source),step.sourceSha256);}assert.equal(source,input.compiledModuleSource);const ranges=input.closureRanges.toSorted((a,b)=>a.sourceStart-b.sourceStart);assert.equal(ranges.map(range=>closure.source.slice(range.start,range.end)).join(''),source);for(const range of ranges)assert.equal(closure.source.slice(range.start,range.end),source.slice(range.sourceStart,range.sourceEnd));}
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 const selected=attempt('actual export entry projection',()=>projectSourceLibraryEntryPoints(checked,closure,entry).program),packet=attempt('typed class closure construction',()=>encodeTypedProgramLinks(product(selected,'actual export entry projection'),{provenance:{sourceClosure:closure,sourceDeclarationOwners:owners}}));
 if(row.retainedClasses)attempt('selected actual class witness rows',()=>assert.deepEqual(product(selected,'actual export entry projection').sourceClassOwners.owners.map(owner=>owners.find(row=>row.name===owner.constructor).sourceName).sort(),[...row.retainedClasses].sort()));
 const text=attempt('typed text encoding',()=>product(packet,'typed class closure construction').network.toLino()),binary=attempt('typed binary encoding',()=>encodeLinkNetworkBinary(product(packet,'typed class closure construction').network));
 const variants=[attempt('original typed graph',()=>product(packet,'typed class closure construction').network),attempt('typed text reload',()=>LinkNetwork.fromLino(product(text,'typed text encoding'))),attempt('typed binary reload',()=>decodeLinkNetworkBinary(product(binary,'typed binary encoding')))];
 for(let index=0;index<variants.length;index++){
  const network=variants[index],transport=['original','text','binary'][index];attempt(transport+'/exact graph',()=>assert.equal(product(network,transport+' graph').toLino(),product(text,'typed text encoding')));
  const restored=attempt(transport+'/typed check',()=>recheckLinkedTypedProgram(decodeTypedProgramLinks(product(network,transport+' graph'),product(packet,'typed class closure construction').root),{wholeProgram:true}));
  attempt(transport+'/persistent owners',()=>assert.deepEqual(product(restored,transport+'/typed check').sourceClassOwners,product(selected,'actual export entry projection').sourceClassOwners));
  const emitted=attempt(transport+'/JavaScript/emission',()=>emitJavaScript(product(restored,transport+'/typed check')).text.replace(/^export /gmu,''));
  attempt(transport+'/JavaScript/literal result',()=>assert.equal(runInNewContext(product(emitted,transport+'/JavaScript/emission')+'\nrun(...input)',{input:row.arguments},{timeout:1000}),row.expected));
  for(const emit of [emitRust,emitLean,emitRocq])attempt(transport+'/'+emit.name+'/emission',()=>assert.ok(emit(product(restored,transport+'/typed check')).definitions.length>0));
 }
 attempt('exact binary replay',()=>assert.deepEqual(encodeLinkNetworkBinary(decodeLinkNetworkBinary(product(binary,'typed binary encoding'))),product(binary,'typed binary encoding')));
 finishControlDiagnostics({name:row.name,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
});
for(const row of [...refusals,...reExportRefusals])test(row.name,context=>{const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered '+process.versions.node+'; '+row.versions.node);const{entry}=supplied(row,context);assert.throws(()=>{const closure=readSourceLibraryClosure(entry,{aliases:true,classes:'closed local class declarations 1',reExports:row.reExports??''});return readSourceLibraryDeclarationOwnership(closure,checkProgram(parseJavaScript(closure.source,{referenceSemantics:true})));},error=>String(error.message).includes(row.error));});
