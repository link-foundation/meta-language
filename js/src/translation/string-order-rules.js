// Generated from the complete shared stringOrderModel. Do not edit by hand.

/** @param {string} language @param {string} operation @param {string} kind @returns {string} */
export function readStringOrderOperation(language, operation, kind) {
  if (language !== "JavaScript" || kind !== "string") return "";
  if (operation === "lt") return "unicodeCodeUnitLessThan";
  if (operation === "le") return "unicodeCodeUnitLessThanOrEqual";
  if (operation === "gt") return "unicodeCodeUnitGreaterThan";
  if (operation === "ge") return "unicodeCodeUnitGreaterThanOrEqual";
  return "";
}

/** @param {string} expression @param {string} operation @returns {boolean} */
export function requireStringOrderUnits(expression, operation) {
  return expression === "stringTest" && (operation === "unicodeCodeUnitLessThan" || operation === "unicodeCodeUnitLessThanOrEqual" || operation === "unicodeCodeUnitGreaterThan" || operation === "unicodeCodeUnitGreaterThanOrEqual");
}

/** @param {string} target @param {string} operation @param {string} left @param {string} right @returns {string} */
export function renderStringOrderExpression(target, operation, left, right) {
  if (operation === "unicodeCodeUnitLessThan") {
    if (target === "JavaScript") return "((" + left + ") < (" + right + "))";
    if (target === "Rust") return "<crate::ml_string::Text as std::cmp::PartialOrd>::lt(" + left + ", " + right + ")";
  }
  if (operation === "unicodeCodeUnitLessThanOrEqual") {
    if (target === "JavaScript") return "((" + left + ") <= (" + right + "))";
    if (target === "Rust") return "<crate::ml_string::Text as std::cmp::PartialOrd>::le(" + left + ", " + right + ")";
  }
  if (operation === "unicodeCodeUnitGreaterThan") {
    if (target === "JavaScript") return "((" + left + ") > (" + right + "))";
    if (target === "Rust") return "<crate::ml_string::Text as std::cmp::PartialOrd>::gt(" + left + ", " + right + ")";
  }
  if (operation === "unicodeCodeUnitGreaterThanOrEqual") {
    if (target === "JavaScript") return "((" + left + ") >= (" + right + "))";
    if (target === "Rust") return "<crate::ml_string::Text as std::cmp::PartialOrd>::ge(" + left + ", " + right + ")";
  }
  return "";
}

/** @param {string} target @param {string} operation @returns {string} */
export function readStringOrderRefusal(target, operation) {
  if (!requireStringOrderUnits("stringTest", operation) || target === "JavaScript" || target === "Rust") return "";
  return "this target needs a UTF-16 operational string encoding for lexical comparison";
}
