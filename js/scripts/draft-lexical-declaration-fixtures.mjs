// Future controls only: write source text and expectations without parsing it.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value){const entry='lexicalDeclarationCase'+cases.length;cases.push({name,source:'export function compute(){'+body+'}',entry,value,abort:null});}
add('nested const shadows an outer const','const value=3;{const value=4;}return value;','3');
add('nested let shadows an outer const','const value=3;{let value=4;value+=1;}return value;','3');
add('sibling blocks have separate names','let total=0;{const value=3;total+=value;}{const value=4;total+=value;}return total;','7');
add('loop head may shadow an existing binding','const value=7;for(let value=0;value<2;value+=1){}return value;','7');
add('loop body may shadow its head','let total=0;for(let value=0;value<2;value+=1){const value=4;total+=value;}return total;','8');
add('record BoundNames shadow only within their block','const value=3;{const {left:value}={left:4};}return value;','3');
add('array BoundNames shadow only within their block','const value=3;{const [value]=[4];}return value;','3');
const refusals=[
 ['separate const declarations','function f(){const value=1;const value=2;return value;}','value'],
 ['separate let declarations','function f(){let value=1;let value=2;return value;}','value'],
 ['let followed by const','function f(){let value=1;const value=2;return value;}','value'],
 ['const followed by let','function f(){const value=1;let value=2;return value;}','value'],
 ['body binding conflicts with a parameter','function f(value){const value=2;return value;}','value'],
 ['record BoundName conflicts with a parameter','function f(value){const {left:value}={left:2};return value;}','value'],
 ['array BoundName conflicts with a parameter','function f(value){const [value]=[2];return value;}','value'],
 ['separate record BoundNames','function f(){const {left:value}={left:1};const {right:value}={right:2};return value;}','value'],
 ['decoded escaped BoundNames compare identically','function f(){const value=1;const v\\u0061lue=2;return value;}','value'],
 ['constant callable binding duplicates','const f=()=>1;const f=()=>2;','f'],
 ['function declaration conflicts with a const','function f(){return 1;}const f=2;','f'],
].map(([name,source,binding])=>({name,source,binding,kind:'syntax',reason:'duplicate lexical binding '+binding}));
writeProjectData(new URL('../../parity/fixtures/lexical-declarations.lino',import.meta.url),{schemaVersion:1,model:'actual lexical declaration early errors 1',sourceLanguage:'JavaScript',cases,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),refusals,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['actualParsedBoundNamesAndOriginalSpans','separateStatementLexicalCollisions','immediateCallableParameterConflict','nestedSiblingAndLoopShadowing','decodedIdentifierEquality','completeSharedEarlyErrorDefinition','completeCodecsFourEmissionsAndNativeProducts'],
 boundary:'Unexecuted source controls. Let/const and their actual destructuring BoundNames, constant callables and top-level function/lexical conflicts use parsed declarations. Script/Module-dependent duplicate ordinary functions, general var/import/class declarations, Annex B and scalar-loop shortcut admission remain separate obligations. Declaration lookahead counts are not the new conflict oracle.'});
