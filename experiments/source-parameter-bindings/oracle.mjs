// Deferred original/renamed Node oracle. Source authors must never import it.
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
export function observeSourceParameterProgram(row,source){
 const [major,minor]=process.versions.node.split('.').map(Number);
 if(!(major===22&&minor>=15||major===24))throw new TypeError('uncovered Node '+process.versions.node);
 return {value:String(runInNewContext('"use strict";\n'+row.context+'\n'+source+'\n'+row.probe,{}, {timeout:1000}))};
}
if(process.argv[1]&&pathToFileURL(process.argv[1]).href===import.meta.url){
 const fixture=readProjectData(new URL('../../parity/fixtures/source-parameter-bindings.lino',import.meta.url)),index=Number(process.argv[2]);
 if(!Number.isSafeInteger(index)||index<0||index>=fixture.cases.length)throw new TypeError('an actual fixture case is required');
 const row=fixture.cases[index],source=process.argv[3]?readFileSync(process.argv[3],'utf8'):row.source;
 process.stdout.write(structuredDataToLino(observeSourceParameterProgram(row,source)));
}
