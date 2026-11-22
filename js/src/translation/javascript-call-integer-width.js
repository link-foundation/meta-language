/** The selected resource profile bounds intermediate work separately from
 * arbitrary-integer language semantics. -1 denotes an oversized input.
 * @param {bigint} value @returns {number} */
export function readJavaScriptCallIntegerWidth(value){
 let rest=value<0n?-value:value,width=0;
 while(rest>0n){if(width>=4096)return -1;rest/=2n;width+=1;}
 return width;
}
