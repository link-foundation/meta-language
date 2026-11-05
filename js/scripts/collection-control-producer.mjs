// One bounded case is retained at a time. A valid diagnostic envelope is
// completed before a failing exit code is set; no partial JSON is evidence.
import {once} from 'node:events';
import {collectionCopyTargets as targets,collectionCopyTransports as transports} from '../tests/fixtures/collection-copies.mjs';
import {requireCollectionCopyNodeVersion,readCollectionCopyOriginalDiagnostic,readCollectionCopyRefusalDiagnostic} from '../tests/support/collection-copy-integration.js';
import {createControlDiagnostics,readControlDiagnosticFallback} from '../tests/support/control-diagnostics.js';
export async function produceCollectionControlFixture({model,originals,refusals,sourceAnchors=[],additionalGroups=[]}){
 const diagnostic=createControlDiagnostics();diagnostic.attempt('active source version',requireCollectionCopyNodeVersion);let failureCount=diagnostic.failures.length;
 async function write(value){if(!process.stdout.write(value))await once(process.stdout,'drain');}
 await write(JSON.stringify({model,nodeVersion:process.versions.node,targets,transports,sourceAnchors}).slice(0,-1));
 for(const group of [{key:'cases',rows:originals,read:readCollectionCopyOriginalDiagnostic},{key:'refusals',rows:refusals,read:readCollectionCopyRefusalDiagnostic},...additionalGroups]){
  await write(','+JSON.stringify(group.key)+':[');let first=true;
  for(const row of group.rows){
   let record=readControlDiagnosticFallback(row,()=>group.read(row)),encoded;
   try{encoded=JSON.stringify(record);}catch(error){record={...row,diagnostics:{assertions:[...(record.diagnostics?.assertions??[]),'diagnostic record serialization'],failures:[...(record.diagnostics?.failures??[]),{assertion:'diagnostic record serialization',message:String(error?.message??error).slice(0,4096)}]}};encoded=JSON.stringify(record);}
   failureCount+=record.diagnostics?.failures.length??0;await write((first?'':',')+encoded);first=false;
  }
  await write(']');
 }
 await write(',"diagnostics":'+JSON.stringify({assertions:diagnostic.assertions,failures:diagnostic.failures})+'}\n');
 if(failureCount>0){process.stderr.write(model+': '+failureCount+' declared control assertions failed; complete cells are in stdout JSON\n');process.exitCode=1;}
}
