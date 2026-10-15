import {acceptSourceTextScalarUnits,acceptSourceTextByteSlice,readSourceTextCodeUnitOffset} from './source-text-byte-rules.mjs';

/** The only realm-dependent boundary: a primitive string becomes code units. */
export function readSourceTextUnits(text){
 if(typeof text!=='string')throw new TypeError('source text must be a primitive string');
 const units=[];
 for(let index=0;index<text.length;index++)units.push(text.charCodeAt(index));
 if(!acceptSourceTextScalarUnits(units))throw new TypeError('source text contains an unpaired Unicode surrogate');
 return units;
}

/** Checked UTF-8 byte endpoints preserve the exact original string contents. */
export function sliceSourceTextBytes(text,start,end){
 const units=readSourceTextUnits(text);
 if(typeof start!=='number'||typeof end!=='number'||!acceptSourceTextByteSlice(units,start,end))return null;
 return text.slice(readSourceTextCodeUnitOffset(units,start),readSourceTextCodeUnitOffset(units,end));
}
