// Future source products and exact predecessors, without behavioral execution.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,prefix='',abort=null){const ordinal=cases.length;cases.push({name,entry:'bindingInitializationCase'+ordinal,source:(prefix+'export function compute(){'+body+'}').replaceAll('OWNER','bindingInitializationOwner'+ordinal),value,abort,...(abort===null?{}:{errorClass:'RangeError',messagePresent:true,messageUnits:Array.from({length:abort.length},(_,index)=>abort.charCodeAt(index))})});}
add('an empty array formal initializes its omitted default','const read=([]=[3])=>7;return read();','7');
add('an empty array formal initializes explicit undefined','const read=([]=[3])=>7;return read(undefined);','7');
add('an empty array formal retains a supplied iterable','const read=([]=[3])=>7;return read([4]);','7');
add('a named empty array default retains its argument slot','return OWNER();','7','function OWNER([]=[3]){return 7;}');
add('a stored named empty default keeps body initialization','const read=OWNER;return read();','7','function OWNER([]=[3]){return 7;}');
add('a constant empty pattern default retains its body entry','return OWNER();','7','const OWNER=([]=[3])=>7;');
add('an empty pattern fallback reads the earlier scalar','const read=(first,[]=[first])=>first;return read(7);','7');
add('an empty pattern fallback reads an earlier default','const read=(first=7,[]=[first])=>first;return read();','7');
add('a later default reads the actual earlier scalar across an empty pattern','const read=(first,[]=[first],last=first+1)=>last;return read(6);','7');
add('a later array pattern keeps its actual binder','const read=([]=[3],[last]=[7])=>last;return read();','7');
add('a later record pattern retains its original property','const read=([]=[3],{last}={last:7})=>last;return read();','7');
add('a whole array without descendant names keeps every initialization','const read=([[]=[]]=[[]])=>7;return read();','7');
add('a missing nested array selects its nameless initializer','const [[]=[]]=[undefined];return 7;','7');
add('a present nested array leaves its initializer lazy','const [[]=OWNER()]=[[]];return 7;','7','/** @returns {number[]} */function OWNER(){throw new RangeError("empty binding default");}');
add('a missing nested array observes its initializer failure','const [[]=OWNER()]=[undefined];return 7;',null,'/** @returns {number[]} */function OWNER(){throw new RangeError("empty binding default");}','empty binding default');
add('a nested empty array reads earlier array binders','const [first,[]=[first]]=[7,undefined];return first;','7');
add('a record owned empty array reads earlier record binders','const {first,items:[]=[first]}={first:7,items:undefined};return first;','7');
add('a nested nameless initializer keeps an outer capture','const first=7;const [[]=[first]]=[undefined];return first;','7');
add('an empty iterable default may be a String','const read=([]="text")=>7;return read();','7');
add('two empty pattern parameters retain distinct actual boundaries','const read=([]=[3],[]=[4],last=7)=>last;return read();','7');
const traces=[
 {name:'nameless defaults execute before later parameter initialization',source:'function first(){console.log("first");return [];}function last(){console.log("last");return 7;}const read=([]=first(),value=last())=>{console.log("body");return value;};console.log(read());',lines:['first','last','body','7'],error:null},
 {name:'a present iterable keeps a nameless default lazy',source:'function initialize(){console.log("default");return [];}const read=([]=initialize())=>7;console.log(read([3]));',lines:['7'],error:null},
 {name:'all arguments precede empty pattern initialization',source:'function initialize(){console.log("default");return [];}function extra(){console.log("extra");return 4;}const read=([]=initialize())=>7;console.log(read(undefined,extra()));',lines:['extra','default','7'],error:null},
 {name:'a selected nested empty default executes once',source:'function initialize(){console.log("default");return [];}const [[] = initialize()]=[undefined];console.log(7);',lines:['default','7'],error:null}
];
const refusals=[{name:'a later parameter remains unavailable at a nameless initializer',source:'function read([]=[later],later=7){return later;}',kind:'unsupported',reason:'whole parameter default reading parameter later: the current default profile requires a closed parameter initialization environment'}];
const revision='fd3d747f',history=['js/src/translation/formal-initializers.js','js/src/translation/whole-parameter-defaults.js','js/src/translation/recursive-object-bindings.js','js/src/translation/array-formal-parameter-rules.js','rust/src/translation/javascript/formal_initializers.rs','rust/src/translation/javascript/whole_parameter_defaults.rs','rust/src/translation/javascript/recursive_object_bindings.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/binding-initialization.lino',import.meta.url),{schemaVersion:1,model:'actual source initialization boundaries for nameless binding patterns 1',sourceLanguage:'JavaScript',cases,traces,history,refusals,source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['actualPatternBoundaryWithoutInventedBoundNames','earlierBindingsAvailableLaterBindingsUninitialized','wholeAndNestedNamelessArrayDefaultsRetainSlots','existingIterationAndLazyInitializerEffects','completePredecessorSourcesBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Ordinary array iteration uses the existing represented immutable iterable profile; mutable/custom iterator consumption and closing remain obligations. Empty object destructibility, general parameter environments, unions, complete language and grammar/concept/knowledge/inference requirements remain unfinished.'});
