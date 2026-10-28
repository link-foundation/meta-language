// Literal original source, inputs and independent expected values only.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),number=value=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return {$:'number',bits:view.getBigUint64(0).toString(16).padStart(16,'0')};};
const rows=[
 {name:'actual arithmetic declaration',source:'/** @param {number} value @returns {number} */ export function addOne(value){return value+1;}',entry:'addOne',functions:['addOne'],arguments:[number(40)],expected:number(41)},
 {name:'actual conditional source blocks',source:'/** @param {boolean} flag @param {number} value @returns {number} */ export function choose(flag,value){return flag?value*2:value/2;}',entry:'choose',functions:['choose'],arguments:[{$:'boolean',value:false},number(8)],expected:number(4)},
 {name:'actual nested global calls',source:'/** @param {number} value @returns {number} */ function addOne(value){return value+1;} /** @param {number} value @returns {number} */ export function twice(value){return addOne(addOne(value));}',entry:'twice',functions:['addOne','twice'],arguments:[number(40)],expected:number(42)},
 {name:'actual local binder ownership',source:'/** @param {number} value @returns {number} */ export function local(value){const doubled=value*2;return doubled+value;}',entry:'local',functions:['local'],arguments:[number(4)],expected:number(12)},
 {name:'actual text preserves isolated code units',source:'/** @param {string} value @returns {string} */ export function text(value){return value;}',entry:'text',functions:['text'],arguments:[{$:'text',units:[55296,0,56320]}],expected:{$:'text',units:[55296,0,56320]}},
 {name:'actual arbitrary integer identity',source:'/** @param {bigint} value @returns {bigint} */ export function integer(value){return value;}',entry:'integer',functions:['integer'],arguments:[{$:'integer',value:'184467440737095516160000000000001'}],expected:{$:'integer',value:'184467440737095516160000000000001'}},
 {name:'actual negative zero literal',source:'/** @returns {number} */ export function negativeZero(){return -0;}',entry:'negativeZero',functions:['negativeZero'],arguments:[],expected:number(-0)},
 {name:'actual short circuit boolean source',source:'/** @param {boolean} left @param {boolean} right @returns {boolean} */ export function conjunction(left,right){return left&&right;}',entry:'conjunction',functions:['conjunction'],arguments:[{$:'boolean',value:false},{$:'boolean',value:true}],expected:{$:'boolean',value:false}},
];
const equalitySource='/** @param {number} left @param {number} right @returns {boolean} */ export function equal(left,right){return left===right;}';
rows.push({name:'strict Number equality keeps signed zeros equal',source:equalitySource,entry:'equal',functions:['equal'],arguments:[number(-0),number(0)],expected:{$:'boolean',value:true}},
 {name:'strict Number equality never equates NaN with itself',source:equalitySource,entry:'equal',functions:['equal'],arguments:[number(NaN),number(NaN)],expected:{$:'boolean',value:false}},
 {name:'strict absence comparison keeps exact undefined',source:'/** @param {number | undefined} value @returns {boolean} */ export function absent(value){return value===undefined;}',entry:'absent',functions:['absent'],arguments:[{$:'undefined'}],expected:{$:'boolean',value:true}},
 {name:'strict text equality preserves complete code units',source:'/** @param {string} left @param {string} right @returns {boolean} */ export function equal(left,right){return left===right;}',entry:'equal',functions:['equal'],arguments:[{$:'text',units:[55296]},{$:'text',units:[55296]}],expected:{$:'boolean',value:true}});
