// Deferred originals and shared product controls; no native compiler runs here.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {captureJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';
import {javascriptArraySortOriginals as originals,javascriptArraySortAssertionPolicy as policy} from './fixtures/javascript-array-sort-source.mjs';
for(const row of originals)test(row.name,()=>{
 assert.equal(selectVersionedAssertion(policy,process.versions.node).expected,row.expected);
 const capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-array-sort.js',entries:row.entries}),instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:83n}),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,row.arguments),{steps:65536}).transition;
 assert.equal(transition.$,'machineCompleted',transition.reason);assert.deepEqual(instance.embedding.materializeData(transition.value),row.expected);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn '+row.entry+';',[])();assert.equal(original(...row.arguments),row.expected);
});
