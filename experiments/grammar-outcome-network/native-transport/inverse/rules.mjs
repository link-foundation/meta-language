// Complete portable decisions shared by construction and independent validation.

/** @param {number} width @returns {boolean} */
export function acceptOwnedOutcomeTargetWidth(width) {
  return width===32||width===64;
}

/** @param {boolean} nullValue @param {boolean} arrayValue @param {string} valueType @returns {string} */
export function readOwnedOutcomeJsonKind(nullValue,arrayValue,valueType) {
  if(nullValue)return 'null';
  if(arrayValue)return 'array';
  if(valueType==='object')return 'object';
  if(valueType==='number')return 'number';
  if(valueType==='boolean')return 'boolean';
  if(valueType==='string')return 'string';
  return 'invalid';
}

/** @param {string} kind @returns {string[]} */
export function readOwnedOutcomeJsonFields(kind) {
  if(kind==='object')return ['kind','keys','aliasOf'];
  if(kind==='array')return ['kind','aliasOf'];
  if(kind==='number')return ['kind','bits'];
  if(kind==='string'||kind==='boolean')return ['kind','value'];
  return ['kind'];
}

/** @param {string} sourceKind @param {string} jsonKind @param {boolean} variantRecord @returns {boolean} */
export function acceptOwnedOutcomeContainer(sourceKind,jsonKind,variantRecord) {
  return sourceKind==='Vec'||sourceKind==='IndexMap'||sourceKind==='struct_item'
    ||sourceKind==='enum_item'&&variantRecord
    ||sourceKind==='json'&&(jsonKind==='array'||jsonKind==='object');
}
