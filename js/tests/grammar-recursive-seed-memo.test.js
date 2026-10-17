import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {GrammarBuilder as G} from '../src/grammar.js';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {Executor} from '../src/grammar-runtime/executor.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {readRecursiveMemoSeed,rememberRecursiveMemoDependency,acceptRecursiveMemoEntry,reuseRecursiveMemoEntry,advanceRecursiveMemoSeed,finishRecursiveMemoSeed,storeRecursiveMemoEntry} from '../src/grammar-runtime/recursive-seed-memo.js';
import {readTriviaPrefixPolicy} from '../src/grammar-runtime/trivia-prefix-cache.js';

test('recursive memo dependencies preserve active frames revisions and conservative refusal',()=>{
  const owner={},seed=readRecursiveMemoSeed(owner,0),dependent={results:[],keywordContext:{parents:[null]}},stack=[owner,{}],memo=new Map();
  assert.equal(readRecursiveMemoSeed(owner,0),seed);assert.deepEqual(Object.keys(seed),['depth','revision','keys']);
  rememberRecursiveMemoDependency(dependent,seed);rememberRecursiveMemoDependency(dependent,seed);
  assert.equal(dependent.recursiveMemoDependencies.length,1);assert.equal(acceptRecursiveMemoEntry(dependent,stack),true);
  storeRecursiveMemoEntry(dependent,'cached',memo);assert.equal(memo.get('cached').results,dependent.results);
  assert.equal(reuseRecursiveMemoEntry(memo.get('cached'),stack),true);assert.equal(stack[1].involved,true);
  assert.equal(stack[1].recursiveMemoDependencies[0].seed,seed);
  memo.set('active',{evaluating:true,recursiveMemoDependencies:dependent.recursiveMemoDependencies});seed.keys.add('active');
  const unrelated={results:[]};memo.set('unrelated',unrelated);
  advanceRecursiveMemoSeed(owner,memo);
  assert.equal(seed.revision,1);assert.equal(seed.keys.size,0);assert.equal(memo.has('cached'),false);assert.equal(memo.has('active'),true);assert.equal(memo.get('unrelated'),unrelated);
  assert.equal(reuseRecursiveMemoEntry(dependent,stack),false);rememberRecursiveMemoDependency(dependent,seed);assert.equal(dependent.recursiveMemoConflict,true);
  const bounded={};for(let index=0;index<9;index++)rememberRecursiveMemoDependency(bounded,readRecursiveMemoSeed({},index));
  assert.equal(bounded.recursiveMemoDependencies.length,8);assert.equal(bounded.recursiveMemoConflict,true);
  assert.equal(acceptRecursiveMemoEntry({},stack),false);
  const detached={};rememberRecursiveMemoDependency(detached,seed);assert.equal(acceptRecursiveMemoEntry(detached,[{}]),false);
});

test('recursive memo final seeds retain exact results and substitute every outer dependency',()=>{
  const owner={seed:[],results:[],involved:false},seed=readRecursiveMemoSeed(owner,0),dependent={results:[],keywordContext:{parents:[null]}},memo=new Map();
  rememberRecursiveMemoDependency(dependent,seed);storeRecursiveMemoEntry(dependent,'cached',memo);
  const results=dependent.results,context=dependent.keywordContext;
  finishRecursiveMemoSeed(owner,memo,[owner]);
  assert.deepEqual(Object.keys(memo.get('cached')),['results','keywordContext']);assert.equal(memo.get('cached').results,results);assert.equal(memo.get('cached').keywordContext,context);assert.equal(seed.keys.size,0);
  const outer={},outerSeed=readRecursiveMemoSeed(outer,0),inner={seed:[],results:[],involved:true},innerSeed=readRecursiveMemoSeed(inner,1),child={results:[],keywordContext:context};
  rememberRecursiveMemoDependency(inner,outerSeed);rememberRecursiveMemoDependency(child,innerSeed);rememberRecursiveMemoDependency(child,outerSeed);storeRecursiveMemoEntry(child,'conditional',memo);
  finishRecursiveMemoSeed(inner,memo,[outer,inner]);
  assert.equal(memo.get('conditional').recursiveMemoDependencies.length,1);assert.equal(memo.get('conditional').recursiveMemoDependencies[0].seed,outerSeed);assert.ok(outerSeed.keys.has('conditional'));
  advanceRecursiveMemoSeed(outer,memo);assert.equal(memo.has('conditional'),false);assert.equal(memo.get('cached').results,results);
  for(const change of[head=>{head.results=[{end:1}];},head=>{head.involved=true;},head=>{head.recursiveMemoConflict=true;head.involved=true;}]){
    const head={seed:[{end:1}],results:null,involved:false};head.results=[head.seed[0]];const current=readRecursiveMemoSeed(head,0),entry={results:[]};
    rememberRecursiveMemoDependency(entry,current);storeRecursiveMemoEntry(entry,'refused',memo);change(head);finishRecursiveMemoSeed(head,memo,[head]);assert.equal(memo.has('refused'),false,'only the identical complete final seed can be promoted');
  }
});

