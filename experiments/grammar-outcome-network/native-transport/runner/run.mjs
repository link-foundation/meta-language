// CI-only experiment runner. Default is source plan/check only.
// No production source write, install, fetch, clone, or observation-ledger call.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,symlinkSync,writeFileSync} from 'node:fs';
import {resolve,join,dirname} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {overlayPolicy as policy} from './policy.mjs';
import {bytesAt,crateInventory,createOverlayPlan,frozenSourceBytes,sha256,within} from './plan.mjs';
import {checkRunnerPublication} from './generate-publication.mjs';
import {verifyPublication} from '../wire/publication.mjs';

/** Must run before ANY compiler command, including metadata/version queries,
 * before compiler scratch creation, and before injected process contracts.
 */
export function requireNativeCI(plan,environment=process.env,platform=process.platform) {
  assert.equal(environment.META_LANGUAGE_B95_NATIVE_OUTCOME,'1','explicit native experiment opt-in required');
  assert.equal(environment.CI,'true','native execution is forbidden locally');
  assert.equal(environment.GITHUB_ACTIONS,'true','native execution requires GitHub Actions');
  assert.equal(platform,'linux','first experiment process-group profile is Linux only');
  assert.equal(realpathSync(environment.GITHUB_WORKSPACE),plan.repository,'actual CI checkout owner');
  const temporary=realpathSync(environment.RUNNER_TEMP);
  assert.ok(temporary!=='/'&&!within(plan.repository,temporary)&&temporary!==plan.repository,'compiler scratch is outside production checkout');
  return temporary;
}

/** Bounded actual process transport. Process groups contain ONLY this spawned
 * command and its descendants; no process-name kill or unrelated cleanup.
 */
export function runBounded(command,args,{cwd,environment,timeout,logs,label,spawnProcess=spawn}) {
  return new Promise((accept,reject)=>{
    let child;
    try {child=spawnProcess(command,args,{cwd,env:environment,shell:false,detached:true,stdio:['ignore','pipe','pipe']});}
    catch(error) {reject(error);return;}
    const started=Date.now(),captured={stdout:[],stderr:[]},counts={stdout:0,stderr:0},kept={stdout:0,stderr:0};
    let failure=null,forceTimer;
    const stop=reason=>{
      failure??=reason;
      if(Number.isInteger(child.pid)&&child.pid>0) {
        try {process.kill(-child.pid,'SIGTERM');} catch(error) {if(error.code!=='ESRCH') failure+='; TERM '+error.message;}
        forceTimer??=setTimeout(()=>{try{process.kill(-child.pid,'SIGKILL');}catch(error){if(error.code!=='ESRCH') failure+='; KILL '+error.message;}},policy.terminationGrace);
      }
    };
    const timer=setTimeout(()=>stop('deadline '+timeout+'ms'),timeout);
    for(const stream of ['stdout','stderr']) child[stream].on('data',chunk=>{
      counts[stream]+=chunk.length;
      const width=Math.min(chunk.length,Math.max(0,policy.maximumOutputBytes-kept[stream]));
      if(width) {captured[stream].push(chunk.subarray(0,width));kept[stream]+=width;}
      if(counts[stream]>policy.maximumOutputBytes) stop(stream+' output limit');
    });
    child.on('error',error=>{failure??=error.message;});
    child.on('close',(status,signal)=>{
      clearTimeout(timer);if(forceTimer)clearTimeout(forceTimer);
      const result={label,command,arguments:args,status,signal,elapsedMs:Date.now()-started,timeout,
        processMode:spawnProcess===spawn?'actual subprocess':'injected process contract',
        error:failure,stdoutBytes:counts.stdout,stderrBytes:counts.stderr,streams:{}};
      const buffers={};
      try {
        for(const stream of ['stdout','stderr']) {
          const bytes=Buffer.concat(captured[stream]);buffers[stream]=bytes;
          const path=join(logs,label+'.'+stream+'.log');writeFileSync(path,bytes,{flag:'wx'});
          result.streams[stream]={path,bytes:bytes.length,sha256:sha256(bytes),truncated:counts[stream]!==bytes.length};
        }
      } catch(error) {reject(error);return;}
      accept({result,...buffers}); // Caller preserves failures; never calls them observations.
    });
  });
}

function environmentFor(work,original) {
  const environment={...original};
  for(const name of Object.keys(environment)) if(name.startsWith('ISSUE_195_')) delete environment[name];
  for(const name of ['CARGO_TARGET_DIR','CARGO_BUILD_TARGET_DIR','CARGO_BUILD_TARGET','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS',
    'RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTC','CARGO_BUILD_RUSTC','CARGO_BUILD_RUSTC_WRAPPER','CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER']) delete environment[name];
  return {...environment,CARGO_TARGET_DIR:join(work,'target'),CARGO_BUILD_JOBS:'1',CARGO_NET_OFFLINE:'true',
    CARGO_INCREMENTAL:'0',NODE_OPTIONS:'',RUST_TEST_THREADS:'1'};
}

