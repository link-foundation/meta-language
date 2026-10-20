import assert from 'node:assert/strict';
import {stripTypeScriptTypes} from 'node:module';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateTypeScriptReceiverFixtures} from '../scripts/generate-typescript-receiver-fixtures.mjs';
import {generateTypeScriptReceiverParameters} from '../scripts/generate-typescript-receiver-parameters.mjs';
import {eraseTypeScript,requiresTypeScriptErasure} from '../src/typescript-erasure.js';
import {projectTypeScriptTyping} from '../src/typescript-portable-typing.js';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';
import {selfTranslate} from '../src/self-translation.js';
import {tokenize} from '../src/translation/lexer.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {acceptTypeScriptReceiverParameter,eraseTypeScriptReceiverSeparator} from '../src/typescript-erasure-rules.js';

const fixture=readProjectData(new URL('../../parity/fixtures/typescript-receiver-parameters.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,
  'Uncovered Node type-stripping oracle '+process.versions.node+'; covered: '+fixture.oracleRanges.join(' or '));
const tokens=source=>tokenize(source,'JavaScript').tokens.map(({kind,value,raw})=>({kind,value,raw}));
const evaluate=(code,invocation)=>JSON.parse(vm.runInNewContext(code+'\nJSON.stringify('+invocation+');',{}, {timeout:1000}));

test('receiver erasure agrees with the bounded independent Node oracle on every actual result and runtime arity',()=>{
  generateTypeScriptReceiverParameters({check:true});generateTypeScriptReceiverFixtures({check:true});
  for(const row of fixture.cases){
    const actual=eraseTypeScript(row.source),oracle=stripTypeScriptTypes(row.source,{mode:'strip'});
    assert.deepEqual(actual,row.erased,row.id);
    assert.deepEqual(tokens(actual.code),tokens(oracle),row.id);
    assert.deepEqual(evaluate(actual.code,row.invocation),row.expectedValue,row.id);
    assert.deepEqual(evaluate(oracle,row.invocation),row.expectedValue,row.id);
    assert.equal(actual.code.match(/\r\n|\r|\n/gu)?.join(''),row.source.match(/\r\n|\r|\n/gu)?.join(''),row.id);
    const receiver=actual.erasures.filter(erasure=>erasure.term==='this_parameter');
    assert.equal(receiver.length,row.id==='runtime-this-spelling'?0:1,row.id);
    for(const erasure of receiver)assert.match(Buffer.from(row.source).subarray(erasure.start,erasure.end).toString(),/^this\s*:/u);
  }
});

test('complete source concrete trees and TypeScript translation provenance survive text binary and reverse translation',()=>{
  for(const row of fixture.cases){
    const source=LinkNetwork.parse(row.source,'TypeScript');
    assert.equal(source.verifyFullMatch().isClean(),true,row.id);
    for(const network of [LinkNetwork.fromLino(source.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(source))]){
      assert.equal(network.reconstructText(),row.source,row.id);
      assert.equal(network.toLino(),source.toLino(),row.id);
    }
    const translated=selfTranslate(row.source,'TypeScript','JavaScript');
    assert.equal(translated.code,row.javascript,row.id);
    assert.ok(translated.items.every(item=>item.status==='translated'),row.id);
    assert.deepEqual(evaluate(translated.code,row.invocation),row.expectedValue,row.id);
    assert.equal(selfTranslate(translated.code,'JavaScript','TypeScript').code,row.source,row.id);
    if(row.portable){
      const rust=selfTranslate(row.source,'TypeScript','Rust');
      assert.equal(rust.code,row.rust,row.id);
      assert.ok(rust.items.every(item=>item.status==='translated'||item.status==='comment'),row.id);
      assert.equal(selfTranslate(rust.code,'Rust','TypeScript').code,row.source,row.id);
    }
  }
});

test('portable annotations preserve the void receiver contract without adding a runtime argument or erasing an unsupported receiver type',()=>{
  for(const row of fixture.cases){
    if(row.typingRefusal){
      assert.throws(()=>projectTypeScriptTyping(row.source),error=>error.reason===row.typingRefusal.reason,row.id);
    }else assert.equal(projectTypeScriptTyping(row.source).code,row.typing,row.id);
    if(row.receiverTypeRefusal){
      const result=selfTranslate(row.source,'TypeScript','Rust');
      assert.ok(result.items.some(item=>item.status==='carried'&&item.reason.includes('explicit portable receiver interpretation')));
    }
    if(row.portableRefusal){
      const result=selfTranslate(row.source,'TypeScript','Rust');
      assert.equal(result.code,row.rust,row.id);
      assert.ok(result.items.some(item=>item.status==='carried'&&item.reason===row.portableRefusal),row.id);
      assert.equal(selfTranslate(result.code,'Rust','TypeScript').code,row.source,row.id);
    }
    if(row.portable&&row.id!=='runtime-this-spelling'){
      assert.match(row.typing,/@this \{void\}/u,row.id);assert.doesNotMatch(row.typing,/@param \{[^}]*\} this(?:\s|$)/u,row.id);
    }
  }
  const ordinary='function read(thisValue) { return thisValue; }\n';
  assert.equal(requiresTypeScriptErasure(parseProgrammingLanguage(ordinary,'TypeScript')),false);
  assert.deepEqual(eraseTypeScript(ordinary),{code:ordinary,erasures:[]});
  const source=fixture.cases[0].source;
  const translated=selfTranslate(source,'TypeScript','JavaScript');
  const at=translated.code.lastIndexOf('return a + b;');assert.ok(at>=0);
  const changed=translated.code.slice(0,at)+translated.code.slice(at).replace('return a + b;','return a - b;');
  const restored=selfTranslate(changed,'JavaScript','TypeScript');
  assert.notEqual(restored.code,source);assert.deepEqual(evaluate(restored.code,'[add(2,3),add.length]'),[-1,2]);
});

test('every invalid or unsupported receiver remains explicit and all generated Boolean admission decisions retain their complete truth tables',()=>{
  for(const row of fixture.refusals){
    assert.throws(()=>eraseTypeScript(row.source),error=>error.reason===row.error.reason
      &&assert.deepEqual(error.span,row.error.span)===undefined,row.source);
    const translated=selfTranslate(row.source,'TypeScript','JavaScript');
    assert.ok(translated.items.some(item=>item.status==='carried'),row.source);
    assert.equal(selfTranslate(translated.code,'JavaScript','TypeScript').code,row.source,row.source);
  }
  for(let mask=0;mask<32;mask+=1){
    const values=Array.from({length:5},(_,index)=>(mask&(1<<index))!==0);
    assert.equal(acceptTypeScriptReceiverParameter(...values),mask===23,mask.toString());
  }
  for(const comma of [false,true])for(const pending of [false,true]){
    assert.equal(eraseTypeScriptReceiverSeparator(comma,pending),comma&&pending);
  }
});
