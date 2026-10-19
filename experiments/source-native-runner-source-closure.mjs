// Exact source conservation for the CI process adapter; no native execution or grammar loading.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
export const SOURCE_NATIVE_RUNNER_FILE='experiments/source-native-observations.mjs';
export const SOURCE_NATIVE_RUNNER_BASELINE='7375572e830c9d7a686efe84fb57111cec7861a334851ecad8b5604cf86440a8';
export const SOURCE_NATIVE_RUNNER_VERIFIER_FILES=Object.freeze(['experiments/source-native-runner-source-closure.mjs']);
const sha256=value=>createHash('sha256').update(value).digest('hex');
export const SOURCE_NATIVE_RUNNER_HISTORICAL_BINDINGS=Object.freeze([
  {
    "producerCommit": "9123dd5d19fc26c95a9d31eff47dd9655c33da53",
    "commit": "7fd5bcaf21c6ef1f620181aeeaba8874dd70383c",
    "file": "experiments/original-link-complete-table-projection/raw-native-observations.mjs",
    "table": "sourceDigests",
    "bytes": 38687,
    "sha256": "9b88dd3d911374cc9e96175881b919c607f5e499c3a20e80581a18bfed4a1fbe",
    "bindingCount": 47,
    "bindingsSha256": "596bf8a7e8dfed09a1de977a40165adb09cde6105d053b25d3d847785f005b15"
  },
  {
    "producerCommit": "9a835e469572341e6a8e5c6e9fe6f40b8655dfbe",
    "commit": "80c34adc07aaef49e7c3e8b44f41f62f584f005f",
    "file": "experiments/original-link-complete-table-projection/native-observations.mjs",
    "table": "digests",
    "bytes": 38243,
    "sha256": "3cd027ed3403001c8467c43843a29b450ed5b78dc36a875524c9e287916c6188",
    "bindingCount": 35,
    "bindingsSha256": "e52bd42cc9c348a49d73fd39675378d6015ada45bac5238e113565b637f0b279"
  }
].map(Object.freeze));
export const SOURCE_NATIVE_RUNNER_CORRECTIONS=Object.freeze([
  {
    "name": "explicit compiler profile selection",
    "before": "import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';\nimport {tmpdir} from 'node:os';\nimport {dirname,join} from 'node:path';\nconst sha256=value=>createHash('sha256').update(value).digest('hex');\n/** Configure the existing whole-library byte-receipt/process protocol, with no process at import. */\nexport function createSourceNativeObservationRunner({nativeMode,checkedPlan,textBytes,parseObservations,entryReceipt,temporaryPrefix,failureLabel}){\nfunction prepareBundle(entry){\n const descriptor=Object.getOwnPropertyDescriptor(entry,'additionalSources');if(!descriptor)return null;\n assert.ok(Object.hasOwn(descriptor,'value')&&descriptor.enumerable,'additionalSources must be an owned data field');\n",
    "after": "import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';\nimport {tmpdir} from 'node:os';\nimport {dirname,join} from 'node:path';\nimport {readSourceNativeCompilerArguments,retainSourceNativeProcessDiagnostics} from './source-native-process-diagnostics.mjs';\nconst sha256=value=>createHash('sha256').update(value).digest('hex');\n/** Configure the existing whole-library byte-receipt/process protocol, with no process at import. */\nexport function createSourceNativeObservationRunner({nativeMode,checkedPlan,textBytes,parseObservations,entryReceipt,temporaryPrefix,failureLabel,compilerProfile='original'}){\nconst compilerArguments=readSourceNativeCompilerArguments(compilerProfile);\nfunction prepareBundle(entry){\n const descriptor=Object.getOwnPropertyDescriptor(entry,'additionalSources');if(!descriptor)return null;\n assert.ok(Object.hasOwn(descriptor,'value')&&descriptor.enumerable,'additionalSources must be an owned data field');\n"
  },
  {
    "name": "bounded diagnostic retention",
    "before": "}\nfunction runProcess(spawn,command,args,options,label){\n const result=spawn(command,args,{...options,shell:false,encoding:'utf8',maxBuffer:1024*1024});\n if(result.error)throw new Error(label+': '+(result.error.code==='ETIMEDOUT'?'timed out after '+options.timeout+'ms':result.error.message),{cause:result.error});\n if(result.signal)throw new Error(label+': terminated by '+result.signal);\n if(result.status!==0)throw new Error(label+': exit '+result.status+'; '+String(result.stderr??'').slice(0,4096));\n if(typeof result.stdout!=='string')throw new TypeError(label+': stdout must be actual UTF-8 text');\n return result.stdout;\n}\n",
    "after": "}\nfunction runProcess(spawn,command,args,options,label){\n const result=spawn(command,args,{...options,shell:false,encoding:'utf8',maxBuffer:1024*1024});\n const reason=result.error?(result.error.code==='ETIMEDOUT'?'timed out after '+options.timeout+'ms':String(result.error.message??result.error.code??'process failed').slice(0,4096))\n  :result.signal?'terminated by '+result.signal:result.status!==0?'exit '+result.status+'; '+String(result.stderr??'').slice(0,4096):null;\n if(reason!==null||(typeof result.stderr==='string'||Buffer.isBuffer(result.stderr))&&result.stderr.length>0){\n  let diagnostics;\n  try{diagnostics=retainSourceNativeProcessDiagnostics(result,{command,args,options,label,processMode:spawn===spawnSync?'native subprocesses':'injected process contract'});}\n  catch(error){throw new Error(label+': '+reason+'; retaining process diagnostics failed: '+String(error.message).slice(0,1024),{cause:result.error??error});}\n  if(reason!==null)throw new Error(label+': '+reason+'; process diagnostics: '+diagnostics.receiptPath,{cause:result.error});\n  process.stderr.write(label+': compiler diagnostics retained: '+diagnostics.receiptPath+'\\n');\n }\n if(typeof result.stdout!=='string')throw new TypeError(label+': stdout must be actual UTF-8 text');\n return result.stdout;\n}\n"
  },
  {
    "name": "optional diagnostic directory",
    "before": " checkFiles(entry,files);try{return runProcess(spawn,command,args,options,label);}finally{checkFiles(entry,files);}\n}\n/** Authorization precedes every process, including injected contract processes. */\nreturn function runSourceNativeObservations(plan,{environment=process.env,spawn=spawnSync,compiler='rustc',temporaryRoot=tmpdir(),compilerTimeout=30000,executionTimeout=10000}={}){\n if(!nativeMode(environment))throw new TypeError('native observations require the explicit CI-only switch');\n if(!checkedPlan(plan))throw new TypeError('native execution requires the exact checked in-memory observation plan');\n if(typeof spawn!=='function'||typeof compiler!=='string'||!compiler)throw new TypeError('the native runner requires an actual process function and compiler path');\n",
    "after": " checkFiles(entry,files);try{return runProcess(spawn,command,args,options,label);}finally{checkFiles(entry,files);}\n}\n/** Authorization precedes every process, including injected contract processes. */\nreturn function runSourceNativeObservations(plan,{environment=process.env,spawn=spawnSync,compiler='rustc',temporaryRoot=tmpdir(),compilerTimeout=30000,executionTimeout=10000,diagnosticsDirectory}={}){\n if(!nativeMode(environment))throw new TypeError('native observations require the explicit CI-only switch');\n if(!checkedPlan(plan))throw new TypeError('native execution requires the exact checked in-memory observation plan');\n if(typeof spawn!=='function'||typeof compiler!=='string'||!compiler)throw new TypeError('the native runner requires an actual process function and compiler path');\n"
  },
  {
    "name": "recorded compiler arguments",
    "before": "    for(const source of files.additionalSources){const path=join(directory,source.relativePath);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,source.bytes,{flag:'wx'});regularPath(directory,source.relativePath);}\n   }\n   writeFileSync(files.source,textBytes(entry.source));writeFileSync(files.consumer,entry.consumerSource);checkFiles(entry,files);\n   const options={cwd:directory,env:environment,timeout:compilerTimeout};\n   const libraryArgs=['--edition=2024','--crate-type=rlib','--crate-name','oracle_source',files.source,'-o',files.library];\n   processCounts.libraryCompilations++;runCheckedProcess(entry,files,spawn,compiler,libraryArgs,options,entry.id+' original library compiler');checkFiles(entry,files);assert.ok(existsSync(files.library),'the compiler did not create the original library');\n   const consumerArgs=['--edition=2024',files.consumer,'--extern','oracle_source='+files.library,'-o',files.executable];\n   processCounts.consumerCompilations++;runCheckedProcess(entry,files,spawn,compiler,consumerArgs,options,entry.id+' external consumer compiler');checkFiles(entry,files);assert.ok(existsSync(files.executable),'the compiler did not create the external consumer');\n   processCounts.executions++;const stdout=runCheckedProcess(entry,files,spawn,files.executable,[],{...options,timeout:executionTimeout},entry.id+' external original function execution');checkFiles(entry,files);\n   observations.push({id:entry.id,kind:entry.kind,fixtureId:entry.fixtureId,sourceBytes:entry.sourceBytes,sourceSha256:entry.sourceSha256,replaySha256:entry.replaySha256,\n",
    "after": "    for(const source of files.additionalSources){const path=join(directory,source.relativePath);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,source.bytes,{flag:'wx'});regularPath(directory,source.relativePath);}\n   }\n   writeFileSync(files.source,textBytes(entry.source));writeFileSync(files.consumer,entry.consumerSource);checkFiles(entry,files);\n   const options={cwd:directory,env:environment,timeout:compilerTimeout,...(diagnosticsDirectory===undefined?{}:{diagnosticsDirectory})};\n   const libraryArgs=['--edition=2024',...compilerArguments,'--crate-type=rlib','--crate-name','oracle_source',files.source,'-o',files.library];\n   processCounts.libraryCompilations++;runCheckedProcess(entry,files,spawn,compiler,libraryArgs,options,entry.id+' original library compiler');checkFiles(entry,files);assert.ok(existsSync(files.library),'the compiler did not create the original library');\n   const consumerArgs=['--edition=2024',...compilerArguments,files.consumer,'--extern','oracle_source='+files.library,'-o',files.executable];\n   processCounts.consumerCompilations++;runCheckedProcess(entry,files,spawn,compiler,consumerArgs,options,entry.id+' external consumer compiler');checkFiles(entry,files);assert.ok(existsSync(files.executable),'the compiler did not create the external consumer');\n   processCounts.executions++;const stdout=runCheckedProcess(entry,files,spawn,files.executable,[],{...options,timeout:executionTimeout},entry.id+' external original function execution');checkFiles(entry,files);\n   observations.push({id:entry.id,kind:entry.kind,fixtureId:entry.fixtureId,sourceBytes:entry.sourceBytes,sourceSha256:entry.sourceSha256,replaySha256:entry.replaySha256,\n"
  }
].map(Object.freeze));
export const SOURCE_NATIVE_RUNNER_ADDITIONS=Object.freeze([
  {
    "file": "experiments/source-native-process-diagnostics.mjs",
    "bytes": 3801,
    "sha256": "a10d0e6e94f4ffcbebe8ee1b0c12372ceb601184faaf4023b61eb8dd0f7d1a3a"
  },
  {
    "file": "experiments/source-native-compiler-profiles.lino",
    "bytes": 1725,
    "sha256": "21cc89627ca7e8a8d1c72c3a0b5bf07af68ef07acabb8a04a7d387ba12a08972"
  }
].map(Object.freeze));

