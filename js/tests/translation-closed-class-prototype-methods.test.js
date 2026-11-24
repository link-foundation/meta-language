// Future controls over complete actual Scope and own-shaped composition.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import * as rules from '../src/translation/javascript-class-source-rules.js';
import * as observations from '../src/translation/javascript-class-observation-rules.js';
import {closedClassPrototypeDecisions as decisions,closedClassPrototypeObservationDecisions as observationDecisions,closedClassPrototypeOriginals as originals,closedClassPrototypeRefusals as refusals,closedClassPrototypeSourceAnchors as anchors} from './fixtures/closed-class-prototype-methods.mjs';
import {readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from './support/collection-copy-integration.js';
test('prototype helpers use complete actual brand and declaration ownership',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,JSON.stringify(row));});
test('hidden class values retain explicit observation roles',()=>{for(const row of observationDecisions)assert.deepEqual(observations[row.operation](...row.arguments),row.expected,JSON.stringify(row));});
test('complete selected Scope owner and identifier helper stay exact',()=>{for(const row of anchors)assert.ok(readFileSync(new URL('../../'+row.path,import.meta.url),'utf8').includes(row.source),row.path);});
for(const row of originals)test('closed class prototype complete source: '+row.name,()=>readCollectionCopyOriginalControl(row));
for(const row of refusals)test('closed class prototype observation boundary: '+row.name,()=>readCollectionCopyRefusalControl(row));
