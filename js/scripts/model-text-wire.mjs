// Transport only. Ordinary Unicode strings retain their original wire shape.
// Explicit units preserve JavaScript strings outside Rust String's domain.
export const modelTextWirePolicy={model:'lossless model text transport 1',tag:'UTF16 code units 1',maximumUnits:16777216};
export function encodeModelText(value){
 if(typeof value!=='string')throw new TypeError('model text');
 if(value.length>modelTextWirePolicy.maximumUnits)throw new RangeError('model text unit budget');
 if(value.isWellFormed())return value;
 const units=[];for(let index=0;index<value.length;index+=1)units.push(value.charCodeAt(index));
 return {textEncoding:modelTextWirePolicy.tag,units};
}
export function decodeModelText(value){
 if(typeof value==='string'){
  if(value.length>modelTextWirePolicy.maximumUnits)throw new RangeError('model text unit budget');
  return value;
 }
 if(!value||typeof value!=='object'||Array.isArray(value)||Object.keys(value).length!==2||value.textEncoding!==modelTextWirePolicy.tag||!Array.isArray(value.units))throw new TypeError('qualified model text units');
 if(value.units.length>modelTextWirePolicy.maximumUnits)throw new RangeError('model text unit budget');
 let text='';for(const unit of value.units){if(!Number.isInteger(unit)||unit<0||unit>65535)throw new TypeError('model UTF16 unit');text+=String.fromCharCode(unit);}
 return text;
}
