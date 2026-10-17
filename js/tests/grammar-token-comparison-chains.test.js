import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';

const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
const executorPath='js/src/grammar-runtime/executor.js';
const driverPath='js/src/grammar-runtime.js';
const pins={
  [executorPath]:'3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81',
  [driverPath]:'9157c49c2d917dff5d3e13dbbbf60940971a837f6edc588fc80bf2682362cef1',
};

function readTokenComparisonModule(source,path,overrides=new Map(),expose=false) {
  const base=new URL('../../'+path,import.meta.url);
  const linked=source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,address)=>{
    const resolved=new URL(address,base).href;
    return 'from '+quote+(overrides.get(resolved)??resolved)+quote;
  }).replaceAll('import.meta.url',JSON.stringify(base.href));
  return 'data:text/javascript;base64,'+Buffer.from(linked+(expose?'\nexport {preferredTokens,CHAIN,shareChildren};':'')).toString('base64');
}

async function readTokenComparisonBaseline() {
  assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
  assert.deepEqual(controls.baseline.modules.map(row=>row.path),Object.keys(pins));
  for(const row of controls.baseline.modules){
    assert.equal(row.sha256,pins[row.path]);
    assert.equal(createHash('sha256').update(row.source).digest('hex'),pins[row.path]);
    assert.equal(Buffer.byteLength(row.source),row.bytes);
  }
  const [executor,driver]=controls.baseline.modules;
  const originalAddress=readTokenComparisonModule(executor.source,executor.path,new Map(),true);
  const currentSource=readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8');
  const currentAddress=readTokenComparisonModule(currentSource,executorPath,new Map(),true);
  const overrides=new Map([[new URL('../../'+executor.path,import.meta.url).href,originalAddress]]);
  const [original,current,baseline]=await Promise.all([
    import(originalAddress),import(currentAddress),import(readTokenComparisonModule(driver.source,driver.path,overrides)),
  ]);
  return {original,current,baseline};
}

const leaf=(kind,start,end,fields={})=>({type:'token',kind,start,end,...fields});
const node=(kind,children)=>({type:'node',kind,start:children[0]?.start??0,end:children.at(-1)?.end??0,children});
const tokens={
  bytes:Buffer.from('x'.repeat(1024)),
  kinds:new Map([
    ['identifier',{priority:0,specificity:1,order:0}],
    ['low',{priority:0,specificity:1,order:1}],
    ['high',{priority:1,specificity:1,order:2}],
  ]),
  literals:new Map(),
};

test('token comparison skips actual shared child segments without flattening or visiting their leaves',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const compare=operations=>{
    let reads=0;
    const leaves=Array.from({length:128},(_,index)=>({
      type:'token',kind:'identifier',get start(){reads++;return index;},get end(){reads++;return index+1;},
    }));
    let prefix={left:[leaves[0]],right:[],length:1,flat:null};
    for(const item of leaves.slice(1))prefix={left:prefix,right:[item],length:prefix.length+1,flat:null};
    const left={left:prefix,right:[leaf('high',128,129)],length:129,flat:null};
    const right={left:prefix,right:[leaf('low',128,129)],length:129,flat:null};
    const owner=chain=>operations.shareChildren({},{[operations.CHAIN]:chain});
    const result=operations.preferredTokens(owner(left),owner(right),tokens);
    return {result,reads,prefix,left,right};
  };
  const expected=compare(original),actual=compare(current);
  assert.equal(expected.result,1);
  assert.equal(actual.result,expected.result);
  assert.ok(expected.reads>0,'the original comparator visits the shared token prefix');
  assert.equal(actual.reads,0,'the linked prefix contains no unresolved lookahead or new token conflict');
  for(const chain of[actual.prefix,actual.left,actual.right])assert.equal(chain.flat,null);
});

test('token comparison opens a shared segment while a reduction still awaits its lookahead',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const compare=operations=>{
    let reads=0;
    const first=leaf('identifier',0,1,{alone:true});
    const wrapper=node('wrapper',[first]);
    const lookahead={type:'token',kind:'identifier',get start(){reads++;return 1;},end:2};
    const shared={left:[lookahead],right:[],length:1,flat:null};
    const build=(head,tail)=>operations.shareChildren({},{[operations.CHAIN]:{
      left:{left:[head],right:shared,length:2,flat:null},right:[tail],length:3,flat:null,
    }});
    return {result:operations.preferredTokens(build(wrapper,leaf('high',3,4)),build(first,leaf('low',3,4)),tokens),reads};
  };
  const expected=compare(original),actual=compare(current);
  assert.equal(expected.result,0,'a later lexical difference is past the actual lookahead');
  assert.equal(actual.result,expected.result);
  assert.ok(actual.reads>0,'pending lookahead prevents an otherwise shared-segment shortcut');
});

