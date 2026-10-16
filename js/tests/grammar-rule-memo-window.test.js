import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {GrammarBuilder as G} from '../src/grammar.js';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {Executor} from '../src/grammar-runtime/executor.js';
import {retireCompletedRuleMemoPrefix} from '../src/grammar-runtime/rule-memo-window.js';
import {readProjectData} from '../scripts/project-data.mjs';

test('rule memo retirement preserves active recursion and exact surviving entries',()=>{
  const complete=()=>({results:[],keywordContext:{parents:[null]}}),active={evaluating:true,results:null},involved={involved:true,results:[]};
  const entries=[['0|0|state',active],['1|0|state',complete()],['2|3|state',complete()],['3|0|state',involved],['unknown',complete()],['4|65536|state',complete()],['5|1|state',complete()]];
  const memo=new Map(entries),before=[...memo];
  assert.equal(retireCompletedRuleMemoPrefix(memo,65536),2);
  assert.deepEqual([...memo],before.filter(([key])=>!['1|0|state','2|3|state'].includes(key)));
  for(const[key,value]of memo)assert.equal(value,entries.find(([name])=>name===key)[1]);
  assert.equal(retireCompletedRuleMemoPrefix(memo,65536),0,'a current completed entry ends the retirement prefix');
  assert.equal(retireCompletedRuleMemoPrefix(memo,131072),2);
  assert.deepEqual([...memo],[entries[0],entries[3],entries[4]]);
  for(const position of[0,1,65535]){const small=new Map(before);assert.equal(retireCompletedRuleMemoPrefix(small,position),0);assert.deepEqual([...small],before);}
  const malformed=new Map(['01','-0','-1','1.5','9007199254740993'].map(text=>['1|'+text+'|state',complete()]));
  assert.equal(retireCompletedRuleMemoPrefix(malformed,131072),0);assert.equal(malformed.size,5);
  const unknownRules=new Map(['rule','01','-0','-1','1.5','9007199254740993'].map(text=>[text+'|0|state',complete()]));
  assert.equal(retireCompletedRuleMemoPrefix(unknownRules,131072),0);assert.equal(unknownRules.size,6);
});

test('rule memo retirement stops at the first retained completed page',()=>{
  const memo=new Map(Array.from({length:64},(_,at)=>[at+'|'+(at===0?65536:0)+'|state',{results:[]}])) ,original=memo[Symbol.iterator];
  let visited=0;memo[Symbol.iterator]=function*(){for(const row of original.call(this)){visited++;yield row;}};
  assert.equal(retireCompletedRuleMemoPrefix(memo,65536),0);assert.equal(visited,1);assert.equal(memo.size,64);
});

test('an authored cross page grammar reuses current calls without releasing charged cells',async()=>{
  const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
  assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
  const executorSource=controls.baseline.modules.find(row=>row.path==='js/src/grammar-runtime/executor.js'),driverSource=controls.baseline.modules.find(row=>row.path==='js/src/grammar-runtime.js');
  for(const[row,expected]of[[executorSource,'3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81'],[driverSource,'9157c49c2d917dff5d3e13dbbbf60940971a837f6edc588fc80bf2682362cef1']]){
    assert.equal(row.sha256,expected);assert.equal(createHash('sha256').update(row.source).digest('hex'),expected);assert.equal(Buffer.byteLength(row.source),row.bytes);
  }
  const address=new URL('../../'+executorSource.path,import.meta.url);
  const linked=(source,base,overrides=new Map())=>source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>{const url=new URL(path,base).href;return 'from '+quote+(overrides.get(url)??url)+quote;}).replaceAll('import.meta.url',JSON.stringify(base.href));
  const module=source=>'data:text/javascript;base64,'+Buffer.from(source).toString('base64');
  const priorAddress=module(linked(executorSource.source,address)),priorExecutor=await import(priorAddress),driver=new URL('../../'+driverSource.path,import.meta.url);
  const priorParser=await import(module(linked(driverSource.source,driver,new Map([[address.href,priorAddress]]))));
  const padding=' '.repeat(65536),grammar=new G('start').source('peg')
    .rule('start',G.seq(G.ref('old'),G.literal(padding),G.choice(G.seq(G.ref('expression'),G.literal('x')),G.seq(G.ref('expression'),G.literal('y')),G.ref('expression'))))
    .rule('old',G.seq(G.ref('first'),G.ref('second'),G.ref('third')))
    .rule('first',G.empty()).rule('second',G.empty()).rule('third',G.empty()).rule('atom',G.literal('a'))
    .rule('expression',G.choice(G.seq(G.ref('expression'),G.literal('+'),G.ref('atom')),G.ref('atom'))).build();
  const read=(runtime,parser,body='a')=>{
    const run=runtime.prototype.run;let observed;
    runtime.prototype.run=function(...args){const result=run.apply(this,args);observed={cells:this.budget.memory.cells,memo:this.memo.size,retired:this.retiredRuleMemoEntries??0};return result;};
    try{return {outcome:parser.createGrammarParser(grammar).parseTree(padding+body,{memoLimit:4}),observed};}finally{runtime.prototype.run=run;}
  };
  const actual=read(Executor,{createGrammarParser}),expected=read(priorExecutor.Executor,priorParser);
  assert.equal(expected.outcome.ok,true);assert.deepEqual(actual.outcome,expected.outcome);
  assert.ok(actual.observed.retired>0);assert.equal(expected.observed.retired,0);
  assert.ok(actual.observed.memo<=4);assert.ok(actual.observed.cells<expected.observed.cells,'cache hits avoid new charged computations');
  assert.ok(actual.observed.cells>actual.observed.memo,'retired entries remain charged');
  for(const body of['a+a','a+a+a','a+'])assert.deepEqual(read(Executor,{createGrammarParser},body).outcome,read(priorExecutor.Executor,priorParser,body).outcome,'left recursion and refusals remain exact: '+body);
});
