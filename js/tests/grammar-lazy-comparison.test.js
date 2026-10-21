import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {Executor} from '../src/grammar-runtime/executor.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {readGrammarComparisonPredecessor,verifyGrammarComparisonPredecessor,generateGrammarComparisonPredecessor,
  GRAMMAR_COMPARISON_PREDECESSOR_REVISION,GRAMMAR_COMPARISON_LEGACY_REVISION,
  GRAMMAR_COMPARISON_ASSERTION_VERSIONS} from '../scripts/generate-grammar-lazy-comparison-baseline.mjs';

const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
const executorPath='js/src/grammar-runtime/executor.js',driverPath='js/src/grammar-runtime.js';
const pins={
  [executorPath]:'3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81',
  [driverPath]:'9157c49c2d917dff5d3e13dbbbf60940971a837f6edc588fc80bf2682362cef1',
};

function comparisonModule(source,path,overrides=new Map(),expose=false){
  const base=new URL('../../'+path,import.meta.url);
  const linked=source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,address)=>{
    const url=new URL(address,base).href;
    return 'from '+quote+(overrides.get(url)??url)+quote;
  }).replaceAll('import.meta.url',JSON.stringify(base.href));
  const extra=expose?'\nexport {shiftOrder,addResult,sameChildren,CHAIN,shareChildren,flattenChain};':'';
  return 'data:text/javascript;base64,'+Buffer.from(linked+extra).toString('base64');
}