/** Read a closed literal digest table without executing its source module. */
export function readSourceNativeHistoricalBindings(source,tableName){
 assert.ok(['sourceDigests','digests'].includes(tableName));
 const literal=new RegExp('^const '+tableName+'\\s*=\\s*(\\{[\\s\\S]*?^\\});','mu').exec(source)?.[1];
 assert.ok(literal,'the complete historical source table exists');
 const bindings={};
 for(const line of literal.slice(1,-1).split('\n').filter(line=>line.trim())){
  const match=/^\s*(['"])([^'"]+)\1:\s*(['"])([a-f0-9]{64})\3,?\s*$/u.exec(line);
  assert.ok(match,'historical source bindings must be literal path and digest pairs');
  assert.equal(Object.hasOwn(bindings,match[2]),false,'historical source bindings cannot repeat');
  Object.defineProperty(bindings,match[2],{value:match[4],enumerable:true,configurable:true,writable:true});
 }
 assert.equal(bindings['../source-native-observations.mjs'],SOURCE_NATIVE_RUNNER_BASELINE);
 return bindings;
}

/** Replay only the reviewed literal contexts over the authenticated old runner. */
export function replaySourceNativeRunnerCorrections(baseline){
 assert.equal(typeof baseline,'string');assert.equal(sha256(Buffer.from(baseline,'utf8')),SOURCE_NATIVE_RUNNER_BASELINE,'the original runner source is exact');
 let source=baseline;
 for(const correction of SOURCE_NATIVE_RUNNER_CORRECTIONS){
  const position=source.indexOf(correction.before);
  assert.ok(position>=0&&position===source.lastIndexOf(correction.before),'a reviewed runner context is missing or ambiguous: '+correction.name);
  source=source.slice(0,position)+correction.after+source.slice(position+correction.before.length);
 }
 return source;
}

