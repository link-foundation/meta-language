// Independent literal snapshots/completions; no converter or fixture executes.
import {writeProjectData,readProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,lstatSync} from 'node:fs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const undef={$:'undefined'},nil={$:'null'},bool=value=>({$:'boolean',value}),object=(identity,callable=false)=>({$:'object',identity,callable});
const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))}),num=bits=>({$:'number',bits});
const one=num('3ff0000000000000'),two=num('4000000000000000'),fn5=object('5',true),fn6=object('6',true),fn7=object('7',true);
const empty=()=>({configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
function descriptor(fields){const result=empty();for(const [key,value]of Object.entries(fields)){result[key+'Present']=true;result[key]=value;}return result;}
const data=value=>descriptor({configurable:true,enumerable:true,value,writable:true}),get=getter=>descriptor({configurable:true,enumerable:true,getter,setter:undef});
const property=(name,descriptor)=>({$:'ordinaryProperty',key:text(name),descriptor});
function heap(properties,prototype=nil,parents=[]){return {$:'ordinaryHeap',objects:[{$:'ordinaryObject',identity:'1',prototype,extensible:true,callable:false,properties},...parents,
 ...['5','6','7'].map(identity=>({$:'ordinaryObject',identity,prototype:nil,extensible:true,callable:true,properties:[]}))],symbols:['18446744073709551615']};}
function parent(identity,properties){return {$:'ordinaryObject',identity,prototype:nil,extensible:true,callable:false,properties};}
const cases=[];
function add(name,initial,expected,steps=[],receiver=object('1')){cases.push({name,heap:initial,receiver,expected,steps});}
add('an empty ordinary object yields an empty descriptor',heap([]),{$:'descriptorCompleted',descriptor:empty()});
add('normal values retain their exact descriptor presence',heap([property('enumerable',data(one)),property('configurable',data(undef)),property('value',data(num('8000000000000000'))),property('writable',data(text('')))]),
 {$:'descriptorCompleted',descriptor:descriptor({enumerable:true,configurable:false,value:num('8000000000000000'),writable:false})});
add('inherited descriptor fields are read in their actual context',heap([property('value',data(two))],object('2'),[parent('2',[property('enumerable',data(bool(true))),property('writable',data(bool(true)))])]),
 {$:'descriptorCompleted',descriptor:descriptor({enumerable:true,value:two,writable:true})});
add('callable accessor values retain their actual global context',heap([property('get',data(fn5)),property('set',data(undef))]),{$:'descriptorCompleted',descriptor:descriptor({getter:fn5,setter:undef})});
const initial=heap([property('enumerable',get(fn5)),property('value',data(one)),property('writable',data(bool(false)))]);
const after=heap([property('enumerable',get(fn5)),property('value',data(two)),property('writable',data(bool(true)))]);
add('earlier getter effects change the later value and writable reads',initial,{$:'descriptorCompleted',descriptor:descriptor({enumerable:true,value:two,writable:true})},[
 {callee:fn5,receiver:object('1'),arguments:[],completion:{$:'normalCompletion',value:one},heap:after,
 mutations:[{kind:'define',identity:'1',key:text('value'),descriptor:data(two)},{kind:'define',identity:'1',key:text('writable'),descriptor:data(bool(true))}]}]);
const inheritedInitial=heap([property('enumerable',get(fn5)),property('value',data(one))],object('2'),[parent('2',[property('writable',data(bool(false)))]),parent('3',[property('writable',data(bool(true)))])]);
const inheritedAfter={...inheritedInitial,objects:inheritedInitial.objects.map(entry=>entry.identity==='1'?{...entry,prototype:object('3')}:entry)};
add('earlier getters can change later prototype lookup',inheritedInitial,{$:'descriptorCompleted',descriptor:descriptor({enumerable:true,value:one,writable:true})},[
 {callee:fn5,receiver:object('1'),arguments:[],completion:{$:'normalCompletion',value:bool(true)},heap:inheritedAfter,mutations:[{kind:'prototype',identity:'1',prototype:object('3')}]}]);
const mixed=heap([property('value',data(one)),property('get',get(fn6)),property('set',get(fn7))]);
add('mixed kinds reject only after both accessor field Gets',mixed,{$:'descriptorIntrinsicError',errorClass:'TypeError',reason:'a descriptor cannot contain both data and accessor attributes'},[
 {callee:fn6,receiver:object('1'),arguments:[],completion:{$:'normalCompletion',value:undef},heap:mixed,mutations:[]},
 {callee:fn7,receiver:object('1'),arguments:[],completion:{$:'normalCompletion',value:undef},heap:mixed,mutations:[]}]);
add('invalid getter values reject before looking up the setter',mixed,{$:'descriptorIntrinsicError',errorClass:'TypeError',reason:'a descriptor accessor must be callable or undefined'},[
 {callee:fn6,receiver:object('1'),arguments:[],completion:{$:'normalCompletion',value:bool(false)},heap:mixed,mutations:[]}]);
add('abrupt getter values propagate exactly',initial,{$:'descriptorThrow',value:text('boom')},[
 {callee:fn5,receiver:object('1'),arguments:[],completion:{$:'throwCompletion',value:text('boom')},heap:initial,mutations:[]}]);
add('primitive descriptor sources request the actual intrinsic error',heap([]),{$:'descriptorIntrinsicError',errorClass:'TypeError',reason:'a property descriptor source must be an Object'},[],bool(false));
const booleans=[{name:'undefined',value:undef,expected:false},{name:'null',value:nil,expected:false},{name:'false',value:bool(false),expected:false},{name:'true',value:bool(true),expected:true},
 {name:'positive zero',value:num('0000000000000000'),expected:false},{name:'negative zero',value:num('8000000000000000'),expected:false},{name:'NaN',value:num('7ff8000000000001'),expected:false},{name:'one',value:one,expected:true},
 {name:'positive infinity',value:num('7ff0000000000000'),expected:true},{name:'negative infinity',value:num('fff0000000000000'),expected:true},
 {name:'zero BigInt',value:{$:'integer',value:'0'},expected:false},{name:'unbounded BigInt',value:{$:'integer',value:'340282366920938463463374607431768211455'},expected:true},
 {name:'empty text',value:text(''),expected:false},{name:'nonempty text',value:text('\0'),expected:true},{name:'isolated UTF16 text',value:text('\uD800'),expected:true},{name:'Symbol',value:{$:'symbol',identity:'18446744073709551615'},expected:true},{name:'Object',value:object('1'),expected:true}];
const state={$:'descriptorConversion',receiver:object('1'),next:0,descriptor:empty(),pending:false};
const invalid=[{name:'negative ordinal',state:{...state,next:-1},reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'negative zero ordinal',state:{...state},nextBits:'8000000000000000',reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'NaN ordinal',state:{...state},nextBits:'7ff8000000000001',reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'infinite ordinal',state:{...state},nextBits:'7ff0000000000000',reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'fractional ordinal',state:{...state,next:0.5},reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'ordinal past final field',state:{...state,next:7},reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'pending completion past final field',state:{...state,next:6,pending:true},reason:'descriptor conversion needs its exact ordered field continuation'},
 {name:'an unprocessed field cannot be fabricated',state:{...state,descriptor:descriptor({value:one})},reason:'unprocessed descriptor attributes cannot be fabricated in a continuation'},
 {name:'a pending Get cannot be executed twice',state:{...state,pending:true},reason:'a pending descriptor Get must be completed before its next step'},
 {name:'resumption requires a pending Get',operation:'resume',state,completion:{$:'normalCompletion',value:undef},reason:'descriptor resumption needs its actual pending Get'},
 {name:'new completion references need the updated global context',operation:'resume',state:{...state,pending:true},completion:{$:'normalCompletion',value:object('8')},reason:'a semantic Object must belong to the complete global identity directory'},
 {name:'completion callability remains an intrinsic identity fact',operation:'resume',state:{...state,pending:true},completion:{$:'normalCompletion',value:object('5')},reason:'Object callability must agree with its intrinsic global identity'}];
const revision=execFileSync('git',['rev-parse','f431b5ad'],{encoding:'utf8'}).trim(),path='experiments/javascript-properties/heap-controls.mjs';
const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:1024*1024}),retained='parity/fixtures/history/javascript-descriptor-before-heap-controls.lino',file=new URL('../../'+retained,import.meta.url);
const record={path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('retained oracle source must be a regular file');const prior=readProjectData(file);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('preceding complete oracle source changed');}
else writeProjectData(file,record);
writeProjectData(new URL('../../parity/fixtures/javascript-descriptor-conversion.lino',import.meta.url),{schemaVersion:1,model:'ordinary JavaScript resumable ToPropertyDescriptor 1',
 versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024'],specification:'ECMAScript 2025'},cases,booleans,invalid,invalidHeap:heap([]),
 oraclePredecessor:{path,revision,retained,bytes:record.bytes,sha256:record.sha256},boundary:'Ordinary descriptor conversion transitions and explicit call/intrinsic-error obligations; the evaluator, Error allocation and frontend source calls remain required.'});
