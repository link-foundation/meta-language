import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustTypeNamespaceFixture, rustTypeNamespaceTraitFixture, rustTypeNamespaceContainerFixture, rustTypeNamespaceField } from '../scripts/generate-rust-type-namespace-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { RUST_SOURCE_SIGNATURE_POLICY } from '../src/translation/rust-source-signatures.js';
import { RUST_TYPE_NAMESPACE_POLICY, acceptRustTypeNamespaceSpellingSegment } from '../src/translation/rust-type-namespace.js';
import { tokenize } from '../src/translation/lexer.js';
import { rustFixedType } from '../src/translation/types.js';
import { createTypedProgramLinks, encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-type-namespace.lino', import.meta.url));
const options = {typeNamespaceModel:fixture.typeNamespaceModel};
const output = code => { const lines=[]; vm.runInNewContext(code,{console:{log:(...args)=>lines.push(args.map(String).join(' '))}},{timeout:1000});return lines; };
const field = rustTypeNamespaceField;

test('selected owned containers refuse metadata loss before encoding and retain unmodified source-free execution', () => {
  const containers=readProjectData(new URL('../../parity/fixtures/rust-type-namespace-containers.lino',import.meta.url));
  assert.deepEqual(rustTypeNamespaceContainerFixture(),containers);
  const at=(root,path)=>path.reduce((value,key)=>value instanceof Map?Map.prototype.get.call(value,key):value[key],root);
  const mutate=(value,operation,invoked)=>{
    if(operation==='own field')value.borrowEscape=true;
    else if(operation==='hidden field')Object.defineProperty(value,'borrowEscape',{value:true,enumerable:false});
    else if(operation==='symbol field')value[Symbol('borrowEscape')]=true;
    else if(operation==='accessor field')Object.defineProperty(value,'borrowEscape',{enumerable:true,get(){invoked.count++;return true;}});
    else if(operation==='prototype')Object.setPrototypeOf(value,Object.create(Object.getPrototypeOf(value)));
    else if(operation==='iterator')value[Symbol.iterator]=function(){invoked.count++;throw new Error('overridden declaration iterator ran');};
    else if(operation==='hole'){value.length=Math.max(1,value.length);delete value[0];}
    else if(operation==='hidden position'){if(!value.length)value.push('unrepresented');Object.defineProperty(value,'0',{enumerable:false});}
    else if(operation==='accessor position'){if(!value.length)value.push('unrepresented');Object.defineProperty(value,'0',{enumerable:true,get(){invoked.count++;return 'unrepresented';}});}
    else assert.fail(`unknown actual container mutation: ${operation}`);
  };
  for(const entry of containers.cases){
    const program=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root),rebuilt=encodeTypedProgramLinks(program);
    assert.deepEqual(output(emitTypedProgramLinks(LinkNetwork.fromLino(rebuilt.network.toLino()),rebuilt.root,'JavaScript').code),entry.output,entry.id);
  }
  let observed=0;
  for(const location of containers.locations){
    const entry=containers.cases.find(value=>value.id===location.caseId),rules=containers.mutations.find(value=>value.kind===location.kind);
    for(const operation of rules.operations){
      const program=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root),value=at(program,location.path),invoked={count:0};
      assert.equal(location.kind==='map'?value instanceof Map:location.kind==='array'?Array.isArray(value):typeof value==='object',true,JSON.stringify(location));
      mutate(value,operation,invoked);
      assert.throws(()=>encodeTypedProgramLinks(program),error=>{assert.equal(error.reason,rules.reason,`${location.path.join('.')}: ${operation}`);return true;});
      assert.equal(invoked.count,0,`${location.path.join('.')}: descriptor and iterator overrides must not execute`);
      observed++;
    }
  }
  assert.equal(observed,containers.locations.reduce((count,location)=>count+containers.mutations.find(value=>value.kind===location.kind).operations.length,0));
});

test('selected source checker rejects erased container properties before surface projection',()=>{
  const surface=parseRust(fixture.cases[0].source,options);
  surface.items.borrowEscape=true;
  assert.throws(()=>checkProgram(surface),/ordered container has unrepresented own fields/u);
  const legacy=parseRust('fn keep(value:f64)->f64{value}fn main(){}');
  legacy.items.borrowEscape=true;
  assert.doesNotThrow(()=>checkProgram(legacy));
});

test('Clone datatype paths use their distinct Links role while source type owner descriptors retain theirs',()=>{
  const traits=readProjectData(new URL('../../parity/fixtures/rust-type-namespace-traits.lino',import.meta.url));
  const network=LinkNetwork.fromLino(traits.cases[0].archive),program=decodeTypedProgramLinks(network,traits.cases[0].root);
  const proof=program.declarations.get('Choice').sourceUnitCloneDerivation;
  assert.deepEqual(proof.datatypeOwner,['Choice']);assert.equal(Object.hasOwn(proof,'owner'),false);
  assert.ok(network.fieldRelations().some(row=>row.label==='source derivation datatype owner'));
  assert.deepEqual(program.declarations.get('keep').sourceSignature.parameters[0].owner,{kind:'declaration',path:['Choice'],arguments:[]});
  assert.ok(network.fieldRelations().some(row=>row.label==='source type owner'));
});

