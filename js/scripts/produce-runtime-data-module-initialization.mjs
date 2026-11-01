// Future producer only. Every literal input remains represented, including
// unavailable products; finish a valid envelope before returning failure.
import {once} from 'node:events';
import {runtimeDataModuleModelCases as models,runtimeDataModuleOriginals as cases,runtimeDataModuleRefusals as refusals} from '../tests/fixtures/runtime-data-module-initialization.mjs';
import {readRuntimeDataModelDiagnostic,readRuntimeDataOriginalDiagnostic,readRuntimeDataRefusalDiagnostic} from '../tests/support/runtime-data-module-controls.js';
import {createControlDiagnostics} from '../tests/support/control-diagnostics.js';
import {readActiveRuntimeDataVersions} from '../src/translation/runtime-data-module.js';
const model='actual runtime Links policy initialization controls 1',diagnostic=createControlDiagnostics();diagnostic.attempt('active Node/dependency qualification',readActiveRuntimeDataVersions);let failures=diagnostic.failures.length;
async function write(value){if(!process.stdout.write(value))await once(process.stdout,'drain');}
await write(JSON.stringify({model,nodeVersion:process.versions.node}).slice(0,-1));
for(const group of [{key:'models',rows:models,read:readRuntimeDataModelDiagnostic},{key:'cases',rows:cases,read:readRuntimeDataOriginalDiagnostic},{key:'refusals',rows:refusals,read:readRuntimeDataRefusalDiagnostic}]){
 await write(','+JSON.stringify(group.key)+':[');let first=true;
 for(const definition of group.rows){
  let record;try{record=await group.read(definition);if(!record||!Array.isArray(record.diagnostics?.assertions)||!Array.isArray(record.diagnostics?.failures))throw new TypeError('a control must retain all assertion diagnostics');}
  catch(error){record={definition,diagnostics:{assertions:['control construction'],failures:[{assertion:'control construction',name:String(error?.name??'Error').slice(0,128),message:String(error?.message??error).slice(0,4096)}]}};}
  let encoded;try{encoded=JSON.stringify(record);}catch(error){record={definition,diagnostics:{assertions:[...record.diagnostics.assertions,'control serialization'],failures:[...record.diagnostics.failures,{assertion:'control serialization',message:String(error?.message??error).slice(0,4096)}]}};encoded=JSON.stringify(record);}
  if(encoded.length>16*1024*1024){record={definition,diagnostics:{assertions:[...record.diagnostics.assertions,'bounded control serialization'],failures:[...record.diagnostics.failures,{assertion:'bounded control serialization',message:'case exceeds 16 MiB; actual product unavailable at the declared transport bound'}]}};encoded=JSON.stringify(record);}
  failures+=record.diagnostics.failures.length;await write((first?'':',')+encoded);first=false;
 }
 await write(']');
}
await write(',"diagnostics":'+JSON.stringify({assertions:diagnostic.assertions,failures:diagnostic.failures})+'}\n');
if(failures){process.stderr.write(model+': '+failures+' declared assertions failed; every literal input and diagnostics are retained\n');process.exitCode=1;}
