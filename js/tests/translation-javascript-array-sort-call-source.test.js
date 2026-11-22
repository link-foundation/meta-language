import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptArraySortFunction} from '../src/translation/javascript-array-sort-call-source.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
import {runInNewContext} from 'node:vm';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const object=(identity,callable=false)=>({$:'object',identity,callable}),roots={$:'primitiveWrapperRoots',objectPrototype:object(1n),stringPrototype:object(2n),numberPrototype:object(3n),booleanPrototype:object(4n),integerPrototype:object(5n),symbolPrototype:object(6n)},arrayPrototype=object(10n);
for(const operation of ['Array.sort','Array.toSorted'])test('sorting bytecode preserves actual operation and ordinary continuation ownership '+operation,()=>{
 const fn=compileJavaScriptArraySortFunction(7n,8n,operation,roots,arrayPrototype);assert.ok(fn);assert.equal(fn.instructions[0].$,'argument');assert.equal(fn.instructions[4].operation,'require callable target');assert.equal(fn.instructions.some(row=>row.destination===0),false);
 for(const row of fn.instructions){if(row.$==='branch')for(const target of [row.yes,row.no])assert.ok(target>=0&&target<fn.instructions.length);if(row.$==='jump')assert.ok(row.target>=0&&row.target<fn.instructions.length);}
 assert.equal(fn.instructions.filter(row=>row.$==='call').length,1);assert.deepEqual(fn.instructions.find(row=>row.$==='call').arguments,[18,17]);assert.equal(fn.instructions.some(row=>row.$==='ordinaryHas'),operation==='Array.sort');assert.equal(fn.instructions.some(row=>row.$==='ordinaryDelete'),operation==='Array.sort');
 const directory={$:'collectionCallableDirectory',realms:[8n],exoticKey:{$:'symbol',identity:12n},constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:7n,realm:8n,operation,wrapperRoots:roots,arrayPrototype}]};assert.equal(compileJavaScriptCollectionCallDirectory(directory).error,'');assert.equal(compileJavaScriptArraySortFunction(7n,8n,operation,undefined,arrayPrototype),undefined);assert.equal(compileJavaScriptArraySortFunction(7n,8n,operation,roots,undefined),undefined);
});

test('copying sort snapshots dense reference-backed strings in UTF-16 order',()=>{
 const source=String.raw`
  const values=['\uE000','\uD800','A'];
  const alias=values;
  const ordered=alias.toSorted();
  ordered.push('tail');
  console.log(values.join(','));
  console.log(ordered.join(','));
 `;
 const observe=code=>{
  const output=[];
  runInNewContext(code,{console:{log:value=>output.push(value)}},{timeout:1000});
  return output;
 };
 const expected=['\uE000,\uD800,A','A,\uD800,\uE000,tail'];
 assert.deepEqual(observe(source),expected);
 const checked=checkProgram(parseJavaScript(source,{referenceSemantics:true}));
 assert.deepEqual(observe(emitJavaScript(checked).text),expected);
});