/** Preserve every historical binding and prove the complete current source delta. */
export function verifySourceNativeRunnerSourceClosure(receipt,currentBytes,historicalSources,{producerCommit,additionalSources,verifierSources}){
 assert.equal(receipt.version,1);assert.match(producerCommit,/^[a-f0-9]{40}$/u);assert.equal(receipt.producerCommit,producerCommit);
 const historical=SOURCE_NATIVE_RUNNER_HISTORICAL_BINDINGS.find(row=>row.producerCommit===producerCommit);
 assert.ok(historical,'the original runner consumer is independently pinned');
 assert.deepEqual(receipt.historicalBinding,historical,'the historical consumer source receipt is exact');
 assert.equal(Object.keys(historicalSources).length,historical.bindingCount);
 assert.equal(sha256(JSON.stringify(Object.entries(historicalSources))),historical.bindingsSha256,'the complete ordered historical bindings are independently pinned');
 assert.deepEqual(Object.entries(receipt.historicalSources),Object.entries(historicalSources),'all original source bindings remain exact and ordered');
 assert.equal(historicalSources['../source-native-observations.mjs'],SOURCE_NATIVE_RUNNER_BASELINE);
 for(const value of Object.values(historicalSources))assert.match(value,/^[a-f0-9]{64}$/u);
 assert.equal(receipt.baseline.commit,producerCommit);assert.equal(receipt.baseline.file,SOURCE_NATIVE_RUNNER_FILE);
 assert.equal(typeof receipt.baseline.source,'string');assert.ok(receipt.baseline.source.isWellFormed());
 const baselineBytes=Buffer.from(receipt.baseline.source,'utf8');
 assert.equal(receipt.baseline.bytes,baselineBytes.length);assert.equal(receipt.baseline.sha256,SOURCE_NATIVE_RUNNER_BASELINE);
 assert.equal(sha256(baselineBytes),SOURCE_NATIVE_RUNNER_BASELINE,'the original runner bytes are authenticated');
 assert.deepEqual(receipt.corrections,SOURCE_NATIVE_RUNNER_CORRECTIONS,'only reviewed mechanical runner changes are authorized');
 const expectedBytes=Buffer.from(replaySourceNativeRunnerCorrections(receipt.baseline.source),'utf8');
 assert.ok(Buffer.isBuffer(currentBytes));assert.deepEqual(currentBytes,expectedBytes,'the complete current runner must equal the reviewed source replay');
 assert.equal(receipt.current.file,SOURCE_NATIVE_RUNNER_FILE);assert.equal(receipt.current.bytes,currentBytes.length);
 assert.equal(receipt.current.sha256,sha256(currentBytes),'the current runner full-file pin is exact');
 assert.deepEqual(receipt.additionalSources,SOURCE_NATIVE_RUNNER_ADDITIONS,'the helper and compiler policy have independently pinned complete sources');
 assert.deepEqual(Object.keys(additionalSources),SOURCE_NATIVE_RUNNER_ADDITIONS.map(row=>row.file));
 for(const expected of SOURCE_NATIVE_RUNNER_ADDITIONS){
  const bytes=additionalSources[expected.file];assert.ok(Buffer.isBuffer(bytes));assert.equal(bytes.length,expected.bytes);
  assert.equal(sha256(bytes),expected.sha256,'an added runner dependency changed: '+expected.file);
 }
 assert.deepEqual(Object.keys(verifierSources),SOURCE_NATIVE_RUNNER_VERIFIER_FILES);
 assert.deepEqual(receipt.verifierSources.map(row=>row.file),SOURCE_NATIVE_RUNNER_VERIFIER_FILES);
 for(const row of receipt.verifierSources){
  const bytes=verifierSources[row.file];assert.ok(Buffer.isBuffer(bytes));assert.equal(bytes.length,row.bytes);
  assert.equal(sha256(bytes),row.sha256,'the actual complete source-closure verifier is pinned');
 }
 assert.equal(receipt.scope,'source closure only; no native execution or performance claim');
 return{producerCommit,baselineSha256:SOURCE_NATIVE_RUNNER_BASELINE,sha256:receipt.current.sha256,
  corrections:SOURCE_NATIVE_RUNNER_CORRECTIONS.map(row=>row.name),additionalSources:SOURCE_NATIVE_RUNNER_ADDITIONS,verifierSources:receipt.verifierSources};
}
