// Additive source-position decisions. Historical signature profiles remain exact.
import {acceptRustRetainedFunctionOwner} from './rust-source-owner-retention-rules.js';
/** The existing ordinary profile stays exact. This successor composes only
 * its own signature with the complete otherwise-empty source profile vector.
 * @param {string} signature @param {string[]} profiles @returns {boolean} */
export function acceptRustBorrowedVectorCompletionProfiles(signature,profiles){
 if(!acceptRustBorrowedVectorSignatureModel(signature)||profiles.length!==11||profiles[1]!==signature)return false;
 for(let index=0;index<profiles.length;index+=1)if(index!==1&&profiles[index]!=='')return false;
 return true;
}
/** @param {string[]} profiles @returns {boolean} */
export function acceptRustBorrowedVectorInherentProfiles(profiles){if(profiles.length!==11||!acceptRustBorrowedVectorSignatureModel(profiles[2]??''))return false;for(let index=0;index<profiles.length;index+=1)if(index!==2&&profiles[index]!=='')return false;return true;}
/** @param {string} model @param {boolean} ordinary @param {number} minor @returns {boolean} */
export function acceptRustBorrowedVectorSource(model,ordinary,minor){return acceptRustBorrowedVectorSignatureModel(model)&&ordinary&&Number.isInteger(minor)&&minor>=90&&minor<100;}
/** @param {string} model @returns {boolean} */
export function acceptRustBorrowedVectorSignatureModel(model){return model==='Rust source function signature 3';}
/** @param {string[]} wrappers @param {boolean} child @returns {boolean} */
export function retainRustBorrowedVectorElementPosition(wrappers,child){return wrappers.includes('reference')||child;}
/** Legacy owned graphs cannot interpret a retained borrowed child by erasing it.
 * @param {boolean} child @returns {boolean} */
export function acceptRustBorrowedVectorOwnedInterpretation(child){return !child;}
/** @param {string[]} wrappers @param {number} lifetimes @param {boolean} nested
 * @returns {boolean} */
export function acceptRustBorrowedVectorPosition(wrappers,lifetimes,nested){return wrappers.length===1&&wrappers[0]==='reference'&&lifetimes===1&&!nested;}
/** @param {string} model @param {boolean} child @returns {string} */
export function readRustBorrowedVectorPositionError(model,child){return child&&!acceptRustBorrowedVectorSignatureModel(model)?'a retained array element position requires the borrowed vector source signature profile':'';}
/** @param {string} kind @param {string} owner @param {string} path
 * @param {number} argumentsCount @param {boolean} typeEqual
 * @param {boolean} ownerEqual @returns {string} */
export function readRustBorrowedVectorElementPositionError(kind,owner,path,argumentsCount,typeEqual,ownerEqual){return kind!=='array'||owner!=='library'||path!=='std::vec::Vec'||argumentsCount!==1||!typeEqual||!ownerEqual?'a retained array element position must match its complete canonical vector element type and owner':'';}
/** @param {boolean} selected @param {number} minor @param {string[]} wrappers
 * @param {string[]} lifetimes @param {boolean} known @param {boolean} mutable
 * @returns {boolean} */
export function acceptRustBorrowedVectorElement(selected,minor,wrappers,lifetimes,known,mutable){return selected&&Number.isInteger(minor)&&minor>=90&&minor<=99&&known&&!mutable&&wrappers.length===1&&wrappers[0]==='reference'&&lifetimes.length===1;}
/** @param {number} visits @param {number} depth @returns {string} */
export function readRustBorrowedVectorPositionBudgetError(visits,depth){return !Number.isSafeInteger(visits)||visits<0||visits>100000||!Number.isSafeInteger(depth)||depth<0||depth>128?'retained borrowed vector element positions exceed their finite source profile':'';}
/** Reuse the complete safe function-owner shape without changing its old model gate.
 * @param {string} model @param {string} owner @param {string} type
 * @param {number} paths @param {number} children @param {number} parameters
 * @param {boolean} reference @returns {boolean} */
