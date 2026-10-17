import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {acceptScannerPreemptionBoundary,readScannerContinuationAction} from '../src/translation/frontend-rules.js';

const controls=readProjectData(new URL('../../parity/fixtures/keyword-parse-context-controls.lino',import.meta.url));
const baseline=controls.baseline.modules.find(row=>row.path==='js/src/grammar-runtime/executor.js');
const hash='3c65f3d601f61310a4a2c2230a71730aea1fc6086a1f863f24948a79bb30ed81';
assert.equal(controls.baseline.revision,'ae76b70abaa60d2ec051088da08dcd55fb3f73f4');
assert.equal(baseline.sha256,hash);
assert.equal(createHash('sha256').update(baseline.source).digest('hex'),hash);
assert.equal(Buffer.byteLength(baseline.source),baseline.bytes);

function readScannerPreemptionOperations(source) {
  const chainStart=source.indexOf("const CHAIN = Symbol('children chain');");
  const chainEnd=source.indexOf('// A result whose MISSING leaf',chainStart);
  const walkStart=source.indexOf('function widthless(');
  const walkEnd=source.indexOf('// Whether two subtrees hold the same tokens',walkStart);
  assert.ok(chainStart>=0&&chainEnd>chainStart&&walkStart>=0&&walkEnd>walkStart);
  const scalars=['scannedToken','isTrivia','isSeparator'].map(name=>{
    const start=source.indexOf('function '+name+'('),end=source.indexOf('\n}',start)+2;
    assert.ok(start>=0&&end>start);return source.slice(start,end);
  }).join('\n');
  return new Function('acceptScannerPreemptionBoundary','readScannerContinuationAction','NO_CHILDREN',
    scalars+'\n'+source.slice(chainStart,chainEnd)+'\n'+source.slice(walkStart,walkEnd)
    +'\nreturn {preempted,lexedPast};')(
      acceptScannerPreemptionBoundary,readScannerContinuationAction,Object.freeze([]));
}

const original=readScannerPreemptionOperations(baseline.source);
const current=readScannerPreemptionOperations(readFileSync(new URL('../src/grammar-runtime/executor.js',import.meta.url),'utf8'));
const token=(kind,start,end,fields={})=>({type:'token',kind,start,end,...fields});
const result=(name,children,end,cost=0,state='same')=>({name,children,end,cost,state:{key:state}});
const boundary=offset=>result('boundary',[token('boundary',offset,offset,{scanned:true})],offset);
const names=values=>values===null?null:[...values].map(value=>value.name);

test('scanner preemption index preserves complete frozen state cost alias and span decisions',()=>{
  for(const identity of ['kind','origin','different','undefined-origin','type','span'])
    for(const state of ['same','different'])for(const leftCost of [0,1])for(const otherCost of [0,1])
      for(const shifted of [false,true])for(const suffix of ['after','empty','overlap','reverse','virtual']){
        const run=operations=>{
          const last=token('source',0,1,{lexed:'source-rule'});
          const matched={...last};
          if(identity==='origin')matched.kind='renamed';
          if(identity==='different'){matched.kind='other';matched.lexed='other-rule';}
          if(identity==='undefined-origin'){matched.kind='other';delete matched.lexed;delete last.lexed;}
          if(identity==='type')matched.type='missing';
          if(identity==='span')matched.start=-1;
          const next=suffix==='after'?[token('next',2,3)]:suffix==='empty'?[]:
            suffix==='overlap'?[token('overlap',0,3)]:suffix==='reverse'?[token('next',2,3),token('backward',0,0)]:
              [token('virtual',2,2,{scanned:true})];
          const left=result('left',[last],1,leftCost);
          const other=result('other',[matched,...(shifted?[token('shifted',1,1,{scanned:true})]:[]),...next],3,otherCost,state);
          const continued=[[left,[boundary(1)]],[other,[]]];
          return {pruned:names(operations.preempted(continued)),past:operations.lexedPast(other,last,leftCost)};
        };
        assert.deepEqual(run(current),run(original),JSON.stringify({identity,state,leftCost,otherCost,shifted,suffix}));
      }
});

