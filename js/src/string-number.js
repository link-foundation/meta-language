import {convertJavaScriptStringNumberUnits} from './translation/string-number-rules.js';

/** Transport the actual string's UTF-16 units to the shared definition. */
export function convertJavaScriptStringNumber(text) {
  if(typeof text!=='string')throw new TypeError('numeric string conversion needs an actual string');
  const units=[];
  for(let index=0;index<text.length;index+=1)units.push(text.charCodeAt(index));
  return convertJavaScriptStringNumberUnits(units);
}
