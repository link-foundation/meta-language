import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {planGuardedPrivateArrayWrites,planConstructedGuardedArrayWrites} from '../src/translation/guarded-private-array-write-rules.js';
import {createSurfaceReferenceFacts,createSurfaceReferenceColumns} from '../src/translation/surface-reference-facts.js';
import {decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

function guardedWriteFacts(native=false){
 const name=value=>({k:'name',path:[value]}),number=value=>({k:'num',value:String(value)});
 const binary=(op,left,right)=>({k:'binary',op,left,right});
 const allocation={s:'const',name:'data',value:{k:'array',items:[2,4,6].map(value=>({value:number(value),spread:false}))}};
 const operation={object:name('data'),index:name('position'),value:name('value'),op:'='};
 const write=native?{k:'referenceWrite',...operation,field:null}:{s:'assignReference',...operation};
 const conditional={s:'if',cond:binary('and',
  {k:'math',op:'isInteger',args:[{value:name('position'),spread:false}]},
  binary('and',binary('ge',name('position'),number(0)),binary('lt',name('position'),{k:'field',object:name('data'),field:'length'}))),
  then:[native?{s:'expr',expr:write}:write],else:null};
 const source={statements:[allocation,conditional,{s:'return',expr:name('data')}],parameters:[]};
 return {source,allocation,conditional,write};
}

test('the complete shipped planner retains actual operation identities in both source statement representations',()=>{
 const artifact=readProjectData(new URL('../../parity/self-translation/guarded-private-array-write-rules-native.lino',import.meta.url));
 const text=readFileSync(new URL('../src/data/guarded-private-array-write-program.lino',import.meta.url),'utf8');
 assert.equal(readFileSync(new URL('../../rust/src/data/guarded-private-array-write-program.lino',import.meta.url),'utf8'),text);
 const original=LinkNetwork.fromLino(text),operations=[planGuardedPrivateArrayWrites];
 for(const network of [original,decodeLinkNetworkBinary(encodeLinkNetworkBinary(original))]){
  const checked=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,artifact.archive.root),{wholeProgram:true});
  assert.equal(checked.declarations.size,artifact.inventory.length);
  const emitted=emitRust(checked,{arrayConsumption:true});
  for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(emitted[field],artifact.emitted[field]);
  operations.push(runInNewContext(emitJavaScript(checked).text+'\n;planGuardedPrivateArrayWrites;',{}, {timeout:1000}));
 }
 const constructionNames=new Set(['readConstructedGuardedArrayAppend','acceptConstructedGuardedArrayUses','planConstructedGuardedArrayWrites']);
 assert.equal(artifact.inventory.filter(row=>!row.generated&&!constructionNames.has(row.name)).length,15);
 assert.deepEqual(artifact.inventory.filter(row=>!row.generated&&constructionNames.has(row.name)).map(row=>row.name),[...constructionNames]);
 assert.equal(artifact.inventory.filter(row=>!row.generated).length,18);
 for(const native of [false,true]){
  const input=guardedWriteFacts(native),facts=createSurfaceReferenceFacts(input.source,{retainValues:true});
  const {columns,rows,size}=createSurfaceReferenceColumns(facts);
  const expected=[input.conditional,input.write,input.allocation].map(value=>facts.values.indexOf(value));
  assert.ok(expected.every(value=>value>=0));
  for(const operation of operations){
   assert.deepEqual(Array.from(operation(columns,rows,size,facts.root)),expected);
   for(const mutate of [value=>{value.root=-1;},value=>{value.rows[0]=-1;},value=>{value.rows[value.size]=-1;},value=>{value.rows[value.rows.length-1]=value.size;},value=>{value.columns.pop();}]){
    const invalid=structuredClone({columns,rows,size,root:facts.root});mutate(invalid);
    assert.deepEqual(Array.from(operation(invalid.columns,invalid.rows,invalid.size,invalid.root)),[]);
   }
  }
  for(const mutate of [value=>{value.conditional.else=[];},value=>{value.conditional.cond=value.conditional.cond.right;},value=>{value.source.statements.push({s:'const',name:'alias',value:{k:'name',path:['data']}});}]){
   const invalid=guardedWriteFacts(native);mutate(invalid);
   const facts=createSurfaceReferenceFacts(invalid.source),packed=createSurfaceReferenceColumns(facts);
   for(const operation of operations)assert.deepEqual(Array.from(operation(packed.columns,packed.rows,packed.size,facts.root)),[]);
  }
 }
});

test('the complete shipped joint planner preserves actual nested append and guarded write identities through both archives',()=>{
 const artifact=readProjectData(new URL('../../parity/self-translation/guarded-private-array-write-rules-native.lino',import.meta.url));
 const text=readFileSync(new URL('../src/data/guarded-private-array-write-program.lino',import.meta.url),'utf8');
 const network=LinkNetwork.fromLino(text),operations=[planConstructedGuardedArrayWrites];
 for(const restored of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
  const checked=recheckLinkedTypedProgram(decodeTypedProgramLinks(restored,artifact.archive.root),{wholeProgram:true});
  assert.deepEqual([...checked.declarations.keys()],artifact.checkedNamedSourceInventory.map(row=>row.name));
  const emitted=emitRust(checked,{arrayConsumption:true});
  for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(emitted[field],artifact.emitted[field]);
  operations.push(runInNewContext(emitJavaScript(checked).text+'\n;planConstructedGuardedArrayWrites;',{}, {timeout:1000}));
 }
 for(const native of [false,true]){
  const input=guardedWriteFacts(native);
  input.allocation.value.items=[];
  const append={k:'method',op:'push',object:{k:'name',path:['data']},args:[{k:'num',value:'0'}]};
  const statement={s:'expr',expr:append};
  input.source.statements.splice(1,0,{s:'for',init:[],cond:{k:'bool',value:true},body:[statement],update:[]});
  const facts=createSurfaceReferenceFacts(input.source,{retainValues:true}),packed=createSurfaceReferenceColumns(facts);
  const identities=values=>values.map(value=>facts.values.indexOf(value));
  const expected=[0,...identities([input.conditional,input.write,input.allocation]),1,...identities([statement,append,input.allocation])];
  assert.ok(expected.every(value=>value>=0));
  for(const operation of operations){
   assert.deepEqual(Array.from(operation(packed.columns,packed.rows,packed.size,facts.root)),expected);
   for(const mutate of [value=>{value.root=-1;},value=>{value.rows[0]=-1;},value=>{value.rows[value.size]=-1;},value=>{value.rows[value.rows.length-1]=value.size;},value=>{value.columns.pop();}]){
    const invalid=structuredClone({...packed,root:facts.root});mutate(invalid);
    assert.deepEqual(Array.from(operation(invalid.columns,invalid.rows,invalid.size,invalid.root)),[]);
   }
  }
  for(const mutate of [
   value=>{value.source.statements.push({s:'const',name:'alias',value:{k:'name',path:['data']}});},
   value=>{value.source.statements[1].body[0].expr.args=[{k:'app',fn:{k:'name',path:['next']},args:[]}];},
   value=>{const loop=value.source.statements[1];loop.body[0]={s:'const',name:'length',value:loop.body[0].expr};},
   value=>{const conditional=value.source.statements.splice(2,1)[0];value.source.statements[1].body.push(conditional);},
  ]){
   const invalid=structuredClone(input);mutate(invalid);
   const facts=createSurfaceReferenceFacts(invalid.source),packed=createSurfaceReferenceColumns(facts);
   for(const operation of operations)assert.deepEqual(Array.from(operation(packed.columns,packed.rows,packed.size,facts.root)),[]);
  }
 }
});
