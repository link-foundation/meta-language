// Deferred exact original/transport controls for actual eager consumption.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import * as rules from '../src/translation/disjoint-collection-iteration-rules.js';
import {disjointCollectionSpreadDecisions as decisions,disjointCollectionSpreadOriginals as originals,disjointCollectionSpreadRefusals as refusals,disjointCollectionSpreadSourceAnchors as anchors} from './fixtures/disjoint-collection-spreads.mjs';
import {readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from './support/collection-copy-integration.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
test('eager consumption has a distinct source role and closed receiver scope',()=>{for(const row of decisions)assert.deepEqual(rules[row.operation](...row.arguments),row.expected,JSON.stringify(row));});
test('actual own assumption and encoding spread anchors remain exact',()=>{for(const row of anchors)assert.ok(readFileSync(new URL('../../'+row.path,import.meta.url),'utf8').includes(row.source),row.path);});
for(const row of originals)test('eager collection spread complete source: '+row.name,()=>readCollectionCopyOriginalControl(row));
for(const row of originals)test('eager source private marker is consumed before checked IR: '+row.name,()=>{const program=checkProgram(parseJavaScript(row.source));assert.ok(!JSON.stringify([...program.declarations.values()]).includes('#closed collection '));assert.ok(!JSON.stringify(program.main).includes('#closed collection '));});
for(const row of refusals)test('retained collection spread boundary: '+row.name,()=>readCollectionCopyRefusalControl(row));
