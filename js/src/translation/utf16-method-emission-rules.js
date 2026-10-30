// Mechanical string/unit transport around complete shared UTF16 source methods.
/** @param {string} kind @param {string} receiver @param {string} operation @returns {boolean} */
export function acceptUtf16TextMethod(kind,receiver,operation){return kind==='method'&&receiver==='string'&&(operation==='split'||operation==='replace'||operation==='replaceAll');}

/** @param {string} target @param {string} operation @param {string} object @param {string} argumentsText @returns {string} */
export function renderUtf16TextMethod(target,operation,object,argumentsText){
 if(operation==='replace'||operation==='replaceAll')return renderUtf16TextReplacement(target,object,argumentsText,operation==='replaceAll');
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

/** @param {string} target @param {string} object @param {string} argumentsText @param {boolean} all @returns {string} */
export function renderUtf16TextReplacement(target,object,argumentsText,all){
 const flag=all?'true':'false';
 if(target==='JavaScript'){
  const text='Array.from({length:ml_text.length},(_,ml_index)=>ml_text.charCodeAt(ml_index))';
  const search='Array.from({length:ml_search.length},(_,ml_index)=>ml_search.charCodeAt(ml_index))';
  const replacement='Array.from({length:ml_replacement.length},(_,ml_index)=>ml_replacement.charCodeAt(ml_index))';
  return '((ml_text,ml_search,ml_replacement)=>ml_utf16_text.replace('+text+', '+search+', '+replacement+', '+flag+').map(ml_unit=>String.fromCharCode(ml_unit)).join(""))('+object+', '+argumentsText+')';
 }
 if(target!=='Rust')return '';
 const text='ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()';
 const search='ml_search.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()';
 const replacement='ml_replacement.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()';
 return '(|ml_text:&crate::ml_string::Text,ml_search:&crate::ml_string::Text,ml_replacement:&crate::ml_string::Text|crate::ml_string::Text::from_units(crate::ml_utf16_text::replace_utf16_text(&('+text+'), &('+search+'), &('+replacement+'), '+flag+').into_iter().map(|ml_unit|ml_unit as u16).collect()))(&('+object+'), '+argumentsText+')';
}
