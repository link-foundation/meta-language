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
const units=value=>text(Array.from({length:value.length},(_,index)=>value.charCodeAt(index)));
for(const [name,input,expected]of [
 ['empty numeric text converts to positive zero',text([]),number(0)],
 ['numeric text accepts the complete ECMAScript whitespace set',text([65279,160,9]),number(0)],
 ['numeric text retains signed zero',units('-0'),number(-0)],
 ['numeric hexadecimal ties round once to even',units('0x20000000000001'),number(9007199254740992)],
 ['numeric decimal ties round once to even',units('9007199254740995'),number(9007199254740996)],
 ['numeric overflow retains infinity',units('1e309'),number(Infinity)],
 ['numeric underflow retains negative zero',units('-1e-999'),number(-0)],
 ['numeric subnormal parsing retains its exact lowest bit',units('5e-324'),{$:'number',bits:'0000000000000001'}],
 ['numeric text rejects a missing radix digit',units('0x'),number(NaN)],
 ['numeric text rejects signed radix syntax',units('-0x1'),number(NaN)],
 ['numeric text retains isolated surrogate invalidity',text([55296]),number(NaN)],
 ])rows.push({name,source:'/** @param {string} value @returns {number} */ export function inspect(value){return Number(value);}',entry:'inspect',functions:['inspect'],arguments:[input],expected});
rows.push({name:'explicit Number conversion rounds exact BigInt once',source:'/** @param {bigint} value @returns {number} */ export function inspect(value){return Number(value);}',entry:'inspect',functions:['inspect'],arguments:[integer('9007199254740993')],expected:number(9007199254740992)},
 {name:'Number converts true without string intermediates',source:'/** @param {boolean} value @returns {number} */ export function inspect(value){return Number(value);}',entry:'inspect',functions:['inspect'],arguments:[{$:'boolean',value:true}],expected:number(1)},
 {name:'Number optional null retains positive zero',source:'/** @param {number | null | undefined} value @returns {number} */ export function inspect(value){return Number(value);}',entry:'inspect',functions:['inspect'],arguments:[nil],expected:number(0)},
 {name:'Number optional undefined retains NaN',source:'/** @param {number | null | undefined} value @returns {number} */ export function inspect(value){return Number(value);}',entry:'inspect',functions:['inspect'],arguments:[undef],expected:number(NaN)},
 {name:'coercing finite predicate converts empty text to zero',source:'/** @param {string} value @returns {boolean} */ export function inspect(value){return isFinite(value);}',entry:'inspect',functions:['inspect'],arguments:[text([])],expected:{$:'boolean',value:true}},
 {name:'coercing NaN predicate converts invalid text',source:'/** @param {string} value @returns {boolean} */ export function inspect(value){return isNaN(value);}',entry:'inspect',functions:['inspect'],arguments:[units('wrong')],expected:{$:'boolean',value:true}},
 {name:'implicit BigInt Number conversion requests the actual TypeError realm',source:'/** @param {bigint} value @returns {boolean} */ export function inspect(value){return isFinite(value);}',entry:'inspect',functions:['inspect'],arguments:[integer('1')],expectedOutcome:'allocation',errorClass:'TypeError',errorRealm:'1000',errorReason:'Cannot convert a BigInt value to a number'},
 {name:'Number conversion evaluates every extra argument heap effect',source:'/** @param {string} value @param {{value:number}} holder @returns {number} */ export function inspect(value,holder){return Number(value,update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 0;}',entry:'inspect',functions:['inspect','update'],referenceSemantics:true,arguments:[units('41'),object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(9)))])],expected:number(41)});
