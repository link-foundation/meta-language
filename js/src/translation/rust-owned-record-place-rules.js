/** @returns {number} */
export function readRustOwnedRecordPlaceDepthLimit(){return 256;}
/** Only complete parenthesized/local-field syntax is a non-consuming place.
 * Calls and blocks retain their ordinary whole-value ownership checks.
 * @param {string[]} forms @returns {number} */
export function readRustOwnedRecordPlaceTokenCount(forms){
 let depth=0,atom=true;
 for(let at=0;at<forms.length;at+=1){
  const form=forms[at];
  if(atom){if(form==='('){depth+=1;if(depth>=readRustOwnedRecordPlaceDepthLimit())return -1;}else if(form==='name')atom=false;else return -1;}
  else if(form==='.'){at+=1;if(forms[at]!=='name'&&forms[at]!=='number')return -1;}
  else if(form===')'){if(depth===0)return -1;depth-=1;}
  else if(form==='}')return depth===0?at:-1;
  else return -1;
 }
 return -1;
}
