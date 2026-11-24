// Additive immutable iterator interpretation. Scalar/owned iterator decisions
// and their historical assertions are intentionally separate entry points.
import {readPrimitiveMethodDefaultValue} from './primitive-method-rules.js';
/** @param {boolean} selected @param {string} method @param {number} count
 * @param {string} kind @param {string} owner @param {string} path
 * @param {number} argumentsCount @param {boolean} closed @returns {boolean} */
export function acceptRustBorrowedIteratorReceiver(selected,method,count,kind,owner,path,argumentsCount,closed){return selected&&method==='iter'&&count===0&&kind==='array'&&owner==='library'&&path==='std::vec::Vec'&&argumentsCount===1&&closed;}
/** @param {string} operation @returns {string} */
export function readRustBorrowedIteratorContinuation(operation){return operation==='map'?'map':operation==='chain'?'chain':operation==='collect'?'collect':'';}
/** @param {string} kind @returns {string} */
export function readRustBorrowedIteratorProjectionSyntax(kind){return kind==='name'?'name':kind==='match'?'match':'';}
/** @param {boolean} selected @param {number} parameters @param {boolean} immutable
 * @param {boolean} reference @param {boolean} closed @param {number} captures
 * @param {boolean} bodyProjection @returns {string} */
export function readRustBorrowedIteratorCallbackError(selected,parameters,immutable,reference,closed,captures,bodyProjection){return selected&&parameters===1&&immutable&&reference&&closed&&captures===0&&bodyProjection?'':'a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection';}
/** @param {string[]} loans @param {string[]} existing @param {string[]} local
 * @param {string[]} receiver @returns {string[]} */
export function projectRustBorrowedIteratorCallbackRegions(loans,existing,local,receiver){let result=[];for(const loan of loans){if(receiver.includes(loan)){if(!result.includes(loan))result.push(loan);}else if(local.includes(loan)&&!existing.includes(loan)){for(const root of receiver)if(!result.includes(root))result.push(root);}else return [];}return result;}
/** @param {number} actual @param {number} projected @param {boolean} collecting
 * @returns {string} */
export function readRustBorrowedIteratorRegionError(actual,projected,collecting){return !collecting&&(actual===0||projected===0)?'a readonly iterator value needs its actual live receiver-derived regions':'';}
/** @param {string} path @param {boolean} shadowed @param {number} argumentsCount
 * @returns {boolean} */
export function acceptRustBorrowedIteratorOnce(path,shadowed,argumentsCount){return path==='std::iter::once'&&!shadowed&&argumentsCount===1;}
/** @param {boolean} selected @param {boolean} canonical @param {number} argumentsCount
 * @param {boolean} reference @param {boolean} live @param {boolean} collecting
 * @returns {boolean} */
export function acceptRustBorrowedIteratorBoxReference(selected,canonical,argumentsCount,reference,live,collecting){return selected&&canonical&&argumentsCount===0&&reference&&(live||collecting);}
/** @param {number} stages @param {number} nodes @param {number} regions
 * @returns {string} */
export function readRustBorrowedIteratorBudgetError(stages,nodes,regions){for(const count of [stages,nodes,regions])if(!Number.isSafeInteger(count)||count<0||Object.is(count,-0))return 'invalid readonly iterator source resource count';return stages>64||nodes>100000||regions>65536?'the readonly iterator source pipeline exceeds its finite profile':'';}
/** @param {boolean} type @param {boolean} owner @param {boolean} reference
 * @param {boolean} context @returns {string} */
export function readRustBorrowedIteratorCollectionError(type,owner,reference,context){return type&&owner&&reference&&context?'':'readonly collection needs its actual matching borrowed vector result position';}
/** @param {string} operation @param {boolean} source @param {boolean} result
 * @param {boolean} projection @returns {boolean} */
export function acceptRustBorrowedIteratorProduction(operation,source,result,projection){return ['slice','map','concat'].includes(operation)&&source&&result&&projection;}
/** @param {string[]} kinds @param {string[]} types @param {string[]} values
 * @returns {boolean} */
export function acceptRustBorrowedIteratorWholeSlice(kinds,types,values){return kinds.length===2&&types.length===2&&values.length===2&&kinds[0]==='lit'&&kinds[1]==='lit'&&types[0]==='float'&&types[1]==='float'&&values[0]===readPrimitiveMethodDefaultValue('array','slice',0)&&values[1]===readPrimitiveMethodDefaultValue('array','slice',1);}
/** Generated binder/state strings are ASCII, so these units agree in both hosts.
 * @param {number} count @param {number} text @returns {string} */
export function readRustBorrowedIteratorSnapshotError(count,text){return !Number.isSafeInteger(count)||!Number.isSafeInteger(text)||count<0||text<0||Object.is(count,-0)||Object.is(text,-0)||count>100000||text>16777216?'readonly callback ownership snapshots exceed their finite source profile':'';}
/** Raw identifiers have already passed the common Rust lexical rule. Plain
 * reserved words are not fresh immutable callback binders in this profile.
 * @param {string} name @param {string} raw @returns {string} */
export function readRustBorrowedIteratorParameterError(name,raw){if(raw.startsWith('r#'))return '';return ['_','as','break','const','continue','crate','else','enum','extern','false','fn','for','if','impl','in','let','loop','match','mod','move','mut','pub','ref','return','self','Self','static','struct','super','trait','true','type','unsafe','use','where','while','async','await','dyn','abstract','become','box','do','final','gen','macro','override','priv','try','typeof','unsized','virtual','yield'].includes(name)?'the readonly callback needs a fresh direct immutable binding identifier':'';}
/** @param {string} valueKind @param {boolean} declaredType @returns {string} */
export function readRustBorrowedIteratorBindingOwnerError(valueKind,declaredType){return valueKind==='const'||valueKind==='ctor'||declaredType?'readonly callback parameters require a fresh binder rather than a constructor or constant pattern':'';}
/** Separate small work bound before existing array-based loan union helpers.
 * @param {number} count @returns {string} */
export function readRustBorrowedIteratorRegionWorkError(count){return !Number.isSafeInteger(count)||count<0||Object.is(count,-0)||count>256?'readonly iterator region union exceeds its bounded source work profile':'';}
/** A real Box candidate cannot fall through to a different inner method.
 * @param {boolean} canonical @param {number} count @param {boolean} admitted
 * @param {boolean} projection @returns {string} */
export function readRustBorrowedIteratorBoxReferenceError(canonical,count,admitted,projection){return !canonical?'':count!==0?'canonical Box::as_ref expects zero arguments':!admitted||!projection?'canonical Box::as_ref requires its actual live readonly receiver projection':'';}
