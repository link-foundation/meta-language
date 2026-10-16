import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {mkdtempSync,readdirSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {fileURLToPath} from 'node:url';

const scripts=['generate-global-concept-rule-styles.mjs','generate-frontend-rule-styles.mjs'];

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
