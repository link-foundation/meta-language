// Deferred source/project/ownership controls; no local execution while drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {createRustSourceProjectProgramLinks,readRustSourceProjectConcreteGraphs} from '../src/rust-source-project.js';
import {decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const source="pub fn borrow<'a>(value: &'a i64) -> &'a i64 { value }";
test('supplied libraries retain real readonly source signatures and select a getter only once',()=>{
 let reads=0;const options={library:true,retainCompilation:true,sourceVersion:'1.90.0',get sourceSignatureModel(){reads+=1;return 'Rust source function signature 3';}};
 const packet=createRustSourceProjectProgramLinks([{path:'lib.rs',source}],'lib.rs',options);
 assert.equal(reads,1);assert.equal(packet.sourceProfile.rustSourceSignatureModel,'Rust source function signature 3');assert.equal(packet.entryNormalization,'empty frontend entry for a source library');
 const declaration=packet.program.declarations.get('borrow');assert.equal(declaration.sourceSignature.model,'Rust source function signature 3');assert.equal(declaration.sourceSignature.parameters[0].wrappers[0],'reference');assert.equal(declaration.sourceSignature.result.wrappers[0],'reference');
 for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){const restored=decodeTypedProgramLinks(network,packet.root);recheckLinkedTypedProgram(restored,{wholeProgram:true});assert.deepEqual(restored.declarations.get('borrow').sourceSignature,declaration.sourceSignature);const concrete=readRustSourceProjectConcreteGraphs(network,packet.root);assert.equal(concrete.length,1);assert.equal(concrete[0].source,source);assert.equal(concrete[0].network.reconstructText(),source);}
});
test('project source signature selection remains an explicit guarded interpretation',()=>{
 assert.throws(()=>createRustSourceProjectProgramLinks([{path:'lib.rs',source}],'lib.rs',{library:true,sourceSignatureModel:'invented source signature'}));
});
