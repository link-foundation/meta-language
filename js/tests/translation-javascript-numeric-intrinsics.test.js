import {beginJavaScriptObjectObservation} from '../src/translation/javascript-object-integrity.js';
// Deferred numeric intrinsic source controls; no compilation occurs on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptNumericIntrinsicFunction,readJavaScriptNumberPredicateNames,readJavaScriptMathMethodNames} from '../src/translation/javascript-numeric-intrinsic-call-source.js';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
test('Number call distinguishes missing arguments from explicit undefined and uses explicit numeric conversion',()=>{
 const code=compileJavaScriptNumericIntrinsicFunction(1n,2n,'Number').instructions;assert.equal(code[0].$,'argumentCount');assert.equal(code[5].operation,'convert explicit number');assert.deepEqual(code[7],{$:'return',source:2});assert.deepEqual(code[1].value,{$:'number',value:0});
});
test('Number predicates check actual Number type without invoking object conversion',()=>{
 for(const name of readJavaScriptNumberPredicateNames()){const code=compileJavaScriptNumericIntrinsicFunction(1n,2n,'Number.'+name).instructions;assert.equal(code[1].operation,'read semantic type');assert.equal(code.some(row=>row.$==='primitive'&&(row.operation==='coerce primitive number'||row.operation==='convert explicit number')),false);assert.equal(code[7].value.value,false);}
 assert.equal(executeJavaScriptCallPrimitive('test NaN number',[{$:'number',value:NaN}]).value.value,true);assert.equal(executeJavaScriptCallPrimitive('test finite number',[{$:'number',value:Infinity}]).value.value,false);assert.equal(executeJavaScriptCallPrimitive('test finite number',[{$:'number',value:-0}]).value.value,true);
});
test('Math extrema consume every actual argument and preserve signed-zero reduction',()=>{
 for(const name of ['min','max']){const code=compileJavaScriptNumericIntrinsicFunction(1n,2n,'Math.'+name).instructions;assert.equal(code.some(row=>row.$==='argumentAt'),true);assert.equal(code.some(row=>row.$==='arrayAllocate'||row.$==='argumentRest'),false);assert.ok(code.some(row=>row.$==='primitive'&&row.operation==='coerce primitive number'));}
 assert.equal(Object.is(executeJavaScriptCallPrimitive('read minimum number',[{$:'number',value:0},{$:'number',value:-0}]).value.value,-0),true);assert.equal(Object.is(executeJavaScriptCallPrimitive('read maximum number',[{$:'number',value:-0},{$:'number',value:0}]).value.value,0),true);
});
test('numeric catalog entries retain complete branch targets and actual constructor classification',()=>{
 for(const operation of ['Number','Boolean','BigInt','BigInt.asIntN','BigInt.asUintN','BigInt.toString','Number.parseInt','Number.parseFloat','isNaN','isFinite',...readJavaScriptNumberPredicateNames().map(name=>'Number.'+name),...readJavaScriptMathMethodNames().map(name=>'Math.'+name)]){const fn=compileJavaScriptNumericIntrinsicFunction(1n,2n,operation);for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isInteger(target)&&target>=0&&target<fn.instructions.length);}
 const directory={$:'collectionCallableDirectory',realms:[2n],exoticKey:{$:'symbol',identity:9n},constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:1n,realm:2n,operation:'Number'}]};assert.notEqual(compileJavaScriptCollectionCallDirectory(directory).error,'');assert.equal(compileJavaScriptCollectionCallDirectory({...directory,primitiveMethods:[{...directory.primitiveMethods[0],constructable:true}]}).error,'');
});

test('ToIndex distinguishes truncated negative fractions from invalid integer widths',()=>{
 for(const [input,expected] of [[-0.9,0],[NaN,0],[3.9,3],[-0,0],[9007199254740991,9007199254740991]]){const result=executeJavaScriptCallPrimitive('read integer index',[{$:'number',value:input}]);assert.equal(result.$,'primitiveValue');assert.ok(Object.is(result.value.value,expected));}
 for(const input of [-1,Infinity,-Infinity,9007199254740992]){const result=executeJavaScriptCallPrimitive('read integer index',[{$:'number',value:input}]);assert.equal(result.$,'primitiveError');assert.equal(result.errorClass,'RangeError');}
});
test('integer wrapping keeps exact signed residues and explicit resource boundaries',()=>{
 for(const [operation,width,input,expected] of [['wrap signed integer',8,255n,-1n],['wrap signed integer',8,-129n,127n],['wrap unsigned integer',8,-1n,255n],['wrap unsigned integer',8,257n,1n],['wrap signed integer',0,99n,0n]]){const result=executeJavaScriptCallPrimitive(operation,[{$:'number',value:width},{$:'integer',value:input}]);assert.equal(result.$,'primitiveValue');assert.equal(result.value.value,expected);}
 const limit=executeJavaScriptCallPrimitive('wrap unsigned integer',[{$:'number',value:4097},{$:'integer',value:1n}]);assert.equal(limit.$,'primitiveResourceLimit');
 const invalid=executeJavaScriptCallPrimitive('wrap unsigned integer',[{$:'number',value:0},{$:'number',value:7}]);assert.equal(invalid.$,'primitiveInvalid');
});

