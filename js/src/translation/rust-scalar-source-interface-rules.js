// Normative scalar source-interface decisions and syntax. Auto-translate as one
// library; native hosts only project actual IR and owned signature records.
/** @param {string} model @returns {boolean} */
export function acceptRustScalarSourceInterfaceModel(model) {
  return model === 'retained scalar Rust function interface 1';
}
/** @param {string} kind @param {number} bits @param {boolean} signed @returns {string} */
export function readRustScalarValueType(kind, bits, signed) {
  if (kind === 'unit') return '()';
  if (kind === 'bool') return 'bool';
  if (kind === 'float') return 'f64';
  if (kind !== 'fixed') return '';
  if (bits === 8) return signed ? 'i8' : 'u8';
  if (bits === 16) return signed ? 'i16' : 'u16';
  if (bits === 32) return signed ? 'i32' : 'u32';
  if (bits === 64) return signed ? 'i64' : 'u64';
  if (bits === 128) return signed ? 'i128' : 'u128';
  return '';
}
/** @param {string} valueType @param {string} ownerKind @param {string} owner @param {number} paths @param {number} children @returns {string} */
export function readRustScalarSourceType(valueType, ownerKind, owner, paths, children) {
  if (children !== 0) return '';
  if (valueType === '()') return ownerKind === 'unit' && paths === 0 ? '()' : '';
  if (ownerKind !== 'primitive' || paths !== 1 || valueType === '') return '';
  if (valueType !== 'bool' && valueType !== 'f64' && valueType !== 'i8' && valueType !== 'u8' && valueType !== 'i16' && valueType !== 'u16' && valueType !== 'i32' && valueType !== 'u32' && valueType !== 'i64' && valueType !== 'u64' && valueType !== 'i128' && valueType !== 'u128') return '';
  if (valueType === owner) return valueType;
  if (valueType === 'u64' && owner === 'usize') return 'usize';
  if (valueType === 'i64' && owner === 'isize') return 'isize';
  return '';
}
/** @param {string} language @param {boolean} forbidden @returns {boolean} */
export function acceptRustScalarSourceContext(language, forbidden) { return language === 'Rust' && !forbidden; }
/** @param {string} kind @param {boolean} generated @param {boolean} owned @returns {boolean} */
export function acceptRustScalarSourceDeclaration(kind, generated, owned) { return kind === 'fn' && !generated && owned; }
/** @param {string} field @param {boolean} ordinaryAllowed @returns {boolean} */
export function acceptRustScalarSourceDeclarationField(field, ordinaryAllowed) { return ordinaryAllowed && field !== 'sourceLoop'; }
/** @param {string} signatureModel @param {string} sourceType @param {boolean} invalidWrappers @param {boolean} borrowedResult @returns {boolean} */
export function acceptRustScalarSourcePosition(signatureModel, sourceType, invalidWrappers, borrowedResult) {
  return signatureModel === 'Rust source function signature 2' && sourceType !== '' && !invalidWrappers && !borrowedResult;
}
/** @param {string} kind @returns {boolean} */
export function acceptRustScalarSourceExpression(kind) {
  return kind === 'lit' || kind === 'unit' || kind === 'var' || kind === 'let' || kind === 'if' || kind === 'unary' || kind === 'binary' || kind === 'cast' || kind === 'call';
}
/** @param {string} kind @param {string} field @returns {boolean} */
export function acceptRustScalarSourceExpressionField(kind, field) {
  if (field === 'k' || field === 'type' || field === 'span') return acceptRustScalarSourceExpression(kind);
  if (kind === 'lit') return field === 'value';
  if (kind === 'var') return field === 'name';
  if (kind === 'let') return field === 'name' || field === 'value' || field === 'body';
  if (kind === 'if') return field === 'cond' || field === 'then' || field === 'else';
  if (kind === 'unary') return field === 'op' || field === 'arg' || field === 'semantics';
  if (kind === 'binary') return field === 'op' || field === 'left' || field === 'right' || field === 'domain' || field === 'semantics' || field === 'rounding' || field === 'byZero';
  if (kind === 'call') return field === 'fn' || field === 'args';
  if (kind === 'cast') return field === 'arg' || field === 'from' || field === 'to' || field === 'flavor';
  return false;
}
/** @param {boolean} owned @param {boolean} argumentTypes @param {boolean} resultType @param {boolean} staticBorrow @returns {boolean} */
export function acceptRustScalarSourceCall(owned, argumentTypes, resultType, staticBorrow) {
  return owned && argumentTypes && resultType && !staticBorrow;
}
/** @param {number} visits @returns {boolean} */
export function acceptRustScalarSourceBudget(visits) { return visits >= 0 && visits <= 100000 && visits % 1 === 0; }
/** @param {string} value @param {string} sourceType @param {boolean} intoSource @returns {string} */
export function renderRustScalarSourceConversion(value, sourceType, intoSource) {
  if (sourceType === 'usize') return '(' + value + ') as ' + (intoSource ? 'usize' : 'u64');
  if (sourceType === 'isize') return '(' + value + ') as ' + (intoSource ? 'isize' : 'i64');
  return value;
}
/** @returns {string} */
export function readRustScalarSourceArgumentName() { return 'ml_source_argument'; }
/** @returns {string} */
export function readRustScalarSourceTargetGuard() {
  return '#[cfg(not(target_pointer_width = "64"))]\ncompile_error!("retained scalar Rust function interface 1 requires target_pointer_width=64");';
}
/** @returns {string} */
export function readRustScalarSourceEncoding() { return 'retained-scalar-source-interface'; }
/** @returns {string} */
export function readRustScalarSourceStatement() {
  return 'owned version-two scalar headers retain source primitive identities; calls evaluate each argument once in order before borrowing copies; usize/isize use exact 64-bit boundary conversions and other pointer widths refuse';
}
