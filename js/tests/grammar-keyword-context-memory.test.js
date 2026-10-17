import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {Executor,KeywordLexing} from '../src/grammar-runtime/executor.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';
import {packLexicalCacheResults,readLexicalCacheResults,packTriviaCacheResult} from '../src/grammar-runtime/cache-storage.js';
import {PrefixReachCache} from '../src/grammar-runtime/prefix-reach-cache.js';

const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
const pins={
  'js/src/grammar-runtime/executor.js':'3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81',
  'js/src/grammar-runtime.js':'9157c49c2d917dff5d3e13dbbbf60940971a837f6edc588fc80bf2682362cef1',
};
function frozenKeywordModule(row,overrides=new Map()) {
  assert.equal(createHash('sha256').update(row.source).digest('hex'),pins[row.path]);
  assert.equal(row.sha256,pins[row.path]);assert.equal(Buffer.byteLength(row.source),row.bytes);
  const base=new URL('../../'+row.path,import.meta.url);
  const source=row.source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>{
    const address=new URL(path,base).href;
    return 'from '+quote+(overrides.get(address)??address)+quote;
  }).replaceAll('import.meta.url',JSON.stringify(base.href));
  return 'data:text/javascript;base64,'+Buffer.from(source).toString('base64');
}
async function frozenKeywordParser() {
  assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
  assert.deepEqual(controls.baseline.modules.map(row=>row.path),Object.keys(pins));
  const executor=controls.baseline.modules[0],driver=controls.baseline.modules[1];
  const overrides=new Map([[new URL('../../'+executor.path,import.meta.url).href,frozenKeywordModule(executor)]]);
  return import(frozenKeywordModule(driver,overrides));
}

function readKeywordCallGraph(executor) {
  const pending=[],indices=new Map(),nodes=[];
  const index=call=>{
    if(call===null)return null;
    if(!indices.has(call)){indices.set(call,pending.length);pending.push(call);}
    return indices.get(call);
  };
  for(const cell of executor.memo.values())index(cell.keywordContext??cell);
  for(const collection of[executor.keywords.matched,executor.keywords.matchedImmediate,executor.keywords.matchedSeparators])for(const calls of collection.values())for(const call of calls)index(call);
  for(const join of executor.keywords.matchedJoins.values())for(const call of join.calls)index(call);
  let arrays=0,sets=0;
  for(let at=0;at<pending.length;at++){
    const call=pending[at],parents=[...call.parents];
    assert.equal(new Set(parents).size,parents.length);
    if(Array.isArray(call.parents)){arrays++;assert.ok(parents.length<=8);}
    else sets++;
    nodes.push({position:call.position,builds:call.builds,nodeKind:call.nodeKind,name:call.name,parents:parents.map(index)});
  }
  return {nodes,arrays,sets};
}

test('keyword call contexts retain live ancestry without memo results or seeds',()=>{
  const parent={parents:new Set([null]),position:0,builds:true,nodeKind:'parent',name:'parent'};
  const child={parents:new Set([parent]),position:1,builds:true,nodeKind:'child',name:'child'};
  const entry={keywordContext:child,results:[{privateResult:true}],seed:[{privateSeed:true}],rule:{privateRule:true}};
  const keywords=new KeywordLexing(null);
  keywords.match('1|2',entry);keywords.match('1|2',entry,true);
  keywords.matchSeparator(1,entry);keywords.matchJoin(1,2,entry);
  const calls=[...keywords.matched.get('1|2'),...keywords.matchedImmediate.get('1|2'),...keywords.matchedSeparators.get(1),...keywords.matchedJoins.get(1).calls];
  for(const call of calls){assert.equal(call,child);assert.notEqual(call,entry);assert.equal(call.parents,child.parents);}
  const later={parents:new Set([null]),position:0,builds:false,nodeKind:'later',name:'later'};
  child.parents.add(later);parent.parents.add(child);
  for(const call of calls)assert.ok(call.parents.has(later));
  const pending=[...calls],seen=new Set();
  for(let at=0;at<pending.length;at++){
    const call=pending[at];if(call===null||seen.has(call))continue;seen.add(call);
    assert.deepEqual(Object.keys(call).sort(),['builds','name','nodeKind','parents','position']);
    pending.push(...call.parents);
  }
  assert.equal(seen.size,3);
});

