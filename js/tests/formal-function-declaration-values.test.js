import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {generateBooleanDeclarationValueRules} from '../scripts/generate-boolean-declaration-value-rules.mjs';
import {generateSharedBooleanFunctionModel} from '../scripts/generate-shared-boolean-function-model.mjs';
import {acceptBooleanDeclarationValueContext,acceptBooleanDeclarationValueEntry,acceptBooleanDeclarationValueSelector} from '../src/boolean-declaration-value-rules.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {encodeFormalFunctionDefinition,encodeFormalFunctionDeclarationValue} from '../src/formal-function-links.js';
import {encodeFormalConceptDefinition,readFormalConceptDefinition,proveFormalConceptEquivalence,verifyFormalConceptEquivalence} from '../src/formal-concept-definition.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const model='closed boolean function declaration value';
const options={rustConstFunctionModel:'closed pure Rust const predicates 1'};
const javascript='/** @param {boolean} a @param {boolean} b @param {boolean} c @returns {boolean} */ function decide(a,b,c){return a||b&&c;}';
const rust='const fn choose(a:bool,b:bool,c:bool)->bool{a||b&&c} fn main(){}';
const args=[[false,false,false],[false,false,true],[false,true,false],[false,true,true],[true,false,false],[true,false,true],[true,true,false],[true,true,true]];
const expected=[false,false,false,true,true,true,true,true];
const packets=()=>[createTypedProgramLinks(javascript,'JavaScript'),createTypedProgramLinks(rust,'Rust',options)];
const fields=(network,parent,label)=>network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===label);
const reloads=network=>[LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
const selected=(value)=>fields(value.network,value.root,'selected function declaration')[0];
const clone=value=>({network:LinkNetwork.fromLino(value.network.toLino()),root:value.root});
function owned(network,root,name){
  const body=network.link(root).references()[0];
  const directory=fields(network,body,TYPED_PROGRAM_LINKS_SCHEMA.fields.declarations)[0].child;
  const pair=network.link(directory).references().find(id=>network.link(network.link(id).references()[0]).metadata().definition===name);
  return network.link(pair).references()[1];
}

test('direct Boolean entry decisions retain exact source context original ownership and one selector',()=>{
  const contexts=[['JavaScript',false,true],['JavaScript',true,false],['Rust',false,false],['Rust',true,true],['Lean',false,false],['',true,false]];
  for(const [language,constProof,accepted]of contexts)assert.equal(acceptBooleanDeclarationValueContext(language,constProof),accepted);
  for(const kind of ['fn','data','module',''])for(const nested of [false,true])for(const generated of [false,true])
    assert.equal(acceptBooleanDeclarationValueEntry(kind,nested,generated),kind==='fn'&&!nested&&!generated);
  for(const one of [false,true])for(const owned of [false,true])assert.equal(acceptBooleanDeclarationValueSelector(one,owned),one&&owned);
});

test('original const proof and typed root remain unchanged while both honest codecs select the actual declaration',()=>{
  generateSharedBooleanFunctionModel({check:true});
  const values=[];
  for(const [index,packet]of packets().entries()){
    const name=index?'choose':'decide',before=packet.network.toLino(),id=owned(packet.network,packet.root,name);
    const proof=index?decodeTypedProgramLinks(packet.network,packet.root).declarations.get(name).sourceConstFunction:null;
    const value=encodeFormalFunctionDeclarationValue(packet,id,{provenance:{fixture:'direct original entry'}});
    assert.equal(packet.network.toLino(),before);
    assert.equal(value.network.link(value.root).references()[0].asU64(),packet.root.asU64());
    assert.equal(selected(value).child.asU64(),id.asU64());
    for(const network of reloads(value.network)){
      const restored={network,root:value.root};assert.equal(readFormalConceptDefinition(restored).model,model);
      assert.equal(network.link(value.root).references()[0].asU64(),packet.root.asU64());
      assert.equal(selected(restored).child.asU64(),id.asU64());
      const decoded=decodeTypedProgramLinks(network,packet.root);
      assert.equal(decoded.declarations.has('selected_function_definition'),false);
      if(index)assert.deepEqual(decoded.declarations.get(name).sourceConstFunction,proof);
      const code=emitTypedProgramLinks(network,packet.root,'JavaScript').code;
      for(const [position,input]of args.entries())assert.equal(vm.runInNewContext(code+'\n'+name+'(...input);',{input,process:{argv:[]}},{timeout:1000}),expected[position]);
      if(index)assert.match(emitTypedProgramLinks(network,packet.root,'Rust').code,/const fn choose\(/u);
    }
    values.push(value);
    const conservative=encodeFormalConceptDefinition(packet);
    assert.equal(readFormalConceptDefinition(conservative).model,'checked typed program');
  }
  const proof=proveFormalConceptEquivalence(values[0],values[1]);assert.ok(proof);
  assert.equal(verifyFormalConceptEquivalence(values[0],values[1],proof),true);
  assert.equal(proveFormalConceptEquivalence(encodeFormalConceptDefinition(packets()[0]),encodeFormalConceptDefinition(packets()[1])),null);
});

test('entry edges cannot substitute foreign parameter type or duplicate identities and do not legalize old const wrappers',()=>{
  const packet=packets()[1],value=encodeFormalFunctionDeclarationValue(packet,'choose');
  assert.throws(()=>encodeFormalFunctionDefinition(packet,'choose'),/Rust const function proof/u,'the old wrapper API still cannot reassign an original const proof');
  assert.throws(()=>encodeFormalConceptDefinition(packet,{model}),/primitive|source interpretation|typed|selector/u,'the new basis has no unselected constructor shortcut');
  assert.throws(()=>encodeFormalFunctionDeclarationValue(packet,packet.root),/owned function declaration/u);
  const functionId=selected(value).child;
  const parameters=fields(value.network,functionId,TYPED_PROGRAM_LINKS_SCHEMA.fields.params)[0].child;
  const firstParameter=value.network.link(parameters).references()[0];
  const parameterType=fields(value.network,firstParameter,(TYPED_PROGRAM_LINKS_SCHEMA.fields.type ?? 'type'))[0].child;
  for(const target of [packet.root,parameters,firstParameter,parameterType,value.root]){
    const changed=clone(value),edge=selected(changed),references=edge.link.references();references[2]=target;edge.link.setReferences(references);
    for(const network of reloads(changed.network))assert.throws(()=>readFormalConceptDefinition({network,root:changed.root}),/owned declaration|selected identity/u);
  }
  const missing=clone(value);missing.network.deleteLink(selected(missing).link.id());assert.throws(()=>readFormalConceptDefinition(missing),/exactly one/u);
  const duplicate=clone(value);duplicate.network.insertFieldRelation(duplicate.root,'selected function declaration',functionId);
  assert.throws(()=>readFormalConceptDefinition(duplicate),/exactly one|duplicate/u);
  const moved=clone(value),edge=selected(moved),references=edge.link.references();references[0]=functionId;edge.link.setReferences(references);
  // Moving the selector inside its selected body also creates a structural cycle; either validator must refuse it.
  assert.throws(()=>readFormalConceptDefinition(moved),/exactly one|structural cycle/u);
});

test('the new basis retains Boolean-only context refusals and strict original qualification before any exclusion',()=>{
  for(const [language,source,name,profile]of [
    ['Rust','fn choose(a:bool,b:bool,c:bool)->bool{a||b&&c}fn main(){}','choose',{}],
    ['JavaScript','/** @param {number} value @returns {boolean} */ function numeric(value){return value>0;}','numeric',{}],
    ['JavaScript','/** @param {boolean} value @returns {boolean} */ function defaulted(value=true){return value;}','defaulted',{}],
    ['JavaScript','/** @param {boolean} value @returns {boolean} */ function effect(value){console.log(value);return value;}','effect',{}],
    ['Rust','const fn numeric(value:f64)->bool{value>0.0}fn main(){}','numeric',options],
    ['Rust','const fn first(value:bool)->bool{value}const fn second(value:bool)->bool{first(value)}fn main(){}','second',options],
  ])assert.throws(()=>encodeFormalFunctionDeclarationValue(createTypedProgramLinks(source,language,profile),name),/Boolean|const|guard|default|effect|outside|interpretation|original/u);
  const packet=packets()[1],value=encodeFormalFunctionDeclarationValue(packet,'choose');
  const qualification=fields(value.network,selected(value).child,TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceConstFunction)[0];
  assert.ok(qualification);
  const unqualified=clone(value);
  unqualified.network.deleteLink(fields(unqualified.network,selected(unqualified).child,TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceConstFunction)[0].link.id());
  assert.throws(()=>readFormalConceptDefinition(unqualified),/original Rust const proof|source proof|const/u);
  const changed=clone(value),proof=fields(changed.network,qualification.child,TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceName)[0];
  assert.ok(proof);changed.network.link(proof.child).setMetadata(changed.network.link(proof.child).metadata().withDefinition('different_original_owner'));
  for(const network of reloads(changed.network))assert.throws(()=>readFormalConceptDefinition({network,root:changed.root}),/const source qualification|source name|source proof|source owner|qualification/u);
  const different=createTypedProgramLinks(javascript.replace('a||b&&c','a&&b&&c'),'JavaScript');
  assert.equal(proveFormalConceptEquivalence(value,encodeFormalFunctionDeclarationValue(different,'decide')),null);
});

test('the complete new decision source and all actual generated owners replay through both checked archives',async()=>{
  const result=await generateBooleanDeclarationValueRules({check:true,native:true});
  assert.equal(result.policy.decisions.length,3);
  assert.equal(result.artifact.inventory.length,result.artifact.checkedNamedSourceInventory.length);
  assert.equal(result.artifact.replays.length,2);
});
