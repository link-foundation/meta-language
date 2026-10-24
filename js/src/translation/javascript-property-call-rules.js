// Source property spelling never proves that a receiver owns a builtin method.
/** @param {string} operation @returns {string} */
export function readJavaScriptSourcePropertyOperationKind(operation) {
  if(operation==='indexOf'||operation==='lastIndexOf')return 'index';
  if (operation === 'includes' || operation === 'startsWith' || operation === 'endsWith') return 'test';
  if (operation === 'trim' || operation === 'trimStart' || operation === 'trimEnd'
      || operation === 'toLowerCase' || operation === 'toUpperCase') return 'map';
  return operation === 'toString' ? 'text' : '';
}
/** @param {string} receiver @param {string} operation @returns {string} */
export function readJavaScriptResolvedPropertyCallForm(receiver, operation) {
  const kind = readJavaScriptSourcePropertyOperationKind(operation);
  if (kind === '') return '';
  if (receiver === 'record') return 'record';
  if (kind === 'text') return 'text';
  return receiver === 'string' ? kind : '';
}
/** @param {string} kind @param {number} parameters @param {number} argumentsCount @returns {string} */
export function readJavaScriptRecordMethodInferenceStep(kind, parameters, argumentsCount) {
  if (kind === 'var') return 'infer';
  return kind === 'function' && parameters === argumentsCount ? 'apply' : 'checked error';
}
/** @param {string} operation @param {string} selected @returns {string} */
export function readJavaScriptPropertyCallFallbackKind(operation, selected) {
  if (operation === 'includes') return 'string';
  if (operation === 'toString') return 'scalar';
  return selected;
}
/** @param {string} operation @param {string} selected @returns {string} */
export function readJavaScriptConcatenationFallbackKind(operation, selected) {
  return operation === 'concat' ? 'array' : readJavaScriptPropertyCallFallbackKind(operation, selected);
}
/** @param {string} receiver @param {string} argument @param {boolean} force @returns {string} */
export function readJavaScriptConcatenationArgumentRole(receiver, argument, force) {
  if (argument === 'var' && !force) return 'defer';
  if (receiver === 'array') return argument === 'array' ? 'spread' : 'item';
  if (receiver !== 'string') return '';
  return ['string','float','bool','int','nat','fixed','unit','option'].includes(argument) ? 'text' : 'unsupported';
}

/** @param {string} kind @returns {string} */
export function readJavaScriptPrimitiveTextForm(kind){
 if(kind==='string')return 'identity';
 if(kind==='unit')return 'undefined';
 if(kind==='option')return 'optional';
 return kind==='float'||kind==='bool'||kind==='int'||kind==='nat'||kind==='fixed'?'convert':'refuse';
}
/** Missing/undefined end positions select the complete receiver.
 * @param {string} operation @returns {string} */
export function readJavaScriptStringSearchDefaultPosition(operation){return operation==='endsWith'?'Infinity':operation==='lastIndexOf'?'NaN':'0';}
/** Slicing uses an explicitly nonnegative position, never a relative offset.
 * @param {string} operation @returns {string[]} */
export function planJavaScriptStringSearchWindow(operation){return operation==='endsWith'?['0','position']:['position','Infinity'];}

/** @param {string} operation @returns {string} */
export function readJavaScriptStringSearchResultKind(operation){return operation==='indexOf'||operation==='lastIndexOf'?'float':'bool';}
