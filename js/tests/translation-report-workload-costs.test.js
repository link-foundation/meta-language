import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync,readdirSync} from 'node:fs';
import {partitionReportMeasuredWorkloads} from '../scripts/report-workload-costs.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
const root=new URL('../../',import.meta.url);
const walk=directory=>readdirSync(new URL(directory,root),{withFileTypes:true}).flatMap(entry=>{
 const file=directory+'/'+entry.name;return entry.isDirectory()?walk(file):file.endsWith('.js')?[file]:[];
});
test('fresh source-bound report costs retain every complete current module once across the original eight jobs',()=>{
 const files=walk('js/src').sort(),costs=readProjectData(new URL('parity/fixtures/ci-39857274367-report-costs.lino',root));
 const shards=partitionReportMeasuredWorkloads(files,8);assert.equal(shards.length,8);
 assert.ok(shards.every(shard=>shard.length>0));assert.equal(new Set(shards.flat()).size,files.length);assert.deepEqual(shards.flat().sort(),files);
 const changed=partitionReportMeasuredWorkloads(files,8,{readSource:file=>Buffer.concat([readFileSync(new URL(file,root)),Buffer.from('\n')])});
 assert.deepEqual(changed.flat().sort(),files,'changed source uses an estimate and never omits a module');
 for(const path of ['.github/workflows/ci.yml','.github/workflows/js.yml']){
  const workflow=readFileSync(new URL(path,root),'utf8');
  assert.match(workflow,/(?:self-translation-report|reports):\n[\s\S]*?timeout-minutes: 30/u);
  assert.match(workflow,/shard: \[1, 2, 3, 4, 5, 6, 7, 8\]/u);
 }
 for(const mutate of [value=>{value.run=0;},value=>{value.inventory.pop();},value=>{value.entries.reverse();},value=>{value.entries[0].milliseconds=0;}]){
  const value=structuredClone(costs);mutate(value);assert.throws(()=>partitionReportMeasuredWorkloads(files,8,{costs:value}));
 }
 assert.throws(()=>partitionReportMeasuredWorkloads([...files,files[0]],8));
 const previous=readProjectData(new URL('parity/fixtures/ci-39848242758-report-costs.lino',root));
 assert.deepEqual(partitionReportMeasuredWorkloads(files,8,{costs:previous}).flat().sort(),files);
});
