// Shared source owner identities and complete-runtime target adapters.
/** @param {string} name @returns {string} */
export function readJavaScriptStringConstructionOperation(name){
  if(name==='String.fromCharCode')return 'constructJavaScriptStringFromCharacterCodes';
  return name==='String.fromCodePoint'?'constructJavaScriptStringFromCodePoints':'';
}
/** @param {string} operation @returns {string} */
export function readJavaScriptStringConstructionForm(operation){
  if(operation==='convertJavaScriptCharacterCodesToString')return 'characters';
  return operation==='convertJavaScriptCodePointUnitsResult'?'points':'';
}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedJavaScriptStringConstruction(kind,operation){
  return kind==='math'&&readJavaScriptStringConstructionForm(operation)!=='';
}
/** @param {string} target @param {string} operation @param {string} values @returns {string} */
export function renderJavaScriptStringConstruction(target,operation,values){
  const form=readJavaScriptStringConstructionForm(operation);
  if(form==='')return '';
  if(target==='JavaScript')return form==='characters'?'ml_utf16_text.characterUnits('+values+').map(ml_unit=>String.fromCharCode(ml_unit)).join("")':'ml_utf16_text.codePointUnits('+values+')';
  if(target==='Rust')return form==='characters'?'crate::ml_string::Text::from_units(crate::ml_utf16_text::character_units_utf16_text(&'+values+').into_iter().map(|ml_unit| ml_unit as u16).collect())':'crate::ml_utf16_text::code_point_units_utf16_text(&'+values+')';
  if(target==='Lean'||target==='Rocq')return '(MLUtf16Text.'+(form==='characters'?'characterUnits':'codePointUnits')+' '+values+')';
  return '';
}