const undef={$:'undefined'},nil={$:'null'},object=identity=>({$:'object',identity,callable:false});
const empty=()=>({configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
const data=value=>({...empty(),valuePresent:true,value,writablePresent:true,writable:true});
const entity=(identity,properties=[],prototype=nil)=>({$:'ordinaryObject',identity,prototype,extensible:true,callable:false,properties});
const property=descriptor=>({$:'ordinaryProperty',key:{$:'text',units:[118,97,108,117,101]},descriptor});
const recordSource='/** @param {{value:number}} holder @returns {number} */ export function read(holder){return holder.value;}';
rows.push({name:'actual own record field reaches ordinary heap',source:recordSource,entry:'read',functions:['read'],arguments:[object('1')],objects:[entity('1',[property(data(number(41)))])],expected:number(41)},
 {name:'actual inherited record field retains the source receiver',source:recordSource,entry:'read',functions:['read'],arguments:[object('1')],objects:[entity('1',[],object('2')),entity('2',[property(data(number(42)))])],expected:number(42)},
 {name:'actual source getter body resumes an ordinary record read',source:recordSource+' /** @returns {number} */ function getter(){return 43;}',entry:'read',functions:['read','getter'],arguments:[object('1')],objects:[entity('1',[property({...empty(),getterPresent:true,getter:{$:'object',identity:'11',callable:true},setterPresent:true})])],expected:number(43),registeredCallIdentities:['11'],registeredCalls:[{callee:{$:'object',identity:'11',callable:true},receiver:object('1'),arguments:[]}]});
rows.push({name:'actual record method retains its property reference receiver',source:'/** @param {{method:(value:number)=>number}} holder @param {number} value @returns {number} */ export function invoke(holder,value){return holder.method(value);} /** @param {number} value @returns {number} */ function method(value){return value+1;}',entry:'invoke',functions:['invoke','method'],arguments:[object('1'),number(40)],objects:[entity('1',[{$:'ordinaryProperty',key:{$:'text',units:[109,101,116,104,111,100]},descriptor:data({$:'object',identity:'11',callable:true})}])],expected:number(41),registeredCallIdentities:['11'],registeredCalls:[{callee:{$:'object',identity:'11',callable:true},receiver:object('1'),arguments:[number(40)]}]});
rows.push({name:'actual source loop uses bounded frame storage',source:'/** @param {number} count @returns {number} */ export function sum(count){let total=0;for(let index=0;index<count;index+=1){total+=index;}return total;}',entry:'sum',functions:['sum'],arguments:[number(12)],expected:number(66),maximumFrames:2},
 {name:'loop parameter updates capture all old values before assignment',source:'/** @param {number} count @returns {number} */ export function fibonacci(count){let left=0;let right=1;for(let index=0;index<count;index+=1){const next=left+right;left=right;right=next;}return left;}',entry:'fibonacci',functions:['fibonacci'],arguments:[number(10)],expected:number(55),maximumFrames:2});
const assignmentSource='/** @param {{value:number}} holder @param {number} value @returns {number} */ export function assign(holder,value){holder.value=value;return value;}';
rows.push({name:'actual ordinary record assignment changes the existing heap field',source:assignmentSource,entry:'assign',functions:['assign'],referenceSemantics:true,arguments:[object('1'),number(42)],objects:[entity('1',[property(data(number(3)))])],afterObjects:[entity('1',[property(data(number(42)))])],expected:number(42)},
 {name:'compound assignment captures old field before right hand mutation',source:'/** @param {{value:number}} holder @returns {number} */ export function compound(holder){holder.value+=update(holder);return holder.value;} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=100;return 7;}',entry:'compound',functions:['compound','update'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(3)))])],afterObjects:[entity('1',[property(data(number(10)))])],expected:number(10)},
 {name:'normal setter return false still completes a strict source assignment',source:assignmentSource+' /** @param {number} value @returns {boolean} */ function setter(value){return false;}',entry:'assign',functions:['assign','setter'],referenceSemantics:true,arguments:[object('1'),number(42)],objects:[entity('1',[property({...empty(),getterPresent:true,setterPresent:true,setter:{$:'object',identity:'11',callable:true}})])],expected:number(42),registeredCallIdentities:['11'],registeredCalls:[{callee:{$:'object',identity:'11',callable:true},receiver:object('1'),arguments:[number(42)]}]},
 {name:'readonly strict source assignment requests real realm Error allocation',source:assignmentSource,entry:'assign',functions:['assign'],referenceSemantics:true,arguments:[object('1'),number(42)],objects:[entity('1',[property({...data(number(3)),writable:false})])],expectedOutcome:'allocation',errorClass:'TypeError',errorRealm:'1000',errorReason:'the ordinary assignment was rejected'});
