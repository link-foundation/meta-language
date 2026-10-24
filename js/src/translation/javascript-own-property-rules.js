// Shared closed source-domain selections; hosts provide actual operands/types.
/** @param {string} name @returns {string} */
export function readJavaScriptOwnPropertyBuiltinOperation(name){return name==='Object.hasOwn'?'testJavaScriptOwnedProperty':'';}
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptOwnPropertyReceiverForm(kind,element){
 if(kind==='unit'||kind==='option'&&element==='unit')return 'absent';
 if(kind==='option')return 'guarded';
 if(kind==='record')return 'record';
 if(kind==='array'||kind==='tuple'||kind==='string')return 'indexed';
 if(kind==='bool'||kind==='float'||kind==='int'||kind==='nat'||kind==='map'||kind==='set')return 'empty';
 return 'refuse';
}
/** @returns {string} */
export function readJavaScriptObjectCoercionError(){return 'Cannot convert undefined or null to object';}
/** @returns {string} */
export function readJavaScriptArrayPropertyIndexLimit(){return '4294967295';}
