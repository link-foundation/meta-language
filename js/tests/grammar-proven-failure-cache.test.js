import assert from 'node:assert/strict';
import test from 'node:test';
import {GrammarBuilder as G} from '../src/grammar.js';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {Executor,StepLimitReached,stepBudget} from '../src/grammar-runtime/executor.js';
import {loadProgram} from '../src/grammar-runtime/load.js';
import {encodeText} from '../src/grammar-runtime/text.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {readGrammarFirstPrefixes,instrumentProvenGrammarFailures} from '../../experiments/grammar-proven-failure-cache.mjs';

// Authored grammar controls only. These are not upstream performance probes,
// requirement observations or evidence that the failed workload improved.
function recursiveGrammar(first='a',second='b') {
  return new G('first')
    .rule('first',G.choice(G.seq(G.ref('second'),G.literal('x')),G.literal(first)))
    .rule('second',G.choice(G.seq(G.ref('first'),G.literal('y')),G.literal(second))).build();
}

function firstValues(prefix) {
  return [...prefix.first.values()].map(item=>item.value).sort();
}

function reconstruct(tree) {
  if(tree.type==='node')return tree.children.map(reconstruct).join('');
  return tree.type==='missing'?'':tree.text;
}

test('experimental FIRST certificates retain recursive unions, nullability and decoded Unicode prefixes',()=>{
  const grammar=recursiveGrammar('é😀','\\n'),prefixes=readGrammarFirstPrefixes(grammar);
  assert.ok(prefixes);
  for(const name of['first','second']){
    const prefix=prefixes.rules.get(name);
    assert.equal(prefix.nullable,false);assert.equal(prefix.unknown,false);
    assert.deepEqual(firstValues(prefix),['\\','é']);
  }
  const nullable=new G('optional').rule('optional',G.optional(G.literal('x')))
    .rule('after',G.seq(G.ref('optional'),G.literal('z')))
    .rule('empty',G.empty()).build();
  const result=readGrammarFirstPrefixes(nullable);
  assert.equal(result.rules.get('optional').nullable,true);
  assert.equal(result.rules.get('empty').nullable,true);
  assert.equal(result.rules.get('after').nullable,false);
  assert.deepEqual(firstValues(result.rules.get('after')),['x','z']);
  const unproductive=new G('cycle').rule('cycle',G.ref('cycle')).build();
  const cycle=readGrammarFirstPrefixes(unproductive).rules.get('cycle');
  assert.equal(cycle.nullable,false);assert.equal(cycle.first.size,0);
});

test('experimental certificates refuse scanners, lookahead, actions, immediate tokens and unknown literal settings',()=>{
  const expressions=[
    G.ref('external_scanner'),G.ref('missing_rule'),
    {kind:'immediateToken',item:G.literal('x')},G.literalInsensitive('X'),
    G.and(G.literal('x')),G.not(G.literal('x')),G.and(G.ref('external_scanner')),
    {kind:'predicate',item:G.literal('x'),condition:{kind:'boolean',value:true}},
    {kind:'literal',value:'X',caseInsensitive:true},
    {kind:'literal',value:'X',flags:'i'},
    {kind:'literal',value:'\\n',raw:true},
    {kind:'literal',value:1},
  ];
  for(const expression of expressions){
    const grammar=new G('root').rule('root',expression).build();
    assert.equal(readGrammarFirstPrefixes(grammar).rules.get('root').unknown,true,JSON.stringify(expression));
  }
  for(const attributes of[{action:[]},{parameters:['value']}]){
    const grammar=new G('root').rule('root',G.literal('x')).build();
    grammar.rules.set('root',{...grammar.rules.get('root'),...attributes});
    assert.equal(readGrammarFirstPrefixes(grammar).rules.get('root').unknown,true);
  }
  const many=new G('root').rule('root',G.choice(...Array.from({length:65},(_,index)=>G.literal(String.fromCodePoint(0x100+index))))).build();
  assert.equal(readGrammarFirstPrefixes(many).rules.get('root').unknown,true,'an oversized union refuses a certificate');
});