rows.push({name:'greater comparison keeps NaN unordered',source:'/** @param {number} left @param {number} right @returns {boolean} */ export function compare(left,right){return left>right;}',entry:'compare',functions:['compare'],arguments:[number(NaN),number(3)],expected:{$:'boolean',value:false}},
 {name:'less equal keeps NaN unordered',source:'/** @param {number} left @param {number} right @returns {boolean} */ export function compare(left,right){return left<=right;}',entry:'compare',functions:['compare'],arguments:[number(NaN),number(3)],expected:{$:'boolean',value:false}},
 {name:'greater equal compares both zero signs equally',source:'/** @param {number} left @param {number} right @returns {boolean} */ export function compare(left,right){return left>=right;}',entry:'compare',functions:['compare'],arguments:[number(-0),number(0)],expected:{$:'boolean',value:true}},
 {name:'Number NaN predicate follows strict self inequality',source:'/** @param {number} value @returns {boolean} */ export function predicate(value){return Number.isNaN(value);}',entry:'predicate',functions:['predicate'],arguments:[number(NaN)],expected:{$:'boolean',value:true}},
 {name:'Number finite predicate rejects positive infinity',source:'/** @param {number} value @returns {boolean} */ export function predicate(value){return Number.isFinite(value);}',entry:'predicate',functions:['predicate'],arguments:[number(Infinity)],expected:{$:'boolean',value:false}},
 {name:'Number finite predicate accepts negative zero',source:'/** @param {number} value @returns {boolean} */ export function predicate(value){return Number.isFinite(value);}',entry:'predicate',functions:['predicate'],arguments:[number(-0)],expected:{$:'boolean',value:true}},
 {name:'optional Number finite predicate rejects undefined without coercing',source:'/** @param {number | undefined} value @returns {boolean} */ export function predicate(value){return Number.isFinite(value);}',entry:'predicate',functions:['predicate'],arguments:[undef],expected:{$:'boolean',value:false}},
 {name:'Number power uses the existing registered power instruction',source:'/** @param {number} left @param {number} right @returns {number} */ export function power(left,right){return left**right;}',entry:'power',functions:['power'],arguments:[number(2),number(10)],expected:number(1024)});
rows.push({name:'greater comparison preserves both operand mutation order',source:'/** @param {{value:number}} holder @returns {boolean} */ export function compare(holder){return first(holder)>second(holder);} /** @param {{value:number}} holder @returns {number} */ function first(holder){holder.value=1;return 3;} /** @param {{value:number}} holder @returns {number} */ function second(holder){holder.value=2;return 1;}',entry:'compare',functions:['compare','first','second'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(2)))])],expected:{$:'boolean',value:true}},
 {name:'optional Number finite predicate rejects null without coercing',source:'/** @param {number | null | undefined} value @returns {boolean} */ export function predicate(value){return Number.isFinite(value);}',entry:'predicate',functions:['predicate'],arguments:[nil],expected:{$:'boolean',value:false}},
 {name:'Number predicate evaluates every otherwise unused extra argument',source:'/** @param {number} value @param {{value:number}} holder @returns {boolean} */ export function predicate(value,holder){return Number.isNaN(value,update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 0;}',entry:'predicate',functions:['predicate','update'],referenceSemantics:true,arguments:[number(NaN),object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(9)))])],expected:{$:'boolean',value:true}});
rows.push({name:'actual immediately applied body retains its immutable lexical capture',source:'/** @param {number} value @returns {number} */ export function immediate(value){return ((other)=>other+value)(1);}',entry:'immediate',functions:['immediate'],arguments:[number(40)],expected:number(41)},
 {name:'immediate checked body runs after all argument heap effects',source:'/** @param {{value:number}} holder @returns {number} */ export function immediate(holder){const alias=holder;return ((value)=>alias.value+value)(update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=5;return 2;}',entry:'immediate',functions:['immediate','update'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(5)))])],expected:number(7)});
