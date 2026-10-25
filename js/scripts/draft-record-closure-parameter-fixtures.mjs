// Actual source/expectation declarations only. No callback or fixture executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value){const entry='recordClosureCase'+cases.length;cases.push({name,source:'export function compute(){'+body+'}',entry,value,abort:null});}
add('arrow expression record binding','return [{value:3},{value:4}].map(({value})=>value+1).join("|");','4|5');
add('arrow statement record binding','return [{value:3},{value:4}].map(({value})=>{return value+1;}).join("|");','4|5');
add('function expression record binding','return [{value:3},{value:4}].map(function({value}){return value+1;}).join("|");','4|5');
add('renamed actual binding','return [{value:3},{value:4}].map(({value:amount})=>amount*2).join("|");','6|8');
add('logical pattern and ordinary positional arguments','return [{value:3},{value:4}].map(({value},index)=>value+index).join("|");','3|5');
add('repeated property preserves distinct bound names','return [{value:3}].map(({value:left,value:right})=>left+right).join("|");','6');
add('pattern shadows an outer captured spelling','const value=10;const result=[{value:2}].map(({value})=>value).join("|");return result+":"+value;','2:10');
add('a changed local binding leaves original record intact','const rows=[{value:3},{value:4}];const result=rows.map(({value})=>{value+=1;return value;}).join("|");return result+":"+(rows[0]??{value:0}).value;','4|5:3');
add('nonblock arrow expression is not a strict directive','return [{value:3}].map(({value})=>"use strict").join("|");','use strict');
add('actual outer capture is retained','const offset=7;return [{value:3}].map(({value})=>value+offset).join("|");','10');
add('retained former default refusal now reads its original present value','return [{value:3}].map(({value=1})=>value).join("|");','3');
const patternReason=code=>'record parameter binding: a nonempty flat required object pattern needs distinct lexical names; defaults, arrays, nested/rest/computed patterns are separate obligations (reason '+code+')';
const refusals=[
 {name:'nested record pattern remains outside the flat profile',source:'export function compute(){return [{value:3}].map(({value:{nested}})=>nested).join("|");}',kind:'unsupported',reason:patternReason(5)},
 {name:'record rest pattern requires complete own-property operations',source:'export function compute(){return [{value:3}].map(({...rest})=>rest.value).join("|");}',kind:'unsupported',reason:patternReason(6)},
 {name:'a non-simple parameter list cannot acquire a strict directive',source:'export function compute(){return [{value:3}].map(({value})=>{"use strict";return value;}).join("|");}',kind:'unsupported',reason:'record parameter strict directive: a non-simple formal parameter list cannot have a Use Strict Directive in its function body'},
];
writeProjectData(new URL('../../parity/fixtures/record-closure-parameters.lino',import.meta.url),{
 schemaVersion:1,model:'actual flat required record closure parameter context 1',sourceLanguage:'JavaScript',cases,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),refusals,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['actualArrowAndFunctionContexts','completeOriginalPatternAndBody','logicalBinderHygieneWithoutFakeSourceTokens','propertyAliasesAndActualArgumentSlots','leftToRightEntryReads','localWritesPreserveOriginalRecord','shadowingAndActualCaptures','strictDirectiveAndUnsupportedPatternRefusals','wholeDefinitionAndSourceGraphsBothCodecsFourEmissions','independentNativeAndSourceProducts'],
 boundary:'Unexecuted future controls. Named declarations keep their original separate profile. Absent-field/whole-parameter defaults, initialization writes, nested/rest/computed patterns, generators, async closures and receiver-dependent object methods still require their full source meanings.',
});
