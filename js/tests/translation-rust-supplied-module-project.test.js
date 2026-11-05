import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
// Deferred supplied-project controls. No parse/compile/validation runs on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {captureRustSuppliedModuleProject,readRustSuppliedModuleOwnership} from '../src/translation/rust-supplied-module-project.js';
import {readMetaLanguageArchive} from '../src/meta-language-archive.js';
const options={crateRoot:'src/main.rs',parseOptions:{sourceVersion:'1.90.0'}};

test('supplied module files become real qualified bodies while every original CST survives',()=>{
 const inputs=[{path:'src/main.rs',source:'mod math; fn main() { assert!(math::read() == 7u8); }'},{path:'src/math.rs',source:'pub fn read() -> u8 { 7u8 }'},{path:'src/unused.rs',source:'pub fn other() -> u8 { 9u8 }'}],result=captureRustSuppliedModuleProject(inputs,options);
 assert.equal(result.moduleGraph.modules.length,1);assert.deepEqual(result.moduleGraph.modules[0].namespace,['crate','math']);assert.equal(result.moduleGraph.modules[0].target,'Rust supplied source src/math.rs');assert.equal(result.program.sourceLanguage,'Rust');assert.ok([...result.program.declarations.values()].some(entry=>entry.k==='fn'&&entry.body));
 const archive=readMetaLanguageArchive(result.archive.network,result.archive.root);for(const input of inputs){const entry=archive.entries.find(row=>row.identifier==='Rust supplied source '+input.path);assert.equal(entry.kind,'concrete syntax');assert.equal(entry.network.reconstructText(),input.source);}
 assert.equal(result.moduleGraph.files.find(row=>row.path==='src/unused.rs').selected,false);
});

test('inline namespaces and both Rust filesystem module conventions select actual supplied owners',()=>{
 for(const outer of ['src/outer.rs','src/outer/mod.rs']){
  const inputs=[{path:'src/main.rs',source:'mod outer; fn main() { assert!(outer::nested::value() == 3u8); }'},{path:outer,source:'pub mod nested; '},{path:'src/outer/nested.rs',source:'pub fn value() -> u8 { 3u8 }'}];const result=captureRustSuppliedModuleProject(inputs,options);assert.deepEqual(result.moduleGraph.modules.map(row=>row.namespace),[['crate','outer'],['crate','outer','nested']]);
 }
 const nested=captureRustSuppliedModuleProject([{path:'src/main.rs',source:'mod outer { pub mod nested; } fn main() { assert!(outer::nested::value() == 3u8); }'},{path:'src/outer/nested.rs',source:'pub fn value() -> u8 { 3u8 }'}],options);assert.deepEqual(nested.moduleGraph.modules[1].namespace,['crate','outer','nested']);
});

test('source mappings preserve original UTF16 coordinates and child line-comment boundaries',()=>{
 const source='// é😀\nmod math; fn main() { assert!(math::read() == 7u8); }',child='pub fn read() -> u8 { 7u8 } // closing line comment';const result=captureRustSuppliedModuleProject([{path:'src/main.rs',source},{path:'src/math.rs',source:child}],options),module=result.moduleGraph.modules[0];assert.equal(source.slice(module.nameSpan.start,module.nameSpan.end),'math');
 const segment=result.moduleGraph.segments.find(row=>row.source==='Rust supplied source src/math.rs'&&row.role==='original');assert.equal(child.slice(segment.start,segment.end),child);
});

test('library admission removes only its synthetic entry and preserves nested functions named main',()=>{
 const inputs=[{path:'src/lib.rs',source:'mod child; pub fn value() -> u8 { child::read() }'},{path:'src/child.rs',source:'pub fn read() -> u8 { 7u8 } pub fn main() -> u8 { 3u8 }'}],result=captureRustSuppliedModuleProject(inputs,{crateRoot:'src/lib.rs',library:true,parseOptions:{sourceVersion:'1.90.0'}});assert.equal(result.entryNormalization,'empty frontend entry for a source library');assert.deepEqual(result.surface.main.effects,[]);assert.deepEqual(result.program.main.effects,[]);assert.ok([...result.program.declarations.values()].some(row=>row.k==='fn'&&row.body));
 const actual=captureRustSuppliedModuleProject([{path:'lib.rs',source:'fn main() {}'}],{crateRoot:'lib.rs',library:true});assert.equal(actual.entryNormalization,'none');
});