const text=units=>({$:'text',units}),sample=text([65,55357,56832,55296]);
// Original programs and expected code units are literal independent inputs.
// This author writes them without parsing, checking or executing their bodies.
for(const [name,body,expected]of [
 ['UTF16 length counts each surrogate','value.length',number(4)],
 ['character retains an individual high surrogate','value.charAt(1)',text([55357])],
 ['code unit retains an individual low surrogate','value.charCodeAt(2)',number(56832)],
 ['relative character retains an isolated surrogate','value.at(-1)',text([55296])],
 ['code point combines only an actual surrogate pair','value.codePointAt(1)',number(128512)],
 ['code point past the end retains undefined','value.codePointAt(4)',undef],
 ['slice retains partial pairs and positive infinite end','value.slice(-2,Infinity)',text([56832,55296])],
 ['substring swaps its actual converted bounds','value.substring(2,0)',text([65,55357])],
 ['substr keeps its independent length semantics','value.substr(1,2)',text([55357,56832])],
 ['character past the end returns empty text','value.charAt(Infinity)',text([])],
 ['code unit past the end returns NaN','value.charCodeAt(-1)',number(NaN)],
 ['relative character before the beginning retains undefined','value.at(-5)',undef],
 ])rows.push({name,source:'/** @param {string} value */ export function inspect(value){return '+body+';}',entry:'inspect',functions:['inspect'],arguments:[sample],expected});
for(const [name,body,left,right,expected]of [
 ['search can match the low half of a surrogate pair','left.indexOf(right,0)',sample,text([56832]),number(2)],
 ['last search treats NaN as positive infinity','left.lastIndexOf(right,NaN)',sample,text([55357]),number(1)],
 ['prefix uses its supplied position','left.startsWith(right,1)',sample,text([55357]),{$:'boolean',value:true}],
 ['suffix uses its supplied end','left.endsWith(right,3)',sample,text([56832]),{$:'boolean',value:true}],
 ['inclusion uses its supplied position','left.includes(right,2)',sample,text([55357]),{$:'boolean',value:false}],
 ['text ordering compares unsigned code units','left<right',text([55296]),text([57344]),{$:'boolean',value:true}],
 ['concatenation retains isolated surrogates and zero','left+right',text([55296]),text([0,56320]),text([55296,0,56320])],
 ])rows.push({name,source:'/** @param {string} left @param {string} right */ export function inspect(left,right){return '+body+';}',entry:'inspect',functions:['inspect'],arguments:[left,right],expected});
rows.push({name:'trim removes ECMAScript whitespace while preserving isolated code units',source:'/** @param {string} value */ export function inspect(value){return value.trim();}',entry:'inspect',functions:['inspect'],arguments:[text([65279,32,55296,10])],expected:text([55296])});
for(const [name,body,args,expected]of [
 ['minimum keeps negative zero','Math.min(left,right)',[number(-0),number(0)],number(-0)],
 ['maximum keeps positive zero','Math.max(left,right)',[number(-0),number(0)],number(0)],
 ['truncation keeps negative fractional zero','Math.trunc(left)',[number(-0.25),number(0)],number(-0)],
 ])rows.push({name,source:'/** @param {number} left @param {number} right */ export function inspect(left,right){return '+body+';}',entry:'inspect',functions:['inspect'],arguments:args,expected});
const integer=value=>({$:'integer',value});
for(const [name,body,args,expected]of [
 ['integer addition retains values beyond unsigned64','left+right',[integer('18446744073709551616'),integer('7')],integer('18446744073709551623')],
 ['integer subtraction retains negative arbitrary values','left-right',[integer('-18446744073709551616'),integer('7')],integer('-18446744073709551623')],
 ['integer multiplication keeps exact signs','left*right',[integer('-9007199254740993'),integer('3')],integer('-27021597764222979')],
 ['integer negation keeps an exact arbitrary value','-left',[integer('-9007199254740993'),integer('0')],integer('9007199254740993')],
 ['integer ordering never passes through Number','left>right',[integer('9007199254740993'),integer('9007199254740992')],{$:'boolean',value:true}],
 ['integer nonstrict ordering retains equality','left<=right',[integer('-9007199254740993'),integer('-9007199254740993')],{$:'boolean',value:true}],
 ['integer conjunction uses infinite signed extension','left&right',[integer('-1'),integer('9007199254740993')],integer('9007199254740993')],
 ['integer disjunction uses infinite signed extension','left|right',[integer('-8'),integer('3')],integer('-5')],
 ['integer exclusion uses infinite signed extension','left^right',[integer('-1'),integer('3')],integer('-4')],
 ['integer complement uses infinite signed extension','~left',[integer('9007199254740993'),integer('0')],integer('-9007199254740994')],
 ['integer arithmetic shift rounds a negative odd value down','left>>right',[integer('-9'),integer('2')],integer('-3')],
 ['negative integer left shift count reverses the direction','left<<right',[integer('-9'),integer('-2')],integer('-3')],
 ['negative integer right shift count reverses the direction','left>>right',[integer('3'),integer('-4')],integer('48')],
 ])rows.push({name,source:'/** @param {bigint} left @param {bigint} right */ export function inspect(left,right){return '+body+';}',entry:'inspect',functions:['inspect'],arguments:args,expected});
