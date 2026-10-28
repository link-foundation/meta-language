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
case "sourceReceiver":return {$:"sourceReceiver"};
case "sourceArgument":return {$:"sourceArgument","index":scalar(value["index"],"number")};
case "sourceThrow":return {$:"sourceThrow","value":decodeJavaScriptCallSource(value["value"])};
case "sourceTry":return {$:"sourceTry","identity":scalar(value["identity"],"number"),"body":decodeJavaScriptCallSource(value["body"]),"handler":decodeJavaScriptCallSource(value["handler"])};
case "sourceBinding":return {$:"sourceBinding","identity":scalar(value["identity"],"number")};
case "sourceLet":return {$:"sourceLet","identity":scalar(value["identity"],"number"),"value":decodeJavaScriptCallSource(value["value"]),"body":decodeJavaScriptCallSource(value["body"])};
case "sourceIf":return {$:"sourceIf","condition":decodeJavaScriptCallSource(value["condition"]),"yes":decodeJavaScriptCallSource(value["yes"]),"no":decodeJavaScriptCallSource(value["no"])};
case "sourceNumberBinary":return {$:"sourceNumberBinary","operator":decodeModelText(value["operator"]),"left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourceSameValue":return {$:"sourceSameValue","left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourceStrictEqual":return {$:"sourceStrictEqual","left":decodeJavaScriptCallSource(value["left"]),"right":decodeJavaScriptCallSource(value["right"])};
case "sourcePrimitive":return {$:"sourcePrimitive","operation":decodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceRecur":return {$:"sourceRecur","arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
case "sourceSet":return {$:"sourceSet","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"]),"value":decodeJavaScriptCallSource(value["value"])};
case "sourceGet":return {$:"sourceGet","object":decodeJavaScriptCallSource(value["object"]),"key":decodeJavaScriptCallSource(value["key"])};
case "sourceCall":return {$:"sourceCall","callee":decodeJavaScriptCallSource(value["callee"]),"receiver":decodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>decodeJavaScriptCallSource(value))};
default:throw new TypeError("unregistered actual model constructor");}}
export function encodeJavaScriptCallSource(value){switch(value.$){
case "sourceConstant":return {$:"sourceConstant","value":encodeJavaScriptSemanticValue(value["value"])};
case "sourceReceiver":return {$:"sourceReceiver"};
case "sourceArgument":return {$:"sourceArgument","index":scalar(value["index"],"number")};
case "sourceThrow":return {$:"sourceThrow","value":encodeJavaScriptCallSource(value["value"])};
case "sourceTry":return {$:"sourceTry","identity":scalar(value["identity"],"number"),"body":encodeJavaScriptCallSource(value["body"]),"handler":encodeJavaScriptCallSource(value["handler"])};
case "sourceBinding":return {$:"sourceBinding","identity":scalar(value["identity"],"number")};
case "sourceLet":return {$:"sourceLet","identity":scalar(value["identity"],"number"),"value":encodeJavaScriptCallSource(value["value"]),"body":encodeJavaScriptCallSource(value["body"])};
case "sourceIf":return {$:"sourceIf","condition":encodeJavaScriptCallSource(value["condition"]),"yes":encodeJavaScriptCallSource(value["yes"]),"no":encodeJavaScriptCallSource(value["no"])};
case "sourceNumberBinary":return {$:"sourceNumberBinary","operator":encodeModelText(value["operator"]),"left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourceSameValue":return {$:"sourceSameValue","left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourceStrictEqual":return {$:"sourceStrictEqual","left":encodeJavaScriptCallSource(value["left"]),"right":encodeJavaScriptCallSource(value["right"])};
case "sourcePrimitive":return {$:"sourcePrimitive","operation":encodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceRecur":return {$:"sourceRecur","arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
case "sourceSet":return {$:"sourceSet","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"]),"value":encodeJavaScriptCallSource(value["value"])};
case "sourceGet":return {$:"sourceGet","object":encodeJavaScriptCallSource(value["object"]),"key":encodeJavaScriptCallSource(value["key"])};
case "sourceCall":return {$:"sourceCall","callee":encodeJavaScriptCallSource(value["callee"]),"receiver":encodeJavaScriptCallSource(value["receiver"]),"arguments":value["arguments"].map(value=>encodeJavaScriptCallSource(value))};
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
case "jump":return {$:"jump","target":scalar(value["target"],"number")};
case "branch":return {$:"branch","condition":scalar(value["condition"],"number"),"yes":scalar(value["yes"],"number"),"no":scalar(value["no"],"number")};
case "call":return {$:"call","destination":scalar(value["destination"],"number"),"callee":scalar(value["callee"],"number"),"receiver":scalar(value["receiver"],"number"),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "return":return {$:"return","source":scalar(value["source"],"number")};
case "throw":return {$:"throw","source":scalar(value["source"],"number")};
case "pushHandler":return {$:"pushHandler","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number")};
case "popHandler":return {$:"popHandler"};
case "numberBinary":return {$:"numberBinary","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"operator":decodeModelText(value["operator"])};
case "sameValue":return {$:"sameValue","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "strictEqual":return {$:"strictEqual","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "requireWrite":return {$:"requireWrite","success":scalar(value["success"],"number")};
case "primitive":return {$:"primitive","destination":scalar(value["destination"],"number"),"operation":decodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
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
case "numberBinary":return {$:"numberBinary","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number"),"operator":encodeModelText(value["operator"])};
case "sameValue":return {$:"sameValue","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "strictEqual":return {$:"strictEqual","destination":scalar(value["destination"],"number"),"left":scalar(value["left"],"number"),"right":scalar(value["right"],"number")};
case "requireWrite":return {$:"requireWrite","success":scalar(value["success"],"number")};
case "primitive":return {$:"primitive","destination":scalar(value["destination"],"number"),"operation":encodeModelText(value["operation"]),"arguments":value["arguments"].map(value=>scalar(value,"number"))};
case "ordinaryGet":return {$:"ordinaryGet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"receiver":scalar(value["receiver"],"number")};
case "ordinarySet":return {$:"ordinarySet","destination":scalar(value["destination"],"number"),"object":scalar(value["object"],"number"),"key":scalar(value["key"],"number"),"value":scalar(value["value"],"number"),"receiver":scalar(value["receiver"],"number")};
case "propertyKey":return {$:"propertyKey","destination":scalar(value["destination"],"number"),"input":scalar(value["input"],"number"),"realm":String(value["realm"])};
case "propertyDefinition":return {$:"propertyDefinition","destination":scalar(value["destination"],"number"),"target":scalar(value["target"],"number"),"key":scalar(value["key"],"number"),"attributes":scalar(value["attributes"],"number"),"realm":String(value["realm"])};
default:throw new TypeError("unregistered actual model constructor");}}
