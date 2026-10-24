// Declarations only: never execute source, an oracle or assertions here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['return sequence','return 1,2,3;','3'],
 ['parenthesized sequence','return (1,2)+3;','5'],
 ['discarded operands retain independent domains','return ("text",false,7);','7'],
 ['assignment sequence result','let value=0;return value=2,value+=3,value;','5'],
 ['statement assignments run in order','let value=0;value=2,value+=3,value*=4;return value;','20'],
 ['discarded arbitrary expressions remain ordered','let value=0;(value=1),value+9,(value=3);return value;','3'],
 ['if condition sequence','let value=0;if(value=2,value>1)return value;return 0;','2'],
 ['while condition sequence','let value=0,total=0;while(value+=1,value<3){total+=value;}return total*10+value;','33'],
 ['do condition sequence','let value=0;do{value++;}while(value<2,value<3);return value;','3'],
 ['for arbitrary init and update expressions','let seen=0,index=0;for((seen=seen*10+1),index=0;index<2;(seen=seen*10+2),index++){}return seen;','122'],
 ['for test sequence runs each visit','let seen=0;for(let index=0;seen+=1,index<2;index++){}return seen;','3'],
 ['computed index sequence','let calls=0;const values=[5,8];return values[(calls+=1,1)]*10+calls;','81'],
 ['template substitution sequence','let value=0;return `${value=2,value+=3,value}`;','5'],
 ['switch discriminant sequence','let value=0;switch(value=2,value){case 2:return value;default:return 0;}','2'],
 ['switch case sequence','let seen=0;switch(2){case (seen=3,1):return 0;case (seen+=4,2):return seen;default:return 0;}','7'],
 ['switch unparenthesized case sequence','switch(2){case 0,2:return 7;default:return 0;}','7'],
 ['case evaluation stays lazy after a match','let seen=0;switch(1){case (seen+=1,1):return seen;case (seen+=9,2):return seen;default:return 0;}','1'],
 ['argument commas remain separators','const values=[(1,2),3];return values.length*100+values[0]*10+values[1];','223'],
 ['declaration commas remain separators','const first=(0,2),second=3;return first*10+second;','23'],
 ['conditional branches retain assignment grammar','return (true?(1,2):3,4);','4'],
 ['short circuit skips whole sequence','let value=0;false&&(value=3,value=4);return value;','0'],
 ['selected arrow body sequence','const select=()=> (1,2);return select();','2'],
 ['nested sequence source names remain hygienic','const ml_temporary_sequence1=7;return (1,(2,3))+ml_temporary_sequence1;','10'],
 ['final undefined remains undefined','return 1,undefined;','undefined'],
 ['final null remains null','return 1,null;','null'],
 ['left operand does not force numeric coercion','return (1n,2);','2'],
 ['return automatic semicolon insertion remains active','return\n(1,2);','undefined'],
 ['void evaluates then returns undefined','let value=1;const ignored=void(value=3);return (ignored??0)*10+value;','3'],
 ['void preserves undefined absence tag','return Object.is(void null,undefined);','true'],
 ['void never resolves a shadowed undefined binding','const undefined=7;return String(void 0)+undefined;','undefined7'],
 ['nested void operations retain effects','let value=0;return (void(void(value+=1)),value);','1'],
 ['void skips ToPrimitive on an object','return String(void {value:1});','undefined'],
 ['void BigInt does not convert its operand','return Number.isNaN(Number(void 1n));','true'],
 ['void participates in lazy selection','let value=0;true||(void(value=4));return value;','0'],
 ['void consumes only its unary operand','let value=0;void value,value=2;return value;','2'],
 ['void temporary name remains hygienic','const ml_temporary_void1=7;return (void 0,ml_temporary_void1);','7'],
 ['for const initializer retains its scope','let value=0;for(const limit=3;value<limit;value++){}return value;','3'],
 ['for const initializer permits ordered bindings','let value=0;for(const first=2,second=3;value<first+second;value++){}return value;','5'],
 ['for const shadows only its own lexical scope','const limit=9;let value=0;for(const limit=2;value<limit;value++){}return limit*10+value;','92'],
 ['Number exponentiation assignment remains accepted','let value=2;return (value**=3)*10+value;','88'],
 ['BigInt exponentiation assignment remains accepted','let value=2n;return (value**=3n)*10n+value;','88'],
];
const cases=inputs.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'sequenceExpression'+index,value,abort:null}));
cases.push({name:'call argument order with parenthesized sequence',source:'function sequenceArguments(first,second){return first*10+second;} export function compute(){let value=0;return sequenceArguments((value=2,value+=1),value);}',entry:'sequenceArgumentsValue',value:'33',abort:null});
cases.push({name:'abrupt left operand prevents right evaluation',source:'/** @returns {number} */ function failSequenceLeft(){throw new Error("left sequence");} export function compute(){return failSequenceLeft(),1;}',entry:'sequenceLeftFailure',value:null,abort:'left sequence'});
cases.push({name:'void preserves abrupt operand completion',source:'/** @returns {number} */ function failVoidOperand(){throw new Error("void operand");} export function compute(){return void failVoidOperand();}',entry:'sequenceVoidFailure',value:null,abort:'void operand'});
cases.push({name:'for arbitrary call update retains effects',source:'function incrementSequenceArray(values){values.push(1);return values.length;} export function compute(){const values=[];for(let index=0;index<2;incrementSequenceArray(values),index++){}return values.length;}',entry:'sequenceForCallUpdate',value:'2',abort:null});
writeProjectData(new URL('../../parity/fixtures/sequence-expressions.lino',import.meta.url),{
 schemaVersion:1,model:'ordered JavaScript comma expression values and statement lists 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-comma-operator ; https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-void-operator ; https://tc39.es/ecma262/2025/multipage/ecmascript-language-statements-and-declarations.html#sec-for-statement',
 correctedAssertions:[{input:{operator:'**=',postfix:false},previousAssertion:'planScalarAssignmentExpression(operator,false) deepEqual []',currentAssertion:'the original input returns a nonempty topologically ordered assignment construction plan',predecessorRevision:'51eecee8',sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-assignment-operators'}],
 boundary:'Future original Node/emitted JavaScript/native values and aborts, both program/definition codecs and four emissions. No observations. General reference receivers/this, direct eval, delete/typeof unresolvable references, arbitrary throws, property state, dynamic unions and every earlier requirement remain. Comma returns a value rather than an assignable reference.',
});