function copySnapshot(plan,overlay) {
  for(const file of plan.inventory.files) {
    const destination=join(overlay,file.path);
    const {bytes}=frozenSourceBytes(plan.repository,file);
    mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,bytes,{flag:'wx'});
    chmodSync(destination,file.mode==='100755'?0o755:0o644);
    assert.equal(sha256(bytesAt(overlay,file.path)),file.sha256,'exact independent scratch file');
  }
  // The Agda dev-oracle build invokes its existing JS queue helper. Its small
  // SOURCE closure is copied above; only the already installed dependency
  // directory is shared. No npm/install script is invoked.
  const modules=realpathSync(join(plan.repository,'js/node_modules'));
  assert.ok(lstatSync(modules).isDirectory(),'existing CI dependency installation');
  symlinkSync(modules,join(overlay,'js/node_modules'),'dir');
  writeFileSync(join(overlay,policy.parent),plan.parent);
  mkdirSync(dirname(join(overlay,policy.child)),{recursive:true});
  writeFileSync(join(overlay,policy.child),plan.child,{flag:'wx'});
  writeFileSync(join(overlay,'rust/tests',policy.testName+'.rs'),plan.probe,{flag:'wx'});
  const producerPath='rust/src/grammar/feature_runtime/grammar_selection.rs';
  assert.equal(readFileSync(join(overlay,producerPath),'utf8'),plan.producerCorrection.original,'actual frozen producer matches the explicit adapter input');
  writeFileSync(join(overlay,producerPath),plan.producerCorrection.source);
}

/** No ledger writes. Native report is only the probe's own post-assertions
 * artifact; this runner does not interpret it as public API/grammar evidence.
 */