async function readRecursiveMemoBaseline(){
  const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
  assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
  const executor=controls.baseline.modules.find(row=>row.path==='js/src/grammar-runtime/executor.js'),driver=controls.baseline.modules.find(row=>row.path==='js/src/grammar-runtime.js');
  for(const[row,hash]of[[executor,'3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81'],[driver,'9157c49c2d917dff5d3e13dbbbf60940971a837f6edc588fc80bf2682362cef1']]){assert.equal(row.sha256,hash);assert.equal(createHash('sha256').update(row.source).digest('hex'),hash);assert.equal(Buffer.byteLength(row.source),row.bytes);}
  const linked=(source,base,overrides=new Map())=>source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>{const address=new URL(path,base).href;return 'from '+quote+(overrides.get(address)??address)+quote;}).replaceAll('import.meta.url',JSON.stringify(base.href));
  const module=source=>'data:text/javascript;base64,'+Buffer.from(source).toString('base64');
  const executorBase=new URL('../../'+executor.path,import.meta.url),driverBase=new URL('../../'+driver.path,import.meta.url),address=module(linked(executor.source,executorBase));
  return {controls,parser:await import(module(linked(driver.source,driverBase,new Map([[executorBase.href,address]]))))};
}

test('recursive seed reuse preserves complete frozen Rust outcomes and unchanged rejection limits',async(t)=>{
  const {controls,parser:baseline}=await readRecursiveMemoBaseline(),grammar=parseGrammarLinks(nativeGrammarText('native-rust'));
  const current=createGrammarParser(grammar),original=baseline.createGrammarParser(grammar),run=Executor.prototype.run;
  const observations=[];let enabled=true;
  Executor.prototype.run=function(...args){this.recursiveMemoEnabled=enabled;this.triviaPrefixPolicy=enabled?readTriviaPrefixPolicy(this.program):null;const result=run.apply(this,args);observations.push({cells:this.budget.memory.cells,steps:this.budget.steps,memo:this.memo.size,limit:this.budget.memory.limit,memoLimit:this.memoLimit});return result;};
  try{
    for(const source of[...controls.sources,'pub fn f(input: &str) -> usize { let value = input.len(); value + 1 }','fn f(){let value=Some(1f64);if value.is_some(){return;}}',
      '// note\nfn f(){}','/// note\npub fn f(){}','/* outer /* inner */ tail */ fn f(){}','\n\tfn f(){let value=1;}','fn f(){/* unfinished','fn f(){let name="é😀";}']){
      assert.ok(Buffer.byteLength(source)<128);
      for(const options of[{}, {errorRecovery:true,recovery:'accept'}, {memoryLimit:0}, {stepLimit:0}, {maxDepth:0}])assert.deepEqual(current.parseTree(source,options),original.parseTree(source,options),source);
    }
    const source='fn f(){let value=Some(1f64);if value.is_some(){return;}}';
    enabled=false;const previous=current.parseTree(source),before=observations.at(-1);
    enabled=true;const actual=current.parseTree(source),after=observations.at(-1);
    assert.deepEqual(actual,previous);assert.ok(after.cells<before.cells);assert.ok(after.steps<before.steps);
    assert.equal(after.limit,before.limit);assert.equal(after.memoLimit,before.memoLimit);assert.ok(after.memo<=after.memoLimit);
    t.diagnostic(JSON.stringify({scope:'one authored Rust snippet only',bytes:Buffer.byteLength(source),previous:before,current:after}));
  }finally{Executor.prototype.run=run;}
  assert.ok(observations.some(row=>row.cells>0&&row.limit===2000000));
});

test('recursive seed reuse preserves authored left recursion ambiguity and empty growth',async()=>{
  const {parser:baseline}=await readRecursiveMemoBaseline(),run=Executor.prototype.run;
  const grammars=[
    new G('expression').rule('expression',G.choice(G.seq(G.ref('expression'),G.literal('+'),G.ref('atom')),G.ref('atom'))).rule('atom',G.choice(G.literal('a'),G.literal('aa'))).build(),
    new G('first').rule('first',G.choice(G.seq(G.ref('second'),G.literal('x')),G.literal('a'))).rule('second',G.choice(G.seq(G.ref('first'),G.literal('y')),G.literal('a'))).build(),
    new G('expression').rule('expression',G.choice(G.seq(G.ref('expression'),G.literal('a')),G.empty())).build(),
    new G('expression').rule('expression',G.choice(G.seq(G.ref('expression'),G.literal('+'),G.ref('expression')),G.literal('a'))).build(),
    new G('expression').source('peg').rule('expression',G.choice(G.seq(G.ref('expression'),G.literal('+'),G.literal('a')),G.literal('a'))).build(),
  ];
  Executor.prototype.run=function(...args){this.recursiveMemoEnabled=true;return run.apply(this,args);};
  try{
    for(const grammar of grammars)for(const source of['','a','aa','a+a','a+a+a','ayx','a+','ax','ayxyx']){
      assert.deepEqual(createGrammarParser(grammar).parseTree(source),baseline.createGrammarParser(grammar).parseTree(source),source);
    }
  }finally{Executor.prototype.run=run;}
});
