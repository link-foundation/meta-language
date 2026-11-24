// Source declarations only. No source or target fixture is evaluated here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
for(const [left,right,quotient,remainder] of [
 ['0','1','0','0'],['1','3','0','1'],['7','3','2','1'],['-7','3','-2','-1'],
 ['7','-3','-2','1'],['-7','-3','2','-1'],['6','3','2','0'],
 ['18446744073709551615','3','6148914691236517205','0'],
 ['-18446744073709551615','3','-6148914691236517205','0'],
 ['9007199254740993','2','4503599627370496','1'],
 ['1','18446744073709551615','0','1'],['0','0',null,null],['7','0',null,null],['-7','0',null,null],
])for(const [operator,value] of [['/',quotient],['%',remainder]]){
 const index=cases.length,name='divideSourceInput'+index;
 cases.push({name:'exact '+left+operator+right,source:'/** @param {bigint} left @param {bigint} right @returns {bigint} */ function '+name+'(left,right){return left'+operator+'right;}\nexport function compute(){return '+name+'('+left+'n,'+right+'n);}',
  entry:'bigIntegerDivision'+index,value,abort:value===null?'Division by zero':null,
  ...(value===null?{errorClass:'RangeError',messagePresent:true,messageUnits:[68,105,118,105,115,105,111,110,32,98,121,32,122,101,114,111]}:{})});
}
cases.push({name:'left value captured before divisor assignment',source:'export function compute(){let value=7n;return value/(value=2n);}',entry:'bigIntegerDivisionCapture',value:'3',abort:null});
cases.push({name:'right argument abort precedes zero division',source:'/** @returns {bigint} */ function failDivisorArgument(){throw new TypeError("divisor argument");}\nexport function compute(){return 0n/failDivisorArgument();}',entry:'bigIntegerDivisionArgumentError',value:null,abort:'divisor argument',errorClass:'TypeError',messagePresent:true,messageUnits:[100,105,118,105,115,111,114,32,97,114,103,117,109,101,110,116]});
writeProjectData(new URL('../../parity/fixtures/big-integer-division.lino',import.meta.url),{
 schemaVersion:1,model:'exact JavaScript BigInt division and remainder source lowering 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-data-types-and-values.html#sec-numeric-types-bigint-divide ; https://tc39.es/ecma262/2025/multipage/ecmascript-data-types-and-values.html#sec-numeric-types-bigint-remainder',
 boundary:'Declarations only. Future independent signed unbounded division/remainder, exact error fields and argument order, both runtimes and complete definition/program codec/emission controls. Mixed numeric errors and other operations remain requirements.',
});
