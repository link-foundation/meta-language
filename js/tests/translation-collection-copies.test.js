// Deferred independent controls; no original or product ran during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import * as rules from '../src/translation/collection-copy-rules.js';
import {collectionCopyDecisions,collectionCopyOriginals,collectionCopyRefusals,collectionCopySourceAnchors} from './fixtures/collection-copies.mjs';
import {readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from './support/collection-copy-integration.js';
test('same-kind copy decisions preserve source facts rather than guessing collection owners',()=>{for(const row of collectionCopyDecisions){assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);}});
test('own source constructor anchors remain exact independent complete inputs',()=>{for(const row of collectionCopySourceAnchors)assert.ok(readFileSync(new URL('../../'+row.path,import.meta.url),'utf8').includes(row.source),row.path);});
for(const row of collectionCopyOriginals)test('collection copy actual complete original: '+row.name,()=>{readCollectionCopyOriginalControl(row);});
for(const row of collectionCopyRefusals)test('collection copy actual source refusal: '+row.name,()=>{readCollectionCopyRefusalControl(row);});
