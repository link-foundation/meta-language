// Actual library declarations plus finite authored semantic controls.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {restoreCiLintSource} from './ci-39496927203-lint-source-history.mjs';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';
import {createTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeFormalFunctionDefinition} from '../src/formal-function-links.js';
import {encodeFormalConceptDefinition,readFormalConceptDefinition,proveFormalConceptEquivalence} from '../src/formal-concept-definition.js';

/** Prepare source-free Boolean graphs; expected truth values are authored inputs. */
export function generateSharedBooleanFunctionFixtures({check=false}={}) {
  const root=new URL('../../',import.meta.url),cases=[],equivalent=[];
  const descriptions=[];
  const actual=[];
  for(const [file,language,declaration]of [
    ['js/src/typescript-erasure-rules.js','JavaScript','eraseTypeScriptNamedImport'],
    ['rust/src/self_translation/typescript_erasure_rules.rs','Rust','erase_type_script_named_import'],
  ]){
    const source=restoreCiLintSource(readFileSync(new URL(file,root),'utf8'),file),bytes=Buffer.from(source);
    const parsed=parseProgrammingLanguage(source,language);
    assert.equal(parsed.tree.flags.hasError,false);
    const owners=parsed.tree.children.filter(owner=>{
      const node=owner.term==='export_statement'?owner.children.find(node=>node.field==='declaration'):owner;
      const name=node?.children.find(node=>node.field==='name');
      return name&&bytes.subarray(name.span.byteRange.start,name.span.byteRange.end).toString()===declaration;
    });
    assert.equal(owners.length,1,'one actual complete owned declaration');
    const owner=owners[0],end=owner.span.byteRange.end;
    let start=owner.span.byteRange.start;
    if(language==='JavaScript'){
      const prefix=bytes.subarray(0,start).toString(),closing=prefix.lastIndexOf('*/');
      assert.ok(closing>=0&&prefix.slice(closing+2).trim()==='');
      const opening=prefix.lastIndexOf('/**',closing);assert.ok(opening>=0);
      start=Buffer.byteLength(prefix.slice(0,opening));
    }
    const selected=bytes.subarray(start,end).toString();
    const harness=language==='Rust'?'\nfn main(){}':'';
    actual.push({language,declaration,source:selected+harness,
      provenance:{file,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),
        completeSource:source,completeConcreteSyntax:structuredClone(parsed.tree),selected:{start,end,source:selected},
        harness,boundary:'Closed callable selection from the actual owned declaration. The Rust empty main is an explicit checking harness, not original library source. Complete module syntax is provenance; this is no whole-module behavioral equivalence claim.'}});
  }
  descriptions.push({sources:actual,inputs:[
    [false,false,false],[false,false,true],[false,true,false],[false,true,true],
    [true,false,false],[true,false,true],[true,true,false],[true,true,true],
  ],expected:[false,false,false,true,true,true,true,true]});
  descriptions.push({sources:[
    {language:'JavaScript',declaration:'negate',source:'/** @param {boolean} value @returns {boolean} */ function negate(value){return !value;}'},
    {language:'Rust',declaration:'invert',source:'fn invert(argument:bool)->bool{!argument} fn main(){}'},
  ],inputs:[[false],[true]],expected:[true,false]});
  descriptions.push({sources:[
    {language:'JavaScript',declaration:'match',source:'/** @param {boolean} a @param {boolean} b @returns {boolean} */ function match(a,b){return a===b;}'},
    {language:'Rust',declaration:'same',source:'fn same(left:bool,right:bool)->bool{left==right} fn main(){}'},
  ],inputs:[[false,false],[false,true],[true,false],[true,true]],expected:[true,false,false,true]});
  descriptions.push({sources:[
    {language:'JavaScript',declaration:'toggle',source:'/** @param {boolean} flag @returns {boolean} */ function toggle(flag){if(flag)return !toggle(false);return false;}'},
    {language:'Rust',declaration:'switch',source:'fn switch(value:bool)->bool{if value{!switch(false)}else{false}} fn main(){}'},
  ],inputs:[[false],[true]],expected:[false,true]});
  descriptions.push({sources:[
    {language:'JavaScript',declaration:'choose',source:'/** @returns {boolean} */ function recur(){return recur();}/** @param {boolean} flag @returns {boolean} */ function choose(flag){return flag||recur();}'},
    {language:'Rust',declaration:'pick',source:'fn again()->bool{again()} fn pick(value:bool)->bool{value||again()} fn main(){}'},
  ],inputs:[[true]],expected:[true]});
  descriptions.push({sources:[
    {language:'JavaScript',declaration:'choose',source:'/** @returns {boolean} */ function recur(){return recur();}/** @param {boolean} flag @returns {boolean} */ function choose(flag){return flag&&recur();}'},
    {language:'Rust',declaration:'pick',source:'fn again()->bool{again()} fn pick(value:bool)->bool{value&&again()} fn main(){}'},
  ],inputs:[[false]],expected:[false]});
  for(const description of descriptions){
    const pair=[];
    for(const row of description.sources){
      const input=createTypedProgramLinks(row.source,row.language,{
        referenceSemantics:false,...(row.language==='Rust'?{rustSourceSignatureModel:'Rust source function signature 1'}:{}),
        ...(row.provenance?{provenance:row.provenance}:{}),
      });
      const conservative=encodeFormalFunctionDefinition(input,row.declaration);
      const typedRoot=conservative.network.link(conservative.root).references()[0];
      const definition=encodeFormalConceptDefinition({network:conservative.network,root:typedRoot},{model:'closed boolean function value'});
      pair.push(cases.length);
      cases.push({...row,inputs:description.inputs,expectedValues:description.expected,
        programRoot:input.root.asU64(),
        links:definition.network.toLino(),root:definition.root.asU64(),expected:readFormalConceptDefinition(definition),
        conservativeRoot:conservative.root.asU64()});
    }
    assert.ok(proveFormalConceptEquivalence(cases[pair[0]],cases[pair[1]]));equivalent.push(pair);
  }
  const value={schemaVersion:1,model:'closed boolean function value',cases,equivalent,
    boundary:'Prepared JavaScript graph inputs and authored Boolean truth controls. Complete actual library owners retain their source and concrete syntax. The callable model excludes public namespace spellings while the original model retains them. These prepared graphs are neither native execution evidence nor proof of whole-module, untyped JavaScript or unrestricted language equivalence.'};
  const file=new URL('parity/fixtures/shared-boolean-function-definitions.lino',root);
  if(check)checkProjectData(file,value);else writeProjectData(file,value);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const value=generateSharedBooleanFunctionFixtures({check:process.argv.includes('--check')});
  console.log('Prepared '+value.cases.length+' Boolean definitions; truth values remain separately authored.');
}