export function acceptRustBorrowedVectorFunctionOwner(model,owner,type,paths,children,parameters,reference){return acceptRustBorrowedVectorSignatureModel(model)&&acceptRustRetainedFunctionOwner('Rust source function signature 2',owner,type,paths,children,parameters,reference);}
/** Header deferral retains syntax and owners without granting target capability.
 * @param {boolean} selected @param {boolean} reference @param {boolean} header
 * @param {boolean} complete @param {boolean} closed @returns {string} */
export function readRustBorrowedVectorElementPhase(selected,reference,header,complete,closed){if(!selected||!reference)return '';if(header&&!complete)return 'defer';return complete&&closed?'admit':'';}
/** A source reference is Copy; its referent never obtains a Copy/Clone proof.
 * @param {boolean} selected @param {string} kind @param {string} owner
 * @param {string} path @param {number} argumentsCount @param {number} wrappers
 * @param {boolean} child @returns {boolean} */
export function acceptRustBorrowedVectorShape(selected,kind,owner,path,argumentsCount,wrappers,child){return selected&&kind==='array'&&owner==='library'&&path==='std::vec::Vec'&&argumentsCount===1&&wrappers===0&&child;}
/** Map a returned reference lifetime to actual reference parameters. The host
 * transports their actual value loans; this plan contains only coordinates.
 * @param {string} lifetime @param {boolean[]} references @param {string[]} lifetimes
 * @returns {number[]} */
export function planRustBorrowedVectorResultParameters(lifetime,references,lifetimes){
 if(references.length!==lifetimes.length||references.length>65536)return [-1];
 const result=[];for(let index=0;index<references.length;index+=1)if(references[index]&&(lifetime!==''&&lifetime!=='_'?lifetimes[index]===lifetime:true))result.push(index);
 if(lifetime==='static')return result;
 if(lifetime===''||lifetime==='_')return result.length===1?result:[-1];
 return result.length?result:[-1];
}
/** @param {boolean} same @param {boolean} reference @param {boolean} available
 * @param {boolean} collecting @returns {string} */
export function readRustBorrowedVectorProductionError(same,reference,available,collecting){return !same||!reference?'a borrowed vector element needs its actual matching immutable reference position':!collecting&&!available?'a borrowed vector element needs its actual live source region':'';}
/** @param {boolean} same @param {boolean} produced @param {boolean} allowed
 * @param {boolean} collecting @returns {string} */
export function readRustBorrowedVectorResultError(same,produced,allowed,collecting){return !same||!collecting&&(!produced||!allowed)?'a borrowed vector result needs an actual producer and parameter-derived or static element regions':'';}
/** @param {number} count @returns {string} */
export function readRustBorrowedVectorElementCountError(count){return !Number.isSafeInteger(count)||count<0||count>65536?'borrowed vector construction exceeds its finite actual element profile':'';}
/** @param {boolean} actual @param {boolean} expected @param {boolean} same
 * @param {boolean} owner @param {string} lifetime @param {string} required
 * @returns {string} */
export function readRustBorrowedVectorArgumentError(actual,expected,same,owner,lifetime,required){return actual!==expected||actual&&(!same||!owner||required==='static'&&lifetime!=='static')?'borrowed vector arguments must retain their actual reference element owner and region contract':'';}
/** @param {boolean} borrowed @param {string} operation @returns {string} */
export function readRustBorrowedVectorObservationError(borrowed,operation){return borrowed&&!['len','is_empty','transfer'].includes(operation)?'borrowed vector observations need their independent reference-preserving source operation profile':'';}
/** @param {boolean} selected @param {boolean} shape @param {boolean} proven
 * @param {boolean} producer @returns {boolean} */
export function acceptRustBorrowedVectorProducedValue(selected,shape,proven,producer){return selected&&shape&&proven&&producer;}
/** @param {boolean} reference @param {number} regions @param {boolean} collecting
 * @returns {string} */
export function readRustBorrowedVectorCallRegionError(reference,regions,collecting){if(!Number.isSafeInteger(regions)||regions<0||regions>65536)return 'borrowed vector calls exceed their finite actual argument region profile';return !collecting&&reference&&regions===0?'a borrowed vector call needs actual argument region provenance; anonymous temporary references need their separate promotion or extension proof':'';}
