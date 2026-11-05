// Deferred source controls; never executed during this drafting batch.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import * as rules from '../src/translation/disjoint-collection-iteration-rules.js';
import {disjointCollectionIterationDecisions as decisions,disjointCollectionIterationOriginals as originals,disjointCollectionIterationRefusals as refusals,disjointCollectionIterationSourceAnchors as anchors} from './fixtures/disjoint-collection-iterations.mjs';
import {readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from './support/collection-copy-integration.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
test('actual allocation and registered effect decisions stay closed',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,JSON.stringify(row));});
test('own source Map values and other-map writes retain exact anchors',()=>{for(const row of anchors)assert.ok(readFileSync(new URL('../../'+row.path,import.meta.url),'utf8').includes(row.source),row.path);});
for(const row of originals)test('disjoint collection complete source: '+row.name,()=>readCollectionCopyOriginalControl(row));
for(const row of originals)test('private iteration proof marker is consumed before checked IR: '+row.name,()=>{const program=checkProgram(parseJavaScript(row.source));assert.ok(!JSON.stringify([...program.declarations.values()]).includes('#closed collection iteration '));assert.ok(!JSON.stringify(program.main).includes('#closed collection iteration '));});
for(const row of refusals)test('retained live or unknown collection boundary: '+row.name,()=>readCollectionCopyRefusalControl(row));
