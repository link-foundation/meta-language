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
