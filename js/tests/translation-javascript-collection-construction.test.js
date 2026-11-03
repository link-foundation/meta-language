// Deferred complete source/receiver controls. No represented code runs on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createContext,runInContext} from 'node:vm';
import {captureJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {beginJavaScriptCallMachine,stepJavaScriptCallMachine,resumeJavaScriptCallObjectAllocation,readJavaScriptCallProgramError} from '../src/translation/javascript-call-machine.js';
import {beginJavaScriptConstruction,stepJavaScriptConstruction} from '../src/translation/javascript-construction.js';
import {readJavaScriptCollectionConstructionReceiver} from '../src/translation/javascript-collection-construction.js';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';
import {javascriptConstructionAssertionPolicy as policy} from './fixtures/javascript-construction-source.mjs';

function instanceFor(source,entries=['read']){return prepareJavaScriptOrdinaryClassModule(captureJavaScriptOrdinaryClassModule(source,{identifier:'actual-collection-construction.js',entries}),{realmIdentity:73n});}
function object(identity,callable=false){return {$:'object',identity,callable};}
function owner(instance,kind){const row=instance.collection.directory.constructors.find(row=>row.kind===kind);assert.ok(row);return row;}
function advance(program,instance,transition){
 if(transition.$==='machineContinue')transition=stepJavaScriptCallMachine(program,transition.machine);
 else if(transition.$==='machineAllocateObject')transition=resumeJavaScriptCallObjectAllocation(program,transition.machine,instance.embedding.allocateIdentity());
 else assert.fail(transition.reason??transition.$);
 instance.embedding.adoptHeap(transition.machine?.heap??transition.heap);return transition;
}
for(const kind of ['Map','Set','WeakMap','WeakSet'])test('generic '+kind+' construction initializes exactly one actual branded receiver',()=>{
 const instance=instanceFor('export function read(){return Reflect.construct('+kind+',[]);}'),before=instance.embedding.heap(),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin('read',[]),{steps:1048576}).transition;assert.equal(transition.$,'machineCompleted',transition.reason);
 const brand=kind==='Map'?'mapObject':kind==='Set'?'setObject':kind==='WeakMap'?'weakMapObject':'weakSetObject',created=transition.heap.objects.filter(row=>!before.objects.some(prior=>prior.identity===row.identity));assert.equal(created.length,2,'one actual argument array and one actual collection');assert.equal(created.filter(row=>row.$===brand).length,1);assert.equal(transition.heap.objects.find(row=>row.identity===transition.value.identity).$,brand);
});
for(const kind of ['Map','Set','WeakMap','WeakSet'])test('actual '+kind+' constructor metadata cannot replace its absent executable initializer',()=>{
 const instance=instanceFor('export function read(){return '+kind+';}'),row=owner(instance,kind),original=instance.embedding.module.program,program={...original,functions:original.functions.filter(fn=>fn.identity!==row.identity)},directory=program.constructorDirectory,callee=object(row.identity,true);let transition=beginJavaScriptConstruction(program,instance.embedding.heap(),directory,callee,callee,[]);
 for(let index=0;index<8&&transition.$==='constructionContinue';index++)transition=stepJavaScriptConstruction(program,transition.heap,directory,transition.state);assert.equal(transition.$,'constructionNeedImplementation');assert.match(transition.reason,/complete registered portable iterable initializer/);
});
test('collection initializer reuse independently checks real parent instruction arguments receiver brand and fresh storage',()=>{
 const instance=instanceFor('export function read(){return Reflect.construct(Map,[]);}'),program=instance.embedding.module.program,row=owner(instance,'Map');let transition=instance.begin('read',[]),machine;
 for(let work=0;work<1000;work++){const candidate=transition.machine,last=candidate?.frames.at(-1);if(last?.identity===row.identity&&last.position===2){machine=candidate;break;}transition=advance(program,instance,transition);}assert.ok(machine,'actual generated Map initializer reached its allocation instruction');
 const frame=machine.frames.at(-1),fn=program.functions.find(fn=>fn.identity===frame.identity),instruction=fn.instructions[frame.position],held=readJavaScriptCollectionConstructionReceiver(program,machine,frame,instruction);assert.equal(held.error,'');assert.equal(held.receiver.identity,frame.registers[0].identity);assert.equal(readJavaScriptCallProgramError(program,machine.heap),'');
 for(const change of [value=>value.frames.splice(-2,1),value=>value.frames.at(-2).pending.state.mode='Set',value=>value.frames.at(-2).pending.state.allocationKind='ordinary',value=>value.frames.at(-1).newTarget=object(owner(instance,'Set').identity,true),value=>value.frames.at(-1).arguments.push({$:'number',value:7}),value=>value.frames.at(-1).position=3,value=>{const id=value.frames.at(-1).registers[0].identity;value.heap.objects.find(row=>row.identity===id).$='setObject';},value=>{const id=value.frames.at(-1).registers[0].identity;value.heap.objects.find(row=>row.identity===id).prototype={$:'null'};},value=>{const id=value.frames.at(-1).registers[0].identity;value.heap.objects.find(row=>row.identity===id).entries.push({$:'collectionEntry',present:true,key:{$:'number',value:3},value:{$:'number',value:7}});}]){const changed=structuredClone(machine);change(changed);const actual=changed.frames.at(-1),result=readJavaScriptCollectionConstructionReceiver(program,changed,actual,instruction);assert.notEqual(result.error,'');assert.equal(result.receiver,undefined);}
});
for(const kind of ['Map','Set'])test('actual foreign '+kind+' fallback comes from newTarget realm without invoking its body',()=>{
 assert.equal(selectVersionedAssertion(policy,process.versions.node).expected,true);
 const instance=instanceFor('export function read(Target){return Reflect.construct('+kind+',[],Target);}'),foreign=runInContext('({Target:function Target(){throw new Error("must not call");},Array,Object,Map,Set})',createContext({}));foreign.Target.prototype=7;
 const target=instance.embedding.encode(foreign.Target),array=instance.embedding.encode(foreign.Array),mapPrototype=instance.embedding.encode(foreign.Map.prototype),setPrototype=instance.embedding.encode(foreign.Set.prototype),directory=instance.collection.speciesDirectory(),constructors=[...directory.constructors,{$:'arrayConstructorFact',identity:target.identity,realm:101n,constructable:true,intrinsicArray:false},{$:'arrayConstructorFact',identity:array.identity,realm:101n,constructable:true,intrinsicArray:true,intrinsicKind:'Array'}].toSorted((first,second)=>first.identity<second.identity?-1:first.identity>second.identity?1:0),extended={...directory,constructors,constructorRealms:[...directory.constructorRealms,{$:'constructorRealm',realm:101n,objectPrototype:instance.embedding.encode(foreign.Object.prototype),arrayPrototype:instance.embedding.encode(foreign.Array.prototype),arrayConstructor:array,mapPrototype,setPrototype}]},program={...instance.embedding.module.program,constructorDirectory:extended},entry=instance.callable('read');assert.equal(readJavaScriptCallProgramError(program,instance.embedding.heap()),'');
 let transition=beginJavaScriptCallMachine(program,instance.embedding.heap(),entry.identity,{$:'undefined'},[target]);for(let work=0;work<1000&&transition.$!=='machineCompleted';work++)transition=advance(program,instance,transition);assert.equal(transition.$,'machineCompleted',transition.reason);const created=transition.heap.objects.find(row=>row.identity===transition.value.identity);assert.equal(created.$,kind==='Map'?'mapObject':'setObject');assert.equal(created.prototype.identity,(kind==='Map'?mapPrototype:setPrototype).identity);assert.equal(Object.getPrototypeOf(Reflect.construct(kind==='Map'?Map:Set,[],foreign.Target)),foreign[kind].prototype,'independent native original');
});
