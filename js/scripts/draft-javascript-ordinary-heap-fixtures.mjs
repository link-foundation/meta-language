// Independent snapshot inputs and literal outcomes, never semantic execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const undef={$:'undefined'},nil={$:'null'},object=(identity,callable=false)=>({$:'object',identity,callable}),text=name=>({$:'text',units:Array.from({length:name.length},(_,at)=>name.charCodeAt(at))});
const one={$:'number',bits:'3ff0000000000000'},two={$:'number',bits:'4000000000000000'},nan={$:'number',bits:'7ff8000000000001'},fn=object('10',true);
const empty=()=>({configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
function proposal(fields){const result=empty();for(const [key,value]of Object.entries(fields)){result[key+'Present']=true;result[key]=value;}return result;}
const data=(value=one,writable=true,configurable=true)=>proposal({configurable,enumerable:true,value,writable});
const accessor=(getter=undef,setter=undef)=>proposal({configurable:true,enumerable:true,getter,setter});
const property=(name,descriptor)=>({key:typeof name==='string'?text(name):name,descriptor});
const root={identity:'1',prototype:nil,extensible:true,callable:false,properties:[
 property('inherited',data()),property('setter',accessor(undef,fn)),property('getter',accessor(fn)),property('frozen',data(one,false,false)),
 property('10',data()),property('2',data()),property('01',data()),property({$:'symbol',identity:'20'},data()),property('after',data()),property('NaN',data(nan,false,false)),
 property({$:'symbol',identity:'18446744073709551615'},data())]};
const heap={objects:[root,{identity:'2',prototype:object('1'),extensible:true,callable:false,properties:[property('own',data())]},
 {identity:'3',prototype:nil,extensible:true,callable:false,properties:[property('inherited',accessor(fn,fn))]},
 {identity:'4',prototype:nil,extensible:true,callable:false,properties:[]},
 {identity:'10',prototype:nil,extensible:true,callable:true,properties:[]}],symbols:['20','18446744073709551615']};
const cases=[];
function add(name,operation,fields,expected){cases.push({name,operation,heap,identity:'2',key:text('inherited'),receiver:object('2'),value:two,descriptor:empty(),prototype:nil,expected,...fields});}
add('Get follows actual prototype links','get',{}, {action:'value',value:one});
add('Get distinguishes an absent property','get',{key:text('absent')},{action:'value',value:undef});
add('Get carries the actual accessor receiver','get',{key:text('getter')},{action:'call',callee:fn,receiver:object('2'),arguments:[]});
add('Get permits primitive accessor receivers','get',{key:text('getter'),receiver:{$:'boolean',value:false}},{action:'call',callee:fn,receiver:{$:'boolean',value:false},arguments:[]});
add('Set creates a receiver own property over writable inherited data','set',{}, {action:'complete',success:true});
add('Set never updates the prototype instead of the receiver','set',{receiver:object('4')},{action:'complete',success:true});
add('an existing receiver accessor blocks a writable data assignment','set',{receiver:object('3')},{action:'complete',success:false});
add('inherited readonly data rejects assignment','set',{key:text('frozen')},{action:'complete',success:false});
add('primitive receivers reject data property creation','set',{receiver:{$:'boolean',value:false}},{action:'complete',success:false});
add('Set returns an explicit setter call and exact argument','set',{key:text('setter')},{action:'call',callee:fn,receiver:object('2'),arguments:[two]});
add('inherited setters retain primitive receivers','set',{key:text('setter'),receiver:{$:'boolean',value:false}},{action:'call',callee:fn,receiver:{$:'boolean',value:false},arguments:[two]});
add('getter-only accessors reject assignment','set',{key:text('getter')},{action:'complete',success:false});
add('DefineOwnProperty creates an independent data property','define',{descriptor:proposal({value:two})},{success:true});
add('DefineOwnProperty does not change frozen data','define',{identity:'1',key:text('frozen'),descriptor:proposal({value:two})},{success:false});
add('readonly NaN retains its stored payload','define',{identity:'1',key:text('NaN'),descriptor:proposal({value:{$:'number',bits:'7ff8000000000002'}})},{success:true});
add('Delete removes only the actual own property','delete',{key:text('own')},{success:true});
add('Delete never removes inherited properties','delete',{}, {success:true});
add('Delete refuses nonconfigurable properties','delete',{identity:'1',key:text('frozen')},{success:false});
add('PreventExtensions changes only the selected receiver','prevent',{}, {success:true});
add('prototype changes preserve property storage','prototype',{prototype:object('4')},{success:true});
add('ordinary prototype cycles reject without mutation','prototype',{identity:'1',prototype:object('2')},{success:false});
add('unchanged prototypes succeed even on nonextensible objects','prototype',{
 heap:{objects:heap.objects.map(entry=>entry.identity==='2'?{...entry,extensible:false}:entry),symbols:heap.symbols},prototype:object('1')},{success:true});
add('different prototypes fail on nonextensible objects','prototype',{
 heap:{objects:heap.objects.map(entry=>entry.identity==='2'?{...entry,extensible:false}:entry),symbols:heap.symbols},prototype:object('4')},{success:false});
add('own keys use numeric order then creation order then Symbols','keys',{identity:'1'},{keys:[text('2'),text('10'),text('inherited'),text('setter'),text('getter'),text('frozen'),text('01'),text('after'),text('NaN'),{$:'symbol',identity:'20'},{$:'symbol',identity:'18446744073709551615'}]});
add('object insertion uses an externally allocated global identity','insert',{identity:'5',prototype:object('1')},{success:true});
add('Symbols cannot collide with an Object identity','symbol',{identity:'1'},{success:false,error:'a fresh Symbol requires its distinct externally allocated unsigned64 global identity'});
add('new Symbol registration retains full unsigned64 IDs','symbol',{identity:'9007199254740993'},{success:true});
const invalid=[
 {name:'duplicate identity directory',heap:{objects:[root,root],symbols:heap.symbols},error:'ordinary Objects need strictly ordered unique positive unsigned64 identities'},
 {name:'Symbol and Object collision',heap:{objects:heap.objects,symbols:['1']},error:'Symbols and Objects need distinct strictly ordered positive unsigned64 identities'},
 {name:'prototype cycles are not ordinary object snapshots',heap:{objects:heap.objects.map(entry=>entry.identity==='1'?{...entry,prototype:object('2')}:entry),symbols:heap.symbols},error:'ordinary prototype chains must be acyclic'},
 {name:'callability belongs to intrinsic object identity',heap:{objects:heap.objects.map(entry=>entry.identity==='2'?{...entry,prototype:object('10',false)}:entry),symbols:heap.symbols},error:'Object callability must agree with its intrinsic global identity'},
 {name:'duplicate own keys retain an explicit error',heap:{objects:[{...root,properties:[property('x',data()),property('x',data())]}],symbols:[]},error:'ordinary property keys must be unique without reordering their creation history'},
 {name:'unregistered Symbol keys cannot acquire an identity by spelling',heap:{objects:[{...root,properties:[property({$:'symbol',identity:'20'},data())]}],symbols:[]},error:'a semantic Symbol must belong to the complete global identity directory'}
];
function qualify(snapshot){return {$:'ordinaryHeap',objects:snapshot.objects.map(entry=>({$:'ordinaryObject',...entry,properties:entry.properties.map(property=>({$:'ordinaryProperty',...property}))})),symbols:snapshot.symbols};}
for(const row of cases)row.heap=qualify(row.heap);
for(const row of invalid)row.heap=qualify(row.heap);
writeProjectData(new URL('../../parity/fixtures/javascript-ordinary-heap.lino',import.meta.url),{schemaVersion:1,
 model:'ordinary JavaScript global identity heap and prototype property transitions 1',versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024'],specification:'ECMAScript 2025'},cases,invalid,
 boundary:'Ordinary normalized snapshot transitions and pending accessor calls only. Frontend shared-cell integration, call execution/resumption, proxies/exotics and lifecycle remain required.'});
