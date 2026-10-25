// Declare future source observations and refusal inputs; never execute them.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,prefix,body,value){const owner='arrayFormalOwner'+cases.length,entry='arrayFormalCase'+cases.length;cases.push({name,source:prefix.replaceAll('OWNER',owner)+'export function compute(){'+body.replaceAll('OWNER',owner)+'}',entry,value,abort:null});}
const sum='function OWNER([first,second]){return (first??0)+(second??0);}';
add('required dense array positions',sum,'return OWNER([3,4]);','7');
add('missing second position retains undefined',sum,'return OWNER([3]);','3');
add('empty input retains both undefined positions',sum,'return OWNER([]);','0');
add('elisions advance actual iterator positions','function OWNER([,second]){return second??0;}','return OWNER([3,4]);','4');
const rest='function OWNER([first,...rest]){return String(first??0)+":"+rest.join("|");}';
add('final rest reads a fresh suffix',rest,'return OWNER([3,4,5]);','3:4|5');
add('empty final rest is a real empty array',rest,'return OWNER([]);','0:');
add('constant expression arrow preserves ordinary arguments','const OWNER=([first,second],scale)=>(first??0)*scale+(second??0);','return OWNER([3,4],2);','10');
add('constant function expression','const OWNER=function([first]){return first??0;};','return OWNER([3]);','3');
add('namespace object method','const OWNER={read([first]){return first??0;}};','return OWNER.read([3]);','3');
add('dense nested array callback','', 'return [[1,2],[3]].map(([first,second])=>(first??0)+(second??0)).join("|");','3|3');
add('map entry tuples preserve separate source positions','', 'return Array.from(new Map([["one",3],["two",4]])).map(([key,value])=>key+":"+String(value)).join("|");','one:3|two:4');
add('proved immutable heterogeneous product remains positional','function OWNER([count,flag]){return flag?count:0;}','const row=Object.freeze([3,true]);return OWNER(row);','3');
add('local bound writes leave original array intact','function OWNER([first]){first=(first??0)+1;return first??0;}','const row=[3];const result=OWNER(row);return String(result)+":"+String(row[0]??0);','4:3');
add('Unicode string iteration uses code points','function OWNER([first,...rest]){return (first??"")+":"+rest.join("");}','return OWNER("🙂z");','🙂:z');
add('isolated UTF16 units remain in string iterator elements','function OWNER([first]){return (first??"").charCodeAt(0);}','return OWNER("\ud800Z");','55296');
add('each array rest retains a separate outer array','function OWNER([...rest]){return rest;}','const input=[3,4];const left=OWNER(input);const right=OWNER(input);left.push(9);return left.join("|")+":"+right.join("|")+":"+input.join("|");','3|4|9:3|4:3|4');
add('ordinary function expression callback','', 'return [[1],[2]].map(function([first]){return first??0;}).join("|");','1|2');
add('receiver-independent object method closure','', 'const value={read([first]){return first??0;}};return value.read([7]);','7');
add('empty pattern retains actual iterator selection','function OWNER([]){return 1;}','return OWNER([3,4]);','1');
add('arrow expression is not a directive prologue','const OWNER=([first])=>"use strict";','return OWNER([3]);','use strict');
const reason=code=>'array formal parameter syntax: flat names, elisions and final named rest use distinct actual BoundNames; defaults and nested/mixed object patterns need separate interpretations (reason '+code+')';
const refusals=[
 {name:'defaults need their actual parameter initialization environment',source:'function f([first=3]){return first;}console.log(f([]));',kind:'unsupported',reason:reason(3)},
 {name:'nested patterns retain their independent iterator obligation',source:'function f([[first]]){return first;}console.log(f([[3]]));',kind:'unsupported',reason:reason(2)},
 {name:'mixed object and array formals retain both actual source roles',source:'function f([first],{second}){return first??second;}console.log(f([3],{second:4}));',kind:'unsupported',reason:reason(2)},
 {name:'rest trailing comma is not silently normalized',source:'function f([...rest,]){return rest.length;}',kind:'unsupported',reason:reason(5)},
 {name:'repeated actual BoundNames refuse',source:'function f([first,first]){return first;}',kind:'unsupported',reason:reason(4)},
 {name:'async formals require a separate async interpretation',source:'async function f([first]){return first;}',kind:'unsupported',reason:'array formal parameter source profile: ordinary array patterns require synchronous callables without parameter documentation tags'},
 {name:'a non-simple list cannot acquire a strict directive',source:'function f([first]){"use strict";return first;}',kind:'unsupported',reason:'record parameter strict directive: a non-simple formal parameter list cannot have a Use Strict Directive in its function body'},
];
writeProjectData(new URL('../../parity/fixtures/array-formal-parameters.lino',import.meta.url),{schemaVersion:1,model:'actual ordinary array formal parameters 1',sourceLanguage:'JavaScript',cases,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),refusals,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['actualOrdinaryCallableContexts','completeFormalTokenAndBoundNamePlans','orderedLogicalArgumentsAndBodyEntryBindings','undefinedArrayPositionsWithoutInventedBounds','UnicodePointIterationAndIsolatedUnitPreservation','localBindingWritesAndFreshRestArrays','actualTuplePositionsAndExistingOwnershipProofs','retainedStrictDirectiveAndUnsupportedProfileRefusals','completeDefinitionProgramBothCodecsFourEmissions','independentOriginalJavaScriptAndNativeProducts'],
 boundary:'Unexecuted future controls. Finite represented arrays/strings and immutable collection iteration reuse established operations. Flat tuple reads remain fixed/in-range under the existing value model. Missing tuple positions, heterogeneous tuple rest, custom/live iterators, descriptors, defaults and nested/mixed object patterns remain separate source meanings. No new Core constructor or source-facet exemption is introduced.',
});
