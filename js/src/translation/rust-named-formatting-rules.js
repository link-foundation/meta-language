// Actual source argument names and normalized field terms enter this shared
// planner. Hosts project token/AST text only; they never resolve argument order.
import {readRustFormattingUnusedPosition} from './rust-formatting-rules.js';

/** @param {string[]} names @returns {string} */
export function readRustFormattingArgumentNameError(names){
 if(names.length>100000)return 'named format arguments exceed their source budget';
 let named=false,work=0;
 for(let index=0;index<names.length;index+=1){
  const name=names[index]??'';
  if(name===''){if(named)return 'positional format arguments cannot follow named arguments';}
  else{
   named=true;
   for(let previous=0;previous<index;previous+=1){
    work+=1;if(work>1000000)return 'named format arguments exceed their source budget';
    if(names[previous]===name)return 'format argument name is repeated';
   }
  }
 }
 return '';
}

/** Resolve complete grammar terms to actual operand positions. Named fields
 * absent from the explicit arguments retain their lexical-capture term. All
 * provided operands, including named ones, must occur at least once. A numeric
 * field addresses its original source index without moving the implicit cursor.
 * Every actual argument is captured once before applying Display.
 * Error -9 is an invalid name sequence, -8 malformed projected facts; existing
 * grammar errors and the original -7 unused-operand diagnosis remain intact.
 * @param {number[]} plan @param {string[]} names @param {string[]} fields
 * @returns {number[]} */
export function planRustNamedFormattingValue(plan,names,fields){
 if(readRustFormattingArgumentNameError(names)!=='')return [-9,0,0];
 if(plan.length===0)return [-8,0,0];
 if(plan[0]!==0)return plan;
 if((plan.length-1)%3!==0||plan.length>300001)return [-8,0,0];
 const result=[0],used=[];
 let named=0,work=0;
 for(let at=1;at<plan.length;at+=3){
  const code=plan[at]?? -1,first=plan[at+1]?? -1,last=plan[at+2]?? -1;
  if(!Number.isSafeInteger(first)||!Number.isSafeInteger(last)||first<0||last<0)return [-8,0,0];
  if(code===0){if(last<first)return [-8,0,0];result.push(0);result.push(first);result.push(last);}
  else if(code===1){
   if(first>=names.length||last!==0)return [-8,0,0];
   used.push(first);result.push(1);result.push(first);result.push(0);
  }else if(code===2){
   if(last<=first||named>=fields.length)return [-8,0,0];
   const name=fields[named]??'';named+=1;
   if(name==='')return [-8,0,0];
   let position=-1;
   for(let index=0;index<names.length;index+=1){work+=1;if(work>1000000)return [-8,0,0];if(names[index]===name)position=index;}
   if(position>=0){used.push(position);result.push(1);result.push(position);result.push(0);}
   else{result.push(2);result.push(first);result.push(last);}
  }else return [-8,0,0];
 }
 if(named!==fields.length)return [-8,0,0];
 return readRustFormattingUnusedPosition(used,names.length,result);
}
