// Normal source-only author; no parsing/translation/report/native execution.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {RELATIVE_ARRAY_CURRENT_FRONTEND,RELATIVE_ARRAY_CURRENT_FRONTEND_OUTPUT,
  RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS,RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING,
  pinRelativeArrayCurrentFrontend,deriveRelativeArrayFrontendIntervals,
  verifyRelativeArrayCurrentFrontendHistory} from './relative-array-current-frontend-history.mjs';
const root=new URL('../../',import.meta.url);
const helper='js/scripts/relative-array-current-frontend-history.mjs';

export function generateRelativeArrayCurrentFrontendHistory({check=false}={}){
  const original=RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS[0];
  let bytes=readFileSync(new URL(RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING.path,root));
  assert.deepEqual(pinRelativeArrayCurrentFrontend(bytes),{bytes:RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING.bytes,sha256:RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING.sha256});
  let archive=readBootstrapStructuredDataLino(bytes.toString('utf8'));
  assert.equal(archive.revision,original.revision);
  const owners=archive.sources.filter(row=>row.file===RELATIVE_ARRAY_CURRENT_FRONTEND);
  assert.equal(owners.length,1);
  const owner=owners[0];
  assert.equal(owner.commit,original.revision);
  assert.deepEqual(pinRelativeArrayCurrentFrontend(owner.sourceText),{bytes:original.bytes,sha256:original.sha256});
  verifyGitArtifactCertificate(owner.certificate,Buffer.from(owner.sourceText),{revision:original.revision,path:RELATIVE_ARRAY_CURRENT_FRONTEND});
  const first={...original,source:owner.sourceText,certificate:structuredClone(owner.certificate)};
  archive=null;bytes=null;global.gc?.();
  const committed=RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS[1];
  const captured=captureGitArtifactCertificate(committed.revision,RELATIVE_ARRAY_CURRENT_FRONTEND);
  assert.deepEqual(Buffer.from(captured.blob.toString('utf8')),captured.blob);
  assert.deepEqual(pinRelativeArrayCurrentFrontend(captured.blob),{bytes:committed.bytes,sha256:committed.sha256});
  const second={...committed,source:captured.blob.toString('utf8'),certificate:captured.certificate};
  const source=readFileSync(new URL(RELATIVE_ARRAY_CURRENT_FRONTEND,root),'utf8');
  const current={...pinRelativeArrayCurrentFrontend(source),source};
  const sources=[first.source,second.source,current.source];
  const edges=sources.slice(0,-1).map((source,index)=>({from:pinRelativeArrayCurrentFrontend(source),
    to:pinRelativeArrayCurrentFrontend(sources[index+1]),coordinateUnit:'UTF-16 code units',
    edits:deriveRelativeArrayFrontendIntervals(source,sources[index+1])}));
  const packet={schemaVersion:1,model:'relative array complete current frontend succession 1',profile:'relative array',file:RELATIVE_ARRAY_CURRENT_FRONTEND,
    sourceArchive:{...RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING},anchors:[first,second],current,edges,
    boundary:'Only the exact current complete frontend source view used by the immutable relative-array descriptor reader. Actual current parsing, source/CST/semantic checks, native execution and all older packet, descriptor, receipt and assertion bytes remain independent. Two immutable Git memberships and complete finite inverses establish source succession, never semantic equivalence. No claim that the current uncommitted source belongs to either original commit.'};
  verifyRelativeArrayCurrentFrontendHistory(packet);
  const output=new URL(RELATIVE_ARRAY_CURRENT_FRONTEND_OUTPUT,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const path=new URL(helper,root),text=readFileSync(path,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(text.split(pattern).length,2);
  const expected=text.replace(pattern,"const packetSha256='"+pinRelativeArrayCurrentFrontend(readFileSync(output)).sha256+"';");
  if(check)assert.equal(text,expected);else writeFileSync(path,expected);
  return packet;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const packet=generateRelativeArrayCurrentFrontendHistory({check:process.argv.includes('--check')});
  console.log(JSON.stringify({profile:packet.profile,anchors:packet.anchors.map(({revision,bytes,sha256})=>({revision,bytes,sha256})),current:pinRelativeArrayCurrentFrontend(packet.current.source),nativeCompilation:false,nativeExecution:false}));
}
