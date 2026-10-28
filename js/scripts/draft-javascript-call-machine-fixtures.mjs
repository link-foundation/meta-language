// Literal executable Links programs and independent original JavaScript only.
// Never execute an instruction, function body, callback or source observation.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),undef={$:'undefined'},nil={$:'null'},object=(identity,callable=false)=>({$:'object',identity,callable}),text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
function number(value){const buffer=new ArrayBuffer(8),view=new DataView(buffer);view.setFloat64(0,value);return {$:'number',bits:view.getBigUint64(0).toString(16).padStart(16,'0')};}
const exoticKey={$:'symbol',identity:'18446744073709551615'},realm='1000';
const empty=()=>({configurablePresent:false,configurable:false,enumerablePresent:false,enumerable:false,valuePresent:false,value:undef,writablePresent:false,writable:false,getterPresent:false,getter:undef,setterPresent:false,setter:undef});
const data=value=>({...empty(),configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,valuePresent:true,value,writablePresent:true,writable:true});
const accessor=(getter,setter)=>({...empty(),configurablePresent:true,configurable:true,enumerablePresent:true,enumerable:true,getterPresent:true,getter,setterPresent:true,setter});
const property=(key,descriptor)=>({$:'ordinaryProperty',key:typeof key==='string'?text(key):key,descriptor});
const entity=(identity,properties=[],callable=false)=>({$:'ordinaryObject',identity,prototype:nil,extensible:true,callable,properties});
const heap=(objects=[])=>({$:'ordinaryHeap',objects:[...['1','2','3'].map(identity=>objects.find(object=>object.identity===identity)??entity(identity)),...['5','6','7','10','11'].map(identity=>entity(identity,[],true))],symbols:[exoticKey.identity]});
const fn=(identity,registers,instructions,parameters=0,extra={})=>({$:'callFunction',identity,realm,receiverMode:'strict',lexicalReceiver:undef,parameters,registers,captures:[],instructions,...extra});
const constant=(destination,value)=>({$:'constant',destination,value}),ret=source=>({$:'return',source});
const invoke=(destination,callee,receiver,arguments_=[])=>({$:'call',destination,callee,receiver,arguments:arguments_});
const programs=[];
function add(name,initial,main,other,source,expected,callbacks=[],receiver=undef,arguments_=[]){programs.push({name,heap:initial,program:{$:'callProgram',functions:[main,...other].sort((a,b)=>BigInt(a.identity)<BigInt(b.identity)?-1:1),realms:[realm],exoticKey},entry:'10',receiver,arguments:arguments_,source,expected,callbacks});}
add('constant copy and exact negative zero return',heap(),fn('10',3,[constant(1,{$:'number',bits:'8000000000000000'}),{$:'copy',destination:2,source:1},ret(2)]),[],"ctx=>-0",{kind:'value',value:{$:'number',bits:'8000000000000000'}});
add('extra arguments remain available without inventing an arguments object',heap(),fn('10',4,[{$:'argument',destination:3,index:2},ret(3)],1),[],"(ctx,...args)=>args[2]",{kind:'value',value:text('\uD800')},[],undef,[number(1),number(2),text('\uD800')]);
add('missing formal arguments bind actual undefined',heap(),fn('10',3,[ret(2)],2),[],"(ctx,...args)=>args[1]",{kind:'value',value:undef},[],undef,[number(1)]);
add('unhandled throws retain their exact text units',heap(),fn('10',2,[constant(1,text('unhandled\uD800')),{$:'throw',source:1}]),[],"ctx=>{throw 'unhandled\\uD800';}",{kind:'throw',value:text('unhandled\uD800')});
add('SameValue preserves the NaN comparison domain',heap(),fn('10',4,[constant(1,{$:'number',bits:'7ff8000000000001'}),constant(2,{$:'number',bits:'7ff8000000000002'}),{$:'sameValue',destination:3,left:1,right:2},ret(3)]),[],"ctx=>Object.is(NaN,NaN)",{kind:'value',value:{$:'boolean',value:true}});
const nested=[constant(1,object('11',true)),constant(2,object('1')),constant(3,number(40)),invoke(4,1,2,[3]),ret(4)];
add('registered nested strict function call returns the computed value',heap(),fn('10',5,nested),[fn('11',3,[constant(2,number(1)),{$:'numberBinary',destination:1,left:1,right:2,operator:'add'},ret(1)],1)],
 "ctx=>Reflect.apply(ctx.objects.get('11'),ctx.objects.get('1'),[40])",{kind:'value',value:number(41)},[{identity:'11',expression:"ctx=>(function(value){'use strict';return value+1;})"}]);
