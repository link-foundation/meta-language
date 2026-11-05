// Literal-key classification; arbitrary runtime coercion is a separate operation.
/** @param {number[]} key @returns {number} */
export function readJavaScriptLiteralPropertyIndex(key){
 if(key.length===0||key.length>10)return -1;
 if(key.length>1&&key[0]===48)return -1;
 let value=0;
 for(let at=0;at<key.length;at+=1){
  const digit=key[at]-48;
  if(!Number.isInteger(digit)||digit<0||digit>9)return -1;
  value=value*10+digit;
 }
 return value<=4294967294?value:-1;
}
/** @param {string} kind @param {number} index @returns {string} */
export function readJavaScriptLiteralPropertyForm(kind,index){
 return (kind==='array'||kind==='tuple'||kind==='string')&&index>=0?'index':'field';
}
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptStringIndexNumberForm(kind,element){
 if(kind==='float')return 'identity';
 if(kind==='int'||kind==='nat')return 'convert';
 if(kind==='unit'||kind==='option'&&element==='unit')return 'absent';
 if(kind==='option'&&(element==='float'||element==='int'||element==='nat'))return 'optional';
 return 'refuse';
}
