import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {restoreCiLintSource} from '../scripts/ci-39496927203-lint-source-history.mjs';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateSharedBooleanFunctionModel} from '../scripts/generate-shared-boolean-function-model.mjs';
import {generateSharedBooleanFunctionFixtures} from '../scripts/generate-shared-boolean-function-fixtures.mjs';
import {createTypedProgramLinks,emitTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {encodeFormalFunctionDefinition} from '../src/formal-function-links.js';
import {encodeFormalConceptDefinition,readFormalConceptDefinition,proveFormalConceptEquivalence,verifyFormalConceptEquivalence} from '../src/formal-concept-definition.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const model='closed boolean function value';
const fixture=readProjectData(new URL('../../parity/fixtures/shared-boolean-function-definitions.lino',import.meta.url));
const archive=row=>({network:LinkNetwork.fromLino(row.links),root:row.root});
const select=(source,language,name,options={})=>{
  const input=createTypedProgramLinks(source,language,{referenceSemantics:false,
    ...(language==='Rust'?{rustSourceSignatureModel:'Rust source function signature 1'}:{}),...options});
  const old=encodeFormalFunctionDefinition(input,name),root=old.network.link(old.root).references()[0];
  return encodeFormalConceptDefinition({network:old.network,root},{model});
};

test('actual Boolean library owners and every prepared recursive definition retain source-free text and binary proofs',()=>{
  assert.equal(createHash('sha256').update(readFileSync(new URL('../../parity/fixtures/shared-boolean-function-definitions.lino',import.meta.url))).digest('hex'),
    '01184f0076da3905c869a7a339c5b46d985f068546e199b79e468aa5219af092','the entire original twelve-case historical corpus remains byte exact');
  generateSharedBooleanFunctionModel({check:true});generateSharedBooleanFunctionFixtures({check:true});
  for(const row of fixture.cases){
    const value=archive(row),before=value.network.toLino();
    for(const network of [LinkNetwork.fromLino(before),decodeLinkNetworkBinary(encodeLinkNetworkBinary(value.network))]){
      const restored={network,root:row.root};assert.deepEqual(readFormalConceptDefinition(restored),row.expected);
      const typedRoot=network.link(row.root).references()[0];
      assert.ok(decodeTypedProgramLinks(network,typedRoot).declarations.has(row.declaration));
    }
    assert.equal(value.network.toLino(),before,'the actual archive is never rewritten into its meaning view');
    if(row.provenance){
      assert.equal(restoreCiLintSource(readFileSync(new URL('../../'+row.provenance.file,import.meta.url),'utf8'),row.provenance.file),row.provenance.completeSource,
        'the original complete source is the certified historical view, not a claim about current source bytes');
      assert.equal(Buffer.from(row.provenance.completeSource).subarray(row.provenance.selected.start,row.provenance.selected.end).toString(),row.provenance.selected.source);
      assert.equal(row.source,row.provenance.selected.source+row.provenance.harness);
      assert.ok(row.provenance.completeConcreteSyntax.children.length>1);
    }
  }
  for(const [first,second]of fixture.equivalent){
    const one=archive(fixture.cases[first]),two=archive(fixture.cases[second]);
    const proof=proveFormalConceptEquivalence(one,two);assert.ok(proof);
    const restored={network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(proof.network)),root:proof.root};
    assert.equal(verifyFormalConceptEquivalence(one,two,restored),true);
    const old=row=>archive({links:row.links,root:row.conservativeRoot});
    assert.equal(proveFormalConceptEquivalence(old(fixture.cases[first]),old(fixture.cases[second])),null,
      'the original source-interface model retains public names and source-language premises');
  }
});

test('independent source functions and both graph interpretations obey every authored truth value and short circuit',async()=>{
  for(const row of fixture.cases){
    const value=archive(row),typedRoot=value.network.link(value.root).references()[0];
    const emitted=emitTypedProgramLinks(value.network,typedRoot,'JavaScript').code;
    const original=row.language==='JavaScript'
      ? (await import('data:text/javascript;base64,'+Buffer.from(row.source+'\nexport const subject = '+row.declaration+';').toString('base64'))).subject:null;
    for(const [index,args]of row.inputs.entries()){
      assert.equal(vm.runInNewContext(emitted+'\nselected_function_definition(...args);',{args,process:{argv:[]}},{timeout:1000}),row.expectedValues[index]);
      if(original)assert.equal(original(...args),row.expectedValues[index]);
    }
  }
});

test('different operators, parameter order, recursive target and false premises cannot share Boolean meanings',()=>{
  const first=fixture.cases[0],one=archive(first);
  const changes=[first.source.replace('typeSpecifier || comma','typeSpecifier && comma'),
    first.source.replace('typeSpecifier, comma, afterTypeSpecifier','comma, typeSpecifier, afterTypeSpecifier')];
  for(const source of changes)assert.equal(proveFormalConceptEquivalence(one,select(source,'JavaScript',first.declaration)),null);
  assert.equal(proveFormalConceptEquivalence(archive(fixture.cases[8]),archive(fixture.cases[10])),null,
    'short circuit with a recursive operand retains the selected operation');
  const forged=archive(first),basis=forged.network.link(forged.root).references()[1];
  const meaning=forged.network.link(basis).references()[0];
  const premise=forged.network.fieldRelations().find(row=>row.parent.equals(meaning)&&row.label==='interface');
  assert.ok(premise);forged.network.link(premise.child).setMetadata(forged.network.link(premise.child).metadata().withDefinition('all arbitrary contexts are equal'));
  assert.throws(()=>readFormalConceptDefinition(forged),/primitive model differs/u);
  assert.equal(proveFormalConceptEquivalence(one,forged),null);
  const changed=archive(first),typedRoot=changed.network.link(changed.root).references()[0];
  const reference=changed.network.fieldRelations().find(row=>row.label===TYPED_PROGRAM_LINKS_SCHEMA.references.declaration);
  const references=reference.link.references();references[2]=typedRoot;reference.link.setReferences(references);
  assert.throws(()=>readFormalConceptDefinition(changed));
});

test('numeric, string, borrowed, default and effectful source contracts refuse the Boolean-only basis',()=>{
  for(const [language,name,source]of [
    ['JavaScript','keep','/** @param {number} a @returns {number} */ function keep(a){return a;}'],
    ['JavaScript','keep','/** @param {string} a @returns {string} */ function keep(a){return a;}'],
    ['JavaScript','keep','/** @param {number} a @returns {boolean} */ function keep(a){return a>0;}'],
    ['JavaScript','keep','/** @param {boolean} a @returns {boolean} */ function keep(a=true){return a;}'],
    ['JavaScript','keep','/** @param {boolean} a @returns {boolean} */ function keep(a){console.log(a);return a;}'],
    ['Rust','keep','fn keep(a:&bool)->bool{*a} fn main(){}'],
  ])assert.throws(()=>select(source,language,name),/Boolean|guard|default|effect|unsupported|signature|output|interpretation/iu);
  const source='/** @param {boolean} value @returns {boolean} */ function keep(value){return value;}';
  assert.throws(()=>select(source,'JavaScript','keep',{utf16Strings:true}),/outside|interpretation/iu);
  const actual=archive(fixture.cases[0]),root=actual.network.link(actual.root).references()[0];
  assert.throws(()=>encodeFormalConceptDefinition({network:actual.network,root},{model:'closed binary floating point function'}),
    /Boolean|binary64|binary primitive/iu,'the previous numeric model keeps its original Boolean-operation boundary');
});