for(const [name,body,input,expected]of [
 ['absolute value clears a negative zero sign','Math.abs(value)',number(-0),number(0)],
 ['absolute value retains positive infinity','Math.abs(value)',number(-Infinity),number(Infinity)],
 ['floor rounds a negative fraction downward','Math.floor(value)',number(-0.25),number(-1)],
 ['ceiling keeps negative fractional zero','Math.ceil(value)',number(-0.25),number(-0)],
 ['round keeps the negative halfway zero sign','Math.round(value)',number(-0.5),number(-0)],
 ['round resolves a positive halfway value upward','Math.round(value)',number(0.5),number(1)],
 ['round resolves a negative halfway value upward','Math.round(value)',number(-1.5),number(-1)],
 ['sign retains negative zero','Math.sign(value)',number(-0),number(-0)],
 ['sign retains NaN','Math.sign(value)',number(NaN),number(NaN)],
 ['sign maps negative infinity to minus one','Math.sign(value)',number(-Infinity),number(-1)],
 ['square root retains negative zero','Math.sqrt(value)',number(-0),number(-0)],
 ['negative square root returns NaN','Math.sqrt(value)',number(-1),number(NaN)],
 ['infinite square root retains infinity','Math.sqrt(value)',number(Infinity),number(Infinity)],
 ['integer predicate accepts negative zero','Number.isInteger(value)',number(-0),{$:'boolean',value:true}],
 ['integer predicate rejects infinity','Number.isInteger(value)',number(Infinity),{$:'boolean',value:false}],
 ['integer predicate rejects fractions','Number.isInteger(value)',number(1.5),{$:'boolean',value:false}],
 ['safe integer predicate accepts its upper boundary','Number.isSafeInteger(value)',number(9007199254740991),{$:'boolean',value:true}],
 ['safe integer predicate rejects the next integer','Number.isSafeInteger(value)',number(9007199254740992),{$:'boolean',value:false}],
 ['safe integer predicate rejects NaN','Number.isSafeInteger(value)',number(NaN),{$:'boolean',value:false}],
 ])rows.push({name,source:'/** @param {number} value */ export function inspect(value){return '+body+';}',entry:'inspect',functions:['inspect'],arguments:[input],expected,...(name==='negative square root returns NaN'?{expectedEncoding:'implementation-selected NaN'}:{})});
rows.push({name:'optional integer predicate rejects null without coercion',source:'/** @param {number | null | undefined} value @returns {boolean} */ export function inspect(value){return Number.isInteger(value);}',entry:'inspect',functions:['inspect'],arguments:[nil],expected:{$:'boolean',value:false}},
 {name:'optional safe integer predicate accepts present zero',source:'/** @param {number | null | undefined} value @returns {boolean} */ export function inspect(value){return Number.isSafeInteger(value);}',entry:'inspect',functions:['inspect'],arguments:[number(0)],expected:{$:'boolean',value:true}},
 {name:'round performs the actual text conversion before rounding',source:'/** @param {string} value @returns {number} */ export function inspect(value){return Math.round(value);}',entry:'inspect',functions:['inspect'],arguments:[units('-0.5')],expected:number(-0)},
 {name:'source minimum preserves its empty infinity identity',source:'/** @returns {number} */ export function inspect(){return Math.min();}',entry:'inspect',functions:['inspect'],arguments:[],expected:number(Infinity)},
 {name:'source maximum preserves its empty negative infinity identity',source:'/** @returns {number} */ export function inspect(){return Math.max();}',entry:'inspect',functions:['inspect'],arguments:[],expected:number(-Infinity)});
// Literal independent text expectations; no conversion program is evaluated.
for(const [name,input,expected]of [
 ['Number text erases negative zero',number(-0),'0'],
 ['Number text uses its NaN spelling',number(NaN),'NaN'],
 ['Number text retains positive infinity',number(Infinity),'Infinity'],
 ['Number text retains negative infinity',number(-Infinity),'-Infinity'],
 ['Number text keeps ordinary shortest decimal',number(0.1),'0.1'],
 ['Number text uses the positive exponent boundary',number(1e21),'1e+21'],
 ['Number text keeps the preceding fixed decimal boundary',number(1e20),'100000000000000000000'],
 ['Number text uses the negative exponent boundary',number(1e-7),'1e-7'],
 ['Number text keeps the small fixed decimal boundary',number(1e-6),'0.000001'],
 ['Number text keeps the smallest positive subnormal',number(5e-324),'5e-324'],
 ['Number text keeps the largest finite scalar',number(1.7976931348623157e308),'1.7976931348623157e+308'],
 ['Number text keeps the safe integer boundary',number(9007199254740991),'9007199254740991']
])rows.push({name,source:'/** @param {number} value @returns {string} */ export function inspect(value){return String(value);}',entry:'inspect',functions:['inspect'],arguments:[input],expected:units(expected)});
rows.push(
 {name:'integer text has no Number rounding or BigInt suffix',source:'/** @param {bigint} value @returns {string} */ export function inspect(value){return value.toString();}',entry:'inspect',functions:['inspect'],arguments:[integer('-184467440737095516160000000000001')],expected:units('-184467440737095516160000000000001')},
 {name:'Boolean text uses the true spelling',source:'/** @param {boolean} value @returns {string} */ export function inspect(value){return String(value);}',entry:'inspect',functions:['inspect'],arguments:[{$:'boolean',value:true}],expected:units('true')},
 {name:'Boolean text uses the false spelling',source:'/** @param {boolean} value @returns {string} */ export function inspect(value){return value.toString();}',entry:'inspect',functions:['inspect'],arguments:[{$:'boolean',value:false}],expected:units('false')},
 {name:'optional text retains actual undefined',source:'/** @param {number | null | undefined} value @returns {string} */ export function inspect(value){return String(value);}',entry:'inspect',functions:['inspect'],arguments:[{$:'undefined'}],expected:units('undefined')},
 {name:'optional text retains actual null',source:'/** @param {number | null | undefined} value @returns {string} */ export function inspect(value){return String(value);}',entry:'inspect',functions:['inspect'],arguments:[{$:'null'}],expected:units('null')},
 {name:'optional text applies Number formatting only to the present value',source:'/** @param {number | null | undefined} value @returns {string} */ export function inspect(value){return String(value);}',entry:'inspect',functions:['inspect'],arguments:[number(-0)],expected:units('0')},
 {name:'text conversion evaluates its actual operand once',source:'/** @param {{value:number}} holder @returns {string} */ export function inspect(holder){return String(update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=holder.value+1;return holder.value;}',entry:'inspect',functions:['inspect','update'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(1)))])],expected:units('1')});
