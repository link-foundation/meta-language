// Fixture declaration author. Neither BigInt nor a product conversion runs here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const strings=[
 ['empty','','0'],['spaces',' \t\n','0'],['decimal','42','42'],['positive decimal','+42','42'],['negative decimal','-42','-42'],
 ['leading zeros','00042','42'],['negative zero','-0','0'],['hexadecimal','0xff','255'],['uppercase hexadecimal','0XFF','255'],
 ['binary','0b101','5'],['uppercase binary','0B101','5'],['octal','0o17','15'],['uppercase octal','0O17','15'],
 ['beyond Number precision','9007199254740993','9007199254740993'],['u64 maximum','18446744073709551615','18446744073709551615'],
 ['u64 successor','18446744073709551616','18446744073709551616'],['large negative','-18446744073709551616','-18446744073709551616'],
 ['large hexadecimal','0xffffffffffffffff','18446744073709551615'],['Unicode spaces','\u200742\u3000','42'],
 ...['+','-','0x','0b','0o','-0x1','+0b1','1.0','1e2','12n','1_000','0b2','0o8','0xg','Infinity','NaN','1 tail','\u0085','\ud800','\udc00','𝟙'].map(text=>['invalid '+JSON.stringify(text),text,null]),
];
const cases=strings.map(([name,text,value])=>({name,kind:'string',units:Array.from({length:text.length},(_,index)=>text.charCodeAt(index)),expected:value===null?{kind:'error',errorClass:'SyntaxError',prefix:'Cannot convert ',suffix:' to a BigInt'}:{kind:'value',value}}));
for(const [name,spelling,value] of [
 ['zero','0','0'],['negative zero','-0','0'],['integer','42','42'],['negative integer','-42','-42'],
 ['large exact integer','9007199254740992','9007199254740992'],['binary64 rounded input','9007199254740993','9007199254740992'],
 ['u64 binary power','18446744073709551616','18446744073709551616'],['large decimal integer','1e20','100000000000000000000'],
 ['fraction','1.5',null],['negative fraction','-1.5',null],['NaN','NaN',null],['positive infinity','Infinity',null],['negative infinity','-Infinity',null],
])cases.push({name:'Number '+name,kind:'number',spelling,expected:value===null?{kind:'error',errorClass:'RangeError',prefix:'The number ',suffix:' cannot be converted to a BigInt because it is not an integer'}:{kind:'value',value}});
writeProjectData(new URL('../../parity/fixtures/big-integer-conversion.lino',import.meta.url),{
 schemaVersion:1,model:'exact JavaScript BigInt primitive conversion 1',cases,
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025 and explicit V8 error message profile',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-stringtobigint ; https://tc39.es/ecma262/2025/multipage/numbers-and-dates.html#sec-bigint-constructor',
 boundary:'Future independent built-in BigInt and JavaScript/native library controls, exact error classes/UTF-16 messages and complete definition codecs/emissions. No observations. Source builtin lowering and general dynamic exception effects remain separate code work.',
});