add('lexical receiver binding ignores a different call receiver',heap(),fn('10',4,[constant(1,object('11',true)),constant(2,object('2')),invoke(3,1,2),ret(3)]),[fn('11',1,[ret(0)],0,{receiverMode:'lexical',lexicalReceiver:object('1')})],
 "ctx=>Reflect.apply(ctx.objects.get('11'),ctx.objects.get('2'),[])",{kind:'value',value:object('1')},[{identity:'11',expression:"ctx=>Reflect.apply(function(){'use strict';return ()=>this;},ctx.objects.get('1'),[])"}]);
add('callee throws unwind into the nearest actual handler',heap(),fn('10',5,[{$:'pushHandler',destination:4,target:6},constant(1,object('11',true)),constant(2,undef),invoke(3,1,2),{$:'popHandler'},ret(3),ret(4)]),[fn('11',2,[constant(1,text('abrupt\uD800')),{$:'throw',source:1}])],
 "ctx=>{try{return Reflect.apply(ctx.objects.get('11'),undefined,[]);}catch(error){return error;}}",{kind:'value',value:text('abrupt\uD800')},[{identity:'11',expression:"ctx=>(function(){'use strict';throw 'abrupt\\uD800';})"}]);
add('normal handler removal and branch/jump targets retain execution order',heap(),fn('10',5,[constant(1,number(0)),{$:'branch',condition:1,yes:2,no:4},constant(2,number(99)),{$:'jump',target:7},{$:'pushHandler',destination:3,target:7},{$:'popHandler'},constant(2,number(7)),ret(2)]),[],"ctx=>{const condition=0;if(condition)return 99;try{}catch(error){}return 7;}",{kind:'value',value:number(7)});
const getHeap=heap([{...entity('1'),prototype:object('2')},entity('2',[property('value',accessor(object('5',true),undef))])]);
add('ordinary inherited protocol getter calls execute as registered functions',getHeap,fn('10',4,[constant(1,object('1')),constant(2,text('value')),{$:'ordinaryGet',destination:3,object:1,key:2,receiver:1},ret(3)]),[fn('5',2,[constant(1,number(41)),ret(1)])],
 "ctx=>Reflect.get(ctx.objects.get('1'),'value',ctx.objects.get('1'))",{kind:'value',value:number(41)},[{identity:'5',expression:"ctx=>(function(){'use strict';return 41;})"}]);
const setHeap=heap([entity('1',[property('value',accessor(undef,object('7',true)))])]);
add('setter calls ignore normal return values and complete only afterward',setHeap,fn('10',5,[constant(1,object('1')),constant(2,text('value')),constant(3,number(9)),{$:'ordinarySet',destination:4,object:1,key:2,value:3,receiver:1},ret(4)]),[fn('7',2,[constant(1,{$:'boolean',value:false}),ret(1)],1)],
 "ctx=>Reflect.set(ctx.objects.get('1'),'value',9,ctx.objects.get('1'))",{kind:'value',value:{$:'boolean',value:true}},[{identity:'7',expression:"ctx=>(function(value){'use strict';return false;})"}]);
