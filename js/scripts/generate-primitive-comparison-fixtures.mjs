// Fixture declarations only. Never obtain comparisons by executing inputs.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
const relations=[
 ['9007199254740992','9007199254740993n',-1],['9007199254740992','9007199254740992n',0],
 ['18446744073709551616','18446744073709551615n',1],['0.5','0n',1],['-0.5','0n',-1],
 ['-1.5','-1n',-1],['-1.5','-2n',1],['Infinity','999999999999999999999999999999n',1],
 ['-Infinity','-999999999999999999999999999999n',-1],['NaN','0n',2],
 ['" 0x10 "','16n',0],['"17"','16n',1],['"15"','16n',-1],['""','0n',0],
 ['"1.0"','2n',2],['"invalid"','0n',2],['"\\ud800"','0n',2],
 ['true','2n',-1],['false','0n',0],['null','1n',-1],['undefined','0n',2],
 ['"2"','"10"',1],['"2"','10',-1],['false','"1"',-1],
];
for(const [left,right,order] of relations)for(const reverse of [false,true])for(const operator of ['<','<=','>','>=']){
 const comparison=reverse&&order!==2?-order:order;
 const value=String(comparison!==2&&(operator==='<'?comparison<0:operator==='<='?comparison<=0:operator==='>'?comparison>0:comparison>=0));
 cases.push({name:(reverse?right:left)+operator+(reverse?left:right),entry:'primitiveComparison'+cases.length,
  source:'export function compute(){return '+(reverse?right:left)+operator+(reverse?left:right)+';}',value,abort:null});
}
for(const [left,right,equal] of [['1','1n',false],['1','true',false],['"1"','1',false],['null','undefined',false],['undefined','undefined',true],['null','null',true],['NaN','NaN',false],['-0','0',true]])for(const operator of ['===','!=='])cases.push({
 name:left+operator+right,entry:'primitiveEquality'+cases.length,source:'export function compute(){return '+left+operator+right+';}',value:String(operator==='==='?equal:!equal),abort:null,
});
for(const [value,expected] of [['"2"','false'],['null','true'],['undefined','false']]){
 const index=cases.length;
 cases.push({name:'nullable String keeps lexical/numeric '+value,entry:'nullablePrimitiveComparison'+index,
  source:'/** @param {string|null|undefined} left @param {string} right @returns {boolean} */ function compareOptionalText'+index+'(left,right){return left<right;}\nexport function compute(){return compareOptionalText'+index+'('+value+',"10");}',value:expected,abort:null});
}
cases.push({name:'comparison captures left before right assignment',entry:'primitiveComparisonCapture',source:'export function compute(){let value="2";return value<(value="10");}',value:'false',abort:null});
for(const [left,right,equal] of [
 ['1','1n',true],['0','false',true],['"1"','1',true],['""','0n',true],['"0x10"','16n',true],
 ['"1.0"','1n',false],['"1.0"','1',true],['null','undefined',true],['null','0',false],
 ['undefined','0',false],['null','false',false],['"false"','false',false],['"0"','false',true],
 ['true','1n',true],['NaN','NaN',false],['-0','0n',true],['"9007199254740993"','9007199254740992n',false],
 ['"\\ud800"','0n',false],['"\\ud800"','"\\ud800"',true],
])for(const operator of ['==','!='])cases.push({name:left+operator+right,entry:'loosePrimitiveEquality'+cases.length,
 source:'export function compute(){return '+left+operator+right+';}',value:String(operator==='=='?equal:!equal),abort:null});