test('actual Rust keyword contexts exclude memo payloads while complete tiny parse outputs match the frozen driver',async()=>{
  const baseline=await frozenKeywordParser();
  const grammar=parseGrammarLinks(nativeGrammarText('native-rust'));
  const current=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
  const original=KeywordLexing.prototype.conflicts,snapshots=[];
  KeywordLexing.prototype.conflicts=function(root){
    snapshots.push(...[...this.matched.values(),...this.matchedImmediate.values(),...this.matchedSeparators.values(),...[...this.matchedJoins.values()].map(join=>join.calls)].flatMap(calls=>[...calls]));
    return original.call(this,root);
  };
  try{
    for(const source of controls.sources){
      assert.ok(Buffer.byteLength(source)<128);
      for(const options of [{},{errorRecovery:true,recovery:'accept'}])assert.deepEqual(current.parseTree(source,options),previous.parseTree(source,options),source);
    }
  }finally{KeywordLexing.prototype.conflicts=original;}
  assert.ok(snapshots.some(Boolean),'actual native parsing recorded keyword contexts');
  const pending=[...snapshots],seen=new Set();
  for(let at=0;at<pending.length;at++){
    const call=pending[at];if(call===null||seen.has(call))continue;seen.add(call);
    assert.deepEqual(Object.keys(call).sort(),['builds','name','nodeKind','parents','position']);
    pending.push(...call.parents);
  }
  assert.ok(seen.size>0);
});

test('keyword context separation preserves unchanged parse budgets and rejection decisions',async()=>{
  const baseline=await frozenKeywordParser(),grammar=parseGrammarLinks(nativeGrammarText('native-rust'));
  const current=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
  for(const memoryLimit of [1,20,1000])for(const errorRecovery of [false,true]){
    const options={memoryLimit,errorRecovery,recovery:'accept'};
    assert.deepEqual(current.parseTree(controls.sources[0],options),previous.parseTree(controls.sources[0],options));
  }
});

test('finished rule call storage preserves parse decisions and budgets',async()=>{
  const baseline=await frozenKeywordParser();
  const frozen=await import(frozenKeywordModule(controls.baseline.modules[0]));
  const grammar=parseGrammarLinks(nativeGrammarText('native-rust'));
  const current=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
  const original=Executor.prototype.run,prior=frozen.Executor.prototype.run;
  const actual=[],expected=[];
  Executor.prototype.run=function(...args){
    const result=original.apply(this,args);
    for(const cell of this.memo.values())assert.deepEqual(Object.keys(cell).sort(),['keywordContext','results']);
    actual.push({cells:this.budget.memory.cells,steps:this.budget.steps,memo:this.memo.size,result:structuredClone(result)});
    return result;
  };
  frozen.Executor.prototype.run=function(...args){
    const result=prior.apply(this,args);
    expected.push({cells:this.budget.memory.cells,steps:this.budget.steps,memo:this.memo.size,result:structuredClone(result)});
    return result;
  };
  try{
    for(const source of controls.sources)for(const memoLimit of [0,20,1000000]){
      const options={memoLimit,errorRecovery:true,recovery:'accept'};
      assert.deepEqual(current.parseTree(source,options),previous.parseTree(source,options));
    }
  }finally{
    Executor.prototype.run=original;frozen.Executor.prototype.run=prior;
  }
  assert.ok(actual.some(run=>run.memo>20));
  assert.deepEqual(actual,expected,'all full run results, step counts, cell budgets and memo admission counts remain equal');
});

