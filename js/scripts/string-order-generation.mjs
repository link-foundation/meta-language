// Generate closed selectors and rendering decisions from the shared Links model.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,serializeProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {STRING_PRELUDE} from '../src/translation/string-runtime.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';

export function generateStringOrderRules({check=false}={}) {
  const root=new URL('../../',import.meta.url),schema=readProjectData(new URL('js/src/data/typed-program-links-schema.lino',root));
  const model=schema.stringOrderModel;
  assert.equal(model.model,'ecmascript unicode code unit lexical order 1');
  assert.equal(model.sourceLanguage,'JavaScript');assert.equal(model.operandKind,'string');assert.equal(model.codeUnitWidth,16);
  assert.equal(model.nativeType,'crate::ml_string::Text');assert.equal(model.nativeTrait,'std::cmp::PartialOrd');
  assert.equal(createHash('sha256').update(STRING_PRELUDE).digest('hex'),model.nativePreludeSha256);
  const expected=[['lt','unicodeCodeUnitLessThan','<'],['le','unicodeCodeUnitLessThanOrEqual','<='],['gt','unicodeCodeUnitGreaterThan','>'],['ge','unicodeCodeUnitGreaterThanOrEqual','>=']];
  assert.deepEqual(model.relations,expected.map(([sourceOperator,operation,symbol])=>({sourceOperator,operation,symbol,nativeMethod:sourceOperator})));
  const history=readProjectData(new URL('parity/naming/unicode-string-order-source-history.lino',root));
  for(const prefix of history.immutablePrefixes){
    const bytes=readFileSync(new URL(prefix.file,root));assert.ok(bytes.length>=prefix.bytes);
    assert.equal(createHash('sha256').update(bytes.subarray(0,prefix.bytes)).digest('hex'),prefix.sha256);
  }
  for(const file of ['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino']){
    const current=readProjectData(new URL(file,root));assert.deepEqual(current,schema);
    const inverse=structuredClone(current);delete inverse.stringOrderModel;
    assert.deepEqual(inverse.expressionOperations.stringTest.splice(-4),model.relations.map(row=>row.operation));
    const original=history.sources.find(row=>row.file===file);assert.ok(original);
    assert.deepEqual(inverse,readStructuredDataLino(original.sourceText));
    assert.equal(serializeProjectData(inverse,file),original.sourceText);
  }
  const quote=JSON.stringify;
  const selected=model.relations.map(row=>'operation === '+quote(row.operation)).join(' || ');
  const source='// Generated from the complete shared stringOrderModel. Do not edit by hand.\n\n'
    +'/** @param {string} language @param {string} operation @param {string} kind @returns {string} */\n'
    +'export function readStringOrderOperation(language, operation, kind) {\n'
    +'  if (language !== '+quote(model.sourceLanguage)+' || kind !== '+quote(model.operandKind)+') return "";\n'
    +model.relations.map(row=>'  if (operation === '+quote(row.sourceOperator)+') return '+quote(row.operation)+';').join('\n')+'\n  return "";\n}\n\n'
    +'/** @param {string} expression @param {string} operation @returns {boolean} */\n'
    +'export function requireStringOrderUnits(expression, operation) {\n  return expression === "stringTest" && ('+selected+');\n}\n\n'
    +'/** @param {string} target @param {string} operation @param {string} left @param {string} right @returns {string} */\n'
    +'export function renderStringOrderExpression(target, operation, left, right) {\n'
    +model.relations.map(row=>'  if (operation === '+quote(row.operation)+') {\n'
      +'    if (target === "JavaScript") return "((" + left + ") '+row.symbol+' (" + right + "))";\n'
      +'    if (target === "Rust") return '+quote('<'+model.nativeType+' as '+model.nativeTrait+'>::'+row.nativeMethod+'(')+' + left + ", " + right + ")";\n  }').join('\n')
    +'\n  return "";\n}\n\n'
    +'/** @param {string} target @param {string} operation @returns {string} */\n'
    +'export function readStringOrderRefusal(target, operation) {\n'
    +'  if (!requireStringOrderUnits("stringTest", operation) || target === "JavaScript" || target === "Rust") return "";\n'
    +'  return "this target needs a UTF-16 operational string encoding for lexical comparison";\n}\n';
  const javascript=new URL('../src/translation/string-order-rules.js',import.meta.url);
  if(check)assert.equal(readFileSync(javascript,'utf8'),source);else writeFileSync(javascript,source);
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const declarations=['readStringOrderOperation','requireStringOrderUnits','renderStringOrderExpression','readStringOrderRefusal',
    'ml_renderStringOrderExpression_join4','ml_renderStringOrderExpression_join3','ml_renderStringOrderExpression_join2','ml_renderStringOrderExpression_join1'];
  assert.deepEqual([...program.declarations.keys()],declarations,'retain every original and lifted continuation declaration');
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.definitions.length,declarations.length);
  assert.equal(emitted.preludes.length,0,'the complete model selectors remain closed scalar decisions');
  const files=writeRecordRuleLibrary(emitted,'string_order_rules',{check});
  const values=['','A','AA','AB','B','\u0000','\uffff','\ue000','😀','\ud800','\udc00','\ud800\udc00',
    '\udbff\udfff','é','e\u0301','a','a\u0000','a\ud800','a\udc00','a😀','\ufffd','Z','0','9','-','00','a'.repeat(64)];
  const units=values.map(value=>Array.from({length:value.length},(_,index)=>value.charCodeAt(index)));
  const cases=[];
  for(let left=0;left<values.length;left++)for(let right=0;right<values.length;right++)for(const row of model.relations){
    const a=values[left],b=values[right],op=row.sourceOperator;
    const expected=op==='lt'?a<b:op==='le'?a<=b:op==='gt'?a>b:a>=b;
    cases.push({left,right,operator:op,operation:row.operation,expected});
  }
  const fixture={schemaVersion:1,model:model.model,nativePreludeSha256:model.nativePreludeSha256,units,cases,
    boundary:'Independent native JavaScript primitive string relational observations, preserving explicit UTF16 input units. Shared selectors and actual emitted programs execute separately. No local native execution or arbitrary Rust String source equivalence is claimed.'};
  const fixtureFile=new URL('parity/fixtures/unicode-string-order.lino',root);
  if(check)checkProjectData(fixtureFile,fixture);else writeProjectData(fixtureFile,fixture);
  return{source,model,files,fixture};
}
