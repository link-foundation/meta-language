// Deferred controls; the projection does not independently prove admissibility.
import test from 'node:test';
import assert from 'node:assert/strict';
import {planFormalDatatypeModulePrefixes} from '../src/translation/formal-datatype-module-rules.js';
import {planFormalDatatypeModules} from '../src/translation/formal-datatype-modules.js';
import {EmitState,orderDeclarations} from '../src/translation/emit-common.js';
import {formalDatatypeModuleDecisions} from './fixtures/formal-datatype-modules.mjs';
for(const [index,row]of formalDatatypeModuleDecisions.entries())test('actual datatype module prefix '+index,()=>assert.deepEqual(planFormalDatatypeModulePrefixes(...row.arguments),row.expected));
const data=(modulePath,name,target)=>({k:'data',modulePath,name,fullName:[...modulePath,name].join('.'),ctors:[{name:'Value',fields:[{name:'next',type:{kind:'data',name:target}}]}]});
test('cross-module mutual placement preserves original identities and mandatory aliases',()=>{
 const a=data(['ir'],'Expr','ir.nodes.Node'),b=data(['ir','nodes'],'Node','ir.Expr');
 const input={declarations:new Map([[a.fullName,a],[b.fullName,b]])};
 const projected=planFormalDatatypeModules(input);
 assert.equal(projected.sourceProgram,input);assert.notEqual(projected.program,input);
 assert.equal(input.declarations.get(b.fullName),b);assert.deepEqual(b.modulePath,['ir','nodes']);
 assert.deepEqual(projected.program.declarations.get(b.fullName).modulePath,['ir']);
 assert.equal(projected.program.declarations.get(b.fullName).fullName,'ir.nodes.Node');
 assert.deepEqual(projected.aliases,[{sourceFullName:'ir.nodes.Node',targetFullName:'ir.nodes.Node',sourceName:'Node',sourceModulePath:['ir','nodes'],targetModulePath:['ir'],constructors:['Value']}]);
 assert.deepEqual(orderDeclarations(projected.program).map(entry=>entry.fullName),['ir.Expr','ir.nodes.Node']);
 assert.throws(()=>orderDeclarations(input),/mutually recursive in different modules/);
});
test('unrelated declarations and same-module groups keep their source objects',()=>{
 const a=data(['same'],'A','same.B'),b=data(['same'],'B','same.A');
 const input={declarations:new Map([[a.fullName,a],[b.fullName,b]])},projected=planFormalDatatypeModules(input);
 assert.deepEqual(projected.aliases,[]);assert.equal(projected.program.declarations.get(a.fullName),a);assert.equal(projected.program.declarations.get(b.fullName),b);
});

test('lifted private names cannot capture original public aliases or sibling declarations',()=>{
 const first=data(['x'],'A','B'),second=data([],'B','x.A');
 const third=data(['x','y'],'A','x.C'),fourth=data(['x'],'C','x.y.A');
 const ordinary=data([],'A','A');ordinary.ctors[0].fields[0].type={kind:'bool'};
 const source={declarations:new Map([first,second,third,fourth,ordinary].map(entry=>[entry.fullName,entry]))};
 const projection=planFormalDatatypeModules(source);
 const state=new EmitState(source,'Rocq',name=>name,new Set(),{ctorStyle:'module',modulesShareTermSpace:true});
 const oldRefs=new Map([...source.declarations.keys()].map(name=>[name,state.ref(name)]));
 const publicAliases=projection.aliases.flatMap(alias=>[state.ref(alias.sourceFullName),...alias.constructors.map(ctor=>state.ctorRef(alias.sourceFullName,ctor))]);
 state.relocateDatatypes(projection.program,projection.aliases);
 for(const alias of projection.aliases){
  assert.ok(!publicAliases.includes(state.ref(alias.targetFullName)));
  for(const ctor of alias.constructors)assert.ok(!publicAliases.includes(state.ctorRef(alias.targetFullName,ctor)));
 }
 for(const name of ['A','B','x.C'])assert.equal(state.ref(name),oldRefs.get(name));
 assert.equal(state.moduleName(['x']),'x');assert.equal(state.moduleName(['x','y']),'y');
 assert.equal(state.program,projection.program);
 assert.throws(()=>state.relocateDatatypes(projection.program,projection.aliases),/exact original nominal owner/);
});
