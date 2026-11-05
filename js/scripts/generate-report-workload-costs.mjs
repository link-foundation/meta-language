// Capture original CI timings; never execute the complete report locally.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
const revision='d3e4076667add3e19e4cf5f10a3cea60255dd2d7';
const jobs=[120067186411,120067186121,120067186381,120067186454,120067186314,120067186085,120067186340,120067186063];
const profiles={
 previous:{revision,run:39848242758,jobs},
 current:{revision:'f3853187c9e3395409361b60b4abe1608cdf1998',run:39857274367,jobs:[120098641287,120098641691,120098641270,120098641463,120098641252,120098641194,120098641377,120098641279]},
};
const digest=value=>createHash('sha256').update(value).digest('hex');
export function generateReportWorkloadCosts({directory,check=false,profile='previous'}={}){
 assert.ok(Object.hasOwn(profiles,profile),'unknown report scheduling source');
 const {revision,run,jobs}=profiles[profile];
 const entries=[],logs=[],seen=new Set();
 const git=args=>execFileSync('git',['--no-replace-objects',...args],{cwd:root,maxBuffer:2*1024*1024,timeout:15000});
 const inventory=git(['ls-tree','-r','--name-only',revision,'js/src']).toString('utf8').trim().split('\n').filter(file=>file.endsWith('.js')).sort();
 for(const[jobIndex,job]of jobs.entries()){
  const bytes=readFileSync(directory+'/job-'+job+'-small.txt'),receipt=JSON.parse(readFileSync(directory+'/job-'+job+'-small-receipt.json','utf8'));
  assert.ok(bytes.length<=262144);assert.equal(bytes.length,receipt.selectedBytes);assert.equal(digest(bytes),receipt.sha256);
  assert.equal(receipt.contentRange,'bytes 0-'+(bytes.length-1)+'/'+bytes.length,'only complete bounded logs supply costs');
  logs.push({job,shard:jobIndex+1,bytes:bytes.length,sha256:digest(bytes)});
  for(const line of bytes.toString('utf8').split('\n')){
   const match=/self-translation report: recorded (js\/src\/[^ ]+\.js) \(([0-9]+) ms\)/u.exec(line);if(!match)continue;
   const file=match[1],milliseconds=Number(match[2]);assert.ok(!seen.has(file));seen.add(file);
   assert.ok(Number.isSafeInteger(milliseconds)&&milliseconds>0);
   const source=git(['show',revision+':'+file]);entries.push({file,bytes:source.length,sha256:digest(source),milliseconds});
  }
 }
 entries.sort((left,right)=>left.file.localeCompare(right.file));
 assert.deepEqual([...seen].sort(),inventory,'all recorded modules must be present once, including the timed-out job');
 const packet={schemaVersion:1,model:'observed CI workload scheduling costs 1',run,revision,logs,inventory,entries,
  boundary:'Complete recorded-module elapsed durations from all eight original bounded CI logs, including the report whose job exceeded thirty minutes after successful report generation and upload. Costs include overlapped resolution and measurement and only choose scheduling owners. Source digests bind each sample; changed sources use the existing estimate. Every complete current module, original assertion, eight-cell inventory and thirty-minute job deadline remains mandatory. These timings are not requirement or native execution evidence.'};
 const file=new URL('../../parity/fixtures/ci-'+run+'-report-costs.lino',import.meta.url);if(check)checkProjectData(file,packet);else writeProjectData(file,packet);
 return{jobs:logs.length,modules:entries.length};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2),check=args.at(-1)==='--check';if(check)args.pop();assert.ok((args.length===2||args.length===4&&args[2]==='--profile')&&args[0]==='--logs');console.log(JSON.stringify(generateReportWorkloadCosts({directory:args[1],check,profile:args[3]??'previous'})));
}
