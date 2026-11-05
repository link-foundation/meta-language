// Deferred String rules and portable-body contracts; imports execute no cases.
import test from 'node:test';import assert from 'node:assert/strict';
import {executeJavaScriptStringMethodPrimitive} from '../src/translation/javascript-string-method-rules.js';
import {compileJavaScriptStringMethodFunction,readJavaScriptStringMethodNames,readJavaScriptStringStaticMethodNames} from '../src/translation/javascript-string-method-call-source.js';
import {compileJavaScriptCollectionCallDirectory} from '../src/translation/javascript-collection-call-source.js';
import {beginJavaScriptObjectObservation} from '../src/translation/javascript-object-integrity.js';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))}),number=value=>({$:'number',value}),object=identity=>({$:'object',identity,callable:false});
test('String rules preserve UTF16 units numeric bounds and swapped substring positions',()=>{
 let next=executeJavaScriptStringMethodPrimitive('read String code unit',[text('😀'),number(1)]);assert.equal(next.value.value,56832);next=executeJavaScriptStringMethodPrimitive('read String code unit',[text('ab'),number(Infinity)]);assert.ok(Number.isNaN(next.value.value));next=executeJavaScriptStringMethodPrimitive('substring String units',[text('abcd'),number(3),number(1)]);assert.deepEqual(next.value,text('bc'));next=executeJavaScriptStringMethodPrimitive('substr String units',[text('abcd'),number(-3),number(2)]);assert.deepEqual(next.value,text('bc'));
});
test('String boundary and inclusion rules observe actual offsets and empty searches',()=>{
 for(const [operation,input,needle,position,expected]of [['test String prefix at','abcd','bc',1,true],['test String suffix at','abcd','bc',3,true],['test String inclusion at','abcd','bc',2,false],['test String inclusion at','abcd','',99,true]])assert.equal(executeJavaScriptStringMethodPrimitive(operation,[text(input),text(needle),number(position)]).value.value,expected);
});
test('boxed String and RegExp observations use internal metadata rather than prototype or property spelling',()=>{
 const input={$:'ordinaryHeap',symbols:[],objects:[{$:'ordinaryObject',identity:1n,prototype:{$:'null'},extensible:true,callable:false,stringData:text('ab'),properties:[]},{$:'ordinaryObject',identity:2n,prototype:object(1n),extensible:true,callable:false,properties:[]},{$:'ordinaryObject',identity:3n,prototype:{$:'null'},extensible:true,callable:false,regularExpression:true,properties:[]}]};let next=beginJavaScriptObjectObservation(input,'String.valueOf',object(1n),object(1n));assert.deepEqual(next.value,text('ab'));next=beginJavaScriptObjectObservation(input,'String.valueOf',object(2n),object(2n));assert.equal(next.$,'objectIntegrityError');next=beginJavaScriptObjectObservation(input,'Object.regularExpressionSlot',object(3n),object(3n));assert.equal(next.value.value,true);next=beginJavaScriptObjectObservation(input,'Object.regularExpressionSlot',object(2n),object(2n));assert.equal(next.value.value,false);
});
test('every protected String method body retains complete bounded branch targets',()=>{
 for(const name of readJavaScriptStringMethodNames()){const fn=compileJavaScriptStringMethodFunction(1n,2n,'String.'+name,{$:'symbol',identity:9n},'16.0');assert.ok(fn);assert.equal(fn.receiverMode,'primitive intrinsic');for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(Number.isInteger(target)&&target>=0&&target<fn.instructions.length);assert.equal(fn.instructions.at(-1).$,'return');}assert.equal(compileJavaScriptStringMethodFunction(1n,2n,'String.includes',undefined),undefined);
});
test('larger protected catalogs require a distinct profile and legacy startsWith directory aliases remain usable',()=>{
 const directory={$:'collectionCallableDirectory',ownerBudget:128,realms:[2n],exoticKey:{$:'symbol',identity:9n},constructors:[],primitiveMethods:[{$:'primitiveMethodOwner',identity:1n,realm:2n,operation:'String.charAt'}]};assert.notEqual(compileJavaScriptCollectionCallDirectory(directory).error,'');assert.equal(compileJavaScriptCollectionCallDirectory({...directory,ownerProfile:'protected standard callable catalog 1'}).error,'');const legacy=compileJavaScriptCollectionCallDirectory({...directory,ownerBudget:16,primitiveMethods:[{$:'primitiveMethodOwner',identity:1n,realm:2n,operation:'String.startsWith'}]});assert.equal(legacy.error,'');assert.equal(legacy.program.functions[0].instructions[0].$,'primitiveMethod');
});
test('primitive String own properties preserve UTF16 length indices and ordinary prototype fallthrough',()=>{
 for(const [key,present]of [[text('length'),true],[text('0'),true],[text('01'),false],[text('2'),false],[{$:'symbol',identity:9n},false]])assert.equal(executeJavaScriptStringMethodPrimitive('test String own key',[text('😀'),key]).value.value,present);assert.deepEqual(executeJavaScriptStringMethodPrimitive('read String own key',[text('😀'),text('1')]).value,{$:'text',units:[0xde00]});assert.equal(executeJavaScriptStringMethodPrimitive('read String own key',[text('😀'),text('length')]).value.value,2);
});
test('primitive String lookup converts its key once and uses a distinct primitive getter receiver',()=>{
 const source={$:'sourceStringGet',input:{$:'sourceConstant',value:text('ab')},key:{$:'sourcePropertyKey',input:{$:'sourceConstant',value:text('field')},realm:2n},prototype:{$:'sourceConstant',value:object(1n)}},compiled=compileJavaScriptMutableCallSourceWithCaptures(source,[],[]);assert.equal(compiled.error,'');assert.equal(compiled.instructions.filter(row=>row.$==='propertyKey').length,1);const get=compiled.instructions.find(row=>row.$==='ordinaryGet');assert.ok(get);assert.notEqual(get.object,get.receiver);assert.equal(compiled.instructions.filter(row=>row.$==='primitive'&&row.operation==='read String own key').length,1);
});

