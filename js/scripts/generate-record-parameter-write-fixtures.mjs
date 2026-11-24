// Source/expected declarations only. No fixture, caller or oracle is executed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['write destructured local','function assignRecordLocal({a}){a=2;return a;} export function compute(){return assignRecordLocal({a:1});}','2'],
 ['compound destructured local','function addRecordLocal({a}){a+=3;return a;} export function compute(){return addRecordLocal({a:2});}','5'],
 ['update destructured local','function incrementRecordLocal({a}){a++;return a;} export function compute(){return incrementRecordLocal({a:2});}','3'],
 ['ordinary parameter beside pattern remains mutable','function assignOrdinaryLocal({a},other){other=2;return a+other;} export function compute(){return assignOrdinaryLocal({a:1},0);}','3'],
 ['closure captures written binding','function captureRecordLocal({a}){const change=()=>{a=2;return a;};return change();} export function compute(){return captureRecordLocal({a:1});}','2'],
 ['returned closures retain per-invocation cells','function createRecordCounter({a}){return ()=>{a+=1;return a;};} export function compute(){const first=createRecordCounter({a:1});const second=createRecordCounter({a:7});first();return first()+second();}','11'],
 ['aliased property binds its local name','function assignAliasedLocal({field:a}){a=9;return a;} export function compute(){return assignAliasedLocal({field:1});}','9'],
 ['binding assignment does not write caller property','function replaceRecordLocal({a}){a=2;return a;} export function compute(){const input={a:1};const value=replaceRecordLocal(input);return value*10+input.a;}','21'],
 ['nested lexical shadow retains its own assignment','function shadowRecordLocal({a}){if(a>0){let a=7;a+=1;return a;}return a;} export function compute(){return shadowRecordLocal({a:1})+shadowRecordLocal({a:-2});}','6'],
];
const cases=inputs.map(([name,source,value],index)=>({name,entry:'recordParameterWrite'+index,source,value,abort:null}));
const correctedAssertions=[
 {source:'function f({a}){a=2;return a;}',call:'f({a:1})',value:'2'},
 {source:'function f({a}){const change=()=>{a=2;return a;};return change();}',call:'f({a:1})',value:'2'},
 {source:'function f({a},other){other=2;return a+other;}',call:'f({a:1},0)',value:'3'},
].map(row=>({...row,previousAssertion:'checkProgram(parseJavaScript(source)) throws /record parameter|mutable|capture|var declaration/'}));
writeProjectData(new URL('../../parity/fixtures/record-parameter-writes.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript record parameter mutable local initialization 1',sourceLanguage:'JavaScript',cases,correctedAssertions,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 predecessorRevision:'b1dac726',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-functiondeclarationinstantiation ; https://tc39.es/ecma262/2025/multipage/ecmascript-language-statements-and-declarations.html#sec-runtime-semantics-bindinginitialization',
 boundary:'Future original Node/emitted JavaScript/native values, per-call captured cells, original corrected inputs/assertions, both program/definition codecs and four emissions. No observations. Parameter defaults, nested/rest patterns, var environments, getters/proxies, general closures/formal cell semantics and every earlier requirement remain.',
});
