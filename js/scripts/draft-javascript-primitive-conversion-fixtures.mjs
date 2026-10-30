// Write literal inputs, completions and expectations. Never execute coercion.
import {writeProjectData,readProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,lstatSync} from 'node:fs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),undef={$:'undefined'},nil={$:'null'},object=(identity,callable=false)=>({$:'object',identity,callable}),symbol=identity=>({$:'symbol',identity});
const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))}),number=bits=>({$:'number',bits});
const fn5=object('5',true),fn6=object('6',true),fn7=object('7',true),key=symbol('18446744073709551615');
const empty=()=>({configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
const data=value=>({...empty(),configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:true,value,writablePresent:true,writable:true});
const getter=value=>({...empty(),configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,getterPresent:true,getter:value,setterPresent:true,setter:undef});
const property=(name,descriptor)=>({$:'ordinaryProperty',key:typeof name==='string'?text(name):name,descriptor});
const entity=(identity,properties=[],prototype=nil,callable=false)=>({$:'ordinaryObject',identity,prototype,extensible:true,callable,properties});
const heap=(properties=[],prototype=nil,parents=[])=>({$:'ordinaryHeap',objects:[entity('1',properties,prototype),...parents,entity('5',[],nil,true),entity('6',[],nil,true),entity('7',[],nil,true)],symbols:['88','18446744073709551615']});
const done=value=>({$:'primitiveCompleted',value}),error=reason=>({$:'primitiveIntrinsicError',errorClass:'TypeError',reason});
const exhausted='ordinary conversion exhausted its callable primitive-producing methods',notCallable='an exotic primitive conversion method must be callable or absent';
const cases=[];
function add(name,initial,hint,expected,calls=[],input=object('1')){cases.push({name,heap:initial,hint,input,exoticKey:key,expected,calls});}
const complete=(callee,value,arguments_=[],after=undefined,mutations=[])=>({callee,receiver:object('1'),arguments:arguments_,completion:{$:'normalCompletion',value},...(after?{heap:after}:{}),mutations});
for(const [name,value]of [['undefined',undef],['null',nil],['false',{$:'boolean',value:false}],['negative zero',number('8000000000000000')],['NaN',number('7ff8000000000001')],['arbitrary BigInt',{$:'integer',value:'340282366920938463463374607431768211455'}],['isolated surrogate',text('\uD800')],['Symbol',symbol('88')]])add('direct primitive retains '+name,heap(),'default',done(value),[],value);
add('missing methods exhaust the ordinary conversion',heap(),'number',error(exhausted));
add('ordinary number conversion calls valueOf first',heap([property('valueOf',data(fn5)),property('toString',data(fn6))]),'number',done(number('4000000000000000')),[complete(fn5,number('4000000000000000'))]);
add('ordinary string conversion calls toString first',heap([property('valueOf',data(fn5)),property('toString',data(fn6))]),'string',done(text('literal')),[complete(fn6,text('literal'))]);
add('default ordinary hint uses number ordering',heap([property('valueOf',data(fn5)),property('toString',data(fn6))]),'default',done(nil),[complete(fn5,nil)]);
for(const hint of ['default','number','string'])add('exotic method receives exact '+hint+' hint',heap([property(key,data(fn5))]),hint,done(symbol('88')),[complete(fn5,symbol('88'),[text(hint)])]);
add('null exotic method falls back and ordinary noncallable methods skip',heap([property(key,data(nil)),property('valueOf',data(object('1'))),property('toString',data(fn6))]),'number',done(text('fallback')),[complete(fn6,text('fallback'))]);
add('ordinary object result continues to the second method',heap([property('valueOf',data(fn5)),property('toString',data(fn6))]),'number',done(number('8000000000000000')),[complete(fn5,object('1')),complete(fn6,number('8000000000000000'))]);
add('two ordinary object results require TypeError',heap([property('valueOf',data(fn5)),property('toString',data(fn6))]),'number',error(exhausted),[complete(fn5,object('1')),complete(fn6,object('1'))]);
add('exotic object result does not fall back',heap([property(key,data(fn5)),property('valueOf',data(fn6))]),'number',error('an exotic primitive conversion must return a primitive'),[complete(fn5,object('1'),[text('number')])]);
add('noncallable exotic value requires TypeError immediately',heap([property(key,data({$:'boolean',value:false})),property('valueOf',data(fn6))]),'number',error(notCallable));
add('noncallable exotic Object does not use its properties as callability',heap([property(key,data(object('1'))),property('valueOf',data(fn6))]),'number',error(notCallable));
add('exotic method getters precede their returned method',heap([property(key,getter(fn5))]),'string',done(text('\uD800')),[complete(fn5,fn6),complete(fn6,text('\uD800'),[text('string')])]);
const initial=heap([property('valueOf',getter(fn5)),property('toString',data(fn6))]);
const after=heap([property('valueOf',getter(fn5)),property('toString',data(fn7))]);
add('getter effects change the later ordinary method',initial,'number',done(text('latest')),[complete(fn5,undef,[],after,[{kind:'define',identity:'1',key:text('toString'),descriptor:data(fn7)}]),complete(fn7,text('latest'),[],after)]);
const inherited=heap([],object('2'),[entity('2',[property('valueOf',data(fn5))])]);
add('inherited ordinary method retains original receiver',inherited,'number',done({$:'integer',value:'9007199254740993'}),[complete(fn5,{$:'integer',value:'9007199254740993'})]);
add('inherited exotic getter and method retain the original receiver',heap([],object('2'),[entity('2',[property(key,getter(fn5))])]),'default',done({$:'boolean',value:false}),[complete(fn5,fn6),complete(fn6,{$:'boolean',value:false},[text('default')])]);
const abrupt={callee:fn5,receiver:object('1'),arguments:[],completion:{$:'throwCompletion',value:text('abrupt\uD800')},mutations:[]};
add('abrupt getter completion propagates its exact value',heap([property('valueOf',getter(fn5))]),'number',{$:'primitiveThrow',value:text('abrupt\uD800')},[abrupt]);
add('abrupt ordinary method completion propagates its exact value',heap([property('valueOf',data(fn5))]),'number',{$:'primitiveThrow',value:text('abrupt\uD800')},[abrupt]);
const base={$:'primitiveConversion',receiver:object('1'),exoticKey:key,hint:'number',phase:'exoticGet'};
const invalid=[
 {name:'unknown hint',state:{...base,hint:'numeric'},reason:'a primitive conversion requires an exact preferred-type hint'},
 {name:'unknown phase',state:{...base,phase:'invented'},reason:'a primitive conversion requires its exact method continuation'},
 {name:'primitive continuation receiver',state:{...base,receiver:nil},reason:'a primitive conversion continuation retains its actual Object receiver'},
 {name:'a spelling cannot replace the well-known Symbol',state:{...base,exoticKey:text('Symbol.toPrimitive')},reason:'the exotic method key must be the actual registered well-known Symbol'},
 {name:'an absent Symbol cannot be invented',state:{...base,exoticKey:symbol('89')},reason:'a semantic Symbol must belong to the complete global identity directory'},
 {name:'pending Get cannot be repeated',state:{...base,phase:'exoticGetter'},reason:'a pending primitive conversion call needs its actual completion'},
 {name:'resumption requires a pending call',operation:'resume',state:base,completion:{$:'normalCompletion',value:undef},reason:'primitive resumption needs its actual pending getter or method'},
 {name:'completion belongs to the latest actual heap',operation:'resume',state:{...base,phase:'firstCall'},completion:{$:'normalCompletion',value:object('8')},reason:'a semantic Object must belong to the complete global identity directory'}];
const revision=execFileSync('git',['rev-parse','4da1c1b0'],{cwd:root,encoding:'utf8'}).trim(),path='experiments/javascript-properties/heap-controls.mjs';
const bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:1024*1024}),retained='parity/fixtures/history/javascript-primitive-before-heap-controls.lino',file=new URL(retained,root);
const record={path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('retained oracle source must remain a regular file');const prior=readProjectData(file);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(name=>prior[name]!==record[name]))throw new TypeError('preceding complete oracle source differs');}else writeProjectData(file,record);
writeProjectData(new URL('parity/fixtures/javascript-primitive-conversion.lino',root),{schemaVersion:1,model:'ordinary JavaScript resumable ToPrimitive 1',
 versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024'],specification:'ECMAScript 2025'},cases,invalid,invalidHeap:heap([]),
 oraclePredecessor:{path,revision,retained,bytes:record.bytes,sha256:record.sha256},boundary:'Ordinary property Get and explicit call continuations. The executor owns actual calls, well-known Symbol authority, realms, intrinsic Error allocation and source frontend integration.'});
