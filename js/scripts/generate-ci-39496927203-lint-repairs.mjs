// Reconstruct the exact reviewed CI lint successors; generated code stays generated.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const revision='24d7cf48778c7e91f15a25d563c225912d6caad0';
const paths=['rust/src/grammar/feature_runtime/grammar_selection.rs',
  'parity/self-translation/typescript-erasure-rules-decorators.lino',
  'rust/src/self_translation/typescript_erasure_rules.rs'];
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Retain complete certified inputs and replay only their reviewed source edits. */
export function generateCiLintRepairs({check=false}={}) {
  const root=new URL('../../',import.meta.url);
  const archive=readProjectData(new URL('parity/self-translation/ci-39496927203-lint-feedback.lino',root));
  assert.equal(archive.schemaVersion,1);assert.equal(archive.revision,revision);
  assert.deepEqual(archive.sources.map(row=>row.file),paths);
  assert.equal(archive.feedback.wholeRawStream.sha256,'d4ce865bd4f861c8db94835e2dcc92002f41c419c3c68fc16568ba659d19d756');
  assert.equal(digest(archive.feedback.excerpt.text),archive.feedback.excerpt.sha256);
  const outputs=archive.sources.map(row=>{
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision,path:row.file});
    assert.equal(digest(row.source),row.sha256);
    let current=row.source;
    for(const edit of row.edits){
      assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=current.length);
      assert.equal(current.slice(edit.start,edit.start+edit.removed.length),edit.removed);
      current=current.slice(0,edit.start)+edit.inserted+current.slice(edit.start+edit.removed.length);
    }
    assert.equal(digest(current),row.currentSha256,row.file);
    let inverse=current;
    for(const edit of row.edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.source,'the entire original source and context stay recoverable');
    const file=new URL(row.file,root),actual=readFileSync(file,'utf8');
    if(check)assert.equal(actual,current,row.file);
    else assert.ok(actual===row.source||actual===current,'unknown current source '+row.file);
    return{file,current,generated:row.generated===true};
  });
  // The ordinary TypeScript rule generator owns its generated Rust file.
  if(!check)for(const row of outputs)if(!row.generated)writeFileSync(row.file,row.current);
  return archive;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateCiLintRepairs({check:process.argv.includes('--check')});
}