test('experimental failed-answer retention preserves full authored outcomes, recovery and unchanged rejection limits',(context)=>{
  const cases=[];
  for(const[grammar,sources]of[
    [recursiveGrammar(),['?','','a','b','bx','ayx','a!']],
    [recursiveGrammar('é😀','\\n'),['?','é😀','\\n','\\nx','é😀yx']],
  ]){
    const parser=createGrammarParser(grammar);
    for(const source of sources)for(const options of[
      {},{errorRecovery:true},{errorRecovery:true,recovery:'accept'},
      {stepLimit:0},{memoryLimit:0},{maxDepth:0},{memoLimit:0},
    ])cases.push({parser,source,options,expected:parser.parseTree(source,options)});
  }
  const original=Executor.prototype.run,instrument=instrumentProvenGrammarFailures();
  try{
    for(const{parser,source,options,expected}of cases){
      const actual=parser.parseTree(source,options);
      assert.deepEqual(actual,expected,JSON.stringify({source,options}));
      if(options.errorRecovery&&actual.tree)assert.equal(reconstruct(actual.tree),source);
      if(Object.hasOwn(options,'stepLimit'))assert.deepEqual(actual.rejection,{reason:'stepLimit',limit:0});
      if(Object.hasOwn(options,'memoryLimit'))assert.deepEqual(actual.rejection,{reason:'memoryBudget',limit:0});
      if(Object.hasOwn(options,'maxDepth'))assert.deepEqual(actual.rejection,{reason:'nestingDepth',limit:0});
    }
    assert.ok(instrument.statistics.admitted>0,'a completed empty recursive answer with a known excluded prefix reaches the actual retention branch');
    assert.ok(instrument.statistics.refused>0);
    context.diagnostic(JSON.stringify({scope:'authored miniature grammars only',cases:cases.length,statistics:instrument.statistics,upstreamImprovementEstablished:false}));
  }finally{instrument.restore();}
  assert.equal(Executor.prototype.run,original);
});

test('experimental retention refuses contextual grammars, catchall trivia and every recovery round',()=>{
  const immediate=recursiveGrammar();
  immediate.rules.set('second',{...immediate.rules.get('second'),expression:G.choice(G.ref('first'),{kind:'immediateToken',item:G.literal('b')})});
  const action=recursiveGrammar();
  action.rules.set('second',{...action.rules.get('second'),action:[]});
  const catchall=recursiveGrammar();catchall.declarations.extras=[G.any()];
  const unknownTrivia=recursiveGrammar();unknownTrivia.declarations.extras=[{kind:'regex',value:'\\s+'}];
  const scanner=parseGrammarLinks(`
    (grammar (start first))
    (scanner external (tokens scanned) (operations (if (next (literal b)) (then advance (emit scanned))) fail))
    (rule first normal (choice unordered (seq (ref second) (literal x)) (literal a)))
    (rule second normal (choice unordered (ref first) (ref scanned)))
  `);
  for(const[grammar,options]of[
    [immediate,{}],[action,{}],[catchall,{}],[unknownTrivia,{}],[scanner,{}],
    [recursiveGrammar(),{errorRecovery:true}],[recursiveGrammar(),{errorRecovery:true,recovery:'accept'}],
  ]){
    const parser=createGrammarParser(grammar),expected=parser.parseTree('?',options);
    const original=Executor.prototype.run,instrument=instrumentProvenGrammarFailures();
    try{
      assert.deepEqual(parser.parseTree('?',options),expected);
      assert.equal(instrument.statistics.admitted,0,'contextual or repairing calls supply no exclusion certificate');
    }finally{instrument.restore();}
    assert.equal(Executor.prototype.run,original);
  }
});

test('experimental instrumentation restores each executor memo method when a run throws',()=>{
  const program=loadProgram(recursiveGrammar(),{}),bytes=encodeText('?'),options={stepLimit:0};
  const executor=new Executor(program,bytes,0,bytes.length,options,stepBudget(options,bytes.length),1000);
  const original=Executor.prototype.run,remove=executor.memo.delete,instrument=instrumentProvenGrammarFailures();
  try{
    assert.throws(()=>executor.run(program.start),StepLimitReached);
    assert.equal(executor.memo.delete,remove);
    assert.equal(instrument.statistics.admitted,0);
  }finally{instrument.restore();}
  assert.equal(Executor.prototype.run,original);
});
