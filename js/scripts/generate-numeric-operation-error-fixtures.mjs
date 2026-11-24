// Declarations only; no original, translated or kernel behavior runs here.
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],mixed='Cannot mix BigInt and other types, use explicit conversions',unsigned='BigInts have no unsigned right shift, use >> instead';
function add(name,source,message,value=null,extra={}){
 const index=cases.length;
 cases.push({name,source,entry:'numericOperationError'+index,value,abort:message,
  ...(message===null?{}:{errorClass:'TypeError',messagePresent:true,messageUnits:Array.from(message,character=>character.charCodeAt(0))}),...extra});
}
for(const operation of ['+','-','*','/','%','&','|','^','<<','>>','>>>'])for(const [left,right] of [['1n','1'],['1','1n']])add('mixed '+left+operation+right,'export function compute(){return '+left+operation+right+';}',mixed);
add('unsigned BigInt shift','export function compute(){return 1n>>>1n;}',unsigned);
add('annotated string result retains nonreturning mixed error','/** @returns {string} */ export function compute(){return 1n+1;}',mixed);
add('annotated Boolean result retains nonreturning unsigned error','/** @returns {boolean} */ export function compute(){return 1n>>>1n;}',unsigned);
add('Boolean ToNumeric before mixed arithmetic','export function compute(){return 1n+true;}',mixed);
add('null ToNumeric before mixed arithmetic','export function compute(){return 1n*null;}',mixed);
add('undefined ToNumeric before mixed arithmetic','export function compute(){return 1n*undefined;}',mixed);
add('String ToNumeric before mixed multiplication','export function compute(){return "1"*2n;}',mixed);
add('String addition stays concatenation','export function compute(){return "1"+2n;}',null,'12');
add('right String addition stays concatenation','export function compute(){return 2n+"1";}',null,'21');
add('compound mixed bitwise assignment','export function compute(){let value=7n;value&=1;return value;}',mixed);
add('right operand exception precedes mixed numeric error','/** @returns {number} */ function failNumericRight(){throw new RangeError("right operand");}\nexport function compute(){return 1n+failNumericRight();}','right operand',null,{errorClass:'RangeError'});
add('left operand exception precedes unsigned shift error','/** @returns {bigint} */ function failNumericLeft(){throw new SyntaxError("left operand");}\nexport function compute(){return failNumericLeft()>>>1n;}','left operand',null,{errorClass:'SyntaxError'});
const previous=readProjectData(new URL('../../parity/fixtures/big-integer-bitwise.lino',import.meta.url));
for(const [index,source] of previous.refusals.entries())add('retained former bitwise refusal '+index,source,index===0?unsigned:mixed,null,{previousExpectation:'checkProgram throws',correction:'ECMAScript numeric operations throw at runtime after evaluating operands; supported primitive domains must preserve that behavior'});
writeProjectData(new URL('../../parity/fixtures/numeric-operation-errors.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript mixed numeric and unsigned BigInt source errors 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:previous.versions,
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-applystringornumericbinaryoperator ; https://tc39.es/ecma262/2025/multipage/ecmascript-data-types-and-values.html#sec-numeric-types-bigint-unsignedRightShift ; https://github.com/nodejs/node/blob/main/deps/v8/src/common/message-template.h',
 boundary:'Declared independent source errors, operand order, nonreturning contextual result types and retained former refusal inputs, complete definition/program codecs and all targets. No execution observations. Object/Symbol coercion, mixed numeric comparison, optional dynamic numeric domains and remaining operations stay requirements.',
});