test('small keyword parent collections preserve the entire frozen call graph',async()=>{
  const baseline=await frozenKeywordParser();
  const frozen=await import(frozenKeywordModule(controls.baseline.modules[0]));
  const grammar=parseGrammarLinks(nativeGrammarText('native-rust'));
  const current=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
  const original=Executor.prototype.run,prior=frozen.Executor.prototype.run;
  const actual=[],expected=[];
  Executor.prototype.run=function(...args){const result=original.apply(this,args);actual.push(readKeywordCallGraph(this));return result;};
  frozen.Executor.prototype.run=function(...args){const result=prior.apply(this,args);expected.push(readKeywordCallGraph(this));return result;};
  try{
    for(const source of controls.sources)assert.deepEqual(current.parseTree(source,{errorRecovery:true,recovery:'accept'}),previous.parseTree(source,{errorRecovery:true,recovery:'accept'}));
  }finally{Executor.prototype.run=original;frozen.Executor.prototype.run=prior;}
  assert.deepEqual(actual.map(row=>row.nodes),expected.map(row=>row.nodes),'all call identities, cyclic edges, shared parents and insertion order remain exact');
  assert.ok(actual.every(row=>row.arrays>row.sets));
  assert.ok(expected.every(row=>row.arrays===0));
  assert.ok(actual.some(row=>row.nodes.some(node=>node.parents.length>8)),'actual parsing reaches promotion to a set');
});

