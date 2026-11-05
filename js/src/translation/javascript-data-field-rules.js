/** Actual constructor-field coordinates and resolved field-type identities.
 * Equal keys come from the owning checker, not unqualified source spellings.
 * Missing own fields yield undefined; inherited members need their prototype.
 * @param {string} language @param {string} field @param {boolean} inherited
 * @param {number[]} positions @param {string[]} types
 * @returns {{mode:string,error:string,optional:boolean}} */
export function readJavaScriptDataFieldPlan(language, field, inherited, positions, types) {
  if (language !== 'JavaScript') return {mode:'unchanged',error:'',optional:false};
  if (positions.length === 0 || positions.length > 65536 || positions.length !== types.length)
    return {mode:'refuse',error:'a datatype property needs its complete bounded constructor directory',optional:false};
  if (field === '$') return {mode:'tag',error:'',optional:false};
  let missing = false;
  let selected = '';
  for (let at = 0; at < positions.length; at += 1) {
    const position = positions[at] ?? -2;
    const type = types[at] ?? '';
    if (!Number.isSafeInteger(position) || position < -1 || (position < 0 ? type !== '' : type === ''))
      return {mode:'refuse',error:'a datatype property needs actual matching field coordinates and types',optional:false};
    if (position < 0) missing = true;
    else if (selected === '') selected = type;
    else if (selected !== type) return {mode:'refuse',error:'different constructor field types need their explicit union property model',optional:false};
  }
  if (missing && inherited) return {mode:'refuse',error:'an inherited datatype property needs its complete prototype operation',optional:false};
  return {mode:selected === '' ? 'absent' : 'field',error:'',optional:missing};
}

/** Inference transports presence only; it does not invent type-equivalence
 * evidence for terms whose types have not yet been solved.
 * @param {string} field @param {number[]} positions @returns {string} */
export function readJavaScriptDataFieldInferenceForm(field, positions) {
  if (positions.length === 0 || positions.length > 65536) return 'refuse';
  if (field === '$') return 'tag';
  let present = false;
  let missing = false;
  for (const position of positions) {
    if (!Number.isSafeInteger(position) || position < -1) return 'refuse';
    if (position < 0) missing = true;
    else present = true;
  }
  return !present ? 'absent' : missing ? 'optional' : 'present';
}

/** A source field spelling must not select a nominal owner before actual call
 * arguments, collection callbacks and declared structural records have settled.
 * This retains the previous unique-owner fallback only at quiescence.
 * @param {string} kind @param {number} owners @param {boolean} settled @returns {boolean} */
export function acceptJavaScriptDataFieldOwnerFallback(kind, owners, settled) {
  return settled && kind === 'var' && owners === 1;
}
