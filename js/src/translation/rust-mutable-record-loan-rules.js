import {acceptRustOwnedRecordLoopCount} from './rust-owned-record-loop-rules.js';
import {testRustOwnedRecordFieldDescendant} from './rust-owned-record-update-rules.js';
/** Actual values carry reference loans; a detached Copy scalar is a value.
 * @param {boolean} reference @param {boolean} place @param {boolean} copy
 * @param {number} contained @returns {boolean} */
export function acceptRustMutableRecordValueLoans(reference,place,copy,contained){return reference||place&&!copy||contained>0;}
/** @param {string} root @param {string} lexical @param {string[]} loans
 * @returns {boolean} */
export function selectRustMutableRecordLoanRoot(root,lexical,loans){if(root==='')return false;for(const loan of loans)if(loan===root||lexical!==''&&loan===lexical||testRustOwnedRecordFieldDescendant(root,loan))return true;return false;}
/** @param {number} ordinal @returns {string} */
export function readRustMutableRecordLoanIdentity(ordinal){return acceptRustOwnedRecordLoopCount(ordinal)?'mutable record loan '+String(ordinal):'';}
/** @param {number} nodes @param {number} text @param {number} addedNodes
 * @param {number} addedText @returns {string} */
export function readRustMutableRecordLoanBudgetError(nodes,text,addedNodes,addedText){return !acceptRustOwnedRecordLoopCount(nodes)||!acceptRustOwnedRecordLoopCount(text)||!acceptRustOwnedRecordLoopCount(addedNodes)||!acceptRustOwnedRecordLoopCount(addedText)||nodes>100000-addedNodes||text>16777216-addedText?'the actual mutable record value loans exceed their source budget':'';}
/** Scope escape retains a real loan only while its root and every dependency
 * survive. The flags come from actual owner maps, never a type signature.
 * @param {boolean} carried @param {boolean} rootSurvives @param {boolean} live
 * @returns {boolean} */
export function acceptRustMutableRecordLoanEscape(carried,rootSurvives,live){return carried&&rootSurvives&&live;}
/** @param {boolean} owned @param {boolean} overlapping @returns {string} */
export function readRustMutableRecordLoanMoveError(owned,overlapping){return owned&&overlapping?'an owned move overlaps an actual surviving mutable record value loan':'';}
