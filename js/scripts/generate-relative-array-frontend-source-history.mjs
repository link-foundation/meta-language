// Project one complete already-certified historical module; reuse the current
// raw-source authority and capture only the finite historical reader adapter.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {readOwnCodebaseHistoricalNamingPolicy} from './own-codebase-baseline-naming-history.mjs';
import {readOwnCodebaseRawDescriptorPolicy} from './own-codebase-raw-source-descriptors.mjs';
import {restoreRelativeArrayCurrentFrontendSource} from './relative-array-current-frontend-history.mjs';
import {restoreReferenceIdentityHistoricalSource} from './reference-identity-source-history.mjs';
import {RELATIVE_ARRAY_FRONTEND_PROFILE,RELATIVE_ARRAY_FRONTEND_HOST,relativeArrayFrontendPin,
  replayRelativeArrayFrontendHost,readAuthenticatedCurrentRelativeArrayFrontend,
  verifyRelativeArrayFrontendSourceHistory,RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE,replayRelativeArrayNullableInference} from './relative-array-frontend-source-history.mjs';
const root=new URL('../../',import.meta.url),output='parity/self-translation/relative-array-frontend-source-history.lino';
const helper='js/scripts/relative-array-frontend-source-history.mjs';

export function generateRelativeArrayFrontendSourceHistory({check=false,apply=false}={}){
  assert.equal(check&&apply,false);
  const naming=readOwnCodebaseHistoricalNamingPolicy(),policy=readOwnCodebaseRawDescriptorPolicy();
  const archiveBytes=readFileSync(new URL(naming.sourceArchive.path,root));
  assert.deepEqual(relativeArrayFrontendPin(archiveBytes),{bytes:naming.sourceArchive.bytes,sha256:naming.sourceArchive.sha256});
  let archive=readBootstrapStructuredDataLino(archiveBytes.toString('utf8'));
  assert.equal(archive.revision,naming.revision);assert.equal(archive.sources.length,naming.sourceArchive.sources);
  const selected=archive.sources.filter(row=>row.file===policy.sourceFile);assert.equal(selected.length,1);
  const row=selected[0],historical={file:row.file,commit:row.commit,bytes:row.bytes,sha256:row.sha256,
    sourceText:row.sourceText,certificate:structuredClone(row.certificate)};
  archive=null;global.gc?.();
  assert.equal(restoreRelativeArrayCurrentFrontendSource(readFileSync(new URL(policy.sourceFile,root),'utf8'),policy.sourceFile,RELATIVE_ARRAY_FRONTEND_PROFILE),readAuthenticatedCurrentRelativeArrayFrontend());
  const inferenceOwner=RELATIVE_ARRAY_NULLABLE_INFERENCE_SOURCE;
  const inferenceCapture=captureGitArtifactCertificate(inferenceOwner.revision,inferenceOwner.path);
  const inferenceSource=inferenceCapture.blob.toString('utf8');assert.deepEqual(Buffer.from(inferenceSource),inferenceCapture.blob);
  const inferenceReplay=replayRelativeArrayNullableInference(inferenceSource);
  assert.equal(restoreReferenceIdentityHistoricalSource(readFileSync(new URL(inferenceOwner.path,root),'utf8'),inferenceOwner.path,'relative array inference'),inferenceReplay.source,'capture refuses unknown nullable-array source successors');
  const captured=captureGitArtifactCertificate(RELATIVE_ARRAY_FRONTEND_HOST.revision,RELATIVE_ARRAY_FRONTEND_HOST.path);
  const sourceText=captured.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),captured.blob);
  const replay=replayRelativeArrayFrontendHost(sourceText);
  const history={schemaVersion:2,model:'relative array frontend descriptor historical source 2',profile:RELATIVE_ARRAY_FRONTEND_PROFILE,
    sourceArchive:{revision:naming.revision,...naming.sourceArchive},historical,
    currentAuthority:{revision:policy.revision,file:policy.sourceFile,predecessor:policy.sourceOwner.predecessor,current:policy.sourceOwner.current},
    nullableInference:{path:inferenceOwner.path,revision:inferenceOwner.revision,...relativeArrayFrontendPin(inferenceSource),
      sourceText:inferenceSource,certificate:inferenceCapture.certificate,current:relativeArrayFrontendPin(inferenceReplay.source),edits:inferenceReplay.edits},
    host:{path:RELATIVE_ARRAY_FRONTEND_HOST.path,revision:RELATIVE_ARRAY_FRONTEND_HOST.revision,
      ...relativeArrayFrontendPin(sourceText),sourceText,certificate:captured.certificate,current:relativeArrayFrontendPin(replay.source),edits:replay.edits},
    boundary:'One explicit historical descriptor view for the relative-array generator only. The complete original frontend retains its independently certified897bfc source context; the distinct current frontend is authenticated through the existing checked raw-source descriptor receipt. A second complete certified7881 inference source has one finite nullable-array owner insertion whose inverse is used only in this same descriptor scope. The actual current inference fix remains independently exercised; no source behavior equivalence is inferred. Every original relative-array descriptor, receipt, native definition,24-source inventory, fixture and two-change assertion stays exact. Actual current parsing, emission, source descriptors and native CI remain independent.'};
  verifyRelativeArrayFrontendSourceHistory(history);
  const host=new URL(RELATIVE_ARRAY_FRONTEND_HOST.path,root),actual=readFileSync(host,'utf8');
  if(check)assert.equal(actual,replay.source);else assert.ok(actual===sourceText||actual===replay.source,'unknown host source is never overwritten');
  if(check)checkProjectData(new URL(output,root),history);else writeProjectData(new URL(output,root),history);
  const helperPath=new URL(helper,root),current=readFileSync(helperPath,'utf8'),pattern=/const historySha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(current.split(pattern).length,2);
  const expected=current.replace(pattern,"const historySha256='"+relativeArrayFrontendPin(readFileSync(new URL(output,root))).sha256+"';");
  if(check)assert.equal(current,expected);else writeFileSync(helperPath,expected);
  if(apply)writeFileSync(host,replay.source);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.equal(new Set(args).size,args.length);assert.ok(args.every(value=>['--check','--apply'].includes(value)));
  const history=generateRelativeArrayFrontendSourceHistory({check:args.includes('--check'),apply:args.includes('--apply')});
  console.log(JSON.stringify({profile:history.profile,file:history.historical.file,historical:history.historical.sha256,
    current:history.currentAuthority.current.sha256,nativeCompilation:false,nativeExecution:false}));
}
