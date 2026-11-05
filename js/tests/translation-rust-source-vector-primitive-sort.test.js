// Deferred source/product controls. Imports declare tests and fixtures only.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {rustFixedType} from '../src/translation/types.js';
import {specializePrimitiveVectorSort,primitiveVectorSortTemplateModel} from '../src/translation/primitive-vector-sort-specialization.js';
import {checkRustSourceVectorPrimitiveSort} from '../src/translation/rust-source-vector-primitive-sort.js';
import {integerVectorKeyPrecedes,booleanVectorKeyPrecedes,stringVectorKeyPrecedes,sortIntegerVectorEntries,sortBooleanVectorEntries,sortStringVectorEntries} from '../src/translation/primitive-vector-sort-template.js';
import {rustSourcePrimitiveVectorSortOriginals as originals,rustSourcePrimitiveVectorSortRefusals as refusals,rustSourcePrimitiveVectorSortVersions as versions} from './fixtures/rust-source-vector-primitive-sort.mjs';
const options=sourceVersion=>({sourceVersion,exclusiveSourceModel:'Rust exclusive source loans 1'});
function nodes(root){const pending=[root],seen=new Set();while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;assert.ok(seen.size<1000000);seen.add(node);pending.push(...(node instanceof Map?[...node.values()]:Object.values(node)));}return [...seen];}
function stdout(text){const lines=[];vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:10000});return lines.length?lines.join('\n')+'\n':'';}
test('every closed primitive original checks and emits actual stdout without a collection library',()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 for(const sourceVersion of ['1.90.0','1.99.0'])for(const row of originals){const surface=parseRust(row.source,options(sourceVersion)),checked=checkProgram(surface);assert.equal(stdout(emitJavaScript(checked).text),row.stdout,row.name+'/'+sourceVersion);assert.ok(!nodes(checked).some(node=>node.k==='math'&&node.op==='rustSourcePrimitiveVectorSort'),row.name);}
});
test('actual unsupported Ord domains and source alias failures retain their original refusals',()=>{for(const row of refusals)assert.throws(()=>checkProgram(parseRust(row.source,options('1.90.0'))),error=>['type','unsupported','syntax'].includes(error.kind),row.name);});
test('actual source insertion preserves equal-key opaque payload identities in every domain',()=>{
 for(const [sort,keys]of [[sortIntegerVectorEntries,[7,1,7]],[sortBooleanVectorEntries,[true,false,true]],[sortStringVectorEntries,['z','a','z']]]){const payloads=[{identity:1},{identity:2},{identity:3}],entries=keys.map((key,index)=>({$:sort===sortIntegerVectorEntries?'integerSortEntry':sort===sortBooleanVectorEntries?'booleanSortEntry':'stringSortEntry',key,value:{$:'opaqueValue',value:payloads[index]}})),before=entries.slice(),result=sort(entries);assert.deepEqual(result.map(entry=>entry.value.value.identity),[2,1,3]);assert.equal(result[1],entries[0]);assert.equal(result[2],entries[2]);assert.deepEqual(entries,before);}
});
test('source comparison distinguishes scalar String Ord from UTF16 and keeps exact integer boundaries',()=>{
 assert.equal(stringVectorKeyPrecedes('\u{e000}','\u{10000}'),true);assert.equal(stringVectorKeyPrecedes('\u{10000}','\u{e000}'),false);assert.equal(stringVectorKeyPrecedes('','a'),true);assert.equal(stringVectorKeyPrecedes('a','aa'),true);assert.equal(stringVectorKeyPrecedes('same','same'),false);assert.equal(booleanVectorKeyPrecedes(false,true),true);assert.equal(booleanVectorKeyPrecedes(true,false),false);assert.equal(integerVectorKeyPrecedes(9007199254740992n,9007199254740993n),true);
});
test('ordinary specialization retains exact fixed comparison domains and complete source bodies',()=>{
 for(const owner of ['u8','i16','u64','i128']){const element=rustFixedType(owner),specialization=specializePrimitiveVectorSort(element,'primitive_'+owner);assert.equal(specialization.model,primitiveVectorSortTemplateModel);const comparison=specialization.library.declarations.get(specialization.keyComparison);assert.deepEqual(comparison.params.map(parameter=>parameter.type),[element,element]);assert.ok(nodes(comparison.body).some(node=>node.k==='binary'&&node.op==='lt'&&node.domain?.kind==='fixed'));assert.ok(!nodes(comparison.body).some(node=>node.k==='cast'&&node.to?.kind==='float'));assert.ok(specialization.library.declarations.get(specialization.entry).body);}
 for(const element of [{kind:'bool'},{kind:'string'}])assert.deepEqual(specializePrimitiveVectorSort(element,'primitive_'+element.kind).keyType,element);
 assert.throws(()=>specializePrimitiveVectorSort({kind:'float'},'unsupported'),/closed primitive/);
});
test('private method admission cannot be forged by an ordinary surface marker',()=>{assert.throws(()=>checkRustSourceVectorPrimitiveSort({language:'Rust'},{op:'rustSourcePrimitiveVectorSort',args:[],sourcePrimitiveSortAdmission:{domain:'u8'}},new Map(),[],null),/actual parsed standard method/);});
test('literal originals retain version boundaries and complete independent outputs',()=>{assert.equal(originals.length,25);assert.equal(refusals.length,13);for(const row of [...originals,...refusals]){assert.equal(row.sourceVersionRange,versions.rust);assert.match(row.source,/fn main/);}for(const row of originals)assert.equal(typeof row.stdout,'string');for(const sourceVersion of ['1.89.0','1.100.0'])assert.throws(()=>parseRust(originals[0].source,options(sourceVersion)));});
