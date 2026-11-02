import {readJavaScriptWeakCollectionOperationKind} from './javascript-weak-collection-rules.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit} from './javascript-semantic-values.js';
/** @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right @returns {boolean} */
export function testJavaScriptCollectionKey(left,right){return left.$==='number'&&right.$==='number'?(left.value===right.value||Number.isNaN(left.value)&&Number.isNaN(right.value)):testJavaScriptSemanticSameValue(left,right);}
/** Storage normalizes both signs of Number zero as required by Map/Set.
 * @param {JavaScriptSemanticValue} key @returns {JavaScriptSemanticValue} */
export function normalizeJavaScriptCollectionKey(key){return key.$==='number'&&key.value===0?{$:'number',value:0}:key;}
/** @param {string} operation @returns {string} */
export function readJavaScriptCollectionOperationKind(operation){return ['Map.get','Map.set','Map.has','Map.delete','Map.clear','Map.size'].includes(operation)?'Map':['Set.add','Set.has','Set.delete','Set.clear','Set.size'].includes(operation)?'Set':readJavaScriptWeakCollectionOperationKind(operation);}
/** @param {JavaScriptSemanticValue} value @returns {number} */
export function readJavaScriptCollectionKeyRank(value){switch(value.$){case'undefined':return 0;case'null':return 1;case'boolean':return 2;case'number':return 3;case'integer':return 4;case'text':case'sourceText':return 5;case'symbol':return 6;case'object':return 7;}return -1;}
/** Deterministic validation order, not observable Map iteration order.
 * @param {JavaScriptSemanticValue} left @param {JavaScriptSemanticValue} right @returns {number} */
export function compareJavaScriptCollectionKeys(left,right){
 const a=readJavaScriptCollectionKeyRank(left),b=readJavaScriptCollectionKeyRank(right);if(a!==b)return a<b?-1:1;
 if(left.$==='number'&&right.$==='number'){if(testJavaScriptCollectionKey(left,right))return 0;if(Number.isNaN(left.value))return 1;if(Number.isNaN(right.value))return -1;return left.value<right.value?-1:1;}
 if(left.$==='integer'&&right.$==='integer')return left.value===right.value?0:left.value<right.value?-1:1;
 if(left.$==='boolean'&&right.$==='boolean')return left.value===right.value?0:left.value?1:-1;
 if((left.$==='object'||left.$==='symbol')&&(right.$==='object'||right.$==='symbol'))return left.identity===right.identity?0:left.identity<right.identity?-1:1;
 if(a===5){
  if(left.$==='sourceText'&&right.$==='sourceText')return left.text===right.text?0:left.text<right.text?-1:1;
  const first=readJavaScriptSemanticTextLength(left),second=readJavaScriptSemanticTextLength(right),length=Math.min(first,second);
  for(let index=0;index<length;index++){const x=readJavaScriptSemanticTextUnit(left,index),y=readJavaScriptSemanticTextUnit(right,index);if(x!==y)return x<y?-1:1;}
  return first===second?0:first<second?-1:1;
 }
 return 0;
}
/** Bottom-up merge ordering keeps validation bounded without host comparator
 * callbacks or quadratic insertion scans. Input order remains untouched.
 * @param {JavaScriptSemanticValue[]} values @returns {JavaScriptSemanticValue[]} */
export function orderJavaScriptCollectionKeys(values){
 let source=values.slice();
 for(let width=1;width<source.length;width*=2){
  const target=[];
  for(let start=0;start<source.length;start+=width*2){
   const middle=Math.min(start+width,source.length),end=Math.min(start+width*2,source.length);
   let left=start,right=middle;
   while(left<middle||right<end){
    const a=source[left],b=source[right];
    if(right>=end||left<middle&&a!==undefined&&b!==undefined&&compareJavaScriptCollectionKeys(a,b)<=0){
     if(a!==undefined)target.push(a);left++;
    }else{if(b!==undefined)target.push(b);right++;}
   }
  }
  source=target;
 }
 return source;
}
