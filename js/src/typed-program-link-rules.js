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