export async function runOverlay(plan,{environment=process.env,spawnProcess=spawn}={}) {
  const temporary=requireNativeCI(plan,environment);
  const artifactParent=join(plan.repository,'issue-195-artifacts','bulk95-outcome-overlay');
  mkdirSync(artifactParent,{recursive:true});assert.ok(!lstatSync(artifactParent).isSymbolicLink());
  assert.ok(within(plan.repository,realpathSync(artifactParent)),'artifact owner cannot escape through an ancestor symlink');
  const artifacts=mkdtempSync(join(artifactParent,'run-'));
  const work=mkdtempSync(join(temporary,'meta-language-bulk95-outcome-'));
  const overlay=join(work,'overlay');mkdirSync(overlay);
  const {writeProjectData,readProjectData}=await import(pathToFileURL(join(plan.repository,'js/scripts/project-data.mjs')));
  writeProjectData(join(work,'.meta-language-bulk95-owned.lino'),{repository:plan.repository,executionCommit:plan.executionCommit,artifacts});
  const receipt={schemaVersion:1,scope:policy.profile,baseCommit:policy.baseCommit,executionCommit:plan.executionCommit,
    processMode:spawnProcess===spawn?'actual subprocess':'injected process contract',
    work,artifacts,sourceManifest:plan.inventory.files,sourceBytes:plan.inventory.bytes,
    originalCandidatePins:policy.candidatePins,originalProbePins:policy.probePins,
    publication:plan.publication,runnerPublication:plan.runnerPublication.publication,
    runnerAdapterSources:plan.runnerPublication.newSources,
    bridgeReceipt:plan.receipt,producerCorrection:plan.producerCorrection.receipt,nativeOraclePolicy:plan.nativeOraclePolicy,processes:[]};
  let failure=null;
  const execute=async(command,args,timeout,label,env=environmentFor(work,environment))=>{
    requireNativeCI(plan,environment); // Also guards injected contract processes.
    const process=await runBounded(command,args,{cwd:join(overlay,'rust'),environment:env,timeout,logs:artifacts,label,spawnProcess});
    receipt.processes.push(process.result);
    if(process.result.error||process.result.status!==0||process.result.signal) throw new Error(label+' failed; exact logs in '+artifacts);
    return process.stdout.toString('utf8');
  };
  try {
    copySnapshot(plan,overlay);
    // Dev-dependency build scripts require their registered external oracle
    // source assets even when the selected probe never calls those oracles.
    // Acquire them only inside this owned overlay, never in production.
    const assetRequest=join(work,'native-oracle-assets-request.lino');
    const assetReceipt=join(artifacts,'native-oracle-assets-receipt.lino');
    writeProjectData(assetRequest,plan.nativeOraclePolicy);
    await execute(process.execPath,['--max-old-space-size=512',fileURLToPath(new URL('./provision-assets.mjs',import.meta.url)),
      '--repository',plan.repository,'--work',work,'--overlay',overlay,'--request',assetRequest,
      '--request-sha256',sha256(readFileSync(assetRequest)),'--receipt',assetReceipt],plan.nativeOraclePolicy.timeoutMs,'external-oracle-assets');
    const assetBytes=readFileSync(assetReceipt);
    receipt.nativeOracleAssets={path:assetReceipt,bytes:assetBytes.length,sha256:sha256(assetBytes)};
    const fixture=join(artifacts,'oracle.lino'),report=join(artifacts,'native-report.lino');
    assert.ok(!existsSync(fixture)&&!existsSync(report));
    // Exactly one independent JS oracle generation, never substitute it for
    // native probe execution or attach a ledger record to its metadata.
    const authoredFixture=join(work,'oracle.lino');
    await execute(process.execPath,['--max-old-space-size=512',join(plan.probeDirectory,'oracle.mjs'),
      '--repository',plan.repository,'--output',authoredFixture],policy.oracleTimeout,'oracle');
    const oracleBytes=bytesAt(work,'oracle.lino',8*1024*1024);
    writeFileSync(fixture,oracleBytes,{flag:'wx'});
    assert.deepEqual(bytesAt(artifacts,'oracle.lino',8*1024*1024),oracleBytes,'retained exact outside-checkout oracle output');
    receipt.oracle={bytes:oracleBytes.length,sha256:sha256(oracleBytes),authoredFixture,fixture};
    await execute('cargo',['--version','--verbose'],10000,'cargo-version');
    await execute('rustc',['--version','--verbose'],10000,'rustc-version');
    const nativeEnvironment={...environmentFor(work,environment),META_LANGUAGE_B95_OUTCOME_FIXTURE:fixture,META_LANGUAGE_B95_OUTCOME_REPORT:report};
    const build=await execute('cargo',['test','--manifest-path',join(overlay,'rust/Cargo.toml'),'--locked','--offline',
      '--no-default-features','--jobs','1','--test',policy.testName,'--no-fail-fast','--no-run',
      '--message-format=json-render-diagnostics'],policy.compilerTimeout,'compile',nativeEnvironment);
    const targets=build.split(/\r?\n/u).filter(Boolean).map(line=>JSON.parse(line)).filter(item=>
      item.reason==='compiler-artifact'&&item.target?.name===policy.testName&&item.target.kind.includes('test')&&item.executable);
    assert.equal(targets.length,1,'one actual selected native test artifact');
    const executable=realpathSync(targets[0].executable);
    assert.ok(within(join(work,'target'),executable)&&lstatSync(executable).isFile(),'native artifact belongs to exact scratch target');
    await execute(executable,['--test-threads=1','--nocapture'],policy.executionTimeout,'native-probe',nativeEnvironment);
    const nativeBytes=bytesAt(artifacts,'native-report.lino',8*1024*1024);
    const data=readProjectData(report);assert.ok(data&&typeof data==='object','actual native post-assertions linked report');
    receipt.nativeReport={bytes:nativeBytes.length,sha256:sha256(nativeBytes),path:report};
    // Even Cargo.lock and all dev-oracle/generated sources remain original in
    // the checkout. The two candidate source files exist only in the overlay.
    assert.deepEqual(crateInventory(plan.repository),plan.inventory,'production inputs remain unchanged');
    assert.deepEqual(checkRunnerPublication(),plan.runnerPublication,'runner publication remains unchanged');
    assert.deepEqual(verifyPublication(),plan.publication,'bridge/probe publication remains unchanged');
  } catch(error) {
    failure=error;receipt.failure={name:error.name,message:error.message};
  } finally {
    writeProjectData(join(artifacts,'runner-receipt.lino'),receipt);
    // Deliberately retain logs, native report and exact scratch ownership. CI
    // teardown may clean this stopped task's work/target; never the evidence.
    console.error('B95 scratch outcome diagnostics: '+artifacts+'; stopped scratch: '+work);
  }
  if(failure) throw failure;
  return receipt;
}

async function main() {
  const args=process.argv.slice(2),options={};let native=false;
  for(let index=0;index<args.length;index++) {
    if(args[index]==='--native') {assert.equal(native,false);native=true;}
    else {
      assert.ok(['--repository','--candidate-dir','--probe-dir'].includes(args[index]));
      const key={'--repository':'repository','--candidate-dir':'candidateDirectory','--probe-dir':'probeDirectory'}[args[index]];
      assert.ok(!options[key]&&typeof args[index+1]==='string');options[key]=resolve(args[++index]);
    }
  }
  const plan=await createOverlayPlan(options);
  if(native) await runOverlay(plan);
  else console.log(JSON.stringify({scope:'source plan only; no Cargo/oracle execution',baseCommit:policy.baseCommit,
    executionCommit:plan.executionCommit,files:plan.inventory.files.length,bytes:plan.inventory.bytes,probe:policy.testName}));
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)) await main();
