// Source/expectation declarations only; no executable product runs.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['own trim callable','export function compute(){const item={trim(value){return value+7;}};return item.trim(3);}','10'],
 ['own includes returns Number','export function compute(){const item={includes(value){return value.length+8;}};return item.includes("abc");}','11'],
 ['own startsWith has no arguments','export function compute(){const item={startsWith(){return 12;}};return item.startsWith();}','12'],
 ['own endsWith has independent arguments','export function compute(){const item={endsWith(left,right){return left+right;}};return item.endsWith(6,7);}','13'],
 ['own toString callable with argument','export function compute(){const item={toString(value){return value+10;}};return item.toString(4);}','14'],
 ['own lower-case callable','export function compute(){const item={toLowerCase(value){return value*3;}};return item.toLowerCase(5);}','15'],
 ['own upper-case callable','export function compute(){const item={toUpperCase(){return 16;}};return item.toUpperCase();}','16'],
 ['own leading trim callable','export function compute(){const item={trimStart(left,right){return left*10+right;}};return item.trimStart(1,7);}','17'],
 ['own trailing trim callable','export function compute(){const item={trimEnd(value){return value+9;}};return item.trimEnd(9);}','18'],
 ['generic record method infers Number argument','export function compute(){const item={read(value){return value+12;}};return item.read(7);}','19'],
 ['annotated call receiver keeps own method','function readOwnTrim(item){return item.trim(8);} export function compute(){return readOwnTrim({trim(value){return value+12;}});}','20'],
 ['function-valued field keeps own meaning','export function compute(){const item={includes:value=>value+13};return item.includes(8);}','21'],
 ['string trim remains builtin','export function compute(){return "  abc  ".trim().length+19;}','22'],
 ['string startsWith remains builtin','export function compute(){return "abc".startsWith("ab")?23:0;}','23'],
 ['string includes remains builtin','export function compute(){return "abc".includes("b")?24:0;}','24'],
 ['string case map remains builtin','export function compute(){return "ABC".toLowerCase()==="abc"?25:0;}','25'],
 ['primitive toString remains builtin','export function compute(){return (26).toString()==="26"?26:0;}','26'],
 ['array includes retains actual owner','export function compute(){return [27,28].includes(27)?27:0;}','27'],
 ['receiver failure precedes argument','/** @returns {{trim: (number)=>number}} */ function failedPropertyReceiver(){throw new Error("receiver first");} function failedPropertyArgument(){throw new Error("argument later");return 0;} export function compute(){return failedPropertyReceiver().trim(failedPropertyArgument());}',null,'receiver first'],
 ['call argument order','function failedFirstPropertyArgument(){throw new Error("first argument");return 0;} function failedSecondPropertyArgument(){throw new Error("second argument");return 0;} export function compute(){const item={endsWith(left,right){return left+right;}};return item.endsWith(failedFirstPropertyArgument(),failedSecondPropertyArgument());}',null,'first argument'],
];
const cases=inputs.map(([name,source,value,abort],index)=>({name,module:'',entry:'compute'+index,source:source.replace('export function compute','\nexport function compute'),value,abort:abort??null}));
const refusals=[
 {name:'actual non-callable property',source:'export function compute(){const item={trim:3};return item.trim();}',kind:'type',reason:'record has no method trim'},
 {name:'own method actual arity',source:'export function compute(){const item={trim(value){return value;}};return item.trim();}',kind:'type',reason:'.trim() expects 1 arguments'},
];
writeProjectData(new URL('../../parity/fixtures/javascript-property-calls.lino',import.meta.url),{
 schemaVersion:1,model:'actual JavaScript property call owner 1',sourceLanguage:'JavaScript',cases,refusals,
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript modules',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-function-calls ; resolve the actual property value before invoking it; its spelling does not establish builtin ownership.',
 boundary:'Future original Node and emitted JavaScript values/aborts, native Rust execution, exact paired program/definition graphs, text/binary codecs and all four emissions. No observations. Dynamic this/prototypes/accessors, class construction/inheritance, array concat ownership, full coercion and error objects retain their remaining obligations.',
});
