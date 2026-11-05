// Dense immutable copying sort; mutating sort and comparator effects stay separate.
/** @param {string} kind @param {string} operation @param {string} receiver
 * @param {string} element @param {boolean} reference @param {number} count @returns {boolean} */
export function acceptLinkedStringArrayCopySort(kind,operation,receiver,element,reference,count){return kind==='method'&&operation==='toSorted'&&receiver==='array'&&element==='string'&&!reference&&count===0;}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function requireStringArrayCopySortUnits(kind,operation){return kind==='method'&&operation==='toSorted';}
/** A stable built-in sort compares exact UTF-16 units, never locale or UTF-8.
 * The receiver is evaluated once; the result is an independent dense vector.
 * @param {string} target @param {string} object @param {boolean} units @returns {string} */
export function renderStringArrayCopySort(target,object,units){
 if(target==='JavaScript')return '('+object+').toSorted()';
 if(target!=='Rust')return '';
 const comparison=units?'ml_sort_left.0.cmp(&ml_sort_right.0)':'ml_sort_left.encode_utf16().cmp(ml_sort_right.encode_utf16())';
 return '{ let mut ml_sorted_values = '+object+'; ml_sorted_values.sort_by(|ml_sort_left, ml_sort_right| '+comparison+'); ml_sorted_values }';
}
