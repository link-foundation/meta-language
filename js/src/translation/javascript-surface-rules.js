// Source decisions shared by the JavaScript frontend and its native adapters.

/** @param {string} receiver @param {boolean} force @returns {string} */
export function readJavaScriptIncludesReceiverForm(receiver, force) {
  if (receiver === 'array') return 'array';
  if (receiver === 'var' && !force) return 'defer';
  return 'string';
}

/** @param {string} kind @param {string} value @param {string} following @param {string} afterGroup @returns {boolean} */
export function acceptJavaScriptFunctionStart(kind, value, following, afterGroup) {
  return kind === 'identifier' && (value === 'function' || following === '=>')
    || kind === 'punct' && value === '(' && afterGroup === '=>';
}

/** @param {string} kind @param {string} following @returns {boolean} */
export function acceptJavaScriptNamespaceMethod(kind, following) {
  return kind === 'identifier' && following === '(';
}

/** @param {string} name @param {boolean} extraArgument @returns {string} */
export function readJavaScriptCollectionConstruction(name, extraArgument) {
  if (extraArgument) return '';
  if (name === 'Set') return 'set';
  if (name === 'Map') return 'map';
  return '';
}

/** @returns {string} */
export function readJavaScriptConstructionObligation() {
  return 'the constructor needs a registered portable type and construction rule';
}

/** @param {boolean} tagged @returns {string} */
export function readJavaScriptDocumentationTypeForm(tagged) {
  return tagged ? 'tagged' : 'alias';
}

/** @param {string} kind @param {string} value @param {string} suffix @param {boolean} negative @returns {string} */
export function readJavaScriptValueCaseForm(kind, value, suffix, negative) {
  if (kind === 'number') return suffix === 'n' ? 'numLit' : 'value';
  if (!negative && (kind === 'identifier' || kind === 'punct') && (value === 'true' || value === 'false')) return 'boolLit';
  if (!negative && (kind === 'string' || kind === 'identifier')) return 'value';
  return '';
}

/** @param {boolean} tagged @param {boolean} valueCase @returns {boolean} */
export function acceptJavaScriptValueSwitch(tagged, valueCase) {
  return !tagged && valueCase;
}

// The host supplies actual clause/test cardinalities and constructs nodes for
// these instructions. Both lowerers read the same ordered instruction cursor.
/** @param {boolean} imperative @param {number} step @param {number} count @returns {string} */
export function readJavaScriptValueSwitchInstruction(imperative, step, count) {
  if (step === 0) return 'capture';
  if (step === 1) return 'fallback';
  if (!imperative && step === 2) return 'reachability';
  const start = imperative ? 2 : 3;
  if (step >= start + count * 3) return 'bind';
  const phase = (step - start) % 3;
  if (phase === 0) return 'test';
  if (phase === 1) return 'body';
  return 'select';
}

/** @param {boolean} imperative @param {number} step @param {number} count @returns {number} */
export function readJavaScriptValueSwitchPosition(imperative, step, count) {
  const start = imperative ? 2 : 3;
  const offset = step - start;
  return count - 1 - (offset - offset % 3) / 3;
}

/** @param {boolean} tagged @param {boolean} methods @param {boolean} precedingStatement @returns {string} */
export function readJavaScriptObjectStatementForm(tagged, methods, precedingStatement) {
  if (tagged) return 'value';
  if (precedingStatement) return 'refuse-order';
  return methods ? 'namespace' : 'value';
}

/** @param {string} name @param {boolean} method @param {boolean} colon @param {boolean} shorthand @returns {string} */
export function readJavaScriptObjectPropertyForm(name, method, colon, shorthand) {
  if (name === '$') return 'tag';
  if (method) return 'method';
  if (colon) return 'field';
  return shorthand ? 'shorthand' : 'missing-colon';
}

/** @param {boolean} tagged @param {boolean} duplicate @param {boolean} spread @returns {string} */
export function readJavaScriptObjectValueForm(tagged, duplicate, spread) {
  if (!tagged) return 'record';
  if (duplicate) return 'duplicate';
  if (spread) return 'spread';
  return 'constructor';
}

/** @param {string} operation @returns {string} */
export function readJavaScriptMethodReceiverKind(operation) {
  if (operation === 'map' || operation === 'flatMap' || operation === 'filter' || operation === 'find' || operation === 'findIndex' || operation === 'some' || operation === 'every' || operation === 'reduce' || operation === 'join' || operation === 'push' || operation === 'pop' || operation === 'shift' || operation === 'unshift' || operation === 'splice' || operation === 'reverse' || operation === 'sort' || operation === 'fill' || operation === 'copyWithin') return 'array';
  if (operation === 'split' || operation === 'trim' || operation === 'trimStart' || operation === 'trimEnd' || operation === 'toLowerCase' || operation === 'toUpperCase' || operation === 'startsWith' || operation === 'endsWith' || operation === 'charAt' || operation === 'charCodeAt' || operation === 'codePointAt' || operation === 'substring' || operation === 'repeat' || operation === 'padStart' || operation === 'padEnd' || operation === 'replace' || operation === 'replaceAll') return 'string';
  return '';
}

/** @param {string} operation @param {number} count @returns {string} */
export function readJavaScriptArrayMethodSurfaceForm(operation, count) {
  if (operation !== 'slice') return 'array';
  return count === 0 ? 'copySlice' : 'method';
}

/** @param {string} receiver @param {string} operation @param {number} index @returns {string} */
export function readJavaScriptMethodArgumentKind(receiver, operation, index) {
  if (receiver === 'array' && operation === 'slice') return 'float';
  if (receiver === 'array' && (operation === 'includes' || operation === 'indexOf' || operation === 'lastIndexOf')) {
    if (index === 0) return 'element';
    return index === 1 ? 'float' : '';
  }
  if (receiver !== 'string') return '';
  if (index === 0 && (operation === 'slice' || operation === 'substring' || operation === 'repeat' || operation === 'charAt' || operation === 'charCodeAt' || operation === 'codePointAt' || operation === 'padStart' || operation === 'padEnd')) return 'float';
  if (index === 0 && (operation === 'includes' || operation === 'startsWith' || operation === 'endsWith' || operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'split')) return 'string';
  if (index === 1 && (operation === 'slice' || operation === 'substring' || operation === 'includes' || operation === 'startsWith' || operation === 'endsWith' || operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'charCodeAt' || operation === 'split')) return 'float';
  if (index === 1 && (operation === 'padStart' || operation === 'padEnd')) return 'string';
  return '';
}

/** @param {string} operation @returns {string} */
export function readJavaScriptStringMethodResultForm(operation) {
  if (operation === 'includes' || operation === 'startsWith' || operation === 'endsWith') return 'bool';
  if (operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'charCodeAt') return 'float';
  if (operation === 'codePointAt') return 'optional-float';
  if (operation === 'split') return 'array-string';
  return 'string';
}
