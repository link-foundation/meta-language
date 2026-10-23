// Split complete logical cohorts; preserve serial execution and every workload.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {partitionGroupFiles,partitionMeasuredWorkloads,partitionProblems,partitionTests,testFiles} from './test-groups.mjs';
const packageRoot=fileURLToPath(new URL('../',import.meta.url));
export function partitionCiTestGroupFiles(group,files,shard='1/1'){
 const match=/^([1-9][0-9]*)\/([1-9][0-9]*)$/u.exec(shard),index=Number(match?.[1]),count=Number(match?.[2]);
 assert.ok(Number.isSafeInteger(index)&&Number.isSafeInteger(count)&&index<=count&&count<=files.length,'a physical shard requires its complete owned file inventory');
 assert.equal(new Set(files).size,files.length,'workload file identities are unique');
 const logicalCount=group==='translation'&&count===12?6:group==='conformance'&&count===8?4:null;
 if(logicalCount===null)return partitionGroupFiles(files,shard);
 const logical=partitionGroupFiles(files,`${(index-1)%logicalCount+1}/${logicalCount}`),half=Math.floor((index-1)/logicalCount);
 if(group==='translation')return partitionMeasuredWorkloads(logical.map(file=>'js/tests/'+file),2)[half].map(file=>file.slice(9));
 return partitionGroupFiles(logical,`${half+1}/2`);
}
export function readCiTestGroupFiles(group,shard='1/1'){
 const partition=partitionTests(testFiles()),problems=partitionProblems(partition);
 assert.deepEqual(problems,[],'every current test has exactly one logical group');
 assert.ok(Object.hasOwn(partition.groups,group),'unknown test group');
 return partitionCiTestGroupFiles(group,partition.groups[group],shard);
}
if(process.argv[1]&&path.resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 const[mode,group,...options]=process.argv.slice(2);
 assert.ok(['--list','--run'].includes(mode));assert.ok(options.length===0||options.length===2&&options[0]==='--shard');
 const files=readCiTestGroupFiles(group,options[1]??process.env.TEST_GROUP_SHARD??'1/1').map(file=>'tests/'+file);
 if(mode==='--list')console.log(files.join('\n'));
 else{const{status,signal}=spawnSync(process.execPath,['--test','--test-concurrency=1',...files],{cwd:packageRoot,stdio:'inherit'});process.exit(status??(signal?1:0));}
}
