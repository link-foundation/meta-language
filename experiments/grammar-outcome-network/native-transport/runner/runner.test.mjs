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
import {createOverlayPlan,frozenSourceBytes,git,sha256,verifySharedJavaScriptDependencies} from './plan.mjs';
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

test('published changelog fragments can be absent without allowing any other missing source',()=>{
  const repository = fileURLToPath(new URL('../../../../', import.meta.url));
  const path = 'rust/changelog.d/0.58.3-character-class-sets.md';
  const source = `${overlayPolicy.baseCommit}:${path}`;
  const original = Buffer.from(git(repository, ['show', source]));
  const file = {path, gitObject: git(repository, ['rev-parse', source]).trim(), sha256: sha256(original)};
  const missing = Object.assign(new Error('missing current file'), {code: 'ENOENT'});
  const readWorking = () => {throw missing;};

  const selected = frozenSourceBytes(repository, file, {readWorking, allowReleasedAbsence: true});
  assert.deepEqual(selected, {bytes: original, workingSha256: null, workingBytes: null});
  const planned = {...file, workingSha256: null};
  assert.deepEqual(frozenSourceBytes(repository, planned, {readWorking}), selected);
  assert.throws(() => frozenSourceBytes(repository, file, {readWorking}), missing);
  assert.throws(() => frozenSourceBytes(repository, {...file, path: 'rust/src/lib.rs'},
    {readWorking, allowReleasedAbsence: true}), missing);
  assert.throws(() => frozenSourceBytes(repository, planned, {readWorking: () => original}), /production input changed/u);
  assert.throws(() => frozenSourceBytes(repository, planned, {readWorking, readBlob: () => Buffer.from('corrupt')}),
    /frozen Git blob identity/u);
});

test('shared JavaScript dependencies accept both released roots and refuse every other byte change',()=>{
  const repository = fileURLToPath(new URL('../../../../', import.meta.url));
  const manifest = 'js/package.json';
  const lockfile = 'js/package-lock.json';
  const readRelease = commit => Object.fromEntries([manifest, lockfile].map(file =>
    [file, Buffer.from(git(repository, ['show', `${commit}:${file}`]))]));
  const verify = files => verifySharedJavaScriptDependencies(repository, {readWorking: (_directory, file) => files[file]});
  const released = readRelease('8fd7c884ecf8467fd85b6fd8b7db465b0e9b0a18');
  verify(readRelease(overlayPolicy.baseCommit));
  verify(released);
  verifySharedJavaScriptDependencies(repository);

  const edit = (file, mutate) => {
    const document = JSON.parse(released[file]);
    mutate(document);
    return {...released, [file]: Buffer.from(`${JSON.stringify(document, null, 2)}\n`)};
  };
  const patchManifest = edit(manifest, document => {document.version = '1.1.1';});
  const patchLockfile = edit(lockfile, document => {
    document.version = '1.1.1';
    document.packages[''].version = '1.1.1';
  });
  verify({...patchManifest, [lockfile]: patchLockfile[lockfile]});
  for (const version of ['2.0.0', '1.1.1-rc.0', '1.1.1\n']) {
    assert.throws(() => verify(edit(manifest, document => {document.version = version;})), /uncovered JavaScript package version/u);
  }
  assert.throws(() => verify(edit(manifest, document => {document.dependencies['links-notation'] = '0.23.0';})), /frozen input/u);
  assert.throws(() => verify(edit(lockfile, document => {document.packages['node_modules/links-notation'].version = '0.23.0';})), /frozen input/u);
  assert.throws(() => verify(edit(lockfile, document => {document.packages[''].version = '1.0.0';})), /frozen input/u);
  assert.throws(() => verify({...released, [lockfile]: Buffer.concat([released[lockfile], Buffer.from('\n')])}), /frozen input/u);
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
