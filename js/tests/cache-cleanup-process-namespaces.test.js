import assert from 'node:assert/strict';
import {mkdirSync,symlinkSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {excludeProcessAncestors,linuxProcesses} from '../../scripts/lib/cache-cleanup.mjs';
import {makeBase} from './support/cache-fixtures.js';

test('cleanup excludes ancestors before translating mounted procfs identifiers with colliding child identifiers',()=>{
  // The child API PID 9001 equals an ancestor's outer PID. Translating child
  // IDs before walking the outer parent chain incorrectly hides this child.
  const rows=[
    {pid:9001,ppid:1,namespacePid:1,executable:'/runner'},
    {pid:9010,ppid:9001,namespacePid:10,executable:'/node'},
    {pid:9020,ppid:9010,namespacePid:9001,executable:'/target/unit-fixture'},
    {pid:9030,ppid:9010,namespacePid:9002,executable:'/tools/cargo'},
    {pid:10,ppid:1,executable:'/foreign-namespace/worker'},
  ];
  assert.deepEqual(excludeProcessAncestors(rows,9010,0),[
    {pid:9001,ppid:10,executable:'/target/unit-fixture'},
    {pid:9002,ppid:10,executable:'/tools/cargo'},
    {pid:10,ppid:1,executable:'/foreign-namespace/worker'},
  ]);
  assert.deepEqual(rows[2],{pid:9020,ppid:9010,namespacePid:9001,executable:'/target/unit-fixture'});
  assert.deepEqual(excludeProcessAncestors([{pid:5,ppid:6},{pid:6,ppid:5},{pid:7,ppid:5}],5,0),[{pid:7,ppid:5}]);
  assert.deepEqual(excludeProcessAncestors([{pid:9,ppid:8},{pid:8,ppid:1},{pid:10,ppid:9}],9,8),[{pid:10,ppid:9}]);
});

test('procfs retains raw parent coordinates and translates only exact matching namespace status rows',(t)=>{
  const base=makeBase(t),proc=path.join(base,'proc');
  mkdirSync(path.join(proc,'self/ns'),{recursive:true});
  symlinkSync('pid:[4001]',path.join(proc,'self/ns/pid'));
  for(const [pid,namespace,status]of [
    [9020,'pid:[4001]','NSpid:\t9020\t9001\nNSpgid:\t1\n'],
    [9030,'pid:[4002]','NSpid:\t9030\t9002\n'],
    [9040,'pid:[4001]','NSpid:\t9040\tbroken\n'],
    [9050,'pid:[4001]','NSpid:\t\n9060\n'],
  ]){
    const entry=path.join(proc,String(pid));mkdirSync(path.join(entry,'ns'),{recursive:true});
    writeFileSync(path.join(entry,'stat'),`${pid} (unit fixture) S 9010 1 0`);
    writeFileSync(path.join(entry,'status'),status);
    writeFileSync(path.join(entry,'cmdline'),'unit-fixture\0');
    symlinkSync(base,path.join(entry,'cwd'));
    symlinkSync(namespace,path.join(entry,'ns/pid'));
  }
  const rows=linuxProcesses(proc);
  assert.deepEqual(rows.map(({pid,ppid,namespacePid})=>[pid,ppid,namespacePid]),[
    [9020,9010,9001],[9030,9010,undefined],[9040,9010,undefined],[9050,9010,undefined],
  ]);
  assert.deepEqual(excludeProcessAncestors(rows,9010,0).map(row=>row.pid),[9001,9030,9040,9050]);
});
