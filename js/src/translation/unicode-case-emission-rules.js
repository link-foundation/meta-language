// Mechanical string/unit transport around the complete shared Unicode kernel.
/** @param {string} operation @returns {boolean} */
export function acceptUnicodeCaseMappingOperation(operation){return operation==='toLowerCase'||operation==='toUpperCase';}

/** @returns {string} */
export function readUnicodeCaseEncodingStatement(){return 'Unicode 16.0.0 full locale-insensitive case mappings, contextual final sigma and preserved isolated UTF16 units; alternate Unicode/locale profiles require their own declared tables';}

/** Input and result are actual target numeric UTF16 columns.
 * @param {string} target @param {string} operation @param {string} value @returns {string} */
export function renderUnicodeCaseColumnMapping(target,operation,value){
 if(!acceptUnicodeCaseMappingOperation(operation))return '';
 const upper=operation==='toUpperCase'?'true':'false';
 if(target==='JavaScript')return 'ml_unicode_case.mapCase('+value+', '+upper+')';
 if(target==='Rust')return 'crate::ml_unicode_case::map_unicode_case_utf16_text(&('+value+'), '+upper+')';
 return target==='Lean'||target==='Rocq'?'(MLUnicodeCase.mapCase '+value+' '+upper+')':'';
}

/** @param {string} target @param {string} operation @param {boolean} units @param {string} value @returns {string} */
export function renderUnicodeCaseStringMap(target,operation,units,value){
 if(!acceptUnicodeCaseMappingOperation(operation))return '';
 if(target==='JavaScript'){
  const input='Array.from({length:ml_text.length}, (_,ml_index)=>ml_text.charCodeAt(ml_index))';
  return '((ml_text)=>'+renderUnicodeCaseColumnMapping(target,operation,input)+'.map(ml_unit=>String.fromCharCode(ml_unit)).join(""))('+value+')';
 }
 if(target!=='Rust')return '';
 const input=units?'('+value+').0.iter().map(|ml_unit| f64::from(*ml_unit)).collect::<Vec<f64>>()'
  :'('+value+').encode_utf16().map(f64::from).collect::<Vec<f64>>()';
 const result=renderUnicodeCaseColumnMapping(target,operation,input)+'.into_iter().map(|ml_unit| ml_unit as u16).collect::<Vec<u16>>()';
 return units?'crate::ml_string::Text::from_units('+result+')':'String::from_utf16(&('+result+')).expect("Unicode case conversion preserves scalar string input")';
}
