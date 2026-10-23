// Normal writer for the exact workflow change, independent of runtime execution.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {CI_TEST_PARTITION_REVISION,CI_TEST_PARTITION_HISTORY,CI_TEST_PARTITION_CHANGES,deriveCiTestPartitionWorkflow,verifyCiTestPartitionHistory} from './ci-test-partition-history.mjs';
const root=new URL('../../',import.meta.url),pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
export function generateCiTestPartitionHistory({check=false,apply=false}={}){
 assert.equal(check&&apply,false);const file='.github/workflows/js.yml',captured=captureGitArtifactCertificate(CI_TEST_PARTITION_REVISION,file);
 const source=captured.blob.toString('utf8');assert.deepEqual(Buffer.from(source),captured.blob);
 const current=deriveCiTestPartitionWorkflow(source),actual=readFileSync(new URL(file,root),'utf8');assert.ok(actual===source||actual===current,'unknown workflow changes refuse');
 const feedbackFile='js/scripts/translation-test-feedback.mjs',previousFeedback=captureGitArtifactCertificate(CI_TEST_PARTITION_REVISION,feedbackFile);
 const oldFeedback=previousFeedback.blob.toString('utf8'),currentFeedback=readFileSync(new URL(feedbackFile,root),'utf8');
 const feedback={file:feedbackFile,previous:{source:oldFeedback,...pin(oldFeedback),certificate:previousFeedback.certificate},current:{source:currentFeedback,...pin(currentFeedback)}};
 const packet={schemaVersion:1,model:'complete CI physical test partition succession 1',revision:CI_TEST_PARTITION_REVISION,run:39839936265,file,
  previous:{source,...pin(source),certificate:captured.certificate},current:{source:current,...pin(current)},changes:CI_TEST_PARTITION_CHANGES,feedback,
  boundary:'Physical twelve-cell translation and eight-cell conformance scheduling, plus two physical report parts per original logical report cell. Each pair retains exactly one complete original logical partition. Tests execute serially within each runner under the unchanged fifteen-minute deadline; reports retain their thirty-minute deadline and two-worker bound. Every original assertion, test file, complete report module, feedback payload and failure predicate remains mandatory. Twelve exact-commit feedback partitions and every report module must merge without missing, duplicate or foreign scope. Source conservation is not test execution or requirement completion.'};
 verifyCiTestPartitionHistory(packet);const output=new URL(CI_TEST_PARTITION_HISTORY,root);
 if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
 const helper=new URL('js/scripts/ci-test-partition-history.mjs',root),text=readFileSync(helper,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;
 assert.equal(text.split(pattern).length,2);const expected=text.replace(pattern,"const packetSha256='"+pin(readFileSync(output)).sha256+"';");
 if(check){assert.equal(actual,current);assert.equal(text,expected);}else{writeFileSync(helper,expected);if(apply)writeFileSync(new URL(file,root),current);}
 return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 assert.ok(process.argv.slice(2).every(value=>['--check','--apply'].includes(value)));
 generateCiTestPartitionHistory({check:process.argv.includes('--check'),apply:process.argv.includes('--apply')});console.log('Preserved complete logical test cohorts and unchanged deadlines across smaller physical partitions.');
}