async function loadComparisonBaseline(){
  assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
  assert.deepEqual(controls.baseline.modules.map(row=>row.path),Object.keys(pins));
  for(const row of controls.baseline.modules){
    assert.equal(row.sha256,pins[row.path]);assert.equal(Buffer.byteLength(row.source),row.bytes);
    assert.equal(createHash('sha256').update(row.source).digest('hex'),pins[row.path]);
  }
  const [executor,driver]=controls.baseline.modules;
  const oldAddress=comparisonModule(executor.source,executorPath,new Map(),true);
  const overrides=new Map([[new URL('../../'+executorPath,import.meta.url).href,oldAddress]]);
  const [legacy,legacyBaseline,current]=await Promise.all([
    import(oldAddress),import(comparisonModule(driver.source,driverPath,overrides)),
    import(comparisonModule(readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8'),executorPath,new Map(),true)),
  ]);
  const archive=readGrammarComparisonPredecessor();
  assert.equal(archive.revision,GRAMMAR_COMPARISON_PREDECESSOR_REVISION);
  assert.equal(archive.legacyRevision,GRAMMAR_COMPARISON_LEGACY_REVISION);
  assert.deepEqual(archive.assertionVersions,GRAMMAR_COMPARISON_ASSERTION_VERSIONS);
  const [major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node version '+process.versions.node);
  const sources=new Map(archive.modules.map(row=>[row.path,row.source]));
  const rulePath='js/src/translation/grammar-selection-rules.js',equalityPath='js/src/grammar-runtime/tree-equality.js';
  const actualOverrides=new Map();
  for(const path of[rulePath,equalityPath,executorPath]){
    const address=comparisonModule(sources.get(path),path,actualOverrides,path===executorPath);
    actualOverrides.set(new URL('../../'+path,import.meta.url).href,address);
  }
  const [previous,baseline]=await Promise.all([
    import(actualOverrides.get(new URL('../../'+executorPath,import.meta.url).href)),
    import(comparisonModule(sources.get(driverPath),driverPath,actualOverrides)),
  ]);
  return {previous,baseline,current,legacy,legacyBaseline};
}
let comparisonBaselinePromise;
function comparisonBaseline(){return comparisonBaselinePromise??=loadComparisonBaseline();}

test('actual comparator source epoch is normally generated and verifies complete immutable source membership offline',()=>{
  const original=generateGrammarComparisonPredecessor({check:true});
  assert.equal(verifyGrammarComparisonPredecessor(original),original);
  for(const mutate of[
    value=>{value.revision=GRAMMAR_COMPARISON_LEGACY_REVISION;},
    value=>{value.legacyRevision=GRAMMAR_COMPARISON_PREDECESSOR_REVISION;},
    value=>{value.assertionVersions.Node='*';},
    value=>{value.modules.reverse();},
    value=>{value.modules.pop();},
    value=>{value.modules[0].source+='\n';},
    value=>{value.modules[0].bytes++;},
    value=>{value.modules[0].sha256='0'.repeat(64);},
    value=>{value.modules[0].path='js/src/grammar-runtime.js';},
    value=>{value.modules[0].certificate.commitText+='\n';},
    value=>{value.modules[0].certificate.trees[0].entries.pop();},
    value=>{value.modules[0].certificate.blob.objectId='0'.repeat(40);},
    value=>{value.modules[0].future=true;},
    value=>{value.future=true;},
  ]){
    const changed=structuredClone(original);mutate(changed);
    assert.throws(()=>verifyGrammarComparisonPredecessor(changed),mutate.toString());
  }
  assert.equal(verifyGrammarComparisonPredecessor(original),original,'counterexamples cannot mutate the original archive');
});

function tiedGrammar(word='x'){
  return parseGrammarLinks(`
    (grammar (start document) (matching longest) (settling tokens precedence dynamic ambiguity))
    (extra (class plain (char %20) (char %0A) (char %0D)))
    (extra (ref note))
    (rule note token (literal %23note))
    (rule document normal (seq (choice unordered (seq (repeat1 (ref item))) (seq (repeat1 (ref item)))) (literal %21)))
    (rule item silent (capture labeled value (ref word)))
    (rule word token (literal ${encodeURIComponent(word)}))`);
}

function leaves(tree){
  if(tree.type==='node')return tree.children.flatMap(leaves);
  if(tree.type==='embed')return (tree.sourceChildren??[tree.root]).flatMap(leaves);
  return [tree];
}

function observe(runtime,parser,grammar,source,options){
  const run=runtime.prototype.run,observations=[];
  runtime.prototype.run=function(...args){
    try{return run.apply(this,args);}
    finally{observations.push({steps:this.budget.steps,cells:this.budget.memory.cells,
      memoryLimit:this.budget.memory.limit,memoLimit:this.memoLimit,depth:this.depth,
      memoKeys:[...this.memo.keys()],triviaKeys:[...this.triviaMemo.keys()],
      farthest:this.farthest,expected:[...this.expected]});}
  };
  try{return {outcome:parser.createGrammarParser(grammar).parseTree(source,options),observations};}
  finally{runtime.prototype.run=run;}
}

test('tied repeated prefixes retain complete independent CST fields trivia order and exact frozen metrics',async()=>{
  const {previous,baseline}=await comparisonBaseline();
  for(const [word,count,gap]of[['x',1,' '],['x',8,' '],['x',32,' #note\n'],['π',8,' #note\r\n']]){
    const grammar=tiedGrammar(word),source=(word+gap).repeat(count)+'!';
    assert.ok(Buffer.byteLength(source)<1024,'authored miniature input, never the OOM workload');
    for(const options of[{},{ambiguity:'reject'},{errorRecovery:true,recovery:'accept'}]){
      const actual=observe(Executor,{createGrammarParser},grammar,source,options);
      const expected=observe(previous.Executor,baseline,grammar,source,options);
      assert.deepEqual(actual,expected,'every public CST field, ambiguity, diagnostic and execution metric');
      assert.equal(actual.outcome.ok,true);assert.deepEqual(actual.outcome.ambiguities,[]);
      const all=leaves(actual.outcome.tree),physical=all.filter(leaf=>leaf.type!=='missing');
      assert.equal(physical.map(leaf=>leaf.text).join(''),source);
      assert.deepEqual(all.filter(leaf=>!leaf.trivia).map(leaf=>leaf.text),[...Array(count).fill(word),'!']);
      const words=all.filter(leaf=>leaf.kind==='word');
      assert.equal(words.length,count);assert.ok(words.every(leaf=>leaf.field==='value'&&leaf.trivia!==true));
      const width=Buffer.byteLength(word+gap),size=Buffer.byteLength(word);
      assert.deepEqual(words.map(leaf=>[leaf.start,leaf.end]),Array.from({length:count},(_,index)=>[index*width,index*width+size]));
      assert.equal(all.filter(leaf=>leaf.kind==='note'&&leaf.trivia===true).length,gap.includes('#note')?count:0);
      for(let at=1;at<physical.length;at++)assert.equal(physical[at-1].end,physical[at].start);
      assert.equal(physical[0].start,0);assert.equal(physical.at(-1).end,Buffer.byteLength(source));
    }
  }
});

test('tied-prefix errors recovery and resource refusals retain complete frozen diagnostics and metrics',async()=>{
  const {previous,baseline}=await comparisonBaseline(),grammar=tiedGrammar();
  for(const source of['x x x !','x x x ?','x x x ','!',''])for(const options of[
    {},{errorRecovery:true},{errorRecovery:true,recovery:'accept'},
    {memoryLimit:0},{memoryLimit:1},{memoryLimit:20},{stepLimit:0},{stepLimit:20},{maxDepth:0},{memoLimit:0},
  ]){
    const actual=observe(Executor,{createGrammarParser},grammar,source,options);
    assert.deepEqual(actual,observe(previous.Executor,baseline,grammar,source,options),JSON.stringify({source,options}));
    if(options.memoryLimit===0)assert.deepEqual(actual.outcome.rejection,{reason:'memoryBudget',limit:0});
    if(options.stepLimit===0)assert.deepEqual(actual.outcome.rejection,{reason:'stepLimit',limit:0});
    if(options.maxDepth===0)assert.deepEqual(actual.outcome.rejection,{reason:'nestingDepth',limit:0});
    if(actual.outcome.tree&&options.recovery==='accept')assert.equal(leaves(actual.outcome.tree).map(leaf=>leaf.type==='missing'?'':leaf.text).join(''),source);
  }
});

function owner(operations,children,chains=[],header={}){
  let part=[];
  for(const child of children){part={left:part,right:[child],length:part.length+1,flat:null};chains.push(part);}
  return operations.shareChildren({end:children.at(-1)?.end??0,state:{key:'state'},dynamic:0,precedence:null,tail:null,ambiguous:false,cost:0,...header},
    Array.isArray(part)?{children:part}:{[operations.CHAIN]:part});
}
const token=(kind,start,end,fields={})=>({type:'token',kind,start,end,...fields});

// Supplemental storage control; the independent complete parser oracles above
// remain mandatory. This checks an actual retention mechanism, not a heap guess.
test('actual precedence and output comparisons leave retained joined prefixes lazy',async()=>{
  const {previous,current}=await comparisonBaseline();
  const run=(operations,lazy)=>{
    const chains=[],items=Array.from({length:128},(_,index)=>token('word',index,index+1,{field:'value'}));
    const left=owner(operations,items,chains),right=owner(operations,items,chains);
    assert.equal(operations.shiftOrder(left,right,[],null,Buffer.from('x'.repeat(128))),0);
    const results=new Map();operations.addResult(results,left,null);operations.addResult(results,right,null);
    assert.equal(results.size,1);assert.equal([...results.values()][0],left,'first exact output keeps its actual identity');
    assert.equal(lazy?operations.sameChildren(left,right):operations.sameChildren(left.children,right.children),true);
    if(lazy)assert.ok(chains.every(chain=>chain.flat===null));
    else assert.ok(chains.some(chain=>chain.flat!==null));
    return structuredClone([...results.values()]);
  };
  assert.deepEqual(run(current,true),run(previous,false));
});

test('lazy comparisons preserve sibling lookahead reduction order and full output distinctions',async()=>{
  const {previous,current,legacy}=await comparisonBaseline();
  for(const reverse of[false,true])for(const field of[false,true])for(const trivia of[undefined,false,true,'truthy']){
    const build=operations=>{
      const reduced={level:0,name:null,associativity:'left',rule:'silent'};
      const first=token('word',0,1,{alone:true,reduced}),plain=token('word',0,1);
      const closing=token('close',1,2,{reduced}),tail=token('tail',2,3,{trivia});
      const left=owner(operations,[first,closing,tail]);
      const right=owner(operations,[plain,closing,{...tail,...(field?{field:'other'}:{})}]);
      return reverse?[right,left]:[left,right];
    };
    const order=current.shiftOrder(...build(current),[],null,Buffer.from('abc'));
    assert.equal(order,previous.shiftOrder(...build(previous),[],null,Buffer.from('abc')));
    assert.equal(order,reverse?-1:1,'the real later sibling closes the silent left-associative reduction');
    const out=operations=>{const [left,right]=build(operations),results=new Map();operations.addResult(results,left,null);operations.addResult(results,right,null);return structuredClone([...results.values()]);};
    assert.deepEqual(out(current),out(previous));
    assert.deepEqual(out(current),out(legacy),'the original ae76 sibling inputs retain their unchanged outputs');
  }
  for(const [index,edit]of[
    leaf=>{leaf.field='other';},leaf=>{leaf.trivia=true;},leaf=>{leaf.attributes={meaning:'other'};},
    leaf=>{leaf.lexical={lexer:'other',rule:'word'};},leaf=>{leaf.end=2;},
  ].entries()){
    const compare=operations=>{
      const original=[token('word',0,1,{field:'value'})],changed=structuredClone(original);edit(changed[0]);
      const left=owner(operations,original,[],{end:1}),right=owner(operations,changed,[],{end:1}),results=new Map();
      operations.addResult(results,left,null);operations.addResult(results,right,null);
      return structuredClone([...results.values()]);
    };
    assert.deepEqual(compare(current),compare(previous));
    assert.equal(compare(current)[0].ambiguous,true,edit.toString());
    const historical=compare(legacy);
    // ae76's local sameOutput omitted lexical metadata. The complete c33
    // source pair already distinguishes it before the lazy storage change.
    assert.equal(historical[0].ambiguous,index!==3,'exact ae76 source-epoch oracle: '+edit.toString());
    assert.deepEqual(historical.map(row=>({...row,ambiguous:true})),compare(previous),'every historical payload survives the documented lexical source-epoch correction');
  }
});

test('lazy equality retains embedded physical CSTs and every parallel selection witness',async()=>{
  const {previous,current,legacy}=await comparisonBaseline();
  const original={type:'embed',language:'first',start:0,end:1,
    sourceChildren:[token('physical',0,1,{field:'source'})],
    root:{type:'node',kind:'document',start:0,end:1,children:[token('word',0,1,{field:'value'})]},
    selection:{mode:'all-grammar',mapping:'identity',start:0,end:1,
      candidates:[{language:'first',relation:'accepted'},{language:'second',relation:'accepted'}],
      provenance:{source:'first',language:'first',rules:[]},ambiguous:false,
      alternatives:[{language:'second',provenance:{source:'second',language:'second',rules:[]},
        root:{type:'node',kind:'document',start:0,end:1,children:[token('word',0,1,{field:'value'})]}}]}};
  for(const [index,edit]of[
    value=>{},value=>{value.sourceChildren[0].field='other';},value=>{value.root.children[0].field='other';},
    value=>{value.selection.candidates.reverse();},value=>{value.selection.provenance.source='other';},
    value=>{value.selection.alternatives[0].root.children[0].trivia=true;},
    value=>{value.selection.alternatives[0].root.children[0].attributes={meaning:'other'};},
    value=>{value.selection.alternatives.pop();},value=>{value.selection.future={explicit:true};},
  ].entries()){
    const compare=operations=>{
      const changed=structuredClone(original);edit(changed);
      const first=owner(operations,[original]),second=owner(operations,[changed]),results=new Map();
      operations.addResult(results,first,null);operations.addResult(results,second,null);
      return structuredClone([...results.values()]);
    };
    const actual=compare(current);assert.deepEqual(actual,compare(previous),edit.toString());
    assert.equal(actual[0].ambiguous,index>0);
    const historical=compare(legacy);
    // ae76 did not descend into embeds; c33's shared equality already checks
    // physical children, nested roots and every parallel selection witness.
    assert.equal(historical[0].ambiguous,false,'exact ae76 embedding source epoch: '+edit.toString());
    assert.deepEqual(historical.map(row=>({...row,ambiguous:index>0})),compare(previous),'all original embedded payloads remain complete across the authenticated source epochs');
  }
  for(const trivia of[undefined,false,true,'truthy']){
    const original={type:'node',kind:'group',rule:'group',start:0,end:2,children:[token('word',0,1),token(null,1,2,{trivia})]};
    const other={...original,end:1,children:[token('word',0,1)]};
    const oldFirst=owner(previous,[original]),oldSecond=owner(previous,[other]);
    const first=owner(current,[original]),second=owner(current,[other]);
    assert.equal(current.sameChildren(first,second),previous.sameChildren(oldFirst.children,oldSecond.children));
    const legacyFirst=owner(legacy,[original]),legacySecond=owner(legacy,[other]);
    assert.equal(legacy.sameChildren(legacyFirst.children,legacySecond.children),trivia===true,'ae76 structural trivia is strict true');
    assert.equal(previous.sameChildren(oldFirst.children,oldSecond.children),Boolean(trivia),'c33 structural recursion has the retained truthy-trivia rule');
  }
});

test('lazy precedence keeps the following sibling and parent context after a shared prefix',async()=>{
  const {previous,current}=await comparisonBaseline();
  for(const reverse of[false,true]){
    const run=operations=>{
      const prefix=Array.from({length:64},(_,index)=>token('word',index,index+1));
      const first=token('word',64,65),last=token('tail',65,67);
      const tag=level=>({level,name:null,associativity:'none',rule:'authored'});
      const sibling={type:'node',kind:'tail',rule:'tail',start:65,end:67,children:[last],precedence:tag(2)};
      const whole={type:'node',kind:'whole',rule:'whole',start:64,end:67,children:[first,last],precedence:tag(1)};
      const spacer=token(null,65,65,{trivia:true});
      const a=owner(operations,[...prefix,first,spacer,sibling]),b=owner(operations,[...prefix,whole]);
      return operations.shiftOrder(...(reverse?[b,a]:[a,b]),[],null,Buffer.from('x'.repeat(67)));
    };
    assert.equal(run(current),run(previous));
    assert.equal(run(current),reverse?-1:1,'the following sibling has the independently higher reduction level');
  }
});

test('exact shared child outputs keep the original constant-time predicate-read fast path',async()=>{
  const {previous,current}=await comparisonBaseline();
  for(const materialized of[false,true]){
    const run=(operations,lazy)=>{
      let reads=0;
      const items=Array.from({length:128},(_,index)=>({type:'token',kind:'word',start:index,end:index+1,
        get trivia(){reads++;return false;}}));
      const left=owner(operations,items);
      const right=materialized?{children:left.children}:operations.shareChildren({},left);
      const same=lazy?operations.sameChildren(left,right):operations.sameChildren(left.children,right.children);
      return {same,reads};
    };
    const expected=run(previous,false),actual=run(current,true);
    assert.deepEqual(expected,{same:true,reads:0});assert.deepEqual(actual,expected);
  }
});

test('lazy direct-child suffixes preserve empty linked flat-only and materialized sequence indices',async()=>{
  const source=readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8');
  const current=await import(comparisonModule(source+'\nexport {walkChildPartsFrom};',executorPath));
  const items=[token('first',0,1),token(null,1,2,{trivia:true}),token('last',2,2)];
  const shapes=[[],{left:[],right:[],length:0,flat:null},items,
    {left:null,right:null,length:3,flat:items},
    {left:{left:[],right:[items[0]],length:1,flat:null},right:items.slice(1),length:3,flat:null},
    {left:{left:[],right:[items[0]],length:1,flat:[items[0]]},right:items.slice(1),length:3,flat:items}];
  for(const [index,part]of shapes.entries()){
    const expected=index<2?[]:items;
    for(let start=0;start<=expected.length+2;start++){
      const actual=[...current.walkChildPartsFrom(part,start)],suffix=expected.slice(start);
      assert.deepEqual(actual,suffix);
      for(let at=0;at<actual.length;at++)assert.equal(actual[at],suffix[at],'actual child identity survives suffix traversal');
    }
  }
});
