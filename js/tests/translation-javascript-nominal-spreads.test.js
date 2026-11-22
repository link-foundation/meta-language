// Deferred complete source, copy-order, guard and codec controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {readJavaScriptNominalSpreadError,readJavaScriptNominalSpreadGuardArm} from '../src/translation/javascript-nominal-spread-rules.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-nominal-spreads.lino',import.meta.url));
for(const row of fixture.policies)test(row.name,()=>assert.equal(readJavaScriptNominalSpreadError(row.language,row.constructors,row.declared,row.kinds,row.names,row.owners,row.owner,row.prototype),row.expected));
for(const row of fixture.guards)test('actual absence guard '+row.kind+' '+row.operation+' '+row.absence,()=>assert.equal(readJavaScriptNominalSpreadGuardArm(row.kind,row.operation,row.absence),row.expected));
for(const row of fixture.programs)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const program=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(program),native=emitRust(program);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root),javascript=emitJavaScript(restored),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);assert.deepEqual(emitRust(restored),native);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source))));
test('structural spreads preserve omission, explicit undefined, and the earlier own value',()=>{
 const source=`
/** @typedef {{value?:number}} Shape */
/** @param {boolean} own @returns {Shape} */
function make(own) { return own ? {value:undefined} : {}; }
/** @param {Shape} source @returns {Shape} */
function copy(source) { return {value:7,...source}; }
/** @returns {Shape} */
function widen() { const original={value:3}; return original; }
function compute() {
 const missing=make(false), explicit=make(true);
 const left=copy(missing), right=copy(explicit);
 return !Object.hasOwn(missing,'value') && Object.hasOwn(explicit,'value') &&
  left.value===7 && Object.hasOwn(right,'value') && (right.value ?? 8)===8 &&
  Object.hasOwn(widen(),'value') && widen().value===3;
}`;
 assert.equal(runInNewContext(source+'\ncompute();'),true);
 const program=checkProgram(parseJavaScript(source));
 const {network,root}=encodeTypedProgramLinks(program);
 const rust=emitRust(program);
 for(const restored of [LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
  const decoded=decodeTypedProgramLinks(restored,root);
  const javascript=emitJavaScript(decoded);
  const entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),true);
  assert.deepEqual(emitRust(decoded),rust);
 }
});
test('tag comparisons and guarded branches retain the nullish property error',()=>{
 const source=`
/** @typedef {{$:'trunk',value:number}|{$:'twig'}} Tree */
/** @param {Tree|null|undefined} tree @returns {boolean} */
function compute(tree) { return tree.$==='twig'; }
/** @param {Tree|null|undefined} tree @returns {number} */
function branch(tree) { if(tree.$==='twig')return 1; return 2; }
`;
 const program=checkProgram(parseJavaScript(source));
 const {network,root}=encodeTypedProgramLinks(program);
 const javascript=emitJavaScript(decodeTypedProgramLinks(LinkNetwork.fromLino(network.toLino()),root));
 for(const [name, twig, trunk] of [['compute',true,false],['branch',1,2]]){
  const entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target;
  for(const read of [runInNewContext(source+'\n'+name),runInNewContext(javascript.text+'\n'+entry)]){
   assert.equal(read({$:'twig'}),twig);
   assert.equal(read({$:'trunk',value:3}),trunk);
   for(const absent of [null,undefined])assert.throws(()=>read(absent),
    error=>error.name==='TypeError'&&error.message===`Cannot read properties of ${absent} (reading '$')`);
  }
 }
});
test('short-circuit guards exclude both absences before reading a tagged local',()=>{
 const source=`
/** @typedef {{$:'trunk',value:number}|{$:'twig'}} Tree */
/** @param {Tree} tree @returns {boolean} */
function isTwig(tree) { return tree.$==='twig'; }
/** @param {Tree|null|undefined} tree @returns {boolean} */
function compute(tree) {
 if(tree===null || tree===undefined || tree.$!=='twig')return false;
 return isTwig(tree);
}`;
 const javascript=emitJavaScript(checkProgram(parseJavaScript(source)));
 const entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
 const original=runInNewContext(source+'\ncompute');
 const translated=runInNewContext(javascript.text+'\n'+entry);
 for(const value of [null,undefined,{$:'twig'},{$:'trunk',value:3}]){
  assert.equal(translated(value),original(value));
 }
});
test('nominal rest infers the declared owner and keeps other own fields',()=>{
 const source=readFileSync(new URL('../../experiments/javascript-program/nominal-object-rest.mjs',import.meta.url),'utf8');
 assert.equal(runInNewContext(source+'\ncompute();'),true);
 const program=checkProgram(parseJavaScript(source)),{network,root}=encodeTypedProgramLinks(program);
 for(const restored of [LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
  const javascript=emitJavaScript(decodeTypedProgramLinks(restored,root));
  const entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),true);
 }
 for(const selector of ['value','$:tag']){
  const invalid=source.replace('const {child,...rest}',`const {${selector},...rest}`);
  assert.throws(()=>checkProgram(parseJavaScript(invalid)),/actual tag and every required constructor field/u);
 }
});
test('every complete preceding source remains exact',()=>{assert.equal(fixture.predecessors.length,11);for(const row of fixture.predecessors){const record=readProjectData(new URL('../../'+row.retained,import.meta.url));assert.equal(record.path,row.path);assert.equal(record.revision,row.revision);assert.equal(Buffer.byteLength(record.source),row.bytes);assert.equal(createHash('sha256').update(record.source).digest('hex'),row.sha256);}});
