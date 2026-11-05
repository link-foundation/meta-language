// Literal expectations only. Never invoke the represented semantic operations.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const undef={$:'undefined'},nil={$:'null'},bool=value=>({$:'boolean',value});
const num=bits=>({$:'number',bits}),integer=value=>({$:'integer',value}),text=units=>({$:'text',units});
const object=(identity,callable=false)=>({$:'object',identity,callable}),symbol=identity=>({$:'symbol',identity});
const one=num('3ff0000000000000'),two=num('4000000000000000'),positiveZero=num('0000000000000000'),negativeZero=num('8000000000000000');
const nanA=num('7ff8000000000001'),nanB=num('7ff8000000000002'),fn=object('18446744073709551615',true),other=object('9007199254740993',true);
const empty=()=>({configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,
 valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
function proposal(fields){const result=empty();for(const [key,value]of Object.entries(fields)){result[key+'Present']=true;result[key]=value;}return result;}
const data=(value=one,writable=false,configurable=false,enumerable=false)=>proposal({configurable,enumerable,value,writable});
const accessor=(getter=fn,setter=undef,configurable=false,enumerable=false)=>proposal({configurable,enumerable,getter,setter});
const cases=[];
function add(name,current,proposed,disposition,writes=[],options={}){cases.push({name,exists:true,extensible:true,current,proposed,disposition,error:'',writes,oracle:'ordinary',...options});}
add('create a generic descriptor as a complete data property',empty(),empty(),'create',['configurable','enumerable','value','writable'],{exists:false});
add('create explicit data attributes',empty(),data(two,true,true,true),'create',['configurable','enumerable','value','writable'],{exists:false});
add('create an accessor without executing it',empty(),proposal({getter:fn}),'create',['configurable','enumerable','get','set'],{exists:false});
add('nonextensible receivers reject new properties',empty(),proposal({value:two}),'reject',[],{exists:false,extensible:false});
add('nonextensible receivers still allow existing writable properties',data(one,true),proposal({value:two}),'update',['value'],{extensible:false});
add('empty proposals preserve storage',data(),empty(),'keep');
add('nonconfigurable properties reject configurable true',data(),proposal({configurable:true}),'reject');
add('nonconfigurable properties reject enumerable changes',data(),proposal({enumerable:true}),'reject');
add('nonconfigurable properties reject kind conversion',data(),proposal({getter:fn}),'reject');
add('nonwritable properties reject writable true',data(),proposal({writable:true}),'reject');
add('readonly SameValue preserves the actual old value',data(),proposal({value:one}),'keep');
add('readonly different values reject',data(),proposal({value:two}),'reject');
add('readonly signed zero changes reject',data(positiveZero),proposal({value:negativeZero}),'reject');
add('readonly NaN keeps its original payload bits',data(nanA),proposal({value:nanB}),'keep');
add('writable nonconfigurable properties can become readonly',data(one,true),proposal({writable:false}),'update',['writable']);
add('writable updates preserve requested fields',data(one,true),proposal({value:two,writable:false}),'update',['value','writable']);
add('generic configurable updates do not rewrite NaN values',data(nanA,false,true),proposal({enumerable:true}),'update',['enumerable']);
add('configurable data properties convert to accessors',data(one,true,true,true),proposal({getter:fn}),'convert',['configurable','enumerable','get','set']);
add('configurable accessor properties convert to data',accessor(fn,other,true,true),proposal({value:two}),'convert',['configurable','enumerable','value','writable']);
add('same getter identity is accepted',accessor(),proposal({getter:fn}),'update',['get']);
add('different getter identity is rejected',accessor(),proposal({getter:other}),'reject');
add('different setter identity is rejected',accessor(fn,fn),proposal({setter:other}),'reject');
add('undefined is a valid absent accessor function',accessor(undef,undef),proposal({getter:undef}),'update',['get']);
add('mixed descriptor kinds throw before mutation',data(),proposal({value:two,getter:fn}),'reject',[],{error:'TypeError'});
add('noncallable getters throw before mutation',data(),proposal({getter:object('3')}),'reject',[],{error:'TypeError'});
add('null setters throw before mutation',data(),proposal({setter:nil}),'reject',[],{error:'TypeError'});
add('invalid semantic IDs remain outside the host oracle model',data(),proposal({value:object('0')}),'reject',[],{error:'InvalidSemanticValue',oracle:'invalid-model'});
add('invalid current descriptor is an explicit model error',empty(),empty(),'reject',[],{error:'InvalidCurrentDescriptor',oracle:'invalid-model'});
const ignored={...empty(),value:object('0'),getter:object('0'),setter:nil};
add('payloads of absent fields are never interpreted',data(),ignored,'keep');
const sameValues=[];function same(name,left,right,expected){sameValues.push({name,left,right,expected});}
same('undefined equals itself',undef,undef,true);same('null differs from undefined',nil,undef,false);
same('false retains its Boolean domain',bool(false),bool(false),true);same('Boolean differs from Number',bool(true),one,false);
same('positive and negative zero differ',positiveZero,negativeZero,false);same('NaN payloads share SameValue',nanA,nanB,true);
same('equal finite Numbers match',one,one,true);same('different finite Numbers differ',one,two,false);
same('infinities retain sign',num('7ff0000000000000'),num('fff0000000000000'),false);
same('BigInt remains separate from Number',integer('1'),one,false);
same('arbitrary precision integers match exactly',integer('340282366920938463463374607431768211455'),integer('340282366920938463463374607431768211455'),true);
same('arbitrary precision integer differences remain visible',integer('340282366920938463463374607431768211455'),integer('340282366920938463463374607431768211454'),false);
same('text retains isolated UTF16 surrogates',text([55296,0,65535]),text([55296,0,65535]),true);
same('text distinguishes code units',text([55296]),text([65533]),false);same('text length is significant',text([65]),text([65,0]),false);
same('full unsigned64 object identity is exact',fn,fn,true);same('adjacent identities above 2^53 differ',object('9007199254740992'),other,false);
same('full unsigned64 symbol identity is exact',symbol('18446744073709551615'),symbol('18446744073709551615'),true);
same('symbols and objects remain distinct domains',symbol('3'),object('3'),false);
const invalidValues=[{name:'zero identity',value:object('0'),message:'an object requires a positive unsigned64 global link identity'},
 {name:'negative identity',value:symbol('-1'),message:'a symbol requires a positive unsigned64 global link identity'},
 {name:'overflow identity',value:object('18446744073709551616'),message:'an object requires a positive unsigned64 global link identity'},
 {name:'negative UTF16 unit',value:text([-1]),message:'semantic text requires complete ordered UTF16 code units'},
 {name:'overflow UTF16 unit',value:text([65536]),message:'semantic text requires complete ordered UTF16 code units'},
 {name:'fractional UTF16 unit',value:text([0.5]),message:'semantic text requires complete ordered UTF16 code units'}];
writeProjectData(new URL('parity/fixtures/javascript-property-descriptors.lino',root),{schemaVersion:1,
 model:'normalized JavaScript semantic values and ordinary property descriptor transitions 1',
 scope:'One checked global identity context; intrinsic callability; normalized descriptors. No host conversion, proxies, exotic objects or whole-heap claim.',
 versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024'],specification:'ECMAScript 2025'},
 cases,sameValues,invalidValues,textBoundaries:[{units:1048576,error:''},{units:1048577,error:'semantic text exceeds the explicit UTF16 unit budget'}]});
