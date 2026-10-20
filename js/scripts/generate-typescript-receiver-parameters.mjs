// Replay recorded host CST adapters; pure decisions generate Rust separately.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreCiLintSource} from './ci-39496927203-lint-source-history.mjs';

/** Rebuild complete receiver erasure hosts from certified predecessor sources. */
export function generateTypeScriptReceiverParameters({check=false}={}) {
  const root=new URL('../../',import.meta.url);
  const archive=readProjectData(new URL('parity/self-translation/typescript-receiver-parameter-hosts.lino',root));
  assert.equal(archive.schemaVersion,1);
  assert.equal(archive.revision,'394103f912aaac8e2f69147519ab97c6c5107528');
  assert.equal(archive.hosts.length,8);
  assert.equal(new Set(archive.hosts.map(row=>row.path)).size,archive.hosts.length);
  for(const row of archive.hosts){
    assert.ok(/^(?:js\/src\/|rust\/src\/|parity\/self-translation\/)[a-zA-Z0-9_./-]+$/u.test(row.path));
    assert.equal(row.path.includes('..'),false);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:archive.revision,path:row.path});
    let generated=row.source;
    for(const edit of row.edits){
      assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=generated.length);
      assert.equal(generated.slice(edit.start,edit.start+edit.removed.length),edit.removed);
      generated=generated.slice(0,edit.start)+edit.inserted+generated.slice(edit.start+edit.removed.length);
    }
    assert.equal(createHash('sha256').update(generated).digest('hex'),row.currentSha256,row.path);
    let inverse=generated;
    for(const edit of [...row.edits].reverse()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.source);
    const file=new URL(row.path,root),actual=readFileSync(file,'utf8');
    const current=restoreCiLintSource(actual,row.path);
    if(check)assert.equal(current,generated,row.path);
    else {assert.ok(current===row.source||current===generated,'unknown current source '+row.path);if(actual===current)writeFileSync(file,generated);}
  }
  return archive;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateTypeScriptReceiverParameters({check:process.argv.includes('--check')});
}
