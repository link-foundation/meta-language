// Keep one actual native rule owner, with complete original source inverses.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {readSourceTextRustBlock} from '../../../../js/scripts/source-text-byte-library-style.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {restoreNativeTestLintSource} from '../../../../js/scripts/native-test-lint-source-history.mjs';

const root=new URL('../../../../',import.meta.url),revision='6fba3503a710325310e359995a1a77fcedf8011a';
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});
export function generateStaticBorrowModuleHistory({check=false}={}) {
  const sources=[];
  for(const [file,actions]of [
    ['experiments/grammar-outcome-network/native-transport/inverse/static-borrows.rs',[
      {from:'mod rules;',to:'pub(super) mod rules;'},
    ]],
    ['rust/tests/unit/grammar_outcome_static_borrows.rs',[
      {from:'#[path = "../../src/translation/owned_outcome_static_borrow_rules.rs"]\nmod rules;\n#[path = "../../../experiments/grammar-outcome-network/native-transport/inverse/static-borrows.rs"]\nmod storage;',to:'#[path = "../../../experiments/grammar-outcome-network/native-transport/inverse/static-borrows.rs"]\nmod storage;\nuse storage::rules;'},
      {from:'1048576.0',to:'1_048_576.0'},
      {from:'1048577.0',to:'1_048_577.0'},
      {from:'repeat(1048577)',to:'repeat(1_048_577)'},
    ]],
  ]){
    const original=captureGitArtifactCertificate(revision,file),sourceText=original.blob.toString('utf8');
    verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
    const actual=readFileSync(new URL(file,root),'utf8');
    const current=file.endsWith('grammar_outcome_static_borrows.rs')?restoreNativeTestLintSource(file,actual):actual;let corrected=sourceText;
    for(const action of actions){assert.equal(corrected.split(action.from).length,2);corrected=corrected.replace(action.from,action.to);}
    assert.equal(corrected,current,'only the observed module ownership and literal spelling repairs are admitted');
    let inverse=current;for(const action of [...actions].reverse()){
      assert.equal(inverse.split(action.to).length,2);inverse=inverse.replace(action.to,action.from);
    }
    assert.equal(inverse,sourceText,'all original source inputs and assertions survive exact inverse');
    sources.push({file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current),actions});
  }
  const adapter=sources[0],currentAdapter=readFileSync(new URL(adapter.file,root),'utf8');
  assert.equal(readSourceTextRustBlock(adapter.sourceText,'fn','resolve_owned_outcome_static_borrow'),
    readSourceTextRustBlock(currentAdapter,'fn','resolve_owned_outcome_static_borrow'),'the complete actual storage algorithm and lifetime signature remain byte exact');
  const unchanged=['rust/src/translation/owned_outcome_static_borrow_rules.rs','rust/src/translation/owned_outcome_static_borrow_rules/part_1.rs',
    'parity/fixtures/grammar-outcome-static-borrows.lino'].map(file=>{
    const original=captureGitArtifactCertificate(revision,file),current=readFileSync(new URL(file,root));
    assert.ok(current.equals(original.blob));return{file,...pin(current)};
  });
  const history={schemaVersion:1,revision,sources,unchanged,
    observedLintDiagnostics:{run:39359700443,job:118431586371,logBytes:200564,logSha256:'8d6f8c7e9b9c62c2f138704db97e9c63b9a71943c84681390efcaf0f1c729feb',
      lints:['duplicate_mod','unreadable_literal'],literalValues:[1048576,1048577]},
    boundary:'One actual generated native rule module supplies both direct controls and typed static storage. Only visibility within its parent, an actual module alias and separator spellings change; every original test input assertion native algorithm and existing static lifetime remain. Native execution still requires CI.'};
  const output=new URL('../../../../parity/self-translation/static-borrow-module-source-history.lino',import.meta.url);
  if(check)checkProjectData(output,history);else writeProjectData(output,history);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateStaticBorrowModuleHistory({check:process.argv.includes('--check')});
  console.log('Recorded exact original static-borrow sources and one shared native rule module without changing any original assertion.');
}
