// Portable source decisions. The adapters transport actual token positions and
// original inference terms; no rule below re-evaluates a source expression.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** @param {string} token @returns {string} */
export function readJavaScriptArrayBindingForm(token) {
  if (token === ']') return 'close';
  if (token === ',') return 'elision';
  if (token === '...') return 'rest';
  if (token === '[') return 'nested';
  return 'name';
}

/** A named rest element is last and has no initializer or trailing comma.
 * @param {string} following @returns {string} */
export function readJavaScriptArrayRestBindingError(following) {
  if (following === '=') return 'an array rest binding cannot have a default';
  if (following === ',') return 'an array rest binding must be last without a trailing comma';
  return following === ']' ? '' : 'an array rest binding needs the closing bracket';
}

/** @param {string} kind @param {string} name @param {boolean} duplicate
 * @param {string[]} reserved @returns {boolean} */
export function acceptJavaScriptArrayBindingName(kind, name, duplicate, reserved) {
  if (kind !== 'identifier' || name === '' || duplicate) return false;
  if (name.length >= 3 && name.slice(0, 3) === 'ml_') return false;
  for (let index = 0; index < reserved.length; index += 1) {
    if (readRequiredTextFact(reserved, index) === name) return false;
  }
  return true;
}

/** @param {boolean} rest @param {boolean} defaultValue @returns {string} */
export function readJavaScriptArrayBindingValueForm(rest, defaultValue) {
  return rest ? 'suffix' : defaultValue ? 'default' : 'element';
}

/** @param {string} kind @returns {boolean} */
export function acceptJavaScriptArrayRestReceiver(kind) {
  return kind === 'array';
}

/** Host flags come from the actual numeric literal payload and safe-integer
 * classifier, never the source span or a different pattern's bound.
 * @param {boolean} literal @param {boolean} safeInteger @param {boolean} negative
 * @returns {boolean} */
export function acceptJavaScriptArrayBindingOffset(literal, safeInteger, negative) {
  return literal && safeInteger && !negative;
}

/** @param {string} field @returns {boolean} */
export function acceptJavaScriptArrayBindingOffsetField(field) {
  return field === 'k' || field === 'value' || field === 'negative' || field === 'span';
}

/** Keep source-only wrappers closed before checking their actual operands.
 * @param {string} kind @param {string} field @returns {boolean} */
export function acceptJavaScriptArrayBindingField(kind, field) {
  if (kind !== 'arrayBindingRest' && kind !== 'arrayBindingDefault') return false;
  if (field === 'k' || field === 'span' || field === 'parenthesized') return true;
  if (kind === 'arrayBindingRest') return field === 'object' || field === 'start';
  return kind === 'arrayBindingDefault' && (field === 'value' || field === 'fallback');
}

/** The first stage uses Coalesce only after undefined-only primitive proof.
 * All other default meanings remain an explicit source obligation.
 * @param {string} language @param {string} kind @param {string} absence
 * @param {string} element @param {boolean} sameFallback @returns {string} */
export function readJavaScriptArrayDefaultOperation(language, kind, absence, element, sameFallback) {
  if (language !== 'JavaScript') return 'refuse-language';
  if (kind !== 'option' || absence !== 'undefined') return 'refuse-absence';
  if (element !== 'float' && element !== 'bool' && element !== 'string'
    && element !== 'int' && element !== 'nat' && element !== 'fixed') return 'refuse-element';
  return sameFallback ? 'coalesce' : 'refuse-fallback';
}

/** Empty means defer; [-1] means an unsupported actual absence. Every other
 * result contains two ordered pairs of original term identities, with no
 * replacement or reconstructed type graph.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} resolved
 * @param {number} fallback @param {number} result @returns {number[]} */
export function planJavaScriptArrayDefaultInference(texts, starts, counts, labels, targets, resolved, fallback, result) {
  const kind = readArrayBindingFactField(starts, counts, labels, targets, resolved, 'kind');
  if (kind < 0) return [-1];
  const value = readRequiredTextFact(texts, kind);
  if (value === 'var') return [];
  const absence = readArrayBindingFactField(starts, counts, labels, targets, resolved, 'absence');
  if (value !== 'option' || absence < 0 || readRequiredTextFact(texts, absence) !== 'undefined') return [-1];
  const element = readArrayBindingFactField(starts, counts, labels, targets, resolved, 'element');
  return element < 0 ? [-1] : [element, fallback, result, element];
}

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} node @param {string} field
 * @returns {number} */