test('scanner preemption retains left major identity order and uses already pruned candidates',()=>{
  const run=operations=>{
    const first=token('first',0,1),second=token('second',4,5);
    const left=result('left',[second],5),later=result('later',[first],1);
    const earlyOther=result('early other',[first,token('next',2,3)],3);
    const lateOther=result('late other',[second,token('next',6,7)],7);
    const order=names(operations.preempted([[left,[boundary(5)]],[later,[boundary(1)]],[earlyOther,[]],[lateOther,[]],[lateOther,[]]]));
    const a=result('a',[first],1);
    const b=result('b',[first,token('middle',2,3)],3);
    const c=result('c',[first,token('shifted',1,1,{scanned:true}),token('middle',2,3),token('end',4,5)],5);
    const pruned=names(operations.preempted([[a,[boundary(1)]],[b,[boundary(3)]],[c,[]]]));
    return {order,pruned};
  };
  assert.deepEqual(run(current),run(original));
  assert.deepEqual(run(current),{order:['late other','early other'],pruned:['b','c']});
});

test('scanner preemption index preserves arbitrary leaf boundaries and strict token identities',()=>{
  for(const start of [-1,-0,0,NaN,Infinity])for(const end of [-0,1,NaN,Infinity])for(const kind of [undefined,null,'token',NaN]){
    const run=operations=>{
      const last=token(kind,start,end),left=result('left',[last],0);
      const other=result('other',[{...last},token('next',Infinity,Infinity)],Infinity);
      return names(operations.preempted([[left,[boundary(0)]],[other,[]]]));
    };
    assert.deepEqual(run(current),run(original));
  }
  assert.equal(current.preempted([]),null);
  assert.equal(current.preempted([[result('empty',[],0),[boundary(0)]]]),null);
});

test('scanner preemption keeps exact trace callback mutations and failures in original order',()=>{
  const saved=Object.getOwnPropertyDescriptor(globalThis,'__preemptTrace');
  const run=operations=>{
    const first=token('first',0,1);
    const a=result('a',[first],1),b=result('b',[first,token('middle',2,3)],3);
    const c=result('c',[first,token('shifted',1,1,{scanned:true}),token('middle',2,3),token('last',4,5)],5);
    const events=[];
    globalThis.__preemptTrace=(left,other)=>{events.push([left.name,other.name]);if(other===b)c.cost=-1;};
    const pruned=names(operations.preempted([[a,[boundary(1)]],[b,[boundary(3)]],[c,[]]]));
    globalThis.__preemptTrace=true;
    assert.throws(()=>operations.preempted([[a,[boundary(1)]],[b,[]]]),TypeError);
    return {pruned,events};
  };
  try{
    assert.deepEqual(run(current),run(original));
    assert.deepEqual(run(current),{pruned:['b'],events:[['a','b']]});
  }finally{
    if(saved)Object.defineProperty(globalThis,'__preemptTrace',saved);
    else delete globalThis.__preemptTrace;
  }
});

test('scanner preemption indexes virtual boundaries without repeating a leaf walk for every prefix pair',()=>{
  const run=operations=>{
    let reads=0;
    const leaves=[];
    for(let index=0;index<48;index++){
      leaves.push({type:'token',kind:'word',get start(){reads++;return index*2;},get end(){reads++;return index*2+1;}});
      leaves.push({type:'token',kind:'shifted',scanned:true,get start(){reads++;return index*2+1;},get end(){reads++;return index*2+1;}});
    }
    const continued=Array.from({length:48},(_,index)=>{
      const end=index*2+1;
      return [result('prefix '+index,leaves.slice(0,(index+1)*2),end),[boundary(end)]];
    });
    return {pruned:names(operations.preempted(continued)),reads};
  };
  const expected=run(original),actual=run(current);
  assert.equal(expected.pruned,null,'each prefix already shifted the same-cost virtual boundary');
  assert.deepEqual(actual.pruned,expected.pruned);
  assert.ok(actual.reads*3<expected.reads,JSON.stringify({previous:expected.reads,current:actual.reads}));
});
