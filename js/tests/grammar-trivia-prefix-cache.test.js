import assert from 'node:assert/strict';
import test from 'node:test';
import {readTriviaPrefixPolicy,acceptTriviaExtraPrefix} from '../src/grammar-runtime/trivia-prefix-cache.js';

test('trivia prefix policy preserves padding comments unknown rules and scanner expectations',()=>{
  const literal=value=>({kind:'literal',value}),rule=expression=>({kind:'normal',expression});
  const program={expectedReferences:new Map(),expectedTerminals:new Map(),trivia:[
    {kind:null,expression:{kind:'charClass',negated:false,items:[' ','\t','\n'].map(value=>({kind:'char',value}))}},
    {kind:'comment',expression:{kind:'ref',name:'comment'}},{kind:'unknown',expression:{kind:'ref',name:'unknown'}},
  ],rules:new Map([['comment',rule({kind:'seq',items:[literal('/*'),literal('*/')]})],['unknown',rule({kind:'optional',item:literal('x')})]])};
  const policy=readTriviaPrefixPolicy(program);
  for(const byte of[32,9,10,47])assert.equal(acceptTriviaExtraPrefix(policy,'comment',byte),true);
  for(const byte of[0,13,65,255,undefined])assert.equal(acceptTriviaExtraPrefix(policy,'comment',byte),false);
  for(const kind of[null,'unknown','absent'])for(const byte of[65,undefined])assert.equal(acceptTriviaExtraPrefix(policy,kind,byte),true);
  for(const change of[
    value=>{value.expectedReferences.set('scanner request',1);},value=>{value.expectedTerminals.set('scanner request',1);},
    value=>{value.trivia[0].expression=literal('');},value=>{value.trivia[0].expression={kind:'regex',source:'\\s+'};},
    value=>{value.trivia[0].expression.items[0].value='é';},value=>{value.trivia[0].expression.negated=true;},
    value=>{value.trivia[0].expression.items[0]={kind:'ref',name:'padding'};},
  ]){const changed=structuredClone(program);change(changed);assert.equal(readTriviaPrefixPolicy(changed),null);assert.equal(acceptTriviaExtraPrefix(null,'comment',65),true);}
  for(const expression of[literal(''),literal('é'),{kind:'ref',name:'comment'},{kind:'seq',items:[]}]){
    const changed=structuredClone(program);changed.rules.set('comment',rule(expression));assert.equal(acceptTriviaExtraPrefix(readTriviaPrefixPolicy(changed),'comment',65),true);
  }
});
