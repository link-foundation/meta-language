import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

/** Record only the complete reviewed three-edit scheduling successor. */
export function generateRawPlanningWorkflowHistory({check=false}={}) {
  const root=new URL('../../',import.meta.url),revision='76e27c44c5854a2cb547aea5cef06ea80adf729c',file='.github/workflows/js.yml';
  const original=captureGitArtifactCertificate(revision,file),source=original.blob.toString('utf8');
  const matrix='          - group: source-model-raw\n            generated_slice: first\n';
  assert.equal(source.split(matrix).length,2);
  let expected=source.replace(matrix,matrix+'          - group: source-model-raw-generation\n            generated_slice: first\n');
  for(const name of ['Check complete original link raw projection native generation','Run complete original link raw projection checks']){
    const start=expected.indexOf('      - name: '+name+'\n'),end=expected.indexOf('\n      - name:',start+1);
    assert.ok(start>=0&&end>start);const step=expected.slice(start,end);
    assert.equal(step.split("matrix.group == 'source-model-raw'").length,2);
    expected=expected.slice(0,start)+step.replace("matrix.group == 'source-model-raw'","matrix.group == 'source-model-raw-generation'")+expected.slice(end);
  }
  const actual=readFileSync(new URL(file,root));assert.equal(actual.toString(),expected);
  let start=0,end=actual.length,oldEnd=original.blob.length;
  while(start<end&&start<oldEnd&&actual[start]===original.blob[start])start++;
  while(start>0&&(actual[start]&192)===128)start--;
  while(end>start&&oldEnd>start&&actual[end-1]===original.blob[oldEnd-1]){end--;oldEnd--;}
  while(end<actual.length&&((actual[end]&192)===128||(original.blob[oldEnd]&192)===128)){end++;oldEnd++;}
  const value={schemaVersion:1,revision,file,source,certificate:original.certificate,
    current:{bytes:actual.length,sha256:createHash('sha256').update(actual).digest('hex')},
    edit:{offsetUnit:'UTF-8 bytes',start,end,removed:original.blob.subarray(start,oldEnd).toString(),inserted:actual.subarray(start,end).toString()},
    boundary:'One required generation/projection cell and two scheduling predicates are the only changes. Every command, source, assertion, test gate and ten-minute deadline remains exact. The complete original workflow and immediate byte inverse are authenticated independently; this receipt proves neither new timing nor native execution.'};
  const output=new URL('parity/self-translation/raw-planning-workflow-history.lino',root);
  if(check)checkProjectData(output,value);else writeProjectData(output,value);
  const helper=new URL('js/scripts/raw-planning-workflow-history.mjs',root),text=readFileSync(helper,'utf8');
  const updated=text.replace(/const archiveHash='(?:PENDING|[0-9a-f]{64})';/u,
    "const archiveHash='"+createHash('sha256').update(readFileSync(output)).digest('hex')+"';");
  if(check)assert.equal(text,updated);else writeFileSync(helper,updated);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRawPlanningWorkflowHistory({check:process.argv.includes('--check')});
}