rows.push(
 {name:'String call with no arguments returns empty text',source:'/** @returns {string} */ export function inspect(){return String();}',entry:'inspect',functions:['inspect'],arguments:[],expected:units('')},
 {name:'String call with explicit undefined retains its spelling',source:'/** @returns {string} */ export function inspect(){return String(undefined);}',entry:'inspect',functions:['inspect'],arguments:[],expected:units('undefined')},
 {name:'String call captures its first property before extra argument effects',source:'/** @param {{value:number}} holder @returns {string} */ export function inspect(holder){return String(holder.value,update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 0;}',entry:'inspect',functions:['inspect','update'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(9)))])],expected:units('0')},
 {name:'String call evaluates all extra arguments once in source order',source:'/** @param {{value:number}} holder @returns {string} */ export function inspect(holder){return String(40,first(holder),second(holder));} /** @param {{value:number}} holder @returns {number} */ function first(holder){holder.value=holder.value*10+1;return 0;} /** @param {{value:number}} holder @returns {number} */ function second(holder){holder.value=holder.value*10+2;return 0;}',entry:'inspect',functions:['inspect','first','second'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(12)))])],expected:units('40')},
 {name:'String call preserves abrupt extra argument conversion',source:'/** @param {bigint} value @returns {string} */ export function inspect(value){return String(40,isFinite(value));}',entry:'inspect',functions:['inspect'],arguments:[integer('1')],expectedOutcome:'allocation',errorClass:'TypeError',errorRealm:'1000',errorReason:'Cannot convert a BigInt value to a number'});
