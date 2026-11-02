// Host boundary for the portable Error source machine. Source definitions are
// supplied as real declaration values, checked against the captured text and
// NEVER invoked by this adapter. Only independently supplied realm intrinsics
// and the property operations below execute in the host.
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {planJavaScriptOrdinaryGet,planJavaScriptOrdinarySet,readJavaScriptOrdinaryValueError} from './javascript-ordinary-heap.js';
import {prepareJavaScriptErrorModule,prepareJavaScriptInitializedErrorModule} from './javascript-error-module.js';
import {types as nativeTypes,isDeepStrictEqual} from 'node:util';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {validateJavaScriptErrorClassModule} from './javascript-error-classes.js';
import {readSourceLibraryErrorDeclarationWitness} from './javascript-error-closure-source.js';
import {readJavaScriptOrdinaryClassDeclarationWitness} from './javascript-ordinary-class-closure-source.js';
import {checkJavaScriptProjectDataSource} from './javascript-project-data-source.js';
import {compileJavaScriptErrorModulePlanWithCapabilities} from './javascript-error-class-source.js';
import {constructJavaScriptSourceText,readJavaScriptSemanticTextString,readJavaScriptSemanticTextLength,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptCallProgramError,readJavaScriptCallMachineError,stepJavaScriptCallMachine,extendJavaScriptCallArrayAuthority,resumeJavaScriptCallPrimitiveIteratorPrototype,resumeJavaScriptCallSymbolAllocation,resumeJavaScriptCallSymbolDescription,resumeJavaScriptCallFunctionNameSymbolDescription,resumeJavaScriptCallObjectPrimitivePrototype} from './javascript-call-machine.js';

// External data reification only: these operations receive already primitive
// metadata after portable wrapper semantics completed. They do not run a
// represented constructor, coercion, getter or method as an implementation.
const dataCarrierAuthority={object:Object,ownKeys:Reflect.ownKeys,descriptor:Reflect.getOwnPropertyDescriptor,prototype:Reflect.getPrototypeOf,setPrototype:Reflect.setPrototypeOf,extensible:Reflect.isExtensible,define:Reflect.defineProperty,prevent:Reflect.preventExtensions};
const actualSymbolAllocator=Symbol;
const primitiveValueSlots=[{kind:'number',constructor:Number,test:nativeTypes.isNumberObject,value:Reflect.getOwnPropertyDescriptor(Number.prototype,'valueOf')?.value},{kind:'boolean',constructor:Boolean,test:nativeTypes.isBooleanObject,value:Reflect.getOwnPropertyDescriptor(Boolean.prototype,'valueOf')?.value},{kind:'symbol',constructor:Symbol,test:nativeTypes.isSymbolObject,value:Reflect.getOwnPropertyDescriptor(Symbol.prototype,'valueOf')?.value},{kind:'integer',constructor:BigInt,test:nativeTypes.isBigIntObject,value:Reflect.getOwnPropertyDescriptor(BigInt.prototype,'valueOf')?.value}];
const stringValueSlot=Reflect.getOwnPropertyDescriptor(String.prototype,'valueOf')?.value;
const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))});
const symbolDescriptionGetter=Reflect.getOwnPropertyDescriptor(Symbol.prototype,'description')?.get;
const string=units=>{let result='';for(const unit of units)result+=String.fromCharCode(unit);return result;};

/** `realm` is embedding authority, not serialized source metadata. It supplies
 * actual Object.prototype, Symbol.toPrimitive and each admitted standard Error
 * constructor from ONE real realm. `definitions` contains all actual captured
 * declarations (including private helpers). Their bodies are not a fallback
 * execution path. Modified prototypes, proxies and reflection on VM ordinary
 * objects require successor host protocols and are refused here. */
