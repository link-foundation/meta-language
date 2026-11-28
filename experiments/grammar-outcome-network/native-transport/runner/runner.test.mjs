// Injected process contracts only. No Cargo, compiler or oracle runs.
import assert from 'node:assert/strict';
import test from 'node:test';
import {EventEmitter} from 'node:events';
import {copyFileSync,mkdtempSync,mkdirSync,readFileSync,realpathSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {requireNativeCI,runOverlay,runBounded} from './run.mjs';
import {checkFrozenJavaScriptDependencies,createOverlayPlan,frozenSourceBytes,git,sha256} from './plan.mjs';
import {publicationEdit,replayPublication} from './generate-publication.mjs';
import {overlayPolicy} from './policy.mjs';

test('native authorization refuses before even an injected compiler process or scratch creation',async()=>{
  const repository=realpathSync(mkdtempSync(join(tmpdir(),'meta-language-b95-runner-contract-')));
  const plan={repository};let calls=0;
  const base={CI:'true',GITHUB_ACTIONS:'true',META_LANGUAGE_B95_NATIVE_OUTCOME:'1',GITHUB_WORKSPACE:repository,RUNNER_TEMP:tmpdir()};
  for(const field of ['CI','GITHUB_ACTIONS','META_LANGUAGE_B95_NATIVE_OUTCOME']) {
    const environment={...base,[field]:undefined};
    await assert.rejects(()=>runOverlay(plan,{environment,spawnProcess:()=>{calls++;throw new Error('forbidden');}}));
  }
  assert.equal(calls,0);
  assert.throws(()=>requireNativeCI(plan,base,'win32'),/Linux only/);
  assert.equal(requireNativeCI(plan,base,'linux'),realpathSync(tmpdir()));
});

test('injected process transport retains exact diagnostics and explicitly identifies contracts',async()=>{
  const directory=mkdtempSync(join(tmpdir(),'meta-language-b95-process-contract-'));
  const logs=join(directory,'logs');mkdirSync(logs);
  let invocation;
  const fake=(command,args,options)=>{
    invocation={command,args,options};
    const child=new EventEmitter();child.stdout=new EventEmitter();child.stderr=new EventEmitter();
    queueMicrotask(()=>{child.stdout.emit('data',Buffer.from('oracle-data\n'));child.stderr.emit('data',Buffer.from('retained warning\n'));child.emit('close',0,null);});
    return child;
  };
  const process=await runBounded('contract-command',['--source-only'],{cwd:directory,environment:{},timeout:1000,logs,label:'contract',spawnProcess:fake});
  assert.equal(process.result.processMode,'injected process contract');
  assert.equal(process.stdout.toString(),'oracle-data\n');assert.equal(process.stderr.toString(),'retained warning\n');
  assert.equal(readFileSync(join(logs,'contract.stderr.log'),'utf8'),'retained warning\n');
  assert.equal(invocation.options.shell,false);assert.equal(invocation.options.detached,true);
  assert.equal(process.result.status,0);assert.equal(process.result.signal,null);
});

test('the default CLI has no native branch and the explicit branch selects only the isolated probe',()=>{
  const source=readFileSync(new URL('./run.mjs',import.meta.url),'utf8');
  assert.ok(source.includes('let native=false'));
  assert.ok(source.includes('if(native) await runOverlay(plan);'));
  assert.ok(source.includes("'--test',policy.testName,'--no-fail-fast','--no-run'"));
  assert.ok(source.includes("'--locked','--offline'"));
  assert.ok(source.includes("['--test-threads=1','--nocapture']"));
  assert.ok(!source.includes('recordIssue195Observations')&&!source.includes('cargo fetch')&&!source.includes('npm install'));
  assert.ok(source.includes("name.startsWith('ISSUE_195_')"));
});

test('the compiled snapshot uses exact frozen blobs while separately retaining current publication metadata',()=>{
  const original=Buffer.from('retained frozen source\n'),current=Buffer.from('later source registration\n');
  const gitObject=createHash('sha1').update('blob '+original.length+'\0').update(original).digest('hex');
  const file={path:'rust/src/data/concept-records.lino',gitObject,sha256:sha256(original)};
  const packet=frozenSourceBytes('/unused',file,{readWorking:()=>current,readBlob:()=>original});
  assert.deepEqual(packet.bytes,original);assert.equal(packet.workingSha256,sha256(current));assert.equal(packet.workingBytes,current.length);
  assert.throws(()=>frozenSourceBytes('/unused',file,{readWorking:()=>current,readBlob:()=>Buffer.from('corrupt')}),/frozen Git blob identity/u);
  assert.throws(()=>frozenSourceBytes('/unused',{...file,workingSha256:sha256(original)},{readWorking:()=>current,readBlob:()=>original}),/production input changed after planning/u);
  let loads=0;const retained=frozenSourceBytes('/unused',file,{readWorking:()=>original,readBlob:()=>{loads++;return original;}});
  assert.deepEqual(retained.bytes,original);assert.equal(loads,0);
});

test('removed release fragments preserve frozen bytes and planned absence', () => {
  const original = Buffer.from('retired changelog fragment\n');
  const gitObject = createHash('sha1').update(`blob ${original.length}\0`).update(original).digest('hex');
  const file = { path: 'rust/changelog.d/historical.md', gitObject, sha256: sha256(original) };
  const readers = {
    readWorking: () => { throw Object.assign(new Error('missing changelog fragment'), { code: 'ENOENT' }); },
    readBlob: () => original,
  };
  assert.throws(() => frozenSourceBytes('/unused', file, readers), { code: 'ENOENT' });
  const removed = { ...file, releaseRemoved: true };
  assert.deepEqual(frozenSourceBytes('/unused', removed, readers), {
    bytes: original, workingSha256: null, workingBytes: null,
  });
  assert.throws(() => frozenSourceBytes('/unused', { ...removed, workingSha256: null }, {
    ...readers, readWorking: () => original,
  }), /production input changed after planning/u);
});

test('the frozen experiment accepts release metadata changes and rejects dependency drift', () => {
  const repository = fileURLToPath(new URL('../../../../', import.meta.url));
  const manifest = 'js/package.json';
  const lockfile = 'js/package-lock.json';
  const readSnapshot = (commit) => new Map([manifest, lockfile].map((file) => [
    file, git(repository, ['show', `${commit}:${file}`]),
  ]));
  const baseline = readSnapshot(overlayPolicy.baseCommit);
  const release = readSnapshot('8fd7c884ecf8467fd85b6fd8b7db465b0e9b0a18');
  for (const file of [manifest, lockfile]) {
    let expected = baseline.get(file).replace(/^  "version": "1\.0\.0",$/mu, '  "version": "1.1.0",');
    if (file === lockfile) {
      expected = expected.replace(/^(    "": \{\n      "name": "meta-language",\n)      "version": "1\.0\.0",$/mu,
        '$1      "version": "1.1.0",');
    }
    assert.equal(release.get(file), expected);
  }

  const check = (files) => checkFrozenJavaScriptDependencies(repository, {
    readWorking: (_repository, file) => Buffer.from(files.get(file)),
  });
  check(baseline);
  check(release);
  assert.throws(() => check(new Map(release).set(lockfile, baseline.get(lockfile))), /differs in js\/package-lock\.json/u);
  const alteredLockfile = release.get(lockfile).replace('"links-notation": "0.22.0"', '"links-notation": "0.22.1"');
  assert.notEqual(alteredLockfile, release.get(lockfile));
  assert.throws(() => check(new Map(release).set(lockfile, alteredLockfile)), /differs in js\/package-lock\.json/u);
  const unknownVersion = release.get(manifest).replace('"version": "1.1.0"', '"version": "1.1.1"');
  assert.throws(() => check(new Map(release).set(manifest, unknownVersion)), /No frozen JavaScript dependency profile/u);
});

test('publication edits retain the complete original and refuse a changed original interval',()=>{
  const original='before\nreviewed source\nafter\n',published='before\nportable reviewed source\nafter\n';
  const edit=publicationEdit(original,published);
  assert.equal(replayPublication(original,edit),published);
  assert.throws(()=>replayPublication(original,{...edit,removed:'unrelated'}),/exact publication source interval/u);
  assert.throws(()=>replayPublication(original,{...edit,end:original.length+1}));
  assert.equal(replayPublication(original,publicationEdit(original,original)),original);
});

test('output overflow retains the bounded prefix and reports a failed process contract',async()=>{
  const directory=mkdtempSync(join(tmpdir(),'meta-language-b95-output-contract-'));
  const fake=()=>{
    const child=new EventEmitter();child.stdout=new EventEmitter();child.stderr=new EventEmitter();
    queueMicrotask(()=>{child.stderr.emit('data',Buffer.alloc(overlayPolicy.maximumOutputBytes+1,120));child.emit('close',0,null);});
    return child;
  };
  const result=await runBounded('contract-command',[],{cwd:directory,environment:{},timeout:1000,logs:directory,label:'overflow',spawnProcess:fake});
  assert.equal(result.result.error,'stderr output limit');
  assert.equal(result.result.stderrBytes,overlayPolicy.maximumOutputBytes+1);
  assert.equal(result.stderr.length,overlayPolicy.maximumOutputBytes);
  assert.equal(result.result.streams.stderr.truncated,true);
  assert.equal(result.result.processMode,'injected process contract');
});

test('an alternate probe directory cannot substitute an unqualified oracle or native source',async()=>{
  const directory=mkdtempSync(join(tmpdir(),'meta-language-b95-probe-source-contract-'));
  const files=['oracle.mjs','native-outcome-probe.rs'];
  for(const file of files)copyFileSync(new URL('../probe/'+file,import.meta.url),join(directory,file));
  for(const file of files) {
    const path=join(directory,file),bytes=readFileSync(path);
    writeFileSync(path,Buffer.concat([bytes,Buffer.from('\n')]));
    await assert.rejects(()=>createOverlayPlan({probeDirectory:directory}),/selected actual probe source/u);
    writeFileSync(path,bytes);
  }
});
