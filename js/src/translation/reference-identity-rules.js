// Pure checked-type decisions for the existing compare-reference-identity Links operation.
/** @param {string} operation @param {string} leftKind @param {string} rightKind
 * @param {boolean} leftReference @param {boolean} rightReference
 * @param {boolean} sameDomain @returns {boolean} */
export function acceptReferenceIdentityComparison(operation, leftKind, rightKind, leftReference, rightReference, sameDomain) {
  return (operation === 'eq' || operation === 'ne') && leftReference && rightReference
    && sameDomain && leftKind === rightKind
    && (leftKind === 'array' || leftKind === 'record' || leftKind === 'map' || leftKind === 'set');
}

/** @param {string} operation @param {string} leftKind @param {string} rightKind
 * @param {boolean} leftReference @param {boolean} rightReference
 * @param {boolean} sameDomain @param {boolean} domainPresent
 * @param {boolean} domainReference @param {boolean} sameOperationDomain @returns {string} */
export function readReferenceIdentityComparisonError(operation, leftKind, rightKind, leftReference, rightReference, sameDomain, domainPresent, domainReference, sameOperationDomain) {
  if (!leftReference && !rightReference && !domainReference) return '';
  if (acceptReferenceIdentityComparison(operation, leftKind, rightKind, leftReference, rightReference, sameDomain)
    && domainPresent && domainReference && sameOperationDomain) return '';
  return 'a linked reference comparison requires matching structural reference operands, an explicit identical reference domain, and equality or inequality';
}

/** Both strings are existing borrowed Rc expression renderings, evaluated in source order.
 * @param {string} operation @param {string} left @param {string} right @returns {string} */
export function renderReferenceIdentityComparison(operation, left, right) {
  if (operation === 'eq') return 'std::rc::Rc::ptr_eq(' + left + ', ' + right + ')';
  if (operation === 'ne') return '(!std::rc::Rc::ptr_eq(' + left + ', ' + right + '))';
  return '';
}
