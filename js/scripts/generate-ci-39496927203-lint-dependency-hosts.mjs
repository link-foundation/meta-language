// Reconstruct historical identity adapters from complete certified source owners.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
const revision='24d7cf48778c7e91f15a25d563c225912d6caad0';
const paths=['js/scripts/generate-typescript-receiver-parameters.mjs',
  'experiments/global-grammar-context-reuse/current-source.mjs',
  'experiments/global-grammar-context-reuse/generate.mjs'];
const helper='js/scripts/ci-39496927203-lint-source-history.mjs';
const feedback='parity/self-translation/ci-39496927203-lint-feedback.lino';
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Replay only finite source identity adapters; execution still uses actual sources. */
export function generateCiLintDependencyHosts({check=false}={}){
  const archive=readProjectData(new URL('parity/self-translation/ci-39496927203-lint-dependency-hosts.lino',root));
  assert.equal(archive.schemaVersion,1);assert.equal(archive.revision,revision);
  assert.deepEqual(archive.sources.map(row=>row.file),paths);
  assert.equal(archive.feedback.path,feedback);
  assert.equal(digest(readFileSync(new URL(feedback,root))),archive.feedback.sha256);
  assert.equal(archive.created.file,helper);assert.equal(digest(archive.created.source),archive.created.sha256);
  const outputs=archive.sources.map(row=>{
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision,path:row.file});
    assert.equal(digest(row.source),row.sha256);
    let current=row.source;
    for(const edit of row.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,edit.after);}
    assert.equal(digest(current),row.currentSha256);
    let inverse=current;
    for(const edit of row.edits.toReversed()){assert.equal(inverse.split(edit.after).length,2);inverse=inverse.replace(edit.after,edit.before);}
    assert.equal(inverse,row.source,'every original host byte and assertion stays recoverable');
    return{file:row.file,source:current,original:row.source};
  });
  outputs.push({file:helper,source:archive.created.source,original:null});
  for(const row of outputs){
    let actual;try{actual=readFileSync(new URL(row.file,root),'utf8');}catch(error){if(error.code!=='ENOENT')throw error;actual=null;}
    if(check)assert.equal(actual,row.source,row.file);
    else assert.ok(actual===row.original||actual===row.source,'unknown source '+row.file);
  }
  if(!check)for(const row of outputs)writeFileSync(new URL(row.file,root),row.source);
  return archive;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateCiLintDependencyHosts({check:process.argv.includes('--check')});
}
