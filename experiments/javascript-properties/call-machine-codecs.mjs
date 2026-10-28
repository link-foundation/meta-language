// Generated from actual checked declarations. Transport only; never semantics.
function scalar(value,kind){if(kind==="unit")return undefined;if(kind==="integer"){if(typeof value!=="string")throw new TypeError("model exact integer");return BigInt(value);}if(typeof value!==kind||kind==="number"&&!Number.isFinite(value))throw new TypeError("model "+kind);return value;}
function fromBits(value){if(typeof value!=="string"||!/^[0-9a-fA-F]{16}$/u.test(value))throw new TypeError("complete model IEEE bits");const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+value));return view.getFloat64(0);}
function toBits(value){if(typeof value!=="number")throw new TypeError("model Number");const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return view.getBigUint64(0).toString(16).padStart(16,"0");}
export function decodeJavaScriptOrdinaryHeap(value){switch(value.$){
case "ordinaryHeap":return {$:"ordinaryHeap","objects":value["objects"].map(value=>decodeJavaScriptOrdinaryObject(value)),"symbols":value["symbols"].map(value=>scalar(value,"integer"))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptOrdinaryHeap(value){switch(value.$){
case "ordinaryHeap":return {$:"ordinaryHeap","objects":value["objects"].map(value=>encodeJavaScriptOrdinaryObject(value)),"symbols":value["symbols"].map(value=>String(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptOrdinaryObject(value){switch(value.$){
case "ordinaryObject":return {$:"ordinaryObject","identity":scalar(value["identity"],"integer"),"prototype":decodeJavaScriptSemanticValue(value["prototype"]),"extensible":scalar(value["extensible"],"boolean"),"callable":scalar(value["callable"],"boolean"),"properties":value["properties"].map(value=>decodeJavaScriptOrdinaryProperty(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptOrdinaryObject(value){switch(value.$){
case "ordinaryObject":return {$:"ordinaryObject","identity":String(value["identity"]),"prototype":encodeJavaScriptSemanticValue(value["prototype"]),"extensible":scalar(value["extensible"],"boolean"),"callable":scalar(value["callable"],"boolean"),"properties":value["properties"].map(value=>encodeJavaScriptOrdinaryProperty(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptSemanticValue(value){switch(value.$){
case "undefined":return {$:"undefined"};
case "null":return {$:"null"};
case "boolean":return {$:"boolean","value":scalar(value["value"],"boolean")};
case "number":return {$:"number","value":fromBits(value["bits"])};
case "integer":return {$:"integer","value":scalar(value["value"],"integer")};
case "text":return {$:"text","units":value["units"].map(value=>scalar(value,"number"))};
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
case "symbol":return {$:"symbol","identity":String(value["identity"])};
case "object":return {$:"object","identity":String(value["identity"]),"callable":scalar(value["callable"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptOrdinaryProperty(value){switch(value.$){
case "ordinaryProperty":return {$:"ordinaryProperty","key":decodeJavaScriptSemanticValue(value["key"]),"descriptor":({"configurable":scalar(value["descriptor"]["configurable"],"boolean"),"configurablePresent":scalar(value["descriptor"]["configurablePresent"],"boolean"),"enumerable":scalar(value["descriptor"]["enumerable"],"boolean"),"enumerablePresent":scalar(value["descriptor"]["enumerablePresent"],"boolean"),"getter":decodeJavaScriptSemanticValue(value["descriptor"]["getter"]),"getterPresent":scalar(value["descriptor"]["getterPresent"],"boolean"),"setter":decodeJavaScriptSemanticValue(value["descriptor"]["setter"]),"setterPresent":scalar(value["descriptor"]["setterPresent"],"boolean"),"value":decodeJavaScriptSemanticValue(value["descriptor"]["value"]),"valuePresent":scalar(value["descriptor"]["valuePresent"],"boolean"),"writable":scalar(value["descriptor"]["writable"],"boolean"),"writablePresent":scalar(value["descriptor"]["writablePresent"],"boolean")})};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptOrdinaryProperty(value){switch(value.$){
case "ordinaryProperty":return {$:"ordinaryProperty","key":encodeJavaScriptSemanticValue(value["key"]),"descriptor":({"configurable":scalar(value["descriptor"]["configurable"],"boolean"),"configurablePresent":scalar(value["descriptor"]["configurablePresent"],"boolean"),"enumerable":scalar(value["descriptor"]["enumerable"],"boolean"),"enumerablePresent":scalar(value["descriptor"]["enumerablePresent"],"boolean"),"getter":encodeJavaScriptSemanticValue(value["descriptor"]["getter"]),"getterPresent":scalar(value["descriptor"]["getterPresent"],"boolean"),"setter":encodeJavaScriptSemanticValue(value["descriptor"]["setter"]),"setterPresent":scalar(value["descriptor"]["setterPresent"],"boolean"),"value":encodeJavaScriptSemanticValue(value["descriptor"]["value"]),"valuePresent":scalar(value["descriptor"]["valuePresent"],"boolean"),"writable":scalar(value["descriptor"]["writable"],"boolean"),"writablePresent":scalar(value["descriptor"]["writablePresent"],"boolean")})};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallProgram(value){switch(value.$){
case "callProgram":return {$:"callProgram","functions":value["functions"].map(value=>decodeJavaScriptCallFunction(value)),"realms":value["realms"].map(value=>scalar(value,"integer")),"exoticKey":decodeJavaScriptSemanticValue(value["exoticKey"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallProgram(value){switch(value.$){
case "callProgram":return {$:"callProgram","functions":value["functions"].map(value=>encodeJavaScriptCallFunction(value)),"realms":value["realms"].map(value=>String(value)),"exoticKey":encodeJavaScriptSemanticValue(value["exoticKey"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallFunction(value){switch(value.$){
case "callFunction":return {$:"callFunction","identity":scalar(value["identity"],"integer"),"realm":scalar(value["realm"],"integer"),"receiverMode":scalar(value["receiverMode"],"string"),"lexicalReceiver":decodeJavaScriptSemanticValue(value["lexicalReceiver"]),"parameters":scalar(value["parameters"],"number"),"registers":scalar(value["registers"],"number"),"captures":value["captures"].map(value=>decodeJavaScriptSemanticValue(value)),"instructions":value["instructions"].map(value=>decodeJavaScriptCallInstruction(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallFunction(value){switch(value.$){
case "callFunction":return {$:"callFunction","identity":String(value["identity"]),"realm":String(value["realm"]),"receiverMode":scalar(value["receiverMode"],"string"),"lexicalReceiver":encodeJavaScriptSemanticValue(value["lexicalReceiver"]),"parameters":scalar(value["parameters"],"number"),"registers":scalar(value["registers"],"number"),"captures":value["captures"].map(value=>encodeJavaScriptSemanticValue(value)),"instructions":value["instructions"].map(value=>encodeJavaScriptCallInstruction(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallInstruction(value){switch(value.$){
case "constant":return {$:"constant","destination":scalar(value["destination"],"number"),"value":decodeJavaScriptSemanticValue(value["value"])};
case "copy":return {$:"copy","destination":scalar(value["destination"],"number"),"source":scalar(value["source"],"number")};
case "argument":return {$:"argument","destination":scalar(value["destination"],"number"),"index":scalar(value["index"],"number")};
case "jump":return {$:"jump","target":scalar(value["target"],"number")};
case "branch":return {$:"branch","condition":scalar(value["condition"],"number"),"yes":scalar(value["yes"],"number"),"no":scalar(value["no"],"number")};
case "call":return {$:"call","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "return":return {$:"return","source":scalar(value["source"],"number")};
case "throw":return {$:"throw","source":scalar(value["source"],"number")};
case "pushHandler":return {$:"pushHandler","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number")};
case "popHandler":return {$:"popHandler"};
case "numberBinary":return {$:"numberBinary","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"operator":scalar(value["operator"],"string")};
case "sameValue":return {$:"sameValue","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "strictEqual":return {$:"strictEqual","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "requireWrite":return {$:"requireWrite","success":scalar(value["success"],"number")};
case "ordinaryGet":return {$:"ordinaryGet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinarySet":return {$:"ordinarySet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"receiver":scalar(value["receiver"],"number")};
case "propertyKey":return {$:"propertyKey","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"realm":scalar(value["realm"],"integer")};
case "propertyDefinition":return {$:"propertyDefinition","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"attributes":scalar(value["attributes"],"number"),"realm":scalar(value["realm"],"integer")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallInstruction(value){switch(value.$){
case "constant":return {$:"constant","destination":scalar(value["destination"],"number"),"value":encodeJavaScriptSemanticValue(value["value"])};
case "copy":return {$:"copy","destination":scalar(value["destination"],"number"),"source":scalar(value["source"],"number")};
case "argument":return {$:"argument","destination":scalar(value["destination"],"number"),"index":scalar(value["index"],"number")};
case "jump":return {$:"jump","target":scalar(value["target"],"number")};
case "branch":return {$:"branch","condition":scalar(value["condition"],"number"),"yes":scalar(value["yes"],"number"),"no":scalar(value["no"],"number")};
case "call":return {$:"call","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "return":return {$:"return","source":scalar(value["source"],"number")};
case "throw":return {$:"throw","source":scalar(value["source"],"number")};
case "pushHandler":return {$:"pushHandler","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number")};
case "popHandler":return {$:"popHandler"};
case "numberBinary":return {$:"numberBinary","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"operator":scalar(value["operator"],"string")};
case "sameValue":return {$:"sameValue","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "strictEqual":return {$:"strictEqual","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "requireWrite":return {$:"requireWrite","success":scalar(value["success"],"number")};
case "ordinaryGet":return {$:"ordinaryGet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinarySet":return {$:"ordinarySet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"receiver":scalar(value["receiver"],"number")};
case "propertyKey":return {$:"propertyKey","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"realm":String(value["realm"])};
case "propertyDefinition":return {$:"propertyDefinition","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"attributes":scalar(value["attributes"],"number"),"realm":String(value["realm"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallMachine(value){switch(value.$){
case "callMachine":return {$:"callMachine","heap":decodeJavaScriptOrdinaryHeap(value["heap"]),"frames":value["frames"].map(value=>decodeJavaScriptCallFrame(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallMachine(value){switch(value.$){
case "callMachine":return {$:"callMachine","heap":encodeJavaScriptOrdinaryHeap(value["heap"]),"frames":value["frames"].map(value=>encodeJavaScriptCallFrame(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallFrame(value){switch(value.$){
case "callFrame":return {$:"callFrame","identity":scalar(value["identity"],"integer"),"position":scalar(value["position"],"number"),"registers":value["registers"].map(value=>decodeJavaScriptSemanticValue(value)),"arguments":value["arguments"].map(value=>decodeJavaScriptSemanticValue(value)),"handlers":value["handlers"].map(value=>decodeJavaScriptCallHandler(value)),"pending":decodeJavaScriptCallPending(value["pending"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallFrame(value){switch(value.$){
case "callFrame":return {$:"callFrame","identity":String(value["identity"]),"position":scalar(value["position"],"number"),"registers":value["registers"].map(value=>encodeJavaScriptSemanticValue(value)),"arguments":value["arguments"].map(value=>encodeJavaScriptSemanticValue(value)),"handlers":value["handlers"].map(value=>encodeJavaScriptCallHandler(value)),"pending":encodeJavaScriptCallPending(value["pending"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallHandler(value){switch(value.$){
case "callHandler":return {$:"callHandler","target":scalar(value["target"],"number"),"destination":scalar(value["destination"],"number")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallHandler(value){switch(value.$){
case "callHandler":return {$:"callHandler","target":scalar(value["target"],"number"),"destination":scalar(value["destination"],"number")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallPending(value){switch(value.$){
case "callIdle":return {$:"callIdle"};
case "waitCall":return {$:"waitCall","destination":scalar(value["destination"],"number")};
case "waitGet":return {$:"waitGet","destination":scalar(value["destination"],"number")};
case "waitSet":return {$:"waitSet","destination":scalar(value["destination"],"number")};
case "waitKey":return {$:"waitKey","destination":scalar(value["destination"],"number"),"realm":scalar(value["realm"],"integer"),"state":decodeJavaScriptPrimitiveConversionState(value["state"])};
case "waitDefinition":return {$:"waitDefinition","destination":scalar(value["destination"],"number"),"realm":scalar(value["realm"],"integer"),"state":decodeJavaScriptPropertyDefinitionState(value["state"])};
case "waitError":return {$:"waitError","realm":scalar(value["realm"],"integer"),"errorClass":scalar(value["errorClass"],"string"),"reason":scalar(value["reason"],"string")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallPending(value){switch(value.$){
case "callIdle":return {$:"callIdle"};
case "waitCall":return {$:"waitCall","destination":scalar(value["destination"],"number")};
case "waitGet":return {$:"waitGet","destination":scalar(value["destination"],"number")};
case "waitSet":return {$:"waitSet","destination":scalar(value["destination"],"number")};
case "waitKey":return {$:"waitKey","destination":scalar(value["destination"],"number"),"realm":String(value["realm"]),"state":encodeJavaScriptPrimitiveConversionState(value["state"])};
case "waitDefinition":return {$:"waitDefinition","destination":scalar(value["destination"],"number"),"realm":String(value["realm"]),"state":encodeJavaScriptPropertyDefinitionState(value["state"])};
case "waitError":return {$:"waitError","realm":String(value["realm"]),"errorClass":scalar(value["errorClass"],"string"),"reason":scalar(value["reason"],"string")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptPrimitiveConversionState(value){switch(value.$){
case "primitiveConversion":return {$:"primitiveConversion","receiver":decodeJavaScriptSemanticValue(value["receiver"]),"exoticKey":decodeJavaScriptSemanticValue(value["exoticKey"]),"hint":scalar(value["hint"],"string"),"phase":scalar(value["phase"],"string")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptPrimitiveConversionState(value){switch(value.$){
case "primitiveConversion":return {$:"primitiveConversion","receiver":encodeJavaScriptSemanticValue(value["receiver"]),"exoticKey":encodeJavaScriptSemanticValue(value["exoticKey"]),"hint":scalar(value["hint"],"string"),"phase":scalar(value["phase"],"string")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptPropertyDefinitionState(value){switch(value.$){
case "propertyDefinitionKey":return {$:"propertyDefinitionKey","target":decodeJavaScriptSemanticValue(value["target"]),"attributes":decodeJavaScriptSemanticValue(value["attributes"]),"conversion":decodeJavaScriptPrimitiveConversionState(value["conversion"])};
case "propertyDefinitionDescriptor":return {$:"propertyDefinitionDescriptor","target":decodeJavaScriptSemanticValue(value["target"]),"key":decodeJavaScriptSemanticValue(value["key"]),"conversion":decodeJavaScriptDescriptorConversionState(value["conversion"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptPropertyDefinitionState(value){switch(value.$){
case "propertyDefinitionKey":return {$:"propertyDefinitionKey","target":encodeJavaScriptSemanticValue(value["target"]),"attributes":encodeJavaScriptSemanticValue(value["attributes"]),"conversion":encodeJavaScriptPrimitiveConversionState(value["conversion"])};
case "propertyDefinitionDescriptor":return {$:"propertyDefinitionDescriptor","target":encodeJavaScriptSemanticValue(value["target"]),"key":encodeJavaScriptSemanticValue(value["key"]),"conversion":encodeJavaScriptDescriptorConversionState(value["conversion"])};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptDescriptorConversionState(value){switch(value.$){
case "descriptorConversion":return {$:"descriptorConversion","receiver":decodeJavaScriptSemanticValue(value["receiver"]),"next":scalar(value["next"],"number"),"descriptor":({"configurable":scalar(value["descriptor"]["configurable"],"boolean"),"configurablePresent":scalar(value["descriptor"]["configurablePresent"],"boolean"),"enumerable":scalar(value["descriptor"]["enumerable"],"boolean"),"enumerablePresent":scalar(value["descriptor"]["enumerablePresent"],"boolean"),"getter":decodeJavaScriptSemanticValue(value["descriptor"]["getter"]),"getterPresent":scalar(value["descriptor"]["getterPresent"],"boolean"),"setter":decodeJavaScriptSemanticValue(value["descriptor"]["setter"]),"setterPresent":scalar(value["descriptor"]["setterPresent"],"boolean"),"value":decodeJavaScriptSemanticValue(value["descriptor"]["value"]),"valuePresent":scalar(value["descriptor"]["valuePresent"],"boolean"),"writable":scalar(value["descriptor"]["writable"],"boolean"),"writablePresent":scalar(value["descriptor"]["writablePresent"],"boolean")}),"pending":scalar(value["pending"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptDescriptorConversionState(value){switch(value.$){
case "descriptorConversion":return {$:"descriptorConversion","receiver":encodeJavaScriptSemanticValue(value["receiver"]),"next":scalar(value["next"],"number"),"descriptor":({"configurable":scalar(value["descriptor"]["configurable"],"boolean"),"configurablePresent":scalar(value["descriptor"]["configurablePresent"],"boolean"),"enumerable":scalar(value["descriptor"]["enumerable"],"boolean"),"enumerablePresent":scalar(value["descriptor"]["enumerablePresent"],"boolean"),"getter":encodeJavaScriptSemanticValue(value["descriptor"]["getter"]),"getterPresent":scalar(value["descriptor"]["getterPresent"],"boolean"),"setter":encodeJavaScriptSemanticValue(value["descriptor"]["setter"]),"setterPresent":scalar(value["descriptor"]["setterPresent"],"boolean"),"value":encodeJavaScriptSemanticValue(value["descriptor"]["value"]),"valuePresent":scalar(value["descriptor"]["valuePresent"],"boolean"),"writable":scalar(value["descriptor"]["writable"],"boolean"),"writablePresent":scalar(value["descriptor"]["writablePresent"],"boolean")}),"pending":scalar(value["pending"],"boolean")};
default:throw new TypeError("unregistered actual model constructor");}}
export function decodeJavaScriptCallMachineTransition(value){switch(value.$){
case "machineContinue":return {$:"machineContinue","machine":decodeJavaScriptCallMachine(value["machine"])};
case "machineCompleted":return {$:"machineCompleted","heap":decodeJavaScriptOrdinaryHeap(value["heap"]),"value":decodeJavaScriptSemanticValue(value["value"])};
case "machineThrow":return {$:"machineThrow","heap":decodeJavaScriptOrdinaryHeap(value["heap"]),"value":decodeJavaScriptSemanticValue(value["value"])};
case "machineAllocateError":return {$:"machineAllocateError","machine":decodeJavaScriptCallMachine(value["machine"]),"realm":scalar(value["realm"],"integer"),"errorClass":scalar(value["errorClass"],"string"),"reason":scalar(value["reason"],"string")};
case "machineInvalid":return {$:"machineInvalid","heap":decodeJavaScriptOrdinaryHeap(value["heap"]),"reason":scalar(value["reason"],"string")};
case "machineResourceLimit":return {$:"machineResourceLimit","machine":decodeJavaScriptCallMachine(value["machine"]),"reason":scalar(value["reason"],"string")};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallMachineTransition(value){switch(value.$){
case "machineContinue":return {$:"machineContinue","machine":encodeJavaScriptCallMachine(value["machine"])};
case "machineCompleted":return {$:"machineCompleted","heap":encodeJavaScriptOrdinaryHeap(value["heap"]),"value":encodeJavaScriptSemanticValue(value["value"])};
case "machineThrow":return {$:"machineThrow","heap":encodeJavaScriptOrdinaryHeap(value["heap"]),"value":encodeJavaScriptSemanticValue(value["value"])};
case "machineAllocateError":return {$:"machineAllocateError","machine":encodeJavaScriptCallMachine(value["machine"]),"realm":String(value["realm"]),"errorClass":scalar(value["errorClass"],"string"),"reason":scalar(value["reason"],"string")};
case "machineInvalid":return {$:"machineInvalid","heap":encodeJavaScriptOrdinaryHeap(value["heap"]),"reason":scalar(value["reason"],"string")};
case "machineResourceLimit":return {$:"machineResourceLimit","machine":encodeJavaScriptCallMachine(value["machine"]),"reason":scalar(value["reason"],"string")};
default:throw new TypeError("unregistered actual model constructor");}}