const definitionHeap=heap([entity('2',[property('value',accessor(object('7',true),undef))]),entity('3',[property(exoticKey,accessor(object('5',true),undef))])]);
const stored={...empty(),configurablePresent:true,enumerablePresent:true,valuePresent:true,value:number(2),writablePresent:true};
const definitionAfter={...definitionHeap,objects:definitionHeap.objects.map(entry=>entry.identity==='1'?{...entry,properties:[property('field',stored)]}:entry)};
add('key getter method and descriptor getter execute through one definition',definitionHeap,fn('10',5,[constant(1,object('1')),constant(2,object('3')),constant(3,object('2')),{$:'propertyDefinition',destination:4,target:1,key:2,attributes:3,realm},ret(4)]),
 [fn('5',2,[constant(1,object('6',true)),ret(1)]),fn('6',2,[constant(1,text('field')),ret(1)],1),fn('7',2,[constant(1,number(2)),ret(1)])],
 "ctx=>Object.defineProperty(ctx.objects.get('1'),ctx.objects.get('3'),ctx.objects.get('2'))",{kind:'value',value:object('1'),heap:definitionAfter},
 [{identity:'5',expression:"ctx=>(function(){'use strict';return ctx.objects.get('6');})"},{identity:'6',expression:"ctx=>(function(hint){'use strict';if(hint!=='string')throw new Error('wrong hint');return 'field';})"},{identity:'7',expression:"ctx=>(function(){'use strict';return 2;})"}]);
