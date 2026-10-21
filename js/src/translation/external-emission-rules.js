// Checked external bindings keep the names supplied by their actual provider.
// This initial target profile admits simple ASCII ECMAScript bindings.

/** @param {number} unit @returns {boolean} */
function externalIdentifierHead(unit) {
  return unit >= 65 && unit <= 90 || unit >= 97 && unit <= 122 || unit === 95 || unit === 36;
}

/** @param {string} name @returns {boolean} */
function externalReservedStatement(name) {
  return name === 'break' || name === 'case' || name === 'catch' || name === 'class'
    || name === 'const' || name === 'continue' || name === 'debugger' || name === 'default';
}

/** @param {string} name @returns {boolean} */
function externalReservedControl(name) {
  return name === 'delete' || name === 'do' || name === 'else' || name === 'enum'
    || name === 'export' || name === 'extends' || name === 'false' || name === 'finally';
}

/** @param {string} name @returns {boolean} */
function externalReservedDeclaration(name) {
  return name === 'for' || name === 'function' || name === 'if' || name === 'import'
    || name === 'in' || name === 'instanceof' || name === 'new' || name === 'null';
}

/** @param {string} name @returns {boolean} */
function externalReservedExpression(name) {
  return name === 'return' || name === 'super' || name === 'switch' || name === 'this'
    || name === 'throw' || name === 'true' || name === 'try' || name === 'typeof';
}

/** @param {string} name @returns {boolean} */
function externalReservedBinding(name) {
  return name === 'var' || name === 'void' || name === 'while' || name === 'with'
    || name === 'yield' || name === 'await' || name === 'let' || name === 'static';
}

/** @param {string} name @returns {boolean} */
function externalReservedStrictBinding(name) {
  return name === 'implements' || name === 'interface' || name === 'package'
    || name === 'private' || name === 'protected' || name === 'public'
    || name === 'eval' || name === 'arguments';
}

/** @param {string} name @returns {boolean} */
export function acceptJavaScriptExternalBinding(name) {
  if (name.length === 0 || !externalIdentifierHead(name.charCodeAt(0))) return false;
  for (let at = 1; at < name.length; at += 1) {
    const unit = name.charCodeAt(at);
    if (!externalIdentifierHead(unit) && !(unit >= 48 && unit <= 57)) return false;
  }
  return !externalReservedStatement(name) && !externalReservedControl(name)
    && !externalReservedDeclaration(name) && !externalReservedExpression(name)
    && !externalReservedBinding(name) && !externalReservedStrictBinding(name);
}

/** A constant is read; a function is called with the already rendered ordered
 * arguments. Unknown names/kinds refuse instead of inventing a target alias.
 * @param {string} kind @param {string} name @param {string} argumentsText
 * @returns {string} */
export function renderJavaScriptExternalCall(kind, name, argumentsText) {
  if (!acceptJavaScriptExternalBinding(name)) return '';
  if (kind === 'fn') return name + '(' + argumentsText + ')';
  return kind === 'const' && argumentsText === '' ? name : '';
}
