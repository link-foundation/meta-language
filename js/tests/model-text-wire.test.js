// Deferred transport controls; never executed by source construction.
import test from 'node:test';
import assert from 'node:assert/strict';
import {encodeModelText,decodeModelText} from '../scripts/model-text-wire.mjs';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
assertVersions();
for(const text of ['', 'plain\0text', 'λ🌊', '\uD800', '\uDC00', '\uD800x\uDC00', 'prefix\uD800suffix'])test('lossless schema text '+JSON.stringify(text),()=>{
 const wire=encodeModelText(text);
 assert.equal(decodeModelText(wire),text);
 assert.equal(decodeModelText(JSON.parse(JSON.stringify(wire))),text);
 assert.deepEqual(wire,text.isWellFormed()?text:{textEncoding:'UTF16 code units 1',units:Array.from({length:text.length},(_,index)=>text.charCodeAt(index))});
});
for(const value of [null,[],{}, {textEncoding:'UTF16 code units 1',units:[65536]}, {textEncoding:'UTF16 code units 1',units:[-1]}, {textEncoding:'UTF16 code units 1',units:[1.5]}, {textEncoding:'UTF16 code units 1',units:['1']}, {textEncoding:'other',units:[55296]}, {textEncoding:'UTF16 code units 1',units:[55296],ignored:true}])test('refuse malformed text transport '+JSON.stringify(value),()=>assert.throws(()=>decodeModelText(value),TypeError));
