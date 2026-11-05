// Hosts only transport actual token units and ordinary surface expression nodes.
import {requireJavaScriptStringUnitTransport} from './javascript-string-unit-rules.js';
import {FLOAT} from './types.js';
export function createJavaScriptStringLiteral(value,span,units=undefined) {
  units??=Array.from({length:value.length},(_,at)=>value.charCodeAt(at));
  if(!requireJavaScriptStringUnitTransport(units))return{k:'str',value,span};
  const items=units.map(unit=>({value:{k:'num',value:String(unit),type:FLOAT,span},spread:false}));
  return{k:'math',op:'createJavaScriptStringFromUnits',name:'JavaScript UTF16 literal',
    args:[{value:{k:'array',items,span},spread:false}],span};
}
