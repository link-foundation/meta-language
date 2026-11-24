// Future paired API products. This producer records no execution observations.
import {readProjectData} from '../../js/src/project-data-files.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {composeRustSourceProject,createRustSourceProjectProgramLinks} from '../../js/src/rust-source-project.js';
import {emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-project.lino',import.meta.url));
const cases=fixture.cases.map(row=>{const composition=composeRustSourceProject(row.inputs,row.entry,row.options),packet=createRustSourceProjectProgramLinks(row.inputs,row.entry,row.options);return {...row,composition,links:packet.network.toLino(),root:packet.root.asU64(),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]))};});
const refusals=fixture.refusals.map(row=>{try{composeRustSourceProject(row.inputs,row.entry,row.options);throw new Error('the source refusal was not observed');}catch(error){return {...row,diagnostic:{name:error.name,message:error.message}};}});
process.stdout.write(structuredDataToLino({schemaVersion:1,cases,refusals,planning:fixture.planning,versions:fixture.versions,boundary:'Actual public API products only. Original source compilation, target execution, graph replay and observations belong to the CI controls.'}));
