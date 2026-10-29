// Write independent literal source inputs only; never parse or execute them.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/source-library-constant-functions.lino',import.meta.url),{
 schemaVersion:1,model:'actual source module function bindings 1',
 versions:{node:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rust:'>=1.90.0 <1.100.0; edition 2024'},
 headNames:[{kind:'identifier',expected:true},{kind:'keyword',expected:false},{kind:'punctuation',expected:false},{kind:'',expected:false}],
 classExpressionTerms:[{term:'class',expected:true},{term:'class_declaration',expected:false},{term:'class_body',expected:false},{term:'identifier',expected:false}],
 catchScopeTerms:[{term:'catch_clause',expected:true},{term:'try_statement',expected:false},{term:'statement_block',expected:false},{term:'formal_parameters',expected:false}],
 loopScopeTerms:[{term:'for_statement',expected:true},{term:'for_in_statement',expected:true},{term:'for_of_statement',expected:true},{term:'while_statement',expected:false},{term:'identifier',expected:false}],
 expressionTerms:[{term:'function_expression',expected:true},{term:'generator_function',expected:true},{term:'function_declaration',expected:false},{term:'arrow_function',expected:false},{term:'identifier',expected:false}],
 keywordRoles:[{identifier:true,keyword:true,expected:false},{identifier:true,keyword:false,expected:false},{identifier:false,keyword:true,expected:true},{identifier:false,keyword:false,expected:false}],
 wordBindings:['as','async','from','of','get','set','meta','target'].map(word=>({name:'actual contextual parameter '+word,source:'export function inspect('+word+'){return '+word+';}'+word+';',bindings:[['inspect','function',0],[word,'parameter',1]],unresolved:[word]})),
 wordRenames:['as','async','from','of','get','set','meta','target'].map(word=>({source:'export function inspect(value){return value;}',binding:'value',replacement:word,expected:'export function inspect('+word+'){return '+word+';}'})),
 reservedRenames:['if','await','true','enum','implements','eval','arguments'],
 functionScopeForms:[{term:'function_expression',expected:'function expression'},{term:'generator_function',expected:'function expression'},{term:'function_declaration',expected:'function declaration'},{term:'generator_function_declaration',expected:'function declaration'},{term:'arrow_function',expected:'arrow function'},{term:'method_definition',expected:'method definition',formerDraftExpectation:''}],
 simpleParameterLists:[{terms:['formal_parameters','identifier'],expected:true},{terms:['formal_parameters','assignment_pattern','identifier'],expected:false},{terms:['formal_parameters','rest_pattern'],expected:false},{terms:['formal_parameters','object_pattern'],expected:false},{terms:['formal_parameters','array_pattern'],expected:false}],
 variableRedeclarations:[{kind:'var',existing:true,expected:true},{kind:'var',existing:false,expected:false},{kind:'parameter',existing:true,expected:false},{kind:'const',existing:true,expected:false}],
 parameterVariableDependencies:[{first:'parameter',second:'var',bodyParent:true,expected:true},{first:'var',second:'parameter',bodyParent:true,expected:true},{first:'parameter',second:'var',bodyParent:false,expected:false},{first:'const',second:'var',bodyParent:true,expected:false}],
 parameterEnvironments:[
  {name:'literal words never fabricate unresolved identifier reads',source:'const first=true;const second=false;const third=null;',bindings:[['first','const',0],['second','const',0],['third','const',0]],unresolved:[],parameterScopeCount:0},
  {name:'arrow parameters do not capture preceding source reads',source:'const before=value;const read=value=>value;',bindings:[['before','const',0],['read','const',0],['value','parameter',1]],unresolved:['value'],parameterScopeCount:1},
  {name:'separate arrows retain separate parameter environments',source:'const first=value=>value;const second=value=>value;',bindings:[['first','const',0],['value','parameter',1],['second','const',0],['value','parameter',1]],unresolved:[],parameterScopeCount:2},
  {name:'named defaults retain earlier parameters without leaking them',source:'function read(first=1,second=first){return second;}first;',bindings:[['read','function',0],['first','parameter',1],['second','parameter',1]],unresolved:['first'],parameterScopeCount:1},
  {name:'arrow defaults retain their actual parameter environment',source:'const read=(first=1,second=first)=>second;first;',bindings:[['read','const',0],['first','parameter',1],['second','parameter',1]],unresolved:['first'],parameterScopeCount:1},
  {name:'simple named parameters and var redeclarations share one binding',source:'function read(value){var value;return value;}',bindings:[['read','function',0],['value','parameter',2]],unresolved:[],parameterScopeCount:1,rename:{name:'value',kind:'parameter',replacement:'renamedValue',expected:'function read(renamedValue){var renamedValue;return renamedValue;}'}},
  {name:'simple arrow parameters retain their var redeclaration aliases',source:'const read=(value)=>{var value;return value;};',bindings:[['read','const',0],['value','parameter',2]],unresolved:[],parameterScopeCount:1,rename:{name:'value',kind:'parameter',replacement:'renamedValue',expected:'const read=(renamedValue)=>{var renamedValue;return renamedValue;};'}},
  {name:'non-simple parameters retain separate initialized body cells',source:'function read(value=1){var value;return value;}',bindings:[['read','function',0],['value','parameter',0],['value','var',1]],unresolved:[],parameterScopeCount:1,dependentRenames:[{name:'value',kind:'parameter'},{name:'value',kind:'var'}]},
  {name:'repeated function var declarations retain one actual cell',source:'function read(){var value=1;var value;return value;}',bindings:[['read','function',0],['value','var',2]],unresolved:[],parameterScopeCount:0,rename:{name:'value',kind:'var',replacement:'renamedValue',expected:'function read(){var renamedValue=1;var renamedValue;return renamedValue;}'}},
  {name:'repeated module var declarations retain one actual cell',source:'var value=1;var value;value;',bindings:[['value','var',2]],unresolved:[],parameterScopeCount:0,rename:{name:'value',kind:'var',replacement:'renamedValue',expected:'var renamedValue=1;var renamedValue;renamedValue;'}}
 ],
 namedBindings:[
  {name:'private expression names cover defaults and exclude outer reads',source:'const outer=function inner(value=inner){return value;};inner;',bindings:[['outer','const',0],['inner','function',1],['value','parameter',1]],unresolved:['inner']},
  {name:'later parameters shadow the private expression environment',source:'const outer=function inner(value=inner,inner=1){return value;};',bindings:[['outer','const',0],['inner','function',0],['value','parameter',1],['inner','parameter',1]],unresolved:[]},
  {name:'parameters shadow private expression names without leaking',source:'const outer=function inner(inner){return inner;};inner;',bindings:[['outer','const',0],['inner','function',0],['inner','parameter',1]],unresolved:['inner']},
  {name:'recursive private references retain their own expression owner',source:'const outer=function inner(value){return inner(value);};inner;',bindings:[['outer','const',0],['inner','function',1],['value','parameter',1]],unresolved:['inner']}
 ],
 declarationOffsets:[
  {first:'function',second:'apply',lineBreak:false,expected:0},
  {first:'function',second:'apply',lineBreak:true,expected:0},
  {first:'async',second:'function',lineBreak:false,expected:1},
  {first:'async',second:'function',lineBreak:true,expected:-1},
  {first:'async',second:'apply',lineBreak:false,expected:-1},
  {first:'const',second:'apply',lineBreak:false,expected:-1}
 ],
 anonymousBindings:[
  {name:'anonymous function parameters are not module names',source:'const outer=function(value){return value;};',bindings:[['outer','const',0],['value','parameter',1]]},
  {name:'anonymous generator parameters are not module names',source:'const outer=function*(value){yield value;};',bindings:[['outer','const',0],['value','parameter',1]]},
  {name:'anonymous async parameters are not module names',source:'const outer=async function(value){return value;};',bindings:[['outer','const',0],['value','parameter',1]]},
  {name:'anonymous empty parameter groups do not steal later declarations',source:'const outer=function(){return 1;};const sibling=2;',bindings:[['outer','const',0],['sibling','const',0]]}
 ],
 policies:[
  {keyword:'function',kind:'identifier',name:'apply',callable:false,binding:'function',exported:true},
  {keyword:'const',kind:'identifier',name:'apply',callable:true,binding:'const',exported:true},
  {keyword:'const',kind:'identifier',name:'amount',callable:false,binding:'',exported:false},
  {keyword:'let',kind:'identifier',name:'apply',callable:true,binding:'',exported:false},
  {keyword:'function',kind:'string',name:'apply',callable:true,binding:'',exported:false},
  {keyword:'function',kind:'identifier',name:'',callable:true,binding:'',exported:false}
 ],
 facts:[
  {name:'constant arrow exports retain actual const bindings',source:'export const apply=value=>value;',exports:['apply'],bindings:[{name:'apply',kind:'const',initializer:'arrow'}]},
  {name:'ordinary functions retain their existing declaration form',source:'export function apply(value){return value;}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'async function exports retain their actual names',source:'export async function apply(value){return value;}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'async comments without line breaks preserve the head',source:'export async /* preserved */ function apply(value){return value;}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'async line breaks cannot fabricate export declarations',source:'export async\nfunction apply(value){return value;}',error:true},
  {name:'async comment line breaks cannot fabricate export declarations',source:'export async /*\n*/ function apply(value){return value;}',error:true},
  {name:'anonymous function expressions retain their actual outer binding',source:'export const apply=function(value){return value;};',exports:['apply'],bindings:[{name:'apply',kind:'const',initializer:'function expression'}]},
  {name:'named function expressions are not module declarations',source:'export const apply=function apply(value){return value;};',exports:['apply'],bindings:[{name:'apply',kind:'const',initializer:'function expression',expressionName:'apply'}]},
  {name:'functions inside default parameter groups remain nested',source:'export function apply(read=function nested(){return 1;}){return read();}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'source-like literal text does not declare functions',source:'const words=["function phantom(){}","const fake=()=>1"];export function apply(){return 1;}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'scalar constant exports retain their complete separate obligation',source:'export const amount=3;',error:true},
  {name:'duplicate actual module declarations refuse before renaming',source:'export const apply=value=>value;function apply(value){return value;}',error:true}
 ],
 modules:[
  {name:'standalone exported constant arrow',entry:'entry.js',aliases:false,arguments:[4],expected:5,files:[{path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=value=>value+1;'}]},
  {name:'standalone constant anonymous function',entry:'entry.js',aliases:false,arguments:[4],expected:4,files:[{path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=function(value){return value;};'}]},
  {name:'standalone same-name function expression',entry:'entry.js',aliases:false,arguments:[4],expected:4,files:[{path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=function run(value){return value;};'}]},
  {name:'constant arrow import alias preserves the actual provider',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const double=value=>value*2;'},
   {path:'entry.js',source:'import {double as twice} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return twice(value);}'}]},
  {name:'duplicate private constant helpers retain independent owners',entry:'entry.js',aliases:true,arguments:[4],expected:11,files:[
   {path:'left.js',source:'/** @param {number} value @returns {number} */ const increment=value=>value+1;/** @param {number} value @returns {number} */ export const left=value=>increment(value);'},
   {path:'right.js',source:'/** @param {number} value @returns {number} */ const increment=value=>value+2;/** @param {number} value @returns {number} */ export const right=value=>increment(value);'},
   {path:'entry.js',source:'import {left} from "./left.js";import {right} from "./right.js";/** @param {number} value @returns {number} */ export function run(value){return left(value)+right(value);}'}]},
  {name:'function-expression aliases retain their private scopes',entry:'entry.js',aliases:true,arguments:[4],expected:4,formerDraftDiagnostic:'source named constant function-expression aliases need their separate inner binding scopes',files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const identity=function identity(value){return value;};'},
   {path:'entry.js',source:'import {identity as original} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return original(value);}'}]},
  {name:'anonymous function import aliases preserve parameters',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const double=function(value){return value*2;};'},
   {path:'entry.js',source:'import {double as twice} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return twice(value);}'}]},
  {name:'duplicate anonymous private helpers retain separate bindings',entry:'entry.js',aliases:true,arguments:[4],expected:11,files:[
   {path:'left.js',source:'/** @param {number} value @returns {number} */ const increment=function(value){return value+1;};/** @param {number} value @returns {number} */ export const left=function(value){return increment(value);};'},
   {path:'right.js',source:'/** @param {number} value @returns {number} */ const increment=function(value){return value+2;};/** @param {number} value @returns {number} */ export const right=function(value){return increment(value);};'},
   {path:'entry.js',source:'import {left} from "./left.js";import {right} from "./right.js";/** @param {number} value @returns {number} */ export function run(value){return left(value)+right(value);}'}]},
  {name:'async named source dependency preserves await and alias',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {Promise<number>} */ export async function double(value){return value*2;}'},
   {path:'entry.js',source:'import {double as twice} from "./provider.js";/** @param {number} value @returns {Promise<number>} */ export async function run(value){return await twice(value);}'}]},
  {name:'async anonymous source dependency preserves await and alias',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {Promise<number>} */ export const double=async function(value){return value*2;};'},
   {path:'entry.js',source:'import {double as twice} from "./provider.js";/** @param {number} value @returns {Promise<number>} */ export const run=async value=>await twice(value);'}]},
  {name:'different private recursive names normalize through actual lexical owners',entry:'entry.js',aliases:true,arguments:[4],expected:4,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const count=function step(value){if(value<=0)return 0;return step(value-1)+1;};'},
   {path:'entry.js',source:'import {count as original} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return original(value);}'}]},
  {name:'parameters retain their own owner during private-name normalization',entry:'entry.js',aliases:true,arguments:[4],expected:4,files:[
   {path:'provider.js',source:'/** @param {number} privateName @returns {number} */ export const identity=function privateName(privateName){return privateName;};'},
   {path:'entry.js',source:'import {identity as original} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return original(value);}'}]},
  {name:'private-name normalization preserves every existing capture guard',entry:'entry.js',aliases:true,arguments:[4],expected:4,error:'rename would capture',errorKind:'BindingRenameError',files:[
   {path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=function privateName(value){if(value<=0)return 0;return run(value-1)+1;};'}]},
  {name:'an arrow parameter named async retains its actual identifier role',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} async @returns {number} */ export const double=async=>async*2;'},
   {path:'entry.js',source:'import {double as twice} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return twice(value);}'}]},
  {name:'a named from export and async parameter retain exact import roles',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} async @returns {number} */ export function from(async){return async*2;}'},
   {path:'entry.js',source:'import {from as convert} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return convert(value);}'}]},
  {name:'constant functions retain parameter and var aliases during composition',entry:'entry.js',aliases:true,arguments:[4],expected:4,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const identity=function privateName(value){var value;return value;};'},
   {path:'entry.js',source:'import {identity as original} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return original(value);}'}]}
 ],
 boundary:'Unexecuted literal controls. Actual module bindings, private expression environments, parameter environments and all source bytes remain explicit. Function-only libraries, alias scopes, complete target/native policy behavior and semantic translations retain their independent controls. General constant data/effects, capture-conflicting private expression normalizations, reflection and full JavaScript modules remain required.'});
