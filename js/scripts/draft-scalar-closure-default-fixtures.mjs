// Future products and complete predecessor definitions; execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,prefix='',abort=null){const ordinal=cases.length;cases.push({name,entry:'scalarClosureDefaultCase'+ordinal,source:(prefix+'export function compute(){'+body+'}').replaceAll('OWNER','scalarClosureDefaultOwner'+ordinal),value,abort,...(abort===null?{}:{errorClass:'RangeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
add('a local scalar arrow default admits an omitted value','const read=(value=3)=>value;return read();','3');
add('a local scalar arrow retains a supplied value','const read=(value=3)=>value;return read(7);','7');
add('explicit undefined selects the original scalar default','const read=(value=3)=>value;return read(undefined);','3');
add('an explicit zero does not select the default','const read=(value=3)=>value;return read(0);','0');
add('an explicit false does not select a Boolean default','const read=(value=true)=>value;return read(false);','false');
add('empty text remains present for a String default','const read=(value="fallback")=>value;return read("");','');
add('null is retained separately from undefined','const read=(value=3)=>String(value);return read(null);','null');
add('a function expression retains its body entry default','const read=function(value=3){return value;};return read();','3');
add('a record method retains its scalar default','const object={read(value=3){return value;}};return object.read();','3');
add('a local default captures its actual enclosing value','const offset=7;const read=(value=offset)=>value;return read();','7');
add('later defaults read earlier initialized scalar bindings','const read=(first=3,second=first+1)=>first+second;return read();','7');
add('a later scalar default reads an earlier supplied value','const read=(first,second=first+1)=>first+second;return read(3);','7');
add('a required later value keeps its actual argument slot','const read=(first=3,second)=>first+second;return read(undefined,4);','7');
add('a returned closure keeps an initialized default capture','const make=(first=3)=>()=>first;return make()();','3');
add('a returned defaulted closure remains callable','return OWNER()();','3','function OWNER(){return (value=3)=>value;}');
add('a present map value gets an explicit optional wrapper','return [3,4].map((value=1)=>value+1).join("|");','4|5');
add('an actual undefined map value selects its default','return [undefined,4].map((value=3)=>value+1).join("|");','4|5');
add('an ordinal supplied by map remains present','return [7].map((value=3,index=value+1)=>value+index).join("|");','7');
add('a defaulted filter keeps its Boolean result','return [true,false].filter((value=true)=>value).length;','1');
add('a defaulted reduction retains its accumulator and value order','return [undefined,4].reduce((total=0,value=3)=>total+value,0);','7');
add('Array.from uses the same scalar default bridge','return Array.from([undefined,4],(value=3)=>value+1).join("|");','4|5');
add('an ordinary variable default may hold an array','const read=(values=[3,4])=>values.join("|");return read();','3|4');
add('an ordinary variable default may hold a record','const read=(config={answer:3})=>config.answer;return read();','3');
add('a default array is constructed independently per invocation','const read=(values=[3])=>{values.push(4);return values.join("|");};return read()+":"+read();','3|4:3|4');
add('provided values keep a throwing scalar default lazy','const read=(value=OWNER())=>value;return read(7);','7','/** @returns {number} */function OWNER(){throw new RangeError("scalar closure default");}');
add('an omitted value observes its original default failure','const read=(value=OWNER())=>value;return read();',null,'/** @returns {number} */function OWNER(){throw new RangeError("scalar closure default");}','scalar closure default');
const traces=[
 {name:'a computed scalar-default callee precedes arguments and initialization',source:'function initialize(){console.log("default");return 3;}function factory(){console.log("callee");return (value=initialize())=>{console.log("body");return value;};}function argument(){console.log("argument");return undefined;}console.log(factory()(argument()));',lines:['callee','argument','default','body','3'],error:null},
 {name:'two scalar defaults initialize in their source order',source:'function first(){console.log("first");return 3;}function second(){console.log("second");return 4;}const read=(left=first(),right=second())=>left+right;console.log(read());',lines:['first','second','7'],error:null},
 {name:'ignored arguments precede scalar closure defaults',source:'function initialize(){console.log("default");return 3;}function extra(){console.log("extra");return 4;}const read=(value=initialize())=>value;console.log(read(undefined,extra()));',lines:['extra','default','3'],error:null}
];
const revision='8b173e65',history=['js/src/translation/whole-parameter-defaults.js','js/src/translation/whole-parameter-default-rules.js','js/src/translation/array-formal-parameters.js','js/src/translation/callable-source-rules.js','rust/src/translation/javascript/whole_parameter_defaults.rs','rust/src/translation/javascript/array_formal_parameters.rs','rust/src/translation/javascript/callables.rs','js/tests/translation-self-ordinary-module-ownership.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
const refusals=[{name:'a scalar-default closure retains strict-directive early error',source:'function f(){const read=(value=3)=>{"use strict";return value;};return read();}',kind:'unsupported',reason:'record parameter strict directive: a non-simple formal parameter list cannot have a Use Strict Directive in its function body'}];
writeProjectData(new URL('../../parity/fixtures/scalar-closure-defaults.lino',import.meta.url),{schemaVersion:1,model:'ordinary closure defaults through ordered existing parameter entries 1',sourceLanguage:'JavaScript',cases,traces,history,refusals,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['ordinaryScalarClosureContextAndActualFormalPlan','existingBodyEntryOptionalMarkersAndUndefinedOnlyInitialization','earlierParameterAndOuterCaptureEnvironments','namedConstantAndPatternRoutesRetained','freshArrayDefaultsAndLazyFailures','callbackOrdinalAndAccumulatorRoles','actualCalleeArgumentDefaultAndBodyOrder','completePredecessorDefinitionsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted controls. Reuses existing complete whole-formal parsing, ordered binding entries and optional callable rules. Ordinary named and constant scalar defaults retain their old routes. Full self/later parameter environments, arbitrary present-type unions, implicit this/arguments, async/generators and all preceding requirements remain obligations.'});
