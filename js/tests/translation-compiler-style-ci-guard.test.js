import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdtempSync,readFileSync,readdirSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname,join,resolve} from 'node:path';
import test from 'node:test';
import {fileURLToPath} from 'node:url';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readGlobalConceptIdentityCompilerStyles} from '../scripts/global-concept-identity-styles.mjs';
import {readCompilerFeedbackCorrections} from '../scripts/compiler-style-feedback.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {simplifyRustBranches} from '../scripts/reviewed-rust-branch-styles.mjs';

const scripts=['generate-global-concept-rule-styles.mjs','generate-frontend-rule-styles.mjs'];
const controls=readProjectData(new URL('../../parity/fixtures/compiler-style-closure-controls.lino',import.meta.url));

test('global concept compiler check retains both entire generated modules and original flags',()=>{
  const {baseline}=controls;
  assert.equal(baseline.revision,'61a96299e459aee352c57fb2abe04d4a77dd6f93');
  assert.equal(baseline.file,'js/scripts/generate-global-concept-rule-styles.mjs');
  assert.equal(baseline.bytes,6374);
  assert.equal(baseline.sha256,'e4b52e9e87fc67b294f0cc17e08dd34224836e38a5638bdd424cc798bc8a5410');
  assert.equal(createHash('sha256').update(baseline.source).digest('hex'),baseline.sha256);
  assert.equal(Buffer.byteLength(baseline.source),baseline.bytes);
  const address=new URL('../scripts/generate-global-concept-rule-styles.mjs',import.meta.url);
  const prepare=source=>{
    const inputs=new Map(),scratch='/virtual/meta-language-style-check',stop=new Error('compiler invocation intercepted');
    let invocation,cleaned=0;
    const executable=source.replace(/^#![^\n]*\n/u,'').replace(/^import [^\n]*\n/gmu,'').replaceAll('import.meta.url',JSON.stringify(address.href));
    assert.throws(()=>runInNewContext(executable,{
      process:{argv:['node',fileURLToPath(address),'--check'],env:{CI:'true',GITHUB_ACTIONS:'true'}},
      dirname,join,resolve,fileURLToPath,tmpdir,
      mkdtempSync:()=>scratch,
      readFileSync:path=>readFileSync(path),
      writeFileSync:(path,value)=>inputs.set(path,Buffer.from(value)),
      rmSync:path=>{assert.equal(path,scratch);cleaned++;},
      spawnSync:(command,args,options)=>{invocation={command,args:structuredClone(args),options:structuredClone(options)};throw stop;},
    },{timeout:1000}),error=>error===stop);
    assert.equal(cleaned,1);assert.ok(invocation);
    return {inputs,invocation,scratch};
  };
  const original=prepare(baseline.source),current=prepare(readFileSync(address,'utf8'));
  assert.deepEqual(current.invocation,original.invocation,'every original compiler flag, deadline and buffer bound stays exact');
  for(const[path,source]of original.inputs)if(!path.endsWith('/check.rs'))assert.deepEqual(current.inputs.get(path),source);
  const identity=readFileSync(new URL('../../rust/src/global_concept_identity_rules.rs',import.meta.url));
  assert.deepEqual(current.inputs.get(join(current.scratch,'global_concept_identity_rules.rs')),identity,'the entire actual identity module reaches the compiler boundary');
  const before=original.inputs.get(join(original.scratch,'check.rs')).toString(),after=current.inputs.get(join(current.scratch,'check.rs')).toString();
  assert.ok(after.startsWith(before));assert.ok(after.includes('pub mod global_concept_identity_rules;\n'));
});

test('native compiler style commands refuse local execution before creating scratch',context=>{
  const temporary=mkdtempSync(join(tmpdir(),'meta-language-style-guard-'));
  context.after(()=>rmSync(temporary,{recursive:true,force:true}));
  for(const script of scripts)for(const flags of[{CI:'',GITHUB_ACTIONS:''},{CI:'true',GITHUB_ACTIONS:''},{CI:'',GITHUB_ACTIONS:'true'}]){
    const result=spawnSync(process.execPath,[fileURLToPath(new URL('../scripts/'+script,import.meta.url)),'--check'],{
      encoding:'utf8',timeout:10000,env:{...process.env,...flags,TMPDIR:temporary,PATH:''},
    });
    assert.equal(result.status,1,script);
    assert.match(result.stderr,/native compiler style execution runs only in GitHub CI/u);
    assert.doesNotMatch(result.stderr,/spawn.*clippy-driver|ENOENT/u);
    assert.deepEqual(readdirSync(temporary),[],script+' creates no compiler scratch outside CI');
  }
});

test('offline compiler feedback arguments remain available without native execution',context=>{
  const temporary=mkdtempSync(join(tmpdir(),'meta-language-style-feedback-guard-'));
  context.after(()=>rmSync(temporary,{recursive:true,force:true}));
  for(const script of scripts){
    const result=spawnSync(process.execPath,[fileURLToPath(new URL('../scripts/'+script,import.meta.url)),'--feedback'],{
      encoding:'utf8',timeout:10000,env:{...process.env,CI:'',GITHUB_ACTIONS:'',TMPDIR:temporary,PATH:''},
    });
    assert.equal(result.status,1);
    assert.match(result.stderr,/--feedback needs a raw compiler feedback file/u);
    assert.doesNotMatch(result.stderr,/native compiler style execution runs only in GitHub CI|spawn.*clippy-driver/u);
    assert.deepEqual(readdirSync(temporary),[]);
  }
});

test('actual identity compiler feedback retains provenance and complete function contexts',()=>{
  const address=new URL('../../parity/self-translation/global-concept-identity-compiler-feedback.lino',import.meta.url);
  const bytes=readFileSync(address);
  assert.equal(bytes.length,117526);
  assert.equal(createHash('sha256').update(bytes).digest('hex'),'b8e34972d1938338a4505fabeb773199618e53586f1603d120591ce67da8b859');
  const feedback=readProjectData(address),original=structuredClone(feedback);
  assert.equal(feedback.runId,'38840126150');
  assert.equal(feedback.head,'bf64ad74d9530d3f2a2689402976ef30f1486873');
  assert.equal(feedback.compiler,'clippy-driver');
  const path='rust/src/global_concept_identity_rules.rs';
  const source=Object.values(feedback.sources).find(input=>input.relativePath===path).text;
  const transaction=readCompilerFeedbackCorrections({feedback,allowedPaths:[path]});
  assert.equal(transaction.spans,1);assert.equal(transaction.deferred,1);
  assert.equal(transaction.actions.length,1,'the general reader still defers overlapping suggestions');
  const styles=readGlobalConceptIdentityCompilerStyles(source,feedback);
  const result=styles.decorate('emitter',{format:'Rust',scope:'formatted-module',source}).source;
  assert.deepEqual(feedback,original,'raw diagnostics, applicability, paths, hashes and input snapshots stay exact');
  assert.equal(styles.decorators[0].actions.length,1);
  const action=styles.decorators[0].actions[0];
  assert.equal(action.from,readSourceTextRustBlock(source,'fn','select_global_concept_identity'));
  assert.equal(result,source.replace(action.from,action.to),'all source outside the complete owning function stays exact');
  assert.equal(result,readFileSync(new URL('../../rust/src/global_concept_identity_rules.rs',import.meta.url),'utf8'));
  assert.ok(action.to.includes('if preferred.is_empty() {'));
  assert.ok(action.to.includes('} else if crate::global_concept_identity_rules::contains_global_concept_identity('));
  assert.ok(action.to.includes('reserved, assigned, candidate,'));
  assert.ok(!action.to.includes('candidate.clone()'));
  assert.ok(action.to.includes('preferred.clone()'),'the distinct preferred identity use remains owned');
  assert.deepEqual([...result.matchAll(/^pub fn (\w+)\(/gmu)].map(match=>match[1]),[...source.matchAll(/^pub fn (\w+)\(/gmu)].map(match=>match[1]));
  assert.equal(styles.decorate('emitter',{format:'Rust',scope:'formatted-module',source:result}).source,result);
  assert.throws(()=>readGlobalConceptIdentityCompilerStyles(source+'// changed\n',feedback),/complete fresh generated module/u);
  const stale=structuredClone(feedback);
  for(const input of Object.values(stale.sources))if(input.relativePath===path)input.sha256='0'.repeat(64);
  assert.throws(()=>readGlobalConceptIdentityCompilerStyles(source,stale),/stale/u);
});

test('reviewed branch export preserves the complete frozen producer and original alias',async()=>{
  const branchControls=readProjectData(new URL('../../parity/fixtures/reviewed-branch-export-controls.lino',import.meta.url));
  const {baseline}=branchControls,address=new URL('../../'+baseline.file,import.meta.url);
  assert.equal(baseline.revision,'8b95e57d696c755dc14705962f783d21f69e89fb');assert.equal(baseline.bytes,18016);
  assert.equal(baseline.sha256,'1a93544e4274c20b2e8c27049dce7f0fcbe70f66eb2b433b17a9a2b0b682777e');
  assert.equal(createHash('sha256').update(baseline.source).digest('hex'),baseline.sha256);
  assert.equal(Buffer.byteLength(baseline.source),baseline.bytes);
  const frozen=readFileSync(address,'utf8');
  assert.equal(createHash('sha256').update(frozen).digest('hex'),'9728ead881e57f16401321be9eb00e25416aab74cdb4d7164485cd0a3abbc216');
  assert.equal(frozen+'\nexport { simplifyBranches as simplifyRustBranches };\n',baseline.source,'the whole effective module retains every original byte and export');
  const linked=baseline.source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>'from '+quote+new URL(path,address).href+quote).replaceAll('import.meta.url',JSON.stringify(address.href));
  const original=await import('data:text/javascript;base64,'+Buffer.from(linked).toString('base64'));
  assert.equal(simplifyRustBranches.toString(),original.simplifyRustBranches.toString());
  for(const source of branchControls.sources)assert.equal(simplifyRustBranches(source),original.simplifyRustBranches(source));
  const {verifyBinaryMetadataBufferNativeModel}=await import('../../experiments/binary-metadata-buffer/native-observations.mjs');
  assert.equal(verifyBinaryMetadataBufferNativeModel().length,13,'the unchanged original oracle authenticates every source binding without planning or native execution');
});
