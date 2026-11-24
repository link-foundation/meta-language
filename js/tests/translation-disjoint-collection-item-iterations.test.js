// Deferred additive complete inputs; earlier values controls stay independent.
import test from 'node:test';
import assert from 'node:assert/strict';
import * as rules from '../src/translation/disjoint-collection-iteration-rules.js';
import {disjointCollectionItemIterationDecisions as decisions,disjointCollectionItemIterationOriginals as originals,disjointCollectionItemIterationRefusals as refusals} from './fixtures/disjoint-collection-item-iterations.mjs';
import {readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from './support/collection-copy-integration.js';
test('eager collection item forms use actual standard owner and operation',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,JSON.stringify(row));});
for(const row of originals)test('closed keys/entries complete source: '+row.name,()=>readCollectionCopyOriginalControl(row));
for(const row of refusals)test('retained live keys/entries boundary: '+row.name,()=>readCollectionCopyRefusalControl(row));
