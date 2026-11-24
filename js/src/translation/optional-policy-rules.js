/** Widen only the absence policy, retaining the complete present domain.
 * @param {string} language @param {string} source @param {string} absence
 * @param {string} target @param {string} policy @param {boolean} sameElement
 * @returns {string} */
export function readJavaScriptOptionalPolicyWidening(language,source,absence,target,policy,sameElement){
 return language==='JavaScript'&&source==='option'&&target==='option'&&sameElement&&policy==='both'&&(absence==='null'||absence==='undefined')?absence:'';
}
