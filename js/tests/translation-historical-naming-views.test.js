import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {deriveOwnCodebaseHistoricalNamingViews} from '../scripts/derive-own-codebase-historical-naming-views.mjs';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
test('historical naming extraction retains every epoch owner and old-only name in original order',()=>{
 const source='current shared',sources=[
  {file:'first.js',current:pin(source),predecessor:{revision:'first revision',source:'old shared old'}},
  {file:'first.js',current:pin(source),predecessor:{revision:'second revision',source:'older shared'}},
 ];
 const extract=(text,file)=>{assert.equal(file,'first.js');return text.split(' ');};
 const inventories=[{inventory:'first inventory',files:['first.js'],extract,recordOf:name=>'operation.'+name},
  {inventory:'unrelated',files:['other.js'],extract:()=>{throw new Error('unrelated inventory');}},
  {inventory:'word inventory',files:['first.js'],words:true,extract:()=>{throw new Error('word inventory');}},
  {inventory:'second inventory',files:['first.js'],extract}];
 assert.deepEqual(deriveOwnCodebaseHistoricalNamingViews(sources,inventories,()=>source),[
  {inventory:'first inventory',file:'first.js',revision:'first revision',names:[{name:'old',record:'operation.old'}]},
  {inventory:'second inventory',file:'first.js',revision:'first revision',names:[{name:'old',record:'old'}]},
  {inventory:'first inventory',file:'first.js',revision:'second revision',names:[{name:'older',record:'operation.older'}]},
  {inventory:'second inventory',file:'first.js',revision:'second revision',names:[{name:'older',record:'older'}]},
 ]);
 assert.throws(()=>deriveOwnCodebaseHistoricalNamingViews(sources,inventories,()=>source+' '),/current source pin/u);
 const failure=new Error('extractor failure');
 assert.throws(()=>deriveOwnCodebaseHistoricalNamingViews(sources,[{inventory:'failed',files:['first.js'],extract:()=>{throw failure;}}],()=>source),error=>error===failure);
});
