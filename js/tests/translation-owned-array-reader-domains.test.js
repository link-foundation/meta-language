// Deferred actual-body and source-domain controls; source authors import none.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseJavaScript} from '../src/translation/javascript.js';
import {readOwnedArrayReaderParameters} from '../src/translation/owned-array-append.js';
import {acceptOwnedArrayReaderDomain} from '../src/translation/owned-array-reader-domain-rules.js';
test('array result admission distinguishes complete primitive domains and preserves integer refinements',()=>{
 for(const [result,parameter] of [['float','string'],['string','float'],['float','bool'],['bool','int']])assert.equal(acceptOwnedArrayReaderDomain('array',result,'array',parameter),true);
 for(const [result,parameter] of [['string','string'],['float','float'],['int','nat'],['fixed','int'],['array','float'],['option','string'],['record','float'],['','bool']])assert.equal(acceptOwnedArrayReaderDomain('array',result,'array',parameter),false);
 assert.equal(acceptOwnedArrayReaderDomain('bool','','array','string'),true);
 assert.equal(acceptOwnedArrayReaderDomain('record','','array','string'),false);
});
test('actual helper bodies retain every alias capture and rebinding restriction',()=>{
 const owner=body=>parseJavaScript('/** @param {string[]} values @returns {number[]} */function read(values){'+body+'}').items.find(item=>item.name==='read');
 assert.deepEqual([...readOwnedArrayReaderParameters(owner('return [values.length];'),new Map())],[0]);
 for(const body of ['const alias=values;return [alias.length];','const capture=()=>values.length;return [capture()];','values=[];return [values.length];'])assert.deepEqual([...readOwnedArrayReaderParameters(owner(body),new Map())],[]);
});
