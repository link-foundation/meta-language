import assert from 'node:assert/strict';
import vm from 'node:vm';
import { pathToFileURL } from 'node:url';
import { readProjectData, writeProjectData, checkProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';

export const typeNamespaceModel = 'resolved Rust type namespace 1';
const data = name => ({ kind: 'data', name });
const bool = { kind: 'bool' }, float = { kind: 'float' };
const cases = [
  ['root-type-and-value', 'enum f64{Only}mod nested{fn f64()->bool{true}fn header(value:f64)->bool{true}fn value()->bool{f64()}}fn main(){println!("{}",nested::value());println!("{}",nested::header(f64::Only));}', 'nested.header', data('f64'), ['f64'], ['f64'], bool, ['bool'], ['bool'], ['true','true'], 'nested.value', 'nested.f64'],
  ['boolean-datatype', 'enum bool{Only}fn keep(value:bool)->bool{value}fn main(){let value=keep(bool::Only);println!("{}",match value{bool::Only=>true});}', 'keep', data('bool'), ['bool'], ['bool'], data('bool'), ['bool'], ['bool'], ['true']],
  ['closest-type', 'enum f64{Root}mod nested{enum f64{Local}fn header(value:f64)->bool{true}}fn main(){println!("{}",nested::header(nested::f64::Local));}', 'nested.header', data('nested.f64'), ['nested','f64'], ['f64'], bool, ['bool'], ['bool'], ['true']],
  ['crate-qualified', 'enum f64{Only}mod nested{fn f64()->bool{true}fn header(value:crate::f64)->bool{true}}fn main(){}', 'nested.header', data('f64'), ['f64'], ['crate','f64'], bool, ['bool'], ['bool'], []],
  ['self-qualified', 'enum f64{Root}mod nested{enum f64{Local}fn header(value:self::f64)->bool{true}}fn main(){}', 'nested.header', data('nested.f64'), ['nested','f64'], ['self','f64'], bool, ['bool'], ['bool'], []],
  ['super-qualified', 'enum f64{Root}mod nested{enum f64{Local}mod deep{fn header(value:super::f64)->bool{true}}}fn main(){}', 'nested.deep.header', data('nested.f64'), ['nested','f64'], ['super','f64'], bool, ['bool'], ['bool'], []],
  ['deep-super-qualified', 'enum f64{Root}mod nested{fn f64()->bool{true}mod deep{fn header(value:super::super::f64)->bool{true}}}fn main(){}', 'nested.deep.header', data('f64'), ['f64'], ['super','super','f64'], bool, ['bool'], ['bool'], []],
  ['forward-datatype', 'fn keep(value:f64)->f64{value}enum f64{Only}fn main(){}', 'keep', data('f64'), ['f64'], ['f64'], data('f64'), ['f64'], ['f64'], []],
  ['value-function-only', 'fn f64()->bool{true}fn header(value:f64)->f64{value}fn main(){println!("{}",f64());}', 'header', float, ['f64'], ['f64'], float, ['f64'], ['f64'], ['true']],
  ['boolean-value-only', 'fn bool()->bool{true}fn header(value:bool)->bool{value}fn main(){println!("{}",bool());}', 'header', bool, ['bool'], ['bool'], bool, ['bool'], ['bool'], ['true']],
  ['borrowed-datatype', 'enum f64{Only}fn header(value:&f64)->bool{true}fn main(){println!("{}",header(&f64::Only));}', 'header', data('f64'), ['f64'], ['f64'], bool, ['bool'], ['bool'], ['true'], null, null, ['reference'], [null]],
  ['named-reference', "enum f64{Only}fn header<'a>(value:&'a f64)->bool{true}fn main(){}", 'header', data('f64'), ['f64'], ['f64'], bool, ['bool'], ['bool'], [], null, null, ['reference'], ['a']],
  ['string-token-datatype', 'enum str{Only}fn header(value:&str)->bool{true}fn main(){}', 'header', data('str'), ['str'], ['str'], bool, ['bool'], ['bool'], [], null, null, ['reference'], [null]],
  ['fixed-token-datatype', 'enum u8{Only}fn keep(value:u8)->u8{value}fn main(){}', 'keep', data('u8'), ['u8'], ['u8'], data('u8'), ['u8'], ['u8'], []],
  ['forward-string-token-datatype', 'fn header(value:str)->bool{true}enum str{Only}fn main(){}', 'header', data('str'), ['str'], ['str'], bool, ['bool'], ['bool'], []],
];
const refused = [
  ['root-super', 'enum f64{Only}fn header(value:super::f64)->bool{true}fn main(){}'],
  ['above-crate', 'enum f64{Only}mod nested{fn header(value:super::super::f64)->bool{true}}fn main(){}'],
  ['self-value-only', 'enum f64{Only}mod nested{fn f64()->bool{true}fn header(value:self::f64)->bool{true}}fn main(){}'],
  ['super-value-only', 'enum f64{Only}mod nested{fn f64()->bool{true}mod deep{fn header(value:super::f64)->bool{true}}}fn main(){}'],
  ['borrowed-result', 'fn keep(value:&f64)->&f64{value}fn main(){}'],
  ['named-borrowed-result', "fn keep<'a>(value:&'a f64)->&'a f64{value}fn main(){}"],
  ['box-input', 'fn keep(value:Box<f64>)->f64{*value}fn main(){}'],
  ['vector-input', 'fn keep(value:Vec<f64>)->bool{value.is_empty()}fn main(){}'],
  ['tuple-input', 'fn keep(value:(f64,bool))->bool{value.1}fn main(){}'],
  ['unit-input', 'fn keep(value:())->bool{true}fn main(){}'],
  ['slice-input', 'fn keep(value:&[f64])->bool{value.is_empty()}fn main(){}'],
  ['string-library', 'fn keep(value:String)->bool{true}fn main(){}'],
  ['imported-type', 'enum Choice{Only}mod nested{use crate::Choice;fn keep(value:Choice)->Choice{value}}fn main(){}'],
  ['constant-projection', 'const VALUE:f64=1.0;fn keep(value:f64)->f64{value}fn main(){}'],
  ['constructor-type', 'enum Choice{Only}fn keep(value:Choice::Only)->bool{true}fn main(){}'],
  ['shared-module-type-identity', 'enum Choice{Only}mod Choice{fn keep(value:crate::Choice)->bool{true}}fn main(){}'],
  ['same-type-value-identity', 'enum f64{Only}fn f64()->bool{true}fn keep(value:f64)->bool{true}fn main(){}'],
  ['boolean-string-from', 'fn bad()->bool{String::from(true)=="true"}fn main(){}'],
  ['datatype-string-from', 'enum String{Only}fn bad()->bool{String::from("x")=="x"}fn main(){}'],
  ['unproved-standard-string-from', 'fn good()->bool{String::from("x")=="x"}fn main(){}'],
  ['unboxed-primitive-string-slice', 'fn header(value:str)->bool{true}fn main(){}'],
  ['datatype-signed-from', 'enum i64{Only}fn bad()->bool{i64::from(1i32)==1i64}fn main(){}'],
  ['datatype-byte-from', 'enum u8{Only}fn keep()->bool{u8::from(1u8)==1u8}fn main(){}'],
  ['datatype-word-from', 'enum u32{Only}fn keep()->bool{u32::from(1u32)==1u32}fn main(){}'],
];
const output = code => { const lines = []; vm.runInNewContext(code, {console:{log:(...args)=>lines.push(args.map(String).join(' '))}}, {timeout:1000}); return lines; };
const schema = readProjectData(new URL('../src/data/typed-program-links-schema.lino', import.meta.url));
export function rustTypeNamespaceField(network, parent, name) {
  return network.fieldRelations().find(row => row.parent.equals(parent) && row.label === (schema.fields[name] ?? name.replace(/[A-Z]/gu, letter => ` ${letter.toLowerCase()}`)))?.child;
}
export function rustTypeNamespaceContainerFixture() {
  const sources = [
    ['primitive', 'fn keep(value:f64)->f64{value}fn main(){println!("{}",keep(1f64)==1f64);}'],
    ['unit-clone', '#[derive(Clone)]enum Choice{Only}fn keep(value:Choice)->Choice{value.clone()}fn main(){let value=keep(Choice::Only);println!("{}",match value{Choice::Only=>true});}'],
  ];
  const cases = sources.map(([id, source]) => {
    const archived = createTypedProgramLinks(source, 'Rust', {rustTypeNamespaceModel:typeNamespaceModel});
    const loaded = LinkNetwork.fromLino(archived.network.toLino());
    assert.deepEqual(output(emitTypedProgramLinks(loaded, archived.root, 'JavaScript').code), ['true']);
    return {id, source, archive:archived.network.toLino(), root:archived.root.asU64(), output:['true']};
  });
  const locations = [
    ['primitive', ['declarations'], 'map'],
    ['primitive', ['items'], 'array'],
    ['primitive', ['main','effects'], 'array'],
    ...[['params'], ['modulePath'], ['sourceTypeNamespace','parameters'], ['sourceTypeNamespace','parameters',0], ['sourceTypeNamespace','result'],
      ['sourceSignature','parameters'], ['sourceSignature','lifetimeBindings'], ['sourceSignature','parameters',0,'wrappers'], ['sourceSignature','parameters',0,'lifetimes'],
      ['sourceSignature','parameters',0,'owner','path'], ['sourceSignature','parameters',0,'owner','arguments'], ['sourceSignature','result','wrappers'],
      ['sourceSignature','result','lifetimes'], ['sourceSignature','result','owner','path'], ['sourceSignature','result','owner','arguments']]
      .map(path => ['primitive', ['declarations','keep',...path], 'array']),
    ...[['datatypeOwner'], ['traitOwner'], ['constructorNames'], ['attribute','traits']]
      .map(path => ['unit-clone', ['declarations','Choice','sourceUnitCloneDerivation',...path], 'array']),
    ['unit-clone', ['declarations','Choice','ctors'], 'array'],
    ['unit-clone', ['declarations','Choice','ctors',0,'fields'], 'array'],
    ['unit-clone', ['declarations','Choice','sourceUnitCloneDerivation'], 'record'],
    ['unit-clone', ['declarations','Choice','sourceUnitCloneDerivation','attribute'], 'record'],
    ['primitive', ['declarations','keep','sourceTypeNamespace'], 'record'],
  ].map(([caseId,path,kind]) => ({caseId,path,kind}));
  for(const location of locations){
    const entry=cases.find(value=>value.id===location.caseId);
    const program=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root);
    const value=location.path.reduce((record,key)=>record instanceof Map?Map.prototype.get.call(record,key):record?.[key],program);
    assert.equal(location.kind==='map'?value instanceof Map:location.kind==='array'?Array.isArray(value):value!==null&&typeof value==='object',true,location.path.join('.'));
  }
  const mutations = [
    {kind:'array', operations:['own field','hidden field','symbol field','accessor field','prototype','hole','hidden position','accessor position'], reason:'an owned Rust type namespace an ordered container has unrepresented own fields or sparse positions'},
    {kind:'map', operations:['own field','hidden field','symbol field','accessor field','prototype','iterator'], reason:'an owned Rust type namespace a declaration Map has unrepresented own fields'},
    {kind:'record', operations:['hidden field','symbol field','accessor field','prototype'], reason:'an owned Rust type namespace an owned record has unrepresented own fields or a prototype'},
  ];
  return {schemaVersion:1,typeNamespaceModel,scope:'selected exact dense host containers and data descriptors before encoding; no host-only metadata may disappear through source-free Links reload',cases,locations,mutations};
}
export function rustTypeNamespaceTraitFixture() {
  const cases=['Clone','Clone, Debug, PartialEq, Eq'].map((traits,index)=>{
    const source=`#[derive(${traits})]enum Choice{Only}fn keep(value:Choice)->Choice{value.clone()}fn main(){let value=keep(Choice::Only);println!("{}",match value{Choice::Only=>true});}`;
    const archive=createTypedProgramLinks(source,'Rust',{rustTypeNamespaceModel:typeNamespaceModel}),loaded=LinkNetwork.fromLino(archive.network.toLino());
    const expected=decodeTypedProgramLinks(loaded,archive.root).declarations.get('Choice').sourceUnitCloneDerivation;
    assert.deepEqual(expected.attribute.traits,traits.split(', '));assert.deepEqual(expected.datatypeOwner,['Choice']);assert.deepEqual(expected.constructorNames,['Only']);assert.equal(expected.attribute.span.start,0);assert.equal(expected.attribute.span.end,source.indexOf('enum'));
    assert.deepEqual(output(emitTypedProgramLinks(loaded,archive.root,'JavaScript').code),['true']);
    return{id:index?'combined-unit-clone':'single-unit-clone',source,expected,output:['true'],archive:archive.network.toLino(),root:archive.root.asU64()};
  });
  const sources=[
    ['datatype-clone','enum Choice{Only}fn bad(value:Choice)->Choice{value.clone()}fn main(){}'],
    ['datatype-to-owned','enum Choice{Only}fn bad(value:Choice)->Choice{value.to_owned()}fn main(){}'],
    ['primitive-to-string','fn bad()->bool{true.to_string()=="true"}fn main(){}'],
    ['datatype-as-string-slice','enum Choice{Only}fn bad(value:Choice)->bool{let text=value.as_str();true}fn main(){}'],
    ['unproved-derived-clone','#[derive(Clone)]enum Choice{Only(f64)}fn bad(value:Choice)->Choice{value.clone()}fn main(){}'],
    ['unproved-primitive-clone','fn keep(value:f64)->f64{value.clone()}fn main(){}'],
  ];
  const invalid=sources.map(([id,source])=>{
    assert.doesNotThrow(()=>checkProgram(parseRust(source)),`${id}: unchanged legacy parsing`);
    try{checkProgram(parseRust(source,{typeNamespaceModel}));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
    throw new Error(`unproved selected trait method admitted: ${id}`);
  });
  const malformed=[];
  for(const [id,edit] of [
    ['null-derivation',(n,proof)=>{const row=n.fieldRelations().find(value=>value.child.equals(proof));const nil=n.links().find(value=>value.metadata().term===schema.scalars.null);row.link.setReferences([...row.link.references().slice(0,2),nil.id()]);}],
    ['foreign-derivation-model',(n,proof)=>{const value=rustTypeNamespaceField(n,proof,'model');n.link(value).setMetadata(n.link(value).metadata().withDefinition('foreign clone derivation'));}],
    ['missing-clone-argument',(n,proof)=>{const argument=n.link(rustTypeNamespaceField(n,rustTypeNamespaceField(n,proof,'attribute'),'traits')).references()[0];n.link(argument).setMetadata(n.link(argument).metadata().withDefinition('Debug'));}],
  ]){
    const base=cases[0],network=LinkNetwork.fromLino(base.archive),proof=network.fieldRelations().find(row=>row.label===schema.fields.sourceUnitCloneDerivation).child;
    edit(network,proof);
    for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(LinkNetwork.fromLino(network.toLino()),base.root,target),undefined,id);
    malformed.push({id,archive:network.toLino(),root:base.root});
  }
  return{schemaVersion:1,typeNamespaceModel,scope:'selected method calls require a separately owned actual method implementation; only attached standard field-free datatype Clone derives establish the bounded clone premise',cases,invalid,malformed};
}
export function rustTypeNamespaceFixture() {
  const entries = cases.map(([id, source, functionName, parameterType, parameterOwner, parameterSpelling, resultType, resultOwner, resultSpelling, lines, callFunction, callTarget, wrappers=[], lifetimes=[]]) => {
    const program = checkProgram(parseRust(source, {typeNamespaceModel})), declaration = program.declarations.get(functionName);
    const expected = {parameterType, parameterOwner, parameterSpelling, resultType, resultOwner, resultSpelling, wrappers, lifetimes};
    assert.deepEqual(declaration.params[0].type, parameterType, id);
    assert.deepEqual(declaration.sourceSignature.parameters[0].owner.path, parameterOwner, id);
    assert.deepEqual(declaration.sourceSignature.parameters[0].wrappers, wrappers, id);
    assert.deepEqual(declaration.sourceSignature.parameters[0].lifetimes, lifetimes, id);
    assert.deepEqual(declaration.ret, resultType, id);
    assert.deepEqual(declaration.sourceSignature.result.owner.path, resultOwner, id);
    assert.deepEqual(declaration.sourceTypeNamespace, {model:typeNamespaceModel, language:'Rust', parameters:[parameterSpelling], result:resultSpelling}, id);
    if (callFunction) assert.equal(program.declarations.get(callFunction).body.fn, callTarget, id);
    const archive = createTypedProgramLinks(source, 'Rust', {rustTypeNamespaceModel:typeNamespaceModel, provenance:{project:id}});
    const loaded = LinkNetwork.fromLino(archive.network.toLino());
    assert.deepEqual(decodeTypedProgramLinks(loaded, archive.root).declarations.get(functionName).sourceTypeNamespace, declaration.sourceTypeNamespace, id);
    assert.deepEqual(output(emitTypedProgramLinks(loaded, archive.root, 'JavaScript').code), lines, id);
    return {id, source, functionName, expected, output:lines, callFunction:callFunction??null, callTarget:callTarget??null, archive:archive.network.toLino(), root:archive.root.asU64()};
  });
  const invalid = refused.map(([id,source]) => {
    try { checkProgram(parseRust(source,{typeNamespaceModel})); } catch (error) { return {id,source,kind:error.kind,reason:error.reason,span:error.span??null}; }
    throw new Error(`selected type namespace refusal was admitted: ${id}`);
  });
  const base = entries[0], malformed = [];
  const set = (network,id,text) => network.link(id).setMetadata(network.link(id).metadata().withDefinition(text));
  const field = rustTypeNamespaceField;
  const relation = (network,parent,name) => network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===(schema.fields[name]??name));
  const scalar = (network,proof,kind,text) => network.insertLink([],network.link(field(network,proof,'model')).metadata().withTerm(schema.scalars[kind]).withDefinition(text));
  const edit = (id, mutate) => {
    const network = LinkNetwork.fromLino(base.archive), program = decodeTypedProgramLinks(network, base.root);
    const fn = network.fieldRelations().find(row => row.label === 'full name' && network.link(row.child).metadata().definition === base.functionName)?.parent;
    assert.ok(fn, id);
    const facet = field(network,fn,'sourceTypeNamespace');
    mutate(network,fn,facet,program);
    for (const target of ['JavaScript','Rust']) assert.throws(()=>emitTypedProgramLinks(LinkNetwork.fromLino(network.toLino()),base.root,target),undefined,id);
    malformed.push({id,archive:network.toLino(),root:base.root});
  };
  edit('wrong-model',(n,_fn,p)=>set(n,field(n,p,'model'),'unproved type namespace'));
  edit('wrong-language',(n,_fn,p)=>set(n,field(n,p,'language'),'JavaScript'));
  edit('missing-position',(n,_fn,p)=>n.link(field(n,p,'parameters')).setReferences([]));
  edit('empty-spelling',(n,_fn,p)=>n.link(n.link(field(n,p,'parameters')).references()[0]).setReferences([]));
  edit('wrong-anchor',(n,_fn,p)=>set(n,n.link(n.link(field(n,p,'parameters')).references()[0]).references()[0],'self'));
  edit('above-crate',(n,_fn,p)=>{
    const spelling=n.link(field(n,p,'parameters')).references()[0],old=n.link(spelling).references()[0];
    const parent=n.insertLink([],n.link(old).metadata().withDefinition('super'));n.link(spelling).setReferences([parent,parent,old]);
  });
  edit('forged-datatype',(n,_fn,p)=>set(n,n.link(n.link(field(n,p,'parameters')).references()[0]).references()[0],'bool'));
  edit('forged-result',(n,_fn,p)=>set(n,n.link(field(n,p,'result')).references()[0],'f64'));
  edit('opaque-proof-field',(n,_fn,p)=>n.insertFieldRelation(p,'opaque namespace fact',field(n,p,'model')));
  edit('async-header',(n,fn,p)=>n.insertFieldRelation(fn,'async',field(n,p,'model')));
  edit('opaque-header',(n,fn,p)=>n.insertFieldRelation(fn,'opaque interface fact',field(n,p,'model')));
  edit('opaque-parameter',(n,fn,p)=>n.insertFieldRelation(n.link(field(n,fn,'params')).references()[0],'source span',field(n,p,'model')));
  edit('missing-signature',(n,fn)=>relation(n,fn,'sourceSignature').link.setReferences([]));
  edit('missing-selected-facts',(n,fn)=>relation(n,fn,'sourceTypeNamespace').link.setReferences([]));
  for(const [id,kind,text] of [['null','null',''],['zero','number','0'],['empty-string','string',''],['false','boolean','false']]) {
    edit(`scalar-proof-${id}`,(n,fn,p)=>{const row=relation(n,fn,'sourceTypeNamespace');row.link.setReferences([...row.link.references().slice(0,2),scalar(n,p,kind,text)]);});
  }
  for(const [id,kind,text] of [['null','null',''],['zero','number','0'],['empty-string','string',''],['true','boolean','true']]) {
    edit(`generated-${id}`,(n,fn,p)=>n.insertFieldRelation(fn,'generated',scalar(n,p,kind,text)));
  }
  for(const value of ['false','true'])edit(`optional-${value}`,(n,fn,p)=>n.insertFieldRelation(n.link(field(n,fn,'params')).references()[0],'optional',scalar(n,p,'boolean',value)));
  {
    const entry=entries.find(value=>value.id==='closest-type'),network=LinkNetwork.fromLino(entry.archive);
    const fn=network.fieldRelations().find(row=>row.label==='full name'&&network.link(row.child).metadata().definition===entry.functionName).parent;
    const proof=field(network,fn,'sourceTypeNamespace'),spelling=network.link(field(network,proof,'parameters')).references()[0];
    set(network,network.link(spelling).references()[0],'nested.f64');
    for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(LinkNetwork.fromLino(network.toLino()),entry.root,target),undefined,'joined-type-segment');
    malformed.push({id:'joined-type-segment',archive:network.toLino(),root:entry.root});
  }
  const network = LinkNetwork.fromLino(base.archive);
  const fn = network.fieldRelations().find(row => row.label==='full name' && network.link(row.child).metadata().definition===base.functionName).parent;
  const facet = field(network,fn,'sourceTypeNamespace'), spelling = network.link(field(network,facet,'parameters')).references()[0];
  const old = network.link(spelling).references()[0];
  const crate = network.insertLink([], network.link(old).metadata().withDefinition('crate'));
  network.link(spelling).setReferences([crate,old]);
  assert.deepEqual(decodeTypedProgramLinks(LinkNetwork.fromLino(network.toLino()),base.root).declarations.get(base.functionName).sourceTypeNamespace.parameters,[['crate','f64']]);
  assert.deepEqual(output(emitTypedProgramLinks(network,base.root,'JavaScript').code),base.output);
  return {schemaVersion:1,typeNamespaceModel,scope:'owned source header type bindings before primitive projection; immutable input references retain source facts without outlives, pointer or ABI equivalence',cases:entries,invalid,malformed,spellingEdit:{archive:network.toLino(),root:base.root,functionName:base.functionName,output:base.output}};
}
if (process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href) {
  const file=new URL('../../parity/fixtures/rust-type-namespace.lino',import.meta.url),fixture=rustTypeNamespaceFixture();
  if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
  const traitsFile=new URL('../../parity/fixtures/rust-type-namespace-traits.lino',import.meta.url),traits=rustTypeNamespaceTraitFixture();
  if(process.argv.includes('--check'))checkProjectData(traitsFile,traits);else writeProjectData(traitsFile,traits);
  const containersFile=new URL('../../parity/fixtures/rust-type-namespace-containers.lino',import.meta.url),containers=rustTypeNamespaceContainerFixture();
  if(process.argv.includes('--check'))checkProjectData(containersFile,containers);else writeProjectData(containersFile,containers);
  console.log(`Rust type namespace: ${fixture.cases.length} actual source bindings, ${fixture.invalid.length} source refusals, ${fixture.malformed.length} source-free corruptions`);
  console.log(`Rust type namespace traits: ${traits.invalid.length} precise unproved method refusals`);
}
