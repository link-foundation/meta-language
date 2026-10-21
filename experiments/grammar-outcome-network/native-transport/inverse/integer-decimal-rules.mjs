// Exact source integer spelling, before a binary64 projection can erase it.
// Signed values name i64; unsigned values name an explicit 32/64-bit usize.

/** @param {boolean} negative @param {boolean} signed @param {number} width @returns {string} */
export function readOwnedOutcomeIntegerDecimalLimit(negative,signed,width) {
  if(width!==32&&width!==64)return '';
  if(signed)return negative?'9223372036854775808':'9223372036854775807';
  if(negative)return '';
  return width===32?'4294967295':'18446744073709551615';
}

/** @param {string} text @param {boolean} signed @param {number} width @returns {boolean} */
export function acceptOwnedOutcomeIntegerDecimal(text,signed,width) {
  const count=text.length;
  if(count===0||count>20)return false;
  const negative=text.charAt(0)==='-';
  const offset=negative?1:0;
  const limit=readOwnedOutcomeIntegerDecimalLimit(negative,signed,width);
  const digits=count-offset;
  if(limit.length===0||digits===0||digits>limit.length)return false;
  if(text.charAt(offset)==='0'&&(negative||digits>1))return false;
  let smaller=digits<limit.length;
  let larger=false;
  let index=offset;
  while(index<count){
    const digit='0123456789'.indexOf(text.charAt(index));
    if(digit<0)return false;
    if(!smaller&&!larger){
      const boundary='0123456789'.indexOf(limit.charAt(index-offset));
      if(digit<boundary)smaller=true;
      if(digit>boundary)larger=true;
    }
    index+=1;
  }
  return !larger;
}