for(const [name,body,args,expected]of [
 ['Number conjunction applies signed32 conversion','left&right',[number(4294967295),number(3)],number(3)],
 ['Number disjunction applies signed32 conversion','left|right',[number(4294967296),number(-1)],number(-1)],
 ['Number exclusion applies signed32 conversion','left^right',[number(4294967295),number(3)],number(-4)],
 ['Number complement converts NaN before complementing','~left',[number(NaN),number(0)],number(-1)],
 ['Number left shift masks its count modulo32','left<<right',[number(1),number(33)],number(2)],
 ['Number signed shift rounds negative values down','left>>right',[number(-9),number(2)],number(-3)],
 ['Number unsigned shift retains the full unsigned32 result','left>>>right',[number(-1),number(0)],number(4294967295)],
 ])rows.push({name,source:'/** @param {number} left @param {number} right */ export function inspect(left,right){return '+body+';}',entry:'inspect',functions:['inspect'],arguments:args,expected});
const optionalTypeSource='/** @param {number | null | undefined} value @returns {string} */ export function inspect(value){return typeof value;}';
for(const [name,value,expected]of [
 ['optional type query retains undefined',undef,text([117,110,100,101,102,105,110,101,100])],
 ['optional type query retains null as object',nil,text([111,98,106,101,99,116])],
 ['optional type query retains the present Number',number(-0),text([110,117,109,98,101,114])],
 ])rows.push({name,source:optionalTypeSource,entry:'inspect',functions:['inspect'],arguments:[value],expected});
const optionalReadSource='/** @param {{value:number} | null | undefined} holder @returns {number | undefined} */ export function inspect(holder){return holder?.value;}';
rows.push({name:'optional field skips undefined receivers',source:optionalReadSource,entry:'inspect',functions:['inspect'],arguments:[undef],expected:undef},
 {name:'optional field skips null receivers',source:optionalReadSource,entry:'inspect',functions:['inspect'],arguments:[nil],expected:undef},
 {name:'optional field retains inherited ordinary values',source:optionalReadSource,entry:'inspect',functions:['inspect'],arguments:[object('1')],objects:[entity('1',[],object('2')),entity('2',[property(data(number(42)))])],expected:number(42)});
const optionalFallbackSource='/** @param {{value:number} | null | undefined} holder @param {{value:number}} counter @returns {number} */ export function inspect(holder,counter){return holder?.value??fallback(counter);} /** @param {{value:number}} counter @returns {number} */ function fallback(counter){counter.value+=1;return 7;}';
rows.push({name:'optional coalescing skips all fallback heap effects for present zero',source:optionalFallbackSource,entry:'inspect',functions:['inspect','fallback'],referenceSemantics:true,arguments:[object('1'),object('2')],objects:[entity('1',[property(data(number(0)))]),entity('2',[property(data(number(0)))])],expected:number(0)},
 {name:'optional coalescing runs the fallback after an absent receiver',source:optionalFallbackSource,entry:'inspect',functions:['inspect','fallback'],referenceSemantics:true,arguments:[nil,object('2')],objects:[entity('2',[property(data(number(0)))])],afterObjects:[entity('2',[property(data(number(1)))])],expected:number(7)});
const refusals=[
 {name:'structured arrays require ordinary allocation lowering',source:'/** @param {number[]} values @returns {number} */ export function size(values){return values.length;}',functions:['size']},
 {name:'captured function construction requires explicit lexical cells',source:'/** @param {number} value @returns {(other:number)=>number} */ export function closure(value){return other=>value+other;}',functions:['closure']},
];
writeProjectData(new URL('parity/fixtures/javascript-call-source.lino',root),{schemaVersion:1,model:'actual checked JavaScript function source call lowering 1',nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rustVersionRange:'>=1.90.0 <1.100.0',rows,refusals});
