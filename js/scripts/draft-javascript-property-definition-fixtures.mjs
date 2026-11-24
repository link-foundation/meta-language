// Independent literal states/effects/results only; no definition executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),undef={$:'undefined'},nil={$:'null'},object=(identity,callable=false)=>({$:'object',identity,callable}),text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
const one={$:'number',bits:'3ff0000000000000'},two={$:'number',bits:'4000000000000000'},exoticKey={$:'symbol',identity:'18446744073709551615'},fn5=object('5',true),fn6=object('6',true),fn7=object('7',true);
const empty=()=>({configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
const data=value=>({...empty(),configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:true,value,writablePresent:true,writable:true});
const getter=value=>({...empty(),configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,getterPresent:true,getter:value,setterPresent:true,setter:undef});
const property=(name,descriptor)=>({$:'ordinaryProperty',key:typeof name==='string'?text(name):name,descriptor});
const entity=(identity,properties=[],extensible=true,callable=false)=>({$:'ordinaryObject',identity,prototype:nil,extensible,callable,properties});
const heap=(attributes=[property('value',data(one))],keys=[],target=[],extensible=true)=>({$:'ordinaryHeap',objects:[entity('1',target,extensible),entity('2',attributes),entity('3',keys),entity('5',[],true,true),entity('6',[],true,true),entity('7',[],true,true)],symbols:[exoticKey.identity]});
const update=(before,identity,fields)=>({...before,objects:before.objects.map(entry=>entry.identity===identity?{...entry,...fields}:entry)});
const stored=value=>({...empty(),configurablePresent:true,enumerablePresent:true,valuePresent:true,value,writablePresent:true});
const completed=(before,value=one,writes=['configurable','enumerable','value','writable'])=>({$:'definitionCompleted',heap:update(before,'1',{properties:[property('field',stored(value))]}),value:object('1'),writes});
const error=reason=>({$:'definitionIntrinsicError',errorClass:'TypeError',reason});
const call=(callee,value,arguments_=[],after=undefined,mutations=[])=>({callee,receiver:object('3'),arguments:arguments_,completion:{$:'normalCompletion',value},...(after?{heap:after}:{}),mutations});
const cases=[];
function add(name,initial,expected,calls=[],keyInput=text('field'),attributes=object('2'),target=object('1')){cases.push({name,heap:initial,target,keyInput,attributes,exoticKey,expected,calls});}
const ordinary=heap();add('ordinary definition returns the exact original target and complete heap',ordinary,completed(ordinary));
const primitiveTarget=heap([], [property(exoticKey,data(fn6))]);
add('a primitive target rejects before key or descriptor conversion',primitiveTarget,error('Object.defineProperty requires an Object target'),[],object('3'),nil,nil);
const keyOnly=heap([], [property(exoticKey,data(fn6))]);
add('key conversion precedes rejection of primitive descriptor input',keyOnly,error('a property descriptor source must be an Object'),[call(fn6,text('field'),[text('string')])],object('3'),nil);
const mutationInitial=heap([property('value',data(one))],[property(exoticKey,getter(fn5))]);
const mutationAfter=update(mutationInitial,'2',{properties:[property('value',data(two))]});
add('key method effects change descriptor fields before they are read',mutationInitial,completed(mutationAfter,two),[
 call(fn5,fn6),call(fn6,text('field'),[text('string')],mutationAfter,[{kind:'define',identity:'2',key:text('value'),descriptor:data(two)}])],object('3'));
const ordered=heap([property('value',getter(fn7))],[property(exoticKey,getter(fn5))]);
add('key getter method and descriptor getter keep their exact order',ordered,completed(ordered,two),[call(fn5,fn6),call(fn6,text('field'),[text('string')]),{...call(fn7,two),receiver:object('2')}],object('3'));
const beforePrevent=heap([property('value',getter(fn5))]),afterPrevent=update(beforePrevent,'1',{extensible:false});
add('descriptor effects can make the later target definition reject',beforePrevent,error('ordinary DefineOwnProperty rejected the requested descriptor'),[{...call(fn5,one,[],afterPrevent,[{kind:'prevent',identity:'1'}]),receiver:object('2')}]);
const sealed=heap(undefined,[],[],false);add('a nonextensible target rejects after descriptor conversion',sealed,error('ordinary DefineOwnProperty rejected the requested descriptor'));
const same=heap(undefined,[],[property('field',stored(one))]);add('a readonly SameValue proposal does not rewrite storage',same,{$:'definitionCompleted',heap:same,value:object('1'),writes:[]});
const abrupt=heap([property('value',getter(fn5))]);add('an abrupt descriptor getter preserves its exact payload',abrupt,{$:'definitionThrow',value:text('abrupt\uD800')},[{...call(fn5,undef),receiver:object('2'),completion:{$:'throwCompletion',value:text('abrupt\uD800')}}]);
const conversion={$:'primitiveConversion',receiver:object('3'),exoticKey,hint:'number',phase:'exoticGet'};
const invalid=[{name:'definition key continuation retains the string hint',state:{$:'propertyDefinitionKey',target:object('1'),attributes:object('2'),conversion},reason:'a property key continuation retains its actual string hint'},
 {name:'definition resumption retains its Object target',operation:'resume',state:{$:'propertyDefinitionKey',target:nil,attributes:object('2'),conversion:{...conversion,hint:'string',phase:'exoticCall'}},completion:{$:'normalCompletion',value:text('field')},reason:'a property definition continuation retains its actual Object target'}];
writeProjectData(new URL('parity/fixtures/javascript-property-definition.lino',root),{schemaVersion:1,model:'ordinary JavaScript resumable Object.defineProperty 1',versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024'],specification:'ECMAScript 2025'},cases,invalid,invalidHeap:ordinary,
 boundary:'Ordinary target/key/descriptor/definition composition. The executor, intrinsic registry, Error realms, exotics and source frontend integration remain required. Source lint/format preparation is deferred and all existing gates remain.'});