test('String static scalar construction retains exact UTF16 units and code point errors',()=>{
 for(const [operation,input,expected] of [['convert character code text',65537,[1]],['convert character code text',-1,[65535]],['convert character code text',Infinity,[0]],['convert code point text',128512,[55357,56832]],['convert code point text',55296,[55296]],['convert code point text',-0,[0]]]){const result=executeJavaScriptStringMethodPrimitive(operation,[number(input)]);assert.equal(result.$,'primitiveValue');assert.deepEqual(result.value.units,expected);}
 for(const input of [NaN,Infinity,-Infinity,-1,1.5,1114112]){const result=executeJavaScriptStringMethodPrimitive('convert code point text',[number(input)]);assert.equal(result.$,'primitiveError');assert.equal(result.errorClass,'RangeError');}
});
test('String static bodies consume actual argument vectors and retain full conversion order',()=>{
 for(const name of readJavaScriptStringStaticMethodNames()){const fn=compileJavaScriptStringMethodFunction(1n,2n,'String.'+name,undefined);assert.ok(fn.instructions.some(row=>row.$==='argumentAt'));assert.ok(fn.instructions.some(row=>row.$==='primitive'&&row.operation==='coerce primitive number'));assert.equal(fn.instructions.some(row=>row.$==='argumentRest'||row.$==='arrayAllocate'),false);for(const row of fn.instructions)for(const target of row.$==='branch'?[row.yes,row.no]:row.$==='jump'?[row.target]:[])assert.ok(target>=0&&target<fn.instructions.length);}
});

test('String well formed processing preserves pairs and replaces isolated UTF16 surrogates',()=>{
 const input={$:'text',units:[55296,65,56320,55357,56832,55296]},checked=executeJavaScriptStringMethodPrimitive('test well formed String',[input]),copied=executeJavaScriptStringMethodPrimitive('copy well formed String',[input]);assert.equal(checked.value.value,false);assert.deepEqual(copied.value.units,[65533,65,65533,55357,56832,65533]);assert.equal(executeJavaScriptStringMethodPrimitive('test well formed String',[copied.value]).value.value,true);
 for(const value of [text(''),{$:'text',units:[55357,56832]},{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'plain'}])assert.deepEqual(executeJavaScriptStringMethodPrimitive('copy well formed String',[value]).value,value);
});
