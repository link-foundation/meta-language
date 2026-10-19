// Generated from the linked Boolean const decision-tree policy. Do not edit.
/** @param {string} model @returns {boolean} */
export function acceptRustConstBranchModel(model){return model === "closed Boolean Rust const decision trees 1";}
/** @param {string} kind @returns {boolean} */
export function acceptRustConstBranchType(kind){return kind === "bool";}
/** @param {string} kind @returns {boolean} */
export function acceptRustConstBranchKind(kind){return kind === "lit" || kind === "var" || kind === "unary" || kind === "binary" || kind === "if";}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptRustConstBranchOperation(kind,operation){return kind === "unary" && (operation === "not") || kind === "binary" && (operation === "and" || operation === "or");}
/** @param {string} result @param {string} condition @param {string} consequent @param {string} alternative @returns {boolean} */
export function acceptRustConstBranchConditional(result,condition,consequent,alternative){return result === "bool" && condition === "bool" && consequent === "bool" && alternative === "bool";}
