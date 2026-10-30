// Automatically retain complete independently certified compiler inputs.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url),revision='be0a9d76196075e6b008c05a8c63b4b4611237e7';
assert.equal(process.argv.length,3,'supply the complete original bounded CI lint log');
const bytes=readFileSync(process.argv[2]);
assert.equal(bytes.length,243316);assert.equal(createHash('sha256').update(bytes).digest('hex'),'37263b78bf7bb2ff61faaa5905a4ba5a69c117af98bbfdff233a92dd9ab1f631');
const families=['ascii_text_predicate_rules','declaration_dependency_rules','external_emission_rules','inert_module_binding_rules','javascript_record_parameter_rules','reference_identity_rules','rust_literal_match_rules','rust_shared_array_handle_rules'];
const files=execFileSync('git',['ls-tree','-r','--name-only',revision,'rust/src/translation'],{cwd:root,encoding:'utf8',maxBuffer:1024*1024}).trim().split('\n');
const sources=families.map(family=>({family,files:files.filter(file=>file==='rust/src/translation/'+family+'.rs'||file.startsWith('rust/src/translation/'+family+'/')&&file.endsWith('.rs')).map(file=>{
 const captured=captureGitArtifactCertificate(revision,file);verifyGitArtifactCertificate(captured.certificate,captured.blob,{revision,path:file});
 return{file,source:captured.blob.toString('utf8'),bytes:captured.blob.length,sha256:createHash('sha256').update(captured.blob).digest('hex'),certificate:captured.certificate};
})}));
assert.ok(sources.every(row=>row.files.length>=2));
writeProjectData(new URL('parity/fixtures/ci-39890308543-native-source-styles.lino',root),{schemaVersion:1,model:'reviewed native source styles from complete certified inputs 1',revision,
 diagnostic:{run:39890308543,job:120227286631,url:'https://github.com/link-foundation/meta-language/actions/runs/39890308543/job/120227286631',kind:'rendered compiler diagnostics, reviewed separately from machine-applicable suggestions',source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')},sources,
 boundary:'Complete original Git-certified source inputs and unmodified diagnostic bytes. These are source fixtures, not successful native execution or requirement evidence. Actual corrections are reproduced through normal family Links decorators; native verification remains CI-only.'});
console.log('Retained '+sources.reduce((count,row)=>count+row.files.length,0)+' complete original native sources and the exact rendered CI log.');
