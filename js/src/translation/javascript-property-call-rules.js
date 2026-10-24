// Source property spelling never proves that a receiver owns a builtin method.
/** @param {string} operation @returns {string} */
export function readJavaScriptSourcePropertyOperationKind(operation) {
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
