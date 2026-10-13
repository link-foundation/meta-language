import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustTypeNamespaceFixture, rustTypeNamespaceField } from '../scripts/generate-rust-type-namespace-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createTypedProgramLinks, encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-type-namespace.lino', import.meta.url));
const options = {typeNamespaceModel:fixture.typeNamespaceModel};
const output = code => { const lines=[]; vm.runInNewContext(code,{console:{log:(...args)=>lines.push(args.map(String).join(' '))}},{timeout:1000});return lines; };
const field = rustTypeNamespaceField;

test('fourteen actual source headers distinguish datatype and value namespaces before primitive projection', () => {
  assert.deepEqual(rustTypeNamespaceFixture(), fixture);
  assert.equal(fixture.cases.length,14);
  for (const entry of fixture.cases) {
    const checked=checkProgram(parseRust(entry.source,options)), declaration=checked.declarations.get(entry.functionName);
    assert.deepEqual(declaration.params[0].type,entry.expected.parameterType,entry.id);
    assert.deepEqual(declaration.sourceSignature.parameters[0].owner.path,entry.expected.parameterOwner,entry.id);
    assert.deepEqual(declaration.sourceSignature.parameters[0].wrappers,entry.expected.wrappers,entry.id);
    assert.deepEqual(declaration.sourceSignature.parameters[0].lifetimes,entry.expected.lifetimes,entry.id);
    const network=LinkNetwork.fromLino(entry.archive), decoded=decodeTypedProgramLinks(network,entry.root);
    assert.deepEqual(decoded.declarations.get(entry.functionName).sourceTypeNamespace,declaration.sourceTypeNamespace,entry.id);
    if(entry.callFunction)assert.equal(decoded.declarations.get(entry.callFunction).body.fn,entry.callTarget,entry.id);
    assert.deepEqual(output(emitTypedProgramLinks(network,entry.root,'JavaScript').code),entry.output,entry.id);
    assert.ok(network.links().every(link=>link.metadata().definition!==entry.source),entry.id);
  }
});

test('every selected emitted Rust definition reimports with its actual projected datatype identities', () => {
  for (const entry of fixture.cases) {
    const emitted=emitRust(decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root));
    const target=name=>emitted.mappings.find(mapping=>mapping.source===name)?.target??name;
    const selected=createTypedProgramLinks(`${emitted.definitions.join('\n')}\nfn main(){}`,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel});
    const decoded=decodeTypedProgramLinks(LinkNetwork.fromLino(selected.network.toLino()),selected.root),declaration=decoded.declarations.get(target(entry.functionName));
    const type=source=>source.kind==='data'?{kind:'data',name:target(source.name)}:source;
    assert.deepEqual(declaration.params[0].type,type(entry.expected.parameterType),entry.id);
    assert.deepEqual(declaration.ret,type(entry.expected.resultType),entry.id);
    if(entry.callFunction) {
      assert.equal(decoded.declarations.get(target(entry.callFunction)).body.fn,target(entry.callTarget),entry.id);
      const js=emitTypedProgramLinks(selected.network,selected.root,'JavaScript');
      const caller=js.mappings.find(mapping=>mapping.source===target(entry.callFunction)).target;
      assert.equal(vm.runInNewContext(`${js.code}\n${caller}()`,{},{timeout:1000}),true,entry.id);
    }
  }
});

test('seventeen unresolved or unproved source bindings retain precise selected refusals', () => {
  assert.equal(fixture.invalid.length,17);
  for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,options)),error=>{
    assert.equal(error.kind,entry.kind,entry.id);assert.equal(error.reason,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;
  });
});

test('twenty-four actual malformed header Links graphs refuse both targets after source-free reload', () => {
  assert.equal(fixture.malformed.length,24);
  for(const entry of fixture.malformed)for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,target),undefined,entry.id);
  const parsed=checkProgram(parseRust(fixture.cases[0].source,options));
  for(const invalid of [null,0,'',false]) {
    const program=structuredClone(parsed);
    program.declarations.get('nested.header').sourceTypeNamespace=invalid;
    assert.throws(()=>encodeTypedProgramLinks(program),/malformed or missing selected facts/u);
  }
  for(const invalid of [null,0,'',true]) {
    const program=structuredClone(parsed);
    program.declarations.get('nested.header').generated=invalid;
    assert.throws(()=>encodeTypedProgramLinks(program),/absent or false generated/u);
  }
  const program=structuredClone(parsed);program.declarations.get('nested.header').generated=false;
  assert.doesNotThrow(()=>encodeTypedProgramLinks(program));
});

test('a consistent source spelling edit rebinds through its actual scoped datatype after Links reload', () => {
  const edited=fixture.spellingEdit, network=LinkNetwork.fromLino(edited.archive);
  const declaration=decodeTypedProgramLinks(network,edited.root).declarations.get(edited.functionName);
  assert.deepEqual(declaration.sourceTypeNamespace.parameters,[['crate','f64']]);
  assert.deepEqual(declaration.params[0].type,{kind:'data',name:'f64'});
  assert.deepEqual(declaration.sourceSignature.parameters[0].owner.path,['f64']);
  assert.deepEqual(output(emitTypedProgramLinks(network,edited.root,'JavaScript').code),edited.output);
});