export function createJavaScriptErrorRealm(capture,{realm,definitions,identity=1n,declarationWitness=null,deferModule=false}){
 const ordinaryClasses=capture.model==='actual closed source classes 1';
 const sourceFunctions=capture.model==='actual project data source functions 1'||ordinaryClasses;
 capture=sourceFunctions?checkJavaScriptProjectDataSource(capture):validateJavaScriptErrorClassModule(capture);
 // Keep declaration classifications private and copied before publishing any
 // mutable capture/registry data. Function spelling/prototype shape is no proof.
 const constructorDeclarations=capture.plan.functions.map(definition=>{
  if(!['errorOrdinaryConstructorPlan','errorConstructorPlan','errorStatementFunctionPlan','errorMutableStatementFunctionPlan','errorFunctionPlan','errorMethodFunctionPlan'].includes(definition.$))throw new TypeError('constructor classification requires its actual checked source declaration kind');
  return {source:definition.identity,constructable:definition.$!=='errorMethodFunctionPlan'};
 });
 if(sourceFunctions&&(!deferModule||!ordinaryClasses&&declarationWitness!==null))throw new TypeError('source capabilities must be bound before source entry publication');
 const declarationOwners=declarationWitness===null?null:ordinaryClasses?readJavaScriptOrdinaryClassDeclarationWitness(capture,declarationWitness):readSourceLibraryErrorDeclarationWitness(capture,declarationWitness);
 if(!realm||!realm.errors||!realm.objectPrototype||typeof realm.objectPrototype!=='object'||Reflect.getPrototypeOf(realm.objectPrototype)!==null||typeof realm.toPrimitive!=='symbol'||typeof realm.getPrimitive!=='function'||typeof realm.assign!=='function'||!definitions||typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n)throw new TypeError('actual complete Error realm authority is required');
 let next=1n,heap={$:'ordinaryHeap',objects:[],symbols:[]};
 const forward=new Map(),reverse=new Map(),intrinsics=new Map(),vmProxies=new Map(),hostOwned=new Set(),refusals=new WeakSet(),sourceNames=new Map(),capabilityOwners=new Map(),opaqueOwners=new Map(),portableFunctions=new Map();
 let portableTemplateProgram={templates:undefined},arrayConstructorAuthority=null,primitiveIteratorPrototypes=null;
 function refuse(reason){const error=new TypeError(reason);refusals.add(error);throw error;}
 const allocate=()=>{while(heap.objects.some(row=>row.identity===next)||heap.symbols.includes(next))next+=1n;if(next>18446744073709551615n)throw new RangeError('the authoritative global identity allocator exhausted unsigned64 identities');return next++;};
 const sorted=()=>{heap={...heap,objects:heap.objects.toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0),symbols:heap.symbols.toSorted((a,b)=>a<b?-1:a>b?1:0)};};
 function encode(value){
  if(value===undefined)return {$:'undefined'};if(value===null)return {$:'null'};
  if(typeof value==='string')return sourceFunctions&&value.length>1048576?constructJavaScriptSourceText(value):text(value);if(typeof value==='boolean')return {$:'boolean',value};if(typeof value==='number')return {$:'number',value};if(typeof value==='bigint')return {$:'integer',value};
  if(forward.has(value)){const id=forward.get(value);return typeof value==='symbol'?{$:'symbol',identity:id}:{$:'object',identity:id,callable:typeof value==='function'};}
  if(typeof value==='symbol'){const id=allocate();forward.set(value,id);reverse.set(id,value);heap.symbols.push(id);sorted();return {$:'symbol',identity:id};}
  if(typeof value!=='object'&&typeof value!=='function')throw new TypeError('unmodeled host semantic value');
  if(nativeTypes.isProxy(value))return refuse('capturing a Proxy requires its actual trap continuations before reflection');
  const id=allocate();forward.set(value,id);reverse.set(id,value);
  // Only the successor ordinary-source profile exposes inspectable arrays.
  // Qualified opaque loader arrays retain their distinct explicit host port.
  const row={$:ordinaryClasses&&Array.isArray(value)?'arrayObject':'ordinaryObject',identity:id,prototype:{$:'null'},extensible:Reflect.isExtensible(value),callable:typeof value==='function',properties:[],...(value===realm.objectPrototype?{immutablePrototype:true}:{})};heap.objects.push(row);sorted();
  if(nativeTypes.isRegExp(value))row.regularExpression=true;
  if(nativeTypes.isStringObject(value)){if(typeof stringValueSlot!=='function'||Reflect.getOwnPropertyDescriptor(String.prototype,'valueOf')?.value!==stringValueSlot)throw new TypeError('StringData capture retains its actual protected primitive slot reader');row.stringData=encode(Reflect.apply(stringValueSlot,value,[]));}
  for(const slot of primitiveValueSlots)if(slot.test(value)){if(typeof slot.value!=='function'||Reflect.getOwnPropertyDescriptor(slot.constructor.prototype,'valueOf')?.value!==slot.value)throw new TypeError('primitive data capture retains its actual protected slot reader');const primitive=encode(Reflect.apply(slot.value,value,[]));if(primitive.$!==slot.kind)throw new TypeError('primitive data capture retains its actual internal slot type');row.primitiveData=primitive;}
  row.prototype=encode(Reflect.getPrototypeOf(value));
  for(const key of Reflect.ownKeys(value)){
   const descriptor=Reflect.getOwnPropertyDescriptor(value,key);if(!descriptor)throw new TypeError('actual host descriptors must remain stable during capture');
   const converted=createJavaScriptPropertyDescriptor('value' in descriptor?'data':'accessor',descriptor.configurable,descriptor.enumerable);
   if('value' in descriptor){converted.value=encode(descriptor.value);converted.writable=descriptor.writable;}else{converted.getter=encode(descriptor.get);converted.setter=encode(descriptor.set);}
   row.properties.push({$:'ordinaryProperty',key:encode(key),descriptor:converted});
  }
  return {$:'object',identity:id,callable:typeof value==='function'};
 }
 function decode(value){
  if(value.$==='undefined')return undefined;if(value.$==='null')return null;if(value.$==='text')return string(value.units);if(value.$==='sourceText')return value.text;
  if(['number','integer','boolean'].includes(value.$))return value.value;
  if(value.$==='object'&&value.callable){
   if(intrinsics.has(value.identity)||capabilityOwners.has(value.identity))return reverse.get(value.identity);
   if(vmProxies.has(value.identity))return vmProxies.get(value.identity);
   const handle=function representedSourceCallable(){return refuse('an engine callback needs a portable source-call continuation; original bodies are never a host fallback');};
   if(sourceNames.has(value.identity))Object.defineProperty(handle,'name',{value:sourceNames.get(value.identity),configurable:true,enumerable:false,writable:false});
   else Object.defineProperty(handle,'name',{get(){return refuse('an unknown represented callable name needs its actual source declaration witness');},configurable:true,enumerable:false});
   Object.defineProperty(handle,'toString',{value(){return refuse('represented function source text requires its actual original source mapping');},configurable:true,enumerable:false,writable:false});
   vmProxies.set(value.identity,handle);forward.set(handle,value.identity);return handle;
  }
  if(value.$==='symbol'||hostOwned.has(value.identity)){if(reverse.has(value.identity))return reverse.get(value.identity);}
  if(value.$==='symbol')throw new TypeError('a Symbol needs its actual authoritative host identity');
  const object=heap.objects.find(row=>row.identity===value.identity);if(!object)throw new TypeError('an Object needs its current actual shared heap');
  if(vmProxies.has(value.identity))return vmProxies.get(value.identity);
  // A live handle, never a copied record. Property access consults the latest
  // machine heap. Accessor calls suspend in the machine instead of invoking
  // untranslated source bodies from an engine callback.
  const proxy=new Proxy(Object.create(null),{
   get(_target,key){const plan=planJavaScriptOrdinaryGet(heap,value.identity,encode(key),value);if(plan.action!=='value')return refuse('host callbacks cannot bypass a represented accessor continuation');return decode(plan.value);},
   set(_target,key,assigned){const plan=planJavaScriptOrdinarySet(heap,value.identity,encode(key),encode(assigned),value);if(plan.action!=='complete'||plan.error)return refuse('host callbacks cannot bypass a represented setter continuation');heap=plan.heap;return plan.success;},
   ownKeys(){return refuse('ordinary VM object reflection needs its explicit host protocol');},
   getOwnPropertyDescriptor(){return refuse('ordinary VM object reflection needs its explicit host protocol');},
   getPrototypeOf(){return refuse('ordinary VM prototype reflection needs its explicit host protocol');}
  });
  vmProxies.set(value.identity,proxy);forward.set(proxy,value.identity);reverse.set(value.identity,proxy);return proxy;
 }
 const registerIntrinsic=(fn,invoke)=>{const semantic=encode(fn);intrinsics.set(semantic.identity,invoke);return semantic;};
 const hostGet=registerIntrinsic(function errorHostGet(){},(receiver,args)=>Reflect.get(receiver,args[0],args[1]));
 const hostSet=registerIntrinsic(function errorHostSet(){},(receiver,args)=>Reflect.set(receiver,args[0],args[1],args[2]));
 const primitiveGet=registerIntrinsic(function errorPrimitiveGet(){},(receiver,args)=>{
  if(receiver!==null&&(typeof receiver==='object'||typeof receiver==='function'))throw new TypeError('ordinary objects use machine property dispatch');
  return Reflect.apply(realm.getPrimitive,receiver,args);
 });
 const assign=registerIntrinsic(function errorStrictAssign(){},(receiver,args)=>Reflect.apply(realm.assign,receiver,args));
 // Error.prototype.toString is an actual realm operation; arbitrary prototype
 // methods are not admitted merely because their heap row is callable.
 const bases=new Map();for(const [name,base] of Object.entries(realm.errors)){
  if(readJavaScriptSourceExceptionOperation(name)===''||typeof base!=='function')throw new TypeError('the selected actual standard Error intrinsic is missing');
  bases.set(name,base);encode(base);const method=Reflect.getOwnPropertyDescriptor(base.prototype,'toString');if(method&&typeof method.value==='function')registerIntrinsic(method.value,(receiver,args)=>Reflect.apply(method.value,receiver,args));
 }
 const owners=[],declarations=[];
 for(const definition of capture.plan.functions){
  const supplied=Reflect.getOwnPropertyDescriptor(definitions,ordinaryClasses?String(definition.identity):definition.name),actual=supplied&&Object.hasOwn(supplied,'value')?supplied.value:null,origin=capture.origins.find(row=>row.identity===definition.identity);
  if(typeof actual!=='function'||!origin||Function.prototype.toString.call(actual)!==capture.source.slice(origin.start,origin.end))throw new TypeError('each callable must retain its exact actual captured declaration text');
  const name=declarationOwners===null?definition.name:declarationOwners.find(row=>row.identity===definition.identity&&row.compiledName===definition.name)?.originalName;
  const descriptor=Reflect.getOwnPropertyDescriptor(actual,'name');
  if(typeof name!=='string'||!name||!descriptor||descriptor.value!==definition.name||descriptor.writable!==false||descriptor.enumerable!==false||descriptor.configurable!==true)throw new TypeError('source callable names need the actual standard configurable own-name descriptor and authenticated declaration owner');
  if(definition.$==='errorConstructorPlan'){const base=bases.get(definition.baseClass);if(typeof base!=='function'||Reflect.getPrototypeOf(actual)!==base||Reflect.getPrototypeOf(actual.prototype)!==base.prototype)throw new TypeError('the derived class must retain its actual unmodified realm base and prototype');}
  declarations.push({definition,actual,name});
 }
 // Finish every declaration/prototype proof before changing any actual name.
 for(const {definition,actual,name}of declarations)if(name!==definition.name)Object.defineProperty(actual,'name',{value:name,configurable:true,enumerable:false,writable:false});
 for(const {definition,actual,name}of declarations){
  const callable=encode(actual);
  sourceNames.set(callable.identity,name);
  if(definition.$==='errorConstructorPlan'){
   const base=bases.get(definition.baseClass);
   owners.push({$:'errorConstructorOwner',source:definition.identity,identity:callable.identity,realm:identity,prototype:encode(actual.prototype),baseConstructor:encode(base),baseRealm:identity});
  }else if(definition.$==='errorOrdinaryConstructorPlan')owners.push({$:'ordinaryConstructorOwner',source:definition.identity,identity:callable.identity,realm:identity,prototype:encode(actual.prototype)});
  else owners.push({$:'errorFunctionOwner',source:definition.identity,identity:callable.identity,realm:identity});
 }
 if(ordinaryClasses&&(!Array.isArray(realm.arrayPrototype)||typeof realm.functionPrototype!=='function'))throw new TypeError('ordinary source arrays and closures require their actual realm Array and Function prototypes');
 const operations={$:'errorRealmOperations',realm:identity,objectPrototype:encode(realm.objectPrototype),get:primitiveGet,assign};
 if(ordinaryClasses){operations.arrayPrototype=encode(realm.arrayPrototype);operations.functionPrototype=encode(realm.functionPrototype);}
 const registry={$:'errorModuleRegistry',owners,operations:[operations],intrinsics:[...intrinsics.keys()].map(id=>({$:'errorIntrinsicOwner',identity:id,realm:identity})),realms:[identity],exoticKey:encode(realm.toPrimitive)};
 const constructorFacts=constructorDeclarations.map(declaration=>{const owner=owners.find(row=>row.source===declaration.source);if(!owner)throw new TypeError('constructor facts require their actual validated source owner');return {$:'arrayConstructorFact',identity:owner.identity,realm:owner.realm,constructable:declaration.constructable,intrinsicArray:false};});
 let module=deferModule?null:prepareJavaScriptErrorModule(capture,registry);
 // Only actual initial realm/declaration objects bypass live machine handles.
 // Caller data encoded afterward stays an alias of the mutable semantic heap.
 for(const row of heap.objects)hostOwned.add(row.identity);
 function opaque(value){
  if(forward.has(value))return encode(value);
  const id=allocate();forward.set(value,id);reverse.set(id,value);hostOwned.add(id);
  const prototype=encode(Reflect.getPrototypeOf(value));heap.objects.push({$:'hostErrorObject',identity:id,prototype,extensible:Reflect.isExtensible(value),callable:false,properties:[],hostGet,hostSet});sorted();return {$:'object',identity:id,callable:false};
 }
 function registerCapability(source,value,{operation,owner,verify,invoke}){
  if(typeof source!=='string'||!source||typeof operation!=='string'||!operation||typeof owner!=='string'||!owner||typeof value!=='function'||typeof verify!=='function'||typeof invoke!=='function')throw new TypeError('a capability requires independent actual callable, qualification and verification authority');
  verify();const semantic=encode(value),previous=capabilityOwners.get(semantic.identity);
  if(previous&&(previous.value!==value||previous.operation!==operation||previous.owner!==owner||previous.verify!==verify||previous.invoke!==invoke))throw new TypeError('one actual callable cannot acquire conflicting capability authority');
  capabilityOwners.set(semantic.identity,{value,operation,owner,verify,invoke});hostOwned.add(semantic.identity);
  return {$:'sourceCapabilityOwner',source,identity:semantic.identity,realm:identity,operation,owner,callable:true,executable:true};
 }
 const opaqueGetCapability=sourceFunctions?registerCapability('internal:opaque-get',function sourceOpaqueGet(){},{operation:'opaque property get',owner:'actual registered live host objects 1',verify(){},invoke(receiver,args){return Reflect.get(receiver,args[0],args[1]);}}):null;
 const opaqueSetCapability=sourceFunctions?registerCapability('internal:opaque-set',function sourceOpaqueSet(){},{operation:'opaque property set',owner:'actual registered live host objects 1',verify(){},invoke(receiver,args){return Reflect.set(receiver,args[0],args[1],args[2]);}}):null;
 const capabilityValue=owner=>({$:'object',identity:owner.identity,callable:owner.callable});
 function encodeOpaqueHost(value,kind,verify){
  if(!opaqueGetCapability||!opaqueSetCapability||!['hostURLObject','hostArrayObject','hostByteBufferObject'].includes(kind)||typeof verify!=='function')throw new TypeError('opaque host values require their explicit registered native object kind');
  verify(value);if(forward.has(value)){const id=forward.get(value),row=heap.objects.find(row=>row.identity===id);if(row?.$!==kind||opaqueOwners.get(id)?.value!==value)throw new TypeError('an opaque native value cannot retag an existing ordinary or Error identity');return {$:'object',identity:id,callable:false};}
  const id=allocate();forward.set(value,id);reverse.set(id,value);hostOwned.add(id);const prototype=encode(Reflect.getPrototypeOf(value));
  heap.objects.push({$:kind,identity:id,prototype,extensible:Reflect.isExtensible(value),callable:false,properties:[],hostGet:capabilityValue(opaqueGetCapability),hostSet:capabilityValue(opaqueSetCapability)});opaqueOwners.set(id,{value,kind,verify,prototype});sorted();return {$:'object',identity:id,callable:false};
 }
 function verifyOpaqueHeap(){for(const [id,owner]of opaqueOwners){const row=heap.objects.find(row=>row.identity===id);owner.verify(owner.value);if(row?.$!==owner.kind||row.callable||row.prototype.$!==owner.prototype.$||(row.prototype.$==='object'&&row.prototype.identity!==owner.prototype.identity)||row.properties.length!==0||row.hostGet?.identity!==opaqueGetCapability.identity||row.hostSet?.identity!==opaqueSetCapability.identity)refuse('an archived heap cannot replace an actual opaque host object or its native property authority');}}
 // Check supplied prototype snapshots without invoking any represented body.
 // Actual native references must already have private global identity owners;
 // this check cannot allocate an identity to legitimize an unknown prototype.
 function matchesNativeDataValue(semantic,native){
  if(native!==null&&(typeof native==='object'||typeof native==='function'||typeof native==='symbol')){
   const identity=forward.get(native);return identity!==undefined&&(typeof native==='symbol'?semantic.$==='symbol'&&semantic.identity===identity:semantic.$==='object'&&semantic.identity===identity&&semantic.callable===(typeof native==='function'));
  }
  return testJavaScriptSemanticSameValue(semantic,encode(native));
 }
 function verifyDataPrototypeSnapshot(semantic,native){
  let selected=semantic,actual=native,depth=0;
  while(actual!==null){
   if(++depth>4096||selected.$!=='object'||!matchesNativeDataValue(selected,actual))return refuse('wrapper materialization requires its actual supplied prototype identity');
   const row=heap.objects.find(object=>object.identity===selected.identity),keys=dataCarrierAuthority.ownKeys(actual);
   if(row?.$!=='ordinaryObject'||row.properties.length!==keys.length||row.extensible!==dataCarrierAuthority.extensible(actual))return refuse('wrapper materialization requires an unchanged supplied prototype snapshot');
   for(let at=0;at<keys.length;at++){
    const key=keys[at],property=row.properties[at],descriptor=dataCarrierAuthority.descriptor(actual,key),stored=property?.descriptor;
    if(!descriptor||!stored||!matchesNativeDataValue(property.key,key)||stored.enumerable!==descriptor.enumerable||stored.configurable!==descriptor.configurable)return refuse('wrapper materialization cannot replace represented prototype properties with native ones');
    if('value' in descriptor){if(!stored.valuePresent||!stored.writablePresent||stored.getterPresent||stored.setterPresent||stored.writable!==descriptor.writable||!matchesNativeDataValue(stored.value,descriptor.value))return refuse('wrapper materialization cannot replace represented prototype data with native data');}
    else if(stored.valuePresent||stored.writablePresent||!stored.getterPresent||!stored.setterPresent||!matchesNativeDataValue(stored.getter,descriptor.get)||!matchesNativeDataValue(stored.setter,descriptor.set))return refuse('wrapper materialization cannot replace represented prototype accessors with native ones');
   }
   selected=row.prototype;actual=dataCarrierAuthority.prototype(actual);
  }
  if(selected.$!=='null')return refuse('wrapper materialization requires the complete supplied prototype chain');
 }
 function materializeWrapperCarrier(row,prototype){
  const slot=row.stringData??row.primitiveData,stringSlot=row.stringData!==undefined;
  if(row.stringData!==undefined&&row.primitiveData!==undefined||slot===undefined||readJavaScriptOrdinaryValueError(heap,slot)!==''||stringSlot&&!['text','sourceText'].includes(slot.$)||!stringSlot&&!['number','boolean','integer','symbol'].includes(slot.$))return refuse('wrapper materialization requires exactly one actual primitive internal data slot');
  if(row.properties.length>4096||stringSlot&&readJavaScriptSemanticTextLength(slot)>4095)return refuse('wrapper data reification exceeds its explicit own-property profile');
  if(row.properties.some(property=>property.descriptor.getterPresent||property.descriptor.setterPresent||!property.descriptor.valuePresent))return refuse('plain data materialization cannot bypass an accessor continuation');
  let selected=null;
  if(row.prototype.$!=='null'){
   if(row.prototype.$!=='object')return refuse('wrapper materialization requires a supported actual supplied prototype');
   if(row.prototype.identity===prototype.identity)selected=realm.objectPrototype;
   else{const root=primitiveIteratorPrototypes?.find(candidate=>candidate.prototype.$==='object'&&candidate.prototype.identity===row.prototype.identity);if(root===undefined)return refuse('wrapper materialization refuses unsupported custom prototypes');selected=reverse.get(root.prototype.identity);if(selected===null||typeof selected!=='object')return refuse('wrapper materialization requires its privately supplied native prototype owner');}
   verifyDataPrototypeSnapshot(row.prototype,selected);
  }
  if(dataCarrierAuthority.descriptor(globalThis,'Object')?.value!==dataCarrierAuthority.object)return refuse('wrapper data reification retains its protected native carrier allocator');
  const result=dataCarrierAuthority.object(decode(slot));
  if(stringSlot?!nativeTypes.isStringObject(result):!primitiveValueSlots.find(candidate=>candidate.kind===slot.$)?.test(result))return refuse('wrapper data reification requires a genuine native primitive slot carrier');
  // String's inherent nonconfigurable properties cannot be omitted, changed or
  // synthesized when converting an inconsistent slot/descriptor snapshot.
  if(stringSlot)for(const key of dataCarrierAuthority.ownKeys(result)){
   const actual=dataCarrierAuthority.descriptor(result,key),property=row.properties.find(candidate=>matchesNativeDataValue(candidate.key,key)),stored=property?.descriptor;
   if(!actual||!stored||!stored.valuePresent||!stored.writablePresent||stored.getterPresent||stored.setterPresent||stored.writable!==actual.writable||stored.enumerable!==actual.enumerable||stored.configurable!==actual.configurable||!matchesNativeDataValue(stored.value,actual.value))return refuse('String wrapper data reification requires every exact immutable index and length descriptor');
  }
  if(!dataCarrierAuthority.setPrototype(result,selected))return refuse('wrapper data reification could not retain its selected prototype');return result;
 }
 function materializeData(value,memo=new Map()){
  if(value.$!=='object')return decode(value);
  if(value.callable)return refuse('plain data materialization cannot execute a represented callable');
  if(opaqueOwners.has(value.identity)){const owner=opaqueOwners.get(value.identity);owner.verify(owner.value);return owner.value;}
  if(memo.has(value.identity))return memo.get(value.identity);
  if(memo.size>=4096)return refuse('plain data materialization exceeds its explicit shared object budget');
  const row=heap.objects.find(row=>row.identity===value.identity),prototype=encode(realm.objectPrototype),array=ordinaryClasses&&row?.$==='arrayObject',wrapper=row?.$==='ordinaryObject'&&(row.primitiveData!==undefined||row.stringData!==undefined);
  const arrayPrototype=array&&realm.arrayPrototype?encode(realm.arrayPrototype):null;
  const expected=array?arrayPrototype:prototype;
  if(!row||(!array&&row.$!=='ordinaryObject')||!wrapper&&(row.prototype.$!=='null'&&(row.prototype.$!=='object'||expected?.$!=='object'||row.prototype.identity!==expected.identity)))return refuse('data materialization requires actual plain records, supplied realm arrays or registered native data');
  const result=wrapper?materializeWrapperCarrier(row,prototype):array?[]:Object.create(row.prototype.$==='null'?null:realm.objectPrototype);if(array)Object.setPrototypeOf(result,row.prototype.$==='null'?null:realm.arrayPrototype);memo.set(value.identity,result);
  // Defining an Array's potentially readonly length before its existing indices
  // would reject a valid snapshot. Create the actual sparse own indices first,
  // then install the authentic length descriptor without filling holes.
  let length=null;
  for(const property of row.properties){const descriptor=property.descriptor;if(descriptor.getterPresent||descriptor.setterPresent||!descriptor.valuePresent)return refuse('plain data materialization cannot bypass an accessor continuation');const key=decode(property.key);if(array&&key==='length'){length=descriptor;continue;}const attributes={value:materializeData(descriptor.value,memo),writable:descriptor.writable,enumerable:descriptor.enumerable,configurable:descriptor.configurable};if(wrapper){if(!dataCarrierAuthority.define(result,key,attributes))return refuse('wrapper data reification could not retain its exact own descriptor');}else Object.defineProperty(result,key,attributes);}
  if(array){if(length===null||length.value.$!=='number')return refuse('array materialization requires its actual length descriptor');Object.defineProperty(result,'length',{value:length.value.value,writable:length.writable,enumerable:length.enumerable,configurable:length.configurable});}
  if(!row.extensible){if(wrapper){if(!dataCarrierAuthority.prevent(result))return refuse('wrapper data reification could not retain its actual extensibility');}else Object.preventExtensions(result);}return result;
 }
 function mergeSourceTemplates(...programs){
  const templates=[];let selected=false;
  for(const program of programs){if(program.templates!==undefined)selected=true;for(const template of program.templates??[]){const prior=templates.find(row=>row.owner===template.owner&&row.ordinal===template.ordinal);if(prior&&!isDeepStrictEqual(prior,template))throw new TypeError('one lexical code site cannot acquire conflicting templates during source composition');if(!prior)templates.push(template);if(templates.length>4096)throw new RangeError('source composition exceeds its static template budget');}}
  return selected?{templates:templates.toSorted((a,b)=>a.owner<b.owner?-1:a.owner>b.owner?1:a.ordinal-b.ordinal)}:{};
 }
 function finishSourceCapabilities(capabilities,bindings){
  if(!sourceFunctions||module!==null)throw new TypeError('source function capability publication requires its actual unpublished source module');
  const compiled=compileJavaScriptErrorModulePlanWithCapabilities(capture.plan,registry,bindings,capabilities);if(compiled.error)throw new TypeError(compiled.error);
  const functions=[...compiled.program.functions];for(const fn of portableFunctions.values()){const previous=functions.find(row=>row.identity===fn.identity);if(previous&&!isDeepStrictEqual(previous,fn))throw new TypeError('portable operation frames cannot replace an actual source implementation');if(!previous)functions.push(fn);}
  module={capture,registry,program:{...compiled.program,...mergeSourceTemplates(compiled.program,portableTemplateProgram),functions:functions.toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0)},sourceProgram:compiled.program,sourceUnits:[],bindings,capabilities};return module;
 }
 function registerPortableProgram(program){
  if(!sourceFunctions||module!==null)throw new TypeError('portable operation frames must be linked before source publication');
  if(!isDeepStrictEqual(program.realms,registry.realms)||!testJavaScriptSemanticSameValue(program.exoticKey,registry.exoticKey))throw new TypeError('portable operation frames require the same actual source realm and primitive conversion Symbol');
  const invalid=readJavaScriptCallProgramError(program,heap);if(invalid)throw new TypeError(invalid);
  portableTemplateProgram=mergeSourceTemplates(portableTemplateProgram,program);
  for(const fn of program.functions){const previous=portableFunctions.get(fn.identity);if(previous&&!isDeepStrictEqual(previous,fn))throw new TypeError('one actual operation identity cannot acquire conflicting portable implementations');portableFunctions.set(fn.identity,fn);}
 }
 // Register metadata authority before publication, independently of mutable
 // module/capture records. It never executes a represented constructor body.
 function registerPrimitiveIteratorPrototypes(roots){
  if(!sourceFunctions||module!==null||primitiveIteratorPrototypes!==null||!Array.isArray(roots)||roots.length!==6)throw new TypeError('primitive iterator roots are registered once before source publication');
  const kinds=['text','sourceText','number','boolean','integer','symbol'];
  for(const kind of kinds){const matches=roots.filter(row=>row.kind===kind);if(matches.length!==1||matches[0].prototype?.$!=='object'||readJavaScriptOrdinaryValueError(heap,matches[0].prototype)!=='')throw new TypeError('primitive iteration requires each actual registered realm prototype');}
  primitiveIteratorPrototypes=roots.map(row=>({kind:row.kind,prototype:{...row.prototype}}));
  if(ordinaryClasses){operations.stringPrototype={...primitiveIteratorPrototypes.find(row=>row.kind==='text').prototype};operations.numberPrototype={...primitiveIteratorPrototypes.find(row=>row.kind==='number').prototype};operations.booleanPrototype={...primitiveIteratorPrototypes.find(row=>row.kind==='boolean').prototype};operations.integerPrototype={...primitiveIteratorPrototypes.find(row=>row.kind==='integer').prototype};operations.symbolPrototype={...primitiveIteratorPrototypes.find(row=>row.kind==='symbol').prototype};}
 }
 function resumePrimitiveIteratorGet(request){
  if(module===null||request?.$!=='machineNeedPrimitiveIteratorGet'||!request.machine)throw new TypeError('primitive iterator metadata requires its actual published request');
  const error=readJavaScriptCallMachineError(module.program,request.machine);if(error)throw new TypeError(error);const actual=stepJavaScriptCallMachine(module.program,request.machine);
  if(actual.$!=='machineNeedPrimitiveIteratorGet'||!testJavaScriptSemanticSameValue(actual.receiver,request.receiver)||!testJavaScriptSemanticSameValue(actual.key,request.key))throw new TypeError('primitive iterator metadata cannot substitute the actual receiver or key');
  const root=primitiveIteratorPrototypes?.find(row=>row.kind===request.receiver.$);if(root===undefined)return {resumed:false,transition:request};
  if(heap!==request.machine.heap)throw new TypeError('primitive iterator metadata requires the adopted latest heap');
  const prototype={...root.prototype},transition=resumeJavaScriptCallPrimitiveIteratorPrototype(module.program,request.machine,prototype);return {resumed:true,prototype,transition};
 }
 function resumeObjectPrimitivePrototype(request){
  if(module===null||request?.$!=='machineNeedObjectPrototype'||!request.machine)throw new TypeError('primitive prototype metadata requires its actual published request');
  const error=readJavaScriptCallMachineError(module.program,request.machine);if(error)throw new TypeError(error);const actual=stepJavaScriptCallMachine(module.program,request.machine);
  if(actual.$!=='machineNeedObjectPrototype'||actual.realm!==request.realm||!testJavaScriptSemanticSameValue(actual.input,request.input))throw new TypeError('primitive prototype metadata cannot replace its actual input or source realm');
  if(request.realm!==identity)return {resumed:false,transition:request};const root=primitiveIteratorPrototypes?.find(row=>row.kind===request.input.$);if(root===undefined)return {resumed:false,transition:request};
  if(heap!==request.machine.heap)throw new TypeError('primitive prototype metadata requires the adopted latest heap');
  const prototype={...root.prototype};return {resumed:true,prototype,transition:resumeJavaScriptCallObjectPrimitivePrototype(module.program,request.machine,prototype)};
 }
 function resumeSymbolAllocation(request){
  if(module===null||request?.$!=='machineAllocateSymbol'||!request.machine)throw new TypeError('Symbol allocation requires its actual published request');
  const error=readJavaScriptCallMachineError(module.program,request.machine);if(error)throw new TypeError(error);const actual=stepJavaScriptCallMachine(module.program,request.machine);
  if(actual.$!=='machineAllocateSymbol'||!testJavaScriptSemanticSameValue(actual.description,request.description)||heap!==request.machine.heap)throw new TypeError('Symbol allocation retains its completed description and adopted latest heap');
  if(globalThis.Symbol!==actualSymbolAllocator)throw new TypeError('Symbol allocation requires the captured actual identity allocator');
  const identity=allocate(),transition=resumeJavaScriptCallSymbolAllocation(module.program,request.machine,identity);
  if(transition.$==='machineContinue'){const arguments_=actual.description.$==='undefined'?[]:[readJavaScriptSemanticTextString(actual.description)],symbol=Reflect.apply(actualSymbolAllocator,undefined,arguments_);forward.set(symbol,identity);reverse.set(identity,symbol);}
  return {resumed:true,identity,transition};
 }
 function resumeSymbolDescription(request){
  if(module===null||request?.$!=='machineNeedSymbolDescription'||!request.machine)throw new TypeError('Symbol description metadata requires its actual published request');
  const error=readJavaScriptCallMachineError(module.program,request.machine);if(error)throw new TypeError(error);const actual=stepJavaScriptCallMachine(module.program,request.machine);
  if(actual.$!=='machineNeedSymbolDescription'||!testJavaScriptSemanticSameValue(actual.symbol,request.symbol)||heap!==request.machine.heap)throw new TypeError('Symbol description metadata retains its actual Symbol and adopted latest heap');
  const symbol=reverse.get(request.symbol.identity);if(typeof symbol!=='symbol')return {resumed:false,transition:request};
  if(typeof symbolDescriptionGetter!=='function'||Reflect.getOwnPropertyDescriptor(Symbol.prototype,'description')?.get!==symbolDescriptionGetter)throw new TypeError('Symbol descriptions require their protected actual slot reader');
  const value=Reflect.apply(symbolDescriptionGetter,symbol,[]),description=value===undefined?{$:'undefined'}:value.length>65536?constructJavaScriptSourceText(value):text(value);
  return {resumed:true,description,transition:resumeJavaScriptCallSymbolDescription(module.program,request.machine,description)};
 }
 function resumeFunctionNameSymbolDescription(request){
  if(module===null||request?.$!=='machineNeedFunctionNameSymbolDescription'||!request.machine)throw new TypeError('Symbol name metadata requires its actual published suspended request');
  const error=readJavaScriptCallMachineError(module.program,request.machine);if(error)throw new TypeError(error);const actual=stepJavaScriptCallMachine(module.program,request.machine);
  if(actual.$!=='machineNeedFunctionNameSymbolDescription'||!testJavaScriptSemanticSameValue(actual.object,request.object)||!testJavaScriptSemanticSameValue(actual.key,request.key)||actual.prefix!==request.prefix)throw new TypeError('Symbol name metadata retains its actual function key and prefix');
  if(heap!==request.machine.heap)throw new TypeError('Symbol name metadata requires the adopted latest heap');
  const symbol=reverse.get(request.key.identity);if(typeof symbol!=='symbol')return {resumed:false,transition:request};
  if(typeof symbolDescriptionGetter!=='function'||Reflect.getOwnPropertyDescriptor(Symbol.prototype,'description')?.get!==symbolDescriptionGetter)throw new TypeError('Symbol name metadata requires its protected actual description getter');
  const value=Reflect.apply(symbolDescriptionGetter,symbol,[]),description=value===undefined?{$:'undefined'}:value.length>65536?constructJavaScriptSourceText(value):text(value);
  return {resumed:true,description,transition:resumeJavaScriptCallFunctionNameSymbolDescription(module.program,request.machine,description)};
 }
 function registerArrayConstructorAuthority(provider){
  if(!sourceFunctions||module!==null||arrayConstructorAuthority!==null||typeof provider!=='function')throw new TypeError('constructor metadata authority is registered once before source publication');
  arrayConstructorAuthority=provider;
 }
 function resumeArrayConstructorAuthority(request){
  if(module===null||request?.$!=='machineNeedConstructorAuthority'||!request.machine)throw new TypeError('constructor metadata response requires its actual published pending request');
  const error=readJavaScriptCallMachineError(module.program,request.machine);if(error)throw new TypeError(error);
  const pending=stepJavaScriptCallMachine(module.program,request.machine);
  if(pending.$!=='machineNeedConstructorAuthority'||!testJavaScriptSemanticSameValue(pending.constructor,request.constructor))throw new TypeError('constructor metadata response cannot substitute the actual selected constructor');
  if(arrayConstructorAuthority===null)return {updated:false,transition:request};
  // The caller first adopts the complete pending heap. Capturing actual roots
  // cannot resurrect an older embedding heap or discard represented effects.
  if(heap!==request.machine.heap)throw new TypeError('constructor metadata response requires the adopted current shared heap');
  const directory=arrayConstructorAuthority(),updated=extendJavaScriptCallArrayAuthority(module.program,request.machine,directory);
  if(updated.error)throw new TypeError(updated.error);
  if(isDeepStrictEqual(updated.program,module.program))return {updated:false,transition:request};
  module={...module,program:updated.program};
  return {updated:true,transition:{$:'machineContinue',machine:updated.machine}};
 }
 function attachSourceUnit(input,definitions,capabilities,bindings=[]){
  if(!sourceFunctions||module===null)throw new TypeError('a supplied source unit requires its already published actual source realm');
  const actual=checkJavaScriptProjectDataSource(input),owners=[];
  for(const definition of actual.plan.functions){
   const origin=actual.origins.find(row=>row.identity===definition.identity),descriptor=Reflect.getOwnPropertyDescriptor(definitions,definition.name),value=descriptor?.value;
   if((definition.$!=='errorStatementFunctionPlan'&&!(definition.$==='errorMutableStatementFunctionPlan'&&actual.model==='actual closed source classes 1'))||!origin||typeof value!=='function'||Function.prototype.toString.call(value)!==actual.source.slice(origin.start,origin.end)||Reflect.getOwnPropertyDescriptor(value,'name')?.value!==definition.name)throw new TypeError('an additional source unit needs its exact owned ordinary declarations');
   const semantic=encode(value);if(sourceNames.has(semantic.identity))throw new TypeError('an additional source unit cannot replace an existing declaration owner');sourceNames.set(semantic.identity,definition.name);owners.push({$:'errorFunctionOwner',source:definition.identity,identity:semantic.identity,realm:identity});
  }
  const unitRegistry={...registry,owners},compiled=compileJavaScriptErrorModulePlanWithCapabilities(actual.plan,unitRegistry,bindings,capabilities);if(compiled.error)throw new TypeError(compiled.error);
  const functions=[...module.program.functions];
  for(const fn of compiled.program.functions){const previous=functions.find(row=>row.identity===fn.identity);if(previous){if(!isDeepStrictEqual(previous,fn))throw new TypeError('supplied source units must agree on every shared actual callable implementation');}else functions.push(fn);}
  const unit={capture:actual,registry:unitRegistry,bindings,capabilities,sourceProgram:compiled.program},program={...module.program,...mergeSourceTemplates(module.program,compiled.program),functions:functions.toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0)};module={...module,program,sourceUnits:[...module.sourceUnits,unit]};
  for(const owner of owners)constructorFacts.push({$:'arrayConstructorFact',identity:owner.identity,realm:owner.realm,constructable:true,intrinsicArray:false});
  return {...unit,program};
 }
 const authority={perform(request){
  heap=request.machine.heap;
  const qualified=request.$==='machineInvokeHostCapability'?capabilityOwners.get(request.callee.identity):null;
  if(request.$==='machineInvokeHostCapability'){if(!qualified)refuse('the requested operation lacks independently registered host capability authority');qualified.verify();}
  verifyOpaqueHeap();
  // The independent allocator skips actual occupied identities. A sparse high
  // identity never establishes that every smaller identity has been consumed.
  try{
   if(request.realm!==identity)return refuse('the request belongs to a different actual host realm');
   let result;
   if(request.$==='machineInvokeHostCapability'){
    result=qualified.invoke(decode(request.receiver),request.arguments.map(decode),request.arguments);
   }else if(request.$==='machineInvokeErrorIntrinsic'){
    const invoke=intrinsics.get(request.callee.identity);if(!invoke)return refuse('the requested callable is not a registered actual Error host operation');
    result=invoke(decode(request.receiver),request.arguments.map(decode));
   }else{
    const base=bases.get(request.errorClass);if(!base)return refuse('the requested standard Error constructor is unavailable in this actual realm');
    if(request.$==='machineAllocateErrorValue'){
     if(request.constructor.$!=='object'||!request.constructor.callable||reverse.get(request.constructor.identity)!==base)return refuse('source metadata cannot certify an intrinsic constructor');
     // This is the sole raw source-callable extraction. Reflect.construct
     // invokes the actual BASE, using target only as newTarget/prototype; it
     // does not execute target's represented constructor body.
     const target=reverse.get(request.newTarget.identity),owner=owners.find(row=>row.$==='errorConstructorOwner'&&row.identity===request.newTarget.identity);
     if(!owner||Reflect.getPrototypeOf(target)!==base||target.prototype!==decode(request.prototype))return refuse('the newTarget/prototype no longer match their actual supplied declaration');
     result=Reflect.construct(base,request.messagePresent?[decode(request.message)]:[],target);
    }else if(request.$==='machineAllocateSourceError')result=Reflect.construct(base,request.messagePresent?[decode(request.message)]:[]);
    else result=Reflect.construct(base,[request.reason]);
    const value=opaque(result);return {heap,value,abrupt:false};
   }
   const value=nativeTypes.isNativeError(result)?opaque(result):encode(result);return {heap,value,abrupt:false};
  }catch(error){
   if(refusals.has(error))throw error;
   // Genuine engine exceptions retain their original live objects. Authority
   // configuration failures are explicit too; no guessed error snapshots.
   const value=opaque(error);return {heap,value,abrupt:true};
  }
 }};
 return {get module(){if(module===null)throw new TypeError('source module functions remain unavailable before successful initialization');return module;},authority,encode,decode,sourceConstructorFacts(){return constructorFacts.map(row=>({...row}));},heap:()=>heap,adoptHeap(value){heap=value;},allocateIdentity:allocate,
  registerCapability,encodeOpaqueHost,materializeData,finishSourceCapabilities,attachSourceUnit,registerPortableProgram,registerArrayConstructorAuthority,resumeArrayConstructorAuthority,registerPrimitiveIteratorPrototypes,resumePrimitiveIteratorGet,resumeSymbolAllocation,resumeSymbolDescription,resumeFunctionNameSymbolDescription,resumeObjectPrimitivePrototype,
  refuseCapability:refuse,
  sourceCallableReference(source,semantic,{operation,owner}){if(semantic?.$!=='object'||!semantic.callable||!sourceNames.has(semantic.identity)||!module?.program.functions.some(row=>row.identity===semantic.identity))throw new TypeError('a source import reference requires its actual published declaration and portable body');return {$:'sourceCapabilityOwner',source,identity:semantic.identity,realm:identity,operation,owner,callable:true,executable:false};},
  realmErrorConstructorReference(source,name){const actual=bases.get(name);if(typeof actual!=='function'||realm.errors[name]!==actual)throw new TypeError('Error namespace references retain their actual independently supplied realm constructor');const semantic=encode(actual);return {$:'sourceCapabilityOwner',source,identity:semantic.identity,realm:identity,operation:'actual standard '+name+' namespace',owner:'actual registered Error realm 1',callable:true,executable:false};},
  capabilityReference(source,value,{operation,owner}){const semantic=encode(value);if(semantic.$!=='object')throw new TypeError('a capability reference requires its actual native namespace or source callable identity');return {$:'sourceCapabilityOwner',source,identity:semantic.identity,realm:identity,operation,owner,callable:semantic.callable,executable:false};},
  opaqueCapabilities:()=>opaqueGetCapability&&opaqueSetCapability?[opaqueGetCapability,opaqueSetCapability]:[],
  encodeHostError(value){if(!nativeTypes.isNativeError(value))throw new TypeError('an IO Error result requires actual engine Error authority');return opaque(value);},
  finishModuleInitialization(state){if(module!==null)throw new TypeError('actual source module initialization is completed exactly once');for(const slot of state.slots){const error=readJavaScriptOrdinaryValueError(heap,slot.value);if(error)throw new TypeError(error);}module=prepareJavaScriptInitializedErrorModule(capture,registry,state);return module;}
 };
}