for(const [value,expected] of [['"null"','false'],['null','true'],['undefined','true']]){
 const index=cases.length;
 cases.push({name:'nullable loose absence '+value,entry:'nullableLooseComparison'+index,
  source:'/** @param {string|null|undefined} value @returns {boolean} */ function looselyAbsentText'+index+'(value){return value==null;}\nexport function compute(){return looselyAbsentText'+index+'('+value+');}',value:expected,abort:null});
}
for(const [operator,value,expected] of [['===','undefined','true'],['!==','null','true'],['===','null','false']]){
 const index=cases.length;
 cases.push({name:'void call comparison '+operator+value,entry:'voidPrimitiveComparison'+index,
  source:'function comparisonVoidValue'+index+'(){return;}\nexport function compute(){return comparisonVoidValue'+index+'()'+operator+value+';}',value:expected,abort:null});
}
cases.push({name:'optional absence equals actual void result',entry:'optionalVoidPrimitiveComparison',source:'/** @param {string|undefined} value @returns {string|undefined} */ function comparisonOptionalVoidValue(value){return value;}\nfunction comparisonUndefinedValue(){return;}\nexport function compute(){return comparisonOptionalVoidValue(undefined)===comparisonUndefinedValue();}',value:'true',abort:null});
for(const [argumentsText,equal] of [
 ['',true],['undefined',true],['null',false],['NaN,NaN',true],['0,-0',false],['-0,-0',true],['Infinity,Infinity',true],
 ['1,1n',false],['1n,1n',true],['true,1',false],['null,undefined',false],['"\\ud800","\\ud800"',true],
 ['NaN,NaN,1n',true],
])cases.push({name:'Object.is('+argumentsText+')',entry:'primitiveSameValue'+cases.length,
 source:'export function compute(){return Object.is('+argumentsText+');}',value:String(equal),abort:null});
for(const [value,expected] of [['-0','false'],['0','true'],['NaN','false'],['null','false'],['undefined','false']]){
 const index=cases.length;
 cases.push({name:'nullable Number SameValue '+value,entry:'nullableSameValue'+index,
  source:'/** @param {number|null|undefined} value @returns {boolean} */ function sameOptionalNumber'+index+'(value){return Object.is(value,0);}'+'\nexport function compute(){return sameOptionalNumber'+index+'('+value+');}',value:expected,abort:null});
}
cases.push({name:'Object.is captures every argument before selection',entry:'sameValueCapture',source:'export function compute(){let value=0;return Object.is(value,value,value=1)&&value===1;}',value:'true',abort:null});
cases.push({name:'Object.is evaluates ignored throwing argument',entry:'sameValueExtraError',source:'function sameValueFailure(){throw new TypeError("ignored argument evaluated");}\nexport function compute(){return Object.is(0,0,sameValueFailure());}',value:null,abort:'ignored argument evaluated',errorClass:'TypeError',messagePresent:true,messageUnits:Array.from('ignored argument evaluated',character=>character.charCodeAt(0))});
for(const [argumentsText,value] of [
 ['',false],['undefined',false],['null',false],['false',false],['true',true],['0',false],['-0',false],
 ['NaN',false],['Infinity',true],['-Infinity',true],['0n',false],['-1n',true],['""',false],['"0"',true],
 ['"false"',true],['"\\ud800"',true],['[]',true],['{}',true],['{valueOf:()=>0}',true],['undefined,7n',false],
])cases.push({name:'Boolean('+argumentsText+')',entry:'sourceBooleanConversion'+cases.length,source:'export function compute(){return Boolean('+argumentsText+');}',value:String(value),abort:null});
cases.push({name:'Boolean captures first before ignored assignments',entry:'booleanArgumentCapture',source:'export function compute(){let value=0;return Boolean(value,value=1)===false&&value===1;}',value:'true',abort:null});
cases.push({name:'Boolean converts void call to false',entry:'booleanVoidValue',source:'function booleanUndefinedValue(){return;}\nexport function compute(){return Boolean(booleanUndefinedValue());}',value:'false',abort:null});
cases.push({name:'Boolean preserves ignored argument exceptions',entry:'booleanExtraError',source:'function booleanArgumentError(){throw new Error("Boolean extra");}\nexport function compute(){return Boolean(false,booleanArgumentError());}',value:null,abort:'Boolean extra',errorClass:'Error',messagePresent:true,messageUnits:Array.from('Boolean extra',character=>character.charCodeAt(0))});
cases.push({name:'actual BigInt owner reaches exact conversion',entry:'bigIntegerOwnerConversion',source:'export function compute(){return BigInt("9007199254740993")===9007199254740993n;}',value:'true',abort:null});
writeProjectData(new URL('../../parity/fixtures/primitive-comparisons.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript primitive equality and relational comparison 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-islessthan ; https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-isstrictlyequal ; https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-object.is',
 boundary:'Declared original source results, exact mixed Number/BigInt and StringIntegerLiteral order, strict, loose and SameValue equality, nullable lexical/absence branches, operand capture and full definition/program codec/emission/native controls. No observations. Object/Symbol coercion, heterogeneous dynamic unions and all inherited requirements remain obligations.',
});
