// Actual current owned declarations, kept separate from the historical wrapper corpus.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';
import {tokenize} from '../src/translation/lexer.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {encodeFormalFunctionDeclarationValue} from '../src/formal-function-links.js';
import {readFormalConceptDefinition,proveFormalConceptEquivalence} from '../src/formal-concept-definition.js';

/** Read the actual checked map entry identity without creating a call wrapper. */
function currentBooleanDeclarationId(network,root,name){
  const body=network.link(root).references()[0];
  const maps=network.fieldRelations().filter(row=>row.parent.equals(body)&&row.label===TYPED_PROGRAM_LINKS_SCHEMA.fields.declarations);
  assert.equal(maps.length,1);
  const entries=network.link(maps[0].child).references().filter(id=>{
    const pair=network.link(id).references();return network.link(pair[0]).metadata().definition===name;
  });
  assert.equal(entries.length,1,'one actual named checked declaration');
  return network.link(entries[0]).references()[1];
}

/** Capture both complete current modules/CSTs and only their actual library owners. */
function currentBooleanLibrarySources(){
  const root=new URL('../../',import.meta.url),result=[];
  for(const [file,language,declaration]of [
    ['js/src/typescript-erasure-rules.js','JavaScript','eraseTypeScriptNamedImport'],
    ['rust/src/self_translation/typescript_erasure_rules.rs','Rust','erase_type_script_named_import'],
  ]){
    const source=readFileSync(new URL(file,root),'utf8'),bytes=Buffer.from(source);
    const parsed=parseProgrammingLanguage(source,language);assert.equal(parsed.tree.flags.hasError,false);
    const owners=parsed.tree.children.filter(owner=>{
      const node=owner.term==='export_statement'?owner.children.find(node=>node.field==='declaration'):owner;
      const name=node?.children.find(node=>node.field==='name');
      return name&&bytes.subarray(name.span.byteRange.start,name.span.byteRange.end).toString()===declaration;
    });
    assert.equal(owners.length,1,'one actual complete current owned declaration');
    const owner=owners[0],end=owner.span.byteRange.end;let start=owner.span.byteRange.start;
    if(language==='JavaScript'){
      const prefix=bytes.subarray(0,start).toString(),closing=prefix.lastIndexOf('*/');
      assert.ok(closing>=0&&prefix.slice(closing+2).trim()==='');
      const opening=prefix.lastIndexOf('/**',closing);assert.ok(opening>=0);start=Buffer.byteLength(prefix.slice(0,opening));
    }
    const selected=bytes.subarray(start,end).toString(),harness=language==='Rust'?'\nfn main(){}':'';
    const sourceProfile=language==='Rust'?{rustSourceSignatureModel:'Rust source function signature 1',rustConstFunctionModel:'closed pure Rust const predicates 1'}:{};
    if(language==='Rust'){
      const tokens=tokenize(selected,'Rust').tokens,fn=tokens.findIndex(token=>token.value==='fn');
      assert.ok(fn>0&&tokens[fn-1].value==='const'&&tokens[fn+1].value===declaration,
        'the existing const profile is selected only for the actual current const owner');
    }
    result.push({language,declaration,source:selected+harness,sourceProfile,
      provenance:{file,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),
        completeSource:source,completeConcreteSyntax:structuredClone(parsed.tree),selected:{start,end,source:selected},harness,
        boundary:'Actual current complete module and exact selected owner. The explicit empty Rust main is a checking harness. This source is not replaced by its historical view or rewritten to remove const.'}});
  }
  return result;
}

/** Generate only the two current owners with the same eight independent authored truths. */
export function generateCurrentSharedBooleanFunctionFixtures({check=false}={}){
  const cases=[],inputs=[
    [false,false,false],[false,false,true],[false,true,false],[false,true,true],
    [true,false,false],[true,false,true],[true,true,false],[true,true,true],
  ],expectedValues=[false,false,false,true,true,true,true,true];
  for(const row of currentBooleanLibrarySources()){
    const input=createTypedProgramLinks(row.source,row.language,{referenceSemantics:false,...row.sourceProfile,provenance:row.provenance});
    const original=input.network.toLino(),declarationId=currentBooleanDeclarationId(input.network,input.root,row.declaration);
    const checked=decodeTypedProgramLinks(input.network,input.root),owner=checked.declarations.get(row.declaration);
    const definition=encodeFormalFunctionDeclarationValue(input,row.declaration);
    assert.equal(input.network.toLino(),original,'selection never mutates the original checked graph');
    assert.ok(definition.network.link(definition.root).references()[0].equals(input.root),'the original typed root identity is retained');
    const fields=definition.network.fieldRelations().filter(field=>field.parent.equals(definition.root)&&field.label==='selected function declaration');
    assert.equal(fields.length,1);assert.ok(fields[0].child.equals(declarationId),'the selected original declaration ID is unchanged');
    assert.deepEqual(decodeTypedProgramLinks(definition.network,input.root),checked,'all original types owners proofs and presence survive');
    assert.equal(Object.hasOwn(owner,'sourceConstFunction'),row.language==='Rust');
    cases.push({...row,inputs,expectedValues,programRoot:input.root.asU64(),declarationId:declarationId.asU64(),
      ...(row.language==='Rust'?{sourceConstFunction:structuredClone(owner.sourceConstFunction)}:{}),
      links:definition.network.toLino(),root:definition.root.asU64(),expected:readFormalConceptDefinition(definition),
      emissions:{JavaScript:emitTypedProgramLinks(input.network,input.root,'JavaScript').code,Rust:emitTypedProgramLinks(input.network,input.root,'Rust').code}});
  }
  assert.equal(cases.length,2);assert.ok(proveFormalConceptEquivalence(cases[0],cases[1]));
  const value={schemaVersion:1,model:'closed boolean function declaration value',cases,equivalent:[[0,1]],
    historical:{file:'parity/fixtures/shared-boolean-function-definitions.lino',sha256:'01184f0076da3905c869a7a339c5b46d985f068546e199b79e468aa5219af092',cases:12},
    boundary:'Two actual current source owners with the unchanged exhaustive eight Boolean truth controls. The direct declaration basis preserves the complete checked program, original declaration identities and const source proof. The historical twelve-case wrapper corpus stays separate and unchanged. These generated graphs are not native execution or unrestricted language equivalence evidence.'};
  const root=new URL('../../',import.meta.url),old=readFileSync(new URL(value.historical.file,root));
  assert.equal(createHash('sha256').update(old).digest('hex'),value.historical.sha256,'the entire historical corpus stays byte exact');
  const file=new URL('parity/fixtures/shared-boolean-function-definitions-current.lino',root);
  if(check)checkProjectData(file,value);else writeProjectData(file,value);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateCurrentSharedBooleanFunctionFixtures({check:process.argv.includes('--check')});
}
