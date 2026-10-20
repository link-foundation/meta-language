// Divide canonical decimal digits before binary64 can erase their low bits.
import {acceptOwnedOutcomeIntegerDecimal} from './integer-decimal-rules.mjs';

/** @param {string} text @param {boolean} signed @param {number} width @param {number} projected @returns {boolean} */
export function acceptOwnedOutcomeIntegerProjection(text,signed,width,projected) {
  if(!acceptOwnedOutcomeIntegerDecimal(text,signed,width)||!Number.isFinite(projected))return false;
  if(projected===0&&1/projected<0)return false;
  const negative=text.charAt(0)==='-';
  let digits=text.slice(negative?1:0);
  let power=1;
  let value=0;
  let position=0;
  let first=-1;
  let last=-1;
  while(digits!=='0'){
    let quotient='';
    let carry=0;
    let index=0;
    while(index<digits.length){
      const digit='0123456789'.indexOf(digits.charAt(index));
      const dividend=carry*10+digit;
      const next=Math.trunc(dividend/2);
      carry=dividend-next*2;
      if(quotient.length>0||next>0)quotient+='0123456789'.charAt(next);
      index+=1;
    }
    if(carry===1){
      if(first<0)first=position;
      last=position;
      value+=power;
    }
    digits=quotient.length===0?'0':quotient;
    position+=1;
    power*=2;
  }
  if(last-first>=53)return false;
  return projected===(negative?-value:value);
}
