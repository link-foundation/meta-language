// Generated from linked owned local state policy. Do not edit.
/** @param {string} model @returns {boolean} */
export function acceptRustOwnedLocalStateModel(model){return model === "" || model === "closed owned text local state 1" || model === "canonical owned text local copies 1";}
/** @param {string} kind @param {string} bits @param {boolean} signed @returns {boolean} */
export function acceptRustOwnedLocalValueType(kind,bits,signed){return kind === "string" || kind === "bool" || (kind === "fixed" && bits === "32" && signed);}
/** @param {string} kind @returns {boolean} */
export function acceptRustOwnedLocalExpression(kind){return kind === "lit" || kind === "var" || kind === "tuple" || kind === "tupleIndex" || kind === "let" || kind === "if" || kind === "binary" || kind === "toString";}
/** @param {string} operation @returns {boolean} */
export function acceptRustOwnedLocalBinaryOperation(operation){return operation === "eq";}
/** @param {string} text @returns {boolean} */
export function acceptRustOwnedLocalTextInput(text){
 if(typeof text !== "string")return false;
 for(let index=0;index<text.length;index++){
  const unit=text.charCodeAt(index);
  if(unit>=55296&&unit<=56319){
   index++;
   const next=text.charCodeAt(index);
   if(!(next>=56320&&next<=57343))return false;
  }else if(unit>=56320&&unit<=57343)return false;
 }
 return true;
}
/** @returns {string} */
export function renderRustOwnedLocalTextInput(){return "function ml_ownedTextInput(text){\n if(typeof text !== \"string\")return false;\n for(let index=0;index<text.length;index++){\n  const unit=text.charCodeAt(index);\n  if(unit>=55296&&unit<=56319){\n   index++;\n   const next=text.charCodeAt(index);\n   if(!(next>=56320&&next<=57343))return false;\n  }else if(unit>=56320&&unit<=57343)return false;\n }\n return true;\n}";}
