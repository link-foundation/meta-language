import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';

export function generateOwnedOutcomeIntegerDecimalRules({check=false}={}) {
  const source=readFileSync(new URL('./integer-decimal-rules.mjs',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const declarations=['readOwnedOutcomeIntegerDecimalLimit','acceptOwnedOutcomeIntegerDecimal',
    'ml_acceptOwnedOutcomeIntegerDecimal_loop1_result','ml_acceptOwnedOutcomeIntegerDecimal_join2','ml_acceptOwnedOutcomeIntegerDecimal_loop1'];
  assert.deepEqual([...program.declarations.keys()],declarations,'retain both original functions and every actual lifted control declaration');
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,declarations.length);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_integer_decimal_rules',{check,sourceStyles:readPortableLiteralOptionStyles});
  const inputs=new Set(['','0','-0','00','-00','01','-01','+0','+1',' 1','1 ','\t1','1\n','1.0','1e0','0x10','NaN','Infinity','-Infinity','1_000','١','１','𝟙','\u0000','9'.repeat(21)]);
  for(let exponent=0n;exponent<=65n;exponent++)for(const delta of [-3n,-2n,-1n,0n,1n,2n,3n]){
    const value=(1n<<exponent)+delta;inputs.add(value.toString());inputs.add((-value).toString());
  }
  const cases=[];
  for(const width of [32,64])for(const signed of [false,true])for(const text of inputs){
    let integer=null;
    if(/^(?:0|[1-9][0-9]*|-[1-9][0-9]*)$/u.test(text))integer=BigInt(text);
    const minimum=signed?-(1n<<63n):0n,maximum=signed?(1n<<63n)-1n:(1n<<BigInt(width))-1n;
    const expected=integer!==null&&integer>=minimum&&integer<=maximum;
    cases.push({text,signed,width,expected,oracle:integer===null?null:integer.toString()});
  }
  const fixture={schemaVersion:1,source:'experiments/grammar-outcome-network/native-transport/inverse/integer-decimal-rules.mjs',
    sourceSha256:createHash('sha256').update(source).digest('hex'),declarations,cases,
    boundary:'Canonical decimal i64 and explicit 32/64-bit usize admission before binary64 projection. Independent JavaScript BigInt and native standard integer parsing are oracles. Effective native JSON number origins, complete DTO construction, static borrows and physical CST ownership remain separate obligations. Native execution is CI-only.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-integer-decimal-rules.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return{fixture,files};
}

import {readPortableLiteralOptionStyles} from '../../../../js/scripts/portable-literal-option-style.mjs';

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOwnedOutcomeIntegerDecimalRules({check:process.argv[2]==='--check'});
  console.log('Translated complete exact integer decimal admission against '+result.fixture.cases.length+' independent BigInt observations.');
}
