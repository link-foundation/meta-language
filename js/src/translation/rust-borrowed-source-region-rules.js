// Source-free immutable binder-flow decisions. Hosts supply actual checked
// expressions, declarations and binder links; no runtime identities are inputs.
import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
import {readRustOwnedBindingReadError,consumeRustOwnedBindingState,expireRustOwnedBindingState,mergeRustOwnedBindingStates} from './rust-owned-binding-rules.js';
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} argumentsCount
 * @param {string[]} wrappers @param {boolean} borrowed @returns {string} */
export function readRustBorrowedSourceValueRole(kind,bits,signed,owner,path,argumentsCount,wrappers,borrowed){
 if(wrappers[0]==='reference')return 'reference';
 if(wrappers[0]==='box')return 'box';
 if(kind==='array'&&owner==='library'&&path==='std::vec::Vec'&&argumentsCount===1)return borrowed?'borrowed vector':'owned';
 if(kind==='data'&&(owner==='declaration'||owner==='library'))return 'owned';
 if(kind==='string'&&owner==='library'&&['std::string::String','alloc::string::String'].includes(path)&&argumentsCount===0)return 'string';
 if(kind==='string'&&owner==='primitive'&&path==='str')return 'text';
 if(kind==='unit'&&owner==='unit'&&path===''&&argumentsCount===0)return 'copy';
 if(owner==='primitive'&&argumentsCount===0&&acceptRustSourceSignaturePrimitive(kind,path,bits,signed))return 'copy';
 return '';
}
/** @param {string} role @param {string} mode @returns {string} */
export function readRustBorrowedSourceUsePlan(role,mode){if(!['copy','inferred numeric','reference','owned','string','box','borrowed vector'].includes(role))return 'unsupported';return mode==='move'&&['owned','string','box','borrowed vector'].includes(role)?'consume':'read';}
/** @param {string} state @param {string[]} loans @returns {string} */
export function readRustBorrowedSourceReadError(state,loans){if(state!=='') {const error=readRustOwnedBindingReadError(state);if(error)return error;}for(const loan of loans){const error=readRustOwnedBindingReadError(loan);if(error)return 'a source expression uses a region whose actual binder has moved or expired';}return '';}
/** @param {string} state @returns {string} */
export function consumeRustBorrowedSourceRegion(state){return consumeRustOwnedBindingState(state);}
/** @param {string} state @returns {string} */
export function expireRustBorrowedSourceRegion(state){return expireRustOwnedBindingState(state);}
/** @param {string} source @param {string} target @param {boolean} same
 * @param {boolean} inner @param {boolean} sourceInner @param {boolean} place
 * @param {boolean} literal @param {boolean} text @param {number} sourceReferences
 * @param {number} targetReferences @returns {string} */
export function readRustBorrowedSourceAdjustment(source,target,same,inner,sourceInner,place,literal,text,sourceReferences,targetReferences){
 if(same&&source===target&&sourceReferences===targetReferences)return 'preserve';
 if(source==='inferred numeric'&&target==='copy')return 'constrain primitive';
 if(target==='reference'&&(same||inner)){if(source==='reference'&&targetReferences<=sourceReferences)return 'project reference';if(place)return 'borrow';if(literal)return 'static literal';return 'temporary borrow';}
 if(source==='reference'&&target==='copy'&&same)return 'copy referent';
 if(target==='box'&&inner)return 'construct box';
 if(source==='box'&&sourceInner)return target==='copy'?'copy box payload':'move box payload';
 if(source==='reference'&&target==='string'&&text)return 'materialize text';
 return 'unsupported';
}
/** @param {string} role @param {boolean} same @param {boolean} staticDemand
 * @param {boolean} staticProof @returns {string} */
export function readRustBorrowedSourcePositionFlowError(role,same,staticDemand,staticProof){return !same?'source expression qualifiers or owners disagree with their actual value flow':staticDemand&&!staticProof?'a static source lifetime cannot be supplied by a local binder region':!['copy','reference','owned','string','box','borrowed vector'].includes(role)?'the source expression needs a separate value-domain proof':'';}
/** @param {number[]} regions @param {number[]} allowed @param {number[]} staticRegions
 * @param {boolean} interfaceKnown @returns {string} */
export function readRustBorrowedSourceReturnError(regions,allowed,staticRegions,interfaceKnown){if(!interfaceKnown)return 'a borrowed source result needs its actual source parameter lifetime interface';for(const region of regions)if(!allowed.includes(region)&&!staticRegions.includes(region))return 'a returned source region is not derived from its actual enclosing parameter interface';return '';}
/** @param {string} demand @param {string[]} sources @returns {string} */
export function readRustBorrowedSourceLifetimeError(demand,sources){if(demand===''||demand==='_')return '';for(const source of sources)if(source!=='static'&&source!==demand)return 'an expression lifetime is not supplied by its actual source binder regions';return '';}
/** @param {string} kind @param {string[]} wrappers @param {boolean} child
 * @returns {string} */
