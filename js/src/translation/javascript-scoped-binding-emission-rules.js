// Generated from linked owned JavaScript scoped binding emission policy.
/** @param {string} model @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionModel(model) { return model === "owned JavaScript scoped bindings 1"; }
/** @param {string} language @param {boolean} forbidden @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionContext(language, forbidden) { return (language === "Rust" || language === "JavaScript") && !forbidden; }
/** @param {boolean} present @param {string} form @param {boolean} ownedSnapshot @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionReferenceContext(present, form, ownedSnapshot) { return !present || form === "false" || (form === "true" && ownedSnapshot); }
/** @param {string} kind @param {boolean} nonFlat @param {boolean} generated @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionDeclaration(kind, nonFlat, generated) { return kind === "fn" && !nonFlat && !generated; }
/** @param {string} kind @param {boolean} singleField @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionValueType(kind, singleField) { return (kind === "float" || kind === "bool" || kind === "string" || kind === "unit") && singleField; }
/** @param {string} kind @param {string} element @param {string} absence @param {boolean} exact @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionAbsentValueType(kind, element, absence, exact) { return kind === "option" && element === "float" && absence === "undefined" && exact; }
/** @param {string} kind @param {string} absence @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionAbsentExpression(kind, absence) { return kind === "option" && absence === "undefined"; }
/** @param {string} kind @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionExpression(kind) { return kind === "lit" || kind === "unit" || kind === "none" || kind === "var" || kind === "let" || kind === "if" || kind === "unary" || kind === "binary" || kind === "call" || kind === "print" || kind === "math"; }
/** @param {string} kind @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionEffect(kind) { return kind === "let" || kind === "print"; }
/** @param {string} role @param {string} owner @returns {boolean} */
export function acceptJavaScriptScopedBindingEmissionRole(role, owner) { return (role === "function binding" && owner === "function declaration") || (role === "main binding" && owner === "main effect container") || (role === "scalar loop binding" && owner === "scalar loop declaration"); }
/** @param {boolean} preserve @returns {boolean} */
export function reserveJavaScriptScopedBindingDeclarationNames(preserve) { return !preserve; }
/** @param {string} name @param {boolean} preserve @returns {string} */
export function renderJavaScriptScopedBindingIdentifier(name, preserve) { if (preserve) return name; return "binding_" + name; }
