// The portable decisions of the inspectable structured data representation.
/** @param {string} term @returns {string} */
export function readStructuredDataKind(term) {
  if (term === 'structured data') return 'root';
  if (term === 'object value') return 'object';
  if (term === 'array value') return 'array';
  if (term === 'key value pair') return 'pair';
  if (term === 'string value') return 'string';
  if (term === 'Unicode code unit string') return 'units';
  if (term === 'Unicode code unit') return 'unit';
  if (term === 'number value') return 'number';
  if (term === 'boolean value') return 'boolean';
  if (term === 'null value') return 'null';
  return '';
}

/** @param {string} kind @param {string} arity @param {boolean} hasDefinition @returns {boolean} */
export function acceptStructuredDataShape(kind, arity, hasDefinition) {
  if (kind === 'root') return arity === '1' && hasDefinition;
  if (kind === 'pair') return arity === '2' && !hasDefinition;
  if (kind === 'object' || kind === 'array' || kind === 'units') return !hasDefinition;
  if (kind === 'string') return arity === '0';
  if (kind === 'number' || kind === 'boolean' || kind === 'unit') return arity === '0' && hasDefinition;
  if (kind === 'null') return arity === '0' && !hasDefinition;
  return false;
}
