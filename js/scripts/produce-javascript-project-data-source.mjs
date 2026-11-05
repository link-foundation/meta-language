// Future diagnostic producer. Source plans remain authenticated INPUT assets;
// their presence does not imply a checked portable Core/four-target program.
import {once} from 'node:events';
import {projectDataSourceOriginals as cases,projectDataSourceCacheOriginals as cacheCases,projectDataSourceRefusals as refusals,projectDataSourceRequiredControls as requirements} from '../tests/fixtures/project-data-source-execution.mjs';
import {readProjectDataSourceOriginalDiagnostic,readProjectDataSourceCacheDiagnostic,readProjectDataSourceRefusalDiagnostic} from '../tests/support/project-data-source-controls.js';
import {createControlDiagnostics} from '../tests/support/control-diagnostics.js';
import {readActiveRuntimeDataVersions} from '../src/translation/runtime-data-module.js';
import {runtimeDataModuleOriginals as importerCases} from '../tests/fixtures/runtime-data-module-initialization.mjs';
import {readRuntimeDataOriginalDiagnostic} from '../tests/support/runtime-data-module-controls.js';
const model='actual project data source execution controls 1',diagnostic=createControlDiagnostics();diagnostic.attempt('actual Node/shared decoder version qualification',readActiveRuntimeDataVersions);let failures=diagnostic.failures.length;
async function write(text){if(!process.stdout.write(text))await once(process.stdout,'drain');}
await write(JSON.stringify({model,nodeVersion:process.versions.node,requirements}).slice(0,-1));
for(const group of [{key:'cases',rows:cases,read:readProjectDataSourceOriginalDiagnostic},{key:'cacheCases',rows:cacheCases,read:readProjectDataSourceCacheDiagnostic},{key:'importerCases',rows:importerCases,read:row=>readRuntimeDataOriginalDiagnostic(row,{readerProfile:'actual project data source functions 1',readerRealmIdentity:2n})},{key:'refusals',rows:refusals,read:readProjectDataSourceRefusalDiagnostic}]){
 await write(','+JSON.stringify(group.key)+':[');let first=true;
 for(const definition of group.rows){
  let record;try{record=await group.read(definition);if(!record||!Array.isArray(record.diagnostics?.assertions)||!Array.isArray(record.diagnostics?.failures))throw new TypeError('a source control retains all declared assertion diagnostics');}
  catch(error){record={definition,diagnostics:{assertions:['control construction'],failures:[{assertion:'control construction',name:String(error?.name??'Error').slice(0,128),message:String(error?.message??error).slice(0,4096)}]}};}
  let encoded;try{encoded=JSON.stringify(record);}catch(error){record={definition,diagnostics:{assertions:[...record.diagnostics.assertions,'control serialization'],failures:[...record.diagnostics.failures,{assertion:'control serialization',message:String(error?.message??error).slice(0,4096)}]}};encoded=JSON.stringify(record);}
  if(encoded.length>16*1024*1024){record={definition,diagnostics:{assertions:[...record.diagnostics.assertions,'bounded control serialization'],failures:[...record.diagnostics.failures,{assertion:'bounded control serialization',message:'actual source products unavailable at explicit 16 MiB per-case transport bound'}]}};encoded=JSON.stringify(record);}
  failures+=record.diagnostics.failures.length;await write((first?'':',')+encoded);first=false;
 }
 await write(']');
}
await write(',"diagnostics":'+JSON.stringify({assertions:diagnostic.assertions,failures:diagnostic.failures})+'}\n');
if(failures){process.stderr.write(model+': '+failures+' assertions failed; every original/refusal input is retained\n');process.exitCode=1;}