export function readRustBorrowedSourceTargetFieldError(kind,wrappers,child){return wrappers.includes('reference')||child||['function','tuple','record','map','set','cell'].includes(kind)?'a borrowed referent needs a closed target-copy field graph with represented source qualifiers':'';}
/** @param {string} kind @returns {string} */
export function readRustBorrowedSourceOperation(kind){if(kind==='toString')return 'show';if(['lit','unit','var','let','array','call','ctor','index','length','cast','unary','binary','math','show','concat'].includes(kind))return kind;return '';}
/** @param {number} visits @param {number} depth @param {number} roots
 * @param {number} loans @returns {string} */
export function readRustBorrowedSourceRegionBudgetError(visits,depth,roots,loans){for(const value of [visits,depth,roots,loans])if(!Number.isSafeInteger(value)||value<0||Object.is(value,-0))return 'invalid source region proof resource count';return visits>100000||depth>256||roots>100000||loans>1048576?'the immutable source binder and region proof exceeds its finite resource profile':'';}
/** @param {number} nodes @param {number} units @param {number} addNodes
 * @param {number} addUnits @returns {number[]} */
export function reserveRustBorrowedSourcePosition(nodes,units,addNodes,addUnits){for(const value of [nodes,units,addNodes,addUnits])if(!Number.isSafeInteger(value)||value<0||Object.is(value,-0))return [];const nextNodes=nodes+4*addNodes,nextUnits=units+4*addUnits;return nextNodes>1048576||nextUnits>8388608?[]:[nextNodes,nextUnits];}
/** @param {boolean} selected @param {boolean} runtimeShell @returns {string} */
export function readRustBorrowedSourceRuntimeError(selected,runtimeShell){return selected&&runtimeShell?'borrowed source body positions require their independent runtime-shell binder adapter':'';}
/** @param {string} role @param {string} operation @returns {string} */
export function readRustBorrowedSourceObservationError(role,operation){return role==='source state packet'?'source state packets are interpreted through their actual result/state slots':['reference','box','owned','borrowed vector'].includes(role)&&operation!=='len'?'this source observation needs an independently represented value or source trait proof':'';}
/** @param {string} role @param {string} kind @param {string} owner
 * @param {string} path @param {number} argumentCount @param {string[]} wrappers
 * @returns {string} */
export function readRustBorrowedSourceObservationRole(role,kind,owner,path,argumentCount,wrappers){return role==='reference'&&kind==='string'&&owner==='primitive'&&path==='str'&&argumentCount===0&&wrappers.length===1&&wrappers[0]==='reference'?'text':role;}
/** @param {string} role @param {string} kind @param {string} pattern
 * @param {number} fields @param {number} bindings @returns {string} */
export function readRustBorrowedSourceMatchError(role,kind,pattern,fields,bindings){if(role!=='reference'||kind!=='data')return 'source matching requires its actual borrowed nominal place adapter';if(!['wild','bind','ctor'].includes(pattern))return 'this source pattern needs its independent binding projection';if(pattern==='ctor'&&(!Number.isSafeInteger(fields)||fields<0||fields!==bindings))return 'source constructor patterns need every actual declared field position';return '';}
/** @param {boolean} composed @param {string} kind @param {string} pattern
 * @param {number} fields @param {number} bindings @returns {string} */
export function readRustBorrowedSourceOwnedMatchError(composed,kind,pattern,fields,bindings){return !composed?'owned matches require the explicit closed-call region composition':readRustBorrowedSourceMatchError('reference',kind,pattern,fields,bindings);}
/** @param {number} cells @param {number} addition @returns {number} */
export function reserveRustBorrowedSourceBranches(cells,addition){return !Number.isSafeInteger(cells)||!Number.isSafeInteger(addition)||cells<0||addition<0||Object.is(cells,-0)||Object.is(addition,-0)||addition>1048576-cells?-1:cells+addition;}
/** @param {string} left @param {string} right @returns {string} */
export function mergeRustBorrowedSourceRegion(left,right){return mergeRustOwnedBindingStates(left,right,true,true);}
/** @param {string} kind @returns {string} */
export function readRustBorrowedSourceMatchOperation(kind){return kind==='match'?'match':'';}
/** Standard Box dereference of a shared reference preserves the actual loan.
 * @param {string[]} source @param {string[]} target @param {boolean} canonical
 * @param {boolean} sameLeaf @returns {boolean} */
export function acceptRustBorrowedSourceBoxReference(source,target,canonical,sameLeaf){if(!canonical||!sameLeaf||source[0]!=='reference'||target[0]!=='reference'||source.length<=target.length)return false;for(let index=1;index<source.length;index+=1)if(source[index]!=='box')return false;for(let index=1;index<target.length;index+=1)if(target[index]!=='box')return false;return true;}
/** @param {string} kind @param {string} operation @returns {string} */
export function readRustBorrowedSourceIteratorOperation(kind,operation){if(kind==='append')return 'iterator concat';return kind==='method'&&['map','slice'].includes(operation)?'iterator '+operation:'';}

/** Whole-root replacement creates a new local epoch; it never revives a dropped scope.
 * @param {string} state @returns {string} */
export function readRustBorrowedSourceStorageWriteError(state){return ['available','moved','possibly moved'].includes(state)?'':'a source whole-root write requires its initialized lexical storage scope';}
