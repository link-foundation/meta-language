// Explicit successor composition. Historical source signature3 and exclusive
// profiles keep their original independent gates unless all three are selected.
import {unsupported} from './diagnostics.js';
const contexts=new WeakMap();
export function acceptRustSourceRegionComposition(signature,exclusive,callback){return signature==='Rust source function signature 3'&&exclusive==='Rust exclusive source loans 1'&&callback==='Rust closed source FnMut specializations 1';}
export function initializeRustSourceRegionComposition(parser){if(acceptRustSourceRegionComposition(parser.sourceSignatureModel,parser.exclusiveSourceModel,parser.fnMutSourceModel))contexts.set(parser,{frames:[]});}
export function hasRustSourceRegionComposition(parser){return contexts.has(parser);}
export function retainRustSourceRegionState(parser){const context=contexts.get(parser);return Boolean(context&&context.frames.at(-1)?.mode!=='closed');}
function borrowedVector(origin){const pending=[origin],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(seen.size>=65536)throw unsupported('Rust source region composition','the complete source position exceeds its bounded inspection graph');seen.add(value);if(value.arrayElementPosition)return true;for(const child of value.fields??[])pending.push(child);for(const child of value.standardSumSource?.arguments??[])pending.push(child);}return false;}
export function readRustSourceRegionBodyMode(parser,parameters,result,callback=false){
 if(!contexts.has(parser))return '';const borrowed=parameters.some(borrowedVector)||borrowedVector(result)||result.wrappers[0]==='reference';
 if(callback&&borrowed)throw unsupported('Rust source region composition','a FnMut packet keeps closed owned output/state domains; borrowed-result callbacks require their separate escaping region ABI');
 if(borrowed&&parameters.some(origin=>origin.sourceExclusiveMode==='exclusive'))throw unsupported('Rust source region composition','borrowed results do not acquire mutable parameter or receiver authority');
 return borrowed?'readonly':'closed';
}
export function beginRustSourceRegionBody(parser,mode){const context=contexts.get(parser);if(!context||!mode)return null;const frame={mode};if(context.frames.length>=256)throw unsupported('Rust source region composition','actual callable region scopes exceed their bounded nesting');context.frames.push(frame);return frame;}
export function endRustSourceRegionBody(parser,frame){if(!frame)return;const context=contexts.get(parser);if(context?.frames.at(-1)!==frame)throw new TypeError('source region bodies leave their actual parser scope in order');context.frames.pop();}
export function beginRustSourceRegionCallback(parser){return beginRustSourceRegionBody(parser,contexts.has(parser)?'closed':'');}
// Closed packets use their own retained interfaces and original source loan
// checking. They cannot hide a borrowed-vector carrier by erasing its position.
export function retainRustSourceRegionExpression(parser,origin,span){
 const frame=contexts.get(parser)?.frames.at(-1);if(frame?.mode!=='closed')return true;if(borrowedVector(origin))throw unsupported('Rust source region composition','borrowed vector values inside closed owned/FnMut bodies require their actual internal region ABI',span);return false;
}
