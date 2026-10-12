// Generated from data/typed-program-links-schema.json. Do not edit by hand.

/** @param {string} term @returns {string} */
export function readProgramContainerKind(term) {
  if (term === "program record") return "record";
  if (term === "ordered program list") return "array";
  if (term === "program mapping") return "map";
  return "";
}

/** @param {string} term @returns {string} */
export function readProgramScalarKind(term) {
  if (term === "string value") return "string";
  if (term === "number value") return "number";
  if (term === "boolean value") return "boolean";
  if (term === "integer value") return "bigint";
  if (term === "null value") return "null";
  if (term === "undefined value") return "undefined";
  return "";
}

/** @param {string} term @returns {string} */
export function readProgramExpressionProjection(term) {
  if (term === "lit") return "lit";
  if (term === "lit:bool") return "lit:bool";
  if (term === "lit:string") return "lit:string";
  if (term === "lit:nat") return "lit:nat";
  if (term === "lit:int") return "lit:int";
  if (term === "lit:fixed") return "lit:fixed";
  if (term === "lit:float") return "lit:float";
  if (term === "unit") return "unit";
  if (term === "var") return "var";
  if (term === "call") return "call";
  if (term === "unary") return "unary";
  if (term === "binary") return "binary";
  if (term === "if") return "if";
  if (term === "let") return "let";
  if (term === "toString") return "toString";
  if (term === "toString:true") return "toString:true";
  if (term === "stringMap") return "stringMap";
  if (term === "stringTest") return "stringTest";
  if (term === "cast") return "cast";
  if (term === "abort") return "abort";
  if (term === "print") return "print";
  if (term === "array") return "array";
  if (term === "append") return "append";
  if (term === "index") return "index";
  if (term === "length") return "length";
  if (term === "length:string") return "length:string";
  if (term === "math") return "math";
  return "";
}

/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptProgramExpressionOperation(kind, operation) {
  if (kind === "math") return operation === "abs" || operation === "floor" || operation === "ceil" || operation === "trunc" || operation === "round" || operation === "sign" || operation === "sqrt" || operation === "max" || operation === "min" || operation === "maxOf" || operation === "minOf" || operation === "isInteger" || operation === "isSafeInteger" || operation === "isFinite" || operation === "isNaN";
  if (kind === "stringMap") return operation === "toLowerCase" || operation === "toUpperCase" || operation === "trim" || operation === "trimStart" || operation === "trimEnd" || operation === "rustTrim" || operation === "rustTrimStart" || operation === "rustTrimEnd";
  if (kind === "stringTest") return operation === "startsWith" || operation === "endsWith" || operation === "includes";
  if (kind === "method") return operation === "split" || operation === "slice" || operation === "indexOf" || operation === "lastIndexOf" || operation === "replace" || operation === "replaceAll" || operation === "repeat" || operation === "padStart" || operation === "padEnd" || operation === "codePointAt" || operation === "map" || operation === "filter" || operation === "find" || operation === "findIndex" || operation === "some" || operation === "every" || operation === "flatMap" || operation === "forEach" || operation === "reduce" || operation === "join" || operation === "includes";
  if (kind === "collectionRead") return operation === "get" || operation === "has";
  if (kind === "collectionItems") return operation === "keys" || operation === "values" || operation === "entries";
  if (kind === "optionalPredicate") return operation === "isInteger" || operation === "isSafeInteger" || operation === "isFinite" || operation === "isNaN";
  return false;
}

/** @param {string} label @returns {boolean} */
export function acceptProgramReferenceField(label) {
  return label === "binding reference" || label === "declaration reference";
}

/** @param {string} name @returns {boolean} */
export function isUnsafeProgramField(name) {
  return name === "__proto__" || name === "constructor" || name === "prototype";
}

/** @param {string} term @param {string} revision @param {string} arity @returns {boolean} */
export function acceptProgramSchemaRoot(term, revision, arity) {
  return term === "typed program" && revision === "1" && arity === "1";
}

/** @param {string} width @returns {boolean} */
export function acceptProgramMachineIntegerWidth(width) {
  return width === "8" || width === "16" || width === "32" || width === "64" || width === "128";
}

/** @param {string} absence @returns {boolean} */
export function acceptProgramOptionalAbsence(absence) {
  return absence === "null" || absence === "undefined" || absence === "both";
}

/** @param {string} rounding @returns {boolean} */
export function acceptProgramDivisionRounding(rounding) {
  return rounding === "trunc" || rounding === "euclid" || rounding === "floor";
}
