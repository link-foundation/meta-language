import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {DEFAULT_MAX_DEPTH} from '../src/grammar-runtime/executor.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);

export function generateConcreteProgramOriginFixtures({check=false}={}) {
  const rules={schemaVersion:1,models:['concrete-program-source-v1','typescript-portable-typing-v1'],
    languages:['JavaScript','Rust','TypeScript'],
    limits:{maxGraphLinks:200000,maxReadNodes:200000,maxDataDepth:2*DEFAULT_MAX_DEPTH+32,maxTreeDepth:DEFAULT_MAX_DEPTH,maxBytes:8388608},
    planes:['source','lowered'],
    boundary:'Read the currently stored physical concrete origin, including after semantic edits. Recheck graph/data bounds and physical spans. This is neither current semantic emission, grammar acceptance nor a proof that the origin and edited program are equivalent.'};
  const cases={schemaVersion:1,invalidLimits:[-1,1.5,'1',null],
    boundary:'Finite origin-reader controls use the independently authored concrete-program sources and the unchanged linked physical-span decisions. No compiler, parser replay, public package integration or native observation is supplied by these declarations.'};
  for(const[name,value]of [['concrete-program-origin-rules',rules],['concrete-program-origin',cases]]){
    const path=new URL('parity/fixtures/'+name+'.lino',root);
    if(check)checkProjectData(path,value);else writeProjectData(path,value);
  }
  return{rules,cases};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateConcreteProgramOriginFixtures({check:process.argv.includes('--check')});
}
