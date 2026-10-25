// A JavaScript Unit value denotes undefined; an optional Unit is also nullish
// even when its actual constructor is present. Other language Units differ.
/** @param {string} language @param {string} kind @param {string} element
 * @returns {boolean} */
export function requireJavaScriptAbsenceOnlyValue(language,kind,element){
 return language==='JavaScript'&&(kind==='unit'||kind==='option'&&element==='unit');
}
