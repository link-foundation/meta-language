// Literal independent expectations and source controls only. Never invoke the
// class normalizer, parser, emitted products or represented rules here.
import {writeProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,classSource,body,value,resultType='number',helpers=''){
  const entry='computeClosedClass'+cases.length;
  cases.push({name,classSource,helpers,body,source:classSource+'\n'+helpers+'\nfunction compute(){'+body+'}',entry,value:String(value),abort:null,resultType});
}
add('ordinary construction retains initial own fields','class Initial { constructor(value){this.value=value;} read(){return this.value;} }','const object=new Initial(12);return object.read();',12);
add('an alias sees the same field allocation','class Aliased { constructor(value){this.value=value;} add(amount){this.value+=amount;return this.value;} }','const object=new Aliased(2);const alias=object;alias.add(5);return object.value;',7);
add('external writes and method writes use one allocation','class ExternalWrite { constructor(value){this.value=value;} add(amount){this.value+=amount;return this.value;} }','const object=new ExternalWrite(2);object.value=10;return object.add(3);',13);
add('method calls retain the original instance receiver','class Separate { constructor(value){this.value=value;} add(amount){this.value+=amount;return this.value;} }','const first=new Separate(2);const second=new Separate(20);first.add(3);return second.value-first.value;',15);
add('identity equality observes aliases','class Identity { constructor(value){this.value=value;} read(){return this.value;} }','const object=new Identity(4);const alias=object;return object===alias;',true,'boolean');
add('fresh construction preserves distinct identity','class Distinct { constructor(value){this.value=value;} read(){return this.value;} }','const left=new Distinct(4);const right=new Distinct(4);return left!==right;',true,'boolean');
add('an empty class still has fresh identity','class EmptyIdentity {}','const left=new EmptyIdentity();const right=new EmptyIdentity();return left!==right;',true,'boolean');
add('constructor remainder can call installed methods','class ConstructorCalls { constructor(value){this.value=value;this.increment();} increment(){this.value+=1;return this.value;} read(){return this.value;} }','const object=new ConstructorCalls(9);return object.read();',10);
add('one method can call another on the same receiver','class MethodCalls { constructor(value){this.value=value;} increment(){this.value+=1;return this.value;} twice(){this.increment();return this.increment();} }','const object=new MethodCalls(5);return object.twice();',7);
add('method arguments are evaluated after their actual receiver','class Evaluation { constructor(value){this.value=value;} read(amount){return this.value+amount;} }','let calls=0;const object=new Evaluation(10);const argument=()=>{calls+=1;object.value+=2;return 3;};return object.read(argument())+calls;',16);
add('class methods retain owned String values and documentation','class TextValue { /** @param {string} value */ constructor(value){this.value=value;} /** @returns {string} */ read(){return this.value;} }','const object=new TextValue("hello");return object.read();','hello','string');
add('class methods retain actual Boolean results','class TruthValue { /** @param {boolean} value */ constructor(value){this.value=value;} /** @returns {boolean} */ invert(){return !this.value;} }','const object=new TruthValue(false);return object.invert();',true,'boolean');
add('record-owned slice does not acquire array builtin semantics','class SliceOwner { constructor(value){this.value=value;} slice(){return this.value;} }','const object=new SliceOwner(21);return object.slice();',21);
add('a cursor can preserve an array and mutable position','class Cursor { constructor(tokens){this.tokens=tokens;this.index=0;} next(){const value=this.tokens[this.index]??0;this.index+=1;return value;} }','const cursor=new Cursor([4,9]);const first=cursor.next();return first+cursor.next()+cursor.index;',15);
add('literal delimiter contents do not change class group boundaries','class Delimiters { constructor(value){this.value=value;} read(){return this.value;} }','const object=new Delimiters("{ class ( ] }");return object.read();','{ class ( ] }','string');
add('source-owned private-looking names cannot capture generated receivers','class Hygienic { constructor(ml_class_receiver_0){this.value=ml_class_receiver_0;} read(){return this.value;} }','const object=new Hygienic(31);return object.read();',31);
add('constructor field initializer effects retain their source order','class InitializationOrder { constructor(first,second){this.first=first;this.second=second;} read(){return this.first*10+this.second;} }','let count=0;const next=()=>{count+=1;return count;};const object=new InitializationOrder(next(),next());return object.read()+count;',14);
add('default base constructor keeps excess argument effects','class DefaultBase { read(){return 8;} }','let count=0;const next=()=>{count+=1;return 2;};const object=new DefaultBase(next());return object.read()+count;',9);
const counter='class Refusal { constructor(value){this.value=value;} read(){return this.value;} }\n';
const refusals=[
 ['a constructor cannot be called as an ordinary function',counter+'function compute(){return Refusal(1).read();}'],
 ['a detached method cannot become a permanently bound closure',counter+'function compute(){const object=new Refusal(1);const method=object.read;return method();}'],
 ['computed method detachment retains its receiver obligation',counter+'function compute(){const object=new Refusal(1);const method=object["read"];return method();}'],
 ['renamed method destructuring cannot fabricate permanent binding',counter+'function compute(){const object=new Refusal(1);const {read:method}=object;return method();}'],
 ['shorthand method destructuring cannot fabricate permanent binding',counter+'function compute(){const object=new Refusal(1);const {read}=object;return read();}'],
 ['computed method keys cannot bypass typed receiver constraints',counter+'function compute(){const object=new Refusal(1);const key="r"+"ead";const method=object[key];return method();}'],
 ['method replacement retains ordinary callable this semantics',counter+'function compute(){const object=new Refusal(1);object.read=()=>2;return object.read();}'],
 ['own property reflection cannot observe synthetic method slots',counter+'function compute(){return Object.keys(new Refusal(1)).length;}'],
 ['serialization cannot expose the target representation',counter+'function compute(){return JSON.stringify(new Refusal(1));}'],
 ['deep equality needs actual own fields and prototypes',counter+'function compute(){const assert=1;return assert.deepEqual(new Refusal(1),new Refusal(1));}'],
 ['exported constructors need complete external class interfaces','export '+counter+'function compute(){return 1;}'],
 ['object spread cannot copy synthetic method slots',counter+'function compute(){const object=new Refusal(1);const copy={...object};return copy.value;}'],
 ['constructor return overrides remain explicit','class ReturnOverride { constructor(){return {value:1};} } function compute(){return new ReturnOverride().value;}'],
 ['reads before initial allocation cannot use a fabricated instance','class EarlyRead { constructor(){this.value=this.other;} } function compute(){return 1;}'],
 ['an instance cannot escape during initial own field creation','class EarlyEscape { constructor(){this.value=observe(this);} } function compute(){return 1;}'],
 ['inheritance retains super and prototype obligations','class Derived extends Refusal {} function compute(){return 1;}'],
 ['private state needs its complete brand model','class PrivateState { #value=1; read(){return this.#value;} } function compute(){return 1;}'],
 ['accessors cannot become ordinary own fields','class Accessors { get value(){return 1;} } function compute(){return 1;}'],
 ['static initialization retains its timing and owner','class StaticState { static value=1; } function compute(){return 1;}'],
 ['constructor effects before initial fields do not become fabricated initialization','class LateField { constructor(){observe();this.value=1;} } function compute(){return new LateField().value;}'],
 ['returning this needs an explicit recursive class interface','class RecursiveInterface { constructor(){this.value=1;} identity(){return this;} } function compute(){return 1;}'],
 ['class temporal dead zones cannot be hoisted past effects','const object=new Later(); class Later {} function compute(){return 1;}'],
 ['nested ordinary functions retain their own dynamic this','class DynamicReceiver { method(){return function(){return this.value;};} } function compute(){return 1;}'],
 ['a comma statement cannot become one field initializer','class CommaStatement { constructor(first,second){this.value=first,second;} } function compute(){return 1;}'],
 ['ordinary dollar fields cannot acquire the core tagged constructor protocol','class DollarField { constructor(value){this.$=value;} } function compute(){return new DollarField(1).$;}'],
].map(([name,source])=>({name,source,kind:'unsupported'}));
const malformed=[
 {name:'a class body must close at its actual source boundary',source:'class Broken { constructor(){this.value=1;} function compute(){return 1;}'},
 {name:'a method parameter group must close before its body',source:'class Broken { method(value {return value;} } function compute(){return 1;}'},
 {name:'an escaped class keyword cannot become a valid declaration',source:'cl\\u0061ss Broken {} function compute(){return 1;}'},
 {name:'an escaped construction keyword cannot become an ordinary call',source:'class Broken {} function compute(){return n\\u0065w Broken();}'},
];
const source=cases.map(row=>row.classSource).join('\n')+'\n'+cases.map(row=>row.helpers+'\nfunction '+row.entry+'(){'+row.body+'}').join('\n');
const history=['js/src/translation/javascript.js','js/src/self-translation.js','rust/src/translation/javascript.rs','rust/src/self_translation.rs','rust/src/translation/mod.rs'].map(path=>{
  const revision='1c270d56bcea41203f1beac768cd78a9185fb19b',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
  return{path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
});
writeProjectData(new URL('../../parity/fixtures/javascript-closed-classes.lino',import.meta.url),{schemaVersion:1,source,cases,refusals,malformed,history,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['ordinaryNewCallsPreserveConstructorArgumentsAndFieldInitializationOrder','sharedInstanceAliasesPreserveExternalAndMethodFieldWrites','methodCallsRetainTheirActualReceiverAndArgumentEvaluationOrder','freshInstancesAndEmptyClassesRetainDistinctReferenceIdentity','allOriginalDocumentationSignaturesAndSourceIntervalsRemainRepresented','literalContentsNeverBecomeGrammarDelimitersOrIdentifierOwners','bothFrontendsAndBothCodecsPreserveTheCompleteCheckedSource','originalJavaScriptAndBothGeneratedNativeProductsRetainEveryDeclaredValue','reflectivePrototypeDetachedMethodAndExternalInterfaceGapsRefuse'],
 boundary:'Unexecuted source/data draft. Ordinary closed base classes use shared own-field records and installed receiver-capturing callables. Inheritance, public field declaration syntax, private fields, accessors, static members, method escape/replacement, this as a returned value, constructor replacement, arbitrary module consumers, full object/prototype descriptors, target cycle collection and complete own-codebase translation remain obligations. Strong shared receiver cycles require the target lifecycle model before arbitrary long-running workloads can claim resource parity. No normalizer/fixture/product/oracle/test/compiler was executed by this author.'});
process.stdout.write('Authored '+cases.length+' closed class source programs and '+refusals.length+' refusal controls.\n');
