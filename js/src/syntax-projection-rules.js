// Complete generic CST projection policy; original concrete links stay present.
/** @param {string} role @returns {string} */
export function readSyntaxProjectionTerm(role) {
  if (role === 'language') return 'meta-language syntax projection';
  if (role === 'root') return 'abstract syntax';
  if (role === 'node') return 'abstract syntax node';
  if (role === 'children') return 'abstract syntax children';
  if (role === 'edge') return 'abstract syntax edge';
  if (role === 'index') return 'concrete syntax child position';
  return '';
}
/** @param {string} type @returns {boolean} */
export function acceptConcreteSyntaxNodeType(type) {
  return type === 'node' || type === 'token' || type === 'embed'
    || type === 'error' || type === 'missing';
}
/** Operators and literal spellings remain semantic inputs. Only explicitly
 * marked trivia is omitted by default; recovery records remain visible.
 * @param {string} type @param {boolean} trivia @param {boolean} dropped @returns {boolean}
 */
export function acceptAbstractSyntaxNode(type, trivia, dropped) {
  return !dropped && (type === 'error' || type === 'missing' || !trivia);
}
/** @param {string} type @param {string} role @returns {boolean} */
export function acceptConcreteSyntaxChildRole(type, role) {
  if (type === 'node') return role === 'children';
  if (type === 'embed') return role === 'root' || role === 'sourceChildren' || role === 'alternative root';
  return false;
}
