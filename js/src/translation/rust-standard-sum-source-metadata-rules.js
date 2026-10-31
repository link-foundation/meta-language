// Semantic metadata cost roles. Transport adapters traverse these same records
// without first cloning origins, producing JSON, or enumerating host-only keys.
import {readRustStandardSumSourceMaterializationError} from './rust-standard-sum-source-rules.js';
/** Arrays have ordered children and no stringified numeric index keys.
 * Missing optional fields are represented by one null node.
 * @param {string} form @returns {string[]} */
export function readRustStandardSumSourceMetadataKeys(form){
 if(form==='origin')return ['type','path','wrappers','sourceOwner','expandedSourceOwner','fields','callableParameters','callableResult','floatSourceModel','textSourceModel','arraySourceModel','sourceLifetimes','namespaceSpelling','standardSum'];
 if(form==='owner')return ['kind','path','arguments'];
 if(form==='sum')return ['owner','arguments'];
 if(form==='identity value')return ['type','owner','wrappers','fields','float','text'];
 if(form==='identity sum')return ['owner','arguments','wrappers'];
 if(form==='identity header')return ['type','path','owner','wrappers'];
 if(form==='fixed')return ['kind','bits','signed'];
 if(form==='named')return ['kind','path'];
 if(form==='data')return ['kind','name'];
 if(form==='array'||form==='set')return ['kind','element','reference'];
 if(form==='cell')return ['kind','element'];
 if(form==='option')return ['kind','element','absence'];
 if(form==='absence')return ['kind','absence'];
 if(form==='tuple')return ['kind','elements'];
 if(form==='union')return ['kind','alternatives'];
 if(form==='record')return ['kind','fields','ordered','open','reference'];
 if(form==='record field')return ['name','type'];
 if(form==='map')return ['kind','key','value','reference'];
 if(form==='function')return ['kind','params','ret'];
 return form==='primitive'?['kind']:[];
}
/** @param {string} kind @returns {string} */
export function readRustStandardSumSourceMetadataTypeForm(kind){
 return ['fixed','named','data','array','cell','option','absence','tuple','union','record','map','set','function'].includes(kind)?kind:['nat','int','bool','string','unit','float','literal','hypothesis','output'].includes(kind)?'primitive':'';
}
/** One record node plus all canonical property-name UTF16 units.
 * @param {string} form @returns {number[]} */
export function readRustStandardSumSourceMetadataRecordCost(form){
 const keys=readRustStandardSumSourceMetadataKeys(form);if(keys.length===0)return [];
 let units=0;for(let at=0;at<keys.length;at+=1)units+=(keys[at]??'').length;
 return [1,units];
}
/** @param {number} nodes @param {number} units @param {number} depth @returns {string} */
export function readRustStandardSumSourceMetadataCountError(nodes,units,depth){
 const error=readRustStandardSumSourceMaterializationError(0,0,nodes,units);
 return error||!Number.isInteger(depth)||depth<0||depth>256?'the complete standard sum source metadata exceeds its finite node, UTF16 or depth budget':'';
}
/** Sixteen semantic copies reserve identity encoding and the bounded owned
 * constructor/catalog transports together. Counters never reset within a parser.
 * @param {number} nodes @param {number} units @param {number} costNodes
 * @param {number} costUnits @returns {number[]} */
export function readRustStandardSumSourceMetadataReservation(nodes,units,costNodes,costUnits){
 const additionalNodes=costNodes*16,additionalUnits=costUnits*16;
 return readRustStandardSumSourceMaterializationError(nodes,units,additionalNodes,additionalUnits)===''?[nodes+additionalNodes,units+additionalUnits]:[];
}
