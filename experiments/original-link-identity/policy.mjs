// Exact identifier transport only. Ownership and meaning stay with their checked models.
import {LinkId} from '../../js/src/primitives.js';

export const originalLinkIdentityPolicy={
 schemaVersion:1,model:'exact original link identity ordinal lookup 1',
 word:{minimum:0,maximum:4294967295,bits:32,order:['high word','low word']},maximumRows:100000,
 index:'Strictly ascending unique unsigned64 identities represented as two exact uint32 Number words. Zero is a key, never the null-handle sentinel.',
 queries:'Fixed ordered high-word and low-word vectors. Every repeated original identity receives its same positive one-based index ordinal.',
 output:{success:'Exactly one positive ordinal per ordered query; no queries produce an empty vector.',refusals:{index:[-1],queryShape:[-2],queryWord:[-3,'query position'],missing:[-4,'query position']},nullHandle:0,absentHandle:-1},
 assumptions:['Inputs cross the existing dense primitive array boundary; this numeric kernel selects no object descriptor or prototype model.','Sorted exact lookup establishes identity correspondence and membership only. It proves neither source ownership nor semantic equality.','Actual original LinkId objects stay in the host anchor vector; ordinal handles are not replacement identities.','All table words are finite integers in the uint32 domain. No complete u64 is converted to Number.','Fixed-count Array.map constructs the result without mutating inputs or growing an aliased result array.','Public JavaScript LinkId currently coerces to Number. Its unsafe numeric values refuse before BigInt conversion; original lost bits cannot be recovered. Exact BigInt-origin keys are a separate honest input boundary.','Default shared-reference Rust emission currently copies array snapshots for reads. Maximum-size observations execute JavaScript and establish no native resource behavior.','The complete checked kernel is automatically emitted as Rust source data. Native library adaptation, native execution and the complete17-table source certificate projection remain separate work.'],
 feasibility:{
  growth:{source:'/** @param {Array<number>} words @returns {Array<number>} */\nfunction identityGrowth(words){const result=[1];for(let index=0;index<words.length;index++){result.push(words[index]);}return result;}',kind:'unsupported',reason:'method call .push(): .push() has no checked operation on array:float',span:{start:153,end:178}},
  arrayFrom:{source:'/** @param {Array<number>} words @returns {Array<number>} */\nfunction identityFixed(words){return Array.from({length:words.length},(_,index)=>words[index]);}',kind:'unsupported',reason:'Array.from: this iterable needs an operational iterator encoding',span:{start:98,end:155}},
  publicCollisions:[['9007199254740992','9007199254740993'],['18446744073709551614','18446744073709551615']],
 },
};

export function originalLinkIdentityRulesSource(){
 const {maximumRows,word}=originalLinkIdentityPolicy;
 return '// Generated from the experimental exact original-link identity policy. Do not edit.\n'
 + '/** @param {number} value @returns {boolean} */\nexport function acceptOriginalLinkIdentityWord(value){return Number.isFinite(value)&&value>='+word.minimum+'&&value<='+word.maximum+'&&value%1===0;}\n'
 + '/** @param {Array<number>} high @param {Array<number>} low @returns {boolean} */\n'
 + 'function checkOriginalLinkIdentityIndex(high,low){\n'
 + ' if(high.length!==low.length||high.length>'+maximumRows+')return false;\n'
 + ' for(let index=0;index<high.length;index++){\n'
 + '  if(!acceptOriginalLinkIdentityWord(high[index]??-1)||!acceptOriginalLinkIdentityWord(low[index]??-1))return false;\n'
 + '  if(index>0&&!(high[index-1]<high[index]||(high[index-1]===high[index]&&low[index-1]<low[index])))return false;\n'
 + ' }\n return true;\n}\n'
 + '/** The caller owns the checked immutable index. @param {Array<number>} high @param {Array<number>} low @param {number} queryHigh @param {number} queryLow @returns {number} */\n'
 + 'function findOriginalLinkIdentityOrdinal(high,low,queryHigh,queryLow){\n'
 + ' let lower=0;let upper=high.length;\n'
 + ' while(lower<upper){\n'
 + '  const middle=Math.floor((lower+upper)/2);\n'
 + '  if(high[middle]<queryHigh||(high[middle]===queryHigh&&low[middle]<queryLow))lower=middle+1;else upper=middle;\n'
 + ' }\n'
 + ' if(lower>=high.length)return 0;\n'
 + ' return high[lower]===queryHigh&&low[lower]===queryLow?lower+1:0;\n}\n'
 + '/** @param {Array<number>} high @param {Array<number>} low @param {Array<number>} queryHigh @param {Array<number>} queryLow @returns {Array<number>} */\n'
 + 'export function readOriginalLinkIdentityOrdinals(high,low,queryHigh,queryLow){\n'
 + ' if(!checkOriginalLinkIdentityIndex(high,low))return [-1];\n'
 + ' if(queryHigh.length!==queryLow.length||queryHigh.length>'+maximumRows+')return [-2];\n'
 + ' for(let index=0;index<queryHigh.length;index++){\n'
 + '  if(!acceptOriginalLinkIdentityWord(queryHigh[index]??-1)||!acceptOriginalLinkIdentityWord(queryLow[index]??-1))return [-3,index];\n'
 + ' }\n'
 + ' const result=queryHigh.map((value,index)=>findOriginalLinkIdentityOrdinal(high,low,value,queryLow[index]??-1));\n'
 + ' for(let index=0;index<result.length;index++)if(result[index]===0)return [-4,index];\n'
 + ' return result;\n}\n';
}

// Host scalar adaptation, not a second identity or ownership algorithm.
export function exactOriginalLinkIdentity(value){
 if(value instanceof LinkId)value=value.asU64();
 if(typeof value==='number'){
  if(!Number.isSafeInteger(value))throw new RangeError('an unsafe current LinkId Number cannot certify its original bits');
  value=BigInt(value);
 }
 if(typeof value!=='bigint'||value<0n||value>18446744073709551615n)throw new RangeError('an original identity requires an exact unsigned64 value');
 return value;
}
export function originalLinkIdentityWords(value){
 const id=exactOriginalLinkIdentity(value);
 return [Number(id>>32n),Number(id&4294967295n)];
}
export function originalLinkIdentityIndex(values){
 if(!Array.isArray(values)||values.length>originalLinkIdentityPolicy.maximumRows)throw new RangeError('the original identity index exceeds its row bound');
 const anchors=values.map(original=>({original,key:exactOriginalLinkIdentity(original)})).sort((first,second)=>first.key<second.key?-1:first.key>second.key?1:0);
 return {anchors,high:anchors.map(row=>originalLinkIdentityWords(row.key)[0]),low:anchors.map(row=>originalLinkIdentityWords(row.key)[1])};
}
