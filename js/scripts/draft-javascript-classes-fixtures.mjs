// Literal independent expectations and source controls only. Never invoke the
// class normalizer, parser, emitted products or represented rules here.
import {writeProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeFileSync,readFileSync,existsSync,lstatSync} from 'node:fs';
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
add('constructor literal defaults run when their argument is omitted','class ConstructorDefault { constructor(value=7){this.value=value;} read(){return this.value;} }','const object=new ConstructorDefault();return object.read();',7);
add('an explicit argument replaces the constructor literal default','class ConstructorOverride { constructor(value=7){this.value=value;} read(){return this.value;} }','const object=new ConstructorOverride(12);return object.read();',12);
add('undefined selects the constructor default','class ConstructorUndefined { constructor(value=7){this.value=value;} read(){return this.value;} }','const object=new ConstructorUndefined(undefined);return object.read();',7);
add('method literal defaults remain invocation parameters','class MethodDefault { constructor(value){this.value=value;} add(amount=3){return this.value+amount;} }','const object=new MethodDefault(10);return object.add();',13);
add('an explicit method argument replaces its literal default','class MethodOverride { constructor(value){this.value=value;} add(amount=3){return this.value+amount;} }','const object=new MethodOverride(10);return object.add(4);',14);
add('undefined selects the method default','class MethodUndefined { constructor(value){this.value=value;} add(amount=3){return this.value+amount;} }','const object=new MethodUndefined(10);return object.add(undefined);',13);
add('String defaults retain their original source value','class TextDefault { constructor(value="fallback"){this.value=value;} read(){return this.value;} }','const object=new TextDefault();return object.read();','fallback','string');
add('Boolean defaults retain their original source value','class BooleanDefault { constructor(value=true){this.value=value;} read(){return this.value;} }','const object=new BooleanDefault();return object.read();',true,'boolean');
add('signed zero defaults retain IEEE sign','class SignedZeroDefault { constructor(value=-0){this.value=value;} read(){return this.value;} }','const object=new SignedZeroDefault();return 1/object.read()===-Infinity;',true,'boolean');
add('a fresh empty array default is not shared between constructions','class FreshArrayDefault { constructor(items=[]){this.items=items;} read(){return this.items.length;} }','const first=new FreshArrayDefault();const second=new FreshArrayDefault();return first.items!==second.items;',true,'boolean');
add('a fresh empty record default is not shared between constructions','class FreshRecordDefault { constructor(value={}){this.value=value;} }','const first=new FreshRecordDefault();const second=new FreshRecordDefault();return first.value!==second.value;',true,'boolean');
add('null defaults remain distinct source absence values','class NullDefault { constructor(value=null){this.value=value;} }','const object=new NullDefault();return object.value===null;',true,'boolean');
add('BigInt literal defaults retain their exact integer domain','class IntegerDefault { constructor(value=5n){this.value=value;} read(){return String(this.value);} }','const object=new IntegerDefault();return object.read();','5','string');
add('static factory calls preserve ordinary construction','class Factory { constructor(value){this.value=value;} static create(value){return new Factory(value);} read(){return this.value;} }','const object=Factory.create(19);return object.read();',19);
add('static calls retain literal parameter defaults','class StaticDefault { static choose(value=11){return value;} }','return StaticDefault.choose();',11);
add('static and instance member owners stay distinct','class SeparateOwners { constructor(value){this.value=value;} static read(){return 20;} read(){return this.value;} }','const object=new SeparateOwners(3);return SeparateOwners.read()+object.read();',23);
add('one static factory can call another registered factory','class NestedFactories { constructor(value){this.value=value;} static first(value){return new NestedFactories(value);} static second(value){return NestedFactories.first(value+1);} read(){return this.value;} }','return NestedFactories.second(7).read();',8);
add('constructor field initialization can call a static helper','class StaticInitialization { constructor(value){this.value=StaticInitialization.adjust(value);} static adjust(value){return value+4;} read(){return this.value;} }','return new StaticInitialization(8).read();',12);
add('static calls keep their original argument evaluation order','class StaticArguments { static combine(first,second){return first*10+second;} }','let count=0;const next=()=>{count+=1;return count;};return StaticArguments.combine(next(),next())+count;',14);
add('methods keep module function owners despite constructor parameter shadowing','class ConstructorScope { constructor(readModuleValue){this.value=readModuleValue;} read(){return readModuleValue();} }','const object=new ConstructorScope(7);return object.read();',31,'number','function readModuleValue(){return 31;}');
add('method defaults keep module globals despite constructor parameter shadowing','class DefaultScope { constructor(Infinity){this.value=Infinity;} read(value=Infinity){return value>this.value;} }','const object=new DefaultScope(2);return object.read();',true,'boolean');
add('constructor locals cannot capture a method module binding','class LocalScope { constructor(initial){this.value=initial;const readLocalModuleValue=initial+1;this.value=readLocalModuleValue;} read(){return readLocalModuleValue();} }','const object=new LocalScope(7);return object.read();',31,'number','function readLocalModuleValue(){return 31;}');
add('constructor guards precede initial public fields','class GuardedInput { constructor(value){if(value<0){throw new RangeError("negative");}this.value=value;} read(){return this.value;} }','return new GuardedInput(8).read();',8);
add('constructor preparation locals retain their original scope','class PreparedInput { constructor(value){const prepared=value+2;this.value=prepared;} read(){return this.value;} }','return new PreparedInput(8).read();',10);
add('mutable constructor preparation runs once before initial fields','class MutablePreparation { constructor(value){let prepared=value;prepared+=3;this.value=prepared;} read(){return this.value;} }','return new MutablePreparation(8).read();',11);
add('constructor helper effects retain their original field values','class PreparedHelper { constructor(value){const prepared=prepareConstructorValue(value);this.value=prepared;} read(){return this.value;} }','return new PreparedHelper(8).read();',12,'number','function prepareConstructorValue(value){return value+4;}');
add('all statements of a receiver-free constructor stay in order','class ReceiverFree { constructor(value){if(value<0){throw new RangeError("negative");}const prepared=value+1;} read(){return 9;} }','return new ReceiverFree(8).read();',9);
add('constructor guards and method calls keep one initialized instance','class GuardedRemainder { constructor(value){if(value<0){throw new RangeError("negative");}this.value=value;this.increment();} increment(){this.value+=1;return this.value;} read(){return this.value;} }','return new GuardedRemainder(8).read();',9);
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
 ['dependent parameter defaults keep their actual binding environment','class DependentDefault { constructor(first=1,second=first){this.value=second;} } function compute(){return new DependentDefault().value;}'],
 ['an instance-reading method default cannot invent a receiver environment','class InstanceDefault { constructor(){this.value=1;} read(value=this.value){return value;} } function compute(){return new InstanceDefault().read();}'],
 ['effectful defaults retain their call-time environment obligation','class EffectDefault { constructor(value=observe()){this.value=value;} } function compute(){return 1;}'],
 ['a nonempty array default requires its complete parameter initialization rules','class ArrayDefault { constructor(value=[1]){this.value=value;} } function compute(){return 1;}'],
 ['static this retains the actual constructor object obligation','class StaticReceiver { static create(){return new this();} } function compute(){return 1;}'],
 ['a detached static factory remains an ordinary class object read','class DetachedStatic { static create(){return 1;} } function compute(){const create=DetachedStatic.create;return create();}'],
 ['a static method is not a constructable ordinary function','class ConstructStatic { static create(){return 1;} } function compute(){return new ConstructStatic.create();}'],
 ['an object property with a class spelling cannot acquire its static owner','class ActualOwner { static create(){return 1;} } function compute(){const object={ActualOwner:{create:()=>2}};return object.ActualOwner.create();}'],
 ['constructor parameters cannot invent free method bindings','class UnboundMethod { constructor(hidden){this.value=hidden;} read(){return hidden;} } function compute(){return new UnboundMethod(7).read();}'],
].map(([name,source])=>({name,source,kind:'unsupported'}));
const malformed=[
 {name:'a class body must close at its actual source boundary',source:'class Broken { constructor(){this.value=1;} function compute(){return 1;}'},
 {name:'a method parameter group must close before its body',source:'class Broken { method(value {return value;} } function compute(){return 1;}'},
 {name:'an escaped class keyword cannot become a valid declaration',source:'cl\\u0061ss Broken {} function compute(){return 1;}'},
 {name:'an escaped construction keyword cannot become an ordinary call',source:'class Broken {} function compute(){return n\\u0065w Broken();}'},
];
const errors=[
 {name:'a failing constructor guard retains RangeError and every message unit',source:'class RangeGuard { constructor(value){if(value<0){throw new RangeError("negative");}this.value=value;} } function compute(){return new RangeGuard(-1).value;}',errorClass:'RangeError',messageUnits:[110,101,103,97,116,105,118,101],messagePresent:true,resultType:'error'},
 {name:'a failing constructor guard retains absent message ownership',source:'class NoMessageGuard { constructor(value){if(value<0){throw TypeError();}this.value=value;} } function compute(){return new NoMessageGuard(-1).value;}',errorClass:'TypeError',messageUnits:[],messagePresent:false,resultType:'error'},
 {name:'constructor errors retain isolated UTF16 units without scalar replacement',source:'class SurrogateGuard { constructor(value){if(value<0){throw Error("\\ud800");}this.value=value;} } function compute(){return new SurrogateGuard(-1).value;}',errorClass:'Error',messageUnits:[55296],messagePresent:true,resultType:'error'},
];
const source=cases.map(row=>row.classSource).join('\n')+'\n'+cases.map(row=>row.helpers+'\nfunction '+row.entry+'(){'+row.body+'}').join('\n');
const history=['js/src/translation/javascript.js','js/src/self-translation.js','rust/src/translation/javascript.rs','rust/src/self_translation.rs','rust/src/translation/mod.rs'].map(path=>{
  const revision='1c270d56bcea41203f1beac768cd78a9185fb19b',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
  return{path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
});
const predecessorRevision='eae050126c3df59af868552f7d1c764d968c5550';
const predecessors=[['js/src/data/javascript-class-source-program.lino','javascript-class-source-initial-program.lino'],['parity/self-translation/javascript-class-source-rules-native.lino','javascript-class-source-initial-native.lino'],['parity/fixtures/javascript-closed-classes.lino','javascript-closed-classes-initial.lino']].map(([path,name])=>{
  const bytes=execFileSync('git',['show',predecessorRevision+':'+path],{maxBuffer:16*1024*1024});
  const output='parity/fixtures/history/'+name;
  // Exact original Links bytes: no graph decoder, migration, fixture replay or
  // generated-source replacement is involved in preserving the predecessor.
  const target=new URL('../../'+output,import.meta.url);
  if(existsSync(target)){if(!lstatSync(target).isFile()||lstatSync(target).isSymbolicLink()||!readFileSync(target).equals(bytes))throw new TypeError('the complete retained class predecessor changed: '+output);}
  else writeFileSync(target,bytes,{flag:'wx'});
  return{path,revision:predecessorRevision,retained:output,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
});
for(const [path,name]of [['js/src/data/javascript-class-source-program.lino','javascript-class-source-literal-static-program.lino'],['parity/self-translation/javascript-class-source-rules-native.lino','javascript-class-source-literal-static-native.lino'],['parity/fixtures/javascript-closed-classes.lino','javascript-closed-classes-literal-static.lino']]){
 const revision='90d79ea6',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:16*1024*1024}),retained='parity/fixtures/history/'+name,target=new URL('../../'+retained,import.meta.url);
 if(existsSync(target)){if(!lstatSync(target).isFile()||lstatSync(target).isSymbolicLink()||!readFileSync(target).equals(bytes))throw new TypeError('the complete retained class predecessor changed: '+retained);}
 else writeFileSync(target,bytes,{flag:'wx'});
 predecessors.push({path,revision:execFileSync('git',['rev-parse',revision],{encoding:'utf8'}).trim(),retained,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});
}
writeProjectData(new URL('../../parity/fixtures/javascript-closed-classes.lino',import.meta.url),{schemaVersion:1,source,cases,errors,refusals,malformed,history,predecessors,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 assertions:['ordinaryNewCallsPreserveConstructorArgumentsAndFieldInitializationOrder','sharedInstanceAliasesPreserveExternalAndMethodFieldWrites','methodCallsRetainTheirActualReceiverAndArgumentEvaluationOrder','freshInstancesAndEmptyClassesRetainDistinctReferenceIdentity','allOriginalDocumentationSignaturesAndSourceIntervalsRemainRepresented','literalContentsNeverBecomeGrammarDelimitersOrIdentifierOwners','bothFrontendsAndBothCodecsPreserveTheCompleteCheckedSource','originalJavaScriptAndBothGeneratedNativeProductsRetainEveryDeclaredValue','reflectivePrototypeDetachedMethodAndExternalInterfaceGapsRefuse','literalDefaultsRemainInInvocationScopesAndAllocateFreshAggregates','staticFactoryCallsPreserveActualClassMemberOwners','constructorBindingsCannotCaptureMethodModuleBindings','initialQualifiedSourceDefinitionsAndRecipesRemainExact','allRootSourceAndExpandedOutputBudgetsRemainExplicit','constructorProloguesKeepCompleteStatementBoundariesAndLocalScopes','constructorExceptionsPreserveClassUnitsAndMessagePresence'],
 boundary:'Unexecuted source/data draft. Ordinary closed base classes use shared own-field records and installed receiver-capturing callables. Literal parameter defaults retain their original source and call-time initialization, including fresh empty aggregate defaults. Direct receiver-independent static methods/factories preserve actual member owners and argument evaluation order. Dependent/effectful/instance-reading defaults, patterns, inheritance, public field declaration syntax, private fields, accessors, static state/this and method escape, instance method escape/replacement, this as a returned value, constructor replacement, arbitrary module consumers, full object/prototype descriptors, target cycle collection and complete own-codebase translation remain obligations. Strong shared receiver cycles require the target lifecycle model before arbitrary long-running workloads can claim resource parity. No normalizer/fixture/product/oracle/test/compiler was executed by this author.'});
process.stdout.write('Authored '+cases.length+' closed class source programs and '+refusals.length+' refusal controls.\n');
