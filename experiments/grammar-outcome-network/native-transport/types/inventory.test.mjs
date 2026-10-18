// Draft controls only. The author has not executed this file.
import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {captureNativeOutcomeTypeInventory,readPreErasureType} from './inventory.mjs';

const digest=source=>createHash('sha256').update(source).digest('hex');
let captured;
const current=()=>captured??=captureNativeOutcomeTypeInventory();
const declaration=(model,name)=>model.nodes.find(node=>node.name===name&&model.roots.includes(node.identity));
const referenced=(model,record)=>model.nodes.find(node=>node.identity===record.typeReference);
const allRecords=owner=>[...(owner.fields??[]),...(owner.variants??[]).flatMap(variant=>[...(variant.fields??[]),...(variant.tuple??[])])];

test('complete actual nine-owner declarations fields variants and source evidence survive derivation',()=>{
  const capture=current(),model=capture.derive();
  assert.equal(model.roots.length,9);assert.equal(model.inputProofs.length,4);
  assert.deepEqual(model.sourceInventory,capture.sourceInventory,'whole original inventory remains byte/text/data exact');
  for(const original of capture.sourceInventory.declarations) {
    const projected=declaration(model,original.name);assert.ok(projected);
    assert.equal(projected.source,original.source);assert.deepEqual(projected.span,original.span);assert.equal(projected.sha256,original.sha256);
    assert.equal(digest(projected.source),original.sha256);
    assert.deepEqual(allRecords(projected).map(record=>[record.name,record.type,record.source,record.span]),allRecords(original).map(record=>[record.name,record.type,record.source,record.span]));
    for(const variant of original.variants??[]) {
      const retained=projected.variants.find(value=>value.name===variant.name);
      assert.equal(retained.source,variant.source);assert.deepEqual(retained.span,variant.span);assert.equal(retained.sourceSha256,digest(variant.source));
    }
    const input=model.sourceInventory.inputs.find(value=>value.file===original.file),bytes=Buffer.from(input.source);
    for(const record of [...allRecords(projected),...(projected.sourceType?[projected]:[])]) {
      const evidence=record.sourceType;assert.ok(evidence,'every actual field/tuple/alias retains a complete type CST');
      assert.equal(bytes.subarray(evidence.span.start,evidence.span.end).toString(),evidence.source);
      assert.equal(evidence.sha256,digest(evidence.source));
      assert.deepEqual({start:evidence.cst.start,end:evidence.cst.end},evidence.span);
      assert.equal(typeof evidence.cst.term,'string');
    }
  }
  assert.deepEqual(declaration(model,'SyntaxTree').variants.map(value=>value.name),['Node','Token','Error','Missing','Embed']);
  assert.equal(declaration(model,'SyntaxAttributes').alias,'IndexMap<String,OperationValue>');
  assert.equal(model.rootReexport.source,capture.sourceInventory.rootReexport.source);
});

test('exact static lifetime and owned string remain distinct before v1 erasure',()=>{
  const model=current().derive(),rejection=declaration(model,'ParseRejection');
  const reason=rejection.fields.find(field=>field.name==='reason'),borrow=referenced(model,reason);
  assert.equal(reason.type,"&'staticstr");assert.equal(reason.sourceType.source,"&'static str");
  assert.equal(borrow.kind,'reference');assert.equal(borrow.ownership,'borrowed');assert.equal(borrow.lifetime,'static');assert.equal(borrow.mutable,false);
  assert.deepEqual(borrow.lifetimeTokens.map(token=>token.value),["'",'static']);assert.deepEqual(borrow.mutabilityTokens,[]);
  assert.equal(model.nodes.find(node=>node.identity===borrow.references[0]).kind,'str');
  const field=declaration(model,'Ambiguity').fields.find(field=>field.name==='rule'),owned=referenced(model,field);
  assert.equal(owned.kind,'String');assert.equal(owned.ownership,'owned');assert.notEqual(owned.identity,borrow.identity);
  assert.ok(borrow.obligations.some(value=>value.includes('static reference')));
});

