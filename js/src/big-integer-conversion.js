// Type and UTF-16 transport adapters; integer meaning lives in shared rules.
import {convertJavaScriptStringBigIntegerUnits as readStringIntegerResult,
  convertJavaScriptNumberBigInteger as readNumberIntegerResult,
  convertJavaScriptBooleanBigInteger as readBooleanIntegerResult,
  readJavaScriptBigIntegerConversionErrorClass,renderJavaScriptBigIntegerConversionMessage} from './translation/big-integer-conversion-rules.js';

const unitsOf=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
function conversionError(form,value) {
  const name=readJavaScriptBigIntegerConversionErrorClass(form);
  const message=renderJavaScriptBigIntegerConversionMessage(form,value);
  if(name==='SyntaxError')return new SyntaxError(message);
  if(name==='RangeError')return new RangeError(message);
  if(name==='TypeError')return new TypeError(message);
  throw new TypeError('a BigInt conversion error requires its shared source class');
}

export function convertJavaScriptStringBigInteger(text) {
  if(typeof text!=='string')throw new TypeError('BigInt string conversion needs an actual string');
  const result=readStringIntegerResult(unitsOf(text));
  if(result.length!==1)throw conversionError('string',text);
  return result[0];
}

export function convertJavaScriptStringBigIntegerUnits(units) {
  if(!Array.isArray(units))throw new TypeError('BigInt string conversion needs actual UTF-16 units');
  let text='';
  for(const unit of units){
    if(!Number.isInteger(unit)||unit<0||unit>65535)throw new TypeError('BigInt string conversion needs actual UTF-16 units');
    text+=String.fromCharCode(unit);
  }
  const result=readStringIntegerResult(units);
  if(result.length!==1)throw conversionError('string',text);
  return result[0];
}

export function convertJavaScriptNumberBigInteger(value) {
  if(typeof value!=='number')throw new TypeError('BigInt number conversion needs an actual Number');
  const result=readNumberIntegerResult(value);
  if(result.length!==1)throw conversionError('number',String(value));
  return result[0];
}

export function convertJavaScriptBooleanBigInteger(value) {
  if(typeof value!=='boolean')throw new TypeError('BigInt boolean conversion needs an actual Boolean');
  return readBooleanIntegerResult(value);
}
