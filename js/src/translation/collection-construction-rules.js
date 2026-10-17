// Complete existing collection construction decisions. Hosts retain the
// actual term owners, evaluate children and use the common unifier.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {number} */
export function readCollectionConstructionField(starts, counts, labels, targets, node, field) {
  if (node < 0) return -1;
  const start = readRequiredNumberFact(starts, node);
  const count = readRequiredNumberFact(counts, node);
  for (let at = start; at < start + count; at += 1) {
    if (readRequiredTextFact(labels, at) === field) return readRequiredNumberFact(targets, at);
  }
  return -1;
}

/** @param {number[]} starts @param {number[]} counts @param {number[]} targets @param {number} node @param {number} index @returns {number} */
export function readCollectionConstructionElement(starts, counts, targets, node, index) {
  if (node < 0 || index < 0 || index >= readRequiredNumberFact(counts, node)) return -1;
  return readRequiredNumberFact(targets, readRequiredNumberFact(starts, node) + index);
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @returns {string} */
export function readCollectionConstructionKind(texts, starts, counts, labels, targets, node) {
  const kind = readCollectionConstructionField(starts, counts, labels, targets, node, 'kind');
  return kind < 0 ? '' : readRequiredTextFact(texts, kind);
}

/** [form,context,first,second]: form one selects Set, two Map. Context one
 * retains original expected child owners; zero requests fresh variables.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {string} kind
 * @param {number} expected @returns {number[]}
 */
export function planCollectionConstructionInference(texts, starts, counts, labels, targets, kind, expected) {
  const context = readCollectionConstructionKind(texts, starts, counts, labels, targets, expected) === kind;
  if (kind === 'set') return [1, context ? 1 : 0, context ? readCollectionConstructionField(starts, counts, labels, targets, expected, 'element') : -1, -1];
  if (kind === 'map') return [2, context ? 1 : 0, context ? readCollectionConstructionField(starts, counts, labels, targets, expected, 'key') : -1, context ? readCollectionConstructionField(starts, counts, labels, targets, expected, 'value') : -1];
  return [0, 0, -1, -1];
}

/** [error,first,second] preserves original iterable shape checks and actual
 * element/key/value identities. Primitive admission remains a shared query.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {string} kind
 * @param {number} items @returns {number[]}
 */
export function planCollectionConstructionChecked(texts, starts, counts, labels, targets, kind, items) {
  const array = readCollectionConstructionKind(texts, starts, counts, labels, targets, items) === 'array';
  const element = readCollectionConstructionField(starts, counts, labels, targets, items, 'element');
  if (kind === 'set') return array ? [0, element, -1] : [1, -1, -1];
  const tuple = readCollectionConstructionKind(texts, starts, counts, labels, targets, element) === 'tuple';
  const elements = readCollectionConstructionField(starts, counts, labels, targets, element, 'elements');
  if (!array || !tuple || elements < 0 || readRequiredNumberFact(counts, elements) !== 2) return [2, -1, -1];
  return [0, readCollectionConstructionElement(starts, counts, targets, elements, 0), readCollectionConstructionElement(starts, counts, targets, elements, 1)];
}

/** @param {string} kind @param {boolean} primitive @returns {number} */
export function readCollectionConstructionKeyAdmission(kind, primitive) {
  return primitive ? 0 : kind === 'set' ? 3 : 4;
}

/** @param {number} code @param {string} part @returns {string} */
export function readCollectionConstructionDiagnostic(code, part) {
  if (part === 'message' && code === 1) return 'Set constructor needs an iterable array';
  if (part === 'message' && code === 2) return 'Map constructor needs an iterable of key/value pairs';
  if (part === 'message' && code === 3) return 'set keys need explicit reference identities for structured values';
  if (part === 'message' && code === 4) return 'map keys need explicit reference identities for structured values';
  if (part === 'construct' && code === 3) return 'Set object identity';
  if (part === 'construct' && code === 4) return 'Map object identity';
  return '';
}

/** The host's ordinary Rocq comparison keeps ownership of its type/runtime
 * helpers; this supplies exactly its original source operand spellings.
 * @param {string} target @param {string} kind @param {string} role @returns {string}
 */
export function readCollectionConstructionBinding(target, kind, role) {
  const set = kind === 'set';
  if (target === 'Rocq') return role === 'previous' ? set ? 'ml_previous' : '(fst ml_previous)' : set ? 'ml_value' : '(fst ml_value)';
  return role === 'previous' ? set ? 'ml_previous' : 'ml_previous.1' : set ? 'ml_value' : 'ml_value.1';
}

/** @param {string} target @param {string} kind @param {string} first @param {string} second @returns {string} */
export function renderCollectionConstructionType(target, kind, first, second) {
  if (target === 'Lean') return kind === 'set' ? '(Array ' + first + ')' : '(Array (' + first + ' × ' + second + '))';
  if (target === 'Rocq') return kind === 'set' ? '(list ' + first + ')' : '(list (' + first + ' * ' + second + '))%type';
  return '';
}

/** Exact existing constructor emission, including first insertion order,
 * Map replacement, SameValueZero, and canonical positive zero keys.
 * @param {string} target @param {string} kind @param {string} keyKind
 * @param {string} values @param {string} collectionType @param {string} elementType
 * @param {string} comparison @returns {string}
 */
export function renderCollectionConstruction(target, kind, keyKind, values, collectionType, elementType, comparison) {
  const set = kind === 'set';
  if (target === 'JavaScript') return 'new ' + (set ? 'Set' : 'Map') + '(' + values + ')';
  if (target === 'Rust') {
    const comparator = keyKind === 'float' ? '|a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())' : '|a, b| a == b';
    if (keyKind === 'float') values = '(' + values + ').into_iter().map(|ml_value| ' + (set ? 'if ml_value == 0f64 { 0f64 } else { ml_value }' : '(if ml_value.0 == 0f64 { 0f64 } else { ml_value.0 }, ml_value.1)') + ').collect::<Vec<_>>()';
    return 'crate::ml_array::' + kind + '(' + values + ', ' + comparator + ')';
  }
  const value = readCollectionConstructionBinding(target, kind, 'value');
  const previous = readCollectionConstructionBinding(target, kind, 'previous');
  if (target === 'Lean') {
    const equal = keyKind === 'float' ? '(' + previous + ' == ' + value + ' || (' + previous + '.isNaN && ' + value + '.isNaN))' : '(' + previous + ' == ' + value + ')';
    const normalized = keyKind === 'float' ? set ? '(if ml_original == (0 : Float) then (0 : Float) else ml_original)' : '((if ml_original.1 == (0 : Float) then (0 : Float) else ml_original.1), ml_original.2)' : 'ml_original';
    const existing = set ? 'ml_accumulator' : '(ml_accumulator.map (fun ml_previous => if ' + equal + ' then (ml_previous.1, ml_value.2) else ml_previous))';
    return '(' + values + '.foldl (fun ml_accumulator ml_original => let ml_value := ' + normalized + '; if ml_accumulator.any (fun ml_previous => ' + equal + ') then ' + existing + ' else ml_accumulator.push ml_value) (#[] : ' + collectionType + '))';
  }
  let equal = comparison;
  if (keyKind === 'float') equal = '(orb ' + equal + ' (andb (PrimFloat.is_nan ' + previous + ') (PrimFloat.is_nan ' + value + ')))';
  const normalized = keyKind === 'float' ? set ? '(if PrimFloat.eqb ml_original 0%float then 0%float else ml_original)' : '((if PrimFloat.eqb (fst ml_original) 0%float then 0%float else fst ml_original), snd ml_original)' : 'ml_original';
  const existing = set ? 'ml_accumulator' : '(List.map (fun ml_previous => if ' + equal + ' then (fst ml_previous, snd ml_value) else ml_previous) ml_accumulator)';
  return '(List.fold_left (fun ml_accumulator ml_original => let ml_value := ' + normalized + ' in if List.existsb (fun ml_previous => ' + equal + ') ml_accumulator then ' + existing + ' else List.app ml_accumulator (ml_value :: nil)) ' + values + ' (@nil ' + elementType + '))';
}