function readArrayBindingFactField(starts, counts, labels, targets, node, field) {
  if (node < 0 || node >= starts.length || node >= counts.length) return -1;
  let at = readRequiredNumberFact(starts, node);
  const end = at + readRequiredNumberFact(counts, node);
  while (at < end) {
    if (readRequiredTextFact(labels, at) === field) return readRequiredNumberFact(targets, at);
    at += 1;
  }
  return -1;
}

/** Skip one actual balanced initializer, stopping before the pattern delimiter.
 * Return -1 for malformed or empty input, retaining the ordinary parser's
 * expression admission as a separate boundary.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @returns {number} */
function readArrayBindingInitializerEnd(kinds, values, start) {
  let index = start;
  let closes = [];
  while (index < values.length && index < kinds.length) {
    const kind = readRequiredTextFact(kinds, index);
    const value = readRequiredTextFact(values, index);
    if (kind === 'eof') return -1;
    if (kind === 'punct') {
      if (closes.length === 0 && (value === ',' || value === ']')) return index > start ? index : -1;
      if (value === '(' || value === '[' || value === '{') {
        closes = closes.concat([value === '(' ? ')' : value === '[' ? ']' : '}']);
      } else if (value === ')' || value === ']' || value === '}') {
        if (closes.length === 0 || readRequiredTextFact(closes, closes.length - 1) !== value) return -1;
        closes = closes.slice(0, closes.length - 1);
      }
    }
    index += 1;
  }
  return -1;
}

/** Binder discovery is syntax-only; entries are actual token positions.
 * First cell is the actual closing bracket position; [-1] is malformed.
 * Nested existing patterns are discovered, but nested rest/default admission
 * is still decided separately by their semantic boundary.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @returns {number[]} */
export function planJavaScriptArrayBindingNames(kinds, values, start) {
  if (start < 0 || start % 1 !== 0 || start >= kinds.length || start >= values.length) return [-1];
  if (readRequiredTextFact(kinds, start) !== 'punct' || readRequiredTextFact(values, start) !== '[') return [-1];
  let index = start + 1;
  let positions = [];
  while (index < values.length && index < kinds.length) {
    const kind = readRequiredTextFact(kinds, index);
    const value = readRequiredTextFact(values, index);
    if (kind === 'punct' && value === ']') return [index].concat(positions);
    if (kind === 'punct' && value === ',') { index += 1; continue; }
    let rest = false;
    if (kind === 'punct' && value === '...') { rest = true; index += 1; }
    if (index >= values.length || index >= kinds.length) return [-1];
    if (readRequiredTextFact(kinds, index) === 'punct' && readRequiredTextFact(values, index) === '[') {
      if (rest) return [-1];
      const nested = planJavaScriptArrayBindingNames(kinds, values, index);
      if (readRequiredNumberFact(nested, 0) < 0) return [-1];
      positions = positions.concat(nested.slice(1));
      index = readRequiredNumberFact(nested, 0) + 1;
    } else {
      if (readRequiredTextFact(kinds, index) !== 'identifier') return [-1];
      positions = positions.concat([index]);
      index += 1;
    }
    if (index >= values.length || index >= kinds.length) return [-1];
    if (rest) {
      return readRequiredTextFact(kinds, index) === 'punct' && readRequiredTextFact(values, index) === ']'
        ? [index].concat(positions) : [-1];
    }
    if (readRequiredTextFact(kinds, index) === 'punct' && readRequiredTextFact(values, index) === '=') {
      index = readArrayBindingInitializerEnd(kinds, values, index + 1);
      if (index < 0) return [-1];
    }
    if (readRequiredTextFact(kinds, index) !== 'punct') return [-1];
    const next = readRequiredTextFact(values, index);
    if (next === ']') return [index].concat(positions);
    if (next !== ',') return [-1];
    index += 1;
  }
  return [-1];
}

/** Current defaults inspect only undefined; arbitrary represented present values survive.
 * @param {string} language @param {string} kind @param {string} absence @returns {string} */
