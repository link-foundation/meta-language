import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {generateNativeConstructorStorage,renderNativeConstructorStorage,buildNativeConstructorStorageControls} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-native-constructor-storage.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {parseNative} from '../src/native-grammar-parser.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const root=new URL('../../',import.meta.url);
let generated;
function nativeConstructorContext() {
  generated??=generateNativeConstructorStorage({check:true});return generated;
}
function checkNativeConstructorSourceBindings(source,fields,references,byId) {
  const tokens=tokenize(source,'Rust').tokens;
  const calls=tokens.flatMap((token,index)=>{
    if(token.kind!=='macro'||token.value!=='take_child')return[];
    assert.equal(tokens[index+1].value,'(');assert.equal(tokens[index+2].value,'children');assert.equal(tokens[index+4].kind,'string');
    const reference=tokens[index+4].value,expected=byId.get(reference);assert.ok(expected);
    assert.equal(tokens[index+6].value,expected.storage.carrier);
    assert.equal(tokens[index+8]?.value==='boxed',expected.storage.boxed);
    return[reference];
  });
  assert.deepEqual(calls,references,'each actual source child is consumed exactly once in source order');
  const initializers=tokens.flatMap((token,index)=>token.kind==='identifier'&&tokens[index+1]?.value===':'&&/^field_[0-9]+$/u.test(tokens[index+2]?.value??'')?[[token.value,tokens[index+2].value]]:[]);
  assert.deepEqual(initializers,fields.map((field,index)=>[field.name,'field_'+index]),'every actual native field receives its own complete source-qualified operand');
}
function checkNativeConstructorControl(inventory,recipe) {
  const byId=new Map(inventory.operations.map(row=>[row.sourceReference,row])),pending=[recipe];let count=0;
  while(pending.length){
    const current=pending.pop();assert.ok(++count<=100000);const operation=byId.get(current.reference);assert.ok(operation);
    assert.deepEqual(Object.keys(current),['reference','choice','children','scalar']);assert.ok(Array.isArray(current.children));
    let references=[];
    if(operation.variants){const variant=operation.variants.find(row=>row.identity===current.choice);assert.ok(variant);references=(variant.fields??variant.tuple).map(row=>row.typeReference);}
    else if(operation.form==='Option'){assert.ok(['Some','None'].includes(current.choice));references=current.choice==='Some'?operation.arguments:[];}
    else{
      assert.equal(current.choice,'');
      if(operation.fields)references=operation.fields.map(row=>row.typeReference);
      else if(['named','type_item','Box','reference'].includes(operation.form))references=[operation.child??operation.arguments[0]];
      else if(operation.form==='Vec')references=current.children.map(()=>operation.arguments[0]);
      else if(operation.form==='IndexMap'){assert.equal(current.children.length%2,0);references=current.children.map((_,index)=>operation.arguments[index%2]);}
    }
    assert.deepEqual(current.children.map(row=>row.reference),references);
    if(['bool','usize','i64','String','str','json','reference'].includes(operation.form)){
      assert.ok(current.scalar);assert.deepEqual(Object.keys(current.scalar),['kind','value']);assert.equal(current.scalar.kind,operation.storage.carrier);
      if(operation.form==='bool')assert.equal(typeof current.scalar.value,'boolean');
      if(['String','str','reference'].includes(operation.form))assert.equal(typeof current.scalar.value,'string');
      if(['usize','i64'].includes(operation.form)){assert.match(current.scalar.value,/^-?(?:0|[1-9][0-9]*)$/u);const value=BigInt(current.scalar.value);assert.ok(value>= (operation.form==='i64'?-(1n<<63n):0n)&&value<(operation.form==='i64'?1n<<63n:1n<<32n));}
      if(operation.form==='reference')assert.equal(current.scalar.value,current.children[0].scalar.value);
    }else assert.equal(current.scalar,null);
    pending.push(...current.children);
  }
  return count;
}

test('all actual source declarations and every source type occurrence retain complete deterministic native storage generation',()=>{
  const result=nativeConstructorContext(),inventory=result.inventory;
  assert.equal(inventory.roots.length,9);assert.equal(inventory.operations.length,120);assert.equal(result.outputs.length,6);
  assert.equal(new Set(inventory.operations.map(row=>row.sourceReference)).size,inventory.operations.length);
  for(const output of result.outputs){const bytes=readFileSync(new URL(output.file,root));assert.equal(bytes.length,output.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),output.sha256);}
  assert.match(inventory.boundary,/does not validate a complete owned model/u);
  assert.ok(inventory.operations.some(row=>row.form==='reference'));
  assert.ok(inventory.operations.filter(row=>row.storage.carrier==='Text').length>1,'native storage sharing preserves independently qualified source types');
});

