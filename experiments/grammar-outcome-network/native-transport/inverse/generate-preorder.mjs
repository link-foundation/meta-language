// Translate the complete iterative algorithm; retain independent finite controls.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';

export function generateOwnedOutcomePreorder({check=false}={}) {
  const source=readFileSync(new URL('./preorder-rules.mjs',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const declarations=['checkOwnedOutcomePreorder','ml_checkOwnedOutcomePreorder_loop2_result',
    'ml_checkOwnedOutcomePreorder_loop3_result','ml_checkOwnedOutcomePreorder_loop3',
    'ml_checkOwnedOutcomePreorder_loop2','ml_checkOwnedOutcomePreorder_loop1_result','ml_checkOwnedOutcomePreorder_loop1'];
  assert.deepEqual([...program.declarations.keys()],declarations,'every original and lifted control declaration is retained');
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,declarations.length);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_preorder_rules',{check,sourceStyles:readPortableLiteralOptionStyles});
  const fixture={schemaVersion:1,source:'experiments/grammar-outcome-network/native-transport/inverse/preorder-rules.mjs',
    sourceSha256:createHash('sha256').update(source).digest('hex'),declarations,
    cases:[
      {name:'one node',offsets:[0,0],children:[],limits:[1,1,1],expected:[0,1,0,1]},
      {name:'chain',offsets:[0,1,2,2],children:[2,3],limits:[3,2,3],expected:[0,3,2,3]},
      {name:'siblings',offsets:[0,2,2,2],children:[2,3],limits:[3,2,2],expected:[0,3,2,2]},
      {name:'nested siblings',offsets:[0,2,3,3,4,4],children:[2,4,3,5],limits:[5,4,3],expected:[0,5,4,3]},
      {name:'invalid node limit',offsets:[0,0],children:[],limits:[0,1,1],expected:[-1,0,0,0]},
      {name:'invalid edge limit',offsets:[0,0],children:[],limits:[1,0,1],expected:[-1,0,0,0]},
      {name:'invalid depth limit',offsets:[0,0],children:[],limits:[1,1,0],expected:[-1,0,0,0]},
      {name:'fractional limit',offsets:[0,0],children:[],limits:[1.5,1,1],expected:[-1,0,0,0]},
      {name:'missing node inventory',offsets:[],children:[],limits:[1,1,1],expected:[-2,0,0,0]},
      {name:'node bound',offsets:[0,1,2,2],children:[2,3],limits:[2,2,3],expected:[-2,0,0,0]},
      {name:'edge bound',offsets:[0,1,2,2],children:[2,3],limits:[3,1,3],expected:[-2,0,0,0]},
      {name:'depth bound',offsets:[0,1,2,2],children:[2,3],limits:[3,2,2],expected:[-5,0,0,0]},
      {name:'offset origin',offsets:[1,1],children:[],limits:[1,1,1],expected:[-2,0,0,0]},
      {name:'offset end',offsets:[0,1],children:[],limits:[1,1,1],expected:[-2,0,0,0]},
      {name:'reversed offsets',offsets:[0,2,1,2],children:[2,3],limits:[3,2,3],expected:[-2,0,0,0]},
      {name:'fractional offset',offsets:[0,0.5,1],children:[2],limits:[2,1,2],expected:[-2,0,0,0]},
      {name:'sibling order',offsets:[0,2,2,2],children:[3,2],limits:[3,2,2],expected:[-3,0,0,0]},
      {name:'duplicate owned child',offsets:[0,2,2,2],children:[2,2],limits:[3,2,3],expected:[-3,0,0,0]},
      {name:'self edge',offsets:[0,1,1],children:[1],limits:[2,1,2],expected:[-4,0,0,0]},
      {name:'foreign child',offsets:[0,1,1],children:[3],limits:[2,1,2],expected:[-4,0,0,0]},
      {name:'fractional child',offsets:[0,1,1],children:[1.5],limits:[2,1,2],expected:[-4,0,0,0]},
      {name:'unreachable owned row',offsets:[0,1,1,1],children:[2],limits:[3,1,2],expected:[-6,0,0,0]},
      {name:'pending reservation',offsets:[0,3,3,3],children:[2,2,3],limits:[3,3,2],expected:[-4,0,0,0]},
    ],boundary:'Complete shared preorder and resource algorithm only. Source types, scalars, native DTO construction, unresolved JSON variants, static borrows and physical CST integration remain separate obligations.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-preorder-rules.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return {files,fixture};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOwnedOutcomePreorder({check:process.argv[2]==='--check'});
  console.log('Translated the complete iterative preorder algorithm with '+result.fixture.cases.length+' independent controls.');
}

import {readPortableLiteralOptionStyles} from '../../../../js/scripts/portable-literal-option-style.mjs';