for(const [name,leftType,rightType,left,right,operation,expected]of [
 ['mixed Number order keeps the integer beyond exact Number range','number','bigint',number(9007199254740992),integer('9007199254740993'),'<',true],
 ['mixed reversed order keeps the original BigInt owner','bigint','number',integer('9007199254740993'),number(9007199254740992),'>',true],
 ['mixed negative fraction orders below zero BigInt','number','bigint',number(-0.5),integer('0'),'<',true],
 ['mixed positive fraction orders above zero BigInt','number','bigint',number(0.5),integer('0'),'>',true],
 ['mixed loose equality identifies negative zero and zero BigInt','number','bigint',number(-0),integer('0'),'==',true],
 ['mixed positive infinity orders above a finite BigInt','number','bigint',number(Infinity),integer('9007199254740993'),'>',true],
 ['mixed negative infinity orders below a finite BigInt','number','bigint',number(-Infinity),integer('-9007199254740993'),'<=',true],
 ['mixed NaN less-than remains unordered','number','bigint',number(NaN),integer('1'),'<',false],
 ['mixed NaN less-or-equal remains unordered','number','bigint',number(NaN),integer('1'),'<=',false],
 ['mixed NaN greater-than remains unordered','number','bigint',number(NaN),integer('1'),'>',false],
 ['mixed NaN greater-or-equal remains unordered','number','bigint',number(NaN),integer('1'),'>=',false],
 ['mixed NaN loose equality remains false','number','bigint',number(NaN),integer('1'),'==',false],
 ['mixed NaN loose inequality remains true','number','bigint',number(NaN),integer('1'),'!=',true],
 ['mixed decimal text preserves the unrounded integer','string','bigint',units('9007199254740993'),integer('9007199254740993'),'==',true],
 ['mixed hexadecimal text uses StringIntegerLiteral','string','bigint',units('0x10'),integer('16'),'==',true],
 ['mixed signed hexadecimal text remains invalid','string','bigint',units('+0x10'),integer('16'),'==',false],
 ['mixed exponent text does not use Number conversion','string','bigint',units('1e3'),integer('1000'),'==',false],
 ['mixed invalid integer text remains unordered','string','bigint',units('1e3'),integer('1000'),'<=',false],
 ['mixed reversed invalid text remains unordered','bigint','string',integer('1000'),units('1e3'),'>=',false],
 ['mixed whitespace text equals zero BigInt','string','bigint',units('\uFEFF \t\n'),integer('0'),'==',true],
 ['mixed empty text equals zero BigInt','string','bigint',units(''),integer('0'),'==',true],
 ['mixed surrogate text is unequal to zero BigInt','string','bigint',{$:'text',units:[55296]},integer('0'),'!=',true]
])rows.push({name,source:'/** @param {'+leftType+'} left @param {'+rightType+'} right @returns {boolean} */ export function inspect(left,right){return left'+operation+'right;}',entry:'inspect',functions:['inspect'],arguments:[left,right],expected:{$:'boolean',value:expected}});
rows.push({name:'mixed comparison evaluates both source operands in original order',source:'/** @param {{value:number}} holder @returns {boolean} */ export function inspect(holder){return first(holder)<second(holder);} /** @param {{value:number}} holder @returns {number} */ function first(holder){holder.value=holder.value*10+1;return 0;} /** @param {{value:number}} holder @returns {bigint} */ function second(holder){holder.value=holder.value*10+2;return 1n;}',entry:'inspect',functions:['inspect','first','second'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(12)))])],expected:{$:'boolean',value:true}});
for(const errorClass of ['Error','TypeError','RangeError','SyntaxError','ReferenceError','URIError','EvalError'])rows.push({name:'source '+errorClass+' retains its exact UTF16 message and realm',source:'/** @param {string} value @returns {number} */ export function inspect(value){throw new '+errorClass+'(value);}',entry:'inspect',functions:['inspect'],arguments:[{$:'text',units:[55296,0,56320]}],expectedOutcome:'sourceAllocation',errorClass,errorRealm:'1000',errorMessage:{$:'text',units:[55296,0,56320]},messagePresent:true});
for(const [name,source,args,errorClass,message,present]of [
 ['source Error without arguments has no own message','/** @returns {number} */ export function inspect(){throw new Error();}',[],'Error',units(''),false],
 ['source Error with undefined has no own message','/** @returns {number} */ export function inspect(){throw new Error(undefined);}',[],'Error',units(''),false],
 ['source Error with empty text has an own message','/** @returns {number} */ export function inspect(){throw new Error("");}',[],'Error',units(''),true],
 ['source TypeError converts a null message','/** @returns {number} */ export function inspect(){throw new TypeError(null);}',[],'TypeError',units('null'),true],
 ['source RangeError formats negative zero as message text','/** @param {number} value @returns {number} */ export function inspect(value){throw new RangeError(value);}',[number(-0)],'RangeError',units('0'),true],
 ['source SyntaxError preserves an unrounded BigInt message','/** @param {bigint} value @returns {number} */ export function inspect(value){throw new SyntaxError(value);}',[integer('9007199254740993')],'SyntaxError',units('9007199254740993'),true],
 ['source Error converts a Boolean message','/** @param {boolean} value @returns {number} */ export function inspect(value){throw new Error(value);}',[{$:'boolean',value:false}],'Error',units('false'),true],
 ['source optional undefined retains absent message presence','/** @param {string | null | undefined} value @returns {number} */ export function inspect(value){throw new Error(value);}',[{$:'undefined'}],'Error',units(''),false],
 ['source optional null retains text message presence','/** @param {string | null | undefined} value @returns {number} */ export function inspect(value){throw new Error(value);}',[{$:'null'}],'Error',units('null'),true]
])rows.push({name,source,entry:'inspect',functions:['inspect'],arguments:args,expectedOutcome:'sourceAllocation',errorClass,errorRealm:'1000',errorMessage:message,messagePresent:present});
rows.push({name:'source Error evaluates ignored arguments before message conversion',source:'/** @param {{value:number}} holder @returns {number} */ export function inspect(holder){throw new TypeError(holder.value,update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 0;}',entry:'inspect',functions:['inspect','update'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(9)))])],expectedOutcome:'sourceAllocation',errorClass:'TypeError',errorRealm:'1000',errorMessage:units('0'),messagePresent:true});
for(const [name,type,input,expected]of [
 ['BigInt parses decimal text without Number rounding','string',units('9007199254740993'),integer('9007199254740993')],
 ['BigInt parses hexadecimal text','string',units('0x10'),integer('16')],
 ['BigInt parses binary text','string',units('0b101'),integer('5')],
 ['BigInt parses octal text','string',units('0o17'),integer('15')],
 ['BigInt maps empty text to zero','string',units(''),integer('0')],
 ['BigInt trims integer whitespace','string',units(' \t-42\n'),integer('-42')],
 ['BigInt preserves negative textual zero','string',units('-0'),integer('0')],
 ['BigInt converts an exact large Number','number',number(9007199254740992),integer('9007199254740992')],
 ['BigInt converts a negative integral Number','number',number(-42),integer('-42')],
 ['BigInt converts negative Number zero','number',number(-0),integer('0')],
 ['BigInt converts Boolean true','boolean',{$:'boolean',value:true},integer('1')],
 ['BigInt converts Boolean false','boolean',{$:'boolean',value:false},integer('0')],
 ['BigInt identity retains its existing integer','bigint',integer('-9007199254740993'),integer('-9007199254740993')]
])rows.push({name,source:'/** @param {'+type+'} value @returns {bigint} */ export function inspect(value){return BigInt(value);}',entry:'inspect',functions:['inspect'],arguments:[input],expected});
for(const [name,input,message]of [
 ['BigInt rejects exponent text',units('1e3'),'Cannot convert 1e3 to a BigInt'],
 ['BigInt rejects a signed hexadecimal prefix',units('+0x10'),'Cannot convert +0x10 to a BigInt'],
 ['BigInt preserves invalid surrogate text in its SyntaxError',{$:'text',units:[55296]},'Cannot convert \uD800 to a BigInt']
])rows.push({name,source:'/** @param {string} value @returns {bigint} */ export function inspect(value){return BigInt(value);}',entry:'inspect',functions:['inspect'],arguments:[input],expectedOutcome:'allocation',errorClass:'SyntaxError',errorRealm:'1000',...(message.isWellFormed()?{errorReason:message}:{errorReasonUnits:Array.from({length:message.length},(_,index)=>message.charCodeAt(index))})});
for(const [name,input,message]of [
 ['BigInt rejects a fractional Number',number(1.5),'The number 1.5 cannot be converted to a BigInt because it is not an integer'],
 ['BigInt rejects NaN',number(NaN),'The number NaN cannot be converted to a BigInt because it is not an integer'],
 ['BigInt rejects infinity',number(Infinity),'The number Infinity cannot be converted to a BigInt because it is not an integer']
])rows.push({name,source:'/** @param {number} value @returns {bigint} */ export function inspect(value){return BigInt(value);}',entry:'inspect',functions:['inspect'],arguments:[input],expectedOutcome:'allocation',errorClass:'RangeError',errorRealm:'1000',...(message.isWellFormed()?{errorReason:message}:{errorReasonUnits:Array.from({length:message.length},(_,index)=>message.charCodeAt(index))})});
for(const [name,operation,left,right,expected]of [
 ['dynamic BigInt division truncates negative quotients','/',integer('-7'),integer('3'),integer('-2')],
 ['dynamic BigInt remainder retains the dividend sign','%',integer('-7'),integer('3'),integer('-1')],
 ['BigInt exponentiation retains its exact result','**',integer('2'),integer('64'),integer('18446744073709551616')],
 ['BigInt zero exponent retains its identity','**',integer('0'),integer('0'),integer('1')]
])rows.push({name,source:'/** @param {bigint} left @param {bigint} right @returns {bigint} */ export function inspect(left,right){return left'+operation+'right;}',entry:'inspect',functions:['inspect'],arguments:[left,right],expected});
for(const [name,operation,right,message]of [
 ['dynamic BigInt division preserves zero-divisor failure','/',integer('0'),'Division by zero'],
 ['dynamic BigInt remainder preserves zero-divisor failure','%',integer('0'),'Division by zero'],
 ['BigInt exponentiation preserves negative-exponent failure','**',integer('-1'),'Exponent must be positive']
])rows.push({name,source:'/** @param {bigint} left @param {bigint} right @returns {bigint} */ export function inspect(left,right){return left'+operation+'right;}',entry:'inspect',functions:['inspect'],arguments:[integer('2'),right],expectedOutcome:'allocation',errorClass:'RangeError',errorRealm:'1000',...(message.isWellFormed()?{errorReason:message}:{errorReasonUnits:Array.from({length:message.length},(_,index)=>message.charCodeAt(index))})});
rows.push({name:'BigInt conversion captures ignored argument effects before parsing',source:'/** @param {string} value @param {{value:number}} holder @returns {bigint} */ export function inspect(value,holder){return BigInt(value,update(holder));} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 0;}',entry:'inspect',functions:['inspect','update'],referenceSemantics:true,arguments:[units('42'),object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(9)))])],expected:integer('42')});
// Constant-width originals exercise the actual shared construction plan.
for(const [method,width,value,result]of [
 ['asUintN','0','123','0'],['asIntN','0','-1','0'],
 ['asUintN','-0','1','0'],['asIntN','-0.5','123','0'],
 ['asUintN','1','-1','1'],['asIntN','1','1','-1'],
 ['asUintN','7','-1','127'],['asIntN','7','127','-1'],
 ['asUintN','8','-1','255'],['asIntN','8','255','-1'],
 ['asUintN','24','-1','16777215'],['asIntN','24','8388608','-8388608'],
 ['asIntN','32.9','4294967295','-1'],
 ['asUintN','64','-1','18446744073709551615'],
 ['asIntN','128','340282366920938463463374607431768211455','-1'],
 ['asUintN','256','1','1'],['asIntN','256','-1','-1'],
 ['asUintN','NaN','123','0']
])rows.push({name:'BigInt fixed width '+method+' '+width+' on '+value,source:'/** @param {bigint} value @returns {bigint} */ export function inspect(value){return BigInt.'+method+'('+width+',value);}',entry:'inspect',functions:['inspect'],arguments:[integer(value)],expected:integer(result)});
rows.push({name:'BigInt fixed width evaluates its value operand exactly once',source:'/** @param {{value:number}} holder @returns {bigint} */ export function inspect(holder){return BigInt.asUintN(7,update(holder));} /** @param {{value:number}} holder @returns {bigint} */ function update(holder){holder.value+=1;return -1n;}',entry:'inspect',functions:['inspect','update'],referenceSemantics:true,arguments:[object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(1)))])],expected:integer('127')});
rows.push({name:'BigInt fixed width captures value before ignored arguments',source:'/** @param {bigint} value @param {{value:number}} holder @returns {bigint} */ export function inspect(value,holder){return BigInt.asIntN(7,first(value,holder),second(holder));} /** @param {bigint} value @param {{value:number}} holder @returns {bigint} */ function first(value,holder){holder.value=1;return value;} /** @param {{value:number}} holder @returns {number} */ function second(holder){holder.value=holder.value*10+2;return 0;}',entry:'inspect',functions:['inspect','first','second'],referenceSemantics:true,arguments:[integer('127'),object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(12)))])],expected:integer('-1')});
rows.push({name:'BigInt fixed width preserves abrupt ignored arguments',source:'/** @param {bigint} value @returns {bigint} */ export function inspect(value){return BigInt.asUintN(7,value,fail());} /** @returns {number} */ function fail(){throw new TypeError("ignored cast argument");}',entry:'inspect',functions:['inspect','fail'],arguments:[integer('-1')],expectedOutcome:'sourceAllocation',errorClass:'TypeError',errorRealm:'1000',errorMessage:units('ignored cast argument'),messagePresent:true});
for(const [name,type,input,result]of [
 ['present optional Number adds normally','number | undefined',number(7),number(8)],
 ['undefined optional Number retains NaN','number | undefined',undef,number(NaN)],
 ['null optional Number converts to zero','number | null | undefined',nil,number(1)],
 ['true optional Boolean converts to one','boolean | null | undefined',{$:'boolean',value:true},number(2)],
 ['false optional Boolean converts to zero','boolean | null | undefined',{$:'boolean',value:false},number(1)],
 ['null optional Boolean converts to zero','boolean | null | undefined',nil,number(1)],
 ['undefined optional Boolean retains NaN','boolean | null | undefined',undef,number(NaN)]
])rows.push({name:'nullable arithmetic '+name,source:'/** @param {'+type+'} value @returns {number} */ export function inspect(value){return value+1;}',entry:'inspect',functions:['inspect'],arguments:[input],expected:result});
for(const [name,type,input,result]of [
 ['null orders as zero','number | null | undefined',nil,true],
 ['undefined is unordered','number | null | undefined',undef,false],
 ['present Number retains its order','number | null | undefined',number(5),false],
 ['true Boolean orders as one','boolean | null | undefined',{$:'boolean',value:true},false],
 ['false Boolean orders as zero','boolean | null | undefined',{$:'boolean',value:false},true]
])rows.push({name:'nullable arithmetic '+name,source:'/** @param {'+type+'} value @returns {boolean} */ export function inspect(value){return value<1;}',entry:'inspect',functions:['inspect'],arguments:[input],expected:{$:'boolean',value:result}});
for(const [name,type,input,result]of [
 ['negated null retains negative zero','number | null | undefined',nil,number(-0)],
 ['negated false retains negative zero','boolean | null | undefined',{$:'boolean',value:false},number(-0)],
 ['negated negative zero becomes positive zero','number | null | undefined',number(-0),number(0)]
])rows.push({name:'nullable arithmetic '+name,source:'/** @param {'+type+'} value @returns {number} */ export function inspect(value){return -value;}',entry:'inspect',functions:['inspect'],arguments:[input],expected:result});
for(const [name,type,input,result]of [
 ['undefined bitwise conversion yields zero','number | undefined',undef,number(0)],
 ['null bitwise conversion yields zero','number | null | undefined',nil,number(0)],
 ['fractional Number bitwise conversion truncates','number | null | undefined',number(5.5),number(5)],
 ['true bitwise conversion yields one','boolean | null | undefined',{$:'boolean',value:true},number(1)]
])rows.push({name:'nullable arithmetic '+name,source:'/** @param {'+type+'} value @returns {number} */ export function inspect(value){return value|0;}',entry:'inspect',functions:['inspect'],arguments:[input],expected:result});
for(const [name,message,construction]of [
 ['plain literal Error','literal source error','new '],
 ['empty literal Error','','new '],
 ['literal Error preserves surrogate and NUL','literal\uD800\0','new '],
 ['literal Error factory call','literal factory error','']
])rows.push({name:'legacy source Error '+name,source:'/** @returns {number} */ export function inspect(){throw '+construction+'Error('+JSON.stringify(message)+');}',entry:'inspect',functions:['inspect'],arguments:[],expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units(message),messagePresent:true});
const legacyConditional='/** @param {number} value @returns {number} */ export function inspect(value){if(value<0)throw new Error("negative source value");return value;}';
rows.push({name:'legacy source Error retains its actual conditional branch',source:legacyConditional,entry:'inspect',functions:['inspect'],arguments:[number(-1)],expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units('negative source value'),messagePresent:true},
 {name:'legacy source Error leaves the successful branch observable',source:legacyConditional,entry:'inspect',functions:['inspect'],arguments:[number(9)],expected:number(9)},
 {name:'legacy source Error retains its actual callee source frame',source:'/** @returns {number} */ export function inspect(){return fail();} /** @returns {number} */ function fail(){throw new Error("callee source failure");}',entry:'inspect',functions:['inspect','fail'],arguments:[],expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units('callee source failure'),messagePresent:true});
