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
// The complete predecessor corpus is retained from immutable commit 858665058648a3f1f9ce05ff3b0668b9ccfd66eb.
const historicalFixture=readProjectData(new URL('../../parity/fixtures/historical/frontend-const-lists/javascript-scoped-binding-emission.lino',import.meta.url));
const fixtureVariants=[fixture,historicalFixture];
const options={javaScriptScopedBindingEmissionModel:fixture.model};
const scalar=value=>value==='-0'?-0:value;
const reload=value=>({network:LinkNetwork.fromLino(value.links),root:value.root});
const field=(network,parent,name)=>network.fieldRelations().find(value=>value.parent.equals(parent)&&value.label===(TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name));

test('current scoped binding fixture matches complete automatic generation',()=>{
  assert.deepEqual(javaScriptScopedBindingEmissionFixture(),fixture);
});

test('five independently specified complete modules preserve calls, prints, aliases and shadowing over four source-free replay cycles',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
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
        for(const observations of [specification,...(specification.additionalObservations??[])])for(let index=0;index<observations.inputs.length;index+=1) {
          context.input=observations.inputs[index].map(scalar);
          assert.ok(Object.is(vm.runInContext(`${observations.entry}(...input)`,context,{timeout:1000}),scalar(observations.expected[index])),`${specification.id}:${cycle}:${index}:${observations.entry}`);
        }
        assert.ok(program.declarations.has(specification.entry));
        if(specification.id==='declaration-loop')assert.ok(emitted.code.includes('const ml_scalar_write0 ='));
        if(specification.id==='scalar-snapshot-declaration-collision'){
          assert.ok(emitted.code.includes('function ml_scalar_write0('));
          assert.ok(emitted.code.includes('const ml_scalar_write0 ='));
          assert.ok(!emitted.code.includes('const ml_scalar_write0_2 ='));
        }
        const next=createTypedProgramLinks(emitted.code,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel});
        archive={network:LinkNetwork.fromLino(next.network.toLino()),root:next.root};
      }
    }
  }
});

test('unchanged default complete replay retains its exact reserved-name refusal; unknown models and wrong targets refuse',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
    const original=reload(fixture.cases[0]);
    const first=emitTypedProgramLinks(original.network,original.root,'JavaScript').code;
    const second=createTypedProgramLinks(first,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel});
    const repeated=emitTypedProgramLinks(second.network,second.root,'JavaScript').code;
    assert.throws(()=>createTypedProgramLinks(repeated,'JavaScript',{javaScriptScalarLoopModel:fixture.loopModel}),error=>error.reason==="reserved identifier: ml_main_ignored1 uses the translator's reserved ml_ prefix"&&error.span.start===390&&error.span.end===406);
    assert.throws(()=>emitTypedProgramLinks(original.network,original.root,'Rust',options),/requires JavaScript target/u);
    assert.throws(()=>emitTypedProgramLinks(original.network,original.root,'JavaScript',{javaScriptScopedBindingEmissionModel:'unknown'}),/JavaScript scoped binding emission model/u);
  }
});

test('actual source-free scalar edits survive complete edited-module and next-module replay',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
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
  }
});

test('foreign declaration objects and counterfeit scalar-loop owners cannot select the preservation role',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
    const archive=reload(fixture.cases[0]);const program=decodeTypedProgramLinks(archive.network,archive.root);
    const alias=readJavaScriptScopedBindingEmission(program,fixture.model);const declaration=program.declarations.get('run');
    assert.equal(alias(declaration.params[0].name,declaration),declaration.params[0].name);
    assert.throws(()=>alias('value',{...declaration}),/actual checked declaration/u);
    for(const change of [value=>{value.sourceLoop.sourceName='counterfeit';},value=>{value.sourceLoop.state[0].name='foreign';},value=>{value.sourceLoop.continuation.path=['other'];}]) {
      const current=decodeTypedProgramLinks(archive.network,archive.root);change(current.declarations.get('run'));
      assert.throws(()=>emitJavaScript(current,{scopedBindingModel:fixture.model}),/loop|continuation|ownership/u);
    }
  }
});

test('authentic JavaScript replay archives retain explicit snapshot context and exact empty-value facts',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
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
  }
});

test('unsupported declaration scopes, values, effects and unowned context premises refuse',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
    for(const source of ['const scope={keep(x){return x;}};scope.keep(2);','function keep(x){return [x];}keep(2);','import assert from "node:assert/strict";function keep(x){return x;}assert.equal(keep(2),2);']) {
      const archive=createTypedProgramLinks(source,'JavaScript');
      assert.throws(()=>emitTypedProgramLinks(archive.network,archive.root,'JavaScript',options),/JavaScript scoped binding emission/u);
    }
    const archive=reload(fixture.cases[2]);
    for(const value of [true,null,0,'',undefined]) {
      const program=decodeTypedProgramLinks(archive.network,archive.root);program.referenceSemantics=value;
      assert.throws(()=>emitJavaScript(program,{scopedBindingModel:fixture.model}),/owned local interpretation/u);
    }
  }
});

test('the observed undefined optional type retains its tag and refuses other elements, tags and some nodes',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
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
  }
});