test('fifteen actual source headers distinguish datatype and value namespaces before primitive projection', () => {
  assert.deepEqual(rustTypeNamespaceFixture(), fixture);
  assert.equal(fixture.cases.length,15);
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

test('twenty-four unresolved or unproved source bindings retain precise selected refusals', () => {
  assert.equal(fixture.invalid.length,24);
  for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,options)),error=>{
    assert.equal(error.kind,entry.kind,entry.id);assert.equal(error.reason,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;
  });
});

test('initial namespace discovery defers the str token while the final primitive position stays refused', () => {
  const forward=fixture.cases.find(value=>value.id==='forward-string-token-datatype');
  const checked=checkProgram(parseRust(forward.source,options));
  assert.deepEqual(checked.declarations.get('header').params[0].type,{kind:'data',name:'str'});
  assert.deepEqual(checked.declarations.get('header').sourceSignature.parameters[0].owner,{kind:'declaration',path:['str'],arguments:[]});
  assert.deepEqual(checked.declarations.get('header').sourceTypeNamespace.parameters,[['str']]);
  const unsupported=fixture.invalid.find(value=>value.id==='unboxed-primitive-string-slice');
  assert.throws(()=>createTypedProgramLinks(unsupported.source,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel}),/Rust type str: outside the portable core/u);
  assert.throws(()=>checkProgram(parseRust(forward.source)),/Rust type str: outside the portable core/u);
});

test('selected String constructors require an actual owner and argument proof without changing legacy parsing', () => {
  for(const id of ['boolean-string-from','datatype-string-from','unproved-standard-string-from']) {
    const entry=fixture.invalid.find(value=>value.id===id);
    assert.ok(entry,id);
    assert.throws(()=>createTypedProgramLinks(entry.source,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel}),error=>{
      assert.equal(error.kind,'unsupported',id);assert.equal(error.reason,'Rust type namespace body constructor: String::from requires a separate actual owner and argument proof',id);assert.deepEqual(error.span,entry.span,id);return true;
    });
    assert.doesNotThrow(()=>checkProgram(parseRust(entry.source)),`${id}: preserved legacy frontend`);
    assert.doesNotThrow(()=>checkProgram(parseRust(entry.source,{sourceSignatureModel:'Rust source function signature 1'})),`${id}: unchanged capture1 frontend`);
    assert.deepEqual(parseRust(entry.source,{typeNamespaceModel:''}),parseRust(entry.source),id);
  }
});

test('every currently recognized integer from shortcut remains unproved under the selected namespace model', () => {
  const owners=Object.keys(RUST_SOURCE_SIGNATURE_POLICY.primitives).filter(name=>rustFixedType(name));
  assert.equal(owners.length,12);
  assert.deepEqual(RUST_TYPE_NAMESPACE_POLICY.unprovedBodyConstructors,['String::from',...owners.map(name=>`${name}::from`)]);
  for(const owner of owners)for(const declaration of ['',`enum ${owner}{Only}`]) {
    const source=`${declaration}fn keep()->bool{${owner}::from(1${owner})==1${owner}}fn main(){}`;
    assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel}),error=>{
      assert.equal(error.kind,'unsupported');assert.equal(error.reason,`Rust type namespace body constructor: ${owner}::from requires a separate actual owner and argument proof`);return true;
    });
    assert.doesNotThrow(()=>checkProgram(parseRust(source)),`${owner}: legacy shortcut unchanged`);
    assert.deepEqual(parseRust(source,{typeNamespaceModel:''}),parseRust(source),owner);
  }
});

test('selected trait and string-slice methods require actual owned implementations without assuming parsed derives', () => {
  const traits=readProjectData(new URL('../../parity/fixtures/rust-type-namespace-traits.lino',import.meta.url));
  assert.deepEqual(rustTypeNamespaceTraitFixture(),traits);assert.equal(traits.invalid.length,6);
  assert.equal(traits.cases.length,2);assert.equal(traits.malformed.length,3);
  for(const entry of traits.cases){const network=LinkNetwork.fromLino(entry.archive);assert.deepEqual(decodeTypedProgramLinks(network,entry.root).declarations.get('Choice').sourceUnitCloneDerivation,entry.expected);assert.deepEqual(output(emitTypedProgramLinks(network,entry.root,'JavaScript').code),entry.output);}
  for(const entry of traits.malformed)for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,target),/unit Clone derivation/u,entry.id);
  for(const entry of traits.invalid) {
    assert.throws(()=>createTypedProgramLinks(entry.source,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel}),error=>{
      assert.equal(error.kind,entry.kind,entry.id);assert.equal(error.reason,entry.reason,entry.id);assert.deepEqual(error.span,entry.span,entry.id);return true;
    });
    assert.doesNotThrow(()=>checkProgram(parseRust(entry.source)),entry.id);
    assert.deepEqual(parseRust(entry.source,{typeNamespaceModel:''}),parseRust(entry.source),entry.id);
  }
  for(const operator of ['==','!=']) {
    const source=`enum Choice{Only}fn bad(value:Choice)->bool{value${operator}value}fn main(){}`;
    assert.throws(()=>checkProgram(parseRust(source,options)),/structural equality of data values/u);
    assert.throws(()=>checkProgram(parseRust(source)),/structural equality of data values/u);
  }
});

