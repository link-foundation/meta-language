// Actual checked JavaScript type facts select existing Boolean/Truthy/Let IR.
/** @param {string} kind @returns {boolean} */
export function readJavaScriptArrayClassification(kind) {
  return kind === 'array' || kind === 'tuple';
}

/** [array result, inspect optional presence]; an empty plan refuses the domain.
 * @param {string} kind @param {string} element @returns {boolean[]} */
export function readJavaScriptArrayClassificationPlan(kind, element) {
  const selected = kind === 'option' ? element : kind;
  if (readJavaScriptArrayClassification(selected)) return [true, kind === 'option'];
  if (selected === 'unit' || selected === 'absence' || selected === 'bool' || selected === 'string') return [false, false];
  if (selected === 'float' || selected === 'int' || selected === 'nat' || selected === 'fixed' || selected === 'literal') return [false, false];
  if (selected === 'record' || selected === 'data' || selected === 'set' || selected === 'map' || selected === 'cell' || selected === 'function') return [false, false];
  return [];
}

/** @param {string} name @returns {string} */
export function readJavaScriptArrayClassificationBuiltinOperation(name){return name==='Array.isArray'?'testJavaScriptArrayValue':'';}
