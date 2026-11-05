// Deferred source/native-original controls. No represented program runs on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {captureJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {extendJavaScriptCallArrayAuthority} from '../src/translation/javascript-call-machine.js';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';
import {javascriptSharedSpeciesConstructionOriginals as originals,javascriptSharedSpeciesConstructionAssertionPolicy as policy} from './fixtures/javascript-shared-species-construction-source.mjs';
for(const row of originals)test(row.name,()=>{
 assert.equal(selectVersionedAssertion(policy,process.versions.node).expected,row.expected);
 const capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-shared-species.js',entries:row.entries}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:83n}),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,[]),{steps:1048576}).transition;
 assert.equal(transition.$,'machineCompleted',transition.reason);assert.equal(instance.embedding.materializeData(transition.value),row.expected);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn '+row.entry+';',[])();assert.equal(original(),row.expected);
});
test('species authority extensions cannot replace or remove actual established Map and Set realm roots',()=>{
 const source='export function transform(input,selected){input.constructor={[Symbol.species]:selected};return input.map(value=>value);}',capture=captureJavaScriptOrdinaryClassModule(source,{identifier:'literal-species-monotonic-roots.js',entries:['transform']}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:83n});
 function unavailable(){throw new Error('an original unavailable constructor must never execute');}
 const selected=instance.embedding.encode(unavailable),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin('transform',[instance.embedding.encode([3]),selected]),{steps:1048576}).transition;assert.equal(transition.$,'machineNeedConstructorAuthority');const program=instance.embedding.module.program,directory=instance.collection.speciesDirectory();
 for(const field of ['mapPrototype','setPrototype'])for(const remove of [false,true]){const changed={...directory,constructorRealms:directory.constructorRealms.map(row=>({...row,[field]:remove?undefined:directory.arrayPrototype}))},result=extendJavaScriptCallArrayAuthority(program,transition.machine,changed);assert.match(result.error,/preserves every established realm root/);assert.equal(result.program,program);assert.equal(result.machine,transition.machine);}
});