test('closed finite reference graph preserves recursive variants wrappers qualified imports and unresolved profiles',()=>{
  const model=current().derive(),identities=new Set(model.nodes.map(node=>node.identity));
  assert.equal(identities.size,model.nodes.length);
  for(const node of model.nodes)for(const reference of node.references)assert.ok(identities.has(reference));
  const tree=declaration(model,'SyntaxTree'),node=tree.variants.find(value=>value.name==='Node');
  const vector=referenced(model,node.fields.find(value=>value.name==='children'));assert.equal(vector.kind,'Vec');
  const self=model.nodes.find(value=>value.identity===vector.arguments[0]);assert.equal(self.kind,'named');assert.deepEqual(self.references,[tree.identity]);
  const embed=tree.variants.find(value=>value.name==='Embed'),option=referenced(model,embed.fields.find(value=>value.name==='selection'));
  assert.equal(option.kind,'Option');const box=model.nodes.find(value=>value.identity===option.arguments[0]);assert.equal(box.kind,'Box');
  const qualified=model.nodes.find(value=>value.identity===box.arguments[0]);assert.deepEqual(qualified.references,[declaration(model,'GrammarSelection').identity]);
  assert.equal(qualified.source,'super::grammar_selection::GrammarSelection');
  const alias=declaration(model,'SyntaxAttributes'),map=model.nodes.find(value=>value.identity===alias.aliasTypeReference);
  assert.equal(map.kind,'IndexMap');assert.equal(map.arguments.length,2);
  assert.equal(map.binding.qualified,'indexmap::IndexMap');assert.match(map.binding.source,/use indexmap::IndexMap;/u);
  const usize=referenced(model,node.fields.find(value=>value.name==='start'));assert.deepEqual(usize.integer,{signed:false,bits:null});assert.ok(usize.obligations[0].includes('pointer width'));
  const selection=declaration(model,'GrammarSelection'),json=referenced(model,selection.fields.find(value=>value.name==='provenance'));
  assert.equal(json.kind,'json');assert.equal(json.binding.qualified,'serde_json::Value');assert.ok(json.obligations.some(value=>value.includes('effective feature')));
  const integer=referenced(model,declaration(model,'OperationValue').variants.find(value=>value.name==='Integer').tuple[0]);assert.deepEqual(integer.integer,{signed:true,bits:64});
  assert.ok(model.nodes.length<256,'syntax references are finite rather than recursive type expansion');
});

test('unknown unsupported shadowed and ambiguous type forms refuse without guessing owners',()=>{
  const scope=current().scopes.find(value=>value.qualifiedOwner.endsWith('::SyntaxTree'));
  for(const source of ['Unknown','foreign::SyntaxTree','Result<String,String>','Vec<String,String>','Option<>','Vec','[String;3]','*const str',"&'a str","&'static mut str",'&str','String String','Vec<Vec<String>> garbage'])
    assert.throws(()=>readPreErasureType(source,scope),undefined,source);
  const shadowed=structuredClone(scope);shadowed.bindings.find(value=>value.name==='String').identity='unadmitted:foreign::String';
  assert.throws(()=>readPreErasureType('String',shadowed),/admitted source-qualified owner/u);
  const ambiguous=structuredClone(scope);ambiguous.bindings.push({...ambiguous.bindings.find(value=>value.name==='String')});
  assert.throws(()=>readPreErasureType('String',ambiguous),/ambiguous type owner/u);
  const unknown=structuredClone(scope);unknown.owners.push({identity:'owner:foreign::String',qualified:'foreign::String',form:'foreign'});unknown.bindings.push({name:'Foreign',identity:'owner:foreign::String'});
  assert.throws(()=>readPreErasureType('Foreign<String>',unknown),/unsupported source-qualified type owner/u);
  assert.throws(()=>readPreErasureType('Foreign',unknown),/unsupported source-qualified type owner/u);
  const type=readPreErasureType("& /* retained */ 'static str",scope,'control:comment',7);
  const root=type.nodes.find(value=>value.identity===type.root);assert.equal(root.source,"& /* retained */ 'static str");assert.equal(root.span.start,7);
});

