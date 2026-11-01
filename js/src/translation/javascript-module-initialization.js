// Shared portable module state. Runtime data is received only AFTER actual IO;
// it is never embedded or frozen by source capture/authoring.
import {readJavaScriptSemanticValueError,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {readJavaScriptModuleInitializationProfileError,testJavaScriptModuleInitializationPhase,readJavaScriptModuleBindingRead} from './javascript-module-initialization-rules.js';
import {testJavaScriptCallCodeCoordinate} from './javascript-call-code.js';
/** @typedef {{$:'moduleDataLoad',binding:number,specifier:string,readerOwner:string} | {$:'moduleConstAlias',binding:number,target:number}} JavaScriptModuleInitializer */
/** @typedef {{$:'moduleInitializationPlan',profile:string,moduleIdentity:string,sourceSha256:string,bindings:number[],names:string[],initializers:JavaScriptModuleInitializer[]}} JavaScriptModuleInitializationPlan */
/** @typedef {{$:'moduleBindingSlot',binding:number,initialized:boolean,value:JavaScriptSemanticValue}} JavaScriptModuleBindingSlot */
/** @typedef {{$:'moduleInitializationState',moduleIdentity:string,sourceSha256:string,moduleURL:string,realm:bigint,phase:string,position:number,slots:JavaScriptModuleBindingSlot[],failure:JavaScriptSemanticValue}} JavaScriptModuleInitializationState */
/** @typedef {{$:'moduleContinue',state:JavaScriptModuleInitializationState} |
 * {$:'moduleReadData',state:JavaScriptModuleInitializationState,binding:number,specifier:string,readerOwner:string} |
 * {$:'moduleTemporalDeadZone',state:JavaScriptModuleInitializationState,binding:number,name:string} |
 * {$:'moduleEvaluated',state:JavaScriptModuleInitializationState} |
 * {$:'moduleAbrupt',state:JavaScriptModuleInitializationState,value:JavaScriptSemanticValue} |
 * {$:'moduleWaiting',state:JavaScriptModuleInitializationState} |
 * {$:'moduleInvalid',state:JavaScriptModuleInitializationState,reason:string}} JavaScriptModuleInitializationTransition */
/** @typedef {{$:'moduleBindingValue',value:JavaScriptSemanticValue} | {$:'moduleBindingUninitialized',binding:number,name:string} | {$:'moduleBindingFailure',value:JavaScriptSemanticValue} | {$:'moduleBindingInvalid',reason:string}} JavaScriptModuleBindingRead */
/** @param {JavaScriptModuleInitializationPlan} plan @returns {string} */
export function readJavaScriptModuleInitializationPlanError(plan){
 const profile=readJavaScriptModuleInitializationProfileError(plan.profile);if(profile!=='')return profile;
 if(plan.moduleIdentity===''||plan.sourceSha256===''||plan.bindings.length>4096||plan.bindings.length!==plan.names.length||plan.initializers.length!==plan.bindings.length)return 'module plans require complete bounded actual source binding owners';
 for(let at=0;at<plan.bindings.length;at++){const binding=plan.bindings[at]??-1,name=plan.names[at]??'';if(!testJavaScriptCallCodeCoordinate(binding,65536)||plan.bindings.indexOf(binding)!==at||name===''||plan.names.indexOf(name)!==at)return 'module const bindings require distinct actual lexical identities and names';}
 const assigned=[];for(const initializer of plan.initializers){if(!plan.bindings.includes(initializer.binding)||assigned.includes(initializer.binding))return 'module initialization order must assign every actual const exactly once';assigned.push(initializer.binding);if(initializer.$==='moduleDataLoad'){if(initializer.specifier===''||initializer.readerOwner==='')return 'a data load retains its actual source URL operand and qualified reader owner';}else if(!plan.bindings.includes(initializer.target))return 'const aliases require an actual owned module binding';}
 return '';
}
/** @param {JavaScriptModuleInitializationPlan} plan @param {JavaScriptModuleInitializationState} state @returns {string} */
export function readJavaScriptModuleInitializationStateError(plan,state){
 const error=readJavaScriptModuleInitializationPlanError(plan);if(error!=='')return error;
 if(state.moduleIdentity!==plan.moduleIdentity||state.sourceSha256!==plan.sourceSha256||state.moduleURL===''||state.realm<=0n||state.realm>18446744073709551615n||!testJavaScriptModuleInitializationPhase(state.phase)||!testJavaScriptCallCodeCoordinate(state.position,plan.initializers.length+1)||state.slots.length!==plan.bindings.length)return 'module states retain actual source, module URL, realm and initialization coordinates';
 if(state.phase==='uninitialized'&&state.position!==0||state.phase==='evaluated'&&state.position!==plan.initializers.length||['loading','temporal dead zone'].includes(state.phase)&&state.position===plan.initializers.length)return 'module phases must agree with their actual source initialization position';
 for(let at=0;at<state.slots.length;at++){const slot=state.slots[at],binding=plan.bindings[at]??-1;if(slot===undefined||slot.binding!==binding||readJavaScriptSemanticValueError(slot.value)!=='')return 'module slots retain their actual owned semantic values';const position=plan.initializers.findIndex(row=>row.binding===binding);if(slot.initialized!==(position<state.position)||!slot.initialized&&slot.value.$!=='undefined')return 'module slot initialization must retain the completed source prefix and temporal dead zones';}
 if(readJavaScriptSemanticValueError(state.failure)!==''||state.phase!=='errored'&&state.failure.$!=='undefined')return 'only a failed module retains its actual cached abrupt value';
 const initializer=plan.initializers[state.position];if(state.phase==='loading'&&initializer?.$!=='moduleDataLoad'||state.phase==='temporal dead zone'&&(initializer?.$!=='moduleConstAlias'||state.slots.some(slot=>slot.binding===initializer.target&&slot.initialized)))return 'module suspension must retain its actual pending load or uninitialized alias';
 return '';
}
/** Requires an actual plan accepted by readJavaScriptModuleInitializationPlanError;
 * public consumers validate before allocating the owned slot prefix.
 * @param {JavaScriptModuleInitializationPlan} plan @param {string} moduleURL @param {bigint} realm @returns {JavaScriptModuleInitializationState} */
export function createJavaScriptModuleInitialization(plan,moduleURL,realm){return {$:'moduleInitializationState',moduleIdentity:plan.moduleIdentity,sourceSha256:plan.sourceSha256,moduleURL,realm,phase:'uninitialized',position:0,slots:plan.bindings.map(binding=>({$:'moduleBindingSlot',binding,initialized:false,value:{$:'undefined'}})),failure:{$:'undefined'}};}
/** @param {JavaScriptModuleInitializationPlan} plan @param {JavaScriptModuleInitializationState} state @returns {JavaScriptModuleInitializationTransition} */
export function stepJavaScriptModuleInitialization(plan,state){
 const error=readJavaScriptModuleInitializationStateError(plan,state);if(error!=='')return {$:'moduleInvalid',state,reason:error};
 if(state.phase==='errored')return {$:'moduleAbrupt',state,value:state.failure};if(state.phase==='evaluated')return {$:'moduleEvaluated',state};if(state.phase==='loading'||state.phase==='temporal dead zone')return {$:'moduleWaiting',state};
 if(state.phase==='uninitialized')return {$:'moduleContinue',state:{...state,phase:'initializing'}};
 if(state.position===plan.initializers.length)return {$:'moduleEvaluated',state:{...state,phase:'evaluated'}};
 const initializer=plan.initializers[state.position];if(initializer===undefined)return {$:'moduleInvalid',state,reason:'the actual module initializer must exist'};
 if(initializer.$==='moduleDataLoad')return {$:'moduleReadData',state:{...state,phase:'loading'},binding:initializer.binding,specifier:initializer.specifier,readerOwner:initializer.readerOwner};
 const target=state.slots.find(slot=>slot.binding===initializer.target);if(target===undefined)return {$:'moduleInvalid',state,reason:'a const alias retains its actual source target'};
 if(!target.initialized)return {$:'moduleTemporalDeadZone',state:{...state,phase:'temporal dead zone'},binding:initializer.target,name:plan.names[plan.bindings.indexOf(initializer.target)]??''};
 const next={...state,position:state.position+1,slots:state.slots.map(slot=>slot.binding===initializer.binding?{...slot,initialized:true,value:target.value}:slot)};
 return {$:'moduleContinue',state:next};
}
/** @param {JavaScriptModuleInitializationPlan} plan @param {JavaScriptModuleInitializationState} state @param {JavaScriptSemanticValue} value @param {boolean} abrupt @returns {JavaScriptModuleInitializationTransition} */
export function resumeJavaScriptModuleInitialization(plan,state,value,abrupt){
 const error=readJavaScriptModuleInitializationStateError(plan,state);if(error!=='')return {$:'moduleInvalid',state,reason:error};
 if(!['loading','temporal dead zone'].includes(state.phase)||readJavaScriptSemanticValueError(value)!=='')return {$:'moduleInvalid',state,reason:'module resumption requires its actual pending IO/TDZ and semantic result'};
 if(abrupt)return {$:'moduleAbrupt',state:{...state,phase:'errored',failure:value},value};
 if(state.phase==='temporal dead zone')return {$:'moduleInvalid',state,reason:'an actual uninitialized lexical read cannot produce a successful value'};
 const initializer=plan.initializers[state.position];if(initializer===undefined)return {$:'moduleInvalid',state,reason:'module load resumption retains its actual source initializer'};
 return {$:'moduleContinue',state:{...state,phase:'initializing',position:state.position+1,slots:state.slots.map(slot=>slot.binding===initializer.binding?{...slot,initialized:true,value}:slot)}};
}
/** @param {JavaScriptModuleInitializationPlan} plan @param {JavaScriptModuleInitializationState} state @param {number} binding @returns {JavaScriptModuleBindingRead} */
export function readJavaScriptModuleBinding(plan,state,binding){
 const error=readJavaScriptModuleInitializationStateError(plan,state);if(error!=='')return {$:'moduleBindingInvalid',reason:error};
 const slot=state.slots.find(slot=>slot.binding===binding);if(slot===undefined)return {$:'moduleBindingInvalid',reason:'a module read requires its actual owned binding'};
 const decision=readJavaScriptModuleBindingRead(state.phase,slot.initialized);if(decision<0)return {$:'moduleBindingFailure',value:state.failure};if(decision===0)return {$:'moduleBindingUninitialized',binding,name:plan.names[plan.bindings.indexOf(binding)]??''};return {$:'moduleBindingValue',value:slot.value};
}
/** State joins never replace handles with structural copies.
 * @param {JavaScriptModuleInitializationState} state @param {number} first @param {number} second @returns {boolean} */
export function testJavaScriptModuleBindingAlias(state,first,second){const left=state.slots.find(slot=>slot.binding===first),right=state.slots.find(slot=>slot.binding===second);return left!==undefined&&right!==undefined&&left.initialized&&right.initialized&&testJavaScriptSemanticSameValue(left.value,right.value);}
