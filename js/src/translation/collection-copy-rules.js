// Exact standard same-kind copy facts. General/custom iterable admission and
// primitive-key policies stay with the existing collection constructor path.
import {readCollectionConstructionField,readCollectionConstructionKind} from './collection-construction-rules.js';
/** An unresolved operand first waits for ordinary source constraints. The
 * settled fallback is the pre-existing array-only inference contract.
 * @param {string} target @param {string} source @param {boolean} settled
 * @returns {string} */
export function readCollectionCopyInferenceRole(target,source,settled){return (target==='map'||target==='set')&&source===target?'copy':source==='var'&&!settled?'wait':'legacy array';}
/** @returns {string} */
export function readCollectionCopySourceNormalization(){return 'arrayFrom';}
/** @param {string} kind @returns {string} */
export function readCollectionCopySnapshotOperation(kind){return kind==='map'?'entries':kind==='set'?'values':'';}
/** @param {string} kind @param {boolean} reference @returns {boolean} */
export function acceptCollectionCopyArrayFromSnapshot(kind,reference){return reference&&(kind==='map'||kind==='set');}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptCollectionCopyMapperSnapshot(kind,operation){const actual=readCollectionCopySnapshotOperation(kind);return actual!==''&&actual===operation;}
/** Factories evaluate before iteration; their unknown returned callbacks cannot
 * certify readonly mapping or snapshot effect order.
 * @param {boolean} snapshot @param {boolean} readonly @returns {string} */
export function readCollectionCopyMapperError(snapshot,readonly){return snapshot&&!readonly?'a mutable collection snapshot mapper needs its complete readonly callback graph; factories and live iterator mutations need their original ordered iterator model':'';}
/** Actual checked type endpoints retain the original key/value owners.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {string} kind
 * @param {number} input @returns {number[]} */
export function planCollectionCopyInput(texts,starts,counts,labels,targets,kind,input){
 if((kind!=='map'&&kind!=='set')||readCollectionConstructionKind(texts,starts,counts,labels,targets,input)!==kind)return [0,-1,-1];
 const first=readCollectionConstructionField(starts,counts,labels,targets,input,kind==='set'?'element':'key');
 const second=kind==='map'?readCollectionConstructionField(starts,counts,labels,targets,input,'value'):-1;
 return first<0||kind==='map'&&second<0?[0,-1,-1]:[kind==='set'?1:2,first,second];
}
