import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync,readdirSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {partitionReportMeasuredWorkloads,selectReportPhysicalPart} from '../scripts/report-workload-costs.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
const root=new URL('../../',import.meta.url);
const walk=directory=>readdirSync(new URL(directory,root),{withFileTypes:true}).flatMap(entry=>{
 const file=directory+'/'+entry.name;return entry.isDirectory()?walk(file):file.endsWith('.js')?[file]:[];
});

test('the actual report command selects both complete physical parts and refuses unowned scope',()=>{
 const script=fileURLToPath(new URL('../scripts/generate-self-translation-report.mjs',import.meta.url));
 const list=args=>spawnSync(process.execPath,['--max-old-space-size=192',script,'--list',...args],{encoding:'utf8',timeout:20000});
 const original=list(['--shard','2/8']);assert.equal(original.status,0,original.stderr);
 const expected=original.stdout.trim().split('\n'),parts=[];
 for(const part of ['1/2','2/2']){
  const actual=list(['--shard','2/8','--physical-part',part]);assert.equal(actual.status,0,actual.stderr);
  parts.push(...actual.stdout.trim().split('\n'));
 }
 assert.equal(new Set(parts).size,expected.length);assert.deepEqual(parts.sort(),expected.sort());
 for(const args of [['--physical-part','1/2'],['--shard','2/8','--physical-part','3/2'],['--shard','2/8','--modules','primitives.js','--physical-part','1/2']]){
  const actual=list(args);assert.notEqual(actual.status,0);assert.equal(actual.stdout,'');
 }
});

test('two physical report parts preserve every complete module in each of the eight logical cells',()=>{
 const files=walk('js/src').sort(),logical=partitionReportMeasuredWorkloads(files,8),physical=[];
 for(const cell of logical){
  const parts=['1/2','2/2'].map(part=>selectReportPhysicalPart(cell,part));
  assert.ok(parts.every(part=>part.length>0));assert.equal(new Set(parts.flat()).size,cell.length);
  assert.deepEqual(parts.flat().sort(),cell.slice().sort(),'the complete logical cell remains mandatory');
  assert.deepEqual(selectReportPhysicalPart(cell,'1/1').slice().sort(),cell.slice().sort());
  for(const geometry of ['0/2','3/2','1/0','1/999','1/2extra','9007199254740993/9007199254740993'])assert.throws(()=>selectReportPhysicalPart(cell,geometry));
  assert.throws(()=>selectReportPhysicalPart([...cell,cell[0]],'1/2'));
  physical.push(...parts);
 }
 assert.equal(physical.length,16);assert.deepEqual(physical.flat().sort(),files);
 for(const path of ['.github/workflows/ci.yml','.github/workflows/js.yml']){
  const workflow=readFileSync(new URL(path,root),'utf8');
  assert.match(workflow,/part: \[1, 2\]/u);
  assert.ok(workflow.includes('--physical-part ${{ matrix.part }}/2'));
  assert.ok(workflow.includes('name: self-translation-report-${{ github.sha }}-${{ matrix.shard }}-${{ matrix.part }}'));
  assert.match(workflow,/pattern: self-translation-report-\$\{\{ github.sha \}\}-\*/u);
 }
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
