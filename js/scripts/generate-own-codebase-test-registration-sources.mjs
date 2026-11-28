// Automatically capture complete original Git inputs and apply only finite reviewed edits.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {readOwnCodebaseTestRegistrationPolicy,verifyOwnCodebaseTestRegistrationHistory} from './own-codebase-test-registration-history.mjs';
const root=new URL('../../',import.meta.url);

/** Reproduce the complete authenticated sources, three current assignments and exact inverse. */
export function generateOwnCodebaseTestRegistrationSources({check=false}={}){
  const policy=readOwnCodebaseTestRegistrationPolicy();
  const history={...policy,sources:policy.sources.map(row=>{
    const {blob,certificate}=captureGitArtifactCertificate(policy.revision,row.file);
    return{...row,predecessor:{...row.predecessor,source:blob.toString('utf8'),certificate}};
  })};
  verifyOwnCodebaseTestRegistrationHistory(history);
  const outputs=history.sources.map(row=>{
    let current=row.predecessor.source;
    for(const edit of row.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,()=>edit.after);}
    const file=new URL(row.file,root),actual=readFileSync(file,'utf8');
    assert.ok(actual===row.predecessor.source||actual===current,'the normal generator never replaces unknown source changes');
    return{file,actual,current};
  });
  const file=new URL('parity/naming/own-codebase-test-registration-source-history.lino',root);
  if(check){checkProjectData(file,history);for(const output of outputs)assert.equal(output.actual,output.current);}
  else{writeProjectData(file,history);for(const output of outputs)writeFileSync(output.file,output.current);}
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
  generateOwnCodebaseTestRegistrationSources({check:args.includes('--check')});
  console.log('Three current tests are classified; complete original classifier and every dependency assertion remain exact.');
}
