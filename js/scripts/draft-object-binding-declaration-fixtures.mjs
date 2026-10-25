// Future source declarations and immutable predecessor inputs; execute nothing.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const versions={JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'};
const cases=[];
function add(name,body,value,prefix='',abort=null){const entry='objectBindingCase'+cases.length;cases.push({name,source:prefix+'export function compute(){'+body+'}',entry,value,abort,...(abort===null?{}:{errorClass:'RangeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
add('ordinary required fields retain source names','const {left,right}={left:3,right:4};return left+right;','7');
add('renamed required field retains its actual local binding','const {left:value}={left:3};return value;','3');
add('missing ordinary field selects a constant initializer','const {value=3}={};return value;','3');
add('present field bypasses a constant initializer','const {value=3}={value:7};return value;','7');
add('own undefined selects a constant initializer','const {value=3}={value:undefined};return value;','3');
add('null does not select the initializer','const {value=3}={value:null};return String(value);','null');
add('zero does not select the initializer','const {value=3}={value:0};return value;','0');
add('false does not select the initializer','const {value=true}={value:false};return value;','false');
add('empty text does not select the initializer','const {value="default"}={value:""};return value;','');
add('renamed missing field retains its original initializer','const {left:value=3}={};return value;','3');
add('earlier initialized lexical names are visible','const {left=3,right=left+1}={};return left+right;','7');
add('earlier ordinary declarators are visible','const first=3,{value=first+1}={},last=value+1;return last;','5');
add('earlier declarations outside the pattern are not invented BoundNames','const earlier=7;const {value=earlier}={};return value;','7');
add('required and initialized bindings retain their ordering','const {left,right=left+1}={left:3};return left+right;','7');
add('same property can supply distinct actual local names','const {value:left=3,value:right=left+1}={};return left+right;','7');
add('nested calls retain initializer commas','const {value=Math.max(3,4)}={};return value;','4');
add('opaque text retains object delimiters','const {value="} , )"}={};return value;','} , )');
add('opaque template retains its actual expression','const {value=`value${3}`}={};return value;','value3');
add('mutable object binding retains local writes','let {value}={value:3};value+=1;return value;','4');
add('mutable initialized object binding retains local writes','let {value=3}={};value+=1;return value;','4');
add('mutable writes do not modify the original property','const record={value:3};let {value}=record;value+=1;return String(value)+":"+String(record.value);','4:3');
add('mixed mutable declarators retain source order','let first=3,{value=first+1}={},last=value+1;value+=last;return value;','9');
add('a nested binding shadow keeps the outer source value','let value=7;{let {value=3}={};value+=1;}return value;','7');
add('a lexical initializer retains its enclosing local capture','const earlier=7;const {value=(()=>earlier)()}={};return value;','7');
add('mutable bindings are captured by their actual shared cell','let {value=3}={};const next=()=>{value+=1;return value;};return next()+next();','9');
add('right hand side is evaluated exactly once','let calls=0;const source=()=>{calls+=1;return {value:7};};const {value=3}=source();return String(calls)+":"+String(value);','1:7');
add('initializers retain source order before subsequent declarations','let events="";const mark=n=>{events+=String(n);return n;};const {second=mark(2),first=mark(1)}={};events+="x";return events+":"+String(first+second);','21x:3');
add('for initializer keeps object-bound mutable state','let total=0;for(let {index=0}={};index<3;index+=1)total+=index;return total;','3');
add('Unicode IdentifierName remains a real property','const {café:value=3}={café:7};return value;','7');
add('reserved property spelling retains its distinct local alias','const {default:value=3}={default:7};return value;','7');
add('an own prototype spelling retains its original value','const {toString:value=3}={toString:7};return value;','7');
add('present value keeps throwing initializer lazy','const {value=failObjectBindingPresent()}={value:7};return value;','7','/** @returns {number} */function failObjectBindingPresent(){throw new RangeError("object initializer");}');
add('missing value observes its exact initializer failure','const {value=failObjectBindingMissing()}={};return value;',null,'/** @returns {number} */function failObjectBindingMissing(){throw new RangeError("object initializer");}','object initializer');
const syntax=code=>'object binding syntax: flat actual property names and distinct lexical bindings need their complete source meanings (reason '+code+')';
const refusals=[
 {name:'self initialization retains its temporal binding obligation',source:'function f(){const {value=value}={};return value;}',kind:'unsupported',reason:'object binding initializer reading binding value: the current binding initializer profile requires initialized earlier lexical bindings'},
 {name:'later binding remains uninitialized',source:'function f(){let {first=second,second=3}={};return first;}',kind:'unsupported',reason:'object binding initializer reading binding second: the current binding initializer profile requires initialized earlier lexical bindings'},
 {name:'writes during initialization retain their environment interpretation',source:'function f(){let {first=3,second=(first=4)}={};return second;}',kind:'unsupported',reason:'object binding initializer assigning binding first: writes during binding initialization need their shared environment interpretation'},
 {name:'rest cannot be followed by a trailing comma',source:'function f(){const {...rest,}={value:3};return rest.value;}',kind:'unsupported',reason:'rest property: name every field'},
 {name:'mixed nested arrays retain their iterator meaning',source:'function f(){const {value:[nested]}={value:[3]};return nested;}',kind:'unsupported',reason:syntax(5)},
 {name:'computed properties retain their property key evaluation obligation',source:'function f(){const {["value"]:value}={value:3};return value;}',kind:'unsupported',reason:syntax(7)},
 {name:'duplicate actual BoundNames remain an early error',source:'function f(){let {left:value,right:value}={left:3,right:4};return value;}',kind:'syntax',reason:'duplicate lexical binding value'},
 {name:'inherited properties are not manufactured missing values',source:'function f(){const {toString:value=3}={};return value;}console.log(f());',kind:'unsupported',reason:'record formal inherited property: an absent own field cannot replace the actual prototype property toString'},
];
const revision='51140898',history=['js/tests/translation-javascript-mutable-array-bindings.test.js','rust/tests/unit/translation_javascript_array_bindings.rs','js/src/translation/javascript.js','rust/src/translation/javascript/statements.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/object-binding-declarations.lino',import.meta.url),{schemaVersion:1,model:'actual flat lexical object binding declarations 1',sourceLanguage:'JavaScript',cases,refusals,history,versions,correctedInputs:[{source:'function selected(values){let {first}={first:1};return 0;}',invocation:'console.log(selected(0));',expected:'0\n',predecessorReason:'let destructuring: declare each binding with its own let'},{source:'function f(){const {...rest}={value:3};return rest.value;}',invocation:'console.log(f());',expected:'3\n',predecessorReason:'rest property: name every field'},{source:'function f(){const {value:{nested}}={value:{nested:3}};return nested;}',invocation:'console.log(f());',expected:'3\n',predecessorReason:'object binding syntax: flat actual property names and distinct lexical bindings need their complete source meanings (reason 5)'}],source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),assertions:['sharedFormalAndLexicalPatternDefinitions','exactActualBoundNamesAndInitializerIntervals','rightHandSideBeforeOrderedLazyDefaults','lexicalEarlierBindingsWithoutFunctionBoundary','undefinedOnlyDefaultsAndPayloadPreservation','actualMutableCellsAndIndependentSourceProperties','retainedRequiredFieldAndConstructorProjectionPath','retainedFormerMutableObjectInputWithCorrectedValue','completeSourceDefinitionTwoCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Flat IdentifierName properties reuse the required-field path or represented-field default protocol; final lexical rest uses the separate complete own-field plan. Mixed array/computed/string-key patterns, complete inherited/getter/proxy protocols, initialization writes and broader TDZ/invocation proofs retain their full obligations. No new Core constructor or fabricated source Token is used.'});

const readerCases=[];
function reader(name,prefix,body,value){const entry='arrayReaderDomainCase'+readerCases.length;readerCases.push({name,source:prefix+'export function compute(){'+body+'}',entry,value,abort:null});}
reader('numeric result array cannot retain the string array owner','/** @param {string[]} input @returns {number[]} */function readerLengths(input){return [input.length];}','const names=["one"];const before=readerLengths(names);names.push("two");return before.join(",")+"|"+names.join(",");','1|one,two');
reader('readonly results retain separate observations before and after append','/** @param {string[]} input @returns {number[]} */function readerSnapshots(input){return [input.length];}','const names=["one"];const before=readerSnapshots(names);names.push("two");const after=readerSnapshots(names);return before.join(",")+"|"+after.join(",");','1|2');
reader('Boolean input and numeric output retain disjoint domains','/** @param {boolean[]} input @returns {number[]} */function readerBooleanLengths(input){return [input.length];}','const flags=[true];const before=readerBooleanLengths(flags);flags.push(false);return before.join(",")+"|"+String(flags.length);','1|2');
reader('text output cannot retain the numeric array owner','/** @param {number[]} input @returns {string[]} */function readerLabels(input){return ["size:"+String(input.length)];}','const values=[1];const label=readerLabels(values);values.push(2);return label.join("")+"|"+String(values.length);','size:1|2');
writeProjectData(new URL('../../parity/fixtures/owned-array-reader-domains.lino',import.meta.url),{schemaVersion:1,model:'actual typed disjoint primitive array readers 1',sourceLanguage:'JavaScript',cases:readerCases,refusals:[],versions,source:readerCases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),assertions:['completeActualReaderBodyAndDeclaredTypes','primitiveResultDomainsCannotRetainInputIdentity','originalAliasCaptureUnknownCallAndRebindingChecksRemain','existingIntegerRefinementsRemainOneDomain','sourceFreeDefinitionTwoCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Only inspected reader declarations with fully checked disjoint primitive array domains gain this result-shape admission. Original scalar-reader and every source occurrence/escape check remain. Same-domain, optional, compound, unknown and forward unavailable helper summaries remain separate obligations.'});
