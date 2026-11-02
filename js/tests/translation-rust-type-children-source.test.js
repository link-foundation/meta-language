// Deferred complete actual readonly body/region controls; never run during the
// source drafting phase. The existing signature3 iterator/kernel owns behavior.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import vm from 'node:vm';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {checkRustBorrowedVectorSourceRegions} from '../src/translation/rust-borrowed-source-regions.js';
import {createRustTypeChildrenSourceOriginals,rustTypeChildrenSourceVersions as versions,rustTypeChildrenSourceLengths as lengths,rustTypeChildrenSourceOrderScenarios as orders} from './fixtures/rust-type-children-source.mjs';
function fixture(){const typesSource=readFileSync(new URL('../../rust/src/translation/types.rs',import.meta.url),'utf8'),spanSource=readFileSync(new URL('../../rust/src/translation/mod.rs',import.meta.url),'utf8');return createRustTypeChildrenSourceOriginals({typesSource,spanSource,typesNetwork:LinkNetwork.parse(typesSource,'Rust'),spanNetwork:LinkNetwork.parse(spanSource,'Rust')});}
function output(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:5000});return lines.length?lines.join('\n')+'\n':'';}
const options=sourceVersion=>({sourceVersion,sourceSignatureModel:versions.sourceSignatureModel});
test('actual complete Type children capture keeps qualified original owners and unchanged method',()=>{
 const capture=fixture(),method=capture.types.items.find(row=>row.name==='children'),source=readFileSync(new URL('../../rust/src/translation/types.rs',import.meta.url),'utf8');assert.equal(capture.originals.length,33);assert.equal(lengths.length,23);assert.equal(orders.length,10);assert.equal(capture.refusals.length,6);assert.equal(source.slice(method.range.start,method.range.end),method.text);assert.equal(createHash('sha256').update(method.text).digest('hex'),method.sha256);assert.ok(method.bodyText.includes('params.iter().chain(std::iter::once(ret.as_ref())).collect()'));
 for(const row of capture.originals){assert.ok(row.source.includes(method.declarationText));assert.ok(row.nativeSource.includes(method.declarationText));assert.ok(row.ownCrateNativeSource.includes('source.children()'));assert.ok(!row.ownCrateNativeSource.includes('impl Type'));assert.deepEqual(row.capture.method.sourceOwner,['crate','translation','types','Type','children']);assert.equal(row.stdout,'true\n');}
});
for(const sourceVersion of ['1.90.0','1.99.0'])test('actual full Type children all variants keep real returned reference regions/'+sourceVersion,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 for(const row of fixture().originals){const checked=checkProgram(parseRust(row.source,options(sourceVersion))),method=checked.declarations.get('translation.types.Type.children'),signature=method.sourceSignature;assert.equal(signature.model,versions.sourceSignatureModel,row.name);assert.deepEqual(signature.parameters[0].wrappers,['reference']);assert.deepEqual(signature.result.arrayElementPosition.wrappers,['reference']);assert.deepEqual(signature.result.arrayElementPosition.owner,{kind:'declaration',path:['translation','types','Type'],arguments:[]});assert.equal(signature.result.arrayElementPosition.lifetimes[0],null);assert.equal(checkRustBorrowedVectorSourceRegions(checked),checked);assert.equal(output(emitJavaScript(checked).text),row.stdout,row.name);}
});
test('actual full Type children retains native rejection inputs and authority boundaries',()=>{for(const row of fixture().refusals){assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,options('1.90.0'))),error=>['syntax','unsupported','type'].includes(error.kind),row.name);}});
test('signature3 standard sum construction preserves actual library source owners',()=>{
 const source='struct Node { label: Option<String> } fn children(value: &Node) -> Vec<&Node> { vec![value] } fn main() { let value = Node { label: Some(String::from("kept")) }; let refs = children(&value); println!("{}", refs.len() == 1); }',checked=checkProgram(parseRust(source,options('1.90.0')));assert.equal(output(emitJavaScript(checked).text),'true\n');assert.equal(checkRustBorrowedVectorSourceRegions(checked),checked);
});
test('actual children declaration source positions and outlives survive Links storage',()=>{
 const row=fixture().originals.find(value=>value.name==='Function visits parameters before boxed return'),packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustSourceSignatureModel:versions.sourceSignatureModel}),text=packet.network.toLino(),bytes=encodeLinkNetworkBinary(packet.network),targets=['JavaScript','Rust','Lean','Rocq'],baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(bytes)]){assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),bytes);const checked=decodeTypedProgramLinks(graph,packet.root);assert.equal(checkRustBorrowedVectorSourceRegions(checked),checked);assert.equal(checked.declarations.get('translation.types.Type.children').sourceSignature.model,versions.sourceSignatureModel);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);if(target==='JavaScript')assert.equal(output(emitted.code),row.stdout);}}
});
test('retained readonly result cannot be requalified as static in checked Links',()=>{
 const row=fixture().originals[8],packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustSourceSignatureModel:versions.sourceSignatureModel}),checked=decodeTypedProgramLinks(packet.network,packet.root),fn=checked.declarations.get('translation.types.Type.children');fn.sourceSignature.result.arrayElementPosition.lifetimes[0]='static';assert.throws(()=>checkRustBorrowedVectorSourceRegions(checked),error=>['type','unsupported'].includes(error.kind));
});

// The exact former indexed matches input is retained as positive after admission.
test('former indexed observation frontier retains its complete original main',()=>{const row=orders.find(value=>value.main);assert.ok(row);const retained=fixture().originals.filter(value=>value.name===row.name);assert.equal(retained.length,1);for(const actual of retained){assert.ok(actual.source.endsWith(row.main));assert.ok(actual.nativeSource.endsWith(row.main));assert.ok(actual.ownCrateNativeSource.endsWith(row.main));assert.equal(actual.compilerAccepted,true);}});