export function readJavaScriptBindingDefaultForm(language,kind,absence){
 if(language!=='JavaScript')return 'refuse-language';
 if(kind==='var'||kind==='literal')return 'defer';
 return kind==='option'&&absence!=='null'?'select':'identity';
}
/** @param {string} left @param {string} rightKind @param {string} right @returns {string} */
export function readJavaScriptBindingDefaultResultAbsence(left,rightKind,right){
 const nullValue=left==='null'||left==='both'||rightKind==='option'&&(right==='null'||right==='both');
 const undefinedValue=rightKind==='unit'||rightKind==='option'&&(right==='undefined'||right==='both');
 return nullValue?(undefinedValue?'both':'null'):undefinedValue?'undefined':'';
}
/** @param {string} absence @returns {number} */
export function readJavaScriptBindingDefaultAbsenceCode(absence){return absence==='null'?1:absence==='undefined'?2:absence==='both'?3:0;}
/** @param {number} code @returns {string} */
export function readJavaScriptBindingDefaultAbsenceName(code){return code===1?'null':code===2?'undefined':code===3?'both':'';}
/** Retain actual term endpoints. The first two cells select output absence/base;
 * remaining pairs constrain actual present domains. No replacement type graphs.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} value
 * @param {number} fallback @returns {number[]} */
export function planJavaScriptBindingDefaultInference(texts,starts,counts,labels,targets,value,fallback){
 const kind=readArrayBindingFactField(starts,counts,labels,targets,value,'kind');
 if(kind<0)return [-1];
 if(readRequiredTextFact(texts,kind)==='var')return [];
 const left=readJavaScriptBindingDefaultTerm(texts,starts,counts,labels,targets,value);
 const right=readJavaScriptBindingDefaultTerm(texts,starts,counts,labels,targets,fallback);
 const element=readRequiredNumberFact(left,0),other=readRequiredNumberFact(right,0);
 if(element<0||other<0)return [-1];
 const code=readRequiredNumberFact(left,1),rightCode=readRequiredNumberFact(right,1);
 if(code===0)return [0,value];
 if(code===1)return [1,element];
 const firstKind=readArrayBindingFactField(starts,counts,labels,targets,element,'kind');
 const otherKind=readArrayBindingFactField(starts,counts,labels,targets,other,'kind');
 if(firstKind<0||otherKind<0)return [-1];
 const firstUnit=readRequiredTextFact(texts,firstKind)==='unit',secondUnit=readRequiredTextFact(texts,otherKind)==='unit';
 const output=readJavaScriptBindingDefaultAbsenceCode(readJavaScriptBindingDefaultResultAbsence(
  readJavaScriptBindingDefaultAbsenceName(code),rightCode===0?readRequiredTextFact(texts,otherKind):'option',readJavaScriptBindingDefaultAbsenceName(rightCode)));
 const base=firstUnit?other:element;
 return firstUnit||secondUnit?[output,base]:[output,base,element,other];
}
/** Collapse actual inferred optional wrappers while retaining their original base.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} value
 * @returns {number[]} */
function readJavaScriptBindingDefaultTerm(texts,starts,counts,labels,targets,value){
 let node=value,absence='';
 let kind=readArrayBindingFactField(starts,counts,labels,targets,node,'kind');
 while(kind>=0&&readRequiredTextFact(texts,kind)==='option'){
  const field=readArrayBindingFactField(starts,counts,labels,targets,node,'absence');
  const current=field<0?'both':readRequiredTextFact(texts,field);
  absence=absence===''||absence===current?current:'both';
  node=readArrayBindingFactField(starts,counts,labels,targets,node,'element');
  kind=readArrayBindingFactField(starts,counts,labels,targets,node,'kind');
 }
 return kind<0?[-1,0]:[node,readJavaScriptBindingDefaultAbsenceCode(absence)];
}
/** The lift preserves actual absence tags; a Unit present domain is only undefined.
 * @param {string} kind @param {string} element @param {string} absence
 * @param {boolean} dropUndefined @returns {string} */
export function readJavaScriptBindingDefaultLiftForm(kind,element,absence,dropUndefined){
 if(kind==='unit')return 'undefined';
 if(kind!=='option')return 'present';
 if(element!=='unit')return 'optional';
 return dropUndefined?'null':absence==='both'?'absences':absence;
}
/** @param {string} absence @param {boolean} dropUndefined @returns {string[]} */
export function readJavaScriptBindingDefaultLiftAbsences(absence,dropUndefined){
 if(absence==='null')return ['null'];
 if(absence==='undefined')return dropUndefined?[]:['undefined'];
 return dropUndefined?['null']:['null','undefined'];
}
