/** @param {string} term @returns {string} */
export function readRustConcreteItemKind(term) {
  switch (term) {
    case 'structure_item': return 'struct_item';
    case 'enumeration_item': return 'enum_item';
    case 'constant_item': return 'const_item';
    case 'module_item': return 'mod_item';
    case 'implementation_item': return 'impl_item';
    default: return term;
  }
}

/** @param {string} term @returns {boolean} */
export function testRustConcreteSelectableItemKind(term) {
  const kind = readRustConcreteItemKind(term);
  return kind === 'function_item' || kind === 'struct_item' || kind === 'enum_item'
    || kind === 'union_item' || kind === 'trait_item' || kind === 'type_item'
    || kind === 'const_item' || kind === 'static_item';
}

/** @param {number} parentStart @param {number} parentEnd @param {number} childStart @param {number} childEnd @returns {boolean} */
export function testRustConcreteSourceContainment(parentStart, parentEnd, childStart, childEnd) {
  return parentStart <= childStart && childEnd <= parentEnd;
}

/** @param {number} parentStart @param {number} parentEnd @param {number} childStart @param {number} childEnd @returns {boolean} */
export function testRustConcreteSourceEnclosure(parentStart, parentEnd, childStart, childEnd) {
  return testRustConcreteSourceContainment(parentStart, parentEnd, childStart, childEnd)
    && (parentStart < childStart || childEnd < parentEnd);
}

/** @param {string} term @param {number} start @param {number} end @param {number} moduleStart @param {number} moduleEnd @returns {boolean} */
export function testRustConcreteModuleDeclaration(term, start, end, moduleStart, moduleEnd) {
  return readRustConcreteItemKind(term) === 'mod_item' && start <= moduleStart && end === moduleEnd;
}

/** @param {string} term @returns {boolean} */
export function testRustConcreteEntryOwnerKind(term) {
  const kind = readRustConcreteItemKind(term);
  return kind === 'mod_item' || kind === 'function_item' || kind === 'impl_item' || kind === 'trait_item';
}
