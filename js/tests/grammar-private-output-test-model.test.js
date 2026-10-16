import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {buildPrivateOutputTestModel,readRustSourceItems} from '../scripts/generate-private-output-test-model.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseNative} from '../src/native-grammar-parser.js';
import {tokenize} from '../src/translation/lexer.js';

const policy=readProjectData(new URL('../../parity/self-translation/private-output-test-model.lino',import.meta.url));
const originalPolicy=structuredClone(policy),model=buildPrivateOutputTestModel(undefined,policy);
const modelBytes=Buffer.from(model.source),tree=parseNative('native-rust',model.source);
const functions=[];
const walk=node=>{if(node.term==='function_item')functions.push(node);for(const child of node.children)walk(child.node);};walk(tree);
const text=(bytes,node)=>bytes.subarray(node.start,node.end).toString('utf8');
const functionName=node=>text(modelBytes,node.children.find(child=>child.field==='name').node);

test('private native model retains complete original test bodies and production output comparison',()=>{
  assert.equal(tree.term,'source_file');assert.equal(tree.hasError,false);assert.ok(modelBytes.length<131072);
  const cases=model.receipt.inputs.find(row=>row.testNames);assert.ok(cases);
  assert.deepEqual(cases.testNames,policy.modules.find(module=>module.cases).cases.originalTests);
  for(const [file,names]of [[cases.file,cases.testNames],['rust/src/grammar/feature_runtime/walk.rs',['same_output']]]){
    const bytes=readFileSync(new URL('../../'+file,import.meta.url)),original=readRustSourceItems(bytes);
    for(const name of names){
      const source=original.items.filter(item=>item.kind==='function_item'&&item.name===name);assert.equal(source.length,1);
      const generated=functions.filter(node=>functionName(node)===name);assert.equal(generated.length,1);
      assert.deepEqual(tokenize(text(modelBytes,generated[0]),'Rust').tokens.map(token=>[token.kind,token.value]),tokenize(text(bytes,source[0].node),'Rust').tokens.map(token=>[token.kind,token.value]),name);
    }
  }
  const extraSources=policy.modules.find(module=>module.cases).cases.extraCases;
  assert.equal(extraSources.length,1);
  const extras=model.receipt.inputs.filter(row=>extraSources.includes(row.file));
  assert.equal(extras.length,extraSources.length);assert.equal(extras.flatMap(row=>row.testNames).length,3);
  assert.equal((model.source.match(/#\[test\]/gu)??[]).length,6);
});

test('private native model retains every selected declaration field and source provenance',()=>{
  assert.deepEqual(policy,originalPolicy,'generation leaves its policy unchanged');
  assert.ok(model.receipt.compileTimeBindings>=35);
  for(const section of model.receipt.sections.filter(row=>row.file)){
    const bytes=readFileSync(new URL('../../'+section.file,import.meta.url));
    assert.equal(createHash('sha256').update(bytes.subarray(section.start,section.end)).digest('hex'),section.sha256);
    assert.ok(section.declarations.length>0);
  }
  assert.ok(model.source.includes('&value.projection_anonymous'));
  assert.ok(model.source.includes('&value.program'));
  assert.ok(!model.source.includes('#[allow(dead_code)]'));
  assert.ok(!model.source.includes('#[cfg(test)]'));
  const walking=model.outputs.find(output=>output.file.endsWith('/walk.rs')).source;
  const original=readRustSourceItems(readFileSync(new URL('../../rust/src/grammar/feature_runtime/walk.rs',import.meta.url)));
  const privatePlan=original.items.find(item=>item.kind==='struct_item'&&item.name==='GrammarTreeEqualityPlan');
  assert.ok(privatePlan);
  assert.ok(!privatePlan.node.children.some(({node})=>node.term==='visibility_modifier'));
  assert.match(walking,/let _ = \|value: &self::GrammarTreeEqualityPlan\|/u);
  assert.match(walking,/&value\.fields, &value\.recursion/u);
  assert.doesNotMatch(model.outputs[0].source,/walk::GrammarTreeEqualityPlan/u,'private bindings stay in the original owning module');
});

test('private native model refuses malformed concrete syntax and missing declaration selectors',()=>{
  assert.throws(()=>readRustSourceItems(Buffer.from('struct {')),/complete native CST|source_file/u);
  const changed=structuredClone(policy);changed.modules[1].sections[0].first.name='AbsentOwnedDeclaration';
  assert.throws(()=>buildPrivateOutputTestModel(undefined,changed),/one declaration/u);
});

test('private native model matches its checked source and full concrete syntax receipt',()=>{
  assert.equal(model.outputs[0].file,policy.output);
  assert.equal(model.outputs.length,policy.modules.length+1);
  for(const output of model.outputs){
    assert.equal(readFileSync(new URL('../../'+output.file,import.meta.url),'utf8'),output.source);
    assert.ok(output.source.split('\n').length<1000,'every complete module fits the source limit');
    assert.equal(parseNative('native-rust',output.source).hasError,false);
  }
  assert.deepEqual(readProjectData(new URL('../../'+policy.receipt,import.meta.url)),model.receipt);
});
