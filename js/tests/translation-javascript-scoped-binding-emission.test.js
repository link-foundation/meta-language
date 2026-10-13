import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {javaScriptScopedBindingEmissionFixture} from '../scripts/generate-javascript-scoped-binding-emission-fixtures.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {readJavaScriptScopedBindingEmission} from '../src/translation/javascript-scoped-binding-emission.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-scoped-binding-emission.lino',import.meta.url));
const options={javaScriptScopedBindingEmissionModel:fixture.model};
const scalar=value=>value==='-0'?-0:value;
const reload=value=>({network:LinkNetwork.fromLino(value.links),root:value.root});
const field=(network,parent,name)=>network.fieldRelations().find(value=>value.parent.equals(parent)&&value.label===(TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name));

test('four independently specified complete modules preserve calls, prints, aliases and shadowing over four source-free replay cycles',()=>{
  assert.deepEqual(javaScriptScopedBindingEmissionFixture(),fixture);
  for(const specification of fixture.cases) {
    let archive=reload(specification);
    for(let cycle=0;cycle<4;cycle+=1) {
      const before=archive.network.toLino();
      const program=decodeTypedProgramLinks(archive.network,archive.root);
      const emitted=emitTypedProgramLinks(archive.network,archive.root,'JavaScript',options);
      assert.equal(archive.network.toLino(),before);
      assert.equal(emitted.code,emitJavaScript(program,{scopedBindingModel:fixture.model}).text);
      const output=[];const context=vm.createContext({console:{log:value=>output.push(String(value))}});
      vm.runInContext(emitted.code,context,{timeout:1000});
      assert.deepEqual(output,specification.output,`${specification.id}:${cycle}`);
      for(let index=0;index<specification.inputs.length;index+=1) {
        context.input=specification.inputs[index].map(scalar);
        assert.ok(Object.is(vm.runInContext(`${specification.entry}(...input)`,context,{timeout:1000}),scalar(specification.expected[index])),`${specification.id}:${cycle}:${index}`);
      }
      assert.ok(program.declarations.has(specification.entry));
      if(specification.id==='declaration-loop')assert.ok(emitted.code.includes('const ml_scalar_write0 ='));
      const next=createTypedProgramLinks(emitted.code,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel});
      archive={network:LinkNetwork.fromLino(next.network.toLino()),root:next.root};
    }
  }
});

test('unchanged default complete replay retains its exact reserved-name refusal; unknown models and wrong targets refuse',()=>{
  const original=reload(fixture.cases[0]);
  const first=emitTypedProgramLinks(original.network,original.root,'JavaScript').code;
  const second=createTypedProgramLinks(first,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel});
  const repeated=emitTypedProgramLinks(second.network,second.root,'JavaScript').code;
  assert.throws(()=>createTypedProgramLinks(repeated,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel}),error=>error.reason==="reserved identifier: ml_main_ignored1 uses the translator's reserved ml_ prefix"&&error.span.start===390&&error.span.end===406);
  assert.throws(()=>emitTypedProgramLinks(original.network,original.root,'Rust',options),/requires JavaScript target/u);
  assert.throws(()=>emitTypedProgramLinks(original.network,original.root,'JavaScript',{javaScriptScopedBindingEmissionModel:'unknown'}),/JavaScript scoped binding emission model/u);
});

test('actual source-free scalar edits survive complete edited-module and next-module replay',()=>{
  const archive=reload(fixture.cases[0]);
  const one=archive.network.links().find(link=>link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.scalars.string&&link.metadata().definition==='1');
  assert.ok(one);one.setMetadata(one.metadata().withDefinition('2'));
  let edited={network:LinkNetwork.fromLino(archive.network.toLino()),root:archive.root};
  for(let cycle=0;cycle<3;cycle+=1) {
    const code=emitTypedProgramLinks(edited.network,edited.root,'JavaScript',options).code;
    assert.equal(vm.runInNewContext(`${code}\nrun(0,3)`,{},{timeout:1000}),4);
    const next=createTypedProgramLinks(code,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel});
    edited={network:LinkNetwork.fromLino(next.network.toLino()),root:next.root};
  }
});

