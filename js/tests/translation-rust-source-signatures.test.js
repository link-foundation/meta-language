import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustSourceSignatureFixture} from '../scripts/generate-rust-source-signature-fixtures.mjs';
import {parseRust} from '../src/translation/rust.js';
import {RustSourceOrigins} from '../src/translation/rust-source-origins.js';
import {RUST_SOURCE_SIGNATURE_POLICY} from '../src/translation/rust-source-signatures.js';
import {checkProgram} from '../src/translation/check.js';
import {threadOutput} from '../src/translation/output.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-signatures.lino',import.meta.url));
const options={sourceSignatureModel:fixture.sourceSignatureModel};
function output(code){const values=[];vm.runInNewContext(code,{console:{log:(...args)=>values.push(args.map(String).join(' '))}},{timeout:1000});return values;}
function field(network,parent,label){return network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label)?.child;}

test('owned source headers bind actual integral Number widths and Boolean signedness before type comparison',()=>{
 const cases=[
  ['pub fn keep(value:i32)->i32{value} fn main(){}',position=>position.type],
  ['pub fn keep(value:(bool,i32))->i32{value.1} fn main(){}',position=>position.type.elements[1]],
  ['pub fn keep(value:&[i32])->i32{0i32} fn main(){}',position=>position.type.element],
 ];
 for(const[source,select]of cases){
  const original=createTypedProgramLinks(source,'Rust',{rustSourceSignatureModel:fixture.sourceSignatureModel});
  const load=()=>decodeTypedProgramLinks(LinkNetwork.fromLino(original.network.toLino()),original.root);
  for(const[slot,values]of [['bits',['32',new Number(32),null,true,32.5,-0,NaN,Infinity,7,0]],['signed',['true','false',1,0,null,new Boolean(true)]]]){
   for(const value of values){
    const program=load(),declaration=[...program.declarations.values()][0];
    declaration.sourceSignature.parameters[0].type=structuredClone(declaration.sourceSignature.parameters[0].type);
    select(declaration.sourceSignature.parameters[0])[slot]=value;
    assert.throws(()=>encodeTypedProgramLinks(program),undefined,source+' '+slot+' '+String(value));
   }
  }
  for(const width of[32,32.0,3.2e1]){
   const program=load(),declaration=[...program.declarations.values()][0];
   select(declaration.sourceSignature.parameters[0]).bits=width;
   const saved=encodeTypedProgramLinks(program),network=LinkNetwork.fromLino(saved.network.toLino());
   const decoded=decodeTypedProgramLinks(network,saved.root);
   assert.equal(typeof select([...decoded.declarations.values()][0].sourceSignature.parameters[0]).bits,'number');
   for(const target of['Rust','JavaScript'])assert.equal(emitTypedProgramLinks(network,saved.root,target).code,emitTypedProgramLinks(original.network,original.root,target).code);
  }
 }
});

test('nineteen actual Rust headers retain independently specified type owners, wrappers and lifetime positions',()=>{
 assert.deepEqual(rustSourceSignatureFixture(),fixture);
 assert.equal(fixture.cases.length,19);
 for(const entry of fixture.cases){
  const checked=checkProgram(parseRust(entry.source,options));
  assert.deepEqual(checked.declarations.get(entry.functionName).sourceSignature,entry.expected,entry.id);
  const archived=LinkNetwork.fromLino(entry.archive),decoded=decodeTypedProgramLinks(archived,entry.root);
  assert.deepEqual(decoded.declarations.get(entry.functionName).sourceSignature,entry.expected,entry.id);
  assert.deepEqual(output(emitTypedProgramLinks(archived,entry.root,'JavaScript').code),entry.output,entry.id);
  assert.ok(archived.links().every(link=>!link.metadata().definition?.includes(entry.source)),entry.id);
 }
});

test('default Rust headers remain facet-free and shadowed type owners preserve precise refusals',()=>{
 for(const entry of fixture.cases){
  const checked=checkProgram(parseRust(entry.source));
  for(const item of checked.declarations.values())assert.equal(Object.hasOwn(item,'sourceSignature'),false,entry.id);
 }
 assert.equal(fixture.invalid.length,7);
 for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,options)),error=>{
  assert.equal(error.kind,entry.kind,entry.id);assert.equal(error.reason,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;
 });
});

