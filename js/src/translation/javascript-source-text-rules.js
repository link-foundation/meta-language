// Successor resource profile. The original unit-array text profile is intact.
/** @param {string} profile @param {number} length @returns {string} */
export function readJavaScriptSourceTextProfileError(profile,length){
 if(profile!=='inspectable UTF16 source text 1')return 'source text requires its explicit inspectable UTF16 resource profile';
 if(!Number.isSafeInteger(length)||length<0||length>16777216||(length===0&&1/length<0))return 'source text exceeds the explicit 16777216 UTF16 unit budget';
 return '';
}