test('every native constructor conserves actual field bindings child order source qualifiers and complete variant choices',()=>{
  const {inventory,outputs}=nativeConstructorContext(),byId=new Map(inventory.operations.map(row=>[row.sourceReference,row]));
  const texts=new Map(outputs.map(row=>[row.file.split('/').at(-1),readFileSync(new URL(row.file,root),'utf8')]));
  for(const text of texts.values()){const tree=parseNative('native-rust',text);assert.equal(tree.hasError,false);assert.equal(tree.start,0);assert.equal(tree.end,Buffer.byteLength(text));}
  for(let index=0;index<inventory.operations.length;index++){
    const operation=inventory.operations[index],text=texts.get('part_'+Math.floor(index/24)+'.rs');
    const source=readSourceTextRustBlock(text,'fn','construct_'+index);
    let references=operation.fields?.map(row=>row.typeReference)??(['named','type_item','reference'].includes(operation.form)?[operation.child]:operation.arguments??[]);
    if(operation.variants)references=[];
    checkNativeConstructorSourceBindings(source,operation.fields??[],references,byId);
    for(let variantIndex=0;variantIndex<(operation.variants?.length??0);variantIndex++){
      const variant=operation.variants[variantIndex],fields=variant.fields??variant.tuple;
      assert.ok(source.includes(JSON.stringify(variant.identity)));
      const body=readSourceTextRustBlock(text,'fn','construct_variant_'+index+'_'+variantIndex);
      checkNativeConstructorSourceBindings(body,variant.fields??[],fields.map(row=>row.typeReference),byId);
      assert.ok(body.includes(operation.storage.rust+'::'+variant.name));
    }
  }
  const operation=inventory.operations.find(row=>row.fields?.length>1),index=inventory.operations.indexOf(operation),text=texts.get('part_'+Math.floor(index/24)+'.rs');
  const body=readSourceTextRustBlock(text,'fn','construct_'+index),field=operation.fields[0];
  const changed=body.replace(field.name+': field_0',field.name+': field_1');assert.notEqual(changed,body);
  assert.throws(()=>checkNativeConstructorSourceBindings(changed,operation.fields,operation.fields.map(row=>row.typeReference),byId));
});

test('all constructor branches have complete typed controls and malformed child or lifetime labels refuse',()=>{
  const {inventory}=nativeConstructorContext(),controls=buildNativeConstructorStorageControls(inventory);
  assert.deepEqual(controls,readProjectData(new URL('../../parity/fixtures/native-constructor-storage.lino',import.meta.url)));
  assert.deepEqual([...new Set(controls.cases.map(row=>row.sourceReference))],controls.sourceReferences);
  for(const row of controls.cases){assert.ok(checkNativeConstructorControl(inventory,row.recipe)>0);assert.equal(row.expectedStorageType,inventory.operations.find(operation=>operation.sourceReference===row.sourceReference).storage.rust);}
  for(const operation of inventory.operations){
    const cases=controls.cases.filter(row=>row.sourceReference===operation.sourceReference);
    if(operation.variants)assert.deepEqual(cases.map(row=>row.recipe.choice),operation.variants.map(row=>row.identity));
    if(operation.form==='Option')assert.deepEqual(cases.map(row=>row.recipe.choice),['None','Some']);
    if(['Vec','IndexMap'].includes(operation.form))assert.deepEqual(cases.map(row=>row.recipe.children.length),[0,operation.arguments.length]);
  }
  const some=controls.cases.find(row=>row.recipe.choice==='Some').recipe;
  for(const mutate of [value=>{value.reference='foreign';},value=>{value.children[0].reference='foreign';},value=>{value.children=[];},value=>{value.choice='foreign';}]){
    const value=structuredClone(some);mutate(value);assert.throws(()=>checkNativeConstructorControl(inventory,value));
  }
  const reference=controls.cases.find(row=>row.recipe.scalar?.kind==='ExistingStaticReference').recipe,changed=structuredClone(reference);
  changed.children[0].scalar.value='different';assert.throws(()=>checkNativeConstructorControl(inventory,changed));
  const module=readFileSync(new URL('../../rust/tests/unit/mod.rs',import.meta.url),'utf8');assert.ok(module.includes('mod grammar_outcome_constructor_storage;'));
  const native=readFileSync(new URL('../../rust/tests/unit/grammar_outcome_constructor_storage.rs',import.meta.url),'utf8');
  for(const name of ['every_captured_native_constructor_branch_packs_actual_typed_source_values','typed_storage_retains_original_number_variants_payloads_and_the_actual_static_pointer','foreign_reordered_missing_and_extra_source_values_refuse_without_new_lifetimes','reordered_equal_storage_types_and_duplicate_or_incomplete_native_map_pairs_refuse'])assert.ok(readSourceTextRustBlock(native,'fn',name));
});

test('the complete source-qualified storage instruction inventory and controls reload from both Links encodings',()=>{
  const {inventory}=nativeConstructorContext(),controls=buildNativeConstructorStorageControls(inventory);
  const packet=encodeStructuredDataLinks({planSha256:inventory.planSha256,operations:inventory.operations.map(row=>({sourceReference:row.sourceReference,form:row.form,storage:row.storage})),controls});
  const canonical=LinkNetwork.fromLino(packet.network.toLino());
  for(const graph of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(graph,packet.root);
    assert.deepEqual(restored.controls,controls);assert.deepEqual(restored.operations,inventory.operations.map(row=>({sourceReference:row.sourceReference,form:row.form,storage:row.storage})));
    for(const row of restored.controls.cases)checkNativeConstructorControl(inventory,row.recipe);
  }
  assert.equal(renderNativeConstructorStorage(inventory).size,6);
});
