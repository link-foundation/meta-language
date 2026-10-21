import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {acceptDeclarationSignatureBinding,acceptDeclarationModuleScope} from '../src/translation/declaration-dependency-rules.js';
import {readOrdinaryDeclarationModuleFixtures,ordinaryModuleOwnerSource} from '../scripts/declaration-dependency-fixtures.mjs';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

function versions(){
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node version '+process.versions.node);
}
async function observe(fixture){
  if(fixture.sourceMode==='Script')return runInNewContext(fixture.source+'\n({first,second,independent});',{}, {timeout:1000});
  // This loads the exact complete original as an ES module. It does not strip
  // exports, turn an invalid module into a Script, or rewrite its bindings.
  return import('data:text/javascript;base64,'+Buffer.from(fixture.source).toString('base64'));
}

test('ordinary signature admission uses the actual certified scope and unique owner count',()=>{
  versions();
  assert.equal(acceptDeclarationSignatureBinding(true,1),true);
  for(const count of [0,-1,2,1.5,NaN,Infinity])assert.equal(acceptDeclarationSignatureBinding(true,count),false);
  for(const count of [0,1,2])assert.equal(acceptDeclarationSignatureBinding(false,count),false);
  assert.equal(acceptDeclarationModuleScope(['empty class declaration'],false),true);
});

test('the exact original acyclic var overwrite remains an independent negative control',()=>{
  versions();
  assert.equal(ordinaryModuleOwnerSource,'function first(value) { return value; }\nvar first = 0;\nfunction second(value) { return first(value); }\n');
  assert.equal(runInNewContext(ordinaryModuleOwnerSource+'\nfirst;'),0);
  assert.throws(()=>runInNewContext(ordinaryModuleOwnerSource+'\nsecond(0);'),error=>error.name==='TypeError');
  const actual=selfTranslate(ordinaryModuleOwnerSource,'JavaScript','Rust');
  assert.doesNotMatch(actual.code,/pub fn second\(/u);
  assert.deepEqual(selfTranslationSignatures(ordinaryModuleOwnerSource,'JavaScript'),[]);
  assert.equal(selfTranslate(actual.code,'Rust','JavaScript').code,ordinaryModuleOwnerSource);
});

test('acyclic overwritten owners cannot supply ordinary or public signatures and original safe modules stay supported',async()=>{
  versions();
  for(const fixture of readOrdinaryDeclarationModuleFixtures()){
    const raw=await observe(fixture);
    if(Object.hasOwn(fixture,'sourceOwnerValue')){
      assert.equal(raw.first,fixture.sourceOwnerValue,fixture.id);
      assert.throws(()=>raw.second(0),error=>error.name===fixture.sourceOwnerCallError,fixture.id);
    }
    const lines=fixture.calls.map(call=>{
      if(call==='limit')return String(raw.limit);
      const match=/^([A-Za-z_][A-Za-z_0-9]*)\((\d*)\)$/u.exec(call);assert.ok(match);
      return String(match[2]?raw[match[1]](Number(match[2])):raw[match[1]]());
    }).join('\n')+'\n';
    assert.equal(lines,fixture.expectedLines,fixture.id);
    const actual=selfTranslate(fixture.source,fixture.language,'Rust');
    assert.deepEqual(actual.items.filter(item=>['function_declaration','export_statement'].includes(item.term)).map(item=>item.status),fixture.functionStatuses,fixture.id);
    assert.deepEqual(selfTranslationSignatures(fixture.source,fixture.language).map(signature=>signature.name),fixture.signatures,fixture.id);
    assert.equal(selfTranslate(actual.code,'Rust',fixture.language).code,fixture.source,fixture.id);
    if(Object.hasOwn(fixture,'sourceOwnerValue')){
      assert.match(actual.code,/pub fn first\(/u,'the original isolated body proof remains');
      assert.doesNotMatch(actual.code,/pub fn second\(/u,'no stale ordinary dependency signature reaches the caller');
      assert.match(actual.code,/pub fn independent\(/u,'independent body proof remains');
    }
    const concrete=LinkNetwork.parse(fixture.source,fixture.language);
    for(const restored of [LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(restored.reconstructText(),fixture.source);
  }
});

test('duplicate named owners and nonempty class evaluation never acquire the cheap empty-class scope proof',async()=>{
  versions();
  const duplicate='function first(value){return value+1;}\nfunction first(value){return value+2;}\nfunction second(value){return first(value);}\n';
  assert.equal(runInNewContext(duplicate+'\nsecond(0);'),2,'the unchanged duplicate Script uses its actual last function owner');
  const actual=selfTranslate(duplicate,'JavaScript','Rust');
  assert.doesNotMatch(actual.code,/pub fn second\(/u);
  assert.deepEqual(selfTranslationSignatures(duplicate,'JavaScript'),[]);
  assert.equal(selfTranslate(actual.code,'Rust','JavaScript').code,duplicate);
  for(const middle of ['export class Unsupported extends (first = 0, Object) {}\n',
    'export class Unsupported { static { first = 0; } }\n']){
    const source='export function first(value){return value+1;}\n'+middle+'export function second(value){return first(value);}\n';
    const raw=await import('data:text/javascript;base64,'+Buffer.from(source).toString('base64'));
    assert.equal(raw.first,0);assert.throws(()=>raw.second(0),error=>error.name==='TypeError');
    const translated=selfTranslate(source,'JavaScript','Rust');
    assert.doesNotMatch(translated.code,/pub fn second\(/u);
    assert.deepEqual(selfTranslationSignatures(source,'JavaScript'),[]);
    assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
  }
});

test('the exact retained report signature oracle still avoids constructing another source graph',()=>{
  versions();
  const fixture=readOrdinaryDeclarationModuleFixtures().find(fixture=>fixture.id==='retained-count-limit-and-empty-class-oracle');
  const original=LinkNetwork.parse;
  let actual;
  try{LinkNetwork.parse=()=>{throw new Error('signature discovery constructed a source graph');};actual=selfTranslationSignatures(fixture.source,'JavaScript');}
  finally{LinkNetwork.parse=original;}
  assert.deepEqual(actual,[{k:'fn',name:'count',params:[],ret:{kind:'float'}},{k:'const',name:'limit',type:{kind:'float'},literal:true}]);
});
