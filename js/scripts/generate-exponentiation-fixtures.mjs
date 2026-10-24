// Declarations only: no power kernel, original source or target is evaluated.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
const values=[
 ['BigInt zero to zero','return 0n**0n;','1'],['BigInt positive power','return 2n**64n;','18446744073709551616'],
 ['BigInt negative odd power','return (-3n)**3n;','-27'],['BigInt negative even power','return (-3n)**4n;','81'],
 ['BigInt right association','return 2n**3n**2n;','512'],['BigInt grouped association','return (2n**3n)**2n;','64'],
 ['BigInt compound statement','let value=3n;value**=4n;return value;','81'],
 ['BigInt assignment result','let value=3n;return (value**=4n)+value;','162'],
 ['BigInt capture old value','let value=3n;return value**(value=2n);','9'],
 ['Number right association','return 2**3**2;','512'],['Number grouped association','return (2**3)**2;','64'],
 ['Number fractional power','return 9**0.5;','3'],['Number negative exponent','return 2**-3;','0.125'],
 ['Number negative odd power','return (-3)**3;','-27'],['Number unary outside group','return -(2**2);','-4'],
 ['Number compound statement','let value=3;value**=4;return value;','81'],
 ['String ToNumeric power','return "3"**"4";','81'],['Boolean ToNumeric power','return true**7;','1'],
 ['null ToNumeric power','return null**3;','0'],['undefined to zero power','return undefined**0;','1'],
 ['NaN to zero power','return NaN**0;','1'],['NaN exponent','return 1**NaN;','NaN'],
 ['one to infinite power','return 1**Infinity;','NaN'],['negative one to infinite power','return (-1)**Infinity;','NaN'],
 ['negative fractional domain','return (-3)**0.5;','NaN'],['infinite reciprocal power','return Infinity**-2;','0'],
 ['negative zero odd power sign','return 1/((-0)**3);','-Infinity'],['negative zero even power sign','return 1/((-0)**2);','Infinity'],
 ['negative zero negative odd power','return (-0)**-3;','-Infinity'],
 ['negative infinite odd reciprocal sign','return 1/((-Infinity)**-3);','-Infinity'],
 ['positive overflow','return 2**1024;','Infinity'],['subnormal power','return 2**-1074;','5e-324'],
 ['underflow midpoint ties to even zero','return 2**-1075;','0'],
];
for(const [name,body,value] of values)cases.push({name,source:'export function compute(){'+body+'}',entry:'exponentiation'+cases.length,value,abort:null});
for(const [name,body,errorClass,message] of [
 ['negative BigInt exponent','return 2n**-1n;','RangeError','Exponent must be positive'],
 ['negative BigInt zero exponent input','return 0n**-1n;','RangeError','Exponent must be positive'],
 ['mixed left BigInt','return 2n**3;','TypeError','Cannot mix BigInt and other types, use explicit conversions'],
 ['mixed right BigInt','return 2**3n;','TypeError','Cannot mix BigInt and other types, use explicit conversions'],
])cases.push({name,source:'export function compute(){'+body+'}',entry:'exponentiation'+cases.length,value:null,abort:message,errorClass,messagePresent:true,messageUnits:Array.from(message,character=>character.charCodeAt(0))});
cases.push({name:'exponent argument error precedes arithmetic validation',source:'/** @returns {bigint} */ function failPowerArgument(){throw new TypeError("power argument");}\nexport function compute(){return 0n**failPowerArgument();}',entry:'exponentiationArgumentError',value:null,abort:'power argument',errorClass:'TypeError',messagePresent:true,messageUnits:[112,111,119,101,114,32,97,114,103,117,109,101,110,116]});
writeProjectData(new URL('../../parity/fixtures/exponentiation.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript numeric exponentiation source operations 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 syntaxCases:['-2**2','+2**2','!2**2','~2**2','typeof 2**2','2**-3**2'].map(expression=>({source:'export function compute(){return '+expression+';}',kind:'syntax'})),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-exp-operator ; https://tc39.es/ecma262/2025/multipage/ecmascript-data-types-and-values.html#sec-numeric-types-bigint-exponentiate ; https://raw.githubusercontent.com/nodejs/node/v24.0.0/deps/v8/src/common/message-template.h',
 boundary:'Future original-source values, signs, exact errors and evaluation, syntax, all program/definition codecs and four emissions. No behavior observations. Exact V8 approximation parity on general powers and formal recursion budgets remain obligations; canonical Number powers use directed integer intervals.',
});
