import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {readTypedBooleanParameterStyles} from '../../../../js/scripts/typed-boolean-source-style.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';

export function generateOwnedOutcomeNumberKindRules({check=false}={}) {
  const source=readFileSync(new URL('./number-kind-rules.mjs',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const declarations=['readOwnedOutcomeNativeNumberKind'];
  assert.deepEqual([...program.declarations.keys()],declarations);
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,1);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_number_kind_rules',{check,sourceStyles:source=>readTypedBooleanParameterStyles(source,'read_owned_outcome_native_number_kind')});
  // Independent exhaustive predicate table from the pinned dependency source.
  const meanings=['','floating point','unsigned integer','','signed integer','','unsigned integer',''];
  const cases=[];
  for(const profileAvailable of [false,true])for(const version of ['', '1.0.150','1.0.151','1.0.152','1.0.151-extra']){
    for(const arbitraryPrecision of [false,true])for(let flags=0;flags<8;flags++){
      cases.push({profileAvailable,version,arbitraryPrecision,signed:Boolean(flags&4),unsigned:Boolean(flags&2),floating:Boolean(flags&1),
        expected:profileAvailable&&version==='1.0.151'&&!arbitraryPrecision?meanings[flags]:''});
    }
  }
  const fixture={schemaVersion:1,source:'experiments/grammar-outcome-network/native-transport/inverse/number-kind-rules.mjs',
    sourceSha256:createHash('sha256').update(source).digest('hex'),declarations,
    model:{dependency:'serde_json',versionRange:'>=1.0.151 <1.0.152',excludedFeature:'arbitrary_precision',
      predicates:['is_i64','is_u64','is_f64'],authority:'https://docs.rs/serde_json/1.0.151/src/serde_json/number.rs.html'},cases,
    boundary:'A complete predicate selector under an explicitly authenticated linked dependency profile. These synthetic control inputs are not an actual compiler feature attestation or original number observation. Unknown profiles and contradictory predicates refuse. Exact payloads, source occurrence ownership and complete native DTO reconstruction remain separate obligations. Native execution is CI-only.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-number-kind-rules.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return{fixture,files};
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const result=generateOwnedOutcomeNumberKindRules({check:process.argv[2]==='--check'});
  console.log('Translated complete native number predicate selection against '+result.fixture.cases.length+' independent bounded-profile controls.');
}
