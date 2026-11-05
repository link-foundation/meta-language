// Deferred actual runtime products. Never execute during source drafting.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {createUnsignedSourceDocumentLinks,encodeUnsignedLinkNetworkBinary} from '../../js/src/unsigned-link-network.js';
import {createUnsignedTypedProgramLinks,createUnsignedRustSourceOwnerProgramLinks,emitUnsignedTypedProgramLinks} from '../../js/src/unsigned-typed-program-links.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-source-documents.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new TypeError('uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const documents=fixture.documents.map(row=>{const packet=createUnsignedSourceDocumentLinks(row.source,row.language,undefined,{firstIdentity:fixture.firstIdentity});return{...row,text:packet.network.toLino(),bytes:Array.from(encodeUnsignedLinkNetworkBinary(packet.network)),sourceIdentities:packet.sourceIdentities.map(String),identities:packet.identities.map(String)};});
const programs=fixture.programs.map(row=>{const packet=row.factory==='Rust source owners'?createUnsignedRustSourceOwnerProgramLinks(row.source,{}, {firstIdentity:fixture.firstIdentity}):createUnsignedTypedProgramLinks(row.source,row.language,{}, {firstIdentity:fixture.firstIdentity});return{...row,root:String(packet.root),text:packet.network.toLino(),bytes:Array.from(encodeUnsignedLinkNetworkBinary(packet.network)),sourceIdentities:packet.sourceIdentities.map(String),identities:packet.identities.map(String),emissions:['JavaScript','Rust','Lean','Rocq'].map(target=>({target,code:emitUnsignedTypedProgramLinks(packet.network,packet.root,target).code}))};});
process.stdout.write(structuredDataToLino({schemaVersion:1,documents,programs}));