test('foreign declaration objects and counterfeit scalar-loop owners cannot select the preservation role',()=>{
  const archive=reload(fixture.cases[0]);const program=decodeTypedProgramLinks(archive.network,archive.root);
  const alias=readJavaScriptScopedBindingEmission(program,fixture.model);const declaration=program.declarations.get('run');
  assert.equal(alias(declaration.params[0].name,declaration),declaration.params[0].name);
  assert.throws(()=>alias('value',{...declaration}),/actual checked declaration/u);
  for(const change of [value=>{value.sourceLoop.sourceName='counterfeit';},value=>{value.sourceLoop.state[0].name='foreign';},value=>{value.sourceLoop.continuation.path=['other'];}]) {
    const current=decodeTypedProgramLinks(archive.network,archive.root);change(current.declarations.get('run'));
    assert.throws(()=>emitJavaScript(current,{scopedBindingModel:fixture.model}),/loop|continuation|ownership/u);
  }
});

test('authentic JavaScript replay archives retain explicit snapshot context and exact empty-value facts',()=>{
  assert.equal(fixture.replayArchives.length,3);
  for(const value of fixture.replayArchives){
    const archive=reload(value);const program=decodeTypedProgramLinks(archive.network,archive.root);
    assert.equal(program.referenceSemantics,true);
    const code=emitTypedProgramLinks(archive.network,archive.root,'JavaScript',options).code;
    assert.equal(vm.runInNewContext(`${code}\nrun(0,3)`,{},{timeout:1000}),3);
    for(const flag of [null,0,'',undefined]){
      const current=decodeTypedProgramLinks(archive.network,archive.root);current.referenceSemantics=flag;
      assert.throws(()=>emitJavaScript(current,{scopedBindingModel:fixture.model}),/owned local interpretation/u);
    }
  }
});

test('unsupported declaration scopes, values, effects and unowned context premises refuse',()=>{
  for(const source of ['const scope={keep(x){return x;}};scope.keep(2);','function keep(x){return [x];}keep(2);','import assert from "node:assert/strict";function keep(x){return x;}assert.equal(keep(2),2);']) {
    const archive=createTypedProgramLinks(source,'JavaScript');
    assert.throws(()=>emitTypedProgramLinks(archive.network,archive.root,'JavaScript',options),/JavaScript scoped binding emission/u);
  }
  const archive=reload(fixture.cases[2]);
  for(const value of [true,null,0,'',undefined]) {
    const program=decodeTypedProgramLinks(archive.network,archive.root);program.referenceSemantics=value;
    assert.throws(()=>emitJavaScript(program,{scopedBindingModel:fixture.model}),/owned local interpretation/u);
  }
});

test('the observed undefined optional type retains its tag and refuses other elements, tags and some nodes',()=>{
  const original=reload(fixture.cases[0]);
  let archive=original;
  for(let index=0;index<2;index+=1)archive=createTypedProgramLinks(emitTypedProgramLinks(archive.network,archive.root,'JavaScript',options).code,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel});
  const get=()=>decodeTypedProgramLinks(archive.network,archive.root);
  const base=get();const empty=[...base.declarations.values()].find(value=>value.body.k==='none');
  assert.deepEqual(empty.ret,{kind:'option',element:{kind:'float'},absence:'undefined'});
  for(const [element,absence] of [[{kind:'bool'},'undefined'],[{kind:'float'},'null']]) {
    const program=get();const value=program.declarations.get(empty.fullName);value.ret={kind:'option',element,absence};value.body.type=value.ret;value.body.absence=absence;
    assert.throws(()=>emitJavaScript(program,{scopedBindingModel:fixture.model}),/optional value type|type|function call/u);
  }
  const program=get();const value=program.declarations.get(empty.fullName);
  value.body={k:'some',value:{k:'lit',value:'1',type:{kind:'float'}},type:value.ret};
  assert.throws(()=>emitJavaScript(program,{scopedBindingModel:fixture.model}),/ownership role/u);
});
