// Source graph coordinates acquire explicit exact identities, never meaning proofs.
import {acceptUnsignedLinkIdentity,advanceUnsignedLinkIdentity,readUnsignedLinkLimb} from './unsigned-link-identity-rules.js';
/** @param {number[]} first @param {number} count @param {number} limit @returns {string} */
export function readUnsignedSourceAllocationError(first,count,limit){
 if(!acceptUnsignedLinkIdentity(first))return 'source graph allocation needs a positive unsigned first identity';
 if(!Number.isSafeInteger(limit)||limit<1||!Number.isInteger(count)||count<0||count>4294967295||count>limit)return 'source graph allocation exceeds its identity directory limit';
 const lower=readUnsignedLinkLimb(first,0)+(count>0?count-1:0),upper=readUnsignedLinkLimb(first,1)+Math.floor(lower/4294967296);
 return upper>=4294967296?'source graph allocation exceeds the unsigned identity space':'';
}
/** Header: count, next low/high (-1/-1 if exhausted), followed by ordered ID limbs.
 * @param {number[]} first @param {number} count @param {number} limit @returns {number[]} */
export function allocateUnsignedSourceIdentities(first,count,limit){
 if(readUnsignedSourceAllocationError(first,count,limit)!=='')return [];
 let cursor=first,identities=[];
 for(let at=0;at<count;at+=1){identities.push(readUnsignedLinkLimb(cursor,0));identities.push(readUnsignedLinkLimb(cursor,1));cursor=advanceUnsignedLinkIdentity(cursor);}
 return [count,cursor.length===0?-1:readUnsignedLinkLimb(cursor,0),cursor.length===0?-1:readUnsignedLinkLimb(cursor,1)].concat(identities);
}