test('token comparison aligns unequal shared segments before visiting their common leaves',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const compare=(operations,reverse,materialized)=>{
    let reads=0;
    const leaves=Array.from({length:64},(_,index)=>({
      type:'token',kind:'identifier',get start(){reads++;return index;},get end(){reads++;return index+1;},
    }));
    let prefix={left:[],right:[leaves[0]],length:1,flat:null};
    for(const item of leaves.slice(1))prefix={left:prefix,right:[item],length:prefix.length+1,flat:null};
    const owner=part=>operations.shareChildren({},{[operations.CHAIN]:part});
    const short=owner(prefix),long=owner({left:prefix,right:[leaf('identifier',64,65)],length:65,flat:null});
    if(materialized){void short.children;void long.children;reads=0;}
    return {result:operations.preferredTokens(reverse?long:short,reverse?short:long,tokens),reads};
  };
  for(const reverse of[false,true])for(const materialized of[false,true]){
    const expected=compare(original,reverse,materialized),actual=compare(current,reverse,materialized);
    assert.equal(actual.result,expected.result);
    assert.equal(expected.result,0);
    assert.ok(expected.reads>0);
    assert.equal(actual.reads,0,'opening the longer part first exposes the exact common prefix');
  }
});

test('unequal shared segments retain pending lookahead and later conflict boundaries',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const compare=(operations,triviaOnly,reverse)=>{
    let reads=0;
    const first=leaf('identifier',0,1,{alone:true}),wrapper=node('wrapper',[first]);
    const sharedLeaves=Array.from({length:16},(_,index)=>({
      type:'token',kind:triviaOnly?null:'identifier',trivia:triviaOnly,
      get start(){reads++;return index+1;},end:index+2,
    }));
    let shared=[];
    for(const item of sharedLeaves)shared={left:shared,right:[item],length:(shared.length??0)+1,flat:null};
    const owner=(head,tail)=>operations.shareChildren({},{[operations.CHAIN]:{
      left:[head],right:{left:shared,right:tail,length:16+tail.length,flat:null},length:17+tail.length,flat:null,
    }});
    const left=owner(wrapper,[leaf('high',40,41),leaf('identifier',42,43)]);
    const right=owner(first,[leaf('low',40,41)]);
    return {result:operations.preferredTokens(reverse?right:left,reverse?left:right,tokens),reads};
  };
  for(const triviaOnly of[false,true])for(const reverse of[false,true]){
    const expected=compare(original,triviaOnly,reverse),actual=compare(current,triviaOnly,reverse);
    assert.equal(actual.result,expected.result);
    assert.equal(actual.result,triviaOnly?(reverse?-1:1):0);
    if(!triviaOnly)assert.ok(actual.reads>0,'the first shared real token must resolve the pending lookahead');
  }
});

test('token comparison recognizes a retained chain and its exact captured child array',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const compare=(operations,suffix,reverse)=>{
    let reads=0;
    const leaves=Array.from({length:64},(_,index)=>({
      type:'token',kind:'identifier',get start(){reads++;return index;},get end(){reads++;return index+1;},
    }));
    let prefix=[];
    for(const item of leaves)prefix={left:prefix,right:[item],length:prefix.length+1,flat:null};
    const owner=part=>operations.shareChildren({},{[operations.CHAIN]:part});
    let left=owner(prefix);
    let right={...left,field:'captured'};
    assert.equal(right.children,prefix.flat);
    if(suffix){
      left=owner({left:prefix,right:[leaf('high',64,65)],length:65,flat:null});
      right=owner({left:right.children,right:[leaf('low',64,65)],length:65,flat:null});
    }
    reads=0;
    return {result:operations.preferredTokens(reverse?right:left,reverse?left:right,tokens),reads};
  };
  for(const suffix of[false,true])for(const reverse of[false,true]){
    const expected=compare(original,suffix,reverse),actual=compare(current,suffix,reverse);
    assert.equal(actual.result,expected.result);
    assert.equal(actual.result,suffix?(reverse?-1:1):0);
    assert.ok(expected.reads>0);
    assert.equal(actual.reads,0,'only the identical cached array can stand for its owning chain');
  }
  const left=[leaf('high',0,1)],right=[leaf('low',0,1)];
  assert.equal(left.flat,right.flat,'different arrays share the Array prototype method');
  assert.notEqual(left,right);
  assert.equal(current.preferredTokens({children:left},{children:right},tokens),1);
});

