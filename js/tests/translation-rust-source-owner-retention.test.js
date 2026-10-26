// Future complete source facets, actual shared rules, codecs and meaning checks.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {checkRustSourceSignature} from '../src/translation/rust-source-signatures.js';
import {createRustSourceProjectProgramLinks,readRustSourceProjectConcreteGraphs} from '../src/rust-source-project.js';
import {runRustSourceProjectCommand} from '../src/rust-source-project-cli.js';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {encodeFormalConceptDefinition,proveFormalConceptEquivalence} from '../src/formal-concept-definition.js';
import {acceptRustRetainedFunctionOwner,selectRustRetainedSourceSignatureModel} from '../src/translation/rust-source-owner-retention-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-owner-retention.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source);
 for(const network of views(packet))assert.deepEqual(decodeTypedProgramLinks(network,packet.root).declarations.get(row.functionName).sourceSignature,row.signature);
 for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of views(packet))assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}
});
for(const row of fixture.frames)test('actual retained function owner: '+JSON.stringify(row),()=>assert.equal(acceptRustRetainedFunctionOwner(row.model,row.owner,row.type,row.paths,row.children,row.parameters,row.reference),row.expected));
for(const row of fixture.selections)test('actual source facet selection: '+JSON.stringify(row),()=>assert.equal(selectRustRetainedSourceSignatureModel(row.model,row.retain),row.expected));
test('the original signature model keeps its function-type refusal',()=>{
 const row=fixture.cases.find(row=>row.functionName==='invoke'&&row.signature.parameters[0].type.params.length===1),signature={...row.signature,model:'Rust source function signature 1'};
 assert.throws(()=>checkRustSourceSignature(signature,row.signature.parameters.map(position=>position.type),row.signature.result.type,'Rust'),error=>error.reason==='an owned Rust source type has an unsupported value type');
});
test('equal widths cannot certify equal source owner meanings',()=>{
 const packets=fixture.identities.map(row=>createRustSourceOwnerProgramLinks(row.source));
 const programs=packets.map(packet=>decodeTypedProgramLinks(packet.network,packet.root));
 assert.deepEqual(programs[0].declarations.get('identity').params[0].type,programs[1].declarations.get('identity').params[0].type);
 assert.notDeepEqual(programs[0].declarations.get('identity').sourceSignature,programs[1].declarations.get('identity').sourceSignature);
 const definitions=packets.map(packet=>encodeFormalConceptDefinition(packet));
 assert.ok(proveFormalConceptEquivalence(definitions[0],definitions[0]));assert.equal(proveFormalConceptEquivalence(definitions[0],definitions[1]),null);
});
test('source-project and CLI Links options retain owners and every original CST',()=>{
 const {inputs,entry,options}=fixture.project,packet=createRustSourceProjectProgramLinks(inputs,entry,options),expected=structuredDataToLino({inputs,options});
 for(const network of views(packet)){const program=decodeTypedProgramLinks(network,packet.root);for(const name of ['identity','values.identity'])assert.equal(program.declarations.get(name).sourceSignature.model,'Rust source function signature 2');assert.deepEqual(readRustSourceProjectConcreteGraphs(network,packet.root).map(({path,source})=>({path,source})),inputs);}
 for(const binary of [false,true]){const output=runRustSourceProjectCommand(['inventory.lino','--entry',entry,...(binary?['--binary']:[])],{readFile:()=>expected});assert.equal(output.exitCode,0,output.stderr);assert.equal(output.stderr,'');const network=binary?decodeLinkNetworkBinary(output.stdout):LinkNetwork.fromLino(output.stdout);assert.deepEqual(network.toLino(),packet.network.toLino());}
});
