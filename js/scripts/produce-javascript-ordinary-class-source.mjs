// Future bounded diagnostic producer. Every original/refusal and transport
// remains represented even when another generated/native product is missing.
import {once} from 'node:events';
import {javascriptOrdinaryClassOriginals as cases,javascriptOrdinaryClassRefusals as refusals} from '../tests/fixtures/javascript-ordinary-class-source.mjs';
import {readOrdinaryClassSourceOriginalDiagnostic,readOrdinaryClassSourceRefusalDiagnostic} from '../tests/support/ordinary-class-source-controls.js';
import {createControlDiagnostics} from '../tests/support/control-diagnostics.js';
import {readActiveRuntimeDataVersions} from '../src/translation/runtime-data-module.js';
import {javascriptOrdinaryClassClosureOriginals as closures,javascriptOrdinaryClassScopeWorkloads as workloads} from '../tests/fixtures/javascript-ordinary-class-closures.mjs';
import {readOrdinaryClassClosureOriginalDiagnostic} from '../tests/support/ordinary-class-closure-controls.js';
const model='actual ordinary class source execution controls 1',diagnostic=createControlDiagnostics();diagnostic.attempt('actual Node/shared source version profile',readActiveRuntimeDataVersions);let failures=diagnostic.failures.length;
async function write(text){if(!process.stdout.write(text))await once(process.stdout,'drain');}
await write(JSON.stringify({model,nodeVersion:process.versions.node}).slice(0,-1));
for(const group of [{key:'cases',rows:cases,read:readOrdinaryClassSourceOriginalDiagnostic},{key:'closures',rows:closures,read:readOrdinaryClassClosureOriginalDiagnostic},{key:'workloads',rows:workloads,read:readOrdinaryClassClosureOriginalDiagnostic},{key:'refusals',rows:refusals,read:readOrdinaryClassSourceRefusalDiagnostic}]){
 await write(','+JSON.stringify(group.key)+':[');let first=true;
 for(const definition of group.rows){
  let record;try{record=await group.read(definition);if(!record||!Array.isArray(record.diagnostics?.assertions)||!Array.isArray(record.diagnostics?.failures))throw new TypeError('every source control retains all assertion diagnostics');}
  catch(error){record={definition,diagnostics:{assertions:['control construction'],failures:[{assertion:'control construction',name:String(error?.name??'Error').slice(0,128),message:String(error?.message??error).slice(0,4096)}]}};}
  let encoded;try{encoded=JSON.stringify(record);}catch(error){record={definition,diagnostics:{assertions:[...record.diagnostics.assertions,'control serialization'],failures:[...record.diagnostics.failures,{assertion:'control serialization',message:String(error?.message??error).slice(0,4096)}]}};encoded=JSON.stringify(record);}
  if(encoded.length>16*1024*1024){record={definition,diagnostics:{assertions:[...record.diagnostics.assertions,'bounded source control serialization'],failures:[...record.diagnostics.failures,{assertion:'bounded source control serialization',message:'actual products unavailable at explicit 16 MiB per-case bound'}]}};encoded=JSON.stringify(record);}
  failures+=record.diagnostics.failures.length;await write((first?'':',')+encoded);first=false;
 }
 await write(']');
}
await write(',"diagnostics":'+JSON.stringify({assertions:diagnostic.assertions,failures:diagnostic.failures})+'}\n');
if(failures){process.stderr.write(model+': '+failures+' assertions failed; all original definitions retained\n');process.exitCode=1;}
