// Generated from the complete shared number predicate model. Do not edit.

/** @param {string} language @param {string} operation @param {string} kind @param {string} element @returns {string} */
export function readNumberPredicateArgumentForm(language, operation, kind, element) {
  if (language !== "JavaScript" || !(operation === "isInteger" || operation === "isSafeInteger" || operation === "isFinite" || operation === "isNaN")) return "";
  if (kind === "float") return "number";
  if (kind === "option" && element === "float") return "optional-number";
  if (kind === "bool" || kind === "string" || kind === "unit" || kind === "int" || kind === "nat" || kind === "array" || kind === "tuple" || kind === "record" || kind === "map" || kind === "set" || kind === "function" || kind === "data") return "false";
  if (kind === "option" && (element === "bool" || element === "string" || element === "unit" || element === "int" || element === "nat" || element === "array" || element === "tuple" || element === "record" || element === "map" || element === "set" || element === "function" || element === "data")) return "false";
  return "unsupported";
}
