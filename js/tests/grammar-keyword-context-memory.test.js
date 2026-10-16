import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {Executor,KeywordLexing} from '../src/grammar-runtime/executor.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';

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
