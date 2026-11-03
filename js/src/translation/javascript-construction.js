import {allocateJavaScriptWeakCollection} from './javascript-weak-collections.js';
import {allocateJavaScriptCollection} from './javascript-collections.js';
import {testJavaScriptConstructedCollectionKind,readJavaScriptConstructedCollectionAllocationKind} from './javascript-collection-construction-rules.js';
import {readJavaScriptCollectionConstructorKind} from './javascript-collection-call-source.js';
// Generic base [[Construct]] over actual target/newTarget facts and realm roots.
// Intrinsic kinds come from protected identity authority, never function spelling.
import {readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,insertJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet,defineJavaScriptOrdinaryProperty,createJavaScriptOrdinarySetDescriptor} from './javascript-ordinary-heap.js';
import {classifyJavaScriptArraySpeciesConstructor,readJavaScriptArraySpeciesDirectoryError,findJavaScriptArrayConstructorFact} from './javascript-array-species.js';
import {readJavaScriptExecutableCallFunction} from './javascript-call-templates.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {beginJavaScriptScalarProtocol,stepJavaScriptScalarProtocol,resumeJavaScriptScalarProtocol,finishJavaScriptScalarPrimitive} from './javascript-scalar-conversion-protocol.js';
import {readJavaScriptPrimitiveConversionStateError} from './javascript-primitive-conversion.js';
import {convertJavaScriptSemanticBoolean} from './javascript-descriptor-conversion.js';
import {beginJavaScriptPrimitiveWrapper,resumeJavaScriptPrimitiveWrapper} from './javascript-primitive-wrappers.js';
import {allocateJavaScriptArray} from './javascript-arrays.js';
import {normalizeJavaScriptUnsignedInteger} from './number-bitwise-rules.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
import {testJavaScriptReflectionPropertyBudget} from './javascript-property-reflection.js';
/** @typedef {{$:'constructionCallRecord',callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],constructing:boolean,newTarget:JavaScriptSemanticValue}} JavaScriptConstructionCall */
/** @typedef {{$:'constructionState',target:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],phase:string,mode:string,prototype:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,primitive:JavaScriptSemanticValue,arrayIndex:number,allocationKind:string,conversion?:JavaScriptPrimitiveConversionState,call?:JavaScriptConstructionCall}} JavaScriptConstructionState */
/** @typedef {{$:'constructionContinue',heap:JavaScriptOrdinaryHeap,state:JavaScriptConstructionState} |
 * {$:'constructionCall',heap:JavaScriptOrdinaryHeap,state:JavaScriptConstructionState,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],constructing:boolean,newTarget:JavaScriptSemanticValue} |
 * {$:'constructionAllocate',heap:JavaScriptOrdinaryHeap,state:JavaScriptConstructionState,prototype:JavaScriptSemanticValue} |
 * {$:'constructionNeedAuthority',heap:JavaScriptOrdinaryHeap,state:JavaScriptConstructionState,constructor:JavaScriptSemanticValue} |
 * {$:'constructionNeedImplementation',heap:JavaScriptOrdinaryHeap,state:JavaScriptConstructionState,constructor:JavaScriptSemanticValue,newTarget:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],operation:string,reason:string} |
 * {$:'constructionCompleted',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'constructionThrow',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'constructionError',heap:JavaScriptOrdinaryHeap,errorClass:string,reason:string,realm?:bigint} |
 * {$:'constructionInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'constructionResourceLimit',heap:JavaScriptOrdinaryHeap,state:JavaScriptConstructionState,reason:string}} JavaScriptConstructionTransition */
/** @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSemanticValue} candidate @returns {JavaScriptArrayConstructorFact|undefined} */
export function readJavaScriptConstructionFact(directory,candidate){return candidate.$==='object'&&candidate.callable?directory.constructors[findJavaScriptArrayConstructorFact(directory.constructors,candidate.identity)]:undefined;}
/** Explicit intrinsicKind extends existing Array facts. A constructable source
 * fact separately proves that an actual strict function/base class is a
 * constructor; receiverMode alone never grants that classification.
 * @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArrayConstructorFact} fact @returns {string} */
