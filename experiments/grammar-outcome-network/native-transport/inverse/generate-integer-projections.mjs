// Translate the complete algorithm and its exact, unchanged decimal dependency.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {readPortableLiteralOptionStyles} from '../../../../js/scripts/portable-literal-option-style.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';

const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
export function readOwnedOutcomeIntegerProjectionSource() {
  const dependencyFile='experiments/grammar-outcome-network/native-transport/inverse/integer-decimal-rules.mjs';
  const sourceFile='experiments/grammar-outcome-network/native-transport/inverse/integer-projection-rules.mjs';
  const dependency=readFileSync(new URL('./integer-decimal-rules.mjs',import.meta.url),'utf8');
  const original=readFileSync(new URL('./integer-projection-rules.mjs',import.meta.url),'utf8');
  const declaration="import {acceptOwnedOutcomeIntegerDecimal} from './integer-decimal-rules.mjs';\n";
  assert.equal(original.split(declaration).length,2,'one exact explicit dependency import');
  const source=dependency+'\n'+original.replace(declaration,'');
  return{source,inputs:[{file:dependencyFile,...pin(dependency),sourceText:dependency},
    {file:sourceFile,...pin(original),sourceText:original}],assembly:{separator:'\n',removedImport:declaration},...pin(source)};
}

export function generateOwnedOutcomeIntegerProjections({check=false}={}) {
  const closure=readOwnedOutcomeIntegerProjectionSource();
  const program=checkProgram(parseJavaScript(closure.source,{referenceSemantics:false}));
  const declarations=[...program.declarations.keys()];
  assert.equal(declarations.filter(name=>name==='acceptOwnedOutcomeIntegerProjection').length,1);
  assert.ok(declarations.includes('acceptOwnedOutcomeIntegerDecimal'));
  const emitted=emitRust(program,{arrayConsumption:true});
  assert.equal(emitted.definitions.length,declarations.length,'every lifted control declaration remains');
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_integer_projection_rules',{check,sourceStyles:readPortableLiteralOptionStyles});
  const inputs=new Set(['','-0','00','01','-01','+1',' 1','1 ','1.0','1e0','NaN','Infinity','١','１','\ud800','9'.repeat(21)]);
  for(let exponent=0n;exponent<=65n;exponent++)for(const delta of [-3n,-2n,-1n,0n,1n,2n,3n]){
    const integer=(1n<<exponent)+delta;inputs.add(integer.toString());inputs.add((-integer).toString());
  }
  const cases=[];
  for(const width of [32,64])for(const signed of [false,true])for(const text of inputs){
    const value=Number(text),bits=Buffer.alloc(8);bits.writeDoubleBE(value);
    const canonical=/^(?:0|[1-9][0-9]*|-[1-9][0-9]*)$/u.test(text);
    const integer=canonical?BigInt(text):null,minimum=signed?-(1n<<63n):0n,maximum=signed?(1n<<63n)-1n:(1n<<BigInt(width))-1n;
    const expected=integer!==null&&integer>=minimum&&integer<=maximum&&Number.isFinite(value)
      &&Number.isInteger(value)&&!Object.is(value,-0)&&BigInt(value)===integer;
    cases.push({text,signed,width,projectedBits:bits.toString('hex'),expected});
  }
  const fixture={schemaVersion:1,closure,declarations,cases,
    boundary:'Canonical exact integer versus binary64 projection, independently checked with BigInt. Complete source division uses only bounded exact digit arithmetic. Native JSON variant capture, occurrence ownership, complete DTO construction and physical CST ownership remain separate requirements. Native execution is CI-only.'};
  const output=new URL('../../../../parity/fixtures/grammar-outcome-integer-projections.lino',import.meta.url);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return{fixture,files};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  const {fixture}=generateOwnedOutcomeIntegerProjections({check:process.argv[2]==='--check'});
  console.log('Generated every integer projection declaration and '+fixture.cases.length+' independent exact decimal controls.');
}