test('captured child arrays cannot hide a pending reduction lookahead',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const compare=(operations,triviaOnly,reverse)=>{
    let reads=0;
    const first=leaf('identifier',0,1,{alone:true}),wrapper=node('wrapper',[first]);
    const values=Array.from({length:16},(_,index)=>({
      type:'token',kind:triviaOnly?null:'identifier',trivia:triviaOnly,
      get start(){reads++;return index+1;},end:index+2,
    }));
    let prefix=[];
    for(const item of values)prefix={left:prefix,right:[item],length:prefix.length+1,flat:null};
    const owner=part=>operations.shareChildren({},{[operations.CHAIN]:part});
    const capture={...owner(prefix),field:'captured'};
    const left=owner({left:[wrapper],right:{left:prefix,right:[leaf('high',40,41)],length:17,flat:null},length:18,flat:null});
    const right=owner({left:[first],right:{left:capture.children,right:[leaf('low',40,41)],length:17,flat:null},length:18,flat:null});
    reads=0;
    return {result:operations.preferredTokens(reverse?right:left,reverse?left:right,tokens),reads};
  };
  for(const triviaOnly of[false,true])for(const reverse of[false,true]){
    const expected=compare(original,triviaOnly,reverse),actual=compare(current,triviaOnly,reverse);
    assert.equal(actual.result,expected.result);
    assert.equal(actual.result,triviaOnly?(reverse?-1:1):0);
    if(!triviaOnly)assert.ok(actual.reads>0,'cached-array identity is insufficient until the lookahead is known');
  }
});

test('token comparison retains aliases trivia scanner endings and complete prefix decisions',async()=>{
  const {original,current}=await readTokenComparisonBaseline();
  const first=leaf('identifier',0,1);
  const wrapped=node('item',[first]);
  const virtual=leaf('layout',1,1,{scanned:true});
  const trivia=leaf(null,0,1,{trivia:true});
  const cases=[
    [[],[]],[[first],[first]],[[wrapped],[wrapped,virtual]],[[first],[first,virtual]],
    [[leaf('first_alias',0,1,{lexed:'identifier'})],[leaf('second_alias',0,1,{lexed:'identifier'})]],
    [[trivia,leaf('identifier',1,2)],[leaf('identifier',0,2)]],
    [[leaf('high',0,1)],[leaf('low',0,1)]],
    [[node('outer',[wrapped])],[wrapped]],
    [[node('outer',[wrapped]),leaf('high',3,4)],[wrapped,leaf('low',3,4)]],
    [[wrapped,virtual,leaf('identifier',1,2)],[wrapped,leaf('identifier',1,2)]],
  ];
  for(const [left,right]of cases){
    const expected=original.preferredTokens({children:left},{children:right},tokens);
    const actual=current.preferredTokens({children:left},{children:right},tokens);
    assert.equal(actual,expected);
    assert.equal(current.preferredTokens({children:right},{children:left},tokens),original.preferredTokens({children:right},{children:left},tokens));
  }
  assert.equal(current.preferredTokens({children:[wrapped]},{children:[wrapped,virtual]},tokens),-1);
  assert.equal(current.preferredTokens({children:[first]},{children:[first,virtual]},tokens),0);
});

test('linked token comparison preserves complete frozen grammar outcomes and unchanged resource refusals',async()=>{
  const {baseline}=await readTokenComparisonBaseline();
  const samples=[
    ['native-rust',controls.sources],
    ['native-python',[
      'def first(value):\n    """First."""\n    return value\n\ndef second(value):\n    return value + 1\n',
      'def item(value):\n    if value > 0:\n        return value\n    return 0\n',
    ]],
  ];
  for(const [language,sources]of samples){
    const grammar=parseGrammarLinks(nativeGrammarText(language));
    const current=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
    for(const source of sources)for(const options of[{}, {errorRecovery:true,recovery:'accept'}, {memoryLimit:0}, {stepLimit:0}, {maxDepth:0}]){
      assert.deepEqual(current.parseTree(source,options),previous.parseTree(source,options),language+': '+source);
    }
  }
});
