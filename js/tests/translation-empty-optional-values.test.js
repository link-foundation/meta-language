import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {checkProgram} from '../src/translation/check.js';
import {projectLinkedProgramExpression} from '../src/typed-program-expression-projection.js';
import {readProgramExpressionProjection} from '../src/typed-program-link-rules.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';

const source='function empty(){return undefined;}';
const expectedType={kind:'option',element:{kind:'float'},absence:'undefined'};
const model='owned JavaScript scoped bindings 1';
test('the exact undefined optional value retains its tag, Number element and absence through shared projection',()=>{
 const packet=createTypedProgramLinks(source,'JavaScript'),program=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root),entry=program.declarations.get('empty');
 assert.deepEqual(entry.ret,expectedType);assert.deepEqual(entry.body,{k:'none',absence:'undefined',type:expectedType,span:{start:24,end:33}});
 assert.equal(readProgramExpressionProjection('none'),'none');
 const body=projectLinkedProgramExpression(entry.body,program.declarations);
 assert.deepEqual(body,{k:'none',absence:'undefined',span:entry.body.span});
 const checked=checkProgram({language:'JavaScript',items:[{k:'fn',name:'empty',params:[],ret:expectedType,body}],main:null});
 assert.deepEqual(checked.declarations.get('empty').body,entry.body);
 const emitted=encodeTypedProgramLinks(checked),reload=decodeTypedProgramLinks(LinkNetwork.fromLino(emitted.network.toLino()),emitted.root);
 assert.deepEqual(reload.declarations.get('empty').body,entry.body);
 assert.equal(vm.runInNewContext(emitJavaScript(reload).text+'\nempty();',{}, {timeout:1000}),undefined);
 const renderers=[[emitRust,'None::<f64>'],[emitLean,'none'],[emitRocq,'(@None float)']];
 for(const [emit,literal] of renderers)assert.ok(emit(reload).text.includes(literal));
});
test('four complete emitted-module cycles retain the same undefined optional shape under the frozen scope interpretation',()=>{
 let packet=createTypedProgramLinks(source,'JavaScript');
 for(let cycle=0;cycle<4;cycle++){
  const network=LinkNetwork.fromLino(packet.network.toLino());
  const code=emitTypedProgramLinks(network,packet.root,'JavaScript',{javaScriptScopedBindingEmissionModel:model}).code;
  assert.equal(vm.runInNewContext(code+'\nempty();',{}, {timeout:1000}),undefined);
  packet=createTypedProgramLinks(code,'JavaScript');
  const program=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root);
  const start=code.indexOf('return undefined;')+'return '.length;
  assert.deepEqual(program.declarations.get('empty').body,{k:'none',absence:'undefined',type:expectedType,span:{start,end:start+'undefined'.length}});
 }
});
