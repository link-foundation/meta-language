// Generated from actual checked declarations. Transport only; never semantics.
const modelTextWirePolicy={"model":"lossless model text transport 1","tag":"UTF16 code units 1","maximumUnits":16777216};
function encodeModelText(value){
 if(typeof value!=='string')throw new TypeError('model text');
 if(value.length>modelTextWirePolicy.maximumUnits)throw new RangeError('model text unit budget');
 if(value.isWellFormed())return value;
 const units=[];for(let index=0;index<value.length;index+=1)units.push(value.charCodeAt(index));
 return {textEncoding:modelTextWirePolicy.tag,units};
}
function decodeModelText(value){
 if(typeof value==='string'){
  if(value.length>modelTextWirePolicy.maximumUnits)throw new RangeError('model text unit budget');
  return value;
 }
 if(!value||typeof value!=='object'||Array.isArray(value)||Object.keys(value).length!==2||value.textEncoding!==modelTextWirePolicy.tag||!Array.isArray(value.units))throw new TypeError('qualified model text units');
 if(value.units.length>modelTextWirePolicy.maximumUnits)throw new RangeError('model text unit budget');
 let text='';for(const unit of value.units){if(!Number.isInteger(unit)||unit<0||unit>65535)throw new TypeError('model UTF16 unit');text+=String.fromCharCode(unit);}
 return text;
}
function scalar(value,kind){if(kind==="unit")return undefined;if(kind==="integer"){if(typeof value!=="string")throw new TypeError("model exact integer");return BigInt(value);}if(typeof value!==kind||kind==="number"&&!Number.isFinite(value))throw new TypeError("model "+kind);return value;}
function fromBits(value){if(typeof value!=="string"||!/^[0-9a-fA-F]{16}$/u.test(value))throw new TypeError("complete model IEEE bits");const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+value));return view.getFloat64(0);}
function toBits(value){if(typeof value!=="number")throw new TypeError("model Number");const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return view.getBigUint64(0).toString(16).padStart(16,"0");}
export function decodeJavaScriptModuleInitializationPlan(value){switch(value.$){
case "moduleInitializationPlan":return {$:"moduleInitializationPlan","profile":decodeModelText(value["profile"]),"moduleIdentity":decodeModelText(value["moduleIdentity"]),"sourceSha256":decodeModelText(value["sourceSha256"]),"bindings":value["bindings"].map(value=>scalar(value,"number")),"names":value["names"].map(value=>decodeModelText(value)),"initializers":value["initializers"].map(value=>decodeJavaScriptModuleInitializer(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptModuleInitializationPlan(value){switch(value.$){
case "moduleInitializationPlan":return {$:"moduleInitializationPlan","profile":encodeModelText(value["profile"]),"moduleIdentity":encodeModelText(value["moduleIdentity"]),"sourceSha256":encodeModelText(value["sourceSha256"]),"bindings":value["bindings"].map(value=>scalar(value,"number")),"names":value["names"].map(value=>encodeModelText(value)),"initializers":value["initializers"].map(value=>encodeJavaScriptModuleInitializer(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptModuleInitializer(value){switch(value.$){
case "moduleDataLoad":return {$:"moduleDataLoad","binding":scalar(value["binding"],"number"),"specifier":decodeModelText(value["specifier"]),"readerOwner":decodeModelText(value["readerOwner"])};
case "moduleConstAlias":return {$:"moduleConstAlias","binding":scalar(value["binding"],"number"),"target":scalar(value["target"],"number")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptModuleInitializer(value){switch(value.$){
case "moduleDataLoad":return {$:"moduleDataLoad","binding":scalar(value["binding"],"number"),"specifier":encodeModelText(value["specifier"]),"readerOwner":encodeModelText(value["readerOwner"])};
case "moduleConstAlias":return {$:"moduleConstAlias","binding":scalar(value["binding"],"number"),"target":scalar(value["target"],"number")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptModuleInitializationState(value){switch(value.$){
case "moduleInitializationState":return {$:"moduleInitializationState","moduleIdentity":decodeModelText(value["moduleIdentity"]),"sourceSha256":decodeModelText(value["sourceSha256"]),"moduleURL":decodeModelText(value["moduleURL"]),"realm":scalar(value["realm"],"integer"),"phase":decodeModelText(value["phase"]),"position":scalar(value["position"],"number"),"slots":value["slots"].map(value=>decodeJavaScriptModuleBindingSlot(value)),"failure":decodeJavaScriptSemanticValue(value["failure"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptModuleInitializationState(value){switch(value.$){
case "moduleInitializationState":return {$:"moduleInitializationState","moduleIdentity":encodeModelText(value["moduleIdentity"]),"sourceSha256":encodeModelText(value["sourceSha256"]),"moduleURL":encodeModelText(value["moduleURL"]),"realm":String(value["realm"]),"phase":encodeModelText(value["phase"]),"position":scalar(value["position"],"number"),"slots":value["slots"].map(value=>encodeJavaScriptModuleBindingSlot(value)),"failure":encodeJavaScriptSemanticValue(value["failure"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptModuleBindingSlot(value){switch(value.$){
case "moduleBindingSlot":return {$:"moduleBindingSlot","binding":scalar(value["binding"],"number"),"initialized":scalar(value["initialized"],"boolean"),"value":decodeJavaScriptSemanticValue(value["value"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptModuleBindingSlot(value){switch(value.$){
case "moduleBindingSlot":return {$:"moduleBindingSlot","binding":scalar(value["binding"],"number"),"initialized":scalar(value["initialized"],"boolean"),"value":encodeJavaScriptSemanticValue(value["value"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptSemanticValue(value){switch(value.$){
case "undefined":return {$:"undefined"};
case "null":return {$:"null"};
case "boolean":return {$:"boolean","value":scalar(value["value"],"boolean")};
case "number":return {$:"number","value":fromBits(value["bits"])};
case "integer":return {$:"integer","value":scalar(value["value"],"integer")};
case "text":return {$:"text","units":value["units"].map(value=>scalar(value,"number"))};
case "sourceText":return {$:"sourceText","profile":decodeModelText(value["profile"]),"text":decodeModelText(value["text"])};
case "symbol":return {$:"symbol","identity":scalar(value["identity"],"integer")};
case "object":return {$:"object","identity":scalar(value["identity"],"integer"),"callable":scalar(value["callable"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptSemanticValue(value){switch(value.$){
case "undefined":return {$:"undefined"};
case "null":return {$:"null"};
case "boolean":return {$:"boolean","value":scalar(value["value"],"boolean")};
case "number":return {$:"number","bits":toBits(value["value"])};
case "integer":return {$:"integer","value":String(value["value"])};
case "text":return {$:"text","units":value["units"].map(value=>scalar(value,"number"))};
case "sourceText":return {$:"sourceText","profile":encodeModelText(value["profile"]),"text":encodeModelText(value["text"])};
case "symbol":return {$:"symbol","identity":String(value["identity"])};
case "object":return {$:"object","identity":String(value["identity"]),"callable":scalar(value["callable"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptModuleInitializationTransition(value){switch(value.$){
case "moduleContinue":return {$:"moduleContinue","state":decodeJavaScriptModuleInitializationState(value["state"])};
case "moduleReadData":return {$:"moduleReadData","state":decodeJavaScriptModuleInitializationState(value["state"]),"binding":scalar(value["binding"],"number"),"specifier":decodeModelText(value["specifier"]),"readerOwner":decodeModelText(value["readerOwner"])};
case "moduleTemporalDeadZone":return {$:"moduleTemporalDeadZone","state":decodeJavaScriptModuleInitializationState(value["state"]),"binding":scalar(value["binding"],"number"),"name":decodeModelText(value["name"])};
case "moduleEvaluated":return {$:"moduleEvaluated","state":decodeJavaScriptModuleInitializationState(value["state"])};
case "moduleAbrupt":return {$:"moduleAbrupt","state":decodeJavaScriptModuleInitializationState(value["state"]),"value":decodeJavaScriptSemanticValue(value["value"])};
case "moduleWaiting":return {$:"moduleWaiting","state":decodeJavaScriptModuleInitializationState(value["state"])};
case "moduleInvalid":return {$:"moduleInvalid","state":decodeJavaScriptModuleInitializationState(value["state"]),"reason":decodeModelText(value["reason"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptModuleInitializationTransition(value){switch(value.$){
case "moduleContinue":return {$:"moduleContinue","state":encodeJavaScriptModuleInitializationState(value["state"])};
case "moduleReadData":return {$:"moduleReadData","state":encodeJavaScriptModuleInitializationState(value["state"]),"binding":scalar(value["binding"],"number"),"specifier":encodeModelText(value["specifier"]),"readerOwner":encodeModelText(value["readerOwner"])};
case "moduleTemporalDeadZone":return {$:"moduleTemporalDeadZone","state":encodeJavaScriptModuleInitializationState(value["state"]),"binding":scalar(value["binding"],"number"),"name":encodeModelText(value["name"])};
case "moduleEvaluated":return {$:"moduleEvaluated","state":encodeJavaScriptModuleInitializationState(value["state"])};
case "moduleAbrupt":return {$:"moduleAbrupt","state":encodeJavaScriptModuleInitializationState(value["state"]),"value":encodeJavaScriptSemanticValue(value["value"])};
case "moduleWaiting":return {$:"moduleWaiting","state":encodeJavaScriptModuleInitializationState(value["state"])};
case "moduleInvalid":return {$:"moduleInvalid","state":encodeJavaScriptModuleInitializationState(value["state"]),"reason":encodeModelText(value["reason"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptModuleBindingRead(value){switch(value.$){
case "moduleBindingValue":return {$:"moduleBindingValue","value":decodeJavaScriptSemanticValue(value["value"])};
case "moduleBindingUninitialized":return {$:"moduleBindingUninitialized","binding":scalar(value["binding"],"number"),"name":decodeModelText(value["name"])};
case "moduleBindingFailure":return {$:"moduleBindingFailure","value":decodeJavaScriptSemanticValue(value["value"])};
case "moduleBindingInvalid":return {$:"moduleBindingInvalid","reason":decodeModelText(value["reason"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptModuleBindingRead(value){switch(value.$){
case "moduleBindingValue":return {$:"moduleBindingValue","value":encodeJavaScriptSemanticValue(value["value"])};
case "moduleBindingUninitialized":return {$:"moduleBindingUninitialized","binding":scalar(value["binding"],"number"),"name":encodeModelText(value["name"])};
case "moduleBindingFailure":return {$:"moduleBindingFailure","value":encodeJavaScriptSemanticValue(value["value"])};
case "moduleBindingInvalid":return {$:"moduleBindingInvalid","reason":encodeModelText(value["reason"])};
default:throw new TypeError("unregistered actual model constructor");}}
