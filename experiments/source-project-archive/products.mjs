// Future public products only; no fixture observations or behavior claims.
import {readProjectData} from '../../js/src/project-data-files.js';
import {createSourceProjectArchive,readSourceProjectArchive,extendSourceProjectArchive} from '../../js/src/source-project-archive.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {encodeLinkNetworkBinary} from '../../js/src/link-network-binary.js';
import {readMetaLanguageArchive,createMetaLanguageArchive} from '../../js/src/meta-language-archive.js';
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/source-project-archive.lino',import.meta.url));
function product(archive,options){const view=readSourceProjectArchive(archive.network,archive.root,options);return {root:archive.root.asU64(),links:archive.network.toLino(),directory:view.directory,sources:view.sources.map(file=>({path:file.path,language:file.language,source:file.source,root:file.root.asU64(),links:file.network.toLino(),binary:Array.from(encodeLinkNetworkBinary(file.network)),projection:file.projection,view:file.view.map(link=>link.id().asU64())}))};}
const cases=fixture.cases.map(row=>({...row,...product(createSourceProjectArchive(row.inputs,row.options),row.options)}));
const extensions=fixture.extensions.map(row=>{let before=createSourceProjectArchive(row.initial,row.options);const entries=readMetaLanguageArchive(before.network,before.root).entries,source=entries.find(entry=>entry.kind==='concrete syntax');source.network._conceptSyntax.set('snapshot marker\0JavaScript','preserved selected syntax');source.network._nextId+=11;before=createMetaLanguageArchive(entries);const after=extendSourceProjectArchive(before.network,before.root,row.added,row.options);return {...row,before:product(before,row.options),after:product(after,row.options)};});
process.stdout.write(structuredDataToLino({schemaVersion:1,cases,extensions,refusals:fixture.refusals,versions:fixture.versions,rootCases:fixture.rootCases,boundary:'Actual public graph products. Native controls must reconstruct and compare complete source graphs and states; no executed conformance observations are recorded here.'}));
