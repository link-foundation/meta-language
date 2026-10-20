// Preserve every command and deadline while distributing all grammar files.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

/** Capture the complete three-to-six grammar test scheduling inverse. */
export function generateGrammarTestPartitionHistory({check=false}={}) {
  const root=new URL('../../',import.meta.url),path='.github/workflows/js.yml';
  const revision='394103f912aaac8e2f69147519ab97c6c5107528';
  const original=captureGitArtifactCertificate(revision,path),source=original.blob.toString('utf8');
  const rows=count=>Array.from({length:count},(_,index)=>
    '          - {group: grammar, partition: '+(index+1)+', shards: '+count+'}\n').join('');
  const before=rows(3),after=rows(6);assert.equal(source.split(before).length,2);
  const expected=source.replace(before,after),actual=readFileSync(new URL(path,root),'utf8');
  assert.equal(actual,expected,'only the complete grammar matrix may change');
  const start=Buffer.byteLength(source.slice(0,source.indexOf(before)));
  const value={schemaVersion:1,revision,path,source,certificate:original.certificate,
    current:{bytes:Buffer.byteLength(actual),sha256:createHash('sha256').update(actual).digest('hex')},
    edit:{offsetUnit:'UTF-8 bytes',start,end:start+Buffer.byteLength(after),removed:before,inserted:after},
    boundary:'Every grammar test runs exactly once in six required cells, with the original commands, assertions, fail-fast setting and fifteen-minute deadline. The preceding source and complete scheduling inverse are independently certified. Corrected CI timing and native behavior remain unverified.'};
  const file=new URL('parity/self-translation/grammar-test-partition-history.lino',root);
  if(check)checkProjectData(file,value);else writeProjectData(file,value);
  const reader=new URL('./grammar-test-partition-history.mjs',import.meta.url),current=readFileSync(reader,'utf8');
  const next=current.replace(/const archiveHash='(?:pending|[a-f0-9]{64})';/u,
    "const archiveHash='"+createHash('sha256').update(readFileSync(file)).digest('hex')+"';");
  if(check)assert.equal(current,next);else writeFileSync(reader,next);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateGrammarTestPartitionHistory({check:process.argv.includes('--check')});
}
