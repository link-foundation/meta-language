// Deferred controls; the projection does not independently prove admissibility.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/formal-datatype-module-rules.js';
import {planFormalDatatypeModules,formalDatatypeModuleDependencies} from '../src/translation/formal-datatype-modules.js';
import {EmitState,orderDeclarations} from '../src/translation/emit-common.js';
import {formalDatatypeModuleDecisions} from './fixtures/formal-datatype-modules.mjs';
for(const [index,row]of formalDatatypeModuleDecisions.entries())test('actual datatype module prefix '+index,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
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
 // Module closure also lifts x.C, whose old name remains a mandatory alias.
 for(const name of ['A','B'])assert.equal(state.ref(name),oldRefs.get(name));
 assert.ok(projection.aliases.some(alias=>alias.sourceFullName==='x.C'));
 assert.equal(state.moduleName(['x']),'x');assert.equal(state.moduleName(['x','y']),'y');
 assert.equal(state.program,projection.program);
 assert.throws(()=>state.relocateDatatypes(projection.program,projection.aliases),/exact original nominal owner/);
});

test('all datatype dependencies inside a protected alias module lift before it opens',()=>{
 const a=data(['x'],'A','y.B'),b=data(['y'],'B','x.A');
 const dependency=data(['x'],'Dependency','none');dependency.ctors[0].fields[0].type={kind:'bool'};
 a.ctors[0].fields.push({name:'dependency',type:{kind:'data',name:'x.Dependency'}});
 const source={declarations:new Map([a,dependency,b].map(entry=>[entry.fullName,entry]))},projected=planFormalDatatypeModules(source);
 assert.deepEqual(projected.aliases.map(alias=>alias.sourceFullName),['x.A','x.Dependency','y.B']);
 assert.ok([...projected.program.declarations.values()].every(entry=>entry.modulePath.length===0));
 assert.equal(orderDeclarations(projected.program)[0].fullName,'x.Dependency');
});

test('alias scopes wait for every lifted block even when an unrelated function was source first',()=>{
 const first={k:'fn',name:'unrelated',fullName:'x.unrelated',modulePath:['x'],params:[],ret:{kind:'bool'},body:{k:'bool',value:true,type:{kind:'bool'}}};
 const a=data(['x'],'A','y.B'),b=data(['y'],'B','x.A');
 const source={declarations:new Map([first,a,b].map(entry=>[entry.fullName,entry]))},plan=planFormalDatatypeModules(source),extraDependencies=formalDatatypeModuleDependencies(plan);
 assert.equal(orderDeclarations(plan.program)[0],first,'the existing default ordering stays exact');
 assert.deepEqual([...extraDependencies.get(first.fullName)].sort(),['x.A']);
 const order=orderDeclarations(plan.program,{requireContiguousModules:true,extraDependencies});
 assert.deepEqual(order.map(entry=>entry.fullName),['x.A','y.B','x.unrelated']);
 assert.throws(()=>orderDeclarations(plan.program,{extraDependencies:new Map([['missing',new Set(['x.A'])]])}),/actual declared owners/);
 assert.throws(()=>orderDeclarations(plan.program,{extraDependencies:new Map([['x.unrelated',new Set(['missing'])]])}),/actual declaration edges/);
});

test('relocation rejects edited source facts before consuming public name reservations',()=>{
 const a=data(['x'],'A','y.B'),b=data(['y'],'B','x.A'),source={declarations:new Map([a,b].map(entry=>[entry.fullName,entry]))};
 const plan=planFormalDatatypeModules(source),state=new EmitState(source,'Lean',name=>name,new Set(),{ctorStyle:'data'}),before=state.ref(a.fullName);
 const forged={...plan.program,declarations:new Map(plan.program.declarations)};
 forged.declarations.set(a.fullName,{...forged.declarations.get(a.fullName),generated:true});
 assert.throws(()=>state.relocateDatatypes(forged,plan.aliases),/every original datatype fact/);
 assert.equal(state.program,source);assert.equal(state.ref(a.fullName),before);
 state.relocateDatatypes(plan.program,plan.aliases);assert.notEqual(state.ref(a.fullName),before);
});

test('a target ancestor waits for external datatype prerequisites before opening',()=>{
 const early={k:'fn',name:'early',fullName:'x.early',modulePath:['x'],params:[],ret:{kind:'bool'},body:{k:'bool',value:true,type:{kind:'bool'}}};
 const a=data(['x','inner'],'A','x.B'),b=data(['x'],'B','x.inner.A'),outside=data(['z'],'Dependency','none');
 outside.ctors[0].fields[0].type={kind:'bool'};
 a.ctors[0].fields.push({name:'external',type:{kind:'data',name:'z.Dependency'}});
 const source={declarations:new Map([early,a,b,outside].map(entry=>[entry.fullName,entry]))};
 const plan=planFormalDatatypeModules(source),extraDependencies=formalDatatypeModuleDependencies(plan);
 assert.ok(extraDependencies.get(early.fullName).has(outside.fullName));
 const order=orderDeclarations(plan.program,{requireContiguousModules:true,extraDependencies});
 assert.deepEqual(order.map(entry=>entry.fullName),['z.Dependency','x.early','x.inner.A','x.B']);
});
