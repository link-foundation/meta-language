import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';

export function generateNumberPredicateRules({check=false}={}) {
  const root=new URL('../../',import.meta.url);
  const model=readProjectData(new URL('js/src/translation/data/number-predicate-model.lino',root));
  assert.deepEqual(readProjectData(new URL('rust/src/data/number-predicate-model.lino',root)),model);
  assert.equal(model.model,'ecmascript noncoercing number predicate 1');
  assert.deepEqual(model.operations,['isInteger','isSafeInteger','isFinite','isNaN']);
  assert.equal(model.sourceLanguage,'JavaScript');assert.equal(model.numberKind,'float');assert.equal(model.optionalKind,'option');
  assert.deepEqual(model.forms,{number:'number',optionalNumber:'optional-number',nonNumber:'false',unsupported:'unsupported'});
  assert.deepEqual(model.nonNumberKinds,['bool','string','unit','int','nat','array','tuple','record','map','set','function','data']);
  assert.deepEqual(model.argumentModel,{missing:'undefined',resultArgument:0,evaluation:'all supplied expressions once in source order',extraArguments:'evaluated and ignored',spread:'requires a separate iterable argument expansion model'});
  const quote=JSON.stringify;
  const source='// Generated from the complete shared number predicate model. Do not edit.\n\n'
    +'/** @param {string} language @param {string} operation @param {string} kind @param {string} element @returns {string} */\n'
    +'export function readNumberPredicateArgumentForm(language, operation, kind, element) {\n'
    +'  if (language !== '+quote(model.sourceLanguage)+' || !('+model.operations.map(op=>'operation === '+quote(op)).join(' || ')+')) return "";\n'
    +'  if (kind === '+quote(model.numberKind)+') return '+quote(model.forms.number)+';\n'
    +'  if (kind === '+quote(model.optionalKind)+' && element === '+quote(model.numberKind)+') return '+quote(model.forms.optionalNumber)+';\n'
    +'  if ('+model.nonNumberKinds.map(kind=>'kind === '+quote(kind)).join(' || ')+') return '+quote(model.forms.nonNumber)+';\n'
    +'  if (kind === '+quote(model.optionalKind)+' && ('+model.nonNumberKinds.map(kind=>'element === '+quote(kind)).join(' || ')+')) return '+quote(model.forms.nonNumber)+';\n'
    +'  return '+quote(model.forms.unsupported)+';\n}\n';
  const file=new URL('../src/translation/number-predicate-rules.js',import.meta.url);
  if(check)assert.equal(readFileSync(file,'utf8'),source);else writeFileSync(file,source);
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  assert.deepEqual([...program.declarations.keys()],['readNumberPredicateArgumentForm']);
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.preludes.length,0);
  const files=writeRecordRuleLibrary(emitted,'number_predicate_rules',{check});
  const values=["'1'","'NaN'",'true','false','null','undefined','1n','[1,2]','{value:1}', 'new Map([[1,2]])','new Set([1,2])','() => 1','0','-0','1','1.5','NaN','Infinity','9007199254740992'];
  const cases=model.operations.flatMap(operation=>values.map(expression=>({operation,expression,expected:Number[operation](runInNewContext(expression.startsWith('{')?'('+expression+')':expression))})).concat({operation,expression:'',expected:Number[operation]()}));
  const fixture={schemaVersion:1,model:model.model,cases,boundary:'Actual independent Node primitive predicate observations on represented numeric and nonnumeric source expressions. Static typing, source-free rechecks, ordered extra effects and native execution are checked separately.'};
  const target=new URL('parity/fixtures/number-predicate-observations.lino',root);
  if(check)checkProjectData(target,fixture);else writeProjectData(target,fixture);
  return{model,source,files,fixture};
}