test('an actual source-free value-body edit changes behavior without changing either namespace identity', () => {
  const entry=fixture.cases[0], network=LinkNetwork.fromLino(entry.archive);
  const functionId=network.fieldRelations().find(row=>row.label==='full name'&&network.link(row.child).metadata().definition==='nested.f64').parent;
  const value=field(network,field(network,functionId,'body'),'value');
  network.link(value).setMetadata(network.link(value).metadata().withDefinition('false'));
  const loaded=LinkNetwork.fromLino(network.toLino()),decoded=decodeTypedProgramLinks(loaded,entry.root);
  assert.equal(decoded.declarations.get('nested.value').body.fn,'nested.f64');
  assert.deepEqual(decoded.declarations.get('nested.header').params[0].type,{kind:'data',name:'f64'});
  assert.deepEqual(decoded.declarations.get('nested.header').sourceTypeNamespace.parameters,[['f64']]);
  assert.deepEqual(output(emitTypedProgramLinks(loaded,entry.root,'JavaScript').code),['false','true']);
  const definitions=emitRust(decoded),emitted=createTypedProgramLinks(`${definitions.definitions.join('\n')}\nfn main(){}`,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel});
  const target=definitions.mappings.find(mapping=>mapping.source==='nested.value').target;
  const js=emitTypedProgramLinks(emitted.network,emitted.root,'JavaScript'),caller=js.mappings.find(mapping=>mapping.source===target).target;
  assert.equal(vm.runInNewContext(`${js.code}\n${caller}()`,{},{timeout:1000}),false);
});

test('the namespace selector is independent and leaves legacy capture refusals and defaults unchanged', () => {
  const old=readProjectData(new URL('../../parity/fixtures/rust-source-signatures.lino',import.meta.url));
  for(const entry of old.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,{sourceSignatureModel:old.sourceSignatureModel})),error=>{
    assert.equal(error.kind,entry.kind);assert.equal(error.reason,entry.reason);assert.deepEqual(error.span??null,entry.span);return true;
  });
  const source='fn keep(value:f64)->f64{value}fn main(){}';
  const legacy=checkProgram(parseRust(source)),empty=checkProgram(parseRust(source,{typeNamespaceModel:''}));
  assert.deepEqual(empty,legacy);
  assert.equal(Object.hasOwn(legacy.declarations.get('keep'),'sourceTypeNamespace'),false);
  assert.equal(Object.hasOwn(legacy.declarations.get('keep'),'sourceSignature'),false);
  const selected=createTypedProgramLinks(source,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel,provenance:{project:'namespace'}});
  const loaded=LinkNetwork.fromLino(selected.network.toLino());
  const provenance=field(loaded,selected.root,'provenance'), profile=field(loaded,provenance,'source profile');
  assert.equal(loaded.link(field(loaded,profile,'rust type namespace model')).metadata().definition,fixture.typeNamespaceModel);
  assert.equal(loaded.link(field(loaded,field(loaded,provenance,'user provenance'),'project')).metadata().definition,'namespace');
  assert.throws(()=>parseRust(source,{typeNamespaceModel:'unknown'}),/Rust type namespace model/u);
  assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustTypeNamespaceModel:'unknown'}),/Rust type namespace model/u);
  assert.throws(()=>createTypedProgramLinks('function keep(value){return value;}','JavaScript',{rustTypeNamespaceModel:fixture.typeNamespaceModel}),/requires Rust source/u);
  assert.throws(()=>parseRust(source,{...options,sourceSignatureModel:'unknown'}),/Rust source signature model/u);
});

test('actual primitive namespace proofs compose with both strict header models and scoped emission', () => {
  const sources = [
    ['fn keep(value:&f64)->f64{*value}fn main(){}', 'primitive borrowed Rust function header 1'],
    ['fn keep(value:&f64)->f64{*value}fn call(value:f64)->f64{keep(&value)}fn main(){}', 'closed primitive borrowed Rust call interface 1'],
  ];
  for (const [source, model] of sources) {
    const packet = createTypedProgramLinks(source, 'Rust', {rustTypeNamespaceModel:fixture.typeNamespaceModel});
    const network = LinkNetwork.fromLino(packet.network.toLino());
    const program = decodeTypedProgramLinks(network, packet.root);
    const original = emitRust(program, {sourceHeaderModel:model});
    assert.match(original.definitions.join('\n'), /value: &f64/u);
    const linked = emitTypedProgramLinks(network, packet.root, 'Rust', {rustSourceHeaderEmissionModel:model});
    assert.equal(linked.code, original.text);
    const scoped = emitTypedProgramLinks(network, packet.root, 'JavaScript', {javaScriptScopedBindingEmissionModel:'owned JavaScript scoped bindings 1'});
    const functionName = source.includes('fn call(') ? 'call' : 'keep';
    const caller = scoped.mappings.find(mapping => mapping.source === functionName).target;
    assert.equal(vm.runInNewContext(`${scoped.code}\n${caller}(3)`, {}, {timeout:1000}), 3);
    const replay = createTypedProgramLinks(`${original.definitions.join('\n')}\nfn main(){}`, 'Rust', {rustTypeNamespaceModel:fixture.typeNamespaceModel});
    const replayProgram = decodeTypedProgramLinks(LinkNetwork.fromLino(replay.network.toLino()), replay.root);
    assert.equal(replayProgram.declarations.get('keep').sourceTypeNamespace.model, fixture.typeNamespaceModel);
    const malformed = structuredClone(program);
    malformed.declarations.get('keep').sourceTypeNamespace = true;
    assert.throws(() => encodeTypedProgramLinks(malformed), /malformed or missing selected facts/u);
  }
});
