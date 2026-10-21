import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../../../../js/src/translation/javascript.js';
import {checkProgram} from '../../../../js/src/translation/check.js';
import {emitRust} from '../../../../js/src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from '../../../../js/scripts/record-rule-library.mjs';
import {writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';

export function generateOwnedOutcomeStaticBorrowRules({check=false}={}) {
  const source=readFileSync(new URL('./static-borrow-rules.mjs',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const declarations=[...program.declarations.keys()];assert.ok(declarations.includes('acceptOwnedOutcomeStaticBorrowResources')&&declarations.includes('findOwnedOutcomeStaticBorrowIndex'));
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,declarations.length);
  const files=writeRecordRuleLibrary(emitted,'owned_outcome_static_borrow_rules',{check});
  const values=['','syntax','ambiguity','nestingDepth','stepLimit','😀','é','e\u0301','\0','\r\n'];
  const registries=[[],...values.map(value=>[value]),values,values.toReversed(),['syntax','syntax'],['missing','missing','syntax'],...Array.from({length:8},(_,shift)=>values.slice(shift).concat(values.slice(0,shift)))];
  const cases=[];
  for(const registry of registries)for(const text of [...values,'missing']){
    const bytes=Buffer.byteLength(text)+registry.reduce((sum,value)=>sum+Buffer.byteLength(value),0);
    const expected=new Set(registry).size!==registry.length?-2:registry.indexOf(text);
    cases.push({text,registry,bytes,expected});
  }
  for(const count of [127,128,129]){
    const registry=Array.from({length:count},(_,index)=>'entry '+index),text=registry.at(-1);
    const bytes=Buffer.byteLength(text)+registry.reduce((sum,value)=>sum+Buffer.byteLength(value),0);
    cases.push({text,registry,bytes,expected:count>128?-3:count-1});
  }
  const fixture={schemaVersion:1,source:'experiments/grammar-outcome-network/native-transport/inverse/static-borrow-rules.mjs',sourceSha256:createHash('sha256').update(source).digest('hex'),declarations,cases,
    boundary:'Complete shared selection from unique caller-owned existing static string references. Native storage supplies actual compiler-checked static references and exact UTF8 byte costs. Selection and resource admission do not reconstruct complete native DTOs, discharge JSON number origins or establish physical CST ownership. Native execution is CI-only.'};
  const target=new URL('../../../../parity/fixtures/grammar-outcome-static-borrows.lino',import.meta.url);
  if(check)checkProjectData(target,fixture);else writeProjectData(target,fixture);
  return{fixture,files};
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--check','--write'].includes(process.argv[2]));
  const result=generateOwnedOutcomeStaticBorrowRules({check:process.argv[2]==='--check'});
  console.log('Translated every static-borrow source declaration; '+result.fixture.cases.length+' independent registry selections.');
}
