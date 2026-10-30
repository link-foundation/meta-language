// Binding candidates and complete default/computed intervals come from actual
// CST nodes. Initializers remain original source, never executed by this pass.
/** @param {string} term @returns {boolean} */
export function acceptJavaScriptSourceParameterIdentifierTerm(term){return term==='identifier'||term==='shorthand_property_identifier_pattern';}
/** @param {string} term @returns {boolean} */
export function acceptJavaScriptSourceBindingRootTerm(term){return term==='identifier'||term==='object_pattern'||term==='array_pattern';}
/** @param {number[]} firsts @param {number[]} lasts @param {number} at @returns {boolean} */
function containsSourceParameterInterval(firsts,lasts,at){
 for(let slot=0;slot<firsts.length;slot+=1)if(at>=(firsts[slot]??-1)&&at<(lasts[slot]??-1))return true;
 return false;
}
/** @param {string[]} kinds @param {string[]} values @param {number[]} firsts
 * @param {number[]} lasts @param {number} first @param {number} last @returns {number} */
function findJavaScriptSourceParameterDefaultSeparator(kinds,values,firsts,lasts,first,last){
 let closes=[];
 for(let at=first;at<last;at+=1){
  if(containsSourceParameterInterval(firsts,lasts,at)||(kinds[at]??'')!=='punct')continue;
  const value=values[at]??'';
  if(value==='='&&!closes.length)return at;
  if(value==='('||value==='['||value==='{')closes=closes.concat([value==='('?')':value==='['?']':'}']);
  else if(value===')'||value===']'||value==='}'){
   if(!closes.length||closes[closes.length-1]!==value)return -1;
   closes=closes.slice(0,closes.length-1);
  }
 }
 return -1;
}
/** @param {number[]} firsts @param {number[]} lasts @param {number} length @returns {boolean} */
function acceptJavaScriptSourceParameterIntervals(firsts,lasts,length){
 if(firsts.length!==lasts.length)return false;
 for(let at=0;at<firsts.length;at+=1){
  const first=firsts[at]??-1,last=lasts[at]??-1;
  if(!Number.isSafeInteger(first)||!Number.isSafeInteger(last)||first<0||last<first||last>length)return false;
 }
 return true;
}
/** Default nodes arrive from inner to outer by token interval width. Computed
 * property keys and already selected nested initializer intervals stay opaque.
 * -1 is an invalid-column diagnostic, distinct from a valid empty census.
 * @param {string[]} kinds @param {string[]} values @param {boolean[]} candidates
 * @param {number[]} defaultFirsts @param {number[]} defaultLasts
 * @param {number[]} computedFirsts @param {number[]} computedLasts @returns {number[]} */
export function planJavaScriptSourceParameterBindings(kinds,values,candidates,defaultFirsts,defaultLasts,computedFirsts,computedLasts){
 const length=kinds.length;
 if(values.length!==length||candidates.length!==length||!acceptJavaScriptSourceParameterIntervals(defaultFirsts,defaultLasts,length)||!acceptJavaScriptSourceParameterIntervals(computedFirsts,computedLasts,length))return [-1];
 let firsts=computedFirsts.slice(),lasts=computedLasts.slice(),previous=-1;
 for(let slot=0;slot<defaultFirsts.length;slot+=1){
  const first=defaultFirsts[slot]??-1,last=defaultLasts[slot]??-1,width=last-first;
  if(width<previous)return [-1];previous=width;
  let masked=false;
  for(let range=0;range<firsts.length;range+=1)if(first>=(firsts[range]??-1)&&last<=(lasts[range]??-1))masked=true;
  if(masked)continue;
  const equals=findJavaScriptSourceParameterDefaultSeparator(kinds,values,firsts,lasts,first,last);
  if(equals<0)return [-1];
  firsts=firsts.concat([equals+1]);lasts=lasts.concat([last]);
 }
 let positions=[];
 for(let at=0;at<length;at+=1)if((candidates[at]??false)&&!containsSourceParameterInterval(firsts,lasts,at))positions=positions.concat([at]);
 return positions;
}