test('ten malformed owned header graph edits refuse both targets after source-free reload',()=>{
 assert.equal(fixture.malformed.length,10);
 for(const entry of fixture.malformed){
  const network=LinkNetwork.fromLino(entry.archive);
  for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(network,entry.root,target),undefined,entry.id+': '+target);
 }
 const checked=checkProgram(parseRust(fixture.cases[0].source,options));
 checked.items[0].sourceSignature.parameters[0].type.unrecognizedSourceFact=true;
 assert.throws(()=>encodeTypedProgramLinks(checked),/unrecognized or missing fields/u);
});

test('body meaning edits change execution while captured borrowed source header facts remain unchanged',()=>{
 const entry=fixture.cases.find(value=>value.id==='borrowed-float');
 assert.deepEqual(output(emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,'JavaScript').code),['true']);
 const edited=fixture.valueEdit,network=LinkNetwork.fromLino(edited.archive);
 assert.deepEqual(decodeTypedProgramLinks(network,edited.root).declarations.get('keep').sourceSignature,edited.expected);
 assert.deepEqual(output(emitTypedProgramLinks(network,edited.root,'JavaScript').code),['false']);
 assert.equal(edited.output[0],'false');
 const emitted=emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,'Rust').code;
 assert.match(emitted,/value: f64/u,'current target ABI remains the existing value projection');
 assert.deepEqual(entry.expected.parameters[0].wrappers,['reference'],'capturing a source qualifier does not assert that target emission retains it');
});

test('independently selected source models and caller provenance retain their actual ownership',()=>{
 const source='fn values(flag:bool)->Vec<f64>{if flag{return vec![1.0];}vec![2.0]}fn main(){println!("{}",values(true).len() as f64==1.0);}';
 const selected={rustSourceSignatureModel:fixture.sourceSignatureModel,rustCompletionModel:'closed guarded function returns 1',rustArrayResultModel:'immutable fresh scalar vector results 1',provenance:{project:'selected header'}};
 const archive=createTypedProgramLinks(source,'Rust',selected),network=LinkNetwork.fromLino(archive.network.toLino());
 const provenance=field(network,archive.root,'provenance'),profile=field(network,provenance,'source profile');
 for(const [key,label] of [['rustSourceSignatureModel','rust source signature model'],['rustCompletionModel','rust completion model'],['rustArrayResultModel','rust array result model']])assert.equal(network.link(field(network,profile,label)).metadata().definition,selected[key]);
 assert.equal(network.link(field(network,field(network,provenance,'user provenance'),'project')).metadata().definition,'selected header');
 assert.deepEqual(output(emitTypedProgramLinks(network,archive.root,'JavaScript').code),['true']);
 assert.throws(()=>parseRust('fn main(){}',{sourceSignatureModel:'unknown'}),/Rust source signature model/u);
 assert.throws(()=>parseRust('fn main(){}',{arrayResultModel:'unknown',sourceSignatureModel:'unknown'}),/Rust array result model/u);
 assert.throws(()=>parseRust('fn main(){}',{completionModel:'unknown',sourceSignatureModel:'unknown'}),/Rust completion model/u);
 assert.throws(()=>createTypedProgramLinks('fn main(){}','Rust',{rustSourceSignatureModel:'unknown'}),/Rust source signature model/u);
 assert.throws(()=>createTypedProgramLinks('function keep(value){return value;}','JavaScript',{rustSourceSignatureModel:fixture.sourceSignatureModel}),/requires Rust source/u);
});

test('source signature capture refuses effect-threaded header replacement instead of carrying stale positions',()=>{
 const source='fn keep(value:bool)->bool{if value{true}else{panic!("message")}}fn main(){}';
 const checked=checkProgram(parseRust(source,options));
 assert.equal(checked.items[0].sourceSignature.parameters.length,1);
 assert.throws(()=>threadOutput(checked),/effect threading changes the captured source header/u);
 assert.doesNotThrow(()=>threadOutput(checkProgram(parseRust(source))),'legacy output rewriting is preserved');
});


test('anchored type namespace lookup never falls through filtered value declarations',()=>{
 const origins=new RustSourceOrigins(new Map([['f64',{k:'data'}],['nested.f64',{k:'fn'}]]));
 const kinds=RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds;
 assert.equal(origins.lookup(['f64'],['nested'],kinds).fullName,'f64');
 assert.equal(origins.lookup(['self','f64'],['nested'],kinds),null);
 assert.equal(origins.lookup(['super','f64'],['nested','deep'],kinds),null);
 assert.equal(origins.lookup(['crate','f64'],['nested'],kinds).fullName,'f64');
 assert.equal(origins.lookup(['f64'],['nested']).fullName,'nested.f64');
});