test('an actual attached unit datatype Clone derive is owned, source-free and permits the preserved emitted definition replay', () => {
  for(const traits of ['Clone','Clone, Debug, PartialEq, Eq']) {
    const source=`#[derive(${traits})]enum Choice{Only}fn keep(value:Choice)->Choice{value.clone()}fn main(){let value=keep(Choice::Only);println!("{}",match value{Choice::Only=>true});}`;
    const archive=createTypedProgramLinks(source,'Rust',{rustTypeNamespaceModel:fixture.typeNamespaceModel});
    const restored=LinkNetwork.fromLino(archive.network.toLino()),program=decodeTypedProgramLinks(restored,archive.root);
    const proof=program.declarations.get('Choice').sourceUnitCloneDerivation;
    assert.equal(proof.model,'Rust unit datatype Clone derivation 1');assert.deepEqual(proof.datatypeOwner,['Choice']);assert.deepEqual(proof.traitOwner,['core','clone','Clone']);assert.deepEqual(proof.constructorNames,['Only']);assert.deepEqual(proof.attribute.traits,traits.split(', '));
    assert.equal(proof.attribute.span.start,0);assert.equal(proof.attribute.span.end,source.indexOf('enum'));
    assert.deepEqual(output(emitTypedProgramLinks(restored,archive.root,'JavaScript').code),['true']);
    for(const mutate of [
      value=>value.sourceUnitCloneDerivation=null,
      value=>value.sourceUnitCloneDerivation.datatypeOwner=['foreign'],
      value=>value.sourceUnitCloneDerivation.constructorNames=['different'],
      value=>value.sourceUnitCloneDerivation.traitOwner=['foreign','Clone'],
      value=>value.sourceUnitCloneDerivation.attribute.traits=['Debug'],
      value=>value.sourceUnitCloneDerivation.attribute.traits=['Clone','Clone'],
      value=>value.sourceUnitCloneDerivation.attribute.span.end=value.span.start+1,
      value=>value.sourceUnitCloneDerivation.opaque=true,
      value=>value.ctors[0].fields.push({name:'field0',type:{kind:'float'}}),
    ]) {
      const modified=structuredClone(program);mutate(modified.declarations.get('Choice'));
      assert.throws(()=>encodeTypedProgramLinks(modified),/unit Clone derivation/u);
    }
  }
  for(const source of [
    '#[derive(Clone,Clone)]enum Choice{Only}fn keep(value:Choice)->Choice{value.clone()}fn main(){}',
    '#[derive(Clone)]#[derive(Clone)]enum Choice{Only}fn keep(value:Choice)->Choice{value.clone()}fn main(){}',
    '#[derive(Clone)]fn keep(value:f64)->f64{value}fn main(){}',
    '#[derive(Clone)]enum Choice{Only}fn keep(value:&Choice)->Choice{value.clone()}fn main(){}',
    '#[derive(Clone)]enum Choice{Only}extern crate alloc;fn keep(value:Choice)->Choice{value.clone()}fn main(){}',
  ])assert.throws(()=>checkProgram(parseRust(source,options)),/Clone derivation|method owner proof|external crate declarations/u);
});

test('twenty-five actual malformed header Links graphs refuse both targets after source-free reload', () => {
  assert.equal(fixture.malformed.length,25);
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

test('owned namespace segments match actual Rust lexer identifiers and cannot alias dotted catalog paths', () => {
  for(let code=0;code<128;code+=1)for(const prefix of ['', 'name']) {
    const value=prefix+String.fromCharCode(code);
    let lexical=false;
    try {const {tokens}=tokenize(value,'Rust');const actual=tokens.filter(token=>token.kind!=='eof');lexical=actual.length===1&&actual[0].kind==='identifier'&&actual[0].raw===value;}catch{}
    assert.equal(acceptRustTypeNamespaceSpellingSegment(value),lexical,JSON.stringify(value));
  }
  for(const value of ['','nested.f64','nested::f64','nested f64','1f64','é','self.'])assert.equal(acceptRustTypeNamespaceSpellingSegment(value),false,value);
  for(const value of ['crate','self','super','_Type2'])assert.equal(acceptRustTypeNamespaceSpellingSegment(value),true,value);
  const parsed=checkProgram(parseRust(fixture.cases.find(value=>value.id==='closest-type').source,options));
  for(const spelling of [['nested.f64'],['nested::f64'],['nested f64']]) {
    const program=structuredClone(parsed);program.declarations.get('nested.header').sourceTypeNamespace.parameters=[spelling];
    assert.throws(()=>encodeTypedProgramLinks(program),/actual primitive or datatype spelling/u);
  }
  const program=structuredClone(parsed);program.declarations.get('nested.f64').name='fake.type';
  assert.throws(()=>encodeTypedProgramLinks(program),/qualified spelling|identifier segment/u);
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
