// Translate every declaration of the portable JavaScript inverse rule module.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';

export function generateOwnedOutcomeInverseRules({check=false}={}) {
  const source=readFileSync(new URL('./rules.mjs',import.meta.url),'utf8');
  const declarations=['acceptOwnedOutcomeTargetWidth','readOwnedOutcomeJsonKind','readOwnedOutcomeJsonFields','acceptOwnedOutcomeContainer'];
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  assert.deepEqual([...program.declarations.keys()].sort(),[...declarations].sort(),'the entire checked rule module is translated');
  const emitted=emitRust(program,{arrayConsumption:true});
  assert.equal(emitted.definitions.length,declarations.length);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_inverse_rules',{check});
  const fixture={schemaVersion:1,source:'experiments/grammar-outcome-network/native-transport/inverse/rules.mjs',
    sourceSha256:createHash('sha256').update(source).digest('hex'),declarations,
    widths:[[16,false],[32,true],[64,true],[128,false],[-1,false],[0,false]],
    kinds:[[true,false,'object','null'],[false,true,'object','array'],[false,false,'object','object'],
      [false,false,'number','number'],[false,false,'boolean','boolean'],[false,false,'string','string'],[false,false,'undefined','invalid']],
    fields:[['object',['kind','keys','aliasOf']],['number',['kind','bits']],['string',['kind','value']],['boolean',['kind','value']],['array',['kind','aliasOf']],['null',['kind']],['invalid',['kind']]],
    containers:[['Vec','',false,true],['IndexMap','',false,true],['struct_item','',false,true],['enum_item','',true,true],
      ['enum_item','',false,false],['json','array',false,true],['json','object',false,true],['json','number',false,false],['reference','object',true,false]],
    boundary:'Complete portable decision translation only; owned graph adapters, native DTO construction and physical CST ownership require separate implementation and execution.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-inverse-rules.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return {fixture,files};
}

if(process.argv[1]===fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOwnedOutcomeInverseRules({check:process.argv[2]==='--check'});
  console.log('Automatically translated all '+result.fixture.declarations.length+' inverse decisions; native execution remains a CI check.');
}