test('whole-inventory conservation rejects forged projections even with unchanged authenticated input sources',()=>{
  const capture=current(),original=structuredClone(capture.sourceInventory),before=capture.derive();
  const edits=[
    value=>{value.declarations[0].source+=' ';},
    value=>{value.declarations[0].span.start++;},
    value=>{value.declarations[0].sha256='0'.repeat(64);},
    value=>{value.declarations.find(item=>item.name==='ParseRejection').fields.find(item=>item.name==='reason').type='String';},
    value=>{value.declarations.find(item=>item.name==='ParseOutcome').fields[0].source='ok: bool';},
    value=>{value.declarations.find(item=>item.name==='ParseOutcome').fields.pop();},
    value=>{value.declarations.find(item=>item.name==='SyntaxTree').variants[0].source+=' ';},
    value=>{value.declarations.find(item=>item.name==='SyntaxTree').variants.pop();},
    value=>{value.declarations.find(item=>item.name==='LeafText').variants[0].tuple[0].span.end++;},
    value=>{value.declarations.find(item=>item.name==='SyntaxAttributes').alias='BTreeMap<String,OperationValue>';},
    value=>{value.inputs[0].imports[0].source+=' ';},
    value=>{value.rootReexport.span.end++;},
    value=>{value.pins[0].sha256='0'.repeat(64);},
    value=>{value.declarations.push(structuredClone(value.declarations[0]));},
    value=>{value.extra=true;},
  ];
  for(const edit of edits){const forged=structuredClone(original);edit(forged);assert.throws(()=>capture.derive(forged),/complete authenticated/u);assert.deepEqual(capture.sourceInventory,original);}
  before.nodes[0].qualified='changed output';assert.notEqual(capture.derive().nodes[0].qualified,'changed output','returned copies cannot rewrite the authenticated source capture');
});


test('Unicode type comments retain exact UTF-8 source spans token spans and hashes',()=>{
  const scope=current().scopes.find(value=>value.qualifiedOwner.endsWith('::SyntaxTree'));
  const source="Vec<& /* 漢字 😀 é */ 'static str>",offset=37;
  const result=readPreErasureType(source,scope,'control:unicode',offset);
  const root=result.nodes.find(value=>value.identity===result.root),bytes=Buffer.from(source);
  assert.equal(root.source,source);assert.deepEqual(root.span,{start:offset,end:offset+bytes.length});assert.equal(root.sha256,digest(source));
  assert.ok(bytes.length>source.length,'non-ASCII fixture actually distinguishes UTF-8 bytes from UTF-16 offsets');
  for(const node of result.nodes){
    assert.equal(bytes.subarray(node.span.start-offset,node.span.end-offset).toString(),node.source);
    assert.equal(node.sha256,digest(node.source));
    for(const token of node.tokens)assert.equal(bytes.subarray(token.byteSpan.start-offset,token.byteSpan.end-offset).toString(),source.slice(token.start,token.end));
  }
  const text=result.nodes.find(value=>value.kind==='str'),at=source.lastIndexOf('str');
  assert.deepEqual(text.span,{start:offset+Buffer.byteLength(source.slice(0,at)),end:offset+bytes.length-1});
});

test('unpaired type source refuses before token interpretation UTF-8 conversion and hashing',()=>{
  const scope=current().scopes.find(value=>value.qualifiedOwner.endsWith('::SyntaxTree'));
  for(const unit of [0xd800,0xdfff])for(const source of ['String /* '+String.fromCharCode(unit)+' */',String.fromCharCode(unit)+'String'])
    assert.throws(()=>readPreErasureType(source,scope),/unpaired UTF-16 type source/u);
});