const naturalIncrement='/** @param {bigint} value @returns {bigint} */ export function inspect(value){if(value<0n)throw new Error("natural required");return value+1n;}';
for(const [value,result]of [['0','1'],['4','5']])rows.push({name:'guarded natural increment '+value,source:naturalIncrement,entry:'inspect',functions:['inspect'],arguments:[integer(value)],expected:integer(result)});
rows.push({name:'guarded natural negative entry retains its Error',source:naturalIncrement,entry:'inspect',functions:['inspect'],arguments:[integer('-1')],expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units('natural required'),messagePresent:true});
const orderedNaturals='/** @param {bigint} first @param {bigint} second @returns {bigint} */ export function inspect(first,second){if(second<0n)throw new Error("second natural");if(first<0n)throw new Error("first natural");return first+second;}';
for(const [name,arguments_,message]of [['both negative retain second-first guard order',[integer('-1'),integer('-2')],'second natural'],['first negative follows successful second guard',[integer('-1'),integer('2')],'first natural']])rows.push({name:'guarded natural '+name,source:orderedNaturals,entry:'inspect',functions:['inspect'],arguments:arguments_,expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units(message),messagePresent:true});
rows.push({name:'guarded natural multiple successful guards retain integer addition',source:orderedNaturals,entry:'inspect',functions:['inspect'],arguments:[integer('1'),integer('2')],expected:integer('3')});
const naturalFactorial='/** @param {bigint} value @returns {bigint} */ export function factorial(value){if(value<0n)throw new Error("natural factorial");if(value===0n)return 1n;return value*factorial(value-1n);}';
for(const [value,result]of [['0','1'],['5','120']])rows.push({name:'guarded natural recursive factorial '+value,source:naturalFactorial,entry:'factorial',functions:['factorial'],arguments:[integer(value)],expected:integer(result)});
rows.push({name:'guarded natural tail recursion retains predecessor and accumulator',source:'/** @param {bigint} value @param {bigint} accumulator @returns {bigint} */ export function sum(value,accumulator){if(value<0n)throw new Error("natural sum");if(value===0n)return accumulator;return sum(value-1n,accumulator+value);}',entry:'sum',functions:['sum'],arguments:[integer('10'),integer('0')],expected:integer('55')});
const naturalTextMessage='natural text\uD800\0',naturalText='/** @param {bigint} value @returns {string} */ export function inspect(value){if(value<0n)throw new Error('+JSON.stringify(naturalTextMessage)+');return String(value);}';
rows.push({name:'guarded natural widens to exact decimal text',source:naturalText,entry:'inspect',functions:['inspect'],arguments:[integer('7')],expected:units('7')},
 {name:'guarded natural Error retains all source message units',source:naturalText,entry:'inspect',functions:['inspect'],arguments:[integer('-1')],expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units(naturalTextMessage),messagePresent:true});
