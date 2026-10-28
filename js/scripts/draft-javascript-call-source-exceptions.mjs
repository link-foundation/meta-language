// Write literal source graphs and independent originals; never compile fixtures.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const undef={$:'undefined'},nil={$:'null'},text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))}),object=(identity,callable=false)=>({$:'object',identity,callable});
function number(value){const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return {$:'number',bits:view.getBigUint64(0).toString(16).padStart(16,'0')};}
const constant=value=>({$:'sourceConstant',value}),binding=identity=>({$:'sourceBinding',identity}),argument=index=>({$:'sourceArgument',index}),raise=value=>({$:'sourceThrow',value}),protect=(identity,body,handler)=>({$:'sourceTry',identity,body,handler});
const programs=[];
function add(name,expression,source,value,extra={}){programs.push({name,expression,parameters:[],source,expected:{kind:'value',value},receiver:undef,arguments:[],callbacks:[],otherFunctions:[],...extra});}
add('source receiver retains its exact object identity',{$:'sourceReceiver'},"ctx=>ctx.objects.get('1')",object('1'),{receiver:object('1')});
add('source receiver retains negative zero',{$:'sourceReceiver'},'ctx=>-0',number(-0),{receiver:number(-0)});
add('source reads every extra argument without an arguments object',argument(2),'(ctx,...args)=>args[2]',text('\uD800'),{arguments:[number(1),number(2),text('\uD800')]});
add('source missing argument is undefined',argument(7),'(ctx,...args)=>args[7]',undef,{arguments:[number(1)]});
add('source throw retains the exact unhandled primitive',raise(constant(text('abrupt\uD800'))),"ctx=>{throw 'abrupt\\uD800';}",text('abrupt\uD800'),{expected:{kind:'throw',value:text('abrupt\uD800')}});
add('normal protected body never evaluates catch',protect(0,constant(number(7)),raise(constant(number(99)))),'ctx=>{try{return 7;}catch(error){throw 99;}}',number(7));
add('catch binds isolated UTF16 units',protect(0,raise(constant(text('\uD800'))),binding(0)),"ctx=>{try{throw '\\uD800';}catch(error){return error;}}",text('\uD800'));
add('catch binding is available to its complete expression',protect(1,raise(argument(0)),{$:'sourceNumberBinary',operator:'add',left:binding(1),right:constant(number(2))}),'(ctx,value)=>{try{throw value;}catch(error){return error+2;}}',number(42),{arguments:[number(40)]});
add('nested catch rethrow reaches the actual outer handler',protect(0,protect(1,raise(constant(number(4))),raise({$:'sourceNumberBinary',operator:'add',left:binding(1),right:constant(number(1))})),binding(0)),'ctx=>{try{try{throw 4;}catch(inner){throw inner+1;}}catch(outer){return outer;}}',number(5));
add('normal nested handler removal preserves the outer handler',protect(0,{$:'sourceLet',identity:1,value:protect(2,constant(number(4)),constant(number(99))),body:raise(constant(number(8)))},binding(0)),'ctx=>{try{try{}catch(inner){return 99;}throw 8;}catch(outer){return outer;}}',number(8));
add('catch recurrence starts a new protected evaluation',protect(1,{$:'sourceIf',condition:{$:'sourceNumberBinary',operator:'lessThan',left:constant(number(0)),right:binding(0)},yes:raise(binding(0)),no:constant(number(0))},{$:'sourceRecur',arguments:[{$:'sourceNumberBinary',operator:'subtract',left:binding(1),right:constant(number(1))}]}),'(ctx,value)=>{for(;;){try{if(value>0)throw value;return 0;}catch(error){value=error-1;}}}',number(0),{parameters:[0],arguments:[number(3)]});
const callback={$:'callFunction',identity:'11',realm:'1000',receiverMode:'strict',lexicalReceiver:undef,parameters:0,registers:2,captures:[],instructions:[{$:'constant',destination:1,value:text('callback\uD800')},{$:'throw',source:1}]};
add('callee throw unwinds through the compiled source catch',protect(0,{$:'sourceCall',callee:constant(object('11',true)),receiver:constant(undef),arguments:[]},binding(0)),"ctx=>{try{return Reflect.apply(ctx.objects.get('11'),undefined,[]);}catch(error){return error;}}",text('callback\uD800'),{otherFunctions:[callback],callbacks:[{identity:'11',expression:"ctx=>(function(){'use strict';throw 'callback\\uD800';})"}]});
add('catch failure cannot enter its own removed handler',protect(0,raise(constant(number(4))),raise(constant(number(9)))),'ctx=>{try{throw 4;}catch(error){throw 9;}}',number(9),{expected:{kind:'throw',value:number(9)}});
add('protected binding never leaks into its catch',protect(0,{$:'sourceLet',identity:1,value:constant(number(6)),body:raise(constant(number(5)))},binding(0)),'ctx=>{try{const local=6;throw 5;}catch(error){return error;}}',number(5));
const refusals=[
 {name:'unowned catch use in protected body',expression:protect(0,binding(0),constant(undef)),parameters:[],reason:'a source binding reference has no actual lexical owner'},
 {name:'protected local is unavailable in catch',expression:protect(0,{$:'sourceLet',identity:1,value:constant(number(1)),body:raise(constant(number(2)))},binding(1)),parameters:[],reason:'a source binding reference has no actual lexical owner'},
 {name:'catch cannot reuse an enclosing binding identity',expression:protect(0,constant(number(1)),binding(0)),parameters:[0],reason:'a catch binding owner must be unique in its lexical scope'},
 {name:'negative source argument coordinate refuses',expression:argument(-1),parameters:[],reason:'source argument reads require an actual bounded nonnegative coordinate'},
 {name:'oversized source argument coordinate refuses',expression:argument(1048576),parameters:[],reason:'source argument reads require an actual bounded nonnegative coordinate'},
 {name:'protected recurrence needs explicit handler unwinding',expression:protect(0,{$:'sourceRecur',arguments:[]},constant(undef)),parameters:[],reason:'source recurrence through an active exception handler requires explicit unwinding'}
];
writeProjectData(new URL('../../parity/fixtures/javascript-call-source-exceptions.lino',import.meta.url),{schemaVersion:1,model:'registered JavaScript source exception compilation 1',programs,refusals});
