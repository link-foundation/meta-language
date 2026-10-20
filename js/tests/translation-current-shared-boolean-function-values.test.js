import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateCurrentSharedBooleanFunctionFixtures} from '../scripts/generate-current-shared-boolean-function-fixtures.mjs';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeFormalFunctionDeclarationValue} from '../src/formal-function-links.js';
import {readFormalConceptDefinition,proveFormalConceptEquivalence,verifyFormalConceptEquivalence} from '../src/formal-concept-definition.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/shared-boolean-function-definitions-current.lino',import.meta.url));
const root=new URL('../../',import.meta.url);

test('current Boolean declaration values keep actual complete source trees original identities and const proofs through both archives',()=>{
  generateCurrentSharedBooleanFunctionFixtures({check:true});assert.equal(fixture.cases.length,2);
  assert.equal(fixture.model,'closed boolean function declaration value');
  const old=readFileSync(new URL(fixture.historical.file,root));
  assert.equal(createHash('sha256').update(old).digest('hex'),fixture.historical.sha256);
  for(const row of fixture.cases){
    const source=readFileSync(new URL(row.provenance.file,root),'utf8');
    assert.equal(source,row.provenance.completeSource,'current provenance always uses the actual source');
    // The generator stores the complete data projection, without live CST class prototypes.
    assert.deepEqual(structuredClone(parseProgrammingLanguage(source,row.language).tree),row.provenance.completeConcreteSyntax);
    assert.equal(Buffer.from(source).subarray(row.provenance.selected.start,row.provenance.selected.end).toString(),row.provenance.selected.source);
    assert.equal(row.source,row.provenance.selected.source+row.provenance.harness);
    const input=createTypedProgramLinks(row.source,row.language,{referenceSemantics:false,...row.sourceProfile,provenance:row.provenance});
    const before=input.network.toLino(),original=decodeTypedProgramLinks(input.network,input.root);
    const definition=encodeFormalFunctionDeclarationValue(input,row.declaration);
    assert.equal(input.network.toLino(),before);
    assert.deepEqual(readFormalConceptDefinition(definition),row.expected);
    for(const network of [LinkNetwork.fromLino(row.links),decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(row.links)))]){
      assert.deepEqual(readFormalConceptDefinition({network,root:row.root}),row.expected);
      assert.equal(network.link(row.root).references()[0].asU64(),row.programRoot);
      const fields=network.fieldRelations().filter(field=>field.parent.asU64()===row.root&&field.label==='selected function declaration');
      assert.equal(fields.length,1);assert.equal(fields[0].child.asU64(),row.declarationId);
      const restored=decodeTypedProgramLinks(network,row.programRoot);assert.deepEqual(restored,original);
      assert.equal(restored.declarations.has('selected_function_definition'),false,'the direct basis invents no executable wrapper');
      assert.equal(Object.hasOwn(restored.declarations.get(row.declaration),'sourceConstFunction'),row.language==='Rust');
      if(row.language==='Rust')assert.deepEqual(restored.declarations.get(row.declaration).sourceConstFunction,row.sourceConstFunction);
      for(const target of ['JavaScript','Rust'])assert.equal(emitTypedProgramLinks(network,row.programRoot,target).code,row.emissions[target]);
    }
    if(row.language==='Rust'){
      assert.throws(()=>createTypedProgramLinks(row.source,'Rust',{referenceSemantics:false,rustSourceSignatureModel:'Rust source function signature 1'}),/const/u,
        'the default legacy parser still refuses const functions');
      const changed=decodeTypedProgramLinks(input.network,input.root);
      changed.declarations.get(row.declaration).sourceConstFunction.sourceName='counterfeit';
      assert.throws(()=>encodeTypedProgramLinks(changed),/const function proof/u);
    }
  }
  const one=fixture.cases[0],two=fixture.cases[1];
  const proof=proveFormalConceptEquivalence(one,two);assert.ok(proof);
  for(const network of [LinkNetwork.fromLino(proof.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(proof.network))]){
    assert.equal(verifyFormalConceptEquivalence(one,two,{network,root:proof.root}),true);
  }
});

test('both current direct source owners and emitted archived programs obey the same eight authored Boolean truths',async()=>{
  const expected=[false,false,false,true,true,true,true,true];
  const inputs=[[false,false,false],[false,false,true],[false,true,false],[false,true,true],
    [true,false,false],[true,false,true],[true,true,false],[true,true,true]];
  for(const row of fixture.cases){
    assert.deepEqual(row.inputs,inputs);assert.deepEqual(row.expectedValues,expected);
    const original=row.language==='JavaScript'?(await import('data:text/javascript;base64,'+Buffer.from(row.source+'\nexport const subject='+row.declaration+';').toString('base64'))).subject:null;
    const network=LinkNetwork.fromLino(row.links);
    for(const graph of [LinkNetwork.fromLino(row.links),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
      const code=emitTypedProgramLinks(graph,row.programRoot,'JavaScript').code;
      for(const [index,args]of inputs.entries()){
        assert.equal(vm.runInNewContext(code+'\n'+row.declaration+'(...args);',{args,process:{argv:[]}},{timeout:1000}),expected[index]);
        if(original)assert.equal(original(...args),expected[index]);
      }
    }
  }
});
