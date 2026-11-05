// Deferred actual-CST controls. This file is authored, not executed locally.
import test from 'node:test';import assert from 'node:assert/strict';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {captureRustConcreteSourceItems} from '../src/translation/rust-concrete-source-items.js';
const source=`// 🦀 A source capture retains original annotations and bodies.
#[derive(Clone, Debug)]
pub enum Type { Unit, Array(Box<Type>) }
#[derive(Clone)]
pub struct TypeField { #[serde(rename="original")] pub name:String, pub ty:Type }
impl Type {
 pub fn map_children<E>(&self, mut map:impl FnMut(&Type)->Result<Type,E>)->Result<Type,E> {
  match self { Self::Unit=>Ok(Self::Unit), Self::Array(child)=>Ok(Self::Array(Box::new(map(child)?))) }
 }
 pub const fn is_reference(&self)->bool { false }
}
mod nested { pub fn map_children(){} }
`;
const selectors=[{kind:'enum_item',name:'Type'},{kind:'struct_item',name:'TypeField'},{kind:'function_item',name:'map_children',implementationType:'Type'},{kind:'function_item',name:'is_reference',implementationType:'Type'},{kind:'function_item',name:'map_children',modulePath:['nested']}];
test('original item bodies source coordinates and annotations survive both graph transports',()=>{
 const original=LinkNetwork.parse(source,'Rust'),expected=captureRustConcreteSourceItems(original,source,{path:'types.rs',selectors});
 assert.equal(expected.items.length,5);assert.equal(expected.sourceUnits,source.length);assert.match(expected.sourceSha256,/^[a-f0-9]{64}$/);
 assert.equal(expected.items[0].kind,'enum_item');assert.equal(expected.items[1].kind,'struct_item');
 const canonicalSelectors=selectors.map(row=>({...row,kind:({enum_item:'enumeration_item',struct_item:'structure_item'})[row.kind]??row.kind}));assert.deepEqual(captureRustConcreteSourceItems(original,source,{path:'types.rs',selectors:canonicalSelectors}),expected);
 assert.ok(expected.items[0].declarationText.startsWith('#[derive(Clone, Debug)]'));
 assert.ok(expected.items[1].attributes.some(row=>row.text==='#[serde(rename="original")]'&&row.ownership==='enclosed'));
 assert.equal(expected.items[2].implementation.type,'Type');assert.equal(expected.items[4].modules[0].name,'nested');
 for(const item of expected.items){assert.equal(item.text,source.slice(item.range.start,item.range.end));assert.equal(item.declarationText,source.slice(item.declarationRange.start,item.declarationRange.end));if(item.bodyText!==undefined)assert.equal(item.bodyText,source.slice(item.bodyRange.start,item.bodyRange.end));assert.equal(Buffer.byteLength(source.slice(0,item.range.start)),item.range.byteStart);assert.equal(Buffer.byteLength(source.slice(0,item.range.end)),item.range.byteEnd);assert.ok(original.link(item.sourceLink));}
 for(const network of [LinkNetwork.fromLino(original.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(original))])assert.deepEqual(captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors}),expected);
});
test('ambiguous duplicate foreign source and budget claims cannot replace actual CST ownership',()=>{
 const network=LinkNetwork.parse(source,'Rust');
 for(const selectors of [[{kind:'function_item',name:'map_children'}],[{kind:'function_item',name:'map_children',implementationType:'Other'}],[{kind:'function_item',name:'missing'}],[{kind:'enum_item',name:'Type'},{kind:'enum_item',name:'Type'}]])assert.throws(()=>captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors}),/actual source declaration|duplicate/);
 assert.throws(()=>captureRustConcreteSourceItems(network,source+' ',{path:'types.rs',selectors}),/does not match/);
 for(const limits of [{byteLimit:1},{nodeLimit:1},{fieldLimit:1},{workLimit:1},{selectorLimit:1}])assert.throws(()=>captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors,...limits}),/budget|bounded|inspection|source text/);
 const selected=captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors}),method=selected.items[2];network.insertFieldRelation(method.sourceLink,'name',selected.items[4].nameLink);
 assert.throws(()=>captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors}),/unambiguous CST owner/);
});

test('coincident source spans cannot replace actual inline module ancestry',()=>{
 const network=LinkNetwork.parse(source,'Rust'),selected=captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors}),nested=selected.items[4],module=network.link(nested.modules[0].sourceLink),references=module.references();
 // Retain the source tokens/bytes and module span while removing its actual
 // body syntax edge. A field label and enclosing coordinates cannot replace it.
 const body=network.fieldRelations().find(edge=>edge.parent.asU64()===module.id().asU64()&&edge.label==='body');assert.ok(body);module.setReferences(references.filter(id=>id.asU64()!==body.child.asU64()));
 assert.throws(()=>captureRustConcreteSourceItems(network,source,{path:'types.rs',selectors:[selectors[4]]}),/actual source declaration|does not match|inside its owner/);
});