const naturalArguments='/** @param {bigint} value @param {{value:number}} holder @returns {bigint} */ export function inspect(value,holder){return requireNatural(value,update(holder));} /** @param {bigint} value @returns {bigint} */ function requireNatural(value){if(value<0n)throw new Error("natural argument");return value+1n;} /** @param {{value:number}} holder @returns {number} */ function update(holder){holder.value=9;return 0;}';
for(const [value,result]of [['2',integer('3')],['-1',null]])rows.push({name:'guarded natural ignored argument effects precede its guard '+value,source:naturalArguments,entry:'inspect',functions:['inspect','requireNatural','update'],referenceSemantics:true,arguments:[integer(value),object('1')],objects:[entity('1',[property(data(number(0)))])],afterObjects:[entity('1',[property(data(number(9)))])],...(result?{expected:result}:{expectedOutcome:'sourceAllocation',errorClass:'Error',errorRealm:'1000',errorMessage:units('natural argument'),messagePresent:true})});
rows.push({name:'guarded natural subtraction retains a negative integer result',source:'/** @param {bigint} first @param {bigint} second @returns {bigint} */ export function inspect(first,second){if(first<0n)throw new Error("first natural");if(second<0n)throw new Error("second natural");return first-second;}',entry:'inspect',functions:['inspect'],arguments:[integer('1'),integer('2')],expected:integer('-1')});
const naturalMixed='/** @param {bigint} value @param {number} limit @returns {boolean} */ export function inspect(value,limit){if(value<0n)throw new Error("natural comparison");return value>limit;}';
for(const [value,limit,result]of [['1',1.5,false],['2',1.5,true],['9007199254740993',9007199254740992,true]])rows.push({name:'guarded natural exact mixed comparison '+value,source:naturalMixed,entry:'inspect',functions:['inspect'],arguments:[integer(value),number(limit)],expected:{$:'boolean',value:result}});
for(const [name,input,expected]of [
 ['positive zero becomes negative zero',number(0),number(-0)],
 ['negative zero becomes positive zero',number(-0),number(0)],
 ['finite value retains its negated magnitude',number(42),number(-42)],
 ['positive infinity becomes negative infinity',number(Infinity),number(-Infinity)],
 ['negative infinity becomes positive infinity',number(-Infinity),number(Infinity)],
 ['canonical quiet NaN follows scalar unary negation',{$:'number',bits:'7ff8000000000000'},{$:'number',bits:'fff8000000000000'}]
])rows.push({name:'number scalar negation '+name,source:'/** @param {number} value @returns {number} */ export function inspect(value){return -value;}',entry:'inspect',functions:['inspect'],arguments:[input],expected});
const refusals=[
 {name:'guarded natural cross-realm casts require complete realm lowering',source:'/** @param {bigint} value @returns {bigint} */ export function inspect(value){return natural(value);} /** @param {bigint} value @returns {bigint} */ function natural(value){if(value<0n)throw new Error("natural realm");return value;}',functions:['inspect','natural'],functionRealms:{inspect:'1000',natural:'1001'}},
 {name:'structured arrays require ordinary allocation lowering',source:'/** @param {number[]} values @returns {number} */ export function size(values){return values.length;}',functions:['size']},
 {name:'captured function construction requires explicit lexical cells',source:'/** @param {number} value @returns {(other:number)=>number} */ export function closure(value){return other=>value+other;}',functions:['closure']},
];
writeProjectData(new URL('parity/fixtures/javascript-call-source.lino',root),{schemaVersion:1,model:'actual checked JavaScript function source call lowering 1',nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rustVersionRange:'>=1.90.0 <1.100.0',rows,refusals});
