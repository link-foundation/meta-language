import assert from 'node:assert/strict';
import test from 'node:test';
import {PrefixReachCache} from '../src/grammar-runtime/prefix-reach-cache.js';

test('prefix cache repeated lookups preserve absent entries promotion updates and exact values',()=>{
  for(const maximum of[1024,Number.MAX_SAFE_INTEGER]){
    const cache=new PrefixReachCache(maximum),oracle=new Map(),object={identity:true};
    let searches=0;
    const get=cache.states.get;
    cache.states.get=function(...args){searches++;return get.apply(this,args);};
    const read=key=>{assert.equal(cache.has(key),oracle.has(key));assert.ok(Object.is(cache.get(key),oracle.get(key)));assert.equal(cache.size,oracle.size);};
    const put=(key,value)=>{assert.equal(cache.set(key,value),cache);oracle.set(key,value);read(key);};
    const absent='0|same state';
    read(absent);const firstSearches=searches;
    for(let index=0;index<32;index++)read(absent);
    assert.equal(searches,firstSearches,'an absent lookup remains absent without another page search');
    put(absent,1);const presentSearches=searches;
    for(let index=0;index<32;index++)read(absent);
    assert.equal(searches,presentSearches,'paired membership and value reads reuse one location');
    for(const state of['same state','different | state 😀'])for(let index=0;index<130;index++){
      const key=index+'|'+state;put(key,index+1);
      for(const value of[index+2,undefined,-0,NaN,object,Number.MAX_SAFE_INTEGER,index+3])put(key,value);
    }
    for(const key of['ordinary','01|state','-0|state',object,Symbol('key'),undefined]){
      for(const value of[undefined,null,false,-0,NaN,Infinity,-1,0,7,object])put(key,value);
    }
    for(const key of oracle.keys())read(key);
    assert.ok([...cache.states.values()].some(pages=>[...pages.values()].some(page=>page instanceof(maximum>0xfffffffe?Float64Array:Uint32Array))));
    assert.equal(cache.locate('missing',true),null);
    assert.equal(cache.locate('65536|new state'),null);put('65536|new state',12);read('65536|new state');
  }
});
