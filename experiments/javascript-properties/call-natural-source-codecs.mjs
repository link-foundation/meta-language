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
export function decodeJavaScriptCallSource(value){switch(value.$){
case "sourceConstant":return {$:"sourceConstant","value":decodeJavaScriptSemanticValue(value["value"])};
case "sourceNewTarget":return {$:"sourceNewTarget"};
case "sourceReceiver":return {$:"sourceReceiver"};
case "sourceArgument":return {$:"sourceArgument","index":scalar(value["index"],"number")};
case "sourceDerivedReceiver":return {$:"sourceDerivedReceiver"};
case "sourceErrorIntrinsic":return {$:"sourceErrorIntrinsic"};
case "sourceConstruct":return {$:"sourceConstruct","callee":decodeJavaScriptCallSource(value["callee"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceConstructErrorClass":return {$:"sourceConstructErrorClass","callee":decodeJavaScriptCallSource(value["callee"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceClassConstructorCall":return {$:"sourceClassConstructorCall","callee":decodeJavaScriptCallSource(value["callee"]),"receiver":decodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceConstructCollection":return {$:"sourceConstructCollection","callee":decodeJavaScriptCallSource(value["callee"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceVariadicCall":return {$:"sourceVariadicCall","callee":decodeJavaScriptCallSource(value["callee"]),"receiver":decodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value)),"mode":decodeModelText(value["mode"]),"receiverAfterArguments":scalar(value["receiverAfterArguments"],"boolean")};
case "sourceArgumentSpread":return {$:"sourceArgumentSpread","value":decodeJavaScriptCallSource(value["value"]),"iteratorKey":decodeJavaScriptCallSource(value["iteratorKey"]),"realm":scalar(value["realm"],"integer")};
case "sourceConstructErrorValue":return {$:"sourceConstructErrorValue","constructor":decodeJavaScriptCallSource(value["constructor"]),"newTarget":decodeJavaScriptCallSource(value["newTarget"]),"prototype":decodeJavaScriptCallSource(value["prototype"]),"message":decodeJavaScriptCallSource(value["message"]),"present":decodeJavaScriptCallSource(value["present"]),"realm":scalar(value["realm"],"integer"),"errorClass":decodeModelText(value["errorClass"]),"bindReceiver":scalar(value["bindReceiver"],"boolean")};
case "sourceThrow":return {$:"sourceThrow","value":decodeJavaScriptCallSource(value["value"])};
case "sourceReturn":return {$:"sourceReturn","value":decodeJavaScriptCallSource(value["value"])};
case "sourceAssignBinding":return {$:"sourceAssignBinding","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"])};
case "sourceImmutableBindingWrite":return {$:"sourceImmutableBindingWrite","value":decodeJavaScriptCallSource(value["value"])};
case "sourceWhile":return {$:"sourceWhile","condition":decodeJavaScriptCallSource(value["condition"]),"body":decodeJavaScriptCallSource(value["body"]),"after":decodeJavaScriptCallSource(value["after"]),...(Object.hasOwn(value,"target")?{"target":(isWireUndefined(value["target"])?undefined:scalar(value["target"],"number"))}:{}),...(Object.hasOwn(value,"update")?{"update":(isWireUndefined(value["update"])?undefined:decodeJavaScriptCallSource(value["update"]))}:{}),...(Object.hasOwn(value,"postTest")?{"postTest":(isWireUndefined(value["postTest"])?undefined:scalar(value["postTest"],"boolean"))}:{}),...(Object.hasOwn(value,"iteration")?{"iteration":(isWireUndefined(value["iteration"])?undefined:decodeJavaScriptCallSource(value["iteration"]))}:{})};
case "sourceCloneBindingCell":return {$:"sourceCloneBindingCell","identity":scalar(value["identity"],"number"),"realm":scalar(value["realm"],"integer"),...(Object.hasOwn(value,"lexical")?{"lexical":(isWireUndefined(value["lexical"])?undefined:scalar(value["lexical"],"boolean"))}:{})};
case "sourceControlJump":return {$:"sourceControlJump","target":scalar(value["target"],"number"),"continuing":scalar(value["continuing"],"boolean")};
case "sourceControlBlock":return {$:"sourceControlBlock","target":scalar(value["target"],"number"),"body":decodeJavaScriptCallSource(value["body"]),"after":decodeJavaScriptCallSource(value["after"])};
case "sourceSwitch":return {$:"sourceSwitch","target":scalar(value["target"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"clauses":value["clauses"].map(value=>decodeJavaScriptCallSourceSwitchClause(value)),"after":decodeJavaScriptCallSource(value["after"])};
case "sourceBindingScope":return {$:"sourceBindingScope","declarations":value["declarations"].map(value=>decodeJavaScriptCallBindingDeclaration(value)),"initializer":decodeJavaScriptCallSource(value["initializer"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceLexicalScope":return {$:"sourceLexicalScope","identities":value["identities"].map(value=>scalar(value,"number")),...(Object.hasOwn(value,"existing")?{"existing":(isWireUndefined(value["existing"])?undefined:value["existing"].map(value=>scalar(value,"number")))}:{}),...(Object.hasOwn(value,"initializers")?{"initializers":(isWireUndefined(value["initializers"])?undefined:value["initializers"].map(value=>decodeJavaScriptCallCellInitializer(value)))}:{}),"realm":scalar(value["realm"],"integer"),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceWriteExisting":return {$:"sourceWriteExisting","identity":scalar(value["identity"],"number"),"cell":scalar(value["cell"],"boolean"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceInitializeCell":return {$:"sourceInitializeCell","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceInitializeBinding":return {$:"sourceInitializeBinding","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceIteratorBinding":return {$:"sourceIteratorBinding","slot":scalar(value["slot"],"number"),"input":decodeJavaScriptCallSource(value["input"]),"iteratorKey":decodeJavaScriptCallSource(value["iteratorKey"]),"body":decodeJavaScriptCallSource(value["body"]),"realm":scalar(value["realm"],"integer")};
case "sourceIteratorStep":return {$:"sourceIteratorStep","slot":scalar(value["slot"],"number"),"realm":scalar(value["realm"],"integer"),"readValue":scalar(value["readValue"],"boolean")};
case "sourceIteratorRemainder":return {$:"sourceIteratorRemainder","slot":scalar(value["slot"],"number"),"prototype":decodeJavaScriptCallSource(value["prototype"]),"realm":scalar(value["realm"],"integer")};
case "sourceForOf":return {$:"sourceForOf","slot":scalar(value["slot"],"number"),"target":scalar(value["target"],"number"),"binding":scalar(value["binding"],"number"),"mutable":scalar(value["mutable"],"boolean"),"input":decodeJavaScriptCallSource(value["input"]),"iteratorKey":decodeJavaScriptCallSource(value["iteratorKey"]),"body":decodeJavaScriptCallSource(value["body"]),"realm":scalar(value["realm"],"integer"),...(Object.hasOwn(value,"scopeBindings")?{"scopeBindings":(isWireUndefined(value["scopeBindings"])?undefined:value["scopeBindings"].map(value=>scalar(value,"number")))}:{})};
case "sourceIteratorLoop":return {$:"sourceIteratorLoop","slot":scalar(value["slot"],"number"),"target":scalar(value["target"],"number"),"binding":scalar(value["binding"],"number"),"mutable":scalar(value["mutable"],"boolean"),"body":decodeJavaScriptCallSource(value["body"]),"realm":scalar(value["realm"],"integer"),...(Object.hasOwn(value,"scopeBindings")?{"scopeBindings":(isWireUndefined(value["scopeBindings"])?undefined:value["scopeBindings"].map(value=>scalar(value,"number")))}:{})};
case "sourceIteratorClose":return {$:"sourceIteratorClose","slot":scalar(value["slot"],"number"),"realm":scalar(value["realm"],"integer")};
case "sourceFinally":return {$:"sourceFinally","body":decodeJavaScriptCallSource(value["body"]),"finalizer":decodeJavaScriptCallSource(value["finalizer"])};
case "sourceTry":return {$:"sourceTry","identity":scalar(value["identity"],"number"),"body":decodeJavaScriptCallSource(value["body"]),"handler":decodeJavaScriptCallSource(value["handler"])};
case "sourceBinding":return {$:"sourceBinding","identity":scalar(value["identity"],"number")};
case "sourceLet":return {$:"sourceLet","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceLocalLet":return {$:"sourceLocalLet","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"]),"mutable":scalar(value["mutable"],"boolean")};
case "sourceArgumentRest":return {$:"sourceArgumentRest","from":scalar(value["from"],"number"),"prototype":decodeJavaScriptCallSource(value["prototype"]),"realm":scalar(value["realm"],"integer")};
case "sourceParameterRest":return {$:"sourceParameterRest","identity":scalar(value["identity"],"number"),"from":scalar(value["from"],"number"),"prototype":decodeJavaScriptCallSource(value["prototype"]),"realm":scalar(value["realm"],"integer"),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceParameterDefault":return {$:"sourceParameterDefault","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceIf":return {$:"sourceIf","condition":decodeJavaScriptCallSource(value["condition"]),"yes":decodeJavaScriptCallSource(value["yes"]),"no":decodeJavaScriptCallSource(value["no"])};
case "sourceNumberBinary":return {$:"sourceNumberBinary","operator":decodeModelText(value["operator"]),"left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourceSameValue":return {$:"sourceSameValue","left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourceStrictEqual":return {$:"sourceStrictEqual","left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourceSnapshotStrictEqual":return {$:"sourceSnapshotStrictEqual","left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourcePrimitive":return {$:"sourcePrimitive","operation":decodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceRecur":return {$:"sourceRecur","arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceAllocate":return {$:"sourceAllocate","prototype":decodeJavaScriptCallSource(value["prototype"])};
case "sourceClosure":return {$:"sourceClosure","owner":scalar(value["owner"],"integer"),"ordinal":scalar(value["ordinal"],"number"),"captures":value["captures"].map(value=>decodeJavaScriptCallSource(value)),"receiver":decodeJavaScriptCallSource(value["receiver"])};
case "sourceObject":return {$:"sourceObject","prototype":decodeJavaScriptCallSource(value["prototype"]),"properties":value["properties"].map(value=>decodeJavaScriptCallSourceProperty(value)),"realm":scalar(value["realm"],"integer")};
case "sourceArrayAccumulation":return {$:"sourceArrayAccumulation","prototype":decodeJavaScriptCallSource(value["prototype"]),"parts":value["parts"].map(value=>decodeJavaScriptCallArrayPart(value)),"iteratorKey":decodeJavaScriptCallSource(value["iteratorKey"]),"realm":scalar(value["realm"],"integer")};
case "sourceArray":return {$:"sourceArray","prototype":decodeJavaScriptCallSource(value["prototype"]),"length":scalar(value["length"],"number"),"elements":value["elements"].map(value=>decodeJavaScriptCallSourceArrayElement(value)),"realm":scalar(value["realm"],"integer")};
case "sourcePropertyKey":return {$:"sourcePropertyKey","input":decodeJavaScriptCallSource(value["input"]),"realm":scalar(value["realm"],"integer")};
case "sourcePropertyDefinition":return {$:"sourcePropertyDefinition","target":decodeJavaScriptCallSource(value["target"]),"key":decodeJavaScriptCallSource(value["key"]),"attributes":decodeJavaScriptCallSource(value["attributes"]),"realm":scalar(value["realm"],"integer")};
case "sourceSet":return {$:"sourceSet","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"]),"value":decodeJavaScriptCallSource(value["value"])};
case "sourceSnapshotSet":return {$:"sourceSnapshotSet","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"]),"value":decodeJavaScriptCallSource(value["value"])};
case "sourceStringGet":return {$:"sourceStringGet","input":decodeJavaScriptCallSource(value["input"]),"key":decodeJavaScriptCallSource(value["key"]),"prototype":decodeJavaScriptCallSource(value["prototype"])};
case "sourceGet":return {$:"sourceGet","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"]),...(Object.hasOwn(value,"receiver")?{"receiver":(isWireUndefined(value["receiver"])?undefined:decodeJavaScriptCallSource(value["receiver"]))}:{})};
case "sourceInstanceOf":return {$:"sourceInstanceOf","left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"]),"key":decodeJavaScriptCallSource(value["key"])};
case "sourceDelete":return {$:"sourceDelete","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"])};
case "sourceHas":return {$:"sourceHas","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"])};
case "sourceCall":return {$:"sourceCall","callee":decodeJavaScriptCallSource(value["callee"]),"receiver":decodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceSnapshotCall":return {$:"sourceSnapshotCall","callee":decodeJavaScriptCallSource(value["callee"]),"receiver":decodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallSource(value){switch(value.$){
case "sourceConstant":return {$:"sourceConstant","value":encodeJavaScriptSemanticValue(value["value"])};
case "sourceNewTarget":return {$:"sourceNewTarget"};
case "sourceReceiver":return {$:"sourceReceiver"};
case "sourceArgument":return {$:"sourceArgument","index":scalar(value["index"],"number")};
case "sourceDerivedReceiver":return {$:"sourceDerivedReceiver"};
case "sourceErrorIntrinsic":return {$:"sourceErrorIntrinsic"};
case "sourceConstruct":return {$:"sourceConstruct","callee":encodeJavaScriptCallSource(value["callee"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceConstructErrorClass":return {$:"sourceConstructErrorClass","callee":encodeJavaScriptCallSource(value["callee"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceClassConstructorCall":return {$:"sourceClassConstructorCall","callee":encodeJavaScriptCallSource(value["callee"]),"receiver":encodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceConstructCollection":return {$:"sourceConstructCollection","callee":encodeJavaScriptCallSource(value["callee"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceVariadicCall":return {$:"sourceVariadicCall","callee":encodeJavaScriptCallSource(value["callee"]),"receiver":encodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value)),"mode":encodeModelText(value["mode"]),"receiverAfterArguments":scalar(value["receiverAfterArguments"],"boolean")};
case "sourceArgumentSpread":return {$:"sourceArgumentSpread","value":encodeJavaScriptCallSource(value["value"]),"iteratorKey":encodeJavaScriptCallSource(value["iteratorKey"]),"realm":String(value["realm"])};
case "sourceConstructErrorValue":return {$:"sourceConstructErrorValue","constructor":encodeJavaScriptCallSource(value["constructor"]),"newTarget":encodeJavaScriptCallSource(value["newTarget"]),"prototype":encodeJavaScriptCallSource(value["prototype"]),"message":encodeJavaScriptCallSource(value["message"]),"present":encodeJavaScriptCallSource(value["present"]),"realm":String(value["realm"]),"errorClass":encodeModelText(value["errorClass"]),"bindReceiver":scalar(value["bindReceiver"],"boolean")};
case "sourceThrow":return {$:"sourceThrow","value":encodeJavaScriptCallSource(value["value"])};
case "sourceReturn":return {$:"sourceReturn","value":encodeJavaScriptCallSource(value["value"])};
case "sourceAssignBinding":return {$:"sourceAssignBinding","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"])};
case "sourceImmutableBindingWrite":return {$:"sourceImmutableBindingWrite","value":encodeJavaScriptCallSource(value["value"])};
case "sourceWhile":return {$:"sourceWhile","condition":encodeJavaScriptCallSource(value["condition"]),"body":encodeJavaScriptCallSource(value["body"]),"after":encodeJavaScriptCallSource(value["after"]),...(Object.hasOwn(value,"target")?{"target":(value["target"]===undefined?{fieldPresence:"undefined"}:scalar(value["target"],"number"))}:{}),...(Object.hasOwn(value,"update")?{"update":(value["update"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptCallSource(value["update"]))}:{}),...(Object.hasOwn(value,"postTest")?{"postTest":(value["postTest"]===undefined?{fieldPresence:"undefined"}:scalar(value["postTest"],"boolean"))}:{}),...(Object.hasOwn(value,"iteration")?{"iteration":(value["iteration"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptCallSource(value["iteration"]))}:{})};
case "sourceCloneBindingCell":return {$:"sourceCloneBindingCell","identity":scalar(value["identity"],"number"),"realm":String(value["realm"]),...(Object.hasOwn(value,"lexical")?{"lexical":(value["lexical"]===undefined?{fieldPresence:"undefined"}:scalar(value["lexical"],"boolean"))}:{})};
case "sourceControlJump":return {$:"sourceControlJump","target":scalar(value["target"],"number"),"continuing":scalar(value["continuing"],"boolean")};
case "sourceControlBlock":return {$:"sourceControlBlock","target":scalar(value["target"],"number"),"body":encodeJavaScriptCallSource(value["body"]),"after":encodeJavaScriptCallSource(value["after"])};
case "sourceSwitch":return {$:"sourceSwitch","target":scalar(value["target"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"clauses":value["clauses"].map(value=>encodeJavaScriptCallSourceSwitchClause(value)),"after":encodeJavaScriptCallSource(value["after"])};
case "sourceBindingScope":return {$:"sourceBindingScope","declarations":value["declarations"].map(value=>encodeJavaScriptCallBindingDeclaration(value)),"initializer":encodeJavaScriptCallSource(value["initializer"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceLexicalScope":return {$:"sourceLexicalScope","identities":value["identities"].map(value=>scalar(value,"number")),...(Object.hasOwn(value,"existing")?{"existing":(value["existing"]===undefined?{fieldPresence:"undefined"}:value["existing"].map(value=>scalar(value,"number")))}:{}),...(Object.hasOwn(value,"initializers")?{"initializers":(value["initializers"]===undefined?{fieldPresence:"undefined"}:value["initializers"].map(value=>encodeJavaScriptCallCellInitializer(value)))}:{}),"realm":String(value["realm"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceWriteExisting":return {$:"sourceWriteExisting","identity":scalar(value["identity"],"number"),"cell":scalar(value["cell"],"boolean"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceInitializeCell":return {$:"sourceInitializeCell","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceInitializeBinding":return {$:"sourceInitializeBinding","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceIteratorBinding":return {$:"sourceIteratorBinding","slot":scalar(value["slot"],"number"),"input":encodeJavaScriptCallSource(value["input"]),"iteratorKey":encodeJavaScriptCallSource(value["iteratorKey"]),"body":encodeJavaScriptCallSource(value["body"]),"realm":String(value["realm"])};
case "sourceIteratorStep":return {$:"sourceIteratorStep","slot":scalar(value["slot"],"number"),"realm":String(value["realm"]),"readValue":scalar(value["readValue"],"boolean")};
case "sourceIteratorRemainder":return {$:"sourceIteratorRemainder","slot":scalar(value["slot"],"number"),"prototype":encodeJavaScriptCallSource(value["prototype"]),"realm":String(value["realm"])};
case "sourceForOf":return {$:"sourceForOf","slot":scalar(value["slot"],"number"),"target":scalar(value["target"],"number"),"binding":scalar(value["binding"],"number"),"mutable":scalar(value["mutable"],"boolean"),"input":encodeJavaScriptCallSource(value["input"]),"iteratorKey":encodeJavaScriptCallSource(value["iteratorKey"]),"body":encodeJavaScriptCallSource(value["body"]),"realm":String(value["realm"]),...(Object.hasOwn(value,"scopeBindings")?{"scopeBindings":(value["scopeBindings"]===undefined?{fieldPresence:"undefined"}:value["scopeBindings"].map(value=>scalar(value,"number")))}:{})};
case "sourceIteratorLoop":return {$:"sourceIteratorLoop","slot":scalar(value["slot"],"number"),"target":scalar(value["target"],"number"),"binding":scalar(value["binding"],"number"),"mutable":scalar(value["mutable"],"boolean"),"body":encodeJavaScriptCallSource(value["body"]),"realm":String(value["realm"]),...(Object.hasOwn(value,"scopeBindings")?{"scopeBindings":(value["scopeBindings"]===undefined?{fieldPresence:"undefined"}:value["scopeBindings"].map(value=>scalar(value,"number")))}:{})};
case "sourceIteratorClose":return {$:"sourceIteratorClose","slot":scalar(value["slot"],"number"),"realm":String(value["realm"])};
case "sourceFinally":return {$:"sourceFinally","body":encodeJavaScriptCallSource(value["body"]),"finalizer":encodeJavaScriptCallSource(value["finalizer"])};
case "sourceTry":return {$:"sourceTry","identity":scalar(value["identity"],"number"),"body":encodeJavaScriptCallSource(value["body"]),"handler":encodeJavaScriptCallSource(value["handler"])};
case "sourceBinding":return {$:"sourceBinding","identity":scalar(value["identity"],"number")};
case "sourceLet":return {$:"sourceLet","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceLocalLet":return {$:"sourceLocalLet","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"]),"mutable":scalar(value["mutable"],"boolean")};
case "sourceArgumentRest":return {$:"sourceArgumentRest","from":scalar(value["from"],"number"),"prototype":encodeJavaScriptCallSource(value["prototype"]),"realm":String(value["realm"])};
case "sourceParameterRest":return {$:"sourceParameterRest","identity":scalar(value["identity"],"number"),"from":scalar(value["from"],"number"),"prototype":encodeJavaScriptCallSource(value["prototype"]),"realm":String(value["realm"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceParameterDefault":return {$:"sourceParameterDefault","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceIf":return {$:"sourceIf","condition":encodeJavaScriptCallSource(value["condition"]),"yes":encodeJavaScriptCallSource(value["yes"]),"no":encodeJavaScriptCallSource(value["no"])};
case "sourceNumberBinary":return {$:"sourceNumberBinary","operator":encodeModelText(value["operator"]),"left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourceSameValue":return {$:"sourceSameValue","left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourceStrictEqual":return {$:"sourceStrictEqual","left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourceSnapshotStrictEqual":return {$:"sourceSnapshotStrictEqual","left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourcePrimitive":return {$:"sourcePrimitive","operation":encodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceRecur":return {$:"sourceRecur","arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceAllocate":return {$:"sourceAllocate","prototype":encodeJavaScriptCallSource(value["prototype"])};
case "sourceClosure":return {$:"sourceClosure","owner":String(value["owner"]),"ordinal":scalar(value["ordinal"],"number"),"captures":value["captures"].map(value=>encodeJavaScriptCallSource(value)),"receiver":encodeJavaScriptCallSource(value["receiver"])};
case "sourceObject":return {$:"sourceObject","prototype":encodeJavaScriptCallSource(value["prototype"]),"properties":value["properties"].map(value=>encodeJavaScriptCallSourceProperty(value)),"realm":String(value["realm"])};
case "sourceArrayAccumulation":return {$:"sourceArrayAccumulation","prototype":encodeJavaScriptCallSource(value["prototype"]),"parts":value["parts"].map(value=>encodeJavaScriptCallArrayPart(value)),"iteratorKey":encodeJavaScriptCallSource(value["iteratorKey"]),"realm":String(value["realm"])};
case "sourceArray":return {$:"sourceArray","prototype":encodeJavaScriptCallSource(value["prototype"]),"length":scalar(value["length"],"number"),"elements":value["elements"].map(value=>encodeJavaScriptCallSourceArrayElement(value)),"realm":String(value["realm"])};
case "sourcePropertyKey":return {$:"sourcePropertyKey","input":encodeJavaScriptCallSource(value["input"]),"realm":String(value["realm"])};
case "sourcePropertyDefinition":return {$:"sourcePropertyDefinition","target":encodeJavaScriptCallSource(value["target"]),"key":encodeJavaScriptCallSource(value["key"]),"attributes":encodeJavaScriptCallSource(value["attributes"]),"realm":String(value["realm"])};
case "sourceSet":return {$:"sourceSet","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"]),"value":encodeJavaScriptCallSource(value["value"])};
case "sourceSnapshotSet":return {$:"sourceSnapshotSet","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"]),"value":encodeJavaScriptCallSource(value["value"])};
case "sourceStringGet":return {$:"sourceStringGet","input":encodeJavaScriptCallSource(value["input"]),"key":encodeJavaScriptCallSource(value["key"]),"prototype":encodeJavaScriptCallSource(value["prototype"])};
case "sourceGet":return {$:"sourceGet","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"]),...(Object.hasOwn(value,"receiver")?{"receiver":(value["receiver"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptCallSource(value["receiver"]))}:{})};
case "sourceInstanceOf":return {$:"sourceInstanceOf","left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"]),"key":encodeJavaScriptCallSource(value["key"])};
case "sourceDelete":return {$:"sourceDelete","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"])};
case "sourceHas":return {$:"sourceHas","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"])};
case "sourceCall":return {$:"sourceCall","callee":encodeJavaScriptCallSource(value["callee"]),"receiver":encodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceSnapshotCall":return {$:"sourceSnapshotCall","callee":encodeJavaScriptCallSource(value["callee"]),"receiver":encodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
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
export function decodeJavaScriptCallSourceSwitchClause(value){switch(value.$){
case "sourceSwitchCase":return {$:"sourceSwitchCase","value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceSwitchDefault":return {$:"sourceSwitchDefault","body":decodeJavaScriptCallSource(value["body"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallSourceSwitchClause(value){switch(value.$){
case "sourceSwitchCase":return {$:"sourceSwitchCase","value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceSwitchDefault":return {$:"sourceSwitchDefault","body":encodeJavaScriptCallSource(value["body"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallBindingDeclaration(value){switch(value.$){
case "sourceBindingDeclaration":return {$:"sourceBindingDeclaration","identity":scalar(value["identity"],"number"),"mutable":scalar(value["mutable"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallBindingDeclaration(value){switch(value.$){
case "sourceBindingDeclaration":return {$:"sourceBindingDeclaration","identity":scalar(value["identity"],"number"),"mutable":scalar(value["mutable"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallCellInitializer(value){switch(value.$){
case "sourceCellInitializer":return {$:"sourceCellInitializer","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"initialized":scalar(value["initialized"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallCellInitializer(value){switch(value.$){
case "sourceCellInitializer":return {$:"sourceCellInitializer","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"initialized":scalar(value["initialized"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallSourceProperty(value){switch(value.$){
case "sourceObjectProperty":return {$:"sourceObjectProperty","key":decodeJavaScriptCallSource(value["key"]),"value":decodeJavaScriptCallSource(value["value"]),...(Object.hasOwn(value,"inferName")?{"inferName":(isWireUndefined(value["inferName"])?undefined:scalar(value["inferName"],"boolean"))}:{})};
case "sourceObjectAccessor":return {$:"sourceObjectAccessor","key":decodeJavaScriptCallSource(value["key"]),"value":decodeJavaScriptCallSource(value["value"]),"kind":decodeModelText(value["kind"]),...(Object.hasOwn(value,"inferName")?{"inferName":(isWireUndefined(value["inferName"])?undefined:scalar(value["inferName"],"boolean"))}:{})};
case "sourceLiteralPrototype":return {$:"sourceLiteralPrototype","value":decodeJavaScriptCallSource(value["value"])};
case "sourceObjectSpread":return {$:"sourceObjectSpread","input":decodeJavaScriptCallSource(value["input"]),...(Object.hasOwn(value,"excluded")?{"excluded":(isWireUndefined(value["excluded"])?undefined:value["excluded"].map(value=>decodeJavaScriptCallSource(value)))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallSourceProperty(value){switch(value.$){
case "sourceObjectProperty":return {$:"sourceObjectProperty","key":encodeJavaScriptCallSource(value["key"]),"value":encodeJavaScriptCallSource(value["value"]),...(Object.hasOwn(value,"inferName")?{"inferName":(value["inferName"]===undefined?{fieldPresence:"undefined"}:scalar(value["inferName"],"boolean"))}:{})};
case "sourceObjectAccessor":return {$:"sourceObjectAccessor","key":encodeJavaScriptCallSource(value["key"]),"value":encodeJavaScriptCallSource(value["value"]),"kind":encodeModelText(value["kind"]),...(Object.hasOwn(value,"inferName")?{"inferName":(value["inferName"]===undefined?{fieldPresence:"undefined"}:scalar(value["inferName"],"boolean"))}:{})};
case "sourceLiteralPrototype":return {$:"sourceLiteralPrototype","value":encodeJavaScriptCallSource(value["value"])};
case "sourceObjectSpread":return {$:"sourceObjectSpread","input":encodeJavaScriptCallSource(value["input"]),...(Object.hasOwn(value,"excluded")?{"excluded":(value["excluded"]===undefined?{fieldPresence:"undefined"}:value["excluded"].map(value=>encodeJavaScriptCallSource(value)))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallArrayPart(value){switch(value.$){
case "sourceArrayValue":return {$:"sourceArrayValue","value":decodeJavaScriptCallSource(value["value"])};
case "sourceArraySpread":return {$:"sourceArraySpread","value":decodeJavaScriptCallSource(value["value"])};
case "sourceArrayHole":return {$:"sourceArrayHole"};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallArrayPart(value){switch(value.$){
case "sourceArrayValue":return {$:"sourceArrayValue","value":encodeJavaScriptCallSource(value["value"])};
case "sourceArraySpread":return {$:"sourceArraySpread","value":encodeJavaScriptCallSource(value["value"])};
case "sourceArrayHole":return {$:"sourceArrayHole"};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallSourceArrayElement(value){switch(value.$){
case "sourceArrayElement":return {$:"sourceArrayElement","index":scalar(value["index"],"number"),"value":decodeJavaScriptCallSource(value["value"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallSourceArrayElement(value){switch(value.$){
case "sourceArrayElement":return {$:"sourceArrayElement","index":scalar(value["index"],"number"),"value":encodeJavaScriptCallSource(value["value"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallSourceCompilation(value){switch(value.$){
case "sourceCompiled":return {$:"sourceCompiled","instructions":value["instructions"].map(value=>decodeJavaScriptCallInstruction(value)),"registers":scalar(value["registers"],"number"),"error":decodeModelText(value["error"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallSourceCompilation(value){switch(value.$){
case "sourceCompiled":return {$:"sourceCompiled","instructions":value["instructions"].map(value=>encodeJavaScriptCallInstruction(value)),"registers":scalar(value["registers"],"number"),"error":encodeModelText(value["error"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallInstruction(value){switch(value.$){
case "constant":return {$:"constant","destination":scalar(value["destination"],"number"),"value":decodeJavaScriptSemanticValue(value["value"])};
case "copy":return {$:"copy","destination":scalar(value["destination"],"number"),"source":scalar(value["source"],"number")};
case "argument":return {$:"argument","destination":scalar(value["destination"],"number"),"index":scalar(value["index"],"number")};
case "argumentCount":return {$:"argumentCount","destination":scalar(value["destination"],"number")};
case "argumentAt":return {$:"argumentAt","destination":scalar(value["destination"],"number"),"index":scalar(value["index"],"number")};
case "functionBind":return {$:"functionBind","destination":scalar(value["destination"],"number")};
case "boundFunctionCall":return {$:"boundFunctionCall","destination":scalar(value["destination"],"number")};
case "functionToString":return {$:"functionToString","destination":scalar(value["destination"],"number")};
case "pushFinally":return {$:"pushFinally","target":scalar(value["target"],"number")};
case "finishFinally":return {$:"finishFinally"};
case "completeJump":return {$:"completeJump","target":scalar(value["target"],"number"),"depth":scalar(value["depth"],"number")};
case "jump":return {$:"jump","target":scalar(value["target"],"number")};
case "branch":return {$:"branch","condition":scalar(value["condition"],"number"),"yes":scalar(value["yes"],"number"),"no":scalar(value["no"],"number")};
case "call":return {$:"call","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "argumentListOpen":return {$:"argumentListOpen","slot":scalar(value["slot"],"number")};
case "argumentListAppend":return {$:"argumentListAppend","slot":scalar(value["slot"],"number"),"value":scalar(value["value"],"number")};
case "argumentListCall":return {$:"argumentListCall","destination":scalar(value["destination"],"number"),"slot":scalar(value["slot"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"mode":decodeModelText(value["mode"])};
case "construct":return {$:"construct","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"newTarget":scalar(value["newTarget"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "constructorCheck":return {$:"constructorCheck","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"required":scalar(value["required"],"boolean")};
case "readNewTarget":return {$:"readNewTarget","destination":scalar(value["destination"],"number")};
case "arrayJoinEnter":return {$:"arrayJoinEnter","destination":scalar(value["destination"],"number"),"receiver":scalar(value["receiver"],"number")};
case "arrayJoinLeave":return {$:"arrayJoinLeave","receiver":scalar(value["receiver"],"number")};
case "constructErrorClass":return {$:"constructErrorClass","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "callClassConstructor":return {$:"callClassConstructor","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "constructCollection":return {$:"constructCollection","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "collectionAllocate":return {$:"collectionAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"kind":decodeModelText(value["kind"])};
case "collectionIteratorAllocate":return {$:"collectionIteratorAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"kind":decodeModelText(value["kind"]),"iterationKind":decodeModelText(value["iterationKind"]),...(Object.hasOwn(value,"input")?{"input":(isWireUndefined(value["input"])?undefined:scalar(value["input"],"number"))}:{})};
case "argumentRest":return {$:"argumentRest","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"from":scalar(value["from"],"number"),"realm":scalar(value["realm"],"integer")};
case "iteratorProtocol":return {$:"iteratorProtocol","destination":scalar(value["destination"],"number"),"done":scalar(value["done"],"number"),"input":scalar(value["input"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number"),"slot":scalar(value["slot"],"number"),"operation":decodeModelText(value["operation"]),"realm":scalar(value["realm"],"integer"),...(Object.hasOwn(value,"readValue")?{"readValue":(isWireUndefined(value["readValue"])?undefined:scalar(value["readValue"],"boolean"))}:{})};
case "collectionIteratorResult":return {$:"collectionIteratorResult","destination":scalar(value["destination"],"number"),"kind":decodeModelText(value["kind"]),"arrayPrototype":scalar(value["arrayPrototype"],"number"),"objectPrototype":scalar(value["objectPrototype"],"number")};
case "collectionOperation":return {$:"collectionOperation","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"])};
case "primitiveMethod":return {$:"primitiveMethod","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"])};
case "collectionForEach":return {$:"collectionForEach","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"])};
case "arrayConcatenation":return {$:"arrayConcatenation","destination":scalar(value["destination"],"number"),...(Object.hasOwn(value,"input")?{"input":(isWireUndefined(value["input"])?undefined:scalar(value["input"],"number"))}:{}),"spreadableKey":scalar(value["spreadableKey"],"number"),"species":decodeJavaScriptArraySpeciesDirectory(value["species"])};
case "arraySlice":return {$:"arraySlice","destination":scalar(value["destination"],"number"),...(Object.hasOwn(value,"input")?{"input":(isWireUndefined(value["input"])?undefined:scalar(value["input"],"number"))}:{}),"species":decodeJavaScriptArraySpeciesDirectory(value["species"])};
case "arrayCreation":return {$:"arrayCreation","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"prototype":scalar(value["prototype"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number")};
case "arrayCallback":return {$:"arrayCallback","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),...(Object.hasOwn(value,"input")?{"input":(isWireUndefined(value["input"])?undefined:scalar(value["input"],"number"))}:{})};
case "arraySearch":return {$:"arraySearch","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),...(Object.hasOwn(value,"input")?{"input":(isWireUndefined(value["input"])?undefined:scalar(value["input"],"number"))}:{})};
case "arrayTransform":return {$:"arrayTransform","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"species":decodeJavaScriptArraySpeciesDirectory(value["species"]),...(Object.hasOwn(value,"input")?{"input":(isWireUndefined(value["input"])?undefined:scalar(value["input"],"number"))}:{})};
case "objectEnumeration":return {$:"objectEnumeration","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"prototype":scalar(value["prototype"],"number")};
case "propertyDefinitions":return {$:"propertyDefinitions","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number")};
case "propertyReflection":return {$:"propertyReflection","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"input":scalar(value["input"],"number"),"key":scalar(value["key"],"number"),"objectPrototype":scalar(value["objectPrototype"],"number"),"arrayPrototype":scalar(value["arrayPrototype"],"number")};
case "instanceOf":return {$:"instanceOf","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"key":scalar(value["key"],"number"),"operation":decodeModelText(value["operation"])};
case "objectObservation":return {$:"objectObservation","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"input":scalar(value["input"],"number"),"other":scalar(value["other"],"number")};
case "collectionCopy":return {$:"collectionCopy","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number"),"adder":scalar(value["adder"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number"),"iteratorMethod":scalar(value["iteratorMethod"],"number"),"iteratorPrototype":scalar(value["iteratorPrototype"],"number"),"nextMethod":scalar(value["nextMethod"],"number")};
case "constructErrorValue":return {$:"constructErrorValue","destination":scalar(value["destination"],"number"),"constructor":scalar(value["constructor"],"number"),"newTarget":scalar(value["newTarget"],"number"),"prototype":scalar(value["prototype"],"number"),"message":scalar(value["message"],"number"),"present":scalar(value["present"],"number"),"realm":scalar(value["realm"],"integer"),"errorClass":decodeModelText(value["errorClass"]),"bindReceiver":scalar(value["bindReceiver"],"boolean")};
case "readDerivedReceiver":return {$:"readDerivedReceiver","destination":scalar(value["destination"],"number")};
case "invokeErrorIntrinsic":return {$:"invokeErrorIntrinsic","destination":scalar(value["destination"],"number")};
case "invokeHostCapability":return {$:"invokeHostCapability","destination":scalar(value["destination"],"number")};
case "return":return {$:"return","source":scalar(value["source"],"number")};
case "throw":return {$:"throw","source":scalar(value["source"],"number")};
case "pushHandler":return {$:"pushHandler","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number")};
case "popHandler":return {$:"popHandler"};
case "numberBinary":return {$:"numberBinary","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"operator":decodeModelText(value["operator"])};
case "sameValue":return {$:"sameValue","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "strictEqual":return {$:"strictEqual","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "requireWrite":return {$:"requireWrite","success":scalar(value["success"],"number")};
case "primitive":return {$:"primitive","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "ordinaryAllocate":return {$:"ordinaryAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number")};
case "literalPrototype":return {$:"literalPrototype","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"value":scalar(value["value"],"number")};
case "literalAccessor":return {$:"literalAccessor","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"kind":decodeModelText(value["kind"]),"realm":scalar(value["realm"],"integer")};
case "objectFromEntries":return {$:"objectFromEntries","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"prototype":scalar(value["prototype"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number")};
case "primitiveWrapper":return {$:"primitiveWrapper","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"prototype":scalar(value["prototype"],"number")};
case "symbolAllocate":return {$:"symbolAllocate","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number")};
case "symbolDescription":return {$:"symbolDescription","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number")};
case "symbolRegistry":return {$:"symbolRegistry","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"operation":decodeModelText(value["operation"])};
case "functionName":return {$:"functionName","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"prefix":decodeModelText(value["prefix"]),"realm":scalar(value["realm"],"integer")};
case "arrayAllocate":return {$:"arrayAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"length":scalar(value["length"],"number")};
case "closureAllocate":return {$:"closureAllocate","destination":scalar(value["destination"],"number"),"owner":scalar(value["owner"],"integer"),"ordinal":scalar(value["ordinal"],"number"),"captures":value["captures"].map(value=>scalar(value,"number")),"receiver":scalar(value["receiver"],"number")};
case "arraySpread":return {$:"arraySpread","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number"),"index":scalar(value["index"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number"),"realm":scalar(value["realm"],"integer")};
case "objectSpread":return {$:"objectSpread","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number"),"realm":scalar(value["realm"],"integer"),...(Object.hasOwn(value,"excluded")?{"excluded":(isWireUndefined(value["excluded"])?undefined:value["excluded"].map(value=>scalar(value,"number")))}:{}),...(Object.hasOwn(value,"operation")?{"operation":(isWireUndefined(value["operation"])?undefined:decodeModelText(value["operation"]))}:{})};
case "ordinaryDataProperty":return {$:"ordinaryDataProperty","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"realm":scalar(value["realm"],"integer")};
case "ordinaryGet":return {$:"ordinaryGet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinaryDelete":return {$:"ordinaryDelete","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number")};
case "ordinaryHas":return {$:"ordinaryHas","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number")};
case "ordinarySet":return {$:"ordinarySet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinaryDefine":return {$:"ordinaryDefine","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"fields":value["fields"].map(value=>scalar(value,"number")),"realm":scalar(value["realm"],"integer")};
case "propertyKey":return {$:"propertyKey","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"realm":scalar(value["realm"],"integer")};
case "propertyDefinition":return {$:"propertyDefinition","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"attributes":scalar(value["attributes"],"number"),"realm":scalar(value["realm"],"integer"),...(Object.hasOwn(value,"operation")?{"operation":(isWireUndefined(value["operation"])?undefined:decodeModelText(value["operation"]))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallInstruction(value){switch(value.$){
case "constant":return {$:"constant","destination":scalar(value["destination"],"number"),"value":encodeJavaScriptSemanticValue(value["value"])};
case "copy":return {$:"copy","destination":scalar(value["destination"],"number"),"source":scalar(value["source"],"number")};
case "argument":return {$:"argument","destination":scalar(value["destination"],"number"),"index":scalar(value["index"],"number")};
case "argumentCount":return {$:"argumentCount","destination":scalar(value["destination"],"number")};
case "argumentAt":return {$:"argumentAt","destination":scalar(value["destination"],"number"),"index":scalar(value["index"],"number")};
case "functionBind":return {$:"functionBind","destination":scalar(value["destination"],"number")};
case "boundFunctionCall":return {$:"boundFunctionCall","destination":scalar(value["destination"],"number")};
case "functionToString":return {$:"functionToString","destination":scalar(value["destination"],"number")};
case "pushFinally":return {$:"pushFinally","target":scalar(value["target"],"number")};
case "finishFinally":return {$:"finishFinally"};
case "completeJump":return {$:"completeJump","target":scalar(value["target"],"number"),"depth":scalar(value["depth"],"number")};
case "jump":return {$:"jump","target":scalar(value["target"],"number")};
case "branch":return {$:"branch","condition":scalar(value["condition"],"number"),"yes":scalar(value["yes"],"number"),"no":scalar(value["no"],"number")};
case "call":return {$:"call","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "argumentListOpen":return {$:"argumentListOpen","slot":scalar(value["slot"],"number")};
case "argumentListAppend":return {$:"argumentListAppend","slot":scalar(value["slot"],"number"),"value":scalar(value["value"],"number")};
case "argumentListCall":return {$:"argumentListCall","destination":scalar(value["destination"],"number"),"slot":scalar(value["slot"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"mode":encodeModelText(value["mode"])};
case "construct":return {$:"construct","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"newTarget":scalar(value["newTarget"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "constructorCheck":return {$:"constructorCheck","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"required":scalar(value["required"],"boolean")};
case "readNewTarget":return {$:"readNewTarget","destination":scalar(value["destination"],"number")};
case "arrayJoinEnter":return {$:"arrayJoinEnter","destination":scalar(value["destination"],"number"),"receiver":scalar(value["receiver"],"number")};
case "arrayJoinLeave":return {$:"arrayJoinLeave","receiver":scalar(value["receiver"],"number")};
case "constructErrorClass":return {$:"constructErrorClass","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "callClassConstructor":return {$:"callClassConstructor","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "constructCollection":return {$:"constructCollection","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "collectionAllocate":return {$:"collectionAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"kind":encodeModelText(value["kind"])};
case "collectionIteratorAllocate":return {$:"collectionIteratorAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"kind":encodeModelText(value["kind"]),"iterationKind":encodeModelText(value["iterationKind"]),...(Object.hasOwn(value,"input")?{"input":(value["input"]===undefined?{fieldPresence:"undefined"}:scalar(value["input"],"number"))}:{})};
case "argumentRest":return {$:"argumentRest","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"from":scalar(value["from"],"number"),"realm":String(value["realm"])};
case "iteratorProtocol":return {$:"iteratorProtocol","destination":scalar(value["destination"],"number"),"done":scalar(value["done"],"number"),"input":scalar(value["input"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number"),"slot":scalar(value["slot"],"number"),"operation":encodeModelText(value["operation"]),"realm":String(value["realm"]),...(Object.hasOwn(value,"readValue")?{"readValue":(value["readValue"]===undefined?{fieldPresence:"undefined"}:scalar(value["readValue"],"boolean"))}:{})};
case "collectionIteratorResult":return {$:"collectionIteratorResult","destination":scalar(value["destination"],"number"),"kind":encodeModelText(value["kind"]),"arrayPrototype":scalar(value["arrayPrototype"],"number"),"objectPrototype":scalar(value["objectPrototype"],"number")};
case "collectionOperation":return {$:"collectionOperation","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"])};
case "primitiveMethod":return {$:"primitiveMethod","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"])};
case "collectionForEach":return {$:"collectionForEach","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"])};
case "arrayConcatenation":return {$:"arrayConcatenation","destination":scalar(value["destination"],"number"),...(Object.hasOwn(value,"input")?{"input":(value["input"]===undefined?{fieldPresence:"undefined"}:scalar(value["input"],"number"))}:{}),"spreadableKey":scalar(value["spreadableKey"],"number"),"species":encodeJavaScriptArraySpeciesDirectory(value["species"])};
case "arraySlice":return {$:"arraySlice","destination":scalar(value["destination"],"number"),...(Object.hasOwn(value,"input")?{"input":(value["input"]===undefined?{fieldPresence:"undefined"}:scalar(value["input"],"number"))}:{}),"species":encodeJavaScriptArraySpeciesDirectory(value["species"])};
case "arrayCreation":return {$:"arrayCreation","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"prototype":scalar(value["prototype"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number")};
case "arrayCallback":return {$:"arrayCallback","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),...(Object.hasOwn(value,"input")?{"input":(value["input"]===undefined?{fieldPresence:"undefined"}:scalar(value["input"],"number"))}:{})};
case "arraySearch":return {$:"arraySearch","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),...(Object.hasOwn(value,"input")?{"input":(value["input"]===undefined?{fieldPresence:"undefined"}:scalar(value["input"],"number"))}:{})};
case "arrayTransform":return {$:"arrayTransform","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"species":encodeJavaScriptArraySpeciesDirectory(value["species"]),...(Object.hasOwn(value,"input")?{"input":(value["input"]===undefined?{fieldPresence:"undefined"}:scalar(value["input"],"number"))}:{})};
case "objectEnumeration":return {$:"objectEnumeration","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"prototype":scalar(value["prototype"],"number")};
case "propertyDefinitions":return {$:"propertyDefinitions","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number")};
case "propertyReflection":return {$:"propertyReflection","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"input":scalar(value["input"],"number"),"key":scalar(value["key"],"number"),"objectPrototype":scalar(value["objectPrototype"],"number"),"arrayPrototype":scalar(value["arrayPrototype"],"number")};
case "instanceOf":return {$:"instanceOf","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"key":scalar(value["key"],"number"),"operation":encodeModelText(value["operation"])};
case "objectObservation":return {$:"objectObservation","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"input":scalar(value["input"],"number"),"other":scalar(value["other"],"number")};
case "collectionCopy":return {$:"collectionCopy","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number"),"adder":scalar(value["adder"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number"),"iteratorMethod":scalar(value["iteratorMethod"],"number"),"iteratorPrototype":scalar(value["iteratorPrototype"],"number"),"nextMethod":scalar(value["nextMethod"],"number")};
case "constructErrorValue":return {$:"constructErrorValue","destination":scalar(value["destination"],"number"),"constructor":scalar(value["constructor"],"number"),"newTarget":scalar(value["newTarget"],"number"),"prototype":scalar(value["prototype"],"number"),"message":scalar(value["message"],"number"),"present":scalar(value["present"],"number"),"realm":String(value["realm"]),"errorClass":encodeModelText(value["errorClass"]),"bindReceiver":scalar(value["bindReceiver"],"boolean")};
case "readDerivedReceiver":return {$:"readDerivedReceiver","destination":scalar(value["destination"],"number")};
case "invokeErrorIntrinsic":return {$:"invokeErrorIntrinsic","destination":scalar(value["destination"],"number")};
case "invokeHostCapability":return {$:"invokeHostCapability","destination":scalar(value["destination"],"number")};
case "return":return {$:"return","source":scalar(value["source"],"number")};
case "throw":return {$:"throw","source":scalar(value["source"],"number")};
case "pushHandler":return {$:"pushHandler","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number")};
case "popHandler":return {$:"popHandler"};
case "numberBinary":return {$:"numberBinary","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"operator":encodeModelText(value["operator"])};
case "sameValue":return {$:"sameValue","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "strictEqual":return {$:"strictEqual","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "requireWrite":return {$:"requireWrite","success":scalar(value["success"],"number")};
case "primitive":return {$:"primitive","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "ordinaryAllocate":return {$:"ordinaryAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number")};
case "literalPrototype":return {$:"literalPrototype","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"value":scalar(value["value"],"number")};
case "literalAccessor":return {$:"literalAccessor","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"kind":encodeModelText(value["kind"]),"realm":String(value["realm"])};
case "objectFromEntries":return {$:"objectFromEntries","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"prototype":scalar(value["prototype"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number")};
case "primitiveWrapper":return {$:"primitiveWrapper","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"prototype":scalar(value["prototype"],"number")};
case "symbolAllocate":return {$:"symbolAllocate","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number")};
case "symbolDescription":return {$:"symbolDescription","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number")};
case "symbolRegistry":return {$:"symbolRegistry","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"operation":encodeModelText(value["operation"])};
case "functionName":return {$:"functionName","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"prefix":encodeModelText(value["prefix"]),"realm":String(value["realm"])};
case "arrayAllocate":return {$:"arrayAllocate","destination":scalar(value["destination"],"number"),"prototype":scalar(value["prototype"],"number"),"length":scalar(value["length"],"number")};
case "closureAllocate":return {$:"closureAllocate","destination":scalar(value["destination"],"number"),"owner":String(value["owner"]),"ordinal":scalar(value["ordinal"],"number"),"captures":value["captures"].map(value=>scalar(value,"number")),"receiver":scalar(value["receiver"],"number")};
case "arraySpread":return {$:"arraySpread","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number"),"index":scalar(value["index"],"number"),"iteratorKey":scalar(value["iteratorKey"],"number"),"realm":String(value["realm"])};
case "objectSpread":return {$:"objectSpread","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"input":scalar(value["input"],"number"),"realm":String(value["realm"]),...(Object.hasOwn(value,"excluded")?{"excluded":(value["excluded"]===undefined?{fieldPresence:"undefined"}:value["excluded"].map(value=>scalar(value,"number")))}:{}),...(Object.hasOwn(value,"operation")?{"operation":(value["operation"]===undefined?{fieldPresence:"undefined"}:encodeModelText(value["operation"]))}:{})};
case "ordinaryDataProperty":return {$:"ordinaryDataProperty","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"realm":String(value["realm"])};
case "ordinaryGet":return {$:"ordinaryGet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinaryDelete":return {$:"ordinaryDelete","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number")};
case "ordinaryHas":return {$:"ordinaryHas","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number")};
case "ordinarySet":return {$:"ordinarySet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinaryDefine":return {$:"ordinaryDefine","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"fields":value["fields"].map(value=>scalar(value,"number")),"realm":String(value["realm"])};
case "propertyKey":return {$:"propertyKey","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"realm":String(value["realm"])};
case "propertyDefinition":return {$:"propertyDefinition","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"attributes":scalar(value["attributes"],"number"),"realm":String(value["realm"]),...(Object.hasOwn(value,"operation")?{"operation":(value["operation"]===undefined?{fieldPresence:"undefined"}:encodeModelText(value["operation"]))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptArraySpeciesDirectory(value){switch(value.$){
case "arraySpeciesDirectory":return {$:"arraySpeciesDirectory","realm":scalar(value["realm"],"integer"),"arrayConstructor":decodeJavaScriptSemanticValue(value["arrayConstructor"]),"arrayPrototype":decodeJavaScriptSemanticValue(value["arrayPrototype"]),"speciesKey":decodeJavaScriptSemanticValue(value["speciesKey"]),"constructors":value["constructors"].map(value=>decodeJavaScriptArrayConstructorFact(value)),...(Object.hasOwn(value,"constructorRealms")?{"constructorRealms":(isWireUndefined(value["constructorRealms"])?undefined:value["constructorRealms"].map(value=>decodeJavaScriptConstructorRealm(value)))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptArraySpeciesDirectory(value){switch(value.$){
case "arraySpeciesDirectory":return {$:"arraySpeciesDirectory","realm":String(value["realm"]),"arrayConstructor":encodeJavaScriptSemanticValue(value["arrayConstructor"]),"arrayPrototype":encodeJavaScriptSemanticValue(value["arrayPrototype"]),"speciesKey":encodeJavaScriptSemanticValue(value["speciesKey"]),"constructors":value["constructors"].map(value=>encodeJavaScriptArrayConstructorFact(value)),...(Object.hasOwn(value,"constructorRealms")?{"constructorRealms":(value["constructorRealms"]===undefined?{fieldPresence:"undefined"}:value["constructorRealms"].map(value=>encodeJavaScriptConstructorRealm(value)))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptArrayConstructorFact(value){switch(value.$){
case "arrayConstructorFact":return {$:"arrayConstructorFact","identity":scalar(value["identity"],"integer"),"constructable":scalar(value["constructable"],"boolean"),"realm":scalar(value["realm"],"integer"),"intrinsicArray":scalar(value["intrinsicArray"],"boolean"),...(Object.hasOwn(value,"intrinsicKind")?{"intrinsicKind":(isWireUndefined(value["intrinsicKind"])?undefined:decodeModelText(value["intrinsicKind"]))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptArrayConstructorFact(value){switch(value.$){
case "arrayConstructorFact":return {$:"arrayConstructorFact","identity":String(value["identity"]),"constructable":scalar(value["constructable"],"boolean"),"realm":String(value["realm"]),"intrinsicArray":scalar(value["intrinsicArray"],"boolean"),...(Object.hasOwn(value,"intrinsicKind")?{"intrinsicKind":(value["intrinsicKind"]===undefined?{fieldPresence:"undefined"}:encodeModelText(value["intrinsicKind"]))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptConstructorRealm(value){switch(value.$){
case "constructorRealm":return {$:"constructorRealm","realm":scalar(value["realm"],"integer"),"objectPrototype":decodeJavaScriptSemanticValue(value["objectPrototype"]),"arrayPrototype":decodeJavaScriptSemanticValue(value["arrayPrototype"]),"arrayConstructor":decodeJavaScriptSemanticValue(value["arrayConstructor"]),...(Object.hasOwn(value,"numberPrototype")?{"numberPrototype":(isWireUndefined(value["numberPrototype"])?undefined:decodeJavaScriptSemanticValue(value["numberPrototype"]))}:{}),...(Object.hasOwn(value,"booleanPrototype")?{"booleanPrototype":(isWireUndefined(value["booleanPrototype"])?undefined:decodeJavaScriptSemanticValue(value["booleanPrototype"]))}:{}),...(Object.hasOwn(value,"stringPrototype")?{"stringPrototype":(isWireUndefined(value["stringPrototype"])?undefined:decodeJavaScriptSemanticValue(value["stringPrototype"]))}:{}),...(Object.hasOwn(value,"integerPrototype")?{"integerPrototype":(isWireUndefined(value["integerPrototype"])?undefined:decodeJavaScriptSemanticValue(value["integerPrototype"]))}:{}),...(Object.hasOwn(value,"symbolPrototype")?{"symbolPrototype":(isWireUndefined(value["symbolPrototype"])?undefined:decodeJavaScriptSemanticValue(value["symbolPrototype"]))}:{}),...(Object.hasOwn(value,"mapPrototype")?{"mapPrototype":(isWireUndefined(value["mapPrototype"])?undefined:decodeJavaScriptSemanticValue(value["mapPrototype"]))}:{}),...(Object.hasOwn(value,"setPrototype")?{"setPrototype":(isWireUndefined(value["setPrototype"])?undefined:decodeJavaScriptSemanticValue(value["setPrototype"]))}:{}),...(Object.hasOwn(value,"weakMapPrototype")?{"weakMapPrototype":(isWireUndefined(value["weakMapPrototype"])?undefined:decodeJavaScriptSemanticValue(value["weakMapPrototype"]))}:{}),...(Object.hasOwn(value,"weakSetPrototype")?{"weakSetPrototype":(isWireUndefined(value["weakSetPrototype"])?undefined:decodeJavaScriptSemanticValue(value["weakSetPrototype"]))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptConstructorRealm(value){switch(value.$){
case "constructorRealm":return {$:"constructorRealm","realm":String(value["realm"]),"objectPrototype":encodeJavaScriptSemanticValue(value["objectPrototype"]),"arrayPrototype":encodeJavaScriptSemanticValue(value["arrayPrototype"]),"arrayConstructor":encodeJavaScriptSemanticValue(value["arrayConstructor"]),...(Object.hasOwn(value,"numberPrototype")?{"numberPrototype":(value["numberPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["numberPrototype"]))}:{}),...(Object.hasOwn(value,"booleanPrototype")?{"booleanPrototype":(value["booleanPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["booleanPrototype"]))}:{}),...(Object.hasOwn(value,"stringPrototype")?{"stringPrototype":(value["stringPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["stringPrototype"]))}:{}),...(Object.hasOwn(value,"integerPrototype")?{"integerPrototype":(value["integerPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["integerPrototype"]))}:{}),...(Object.hasOwn(value,"symbolPrototype")?{"symbolPrototype":(value["symbolPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["symbolPrototype"]))}:{}),...(Object.hasOwn(value,"mapPrototype")?{"mapPrototype":(value["mapPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["mapPrototype"]))}:{}),...(Object.hasOwn(value,"setPrototype")?{"setPrototype":(value["setPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["setPrototype"]))}:{}),...(Object.hasOwn(value,"weakMapPrototype")?{"weakMapPrototype":(value["weakMapPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["weakMapPrototype"]))}:{}),...(Object.hasOwn(value,"weakSetPrototype")?{"weakSetPrototype":(value["weakSetPrototype"]===undefined?{fieldPresence:"undefined"}:encodeJavaScriptSemanticValue(value["weakSetPrototype"]))}:{})};
default:throw new TypeError("unregistered actual model constructor");}}
function isWireUndefined(value){return value!==null&&typeof value==="object"&&Object.keys(value).length===1&&Object.hasOwn(value,"fieldPresence")&&value.fieldPresence==="undefined";}
