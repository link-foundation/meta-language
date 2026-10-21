import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';
import {readOriginalOutcomeIntegerOracle} from './integer-history.mjs';

export function generateOwnedOutcomeIntegerRules({check=false}={}) {
  const source=readFileSync(new URL('./integer-rules.mjs',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  assert.deepEqual([...program.declarations.keys()],['acceptOwnedOutcomeIntegerValue']);
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,1);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_integer_rules',{check});
  const {history,oracle}=readOriginalOutcomeIntegerOracle();
  const values=new Set([0,1,-1,0.5,-0.5,Number.MIN_VALUE,-Number.MIN_VALUE,Number.MAX_VALUE,-Number.MAX_VALUE]);
  for(let exponent=0;exponent<=65;exponent++){
    const center=2**exponent,step=Math.max(1,2**(exponent-52));
    for(const delta of [-step,0,step]){values.add(center+delta);values.add(-center+delta);}
  }
  for(let exponent=53;exponent<=65;exponent++){
    const center=2**exponent,predecessorStep=2**(exponent-53);
    values.add(center-predecessorStep);values.add(-center+predecessorStep);
  }
  const cases=[];
  for(const width of [32,64])for(const signed of [false,true])for(const value of [...values,-0]){
    let output=null,refusal=null;
    try{output=oracle(value,signed?'i64':'usize',{usizeBits:width},{typeReference:'original integer oracle',path:null});}
    catch(error){assert.ok(['integer','integer-range'].includes(error.code));refusal=error.code;}
    const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);
    cases.push({name:(signed?'signed':'unsigned')+' '+width+' bit binary64 '+bytes.toString('hex'),
      value,bits:bytes.toString('hex'),signed,width,expected:refusal===null,
      originalOutput:output===null?null:{decimal:output.decimal,bits:output.bits},originalRefusal:refusal});
  }
  const fixture={schemaVersion:1,source:'experiments/grammar-outcome-network/native-transport/inverse/integer-rules.mjs',
    sourceSha256:createHash('sha256').update(source).digest('hex'),declarations:[...program.declarations.keys()],
    originalSource:{file:history.sources[0].file,revision:history.revision,sha256:history.sources[0].sha256,
      declarationSha256:history.declaration.sha256},cases,
    boundary:'Integer value admission against the exact executable original BigInt source. Decimal spelling, binary64 bit equality, complete source types, native JSON origins, static borrows, physical CST ownership and actual DTO construction remain separately required. Native execution is CI-only.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-integer-rules.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return{fixture,files};
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOwnedOutcomeIntegerRules({check:process.argv[2]==='--check'});
  console.log('Translated the complete integer admission module against '+result.fixture.cases.length+' original BigInt observations.');
}
