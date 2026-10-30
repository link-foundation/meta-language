import {createProjectTranslationArchive,readProjectTranslationArchive} from '../../js/src/project-translation-archive.js';
// Deferred paired products only. Never import during all-requirements authoring.
import {readProjectData} from '../../js/src/project-data-files.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures,selfTranslateWithDiagnostics} from '../../js/src/self-translation.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-versions.lino',import.meta.url));
const rows=[];
for(const semanticTranslation of [false,true])for(const version of fixture.sourceVersions)for(const row of fixture.cases){
 const options={semanticTranslation,rustSourceVersion:version};
 rows.push({name:row.name,source:row.source,version,semanticTranslation,expected:row.expectedByVersion[version],translation:selfTranslate(row.source,'Rust','JavaScript',options),report:selfTranslateWithDiagnostics(row.source,'Rust','JavaScript',options),signatures:semanticTranslation?selfTranslationSignatures(row.source,'Rust',options):null});
}
const projectRows=[];
for(const version of fixture.sourceVersions){
 const inputs=[{identifier:'lib.rs',language:'Rust',source:fixture.cases[0].source,options:{rustSourceVersion:version}},{identifier:'plain.js',language:'JavaScript',source:'export function plain(){return true;}'}],options={targetLanguage:'JavaScript'};
 const packet=createProjectTranslationArchive(inputs,options),view=readProjectTranslationArchive(packet.network,packet.root);
 projectRows.push({version,inputs,options,draft:view.draft,configuration:view.configuration});
}
process.stdout.write(structuredDataToLino({schemaVersion:1,rows,projectRows}));