add('intrinsic failure requests its actual source realm allocation',heap(),fn('10',4,[constant(1,number(1)),constant(2,undef),invoke(3,1,2),ret(3)]),[],"ctx=>Reflect.apply(1,undefined,[])",{kind:'allocation',realm,errorClass:'TypeError',reason:'the selected callee is not callable'});
for(const [name,operation,values,source,expected]of [
 ['primitive minimum keeps negative zero','read minimum number',[number(-0),number(0)],'ctx=>Math.min(-0,0)',number(-0)],
 ['primitive maximum keeps positive zero','read maximum number',[number(-0),number(0)],'ctx=>Math.max(-0,0)',number(0)],
 ['empty minimum retains positive infinity','read minimum number',[],'ctx=>Math.min()',number(Infinity)],
 ['empty maximum retains negative infinity','read maximum number',[],'ctx=>Math.max()',number(-Infinity)],
 ['text construction uses modulo unsigned code units','construct text units',[number(-1),number(65536),number(NaN)],'ctx=>String.fromCharCode(-1,65536,NaN)',{$:'text',units:[65535,0,0]}],
 ['primitive code point retains missing position result','read text code point',[text('a'),number(1)],"ctx=>'a'.codePointAt(1)",undef],
 ]){
 const destination=values.length+1;
 add(name,heap(),fn('10',destination+1,[...values.map((value,index)=>constant(index+1,value)),{$:'primitive',destination,operation,arguments:values.map((_,index)=>index+1)},ret(destination)]),[],source,{kind:'value',value:expected});
}
add('text search work quota retains the unadvanced instruction and actual heap',heap(),fn('10',4,[constant(1,{$:'text',units:Array(16384).fill(65)}),constant(2,{$:'text',units:Array(8192).fill(65)}),{$:'primitive',destination:3,operation:'test text inclusion',arguments:[1,2]},ret(3)]),[],"ctx=>'A'.repeat(16384).includes('A'.repeat(8192))",{kind:'resource',position:2,reason:'primitive text search exceeds its explicit comparison work quota',referenceValue:{$:'boolean',value:true}});
const integer=value=>({$:'integer',value});
for(const [name,operation,values,source,expected]of [
 ['integer division truncates negative values toward zero','divide integers',[integer('-9'),integer('2')],'ctx=>-9n/2n',{kind:'value',value:integer('-4')}],
 ['integer remainder keeps the dividend sign','read integer remainder',[integer('-9'),integer('2')],'ctx=>-9n%2n',{kind:'value',value:integer('-1')}],
 ['integer exponentiation uses exact repeated squaring','raise integer power',[integer('-3'),integer('5')],'ctx=>(-3n)**5n',{kind:'value',value:integer('-243')}],
 ['zero exponent on zero returns one','raise integer power',[integer('0'),integer('0')],'ctx=>0n**0n',{kind:'value',value:integer('1')}],
 ['zero integer divisor requests the actual executing realm','divide integers',[integer('1'),integer('0')],'ctx=>1n/0n',{kind:'allocation',realm,errorClass:'RangeError',reason:'Division by zero'}],
 ['negative integer exponent requests the actual executing realm','raise integer power',[integer('2'),integer('-1')],'ctx=>2n**-1n',{kind:'allocation',realm,errorClass:'RangeError',reason:'Exponent must be positive'}],
 ]){
 const destination=values.length+1;
 add(name,heap(),fn('10',destination+1,[...values.map((value,index)=>constant(index+1,value)),{$:'primitive',destination,operation,arguments:values.map((_,index)=>index+1)},ret(destination)]),[],source,expected);
}
const oversizedInteger='1'+'0'.repeat(1234);
add('integer input work quota retains the unadvanced state',heap(),fn('10',4,[constant(1,integer(oversizedInteger)),constant(2,integer(oversizedInteger)),{$:'primitive',destination:3,operation:'compare integers',arguments:[1,2]},ret(3)]),[],"ctx=>{const value=BigInt('1'+'0'.repeat(1234));return value<value?-1:value>value?1:0;}",{kind:'resource',position:2,reason:'integer input exceeds the selected 4096-bit work profile',referenceValue:number(0)});
add('primitive Symbol Number conversion requests actual realm allocation',heap(),fn('10',3,[constant(1,exoticKey),{$:'primitive',destination:2,operation:'convert explicit number',arguments:[1]},ret(2)]),[],"ctx=>Number(Symbol.toPrimitive)",{kind:'allocation',realm,errorClass:'TypeError',reason:'Cannot convert a Symbol value to a number'});
add('Number text work quota keeps the original instruction state',heap(),fn('10',3,[constant(1,{$:'text',units:Array(4097).fill(32)}),{$:'primitive',destination:2,operation:'convert explicit number',arguments:[1]},ret(2)]),[],"ctx=>Number(' '.repeat(4097))",{kind:'resource',position:1,reason:'Number text conversion exceeds the selected 4096-unit work profile',referenceValue:number(0)});
const base=programs[0];
const invalid=[{name:'unknown source realm',program:{...base.program,realms:[]},reason:'call functions require their actual declared source realm'},
 {name:'out of range register destination',program:{...base.program,functions:[{...base.program.functions[0],instructions:[constant(3,undef),ret(0)]}]},reason:'call instructions require actual in-range register coordinates'},
 {name:'invalid branch target',program:{...base.program,functions:[{...base.program.functions[0],instructions:[{$:'jump',target:2}]}]},reason:'call jumps require actual instruction targets'},
 {name:'unsupported sloppy binding cannot be approximated',program:{...base.program,functions:[{...base.program.functions[0],receiverMode:'sloppy'}]},reason:'call code requires explicit strict or lexical receiver binding'},
 {name:'primitive operation requires actual registration',program:{...base.program,functions:[{...base.program.functions[0],instructions:[{$:'primitive',destination:1,operation:'unknown operation',arguments:[]},ret(1)]}]},reason:'primitive instructions require their actual registered operation'},
 {name:'primitive operation preserves required operand count',program:{...base.program,functions:[{...base.program.functions[0],instructions:[{$:'primitive',destination:1,operation:'slice text',arguments:[0,0]},ret(1)]}]},reason:'primitive instructions require every actual operand coordinate'}];
writeProjectData(new URL('parity/fixtures/javascript-call-machine.lino',root),{schemaVersion:1,model:'ordinary JavaScript registered strict call execution 1',versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024']},programs,invalid,invalidHeap:base.heap,
 boundary:'Registered strict/lexical code and ordinary property protocols. Full source lowering, sloppy receiver/arguments-object semantics, allocation authority, exotics and all language coverage remain required. These literal bytecode controls are independent inputs, not claimed automatic translations of their oracle source.'});
