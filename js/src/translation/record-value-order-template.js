// The payload is opaque to this complete ordinary source algorithm. A source
// specialization changes only its declared field type, retaining the wrapper.
/** @typedef {{$: 'wrappedValue', value: number}} OpaqueValue */
/** @returns {number} */
function refuseOrderPosition(){throw new TypeError('invalid ordered record property order');}
/** @returns {OpaqueValue} */
function refuseOpaqueValue(){throw new TypeError('invalid ordered record property order');}
/** @param {number[]} order @param {number} index @returns {number} */
function readOrderPosition(order,index){return order[index]??refuseOrderPosition();}
/** @param {OpaqueValue[]} values @param {number} index @returns {OpaqueValue} */
function readOpaqueValue(values,index){return values[index]??refuseOpaqueValue();}
/** @param {OpaqueValue[]} values @param {number[]} order @returns {OpaqueValue[]} */
export function reorderWrappedRecordPropertyValues(values,order){
 if(values.length!==order.length)throw new TypeError('invalid ordered record property order');
 let result=[];let seen=[];
 for(let index=0;index<order.length;index+=1){
  const position=readOrderPosition(order,index);
  if(!Number.isInteger(position)||position<0||position>=values.length||position===0&&1/position<0||seen.includes(position))throw new TypeError('invalid ordered record property order');
  seen=seen.concat([position]);
  result=result.concat([readOpaqueValue(values,position)]);
 }
 return result;
}
