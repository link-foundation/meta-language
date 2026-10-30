// Future controls are declarations. This author never runs their source.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const rows=[
  ['empty string','return Number("");','0'],
  ['no arguments','return Number();','0'],
  ['true primitive','return Number(true);','1'],
  ['false primitive','return Number(false);','0'],
  ['null primitive','return Number(null);','0'],
  ['undefined primitive','return Number.isNaN(Number(undefined));','true'],
  ['Number identity','return Number(3.5);','3.5'],
  ['Number negative zero identity','return 1/Number(-0);','-Infinity'],
  ['decimal negative zero','return 1/Number("-0.000e99");','-Infinity'],
  ['positive decimal zero','return 1/Number("+0.000e-99");','Infinity'],
  ['leading decimal point','return Number(".125");','0.125'],
  ['trailing decimal point','return Number("42.");','42'],
  ['decimal exponent','return Number("-1.25e+2");','-125'],
  ['leading decimal zeros','return Number("0000012.5000");','12.5'],
  ['hexadecimal uppercase','return Number("0Xff");','255'],
  ['binary uppercase','return Number("0B101");','5'],
  ['octal uppercase','return Number("0O17");','15'],
  ['unsigned hexadecimal','return Number("0x100000000000001");','72057594037927940'],
  ['unsigned binary exact tie','return Number("0b1'+'0'.repeat(52)+'1");','9007199254740992'],
  ['decimal tie to even lower','return Number("9007199254740993");','9007199254740992'],
  ['decimal tie to even upper','return Number("9007199254740995");','9007199254740996'],
  ['long exact decimal near one','return Number("1.00000000000000011102230246251565404236316680908203125");','1'],
  ['decimal above exact midpoint','return Number("1.00000000000000011102230246251565404236316680908203126");','1.0000000000000002'],
  ['minimum subnormal','return Number("5e-324");','5e-324'],
  ['below half minimum subnormal','return Number("2e-324");','0'],
  ['above half minimum subnormal','return Number("3e-324");','5e-324'],
  ['negative underflow','return 1/Number("-1e-999999999999999999999999");','-Infinity'],
  ['overflow','return Number("1e999999999999999999999999");','Infinity'],
  ['largest finite value','return Number("1.7976931348623157e308");','1.7976931348623157e+308'],
  ['positive infinity','return Number(" +Infinity ");','Infinity'],
  ['negative infinity','return Number("-Infinity");','-Infinity'],
  ['explicit BigInt exact','return Number(42n);','42'],
  ['explicit BigInt tie lower','return Number(9007199254740993n);','9007199254740992'],
  ['explicit BigInt tie upper','return Number(9007199254740995n);','9007199254740996'],
  ['explicit negative BigInt tie','return Number(-9007199254740995n);','-9007199254740996'],
  ['global finite converts string','return isFinite("12");','true'],
  ['global finite converts null','return isFinite(null);','true'],
  ['global finite no arguments','return isFinite();','false'],
  ['global NaN converts invalid text','return isNaN("invalid");','true'],
  ['global NaN converts empty text','return isNaN("");','false'],
  ['global NaN no arguments','return isNaN();','true'],
  ['Number finite stays noncoercing','return Number.isFinite("12");','false'],
  ['Number NaN stays noncoercing','return Number.isNaN("invalid");','false'],
  ['extra arguments retain effects','let value=0;const number=Number("4",value=7);return number*10+value;','47'],
  ['first argument precedes extras','let value=2;const number=Number(value++,value+=3);return number*10+value;','26'],
  ['extra arguments precede conversion','let value=2;const number=Number(value,value=8,value+=1);return number*10+value;','29'],
  ['coercing predicate retains extra effects','let value=0;const result=isFinite("4",value=7);return (result?10:0)+value;','17'],
  ['unary plus numeric text','return +"12.5";','12.5'],
  ['unary plus boolean','return +true;','1'],
  ['unary plus null','return +null;','0'],
  ['unary plus undefined','return Number.isNaN(+undefined);','true'],
  ['unary plus negative zero','return 1/+"-0";','-Infinity'],
  ['unary negation numeric text','return -"12.5";','-12.5'],
  ['unary negation false signed zero','return 1/-false;','-Infinity'],
  ['text subtraction','return "12"-"5";','7'],
  ['boolean subtraction','return true-false;','1'],
  ['undefined subtraction','return Number.isNaN(undefined-1);','true'],
  ['text multiplication','return "12"*"5";','60'],
  ['boolean multiplication','return true*7;','7'],
  ['text division','return "12"/"5";','2.4'],
  ['text remainder','return "12"%"5";','2'],
  ['boolean numeric addition','return true+true;','2'],
  ['undefined numeric addition','return Number.isNaN(undefined+1);','true'],
  ['string addition stays concatenation','return "12"+true;','12true'],
  ['Number bitwise text and boolean','return "7"&true;','1'],
  ['Number bitwise undefined','return undefined|3;','3'],
  ['Number bitwise null','return null^3;','3'],
  ['Number bitwise complement text','return ~"1";','-2'],
  ['Number left shift text','return "3"<<"2";','12'],
  ['Number right shift text','return "-12">>"2";','-3'],
  ['Number unsigned shift text','return "-1">>>"1";','2147483647'],
  ['BigInt negation keeps domain','return -3n;','-3'],
  ['BigInt bitwise complement keeps domain','return ~3n;','-4'],
  ['Math absolute numeric text','return Math.abs("-3");','3'],
  ['Math floor numeric text','return Math.floor("3.75");','3'],
  ['Math ceil boolean','return Math.ceil(true);','1'],
  ['Math trunc numeric text','return Math.trunc("-3.75");','-3'],
  ['Math round numeric text','return Math.round("-3.5");','-3'],
  ['Math sign numeric text','return Math.sign("-3.5");','-1'],
  ['Math square root numeric text','return Math.sqrt("4");','2'],
  ['Math unary no arguments','return Number.isNaN(Math.abs());','true'],
  ['Math maximum primitive conversion','return Math.max("2",true,null,4);','4'],
  ['Math minimum primitive conversion','return Math.min("2",true,null,4);','0'],
  ['Math maximum empty','return Math.max();','-Infinity'],
  ['Math minimum empty','return Math.min();','Infinity'],
  ['Math extra arguments evaluate after first capture','let value=-2;const result=Math.abs(value,value=7);return result*10+value;','27'],
  ['Math ignored BigInt argument is evaluated but not converted','return Math.abs(-2,3n);','2'],
];
const invalid=['+','-','.','1e','1e+','1e-','0x','0b','0o','+0x1','-0b1','0b2','0o8','0xg','1_000','12n','1.2.3','infinity','NaN','12 tail','1e9999999x','\u0085','\ud800','\udc00','𝟙'];
for(const text of invalid)rows.push(['invalid complete numeric text '+JSON.stringify(text),'return Number.isNaN(Number('+JSON.stringify(text)+'));','true']);
const whitespace=[9,10,11,12,13,32,160,5760,8192,8193,8194,8195,8196,8197,8198,8199,8200,8201,8202,8232,8233,8239,8287,12288,65279];
for(const unit of whitespace){
  const text=String.fromCharCode(unit);
  rows.push(['ECMAScript whitespace '+unit,'return Number('+JSON.stringify(text+'42'+text)+');','42']);
}
const cases=rows.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'stringNumberCase'+index,value,abort:null}));
cases.push({name:'optional string present and absent',source:'/** @param {string | null | undefined} value @returns {number} */ function convertOptionalNumericText(value){return Number(value);}\nexport function compute(){return convertOptionalNumericText("7")+convertOptionalNumericText(null)+(Number.isNaN(convertOptionalNumericText(undefined))?10:0);}',entry:'stringNumberOptional',value:'17',abort:null});
cases.push({name:'optional BigInt explicit conversion',source:'/** @param {bigint | null | undefined} value @returns {number} */ function convertOptionalNumericBigInt(value){return Number(value);}\nexport function compute(){return convertOptionalNumericBigInt(7n)+convertOptionalNumericBigInt(null)+(Number.isNaN(convertOptionalNumericBigInt(undefined))?10:0);}',entry:'stringNumberOptionalBigInt',value:'17',abort:null});
cases.push({name:'global finite BigInt throws after arguments',source:'export function compute(){let value=0;return isFinite(1n,value=7);}',entry:'stringNumberBigIntAbort',value:null,abort:'Cannot convert a BigInt value to a number'});
cases.push({name:'global NaN BigInt throws',source:'export function compute(){return isNaN(1n);}',entry:'stringNumberNaNBigIntAbort',value:null,abort:'Cannot convert a BigInt value to a number'});
cases.push({name:'extra argument abort before conversion',source:'/** @returns {number} */ function failNumericArgument(){throw new Error("numeric extra argument");}\nexport function compute(){return Number("7",failNumericArgument());}',entry:'stringNumberExtraAbort',value:null,abort:'numeric extra argument'});
cases.push({name:'optional text numeric arithmetic',source:'/** @param {string | null | undefined} value @returns {number} */ function subtractOptionalNumericText(value){return value-1;}\nexport function compute(){return subtractOptionalNumericText("7")+subtractOptionalNumericText(null)+(Number.isNaN(subtractOptionalNumericText(undefined))?10:0);}',entry:'stringNumberOptionalArithmetic',value:'15',abort:null});
cases.push({name:'unary plus BigInt throws',source:'export function compute(){return +1n;}',entry:'stringNumberUnaryBigIntAbort',value:null,abort:'Cannot convert a BigInt value to a number'});
cases.push({name:'Math first BigInt conversion throws',source:'export function compute(){return Math.abs(1n);}',entry:'stringNumberMathematicalBigIntAbort',value:null,abort:'Cannot convert a BigInt value to a number'});
cases.push({name:'Math floor BigInt conversion throws',source:'export function compute(){return Math.floor(1n);}',entry:'stringNumberMathematicalFloorBigIntAbort',value:null,abort:'Cannot convert a BigInt value to a number'});
cases.push({name:'Math extra abort precedes first conversion',source:'/** @returns {number} */ function failMathematicalArgument(){throw new Error("mathematical extra argument");}\nexport function compute(){return Math.abs(1n,failMathematicalArgument());}',entry:'stringNumberMathematicalExtraAbort',value:null,abort:'mathematical extra argument'});
cases.push({name:'Math maximum abort precedes first conversion',source:'/** @returns {number} */ function failMathematicalMaximumArgument(){throw new Error("mathematical maximum argument");}\nexport function compute(){return Math.max(1n,failMathematicalMaximumArgument());}',entry:'stringNumberMathematicalMaximumAbort',value:null,abort:'mathematical maximum argument'});
cases.push({name:'Math maximum converts every argument after NaN',source:'export function compute(){return Math.max(NaN,1n);}',entry:'stringNumberMathematicalNaNAbort',value:null,abort:'Cannot convert a BigInt value to a number'});
cases.push({name:'unannotated Math parameter inherits primitive calls',source:'function floorPrimitiveMathParameter(value){return Math.floor(value);}\nexport function compute(){return floorPrimitiveMathParameter("7.5");}',entry:'stringNumberInferredMathematicalText',value:'7',abort:null});
cases.push({name:'unannotated arithmetic parameter inherits primitive calls',source:'function subtractPrimitiveParameter(value){return value-1;}\nexport function compute(){return subtractPrimitiveParameter("7");}',entry:'stringNumberInferredArithmeticText',value:'6',abort:null});
cases.push({name:'unannotated variadic Math parameters keep distinct primitive owners',source:'function maximumPrimitiveParameters(first,second){return Math.max(first,second);}\nexport function compute(){return maximumPrimitiveParameters("7",true);}',entry:'stringNumberInferredMathematicalMaximum',value:'7',abort:null});
// Observe through the actual source String conversion, retaining each original
// input. Rust Display alone uses different exponent/Infinity spellings.
for(const [index,row] of cases.entries()){
  row.unwrappedSource=row.source;
  const name='numericControlValue'+index;
  row.source=row.source.replace('export function compute()', 'function '+name+'()')+'\nexport function compute(){return String('+name+'());}';
}
writeProjectData(new URL('../../parity/fixtures/string-number.lino',import.meta.url),{
 schemaVersion:1,model:'exact JavaScript StringNumericLiteral binary64 conversion 1',sourceLanguage:'JavaScript',cases,refusals:[],
 unitCases:[['empty','', '0',false],['negative zero','-0','0',true],['subnormal','5e-324','5e-324',false],['even midpoint','9007199254740993','9007199254740992',false],['odd midpoint','9007199254740995','9007199254740996',false],['negative overflow','-1e99999','-Infinity',false],['negative underflow','-1e-99999','0',true],['nonbreaking spaces','\u00a042\u00a0','42',false],['Unicode spaces','\u200742\u3000','42',false],['isolated high surrogate','\ud800','NaN',false],['isolated low surrogate','\udc00','NaN',false],['astral scalar','𝟙','NaN',false],['trailing syntax','1e999999x','NaN',false]].map(([name,text,value,negativeZero])=>({name,units:Array.from({length:text.length},(_,index)=>text.charCodeAt(index)),value,negativeZero,bytes:name.startsWith('isolated ')?null:[...new TextEncoder().encode(text)]})),
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 correctedAssertions:[
  {source:'console.log(Math.floor(1n));\n',previousDiagnostic:'Math.floor of int; it takes Numbers, and converts or rejects anything else at 23..25',expected:'semantic translation; evaluate the operand then throw TypeError before console output',sourceBasis:'https://tc39.es/ecma262/2025/multipage/numbers-and-dates.html#sec-math.floor'},
  {source:'console.log(Math.abs(1, 2));\n',previousDiagnostic:'Math.abs with 2 arguments: Math.abs takes one Number at 12..26',expected:'semantic translation; evaluate both arguments and print 1',sourceBasis:'https://tc39.es/ecma262/2025/multipage/numbers-and-dates.html#sec-math.abs'},
 ],
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025 modules',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/abstract-operations.html#sec-stringtonumber ; https://tc39.es/ecma262/2025/multipage/numbers-and-dates.html#sec-number-constructor ; https://tc39.es/ecma262/2025/multipage/global-object.html#sec-isfinite-number',
 boundary:'Declarations only. Future independent Node originals, JavaScript and native Rust values/aborts, complete definition/program graphs, both codecs and all emissions. No observations. Object/Symbol conversion, dynamic spreads and complete source error classes retain their obligations.',
});