export function readJavaScriptConstructionMode(program,heap,fact){const actual=heap.objects[findJavaScriptOrdinaryObject(heap.objects,fact.identity)];return actual?.$==='boundFunctionObject'?'bound function':fact.intrinsicKind!==undefined?fact.intrinsicKind:fact.intrinsicArray?'Array':readJavaScriptExecutableCallFunction(program,heap,fact.identity)?.receiverMode??'';}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @returns {string} */
export function readJavaScriptConstructionStateError(program,heap,directory,state){
 const invalid=readJavaScriptArraySpeciesDirectoryError(heap,directory);if(invalid!=='')return invalid;
 if(state.$!=='constructionState'||!['target','newTarget','initialize','convert','converted primitive','prototype','prototype getter','prototype fallback','Object root','allocate','array length','array elements','body'].includes(state.phase)||state.arguments.length>1048576||!Number.isSafeInteger(state.arrayIndex)||Object.is(state.arrayIndex,-0)||state.arrayIndex<0||state.arrayIndex>state.arguments.length)return 'construction retains its actual bounded argument vector and source continuation';
 for(const value of [state.target,state.newTarget,state.prototype,state.receiver,state.primitive,...state.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
 const target=readJavaScriptConstructionFact(directory,state.target),newTarget=readJavaScriptConstructionFact(directory,state.newTarget);
 if(state.phase!=='target'&&(target===undefined||!target.constructable||state.mode!==readJavaScriptConstructionMode(program,heap,target)))return 'construction retains its actual target classification and executable or protected intrinsic kind';
 if(!['target','newTarget'].includes(state.phase)&&(newTarget===undefined||!newTarget.constructable))return 'construction retains its actual validated newTarget constructor';
 for(const fact of [target,newTarget])if(fact!==undefined){const fn=readJavaScriptExecutableCallFunction(program,heap,fact.identity);if(fn!==undefined&&fn.realm!==fact.realm)return 'construction retains the actual executable constructor realm recorded by authority';}
 if(!['','ordinary','array','wrapper','collection','weak collection'].includes(state.allocationKind))return 'construction retains its actual selected allocation kind';
 if(['prototype','prototype getter','prototype fallback','Object root','allocate','array length','array elements','body'].includes(state.phase)){
  const expected=testJavaScriptConstructedCollectionKind(state.mode)?readJavaScriptConstructedCollectionAllocationKind(state.mode):state.mode==='Array'?'array':['Number','Boolean','String'].includes(state.mode)||state.mode==='Object'&&testJavaScriptSemanticSameValue(state.target,state.newTarget)?'wrapper':'ordinary';if(state.allocationKind!==expected)return 'construction cannot replace the allocation kind selected by its actual constructor';
  if(state.mode==='Number'&&state.primitive.$!=='number'||state.mode==='Boolean'&&state.primitive.$!=='boolean'||state.mode==='String'&&state.primitive.$!=='text'&&state.primitive.$!=='sourceText')return 'boxed constructors retain their completed actual primitive data value';
  if(state.mode==='Object'&&testJavaScriptSemanticSameValue(state.target,state.newTarget)&&(!testJavaScriptSemanticSameValue(state.primitive,state.arguments[0]??{$:'undefined'})||state.primitive.$==='object'))return 'Object self-construction retains its actual uncoerced primitive input';
 }
 if(['array length','array elements'].includes(state.phase)&&state.mode!=='Array')return 'Array constructor continuation phases retain their actual Array target kind';
 if(['allocate','array length','array elements','body'].includes(state.phase)&&state.prototype.$!=='object')return 'construction retains the completed GetPrototypeFromConstructor or actual Object root';
 if(['body','array length','array elements'].includes(state.phase)){
  const receiver=state.receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,state.receiver.identity)]:undefined;
  const brand=state.mode==='Map'?'mapObject':state.mode==='Set'?'setObject':state.mode==='WeakMap'?'weakMapObject':state.mode==='WeakSet'?'weakSetObject':'ordinaryObject';
  if(receiver===undefined||state.phase==='body'&&receiver.$!==brand||state.phase!=='body'&&receiver.$!=='arrayObject'||receiver.callable||state.phase!=='body'&&!testJavaScriptSemanticSameValue(receiver.prototype,state.prototype))return 'construction retains its actual allocated receiver kind and selected newTarget prototype';
 }
 if(state.conversion!==undefined){const error=readJavaScriptPrimitiveConversionStateError(heap,state.conversion);if(error!=='')return error;if(state.phase!=='convert'||!['Number','String'].includes(state.mode)||!testJavaScriptSemanticSameValue(state.conversion.receiver,state.arguments[0]??{$:'undefined'})||!testJavaScriptSemanticSameValue(state.conversion.exoticKey,program.exoticKey))return 'construction retains its actual first-argument conversion';if(state.conversion.hint!==(state.mode==='String'?'string':'number'))return 'constructor conversion retains its actual numeric or text hint';}else if(state.phase==='convert')return 'constructor conversion retains its actual suspended primitive protocol';
 if(state.call!==undefined){
  const call=state.call;if(call.$!=='constructionCallRecord'||call.callee.$!=='object'||!call.callee.callable||call.arguments.length>1048576||!['prototype getter','convert','body'].includes(state.phase)||typeof call.constructing!=='boolean')return 'construction retains its actual selected getter conversion or body call';
  for(const value of [call.callee,call.receiver,call.newTarget,...call.arguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
  if(state.phase==='body'){
   const fn=readJavaScriptExecutableCallFunction(program,heap,state.target.identity),collection=testJavaScriptConstructedCollectionKind(state.mode);if(collection&&(fn===undefined||readJavaScriptCollectionConstructorKind(fn)!==state.mode))return 'collection construction retains its actual protected constructor code kind';
   if(!['strict','ordinary class','Map','Set','WeakMap','WeakSet'].includes(state.mode)||!call.constructing||!testJavaScriptSemanticSameValue(call.callee,state.target)||!testJavaScriptSemanticSameValue(call.receiver,state.receiver)||!testJavaScriptSemanticSameValue(call.newTarget,state.newTarget)||call.arguments.length!==state.arguments.length)return 'base construction calls retain actual target newTarget receiver and all arguments';
   for(let index=0;index<state.arguments.length;index+=1)if(!testJavaScriptSemanticSameValue(call.arguments[index],state.arguments[index]))return 'constructor body arguments retain their original ordered values';
  }else if(call.constructing||call.newTarget.$!=='undefined'||state.phase==='prototype getter'&&(!testJavaScriptSemanticSameValue(call.receiver,state.newTarget)||call.arguments.length!==0))return 'constructor setup calls retain their actual ordinary Call receiver and operands';
 }else if(state.phase==='prototype getter'||state.phase==='body')return 'constructor getter and body states retain their actual selected child call';
 return '';
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} newTarget @param {JavaScriptSemanticValue[]} arguments_ @returns {JavaScriptConstructionTransition} */
export function beginJavaScriptConstruction(program,heap,directory,target,newTarget,arguments_){
 const state={$:'constructionState',target,newTarget,arguments:arguments_.slice(),phase:'target',mode:'',prototype:{$:'undefined'},receiver:{$:'undefined'},primitive:{$:'undefined'},arrayIndex:0,allocationKind:''},error=readJavaScriptConstructionStateError(program,heap,directory,state);return error===''?{$:'constructionContinue',heap,state}:{$:'constructionInvalid',heap,reason:error};
}
/** @param {JavaScriptConstructionState} state @returns {JavaScriptConstructionState} */
export function clearJavaScriptConstructionChild(state){const {call,conversion,...base}=state;return base;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptConstructionState} state @param {string} operation @param {string} reason @returns {JavaScriptConstructionTransition} */
export function requestJavaScriptConstructionImplementation(heap,state,operation,reason){return {$:'constructionNeedImplementation',heap,state,constructor:state.target,newTarget:state.newTarget,arguments:state.arguments,operation,reason};}
/** @param {JavaScriptConstructionState} state @returns {string} */
export function readJavaScriptConstructionConversionOperation(state){return state.mode==='String'?'convert primitive text':'convert explicit number';}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @param {JavaScriptScalarConversionTransition} transition @returns {JavaScriptConstructionTransition} */
export function acceptJavaScriptConstructionConversion(heap,directory,state,transition){
 const base=clearJavaScriptConstructionChild(state);
 switch(transition.$){
  case 'scalarContinuation':return {$:'constructionContinue',heap,state:{...base,phase:'convert',conversion:transition.state}};
  case 'scalarCall':{const call={$:'constructionCallRecord',callee:transition.callee,receiver:transition.receiver,arguments:transition.arguments,constructing:false,newTarget:{$:'undefined'}};return {$:'constructionCall',heap,state:{...base,phase:'convert',conversion:transition.state,call},callee:call.callee,receiver:call.receiver,arguments:call.arguments,constructing:false,newTarget:call.newTarget};}
  case 'scalarCompleted':return {$:'constructionContinue',heap,state:{...base,phase:'prototype',primitive:transition.value,allocationKind:'wrapper'}};
  case 'scalarThrow':return {$:'constructionThrow',heap,value:transition.value};
  case 'scalarIntrinsicError':{const fact=readJavaScriptConstructionFact(directory,state.target);return fact===undefined?{$:'constructionInvalid',heap,reason:'constructor conversion retains its actual intrinsic realm fact'}:{$:'constructionError',heap,errorClass:transition.errorClass,reason:transition.reason,realm:fact.realm};}
  case 'scalarInvalid':return {$:'constructionInvalid',heap,reason:transition.reason};
  case 'scalarResourceLimit':return {$:'constructionResourceLimit',heap,state:{...base,phase:'converted primitive',primitive:transition.value},reason:transition.reason};
 }
}
/** Select the correct fallback from newTarget's realm, independently of the
 * target constructor realm. Actual Object self-construction uses its own realm.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @param {boolean} objectSelf @returns {JavaScriptConstructionTransition} */
export function selectJavaScriptConstructionRoot(heap,directory,state,objectSelf=false){
 const candidate=objectSelf?state.target:state.newTarget,fact=readJavaScriptConstructionFact(directory,candidate),root=fact===undefined?undefined:(directory.constructorRealms??[]).find(row=>row.realm===fact.realm);
 if(root===undefined)return {$:'constructionNeedAuthority',heap,state,constructor:candidate};
 let prototype=root.objectPrototype;
 if(objectSelf){const input=state.arguments[0]??{$:'undefined'};prototype=input.$==='text'||input.$==='sourceText'?root.stringPrototype:input.$==='number'?root.numberPrototype:input.$==='boolean'?root.booleanPrototype:input.$==='integer'?root.integerPrototype:input.$==='symbol'?root.symbolPrototype:root.objectPrototype;}
 else prototype=state.mode==='Array'?root.arrayPrototype:state.mode==='Number'?root.numberPrototype:state.mode==='Boolean'?root.booleanPrototype:state.mode==='String'?root.stringPrototype:state.mode==='Map'?root.mapPrototype:state.mode==='Set'?root.setPrototype:state.mode==='WeakMap'?root.weakMapPrototype:state.mode==='WeakSet'?root.weakSetPrototype:root.objectPrototype;
 if(prototype===undefined)return {$:'constructionNeedAuthority',heap,state,constructor:candidate};
 return {$:'constructionContinue',heap,state:{...state,phase:'allocate',prototype}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @param {JavaScriptSemanticValue} value @returns {JavaScriptConstructionTransition} */
export function continueJavaScriptConstructionPrototype(heap,directory,state,value){const base={...clearJavaScriptConstructionChild(state),prototype:value,phase:value.$==='object'?'allocate':'prototype fallback'};return value.$==='object'?{$:'constructionContinue',heap,state:base}:selectJavaScriptConstructionRoot(heap,directory,base);}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @returns {JavaScriptConstructionTransition} */
export function stepJavaScriptConstruction(program,heap,directory,state){
 const error=readJavaScriptConstructionStateError(program,heap,directory,state);if(error!=='')return {$:'constructionInvalid',heap,reason:error};
 if(state.arguments.length>4096)return {$:'constructionResourceLimit',heap,state,reason:'construction exceeds the selected 4096 argument work profile'};
 if(state.call!==undefined)return {$:'constructionCall',heap,state,callee:state.call.callee,receiver:state.call.receiver,arguments:state.call.arguments,constructing:state.call.constructing,newTarget:state.call.newTarget};
 if(state.phase==='target'||state.phase==='newTarget'){
  const candidate=state.phase==='target'?state.target:state.newTarget;if(candidate.$!=='object'||!candidate.callable)return {$:'constructionError',heap,errorClass:'TypeError',reason:state.phase==='target'?'target is not a constructor':'newTarget is not a constructor'};
  const classification=classifyJavaScriptArraySpeciesConstructor(heap,directory,candidate);if(classification.$==='arrayConstructorNotConstructable')return {$:'constructionError',heap,errorClass:'TypeError',reason:state.phase==='target'?'target is not a constructor':'newTarget is not a constructor'};const fact=classification.$==='arrayConstructorKnown'?classification.fact:undefined;if(fact===undefined)return {$:'constructionNeedAuthority',heap,state,constructor:candidate};if(!fact.constructable)return {$:'constructionError',heap,errorClass:'TypeError',reason:state.phase==='target'?'target is not a constructor':'newTarget is not a constructor'};
  if(state.phase==='newTarget'){
   if(!['strict','ordinary class','Array','Number','Boolean','String','Object','Map','Set','WeakMap','WeakSet'].includes(state.mode))return requestJavaScriptConstructionImplementation(heap,state,'Construct','the actual constructor kind requires its own derived bound or exotic implementation');
   const fn=readJavaScriptExecutableCallFunction(program,heap,fact.identity),kind=readJavaScriptConstructionMode(program,heap,fact);
   if(fact.intrinsicKind!==undefined&&!['Array','Object','Number','Boolean','String','Map','Set','WeakMap','WeakSet'].includes(kind)||fact.intrinsicKind===undefined&&!fact.intrinsicArray&&fn!==undefined&&!['strict','ordinary class','primitive intrinsic'].includes(fn.receiverMode))return requestJavaScriptConstructionImplementation(heap,state,'newTarget prototype and realm','the actual newTarget kind requires its bound derived or Proxy prototype/realm protocol');
  }
  return {$:'constructionContinue',heap,state:state.phase==='target'?{...state,phase:'newTarget',mode:readJavaScriptConstructionMode(program,heap,fact)}:{...state,phase:'initialize'}};
 }
 if(!['strict','ordinary class','Array','Number','Boolean','String','Object','Map','Set','WeakMap','WeakSet'].includes(state.mode))return requestJavaScriptConstructionImplementation(heap,state,'Construct','the actual constructor kind requires its own derived bound or exotic implementation');
 if(state.phase==='initialize'){
  if(testJavaScriptConstructedCollectionKind(state.mode)){const fn=readJavaScriptExecutableCallFunction(program,heap,state.target.identity);if(fn===undefined)return requestJavaScriptConstructionImplementation(heap,state,'collection initialization','the actual collection constructor requires its complete registered portable iterable initializer body');if(readJavaScriptCollectionConstructorKind(fn)!==state.mode)return {$:'constructionInvalid',heap,reason:'collection constructor facts and actual initializer code retain the same intrinsic kind'};}
  const input=state.arguments[0]??{$:'undefined'};
  if(state.mode==='Object'&&testJavaScriptSemanticSameValue(state.target,state.newTarget)){
   if(input.$==='object')return {$:'constructionCompleted',heap,value:input};return selectJavaScriptConstructionRoot(heap,directory,{...state,phase:'Object root',primitive:input,allocationKind:'wrapper'},true);
  }
  if(state.mode==='Number'||state.mode==='String')return state.arguments.length===0?{$:'constructionContinue',heap,state:{...state,phase:'prototype',allocationKind:'wrapper',primitive:state.mode==='Number'?{$:'number',value:0}:{$:'text',units:[]}}}:acceptJavaScriptConstructionConversion(heap,directory,state,beginJavaScriptScalarProtocol(heap,input,program.exoticKey,readJavaScriptConstructionConversionOperation(state)));
  if(state.mode==='Boolean')return {$:'constructionContinue',heap,state:{...state,phase:'prototype',allocationKind:'wrapper',primitive:{$:'boolean',value:convertJavaScriptSemanticBoolean(input)}}};
  return {$:'constructionContinue',heap,state:{...state,phase:'prototype',allocationKind:testJavaScriptConstructedCollectionKind(state.mode)?readJavaScriptConstructedCollectionAllocationKind(state.mode):state.mode==='Array'?'array':'ordinary'}};
 }
 if(state.phase==='Object root')return selectJavaScriptConstructionRoot(heap,directory,state,true);
 if(state.phase==='prototype fallback')return selectJavaScriptConstructionRoot(heap,directory,state);
 if(state.phase==='converted primitive')return acceptJavaScriptConstructionConversion(heap,directory,state,finishJavaScriptScalarPrimitive(readJavaScriptConstructionConversionOperation(state),state.primitive));
 if(state.phase==='convert'&&state.conversion!==undefined)return acceptJavaScriptConstructionConversion(heap,directory,state,stepJavaScriptScalarProtocol(heap,state.conversion,readJavaScriptConstructionConversionOperation(state)));
 if(state.phase==='prototype'){
  if(state.newTarget.$!=='object')return {$:'constructionInvalid',heap,reason:'construction prototype Get retains its actual newTarget'};
  const get=planJavaScriptOrdinaryGet(heap,state.newTarget.identity,{$:'text',units:[112,114,111,116,111,116,121,112,101]},state.newTarget);if(get.error!=='')return requestJavaScriptConstructionImplementation(heap,state,'newTarget prototype Get',get.error);
  if(get.action==='value')return continueJavaScriptConstructionPrototype(heap,directory,state,get.value);
  if(get.action!=='call')return {$:'constructionInvalid',heap,reason:'construction prototype Get retains its actual value or getter'};
  const call={$:'constructionCallRecord',callee:get.callee,receiver:get.receiver,arguments:get.arguments,constructing:false,newTarget:{$:'undefined'}};return {$:'constructionCall',heap,state:{...state,phase:'prototype getter',call},callee:call.callee,receiver:call.receiver,arguments:call.arguments,constructing:false,newTarget:call.newTarget};
 }
 if(state.phase==='allocate'){
  if(state.allocationKind==='wrapper'){
   const request=beginJavaScriptPrimitiveWrapper(heap,state.primitive,state.prototype);if(request.$==='primitiveWrapperResourceLimit')return {$:'constructionResourceLimit',heap:request.heap,state,reason:request.reason};if(request.$!=='primitiveWrapperAllocate')return {$:'constructionInvalid',heap:request.heap,reason:request.$==='primitiveWrapperInvalid'?request.reason:'construction wrapper retains its actual pending allocation'};
  }else {let properties=0;for(const object of heap.objects)properties+=object.properties.length;if(heap.objects.length>=4096||state.allocationKind==='array'&&properties>=16384)return {$:'constructionResourceLimit',heap,state,reason:'construction allocation exceeds the shared Object and property budget'};}
  return {$:'constructionAllocate',heap,state,prototype:state.prototype};
 }
 if(state.phase==='array length'){
  const length=state.arguments[0]??{$:'undefined'};if(length.$!=='number'||state.receiver.$!=='object')return {$:'constructionInvalid',heap,reason:'single numeric Array construction retains its actual Number input and allocated receiver'};const unsigned=normalizeJavaScriptUnsignedInteger(length.value);if(unsigned!==length.value){const fact=readJavaScriptConstructionFact(directory,state.target);return fact===undefined?{$:'constructionInvalid',heap,reason:'Array construction retains its actual constructor realm'}:{$:'constructionError',heap,errorClass:'RangeError',reason:'Invalid array length',realm:fact.realm};}
  const descriptor=createJavaScriptPropertyDescriptor('data',false,false);descriptor.value={$:'number',value:unsigned};descriptor.writable=true;const defined=defineJavaScriptOrdinaryProperty(heap,state.receiver.identity,{$:'text',units:[108,101,110,103,116,104]},descriptor);return defined.success&&defined.error===''?{$:'constructionCompleted',heap:defined.heap,value:state.receiver}:{$:'constructionInvalid',heap:defined.heap,reason:defined.error||'fresh Array length initialization must succeed'};
 }
 if(state.phase==='array elements'){
  if(state.arrayIndex===state.arguments.length)return {$:'constructionCompleted',heap,value:state.receiver};if(state.receiver.$!=='object')return {$:'constructionInvalid',heap,reason:'Array element initialization retains its actual receiver'};
  const spelling=String(state.arrayIndex),units=[];for(let index=0;index<spelling.length;index+=1)units.push(spelling.charCodeAt(index));const key={$:'text',units};if(!testJavaScriptReflectionPropertyBudget(heap,state.receiver,key))return {$:'constructionResourceLimit',heap,state,reason:'Array construction elements exceed the actual shared property budget'};
  const defined=defineJavaScriptOrdinaryProperty(heap,state.receiver.identity,key,createJavaScriptOrdinarySetDescriptor(state.arguments[state.arrayIndex]??{$:'undefined'},true));return defined.success&&defined.error===''?{$:'constructionContinue',heap:defined.heap,state:{...state,arrayIndex:state.arrayIndex+1}}:{$:'constructionInvalid',heap:defined.heap,reason:defined.error||'fresh Array element creation must succeed'};
 }
 return {$:'constructionInvalid',heap,reason:'construction retains its actual selected body or setup continuation'};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @param {bigint} identity @returns {JavaScriptConstructionTransition} */
export function resumeJavaScriptConstructionAllocation(program,heap,directory,state,identity){
 const error=readJavaScriptConstructionStateError(program,heap,directory,state);if(error!==''||state.phase!=='allocate')return {$:'constructionInvalid',heap,reason:error||'construction identity responds only to its actual allocation request'};
 const request=stepJavaScriptConstruction(program,heap,directory,state);if(request.$==='constructionResourceLimit')return request;if(request.$!=='constructionAllocate')return {$:'constructionInvalid',heap,reason:'construction identity cannot replace another source operation'};
 if(state.allocationKind==='wrapper'){
  const created=resumeJavaScriptPrimitiveWrapper(heap,{$:'primitiveWrapperState',input:state.primitive,prototype:state.prototype},identity);if(created.$==='primitiveWrapperCompleted')return {$:'constructionCompleted',heap:created.heap,value:created.value};return created.$==='primitiveWrapperResourceLimit'?{$:'constructionResourceLimit',heap:created.heap,state,reason:created.reason}:{$:'constructionInvalid',heap:created.heap,reason:created.$==='primitiveWrapperInvalid'?created.reason:'wrapper allocation retains its actual completion'};
 }
 if(state.allocationKind==='array'){
  const created=allocateJavaScriptArray(heap,identity,state.prototype,state.arguments.length>=2?state.arguments.length:0);if(created.$!=='arrayNormal')return created.$==='arrayResourceLimit'?{$:'constructionResourceLimit',heap:created.heap,state,reason:created.reason}:{$:'constructionInvalid',heap:created.heap,reason:created.$==='arrayInvalid'||created.$==='arrayError'?created.reason:'Array allocation retains its actual creation completion'};
  return state.arguments.length===0?{$:'constructionCompleted',heap:created.heap,value:created.value}:{$:'constructionContinue',heap:created.heap,state:{...state,receiver:created.value,phase:state.arguments.length===1&&state.arguments[0]?.$==='number'?'array length':'array elements'}};
 }
 if(state.allocationKind==='collection'||state.allocationKind==='weak collection'){
  const created=(state.allocationKind==='weak collection'?allocateJavaScriptWeakCollection:allocateJavaScriptCollection)(heap,identity,state.prototype,state.mode);if(created.$!=='collectionNormal')return created.$==='collectionResourceLimit'?{$:'constructionResourceLimit',heap:created.heap,state,reason:created.reason}:{$:'constructionInvalid',heap:created.heap,reason:created.reason};
  const call={$:'constructionCallRecord',callee:state.target,receiver:created.value,arguments:state.arguments.slice(),constructing:true,newTarget:state.newTarget};return {$:'constructionContinue',heap:created.heap,state:{...state,phase:'body',receiver:created.value,call}};
 }
 const created=insertJavaScriptOrdinaryObject(heap,identity,state.prototype);if(!created.success)return {$:'constructionInvalid',heap:created.heap,reason:created.error};const receiver={$:'object',identity,callable:false};
 if(state.mode==='Object')return {$:'constructionCompleted',heap:created.heap,value:receiver};
 const call={$:'constructionCallRecord',callee:state.target,receiver,arguments:state.arguments.slice(),constructing:true,newTarget:state.newTarget};return {$:'constructionContinue',heap:created.heap,state:{...state,phase:'body',receiver,call}};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptArraySpeciesDirectory} directory @param {JavaScriptConstructionState} state @param {JavaScriptPrimitiveConversionCompletion} completion @returns {JavaScriptConstructionTransition} */
export function resumeJavaScriptConstructionCall(program,heap,directory,state,completion){
 const error=readJavaScriptConstructionStateError(program,heap,directory,state)||readJavaScriptOrdinaryValueError(heap,completion.value);if(error!==''||state.call===undefined)return {$:'constructionInvalid',heap,reason:error||'construction resumes only its selected actual child call'};
 if(completion.$==='throwCompletion')return {$:'constructionThrow',heap,value:completion.value};
 if(state.phase==='convert'&&state.conversion!==undefined)return acceptJavaScriptConstructionConversion(heap,directory,state,resumeJavaScriptScalarProtocol(heap,state.conversion,completion,readJavaScriptConstructionConversionOperation(state)));
 if(state.phase==='prototype getter')return continueJavaScriptConstructionPrototype(heap,directory,state,completion.value);
 if(state.phase==='body')return {$:'constructionCompleted',heap,value:completion.value.$==='object'?completion.value:state.receiver};
 return {$:'constructionInvalid',heap,reason:'construction callback completion retains its actual body prototype or primitive conversion phase'};
}
