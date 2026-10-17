import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {existsSync,lstatSync,readFileSync,symlinkSync} from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {runCleanup} from '../../scripts/lib/cache-cleanup.mjs';
import {parseArguments} from '../../scripts/clean-caches.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {CLEAN_CACHES,WITH_CACHE_CLEANUP,exitedPid,makeBase,makeCargoTarget,makeRepository,makeScratch,put,worktreeState} from './support/cache-fixtures.js';

function createProjectScopeFixture(t){
  const fixture=makeRepository(makeBase(t)),inside=makeCargoTarget(path.join(fixture.root,'rust/target'));
  const outside=['explicit target','inherited target','inherited build target'].map(name=>makeCargoTarget(path.join(fixture.base,name)));
  const env={...fixture.env,CARGO_TARGET_DIR:outside[1],CARGO_BUILD_TARGET_DIR:outside[2]};
  return {...fixture,inside,outside,env};
}

test('project scope excludes inherited and explicit foreign targets before discovery and budget pruning',t=>{
  const fixture=createProjectScopeFixture(t),before=worktreeState(fixture.root);
  const report=runCleanup({cwd:fixture.root,tmpRoot:fixture.tmpRoot,env:fixture.env,projectOwnedOnly:true,targetDirectories:[fixture.outside[0]],budgetMb:1,minFreeMb:1000000000,preflight:true,docker:true});
  assert.equal(report.status,'ok');assert.equal(report.scope,'project-owned-only');assert.equal(report.mode,'prune');assert.equal(report.preflight.enabled,false);assert.equal(report.preflight.escalated,false);assert.equal(report.classes.containers.status,'skipped');
  assert.equal(existsSync(path.join(fixture.inside,'debug/incremental')),false);
  assert.ok(existsSync(path.join(fixture.inside,'debug/deps/libfixture.rlib')),'the disabled low disk preflight keeps the within budget warm cache');
  for(const outside of fixture.outside){
    assert.ok(existsSync(path.join(outside,'debug/incremental/fixture/s-1/query-cache.bin')));
    assert.ok(existsSync(path.join(outside,'debug/deps/libfixture.rlib')));
    assert.ok(!JSON.stringify(report.classes).includes(outside),'foreign targets are excluded before candidate traversal and byte accounting');
  }
  assert.equal(worktreeState(fixture.root),before);
});

test('project scope reaches the cleanup command and failure wrapper without widening target ownership',t=>{
  const fixture=createProjectScopeFixture(t),receipt=path.join(fixture.base,'scoped receipt.lino');
  assert.equal(parseArguments(['--project-owned-only']).options.projectOwnedOnly,true);
  const result=spawnSync(process.execPath,[CLEAN_CACHES,'--project-owned-only','--full','--target-dir',fixture.outside[0],'--report',receipt],{cwd:fixture.root,env:fixture.env,encoding:'utf8'});
  assert.equal(result.status,0,result.stderr);assert.equal(readProjectData(receipt).scope,'project-owned-only');assert.equal(existsSync(fixture.inside),false);
  const code="const fs=require('node:fs');fs.mkdirSync('js/node_modules/.cache',{recursive:true});fs.writeFileSync('js/node_modules/.cache/scoped','cache');process.exit(7);";
  const wrapped=spawnSync(process.execPath,[WITH_CACHE_CLEANUP,'--event','test','--project-owned-only','--',process.execPath,'-e',code],{cwd:fixture.root,env:fixture.env,encoding:'utf8'});
  assert.equal(wrapped.status,7,wrapped.stderr);assert.equal(existsSync(path.join(fixture.root,'js/node_modules/.cache/scoped')),false);
  assert.equal(readProjectData(path.join(fixture.root,'.git/meta-language-cache/last-run.lino')).scope,'project-owned-only');
  for(const outside of fixture.outside)assert.ok(existsSync(path.join(outside,'debug/incremental/fixture/s-1/query-cache.bin')));
});

test('project scope preserves dirty source evidence live scratch and foreign markers while clearing owned stopped scratch',t=>{
  const fixture=createProjectScopeFixture(t),pid=exitedPid();
  const owned=makeScratch(fixture,'meta-language-owned',{pid}),live=makeScratch(fixture,'meta-language-live',{pid:process.pid}),foreign=makeScratch(fixture,'meta-language-foreign',{pid,root:path.join(fixture.base,'other repository')});
  const evidence=put(fixture.root,'issue-195-artifacts/actual.lino','(retained: evidence)\n');
  const source=put(fixture.root,'rust/src/lib.rs','pub fn f() {}\n// uncommitted user work\n'),before=worktreeState(fixture.root);
  const report=runCleanup({cwd:fixture.root,tmpRoot:fixture.tmpRoot,env:fixture.env,projectOwnedOnly:true,mode:'full'});
  assert.equal(report.status,'ok');assert.equal(existsSync(owned),false);assert.ok(existsSync(live));assert.ok(existsSync(foreign));
  assert.equal(readFileSync(evidence,'utf8'),'(retained: evidence)\n');assert.equal(readFileSync(source,'utf8'),'pub fn f() {}\n// uncommitted user work\n');assert.equal(worktreeState(fixture.root),before);
  for(const outside of fixture.outside)assert.ok(existsSync(path.join(outside,'debug/deps/libfixture.rlib')));
});

test('project scope excludes target symbolic link escapes before candidate discovery',t=>{
  const fixture=createProjectScopeFixture(t);
  const options={cwd:fixture.root,tmpRoot:fixture.tmpRoot,env:fixture.env,projectOwnedOnly:true,mode:'full'};
  assert.equal(runCleanup(options).status,'ok');assert.equal(existsSync(fixture.inside),false);
  symlinkSync(fixture.outside[0],fixture.inside,'junction');
  const report=runCleanup(options);
  assert.equal(report.status,'ok');assert.ok(lstatSync(fixture.inside).isSymbolicLink());
  assert.deepEqual(report.classes['rust-target'].removed,[]);assert.deepEqual(report.classes['rust-target'].skipped,[]);
  assert.ok(existsSync(path.join(fixture.outside[0],'debug/incremental/fixture/s-1/query-cache.bin')));
});
