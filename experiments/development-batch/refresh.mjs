// Authoring automation only. No verification observation or native compilation.
import assert from 'node:assert/strict';
import {spawnSync,execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {structuredDataToLino,readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
const planFile=new URL('./stages.lino',import.meta.url);
const digest=source=>createHash('sha256').update(source).digest('hex');
const stage=(id,script,args=[])=>({id,script,args});

export function readDevelopmentAuthoringPlan(){
 return{schemaVersion:1,model:'serial JavaScript metadata authoring 1',phases:[
  {name:'snapshot',stages:[stage('raw consuming source profile','js/scripts/generate-raw-consuming-snapshot-source-profile.mjs'),stage('relative character source decisions','experiments/grammar-outcome-network/native-transport/inverse/generate-relative-character-self-translation.mjs')]},
  {name:'history',stages:[
   stage('native compiler host source history','js/scripts/generate-native-compiler-host-source-history.mjs'),
   stage('borrowed JavaScript body source history','js/scripts/generate-borrowed-javascript-body-source-history.mjs'),
   stage('physical CI workflow history','js/scripts/generate-ci-test-partition-history.mjs'),
   stage('grammar context before repair','experiments/grammar-context-source-metadata/generate.mjs'),
   stage('complete CI repair sources','js/scripts/generate-current-ci-repair-source-history.mjs'),
   stage('native compatibility sources','experiments/grammar-outcome-network/native-transport/inverse/generate-native-compatibility-source-history.mjs'),
   stage('evidence translation naming sources','experiments/grammar-outcome-network/native-transport/inverse/generate-evidence-translation-history.mjs',['--naming-only']),
   stage('evidence translation sources','experiments/grammar-outcome-network/native-transport/inverse/generate-evidence-translation-history.mjs'),
   stage('grammar context after repair','experiments/grammar-context-source-metadata/generate.mjs'),
   stage('raw planning sources','experiments/original-link-complete-table-projection/generate-raw-planning-source-history.mjs'),
   stage('Boolean value sources','js/scripts/generate-boolean-value-source-history.mjs'),
   stage('complete original operation declarations','js/scripts/generate-module-scope-declaration-history.mjs'),
  ]},
  {name:'registration',stages:[
   stage('canonical source operations','js/scripts/generate-module-scope-naming.mjs',['--section','canonical']),
   stage('complete source operation bindings','js/scripts/generate-module-scope-naming.mjs',['--section','bindings']),
   stage('complete binding codec epoch','js/scripts/generate-naming-source-binding-codec-profile.mjs'),
  ]},
  {name:'naming',stages:[stage('paired canonical concept catalogs','js/scripts/build-concept-records.mjs'),stage('complete additional naming views','js/scripts/generate-own-codebase-baseline-naming-history.mjs',['--additional-inventories'])]},
  {name:'verification',stages:[
   stage('live requirement sources','js/scripts/check-issue-195-sources.mjs',['--online']),
   stage('dependency inventory','js/scripts/check-dependencies.mjs',['--offline']),
   stage('requirement scope','js/scripts/check-issue-195-acceptance.mjs',['--scope-baseline','HEAD']),
   stage('public exports','js/scripts/generate-export-parity.mjs',['--check']),
   stage('runtime inventory','js/scripts/check-js-rust-parity.mjs',['--manifest-only']),
   stage('documentation','js/scripts/check-documentation.mjs'),
   stage('test ownership','js/scripts/test-groups.mjs',['--check']),
   stage('test partition history','js/scripts/generate-ci-test-partition-history.mjs',['--check']),
   stage('shared array native rules','js/scripts/generate-rust-shared-array-handle-rules.mjs',['--native','--check']),
   stage('inert binding native rules','js/scripts/generate-inert-module-binding-rules.mjs',['--native','--check']),
   stage('shared array source history','js/scripts/generate-shared-array-native-frontend-history.mjs',['--check']),
   stage('typed descriptor source history','js/scripts/generate-typed-source-descriptor-host-history.mjs',['--check']),
   stage('grammar context metadata','experiments/grammar-context-source-metadata/generate.mjs',['--check']),
   stage('evidence translation history','experiments/grammar-outcome-network/native-transport/inverse/generate-evidence-translation-history.mjs',['--check']),
   stage('canonical concept catalogs','js/scripts/build-concept-records.mjs',['--check']),
   stage('historical naming storage','js/scripts/check-historical-naming-inventory-storage.mjs',['--check']),
   stage('relative character source decisions','experiments/grammar-outcome-network/native-transport/inverse/generate-relative-character-self-translation.mjs',['--check']),
   stage('crate embedded source inputs','js/tests/crate-package-include.test.js'),
  ]},
 ],boundary:'Invokes complete existing normal authors and read-only metadata verification, in dependency order and fresh serial processes. The repeated grammar context step is intentional. Naming runs after all source-changing authors. Verification checks live sources before reading the ledger and never refreshes stale outputs. Receipts record invocations only, not runtime, translation, native, CI or requirement evidence. Full workloads and Rust compilation remain CI-only.'};
}

export function developmentAuthoringEnvironment(environment){
 const result={...environment,META_LANGUAGE_JOBS:'1',TRANSLATION_CHECK_RUST:'0',SELF_TRANSLATION_CHECK_RUST:'0',RUSTUP_AUTO_INSTALL:'0'};
 for(const key of ['CARGO_TARGET_DIR','CARGO_BUILD_TARGET_DIR','NODE_OPTIONS','CI'])delete result[key];
 return result;
}

export function runDevelopmentAuthoringPhase(plan,name,{check=false,execute=spawnSync,environment=process.env,record=()=>{}}={}){
 assert.deepEqual(plan,readDevelopmentAuthoringPlan(),'only the complete recorded authoring plan may execute');
 const selected=plan.phases.find(phase=>phase.name===name);assert.ok(selected,'unknown authoring phase: '+name);
 const results=[];
 for(const entry of selected.stages){
  const bytes=readFileSync(resolve(root,entry.script));
  const args=['--expose-gc','--optimize-for-size','--max-old-space-size=352','--max-semi-space-size=1',entry.script,...entry.args,...(check&&name!=='verification'?['--check']:[])];
  process.stderr.write('Authoring '+entry.id+': '+entry.script+'\n');
  const result=execute(process.execPath,args,{cwd:root,env:developmentAuthoringEnvironment(environment),stdio:'inherit',timeout:80000,killSignal:'SIGKILL'});
  results.push({id:entry.id,script:entry.script,sourceSha256:digest(bytes),arguments:args,exitCode:result.status??null,signal:result.signal??null,error:result.error?String(result.error):null});
  record(structuredClone(results));
  if(result.error)throw result.error;
  assert.equal(result.signal,null,entry.id+': author interrupted');
  assert.equal(result.status,0,entry.id+': author failed; dependent stages must not run');
 }
 return results;
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 try{
  const args=process.argv.slice(2);
  if(args.length===1&&args[0]==='--write-plan'){
   writeFileSync(planFile,structuredDataToLino(readDevelopmentAuthoringPlan()));
  }else{
   const plan=readBootstrapStructuredDataLino(readFileSync(planFile,'utf8'));
   assert.deepEqual(plan,readDevelopmentAuthoringPlan(),'regenerate and review a stale plan');
   if(args.length===1&&args[0]==='--list'){
    for(const phase of plan.phases)console.log(phase.name+': '+phase.stages.map(entry=>entry.id).join(' -> '));
   }else{
    const check=args.at(-1)==='--check';if(check)args.pop();
    assert.ok(args.length===4&&args[0]==='--phase'&&args[2]==='--receipt','usage: refresh.mjs --list | --write-plan | --phase snapshot|history|registration|naming|verification --receipt output.lino [--check]');
    const revision=execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim();
    runDevelopmentAuthoringPhase(plan,args[1],{check,record:operations=>writeFileSync(resolve(args[3]),structuredDataToLino({schemaVersion:1,kind:'author invocation receipt',revision,phase:args[1],check,operations,boundary:plan.boundary}))});
   }
  }
 }catch(error){console.error(String(error?.stack??error));process.exitCode=1;}
}
