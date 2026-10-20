import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {freezeOriginalLinkRawData,readOriginalLinkRawFreezeWork} from '../../experiments/original-link-complete-table-projection/raw-data-freeze.mjs';
import {generateOriginalLinkRawPlanningHistory} from '../../experiments/original-link-complete-table-projection/generate-raw-planning-source-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const history=generateOriginalLinkRawPlanningHistory({check:true});
const source=history.sources.find(row=>row.file.endsWith('/raw-native-observations.mjs'));
verifyGitArtifactCertificate(source.certificate,Buffer.from(source.sourceText),{revision:history.revision,path:source.file});
const start=source.sourceText.indexOf('function freeze(value) {'),end=source.sourceText.indexOf('\n}',start)+2;
assert.ok(start>=0&&end>start);assert.equal(source.sourceText.indexOf('function freeze(value) {',start+1),-1);
const originalFreeze=vm.runInThisContext('('+source.sourceText.slice(start,end)+')');

test('raw planning freezes each unique plain object once while retaining every shared identity',()=>{
  const leaf={number:1},row=[leaf,leaf],group={first:row,second:row};
  const root={group,again:group,rows:Array(32).fill(row)};
  Object.freeze(root); // A shallow external freeze cannot stand in for traversal.
  const before=readOriginalLinkRawFreezeWork();
  assert.strictEqual(freezeOriginalLinkRawData(root),root);
  const after=readOriginalLinkRawFreezeWork();
  assert.equal(after.objects-before.objects,5);assert.equal(after.reused-before.reused,35);
  for(const value of [root,group,row,leaf,root.rows])assert.equal(Object.isFrozen(value),true);
  assert.strictEqual(root.group,root.again);assert.strictEqual(root.group.first,root.rows[31]);
  assert.throws(()=>{leaf.number=2;},TypeError);assert.throws(()=>row.push(2),TypeError);
  freezeOriginalLinkRawData(root);
  const repeated=readOriginalLinkRawFreezeWork();
  assert.equal(repeated.objects,after.objects);assert.equal(repeated.reused,after.reused+1);
  freezeOriginalLinkRawData({root});
  assert.equal(readOriginalLinkRawFreezeWork().objects,repeated.objects+1);
});

function createObservableRawFreezeGraph() {
  const calls=[];let index=0;
  const getter={get value(){calls.push('getter');return{index:++index};}};
  const proxy=new Proxy({child:{number:1}},{
    ownKeys(target){calls.push('own keys');return Reflect.ownKeys(target);},
    getOwnPropertyDescriptor(target,key){calls.push('descriptor '+String(key));return Reflect.getOwnPropertyDescriptor(target,key);},
    get(target,key,receiver){calls.push('get '+String(key));return Reflect.get(target,key,receiver);},
    preventExtensions(target){calls.push('prevent extensions');return Reflect.preventExtensions(target);},
    defineProperty(target,key,descriptor){calls.push('define '+String(key));return Reflect.defineProperty(target,key,descriptor);},
  });
  return{root:{first:getter,second:getter,proxy,again:proxy},calls};
}

test('accessor effects and proxy trap ordering match the entire authenticated predecessor',()=>{
  const original=createObservableRawFreezeGraph(),current=createObservableRawFreezeGraph();
  for(let pass=0;pass<2;pass++){
    assert.strictEqual(originalFreeze(original.root),original.root);
    assert.strictEqual(freezeOriginalLinkRawData(current.root),current.root);
    assert.deepEqual(current.calls,original.calls);
  }
  assert.equal(current.calls.filter(value=>value==='getter').length,4);
});

test('enumerable descendants holes and ignored source properties preserve the original freeze boundary',()=>{
  for(const freeze of [originalFreeze,freezeOriginalLinkRawData]){
    const symbol=Symbol('hidden'),hidden={},symbolValue={},rows=new Array(3);rows[2]={number:1};
    const root={rows,[symbol]:symbolValue};Object.defineProperty(root,'hidden',{value:hidden,enumerable:false});
    assert.strictEqual(freeze(root),root);assert.equal(Object.isFrozen(rows[2]),true);
    assert.equal(Object.hasOwn(rows,0),false);assert.equal(rows.length,3);
    assert.equal(Object.isFrozen(hidden),false);assert.equal(Object.isFrozen(symbolValue),false);
    assert.equal(Object.isFrozen(root),true);assert.equal(Object.isFrozen(rows),true);
  }
});

test('primitive function and typed-array outcomes retain the original source behavior',()=>{
  for(const freeze of [originalFreeze,freezeOriginalLinkRawData]){
    for(const value of [null,undefined,0,-0,NaN,false,'text'])assert.ok(Object.is(freeze(value),value));
    const callback=()=>1;assert.strictEqual(freeze(callback),callback);assert.equal(Object.isFrozen(callback),false);
    assert.throws(()=>freeze(new Uint8Array([1,2])),TypeError);
    const empty=new Uint8Array();assert.strictEqual(freeze(empty),empty);assert.equal(Object.isFrozen(empty),true);
  }
});
