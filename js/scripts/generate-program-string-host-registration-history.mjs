// Normal source membership capture only. This invokes no Rust compiler/test.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {PROGRAM_STRING_REGISTRATION_REVISION,PROGRAM_STRING_HOST_REVISION,PROGRAM_STRING_HOST_ARCHIVE,
  PROGRAM_STRING_HOST_PRODUCER,PROGRAM_STRING_REGISTRATION_CHANGES,PROGRAM_STRING_REGISTRATIONS,
  PROGRAM_STRING_HOST_PRODUCER_CHANGES,programStringHostRegistrationPin,replayProgramStringRegistrationChanges,
  verifyProgramStringHostRegistrationHistory} from './program-string-host-registration-history.mjs';
const root=new URL('../../',import.meta.url);
const output='parity/self-translation/program-string-host-registration-history.lino';
const helper='js/scripts/program-string-host-registration-history.mjs';

export function generateProgramStringHostRegistrationHistory({check=false}={}){
  const archived=captureGitArtifactCertificate(PROGRAM_STRING_REGISTRATION_REVISION,PROGRAM_STRING_HOST_ARCHIVE.path);
  assert.deepEqual(archived.blob,readFileSync(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root)),'all nine frozen original host rows stay byte exact');
  const previous=readProjectData(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root));
  const producerOriginal=captureGitArtifactCertificate(PROGRAM_STRING_REGISTRATION_REVISION,PROGRAM_STRING_HOST_PRODUCER.path);
  const producerSource=producerOriginal.blob.toString('utf8');assert.deepEqual(Buffer.from(producerSource),producerOriginal.blob);
  const producerReplay=replayProgramStringRegistrationChanges(producerSource,PROGRAM_STRING_HOST_PRODUCER_CHANGES);
  assert.equal(readFileSync(new URL(PROGRAM_STRING_HOST_PRODUCER.path,root),'utf8'),producerReplay.source);
  const sources=PROGRAM_STRING_REGISTRATION_CHANGES.map(specification=>{
    const row=previous.sources.find(row=>row.file===specification.path);assert.ok(row);
    const captured=captureGitArtifactCertificate(PROGRAM_STRING_REGISTRATION_REVISION,specification.path);
    const source=captured.blob.toString('utf8');assert.deepEqual(Buffer.from(source),captured.blob);
    const replay=replayProgramStringRegistrationChanges(row.proposed.source,specification.changes);
    assert.equal(source,replay.source,'only the finite registration additions belong to this committed successor');
    assert.equal(readFileSync(new URL(specification.path,root),'utf8'),source,'actual current source must still equal the certified committed source');
    return{path:specification.path,predecessor:{source:row.proposed.source,...programStringHostRegistrationPin(row.proposed.source)},
      current:{source,...programStringHostRegistrationPin(source),certificate:captured.certificate},edits:replay.edits};
  });
  const registrations=PROGRAM_STRING_REGISTRATIONS.map(specification=>{
    const captured=captureGitArtifactCertificate(PROGRAM_STRING_REGISTRATION_REVISION,specification.path);
    assert.deepEqual(captured.blob,readFileSync(new URL(specification.path,root)));
    return{...specification,...programStringHostRegistrationPin(captured.blob),certificate:captured.certificate};
  });
  const packet={schemaVersion:1,model:'program string host registration succession 1',revision:PROGRAM_STRING_REGISTRATION_REVISION,
    hostRevision:PROGRAM_STRING_HOST_REVISION,coordinateUnit:'UTF-16 code units',
    hostArchive:{...PROGRAM_STRING_HOST_ARCHIVE,certificate:archived.certificate},
    producer:{path:PROGRAM_STRING_HOST_PRODUCER.path,source:producerSource,...programStringHostRegistrationPin(producerSource),
      certificate:producerOriginal.certificate,current:programStringHostRegistrationPin(producerReplay.source),edits:producerReplay.edits},
    sources,registrations,
    boundary:'Complete committed source membership and finite byte inverses only. Every original program-string host definition, test, source action and assertion remains in the separately pinned unchanged nine-host archive. Current native registrations are proved separately against actual module roots and actual registered files. This authenticates no new runtime behavior, native compilation/execution, grammar coverage or general source equivalence.'};
  verifyProgramStringHostRegistrationHistory(packet);
  if(check)checkProjectData(new URL(output,root),packet);else writeProjectData(new URL(output,root),packet);
  const helperPath=new URL(helper,root),source=readFileSync(helperPath,'utf8'),pattern=/const policySha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(source.split(pattern).length,2);
  const expected=source.replace(pattern,"const policySha256='"+programStringHostRegistrationPin(readFileSync(new URL(output,root))).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helperPath,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const packet=generateProgramStringHostRegistrationHistory({check:process.argv.includes('--check')});
  console.log(JSON.stringify({sourceRoots:packet.sources.map(row=>row.path),registrations:packet.registrations.length,nativeCompilation:false,nativeExecution:false}));
}