test('rule call key construction preserves every frozen key byte',async()=>{
  await frozenKeywordParser();
  const declaration=source=>{
    const lines=source.split('\n'),start=lines.findIndex(line=>line.trim().startsWith('const key')&&line.includes('rule.index'));
    const end=lines.findIndex((line,index)=>index>start&&line.includes('const known = this.memo.get(key)'));
    assert.ok(start>=0&&end>start);return new Function('rule','position','state','inToken','quiet',lines.slice(start,end).join('\n')+'\nreturn key;');
  };
  const current=declaration(readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8'));
  const previous=declaration(controls.baseline.modules[0].source);
  for(const index of[0,97])for(const position of[0,1048576])for(const key of['','0||[]|{}','😀|quiet','x'.repeat(64),'😀'.repeat(33),'x'.repeat(1000)])for(const inToken of[false,true])for(const inExtra of[false,true,'nesting'])for(const extraStart of[-1,0,65535])for(const quiet of['','|quiet','|chained']){
    const context={inExtra,extraStart},args=[{index},position,{key},inToken,quiet];
    assert.equal(current.apply(context,args),previous.apply(context,args));
  }
  for(const value of[undefined,null,NaN,Infinity,-0]){
    const context={inExtra:value,extraStart:value},args=[{index:value},value,{key:value},false,''];
    assert.equal(current.apply(context,args),previous.apply(context,args));
  }
  for(const operation of[current,previous])assert.throws(()=>operation.call({inExtra:false,extraStart:-1},{index:Symbol('key')},0,{key:''},false,''),TypeError);
  const trace=operation=>{
    const events=[],value=name=>({toString(){events.push(name);return name;}});
    const key=operation.call({get inExtra(){events.push('extra');return value('extra value');},get extraStart(){events.push('start');return value('start value');}},{get index(){events.push('rule');return value('rule value');}},value('position'),{get key(){events.push('state');return value('state value');}},false,value('quiet'));
    return {key,events};
  };
  assert.deepEqual(trace(current),trace(previous),'field reads and string coercions retain their original order');
  const args=[{index:0},0,{key:''},false,''];
  const plain={inExtra:false,extraStart:-1};
  assert.equal(current.call({...plain,lexerFamily:''},...args),previous.call(plain,...args));
  assert.notEqual(current.call({...plain,lexerFamily:'source-a'},...args),current.call({...plain,lexerFamily:'source-b'},...args));
});

test('lazy child owners share one accessor while chains and frozen output identities remain exact',async()=>{
  await frozenKeywordParser();
  const readOperations=source=>{
    const start=source.indexOf("const CHAIN = Symbol('children chain');");
    const end=source.indexOf('// A copy of a result with',start);
    assert.ok(start>=0&&end>start);
    return new Function(source.slice(start,end)+'\nreturn {CHAIN,shareChildren,copyNode,partOf,childCount};')();
  };
  const original=readOperations(controls.baseline.modules[0].source);
  const current=readOperations(readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8'));
  const build=operations=>{
    const first={kind:'first'},second={kind:'second'},third={kind:'third'};
    const chain={left:[first],right:[second],length:2};
    const independent={left:[third],right:[],length:1};
    const parts=[chain.left,chain.right,independent.left,independent.right];
    const owner=operations.shareChildren({kind:'owner'},{[operations.CHAIN]:chain});
    const copy=operations.copyNode(owner,{kind:'copy'});
    const other=operations.shareChildren({kind:'other'},{[operations.CHAIN]:independent});
    const descriptors=[owner,copy,other].map(value=>Object.getOwnPropertyDescriptor(value,'children'));
    for(const descriptor of descriptors){assert.equal(descriptor.enumerable,true);assert.equal(descriptor.configurable,true);assert.equal(descriptor.set,undefined);}
    assert.equal(chain.flat,undefined);assert.equal(independent.flat,undefined);
    assert.equal(operations.childCount(owner),2);assert.equal(operations.childCount(other),1);
    assert.equal(operations.partOf(owner),chain);assert.equal(operations.partOf(copy),chain);
    assert.equal(copy.children,owner.children);assert.deepEqual(owner.children,[first,second]);
    assert.equal(independent.flat,undefined,'reading one chain does not flatten another');
    assert.deepEqual(other.children,[third]);assert.notEqual(other.children,owner.children);
    if(operations===original){
      assert.equal(chain.left,null);assert.equal(chain.right,null);
      assert.equal(independent.left,null);assert.equal(independent.right,null);
    }else{
      assert.equal(chain.left,parts[0]);assert.equal(chain.right,parts[1]);
      assert.equal(independent.left,parts[2]);assert.equal(independent.right,parts[3]);
    }
    return {descriptors,owners:structuredClone([owner,copy,other])};
  };
  const expected=build(original),actual=build(current);
  assert.deepEqual(actual.owners,expected.owners);
  assert.equal(new Set(actual.descriptors.map(row=>row.get)).size,1,'all lazy owners use the same accessor');
  assert.equal(new Set(expected.descriptors.map(row=>row.get)).size,3,'the authenticated original allocated a getter for every owner');
});

test('silent reductions preserve every frozen tag without flattening plural child chains',async()=>{
  await frozenKeywordParser();
  const readOperations=source=>{
    const chainStart=source.indexOf("const CHAIN = Symbol('children chain');");
    const chainEnd=source.indexOf('function joinResults(',chainStart);
    const start=source.indexOf('function reducedAlone('),end=source.indexOf('// Which of two results an LR parser keeps',start);
    assert.ok(chainStart>=0&&chainEnd>chainStart&&start>=0&&end>start);
    return new Function('isTrivia',source.slice(chainStart,chainEnd)+'\n'+source.slice(start,end)+'\nreturn {CHAIN,shareChildren,reducedAlone};')(child=>child.trivia===true);
  };
  const original=readOperations(controls.baseline.modules[0].source);
  const current=readOperations(readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8'));
  const cases=[[],[{type:'token',trivia:true}],[{type:'token'}],[{type:'node'}],
    [{type:'token',alone:true,reducedTo:['control'],forkedTo:['control']}],
    [{type:'token',trivia:true},{type:'node'},{type:'token',trivia:true}],
    [{type:'token'},{type:'token',trivia:true},{type:'node'}],
    Array.from({length:32},(_,index)=>({type:index%2?'token':'node',trivia:index%3===0})),
  ];
  for(const children of cases)for(const ranked of[false,true])for(const forked of[false,true]){
    const reduce=operations=>{
      const items=structuredClone(children).map((child,index)=>({...child,kind:'child',start:index,end:index+1}));
      let part=[];
      const chains=[];
      for(const child of items){part={left:part,right:[child],length:chains.length+1,flat:null};chains.push(part);}
      const source=Array.isArray(part)?{children:part}:{[operations.CHAIN]:part};
      const owner=operations.shareChildren({end:items.length,state:{key:''},dynamic:0,cost:0,precedence:null,tail:null,ambiguous:false},source);
      const result=operations.reducedAlone(owner,'control',ranked,forked);
      const flattened=chains.filter(chain=>chain.flat!==null).length;
      const same=result===owner;
      const value=structuredClone(result);
      if(same)for(let at=0;at<items.length;at++)assert.equal(result.children[at],items[at]);
      return {value,same,flattened};
    };
    const expected=reduce(original),actual=reduce(current);
    assert.deepEqual(actual.value,expected.value);assert.equal(actual.same,expected.same);
    if(children.filter(child=>child.trivia!==true).length>1){
      assert.equal(actual.flattened,0,'plural chains remain entirely lazy');
      assert.ok(expected.flattened>0,'the authenticated original flattened the chain');
    }
  }
});

test('silent token rules read children only for action scratch nodes',async()=>{
  await frozenKeywordParser();
  const frozen=await import(frozenKeywordModule(controls.baseline.modules[0]));
  for(const action of[undefined,{kind:'control'}])for(const refused of[false,true]){
    const run=prototype=>{
      let reads=0,actions=0;
      const children=[{type:'token',kind:'child'}];
      const result={end:1,ambiguous:false,tail:null,get children(){reads++;return children;}};
      const rule={kind:'silent',nodeKind:'control',expression:{kind:'empty'},action};
      const executor={program:{conflicts:new Set()},evaluate:()=>[result],runAction:(_rule,value,scratch)=>{
        if(!action)return value;
        actions++;assert.equal(scratch.children,children);assert.equal(scratch.start,0);assert.equal(scratch.end,1);
        return refused?null:value;
      }};
      const built=prototype.ruleBody.call(executor,rule,0,{key:''},true);
      assert.deepEqual(built,action&&refused?[]:[result]);
      if(built.length)assert.equal(built[0],result);
      return {reads,actions};
    };
    const expected=run(frozen.Executor.prototype),actual=run(Executor.prototype);
    assert.equal(actual.actions,expected.actions);
    assert.equal(actual.reads,action?expected.reads:0);
    assert.equal(expected.reads,1);
  }
});

test('compact lexical results preserve every field and conservative fallback',()=>{
  const state={key:'control'},children=[];
  const values=Array.from({length:40},(_,end)=>({end:end===0?-0:end,state,children,dynamic:-0,precedence:null,tail:null,ambiguous:false,cost:NaN,before:undefined,open:false}));
  const stored=packLexicalCacheResults(values);
  assert.ok(!Array.isArray(stored));assert.ok(stored.ends instanceof Float64Array);
  const decoded=readLexicalCacheResults(stored);
  assert.deepEqual(decoded,values);assert.ok(Object.is(decoded[0].end,-0));
  for(let at=0;at<values.length;at++){
    assert.deepEqual(Reflect.ownKeys(decoded[at]),Reflect.ownKeys(values[at]));
    assert.equal(decoded[at].state,state);assert.equal(decoded[at].children,children);
    assert.deepEqual(Object.getOwnPropertyDescriptors(decoded[at]),Object.getOwnPropertyDescriptors(values[at]));
  }
  const mixed=values.map((value,index)=>index%2?Object.fromEntries(Object.entries(value).reverse()):value);
  const mixedStored=packLexicalCacheResults(mixed);assert.ok(!Array.isArray(mixedStored));
  const mixedDecoded=readLexicalCacheResults(mixedStored);assert.deepEqual(mixedDecoded,mixed);
  for(let at=0;at<mixed.length;at++)assert.deepEqual(Reflect.ownKeys(mixedDecoded[at]),Reflect.ownKeys(mixed[at]));
  for(const mutate of[
    array=>{array[3].state={key:'control'};},array=>{array[3].children=[];},array=>{array[3].cost=0;},
    array=>{array[3].dynamic=0;},array=>{array[3].end=Infinity;},array=>{array[3].extra=true;},
    array=>{array[3][Symbol('extra')]=true;},array=>{Object.defineProperty(array[3],'end',{enumerable:false});},
    array=>{Object.setPrototypeOf(array[3],{control:true});},array=>{Object.defineProperty(array[3],'end',{get(){throw new Error('unread cache getter');}});},
  ]){
    const changed=values.map(value=>({...value}));mutate(changed);
    assert.equal(packLexicalCacheResults(changed),changed,'nonuniform or unknown representations remain complete arrays');
  }
  const small=values.slice(0,15);assert.equal(packLexicalCacheResults(small),small);assert.equal(readLexicalCacheResults(small),small);
});

test('compact padding preserves every leaf state span and fallback boundary',()=>{
  const state={key:'control'},pool=new Map();
  const make=(start,length)=>({end:start+length,leaves:Array.from({length},(_,index)=>({type:'token',kind:null,start:start+index,end:start+index+1,trivia:true})),state,states:Array(length+1).fill(state)});
  const source=make(0,64),packed=packTriviaCacheResult(source,0,pool,1000);
  assert.notEqual(packed,source);assert.deepEqual(structuredClone(packed),source);assert.equal(pool.size,64);
  for(let at=0;at<64;at++)assert.equal(packed.leaves[at],source.leaves[at]);
  for(const value of packed.states)assert.equal(value,state);
  const suffix=make(16,48),shared=packTriviaCacheResult(suffix,16,pool,1000);
  assert.deepEqual(structuredClone(shared),suffix);assert.equal(pool.size,64);
  for(let at=0;at<48;at++)assert.equal(shared.leaves[at],source.leaves[at+16]);
  assert.equal(Object.getOwnPropertyDescriptor(packed,'leaves').get,Object.getOwnPropertyDescriptor(shared,'leaves').get);
  assert.equal(Object.getOwnPropertyDescriptor(packed,'states').get,Object.getOwnPropertyDescriptor(shared,'states').get);
  for(const limit of[0,17]){const limited=new Map(),value=packTriviaCacheResult(source,0,limited,limit);assert.equal(limited.size,limit);assert.deepEqual(structuredClone(value),source);}
  for(const mutate of[
    value=>{value.leaves[2].kind='comment';},value=>{value.leaves[2].projectionName='padding';},
    value=>{value.leaves[2].end++;},value=>{value.leaves[2].start++;},value=>{value.leaves[2].trivia=false;},
    value=>{value.states[2]={key:'control'};},value=>{value.states.pop();},
  ]){const changed=make(0,64);mutate(changed);assert.equal(packTriviaCacheResult(changed,0,new Map(),1000),changed);}
  const small=make(0,15);assert.equal(packTriviaCacheResult(small,0,new Map(),1000),small);
});

test('paged prefix reach storage retains every exact Map key value and update',()=>{
  for(const maximum of[1024,Number.MAX_SAFE_INTEGER]){
    const current=new PrefixReachCache(maximum),previous=new Map(),object={identity:true},symbol=Symbol('key');
    const keys=[];
    const put=(key,value)=>{assert.equal(current.set(key,value),current);previous.set(key,value);if(!keys.includes(key))keys.push(key);assert.equal(current.size,previous.size);for(const stored of keys){assert.equal(current.has(stored),previous.has(stored));assert.ok(Object.is(current.get(stored),previous.get(stored)));}};
    for(const state of['','0||[]|{}','spaces | Unicode 😀','x'.repeat(100)])for(let position=0;position<130;position++)put(position+'|'+state,position+1);
    for(const key of['0|','8|0||[]|{}','-0|state','01|state','-1|state','1.5|state','1e3|state','9007199254740993|state','ordinary',object,symbol,undefined,NaN]){
      for(const value of[undefined,null,false,-0,NaN,Infinity,-1,0,7,0xffffffff,Number.MAX_SAFE_INTEGER,object])put(key,value);
    }
    assert.equal(current.has('absent'),false);assert.equal(current.get('absent'),undefined);
    const pages=[...current.states.values()].flatMap(value=>[...value.values()]);
    assert.ok(pages.some(page=>page instanceof(maximum>0xfffffffe?Float64Array:Uint32Array)),'dense pages replace populated numeric buckets');
    assert.ok(current.other.has('ordinary'),'noncanonical keys remain exact Map entries');
  }
});

test('indented Rust preserves frozen trees steps charged cells and memo decisions',async()=>{
  const baseline=await frozenKeywordParser(),frozen=await import(frozenKeywordModule(controls.baseline.modules[0]));
  const grammar=parseGrammarLinks(nativeGrammarText('native-rust')),current=createGrammarParser(grammar),previous=baseline.createGrammarParser(grammar);
  const padding=' '.repeat(32),body='fn f(){\n'+padding+'let value=1;\n'+padding+'value+2\n}';
  const sources=[body,body.replaceAll('\n','\r\n'),'\uFEFF'+body,'fn f(){'+padding+'/* note */'+padding+'let value=1;'+padding+'value+2}'];
  const run=Executor.prototype.run,prior=frozen.Executor.prototype.run,actual=[],expected=[];
  let packed=0;
  const observe=executor=>({cells:executor.budget.memory.cells,steps:executor.budget.steps,memo:executor.memo.size,trivia:executor.triviaMemo.size,prefixEntries:[...executor.prefixReaches?.values()??[]].reduce((sum,cache)=>sum+cache.size,0)});
  Executor.prototype.run=function(...args){const result=run.apply(this,args);actual.push(observe(this));for(const value of this.triviaMemo.values())if(Object.getOwnPropertyDescriptor(value,'leaves').get)packed++;return result;};
  frozen.Executor.prototype.run=function(...args){const result=prior.apply(this,args);expected.push(observe(this));return result;};
  try{
    for(const source of sources){assert.ok(Buffer.byteLength(source)<256);for(const memoLimit of[0,20,1000000])for(const memoryLimit of[1000,2000000])for(const errorRecovery of[false,true]){
      const options={memoLimit,memoryLimit,errorRecovery,recovery:'accept'};
      assert.deepEqual(current.parseTree(source,options),previous.parseTree(source,options));
    }}
  }finally{Executor.prototype.run=run;frozen.Executor.prototype.run=prior;}
  assert.deepEqual(actual,expected,'all steps, charged cells, rule memo admission and trivia entries remain exact');
  assert.ok(packed>0,'actual native parsing uses compact padding storage');
});

test('complete source diagnostics observe real native runs and restore the executor',async()=>{
  const {createCompleteRustSourceArchives,replayCompleteRustSourceArchives}=await import('../../experiments/original-link-complete-table-projection/archive-transport.mjs');
  const {LinkNetwork}=await import('../src/network.js');
  const {encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
  const original=Executor.prototype.run,events=[],source='fn f(){}';
  const archives=createCompleteRustSourceArchives(source,{parseSource:source=>LinkNetwork.parse(source,'Rust'),encodeBinary:encodeLinkNetworkBinary,onNativeRun:event=>events.push(event)});
  assert.equal(Executor.prototype.run,original);
  assert.ok(events.length>=2);assert.equal(events[0].phase,'begin');assert.equal(events.at(-1).phase,'end');
  for(const event of events){assert.equal(event.inputBytes,Buffer.byteLength(source));assert.ok(Number.isSafeInteger(event.memoCells));assert.ok(Number.isSafeInteger(event.storedRules));assert.ok(Number.isSafeInteger(event.heap));}
  assert.deepEqual(replayCompleteRustSourceArchives(archives,{decodeBinary:decodeLinkNetworkBinary}),{binaryReplay:source,textReplay:source});
  assert.throws(()=>createCompleteRustSourceArchives(source,{parseSource:source=>LinkNetwork.parse(source,'Rust'),encodeBinary:encodeLinkNetworkBinary,onNativeRun:()=>{throw new Error('diagnostic control');}}),/diagnostic control/);
  assert.equal(Executor.prototype.run,original);
});

test('valid generated source archives refuse failed or recovered parse graphs',async()=>{
  const {requireSuccessfulSourceArchive}=await import('../../experiments/original-link-complete-table-projection/archive-transport.mjs');
  const {LinkNetwork}=await import('../src/network.js');
  const valid=LinkNetwork.parse('fn f(){}','Rust');
  assert.equal(requireSuccessfulSourceArchive(valid),valid);
  for(const source of ['fn f(){let value=;}','fn f( {']){
    const recovered=LinkNetwork.parse(source,'Rust');
    assert.equal(recovered.reconstructText(),source);
    assert.throws(()=>requireSuccessfulSourceArchive(recovered),/recovered or failed parse/);
  }
});

test('native source progress diagnostics preserve archives and restore both executor methods',async()=>{
  const {createCompleteRustSourceArchives}=await import('../../experiments/original-link-complete-table-projection/archive-transport.mjs');
  const {LinkNetwork}=await import('../src/network.js');
  const {encodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
  const original=Executor.prototype.run,originalRetain=Executor.prototype.retain;
  const source='fn f(){}',events=[];
  const options={parseSource:source=>LinkNetwork.parse(source,'Rust'),encodeBinary:encodeLinkNetworkBinary};
  const baseline=createCompleteRustSourceArchives(source,options);
  const observed=createCompleteRustSourceArchives(source,{...options,nativeRunCellInterval:16,onNativeRun:event=>events.push(event)});
  assert.deepEqual(observed,baseline,'all canonical text, binary bytes and record limits remain identical');
  assert.equal(Executor.prototype.run,original);assert.equal(Executor.prototype.retain,originalRetain);
  assert.equal(events[0].phase,'begin');assert.equal(events.at(-1).phase,'end');
  assert.ok(events.some(event=>event.phase==='progress'));
  for(const event of events){
    for(const value of Object.values(event))assert.ok(['string','number','boolean'].includes(typeof value));
    assert.ok(Number.isSafeInteger(event.memoCells));assert.ok(Number.isSafeInteger(event.storedRules));
    for(const position of[event.farthest,event.callPosition])assert.ok(Number.isSafeInteger(position)&&position>=0&&position<=event.inputBytes);
    assert.ok(Number.isSafeInteger(event.depth)&&event.depth>=0);
  }
  for(const value of [0,-1,1.5,NaN,Infinity])assert.throws(()=>createCompleteRustSourceArchives(source,{...options,nativeRunCellInterval:value,onNativeRun:()=>{}}),/positive integer cell interval/);
  assert.throws(()=>createCompleteRustSourceArchives(source,{...options,nativeRunCellInterval:16,onNativeRun:event=>{if(event.phase==='progress')throw new Error('progress observer control');}}),/progress observer control/);
  assert.equal(Executor.prototype.run,original);assert.equal(Executor.prototype.retain,originalRetain);
  assert.deepEqual(createCompleteRustSourceArchives(source,options),baseline);
});
