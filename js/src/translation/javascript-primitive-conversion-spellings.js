// Generated from parity/translation/javascript-primitive-conversion-spellings.lino. Do not edit.
// Complete source sha256 462775dda74d51dbf254db43c3d1c368cd20e37f05b3d4b07b58840931e613e3
/** @param {string} name @returns {JavaScriptSemanticValue} */
export function readJavaScriptPrimitiveConversionSpelling(name){
  if(name === "default")return {$:"text",units:[100,101,102,97,117,108,116]};
  if(name === "number")return {$:"text",units:[110,117,109,98,101,114]};
  if(name === "string")return {$:"text",units:[115,116,114,105,110,103]};
  if(name === "valueOf")return {$:"text",units:[118,97,108,117,101,79,102]};
  if(name === "toString")return {$:"text",units:[116,111,83,116,114,105,110,103]};
  throw new TypeError("a primitive conversion spelling needs its actual registered name");
}
