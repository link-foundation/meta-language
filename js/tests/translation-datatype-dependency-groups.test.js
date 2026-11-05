// Deferred graph/host controls only; no execution during bulk drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {planDatatypeDependencyGroups} from '../src/translation/datatype-dependency-group-rules.js';
import {datatypeGroups,dependencies,orderDeclarations} from '../src/translation/emit-common.js';
import {datatypeDependencyGroupDecisions} from './fixtures/datatype-dependency-groups.mjs';
for(const [index,row]of datatypeDependencyGroupDecisions.entries())test('actual datatype dependency component '+index,()=>assert.deepEqual(planDatatypeDependencyGroups(...row.arguments),row.expected));
const data=(name,type,modulePath=[])=>({k:'data',name,fullName:[...modulePath,name].join('.'),modulePath,ctors:[{name:'Value',fields:[{name:'value',type}]}]});
test('mutual datatype units preserve source order and independent dependencies',()=>{
 const a=data('A',{kind:'data',name:'B'}),b=data('B',{kind:'array',element:{kind:'data',name:'A'}}),c=data('C',{kind:'bool'}),program={declarations:new Map([['A',a],['C',c],['B',b]])};
 const groups=datatypeGroups(program);assert.deepEqual([...groups.keys()],['A','B']);assert.deepEqual(groups.get('A'),[a,b]);assert.equal(groups.get('A'),groups.get('B'));assert.deepEqual(orderDeclarations(program),[a,b,c]);
});
test('all nested structural type domains retain actual datatype dependencies',()=>{
 const alternatives={kind:'union',alternatives:[{kind:'data',name:'Left'},{kind:'data',name:'Right'}]};
 const ret={kind:'map',key:{kind:'data',name:'Key'},value:{kind:'set',element:{kind:'cell',element:alternatives}}};
 const callable={kind:'function',params:[{kind:'data',name:'Argument'}],ret};
 const type={kind:'tuple',elements:[{kind:'record',fields:[{name:'field',type:{kind:'option',element:callable}}]}]};
 assert.deepEqual([...dependencies(data('Outer',type))].sort(),['Argument','Key','Left','Right']);
});
test('mutual source owners in different modules retain the existing explicit boundary',()=>{
 const a=data('A',{kind:'data',name:'right.B'},['left']),b=data('B',{kind:'data',name:'left.A'},['right']);
 assert.throws(()=>orderDeclarations({declarations:new Map([[a.fullName,a],[b.fullName,b]])}),error=>error.kind==='unsupported'&&error.message.includes('mutually recursive in different modules'));
});