test('numeric prefix parsers retain radix precision and decimal exponent rollback',()=>{
 const text=value=>({$:'text',units:Array.from(value,character=>character.charCodeAt(0))});
 for(const [input,radix,expected] of [['zz!',36,1295],['-0x10rest',NaN,-16],['9007199254740995',10,9007199254740996],['11',4294967298,3],['-0x0!',0,-0]]){const result=executeJavaScriptCallPrimitive('parse integer text prefix',[text(input),{$:'number',value:radix}]);assert.equal(result.$,'primitiveValue');assert.ok(Object.is(result.value.value,expected));}
 for(const [input,expected] of [['1e+',1],['-.5e+2tail',-50],['0x10',0],['-1e-999',-0],['5e-324!',5e-324],['-Infinitytail',-Infinity],['.e2',NaN]]){const result=executeJavaScriptCallPrimitive('parse float text prefix',[text(input)]);assert.equal(result.$,'primitiveValue');assert.ok(Object.is(result.value.value,expected));}
 assert.equal(executeJavaScriptCallPrimitive('parse float text prefix',[{$:'sourceText',text:'1'.repeat(4097)}]).$,'primitiveResourceLimit');
});
test('Math integer kernels retain exact low products and unsigned leading zeros',()=>{
 for(const [left,right,expected] of [[4294967295,4294967295,1],[2147483647,2147483647,1],[-1,5,-5],[3.9,2.9,6],[Infinity,7,0]]){const result=executeJavaScriptCallPrimitive('multiply signed integer bits',[{$:'number',value:left},{$:'number',value:right}]);assert.equal(result.$,'primitiveValue');assert.ok(Object.is(result.value.value,expected));}
 for(const [input,expected] of [[0,32],[-1,0],[1,31],[256,23],[4294967296,32],[NaN,32]])assert.equal(executeJavaScriptCallPrimitive('count leading zero bits',[{$:'number',value:input}]).value.value,expected);
});

test('primitive value extraction reads only actual own internal slots',()=>{
 const heap={$:'ordinaryHeap',symbols:[],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:false,properties:[],primitiveData:{$:'number',value:-0}},{$:'ordinaryObject',identity:2n,prototype:{$:'object',identity:1n,callable:false},extensible:true,callable:false,properties:[]}]};
 const extracted=beginJavaScriptObjectObservation(heap,'Number.valueOf',{$:'object',identity:1n,callable:false},{$:'undefined'});assert.equal(extracted.$,'objectIntegrityCompleted');assert.ok(Object.is(extracted.value.value,-0));
 for(const [operation,input] of [['Number.valueOf',{$:'object',identity:2n,callable:false}],['Boolean.valueOf',{$:'object',identity:1n,callable:false}],['BigInt.valueOf',{$:'number',value:7}]]){const result=beginJavaScriptObjectObservation(heap,operation,input,{$:'undefined'});assert.equal(result.$,'objectIntegrityError');assert.equal(result.errorClass,'TypeError');}
});
test('BigInt radix text retains signs digits truncated radix and receiver-independent range errors',()=>{
 const units=value=>Array.from(value,character=>character.charCodeAt(0));
 for(const [input,radix,expected] of [[255n,10,'255'],[-255n,16,'-ff'],[10n,2,'1010'],[1295n,36,'zz'],[0n,2,'0'],[15n,16.9,'f']]){const result=executeJavaScriptCallPrimitive('convert integer radix text',[{$:'integer',value:input},{$:'number',value:radix}]);assert.equal(result.$,'primitiveValue');assert.deepEqual(result.value.units,units(expected));}
 for(const radix of [NaN,0,1,37,Infinity,-Infinity]){const result=executeJavaScriptCallPrimitive('convert integer radix text',[{$:'integer',value:7n},{$:'number',value:radix}]);assert.equal(result.$,'primitiveError');assert.equal(result.errorClass,'RangeError');}
});

test('binary32 scalar rounding retains ties signed underflow subnormals and overflow',()=>{
 for(const [input,expected] of [[16777217,16777216],[16777219,16777220],[-16777217,-16777216],[1.337,1.3370000123977661],[-0,-0],[-5e-324,-0],[7.006492321624085e-46,0],[2.1019476964872256e-45,2.802596928649634e-45],[3.4028234663852886e38,3.4028234663852886e38],[3.4028235677973366e38,Infinity],[Infinity,Infinity],[-Infinity,-Infinity],[NaN,NaN]]){const result=executeJavaScriptCallPrimitive('round binary32 number',[{$:'number',value:input}]);assert.equal(result.$,'primitiveValue');assert.ok(Object.is(result.value.value,expected));}
});
