// Preserve assertion history and author future controls without executing code.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const revision=execFileSync('git',['rev-parse','bf2ae2f3'],{encoding:'utf8'}).trim();
const sources=['js/tests/translation-javascript-record-parameters.test.js','rust/tests/unit/translation_javascript_record_parameters.rs'].map(path=>{
 const source=execFileSync('git',['show',revision+':'+path],{encoding:'utf8',maxBuffer:2*1024*1024});
 return {path,source,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};
});
const legacy=[{source:'const f=({a})=>a;',call:'f({a:7})',value:'7'},{source:'const f=function({a}){return a;};',call:'f({a:7})',value:'7'},{source:'const Namespace={f({a}){return a;}};',call:'Namespace.f({a:7})',value:'7'}];
const references=['https://tc39.es/ecma262/2025/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions','https://tc39.es/ecma262/2025/multipage/ecmascript-language-functions-and-classes.html#sec-arrow-function-definitions','https://tc39.es/ecma262/2025/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions'];
writeProjectData(new URL('../../parity/fixtures/record-callable-parameter-assertion-history.lino',import.meta.url),{schemaVersion:1,revision,sources,legacy,references,reason:'ECMAScript formal parameters admit BindingPattern in ordinary functions, arrows and object methods. These three old parser refusals described a previous implementation limit, not a language prohibition. Each exact original input remains with syntax and concrete-call coverage; the original named-only profile operation and other refusal inputs are preserved.'});
const cases=[];
function add(name,definition,call,value){const ordinal=cases.length,owner='callableRecordOwner'+ordinal,entry='callableRecordCase'+ordinal;cases.push({name,source:definition.replaceAll('OWNER',owner)+'export function compute(){return '+call.replaceAll('OWNER',owner)+';}',entry,value,abort:null});}
add('constant arrow expression','const OWNER=({a})=>a+1;','OWNER({a:7})','8');
add('constant arrow block','const OWNER=({a})=>{return a+1;};','OWNER({a:7})','8');
add('anonymous constant function expression','const OWNER=function({a}){return a+1;};','OWNER({a:7})','8');
add('matching named function expression','const OWNER=function OWNER({a}){return a+1;};','OWNER({a:7})','8');
add('namespace object method','const OWNER={read({a}){return a+1;}};','OWNER.read({a:7})','8');
add('nested namespace method','const OWNER={Nested:{read({a:value},scale){return value*scale;}}};','OWNER.Nested.read({a:7},3)','21');
add('constant local pattern write','const OWNER=({a})=>{a+=1;return a;};','OWNER({a:7})','8');
add('namespace local pattern write','const OWNER={read({a}){a+=1;return a;}};','OWNER.read({a:7})','8');
add('two source pattern arguments','const OWNER=({a:left},{a:right})=>left+right;','OWNER({a:7},{a:3})','10');
add('constant arrow has no directive prologue for expression body','const OWNER=({a})=>"use strict";','OWNER({a:7})','use strict');
const refusals=[
 {name:'async record constant needs its actual async parameter model',source:'const f=async({a})=>a;',kind:'unsupported',reason:'record callable source profile: ordinary non-simple callables require synchronous bodies without parameter documentation tags'},
 {name:'parameter documentation is not a logical pattern name',source:'/** @param {{a:number}} options */const f=({a})=>a;',kind:'unsupported',reason:'record callable source profile: ordinary non-simple callables require synchronous bodies without parameter documentation tags'},
 {name:'constant non-simple formals retain strict directive early errors',source:'const f=({a})=>{"use strict";return a;};',kind:'unsupported',reason:'record parameter strict directive: a non-simple formal parameter list cannot have a Use Strict Directive in its function body'},
 {name:'namespace non-simple formals retain strict directive early errors',source:'const Namespace={read({a}){"use strict";return a;}};',kind:'unsupported',reason:'record parameter strict directive: a non-simple formal parameter list cannot have a Use Strict Directive in its function body'},
];
writeProjectData(new URL('../../parity/fixtures/record-callable-parameters.lino',import.meta.url),{schemaVersion:1,model:'actual ordinary record callable parameters 1',sourceLanguage:'JavaScript',cases,refusals,legacy,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),references,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['actualConstantAndNamespaceContexts','completeOriginalPatternAndBody','logicalBindersAndIndependentNamedProfile','propertyAliasesOrderedArgumentsAndNestedNamespaces','bodyEntryBindingsBeforeLocalWrites','strictDirectiveAndAsyncDocumentationRefusals','retainedOriginalRefusalInputsWithCorrectedSyntaxAndValues','wholeDefinitionAndSourceGraphsBothCodecsFourEmissions','independentNativeAndOriginalSourceProducts'],
 boundary:'Unexecuted future controls. No source token or named declaration is fabricated. Absent-field/whole-parameter defaults, initialization writes, rest/nested patterns, receiver semantics, getters, prototype coercions and complete async/generator meanings remain separate obligations.',
});
