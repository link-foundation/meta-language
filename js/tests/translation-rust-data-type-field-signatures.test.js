// Deferred controls: author definitions now; execute only after the full draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {encodeFormalConceptDefinition,proveFormalConceptEquivalence} from '../src/formal-concept-definition.js';
import {createRustSourceProjectProgramLinks,readRustSourceProjectConcreteGraphs} from '../src/rust-source-project.js';
import * as rules from '../src/translation/rust-data-type-field-signature-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-data-type-field-signatures.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source);
 for(const network of views(packet))assert.deepEqual(decodeTypedProgramLinks(network,packet.root).declarations.get(row.dataTypeName).sourceFieldSignatures,row.signature);
 for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of views(packet))assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}
});
test('all shared framing field budget and container decisions use actual counts',()=>{
 for(const row of fixture.frames)assert.equal(rules.acceptRustDataTypeFieldSignature(row.model,row.language,row.declared,row.actual,row.constructors,row.expectedCount),row.expected);
 for(const row of fixture.fields)assert.equal(rules.acceptRustConstructorFieldSignature(row.name,row.actual,row.fields,row.expectedCount),row.expected);
 for(const row of fixture.budgets)assert.equal(rules.acceptRustDataTypeFieldSignatureBudget(row.visits,row.depth),row.expected);
 for(const row of fixture.containers){assert.equal(rules.acceptRustDataTypeFieldSignatureContainer(row.kind,row.dense,row.unrepresented),row.expected);assert.equal(rules.acceptRustDataTypeFieldSignatureProgramContainer(row.kind,row.dense,row.unrepresented),row.programExpected);}
});
for(const row of fixture.identities)test(row.name+' remains a distinct formal field meaning',()=>{
 const packets=row.sources.map(createRustSourceOwnerProgramLinks),programs=packets.map(packet=>decodeTypedProgramLinks(packet.network,packet.root));
 assert.deepEqual(programs[0].declarations.get('Value').ctors,programs[1].declarations.get('Value').ctors);
 assert.deepEqual(programs[0].declarations.get('identity').sourceSignature,programs[1].declarations.get('identity').sourceSignature);
 assert.notDeepEqual(programs[0].declarations.get('Value').sourceFieldSignatures,programs[1].declarations.get('Value').sourceFieldSignatures);
 const definitions=packets.map(packet=>encodeFormalConceptDefinition(packet));assert.ok(proveFormalConceptEquivalence(definitions[0],definitions[0]));assert.equal(proveFormalConceptEquivalence(definitions[0],definitions[1]),null);
});
function change(record,path,value){let target=record;for(const field of path.slice(0,-1))target=target[field];target[path.at(-1)]=structuredClone(value);}
test('every malformed field facet refuses at both Surface and source-free Links boundaries',()=>{
 for(const row of fixture.malformed){
  const surface=parseRust(fixture.controlSource,{retainSourceTypeOwners:true}),item=surface.items.find(item=>item.k==='data');change(item.sourceFieldSignatures,row.path,row.value);assert.throws(()=>checkProgram(surface));
  const packet=createRustSourceOwnerProgramLinks(fixture.controlSource),program=decodeTypedProgramLinks(packet.network,packet.root);change(program.declarations.get('Value').sourceFieldSignatures,row.path,row.value);assert.throws(()=>encodeTypedProgramLinks(program));
 }
 for(const value of [null,undefined]){const surface=parseRust(fixture.controlSource,{retainSourceTypeOwners:true});surface.items.find(item=>item.k==='data').sourceFieldSignatures=value;assert.throws(()=>checkProgram(surface));}
});
test('host accessors inherited markers sparse positions and structural cycles cannot supply field definitions',()=>{
 let reads=0;
 for(const edit of [
  item=>Object.defineProperty(item,'sourceFieldSignatures',{enumerable:true,get(){reads+=1;throw new Error('accessor executed');}}),
  item=>{const signature=item.sourceFieldSignatures;delete item.sourceFieldSignatures;Object.setPrototypeOf(item,{sourceFieldSignatures:signature});},
  item=>{delete item.sourceFieldSignatures.ctors[0];},
  item=>{item.sourceFieldSignatures.ctors[0].fields[0].owner.arguments.push(item.sourceFieldSignatures);},
  item=>{item.sourceFieldSignatures.ctors=new Map();},
 ]){const surface=parseRust(fixture.controlSource,{retainSourceTypeOwners:true});edit(surface.items.find(item=>item.k==='data'));assert.throws(()=>checkProgram(surface));}
 assert.equal(reads,0);
});
test('plain parsing stays independent and existing compact reference splitting keeps identical owners',()=>{
 const plain=checkProgram(parseRust(fixture.controlSource));assert.ok(!Object.hasOwn(plain.declarations.get('Value'),'sourceFieldSignatures'));
 const programs=fixture.compact.sources.map(source=>{const packet=createRustSourceOwnerProgramLinks(source);return decodeTypedProgramLinks(packet.network,packet.root);});assert.deepEqual(programs[0].declarations.get('length').sourceSignature,programs[1].declarations.get('length').sourceSignature);
});
test('a source project preserves field definitions and every original concrete graph',()=>{
 const {inputs,entry,options}=fixture.project,packet=createRustSourceProjectProgramLinks(inputs,entry,options);
 for(const network of views(packet)){const program=decodeTypedProgramLinks(network,packet.root),signature=program.declarations.get('values.Value').sourceFieldSignatures;assert.equal(signature.dataTypeName,'values.Value');assert.equal(signature.ctors[0].fields[0].owner.path[0],'usize');assert.deepEqual(readRustSourceProjectConcreteGraphs(network,packet.root).map(({path,source})=>({path,source})),inputs);}
});
