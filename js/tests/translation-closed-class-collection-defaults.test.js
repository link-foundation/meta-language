// Source-only authored controls; execution remains deferred.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/javascript-class-source-rules.js';
import {closedClassCollectionDefaultDecisions as decisions,closedClassCollectionDefaultOriginals as originals,closedClassCollectionDefaultRefusals as refusals} from './fixtures/closed-class-collection-defaults.mjs';
import {readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from './support/collection-copy-integration.js';
test('literal profile and actual own-field proofs remain separate exact decisions',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,JSON.stringify(row));});
for(const row of originals)test('closed class collection default original: '+row.name,()=>readCollectionCopyOriginalControl(row));
for(const row of refusals)test('closed class namespace or method-slot boundary: '+row.name,()=>readCollectionCopyRefusalControl(row));
