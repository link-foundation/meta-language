/** A standard typed array slice allocates a separate container. This says
 * nothing about unknown receivers, custom methods or element deep ownership.
 * @param {string} expression @param {string} operation @param {string} receiver
 * @param {number} pathLength @param {number} argumentsCount @param {boolean} actualArray
 * @returns {boolean} */
export function acceptOwnedArrayFreshSliceSource(expression,operation,receiver,pathLength,argumentsCount,actualArray){
 if(!actualArray||receiver!=='name'||pathLength!==1)return false;
 if(expression==='copySlice')return operation===''&&argumentsCount===0;
 return expression==='method'&&operation==='slice'&&Number.isInteger(argumentsCount)&&argumentsCount>=0&&argumentsCount<=2;
}
