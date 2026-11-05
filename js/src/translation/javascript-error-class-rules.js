// The successor is separate from ordinary closed-record classes. No old
// inheritance/default assertion is changed by selecting this explicit model.
import {findJavaScriptClassGroupEnd} from './javascript-class-source-rules.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readRequiredTextFact} from './fact-column-rules.js';
/** @param {string[]} values @param {number} at @returns {string} */
function text(values,at){return at>=0&&at<values.length?readRequiredTextFact(values,at):'';}
/** @param {string} profile @returns {string} */
export function readJavaScriptErrorClassProfileError(profile){return profile==='actual intrinsic Error subclasses 1'?'':'Error subclasses require their explicit intrinsic/realm construction profile';}
/** Header/constructor statement coordinates only. Expressions and lexical
 * references are read by the ordinary source/CST transport, never executed.
 * Result: [classEnd,parameterOpen,parameterClose,superFirst,superLast,fieldName,
 * fieldValueFirst,fieldValueLast,...]. Negative first element is refusal.
 * @param {string[]} values @param {string[]} kinds @param {number} first @returns {number[]} */
export function readJavaScriptErrorClassDirectory(values,kinds,first){
 if(!Number.isSafeInteger(first)||first<0||values.length!==kinds.length||values.length>8192)return [-1];
 if(text(values,first)!=='class'||text(kinds,first+1)!=='identifier'||text(values,first+2)!=='extends'||text(kinds,first+3)!=='identifier'||readJavaScriptSourceExceptionOperation(text(values,first+3))===''||text(values,first+4)!=='{')return [-1];
 const end=findJavaScriptClassGroupEnd(values,first+4);if(end<0)return [-1];let at=first+5;
 if(text(values,at)!=='constructor'||text(values,at+1)!=='(')return [-2];const open=at+1,close=findJavaScriptClassGroupEnd(values,open);if(close<0||text(values,close+1)!=='{')return [-2];const body=close+1,bodyEnd=findJavaScriptClassGroupEnd(values,body);if(bodyEnd<0||bodyEnd+1!==end)return [-2];
 at=body+1;if(text(values,at)!=='super'||text(values,at+1)!=='(')return [-3];const superEnd=findJavaScriptClassGroupEnd(values,at+1);if(superEnd<0||text(values,superEnd+1)!==';')return [-3];
 let plan=[end,open,close,at+2,superEnd];at=superEnd+2;
 while(at<bodyEnd){if(text(values,at)!=='this'||text(values,at+1)!=='.'||text(kinds,at+2)!=='identifier'||text(values,at+3)!=='=')return [-4];const name=at+2,start=at+4;let last=start;for(;last<bodyEnd;last+=1){const value=text(values,last);if(value===';')break;if(['(','[','{'].includes(value)){const group=findJavaScriptClassGroupEnd(values,last);if(group<0||group>=bodyEnd)return [-4];last=group;}if(value==='return'||value==='super')return [-4];}if(last<=start||last>=bodyEnd||text(values,name)==='__proto__')return [-4];plan=plan.concat([name,start,last]);at=last+1;}
 return plan;
}
/** @param {number} code @returns {string} */
export function readJavaScriptErrorClassRefusal(code){if(code===-1)return 'the Error successor needs an actual named class extending a supplied standard Error constructor';if(code===-2)return 'this Error successor needs its complete single ordinary constructor rather than unmodeled fields, methods or descriptor effects';if(code===-3)return 'a dominant explicit super call must initialize this before the ordered field body';return 'Error constructor fields require complete ordered this assignments; returns, repeated super, prototype mutation and other statements retain their source obligations';}
