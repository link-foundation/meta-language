import assert from 'node:assert/strict';
import test from 'node:test';
import {packLexicalCacheResults,packTriviaCacheResult} from '../src/grammar-runtime/cache-storage.js';

test('compact cache fallback retains unknown records arrays and unread accessor fields',()=>{
  const state={key:'control'},children=[];
  const makeLexical=()=>Array.from({length:20},(_,end)=>({end,state,children,dynamic:0,precedence:null,tail:null,ambiguous:false}));
  for(const value of[null,undefined,{},Array(20)])assert.equal(packLexicalCacheResults(value),value);
  for(const mutate of[
    value=>{delete value[3];},value=>{value.note='retained';},value=>{value[Symbol('note')]=true;},
    value=>{value[3]=null;},value=>{value[3]=7;},value=>{Object.setPrototypeOf(value,Object.create(Array.prototype));},
    value=>{Object.defineProperty(value,'length',{writable:false});},
    value=>{Object.defineProperty(value,3,{get(){throw new Error('unread lexical array accessor');}});},
    value=>{Object.defineProperty(value,3,{writable:false});},
  ]){const value=makeLexical();mutate(value);assert.equal(packLexicalCacheResults(value),value);}
  const makePadding=()=>({end:20,leaves:Array.from({length:20},(_,start)=>({type:'token',kind:null,start,end:start+1,trivia:true})),state,states:Array(21).fill(state)});
  for(const value of[null,undefined,{},Object.create(null)])assert.equal(packTriviaCacheResult(value,0,new Map(),100),value);
  for(const mutate of[
    value=>{value.note='retained';},value=>{value[Symbol('note')]=true;},value=>{Object.setPrototypeOf(value,{note:true});},
    value=>{Object.defineProperty(value,'state',{enumerable:false});},value=>{Object.defineProperty(value,'state',{writable:false});},
    value=>{Object.defineProperty(value,'leaves',{get(){throw new Error('unread padding record accessor');}});},
    value=>{value.leaves=null;},value=>{value.states=null;},
    value=>{delete value.leaves[3];},value=>{delete value.states[3];},value=>{value.leaves[3]=null;},
    value=>{value.leaves.note=true;},value=>{value.states.note=true;},
    value=>{Object.setPrototypeOf(value.leaves,Object.create(Array.prototype));},value=>{Object.setPrototypeOf(value.states,Object.create(Array.prototype));},
    value=>{Object.defineProperty(value.states,'length',{writable:false});},value=>{Object.defineProperty(value.leaves,'length',{writable:false});},
    value=>{Object.defineProperty(value.leaves,3,{get(){throw new Error('unread leaf array accessor');}});},
    value=>{Object.defineProperty(value.states,3,{get(){throw new Error('unread state array accessor');}});},
    value=>{Object.defineProperty(value.leaves[3],'type',{get(){throw new Error('unread leaf field accessor');}});},
    value=>{value.state=0;value.states.fill(0);value.states[3]=-0;},
  ]){const value=makePadding(),pool=new Map();mutate(value);assert.equal(packTriviaCacheResult(value,0,pool,100),value);assert.equal(pool.size,0);}
});

