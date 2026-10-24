// Mechanical string/unit transport around complete shared UTF16 source methods.
/** @param {string} kind @param {string} receiver @param {string} operation @returns {boolean} */
export function acceptUtf16TextMethod(kind,receiver,operation){return kind==='method'&&receiver==='string'&&operation==='split';}

/** @param {string} target @param {string} operation @param {string} object @param {string} argumentsText @returns {string} */
export function renderUtf16TextMethod(target,operation,object,argumentsText){
 if(operation!=='split')return '';
 if(target==='JavaScript'){
  const text='Array.from({length:ml_text.length},(_,ml_index)=>ml_text.charCodeAt(ml_index))';
  const separator='ml_separator===undefined?[]:Array.from({length:ml_separator.length},(_,ml_index)=>ml_separator.charCodeAt(ml_index))';
  return '((ml_text,ml_separator,ml_limit)=>ml_utf16_text.split('+text+', '+separator+', ml_limit, ml_separator===undefined).map(ml_units=>ml_units.map(ml_unit=>String.fromCharCode(ml_unit)).join("")))('+object+', '+argumentsText+')';
 }
 if(target!=='Rust')return '';
 const text='ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()';
 const separator='ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()';
 return '(|ml_text:&crate::ml_string::Text,ml_separator:Option<&crate::ml_string::Text>,ml_limit:f64|{crate::ml_utf16_text::split_utf16_text(&('+text+'), &('+separator+'), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|crate::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&('+object+'), '+argumentsText+')';
}