test('every admitted record role refuses unrepresented ownership facts after actual graph reload',()=>{
  const base=createTypedProgramLinks('function keep(x){return x+1;}keep(2);','JavaScript');
  const roles={program:program=>program,main:program=>program.main,declaration:program=>program.declarations.get('keep'),parameter:program=>program.declarations.get('keep').params[0],effect:program=>program.main.effects[0],expression:program=>program.declarations.get('keep').body};
  for(const [role,owner]of Object.entries(roles))for(const field of ['borrowEscape','opaqueBinding','sourceEmptyOption']){
    const program=decodeTypedProgramLinks(base.network,base.root);
    owner(program)[field]=field==='opaqueBinding'?(role==='parameter'?program.declarations.get('keep').body:program.declarations.get('keep').params[0]):true;
    const attempt=()=>{const encoded=encodeTypedProgramLinks(program);const network=LinkNetwork.fromLino(encoded.network.toLino());return emitTypedProgramLinks(network,encoded.root,'JavaScript',options);};
    if(field==='sourceEmptyOption')assert.throws(attempt,undefined,role+':'+field);
    else assert.throws(attempt,/unrepresented own fields/u,role+':'+field);
  }
});

test('known source facet names still require their actual independently checked owner records',()=>{
  for (const fixture of fixtureVariants) {
    const options={javaScriptScopedBindingEmissionModel:fixture.model};
    const base=createTypedProgramLinks('function keep(x){return x+1;}keep(2);','JavaScript');
    for(const name of ['sourceSignature','sourceLoop','sourcePrimitiveOwners'])for(const value of [undefined,null,{},true]){
      const program=decodeTypedProgramLinks(base.network,base.root);
      program.declarations.get('keep')[name]=value;
      assert.throws(()=>emitJavaScript(program,{scopedBindingModel:fixture.model}),/signature|loop|owner|source profile|primitive|source facet|explicitly undefined field/u,name);
    }
  }
});

test('actual source signatures and qualified primitive owners compose with strict selected scoped emission',()=>{
  const base=createTypedProgramLinks('fn apply(value:&std::primitive::f64)->std::primitive::f64{*value}fn main(){}','Rust',{rustPrimitiveOwnerModel:'standard primitive Rust binary64 owner 1',rustSourceSignatureModel:'Rust source function signature 1'});
  const network=LinkNetwork.fromLino(base.network.toLino());const before=network.toLino();
  const program=decodeTypedProgramLinks(network,base.root);const declaration=program.declarations.get('apply');
  assert.ok(declaration.sourceSignature);assert.ok(declaration.sourcePrimitiveOwners);
  const code=emitTypedProgramLinks(network,base.root,'JavaScript',options).code;
  for(const input of [-0,NaN,Infinity,1.75])assert.ok(Object.is(vm.runInNewContext(`${code}\napply(input)`,{input},{timeout:1000}),input));
  assert.equal(network.toLino(),before);
});

test('present empty-context sequences and represented Boolean slots retain exact scalar shapes',()=>{
  const base=createTypedProgramLinks('function keep(x){return x+1;}keep(2);','JavaScript');
  for(const field of ['imports','externals'])for(const value of [null,{},false,'',0,undefined]){
    const program=decodeTypedProgramLinks(base.network,base.root);program[field]=value;
    const attempt=()=>{const encoded=encodeTypedProgramLinks(program);const network=LinkNetwork.fromLino(encoded.network.toLino());return emitTypedProgramLinks(network,encoded.root,'JavaScript',options);};
    assert.throws(attempt,/actual empty import and external sequences|explicitly undefined field|not iterable/u,field);
  }
  for(const [role,field]of [['main','sequentialAsync'],['function','recursive'],['function','mutual'],['effect','constant']])for(const value of [null,0,'',{},undefined]){
    const program=decodeTypedProgramLinks(base.network,base.root);const owner=role==='main'?program.main:role==='function'?program.declarations.get('keep'):program.main.effects[0];owner[field]=value;
    const encoded=encodeTypedProgramLinks(program);const network=LinkNetwork.fromLino(encoded.network.toLino());
    assert.throws(()=>emitTypedProgramLinks(network,encoded.root,'JavaScript',options),/exact Boolean fields|explicitly undefined field/u,role+':'+field);
  }
});

test('explicit undefined guard, default and source-span facts differ from absent or null fields',()=>{
  const base=createTypedProgramLinks('function keep(x){return x+1;}keep(2);','JavaScript');
  for(const field of ['guard','default','span']){
    const program=decodeTypedProgramLinks(base.network,base.root);const owner=field==='span'?program.declarations.get('keep').body:program.declarations.get('keep').params[0];owner[field]=undefined;
    const encoded=encodeTypedProgramLinks(program);const network=LinkNetwork.fromLino(encoded.network.toLino());
    assert.throws(()=>emitTypedProgramLinks(network,encoded.root,'JavaScript',options),/explicitly undefined field/u,field);
  }
});

test('whole scoped projection refuses stale recursive, mutual and decreasing facts',()=>{
  const base=createTypedProgramLinks('function keep(x){return x+1;}keep(2);','JavaScript');
  for(const [field,value]of [['recursive',true],['mutual',true],['decreasing',0]]){
    const program=decodeTypedProgramLinks(base.network,base.root);program.declarations.get('keep')[field]=value;
    const encoded=encodeTypedProgramLinks(program);const network=LinkNetwork.fromLino(encoded.network.toLino());
    assert.throws(()=>emitTypedProgramLinks(network,encoded.root,'JavaScript',options),/inconsistent types, bindings or operation semantics after checked surface projection/u,field);
  }
});