test('missing ambiguous duplicate conditional and path module owners refuse instead of guessing',()=>{
 const main={path:'src/main.rs',source:'mod child; fn main() {}'};assert.throws(()=>captureRustSuppliedModuleProject([main],options),/missing.*module/);
 assert.throws(()=>captureRustSuppliedModuleProject([main,{path:'src/child.rs',source:''},{path:'src/child/mod.rs',source:''}],options),/ambiguous module/);
 assert.throws(()=>captureRustSuppliedModuleProject([{...main,source:'mod child; mod child; fn main() {}'},{path:'src/child.rs',source:''}],options),/duplicate/);
 for(const attribute of ['#[cfg(any())]','#[cfg_attr(any(), path = "other.rs")]'])assert.throws(()=>captureRustSuppliedModuleProject([{...main,source:attribute+' mod child; fn main() {}'},{path:'src/child.rs',source:''},{path:'src/other.rs',source:''}],options),/explicit configuration interpretation/);
});

test('supplied source capabilities reject traversal duplicate files cycles and resource excess',()=>{
 for(const path of ['../main.rs','/main.rs','src/../main.rs','src\\main.rs','src//main.rs'])assert.throws(()=>captureRustSuppliedModuleProject([{path,source:'fn main() {}'}],{crateRoot:path}),TypeError);
 assert.throws(()=>captureRustSuppliedModuleProject([{path:'main.rs',source:'fn main() {}'},{path:'main.rs',source:'fn main() {}'}],{crateRoot:'main.rs'}),/duplicate/);
 assert.throws(()=>captureRustSuppliedModuleProject([{path:'mod.rs',source:'mod r#mod; fn main() {}'}],{crateRoot:'mod.rs'}),/cyclic Rust project module file/);
 assert.throws(()=>captureRustSuppliedModuleProject([{path:'main.rs',source:'fn main() {}'}],{crateRoot:'main.rs',byteLimit:1}),/byte limit/);
 assert.throws(()=>captureRustSuppliedModuleProject([{path:'main.rs',source:'mod inner { mod leaf {} } fn main() {}'}],{crateRoot:'main.rs',depthLimit:1}),/expansion limit/);
});


test('explicit literal path attributes retain their real CST field and supplied provider',()=>{
 const source='#[path = "elsewhere.rs"] mod child; fn main() { assert!(child::read() == 7u8); }';const result=captureRustSuppliedModuleProject([{path:'src/main.rs',source},{path:'src/elsewhere.rs',source:'pub fn read() -> u8 { 7u8 }'}],options);assert.equal(result.moduleGraph.modules[0].target,'Rust supplied source src/elsewhere.rs');assert.equal(result.composition.mappings.filter(row=>row.kind==='interpreted attribute').length,1);
});


test('CST module ownership survives text and binary reload without a Rust grammar call',()=>{
 const result=captureRustSuppliedModuleProject([{path:'main.rs',source:'mod item; fn main() { assert!(item::value() == 7u8); }'},{path:'item.rs',source:'pub fn value() -> u8 { 7u8 }'}],{crateRoot:'main.rs'});
 const networks=[result.network,LinkNetwork.fromLino(result.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(result.network))],parse=LinkNetwork.parse;
 try{LinkNetwork.parse=function(source,language,...options){assert.notEqual(language,'Rust','ownership reload must not invoke the Rust grammar');return parse.call(this,source,language,...options);};for(const network of networks)assert.deepEqual(readRustSuppliedModuleOwnership(network,result.root),result.moduleGraph);}finally{LinkNetwork.parse=parse;}
});

test('changed module provider directories cannot substitute a different archived source',()=>{
 const result=captureRustSuppliedModuleProject([{path:'main.rs',source:'mod item; fn main() {}'},{path:'item.rs',source:'pub fn value() -> u8 { 7u8 }'},{path:'other.rs',source:'pub fn value() -> u8 { 9u8 }'}],{crateRoot:'main.rs'}),network=LinkNetwork.fromLino(result.network.toLino()),graph={...result.moduleGraph,modules:result.moduleGraph.modules.map(row=>({...row,target:'Rust supplied source other.rs'}))},encoded=encodeStructuredDataLinks(graph),ids=new Map();
 for(const link of encoded.network.links())ids.set(link.id().asU64(),network.insertLink([],link.metadata().clone()));for(const link of encoded.network.links())network.link(ids.get(link.id().asU64())).setReferences(link.references().map(reference=>ids.get(reference.asU64())));
 const field=network.fieldRelations().find(row=>row.parent.asU64()===result.root.asU64()&&row.label==='Rust module concrete ownership'),references=field.link.references();field.link.setReferences([references[0],references[1],ids.get(encoded.root.asU64())]);assert.throws(()=>readRustSuppliedModuleOwnership(network,result.root),/differs from its actual current concrete sources/);
});
