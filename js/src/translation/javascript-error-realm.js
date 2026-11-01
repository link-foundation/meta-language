// Host boundary for the portable Error source machine. Source definitions are
// supplied as real declaration values, checked against the captured text and
// NEVER invoked by this adapter. Only independently supplied realm intrinsics
// and the property operations below execute in the host.
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {planJavaScriptOrdinaryGet,planJavaScriptOrdinarySet,readJavaScriptOrdinaryValueError} from './javascript-ordinary-heap.js';
import {prepareJavaScriptErrorModule,prepareJavaScriptInitializedErrorModule} from './javascript-error-module.js';
import {types as nativeTypes} from 'node:util';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {validateJavaScriptErrorClassModule} from './javascript-error-classes.js';
import {readSourceLibraryErrorDeclarationWitness} from './javascript-error-closure-source.js';

const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))});
const string=units=>{let result='';for(const unit of units)result+=String.fromCharCode(unit);return result;};

/** `realm` is embedding authority, not serialized source metadata. It supplies
 * actual Object.prototype, Symbol.toPrimitive and each admitted standard Error
 * constructor from ONE real realm. `definitions` contains all actual captured
 * declarations (including private helpers). Their bodies are not a fallback
 * execution path. Modified prototypes, proxies and reflection on VM ordinary
 * objects require successor host protocols and are refused here. */
export function createJavaScriptErrorRealm(capture,{realm,definitions,identity=1n,declarationWitness=null,deferModule=false}){
 capture=validateJavaScriptErrorClassModule(capture);
 const declarationOwners=declarationWitness===null?null:readSourceLibraryErrorDeclarationWitness(capture,declarationWitness);
 if(!realm||!realm.errors||!realm.objectPrototype||typeof realm.objectPrototype!=='object'||Reflect.getPrototypeOf(realm.objectPrototype)!==null||typeof realm.toPrimitive!=='symbol'||typeof realm.getPrimitive!=='function'||typeof realm.assign!=='function'||!definitions||typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n)throw new TypeError('actual complete Error realm authority is required');
 let next=1n,heap={$:'ordinaryHeap',objects:[],symbols:[]};
 const forward=new Map(),reverse=new Map(),intrinsics=new Map(),vmProxies=new Map(),hostOwned=new Set(),refusals=new WeakSet(),sourceNames=new Map();
 function refuse(reason){const error=new TypeError(reason);refusals.add(error);throw error;}
 const allocate=()=>{while(heap.objects.some(row=>row.identity===next)||heap.symbols.includes(next))next+=1n;if(next>18446744073709551615n)throw new RangeError('the authoritative global identity allocator exhausted unsigned64 identities');return next++;};
 const sorted=()=>{heap={...heap,objects:heap.objects.toSorted((a,b)=>a.identity<b.identity?-1:a.identity>b.identity?1:0),symbols:heap.symbols.toSorted((a,b)=>a<b?-1:a>b?1:0)};};
 function encode(value){
  if(value===undefined)return {$:'undefined'};if(value===null)return {$:'null'};
  if(typeof value==='string')return text(value);if(typeof value==='boolean')return {$:'boolean',value};if(typeof value==='number')return {$:'number',value};if(typeof value==='bigint')return {$:'integer',value};
  if(forward.has(value)){const id=forward.get(value);return typeof value==='symbol'?{$:'symbol',identity:id}:{$:'object',identity:id,callable:typeof value==='function'};}
  if(typeof value==='symbol'){const id=allocate();forward.set(value,id);reverse.set(id,value);heap.symbols.push(id);sorted();return {$:'symbol',identity:id};}
  if(typeof value!=='object'&&typeof value!=='function')throw new TypeError('unmodeled host semantic value');
  const id=allocate();forward.set(value,id);reverse.set(id,value);
  const row={$:'ordinaryObject',identity:id,prototype:{$:'null'},extensible:Reflect.isExtensible(value),callable:typeof value==='function',properties:[]};heap.objects.push(row);sorted();
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
  if(value.$==='undefined')return undefined;if(value.$==='null')return null;if(value.$==='text')return string(value.units);
  if(['number','integer','boolean'].includes(value.$))return value.value;
  if(value.$==='object'&&value.callable){
   if(intrinsics.has(value.identity))return reverse.get(value.identity);
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
  const supplied=Reflect.getOwnPropertyDescriptor(definitions,definition.name),actual=supplied&&Object.hasOwn(supplied,'value')?supplied.value:null,origin=capture.origins.find(row=>row.identity===definition.identity);
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
  }else owners.push({$:'errorFunctionOwner',source:definition.identity,identity:callable.identity,realm:identity});
 }
 const registry={$:'errorModuleRegistry',owners,operations:[{$:'errorRealmOperations',realm:identity,objectPrototype:encode(realm.objectPrototype),get:primitiveGet,assign}],intrinsics:[...intrinsics.keys()].map(id=>({$:'errorIntrinsicOwner',identity:id,realm:identity})),realms:[identity],exoticKey:encode(realm.toPrimitive)};
 let module=deferModule?null:prepareJavaScriptErrorModule(capture,registry);
 // Only actual initial realm/declaration objects bypass live machine handles.
 // Caller data encoded afterward stays an alias of the mutable semantic heap.
 for(const row of heap.objects)hostOwned.add(row.identity);
 function opaque(value){
  if(forward.has(value))return encode(value);
  const id=allocate();forward.set(value,id);reverse.set(id,value);hostOwned.add(id);
  const prototype=encode(Reflect.getPrototypeOf(value));heap.objects.push({$:'hostErrorObject',identity:id,prototype,extensible:Reflect.isExtensible(value),callable:false,properties:[],hostGet,hostSet});sorted();return {$:'object',identity:id,callable:false};
 }
 const authority={perform(request){
  heap=request.machine.heap;
  // The independent allocator skips actual occupied identities. A sparse high
  // identity never establishes that every smaller identity has been consumed.
  try{
   if(request.realm!==identity)return refuse('the request belongs to a different actual host realm');
   let result;
   if(request.$==='machineInvokeErrorIntrinsic'){
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
   const value=encode(result);return {heap,value,abrupt:false};
  }catch(error){
   if(refusals.has(error))throw error;
   // Genuine engine exceptions retain their original live objects. Authority
   // configuration failures are explicit too; no guessed error snapshots.
   const value=opaque(error);return {heap,value,abrupt:true};
  }
 }};
 return {get module(){if(module===null)throw new TypeError('source module functions remain unavailable before successful initialization');return module;},authority,encode,decode,heap:()=>heap,adoptHeap(value){heap=value;},allocateIdentity:allocate,
  encodeHostError(value){if(!nativeTypes.isNativeError(value))throw new TypeError('an IO Error result requires actual engine Error authority');return opaque(value);},
  finishModuleInitialization(state){if(module!==null)throw new TypeError('actual source module initialization is completed exactly once');for(const slot of state.slots){const error=readJavaScriptOrdinaryValueError(heap,slot.value);if(error)throw new TypeError(error);}module=prepareJavaScriptInitializedErrorModule(capture,registry,state);return module;}
 };
}
