// Declaration writer only. Original Node/native/codec controls run after drafting.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
const inputs = [
  ['assignment value', 'let value=2;return (value=7)+value;', '14'],
  ['right associative assignments', 'let first=0,second=0;return (first=second=5)+first+second;', '15'],
  ['compound captures before RHS', 'let value=2;const result=(value+=(value=7));return result*10+value;', '99'],
  ['postfix increment', 'let value=3;return value++*10+value;', '34'],
  ['prefix increment', 'let value=3;return ++value*10+value;', '44'],
  ['postfix decrement', 'let value=3;return value--*10+value;', '32'],
  ['prefix decrement', 'let value=3;return --value*10+value;', '22'],
  ['both update results', 'let value=2;return (value++ + ++value)*10+value;', '64'],
  ['subtraction assignment', 'let value=8;return (value-=3)*10+value;', '55'],
  ['multiplication assignment', 'let value=3;return (value*=2)*10+value;', '66'],
  ['division assignment', 'let value=12;return (value/=3)*10+value;', '44'],
  ['remainder assignment', 'let value=8;return (value%=3)*10+value;', '22'],
  ['string addition assignment', 'let value="a";const result=(value+="b");return result+value;', 'abab'],
  ['false conjunction skips RHS and write', 'let value=0,calls=0;const result=(value&&=(calls=1));return result*100+calls*10+value;', '0'],
  ['true disjunction skips RHS and write', 'let value=2,calls=0;const result=(value||=(calls=1));return result*100+calls*10+value;', '202'],
  ['true conjunction selects RHS', 'let value=2,calls=0;const result=(value&&=(calls=1));return result*100+calls*10+value;', '111'],
  ['false disjunction selects RHS', 'let value=0,calls=0;const result=(value||=(calls=1));return result*100+calls*10+value;', '111'],
  ['undefined nullish assignment', 'let value=undefined;const result=(value??=4);return (result??0)*10+(value??0);', '44'],
  ['null nullish assignment', 'let value=null;const result=(value??=4);return (result??0)*10+(value??0);', '44'],
  ['zero is present', 'let value=0,calls=0;const result=(value??=(calls=9));return result*100+calls*10+value;', '0'],
  ['false is present', 'let value=false,calls=0;const result=(value??=(calls=1)>0);return (result?10:0)+calls;', '0'],
  ['assigned shared value identity', 'let value=[1];const alias=(value=[2]);alias.push(3);return value.length;', '2'],
  ['captured expression assignment', 'let value=1;const add=()=>value+=2;const first=add();return first*100+add()*10+value;', '355'],
  ['assignment condition retains state', 'let value=0,total=0;while((value+=1)<3){total+=value;}return total*10+value;', '33'],
  ['conditional selected assignment', 'let value=1;return (true?(value=2):(value=3))+value;', '4'],
  ['lexical shadow cells remain distinct', 'let value=2;{let value=7;const result=(value+=1);if(result!==8)throw new Error("shadow");}return value;', '2'],
  ['postfix line terminator', 'let first=1,second=2;const selected=first\n++second;return selected*10+second;', '13'],
  ['postfix internal line terminator', 'let value=3;return (value\n)++*10+value;', '34'],
  ['nullish assignment statement', 'let value=undefined;value??=4;return value??0;', '4'],
  ['parenthesized target', 'let value=1;return ((value)=2)+value;', '4'],
  ['private source temporary collision', 'let ml_value=3;const ml_temporary_assignment_previous1=7;return ml_value++*100+ml_value*10+ml_temporary_assignment_previous1;', '347'],
  ['undefined update converts before postfix result', 'let value=undefined;const previous=value++;return Number.isNaN(previous)&&Number.isNaN(value);', 'true'],
  ['null update converts before postfix result', 'let value=null;const previous=value++;return (previous??7)*10+(value??8);', '1'],
  ['BigInt update', 'let value=3n;return value++*10n+value;', '34'],
  ['BigInt compound capture', 'let value=2n;const result=(value+=(value=7n));return result*10n+value;', '99'],
  ['Number bitwise assignment', 'let value=7;return (value&=3)*10+value;', '33'],
  ['Number shift assignment', 'let value=3;return (value<<=2)*10+value;', '132'],
  ['BigInt bitwise assignment', 'let value=7n;return (value^=3n)*10n+value;', '44'],
  ['BigInt shift assignment', 'let value=3n;return (value<<=2n)*10n+value;', '132'],
];
const cases=inputs.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'scalarAssignmentCase'+index,value,abort:null}));
cases.push({name:'argument evaluation order',source:'function combineAssignmentArguments(first,second,third){return first*100+second*10+third;}\nexport function compute(){let value=0;return combineAssignmentArguments(value=2,value+=3,value);}',entry:'scalarAssignmentArguments',value:'255',abort:null});
cases.push({name:'RHS abort before write result',source:'/** @returns {number} */ function failScalarAssignment(){throw new Error("assignment RHS");}\nexport function compute(){let value=3;return value+=failScalarAssignment();}',entry:'scalarAssignmentFailure',value:null,abort:'assignment RHS'});
writeProjectData(new URL('../../parity/fixtures/scalar-assignments.lino',import.meta.url),{
  schemaVersion:1,model:'ordered scalar assignment expression cells 1',sourceLanguage:'JavaScript',cases,refusals:[],
  source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
  versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript modules',Rust:'>=1.90.0 <1.100.0; edition 2024'},
  sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-assignment-operators ; https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-update-expressions ; capture the old value before a compound RHS, short circuit logical writes, return the selected assignment or prefix/postfix value.',
  boundary:'Future original Node/emitted JavaScript/native Rust values and aborts, paired definition/program graphs, both codecs and four emissions. No observations. Constant short-circuit targets, arbitrary coercions, property/index assignments, exponentiation and general assignment errors remain separate obligations.'
});
