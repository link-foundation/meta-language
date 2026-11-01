// Actual owned pattern shape transport. Scope construction remains in the
// source-origin table, which owns all root/loan identities and transitions.
import {unsupported} from './diagnostics.js';
import {isRustOwnedStandardSumValue} from './rust-standard-sum-ownership.js';
import {readRustStandardSumGuardMode,readRustStandardSumGuardAlternativePatternError} from './rust-standard-sum-guard-rules.js';
export function countRustStandardSumGuardAliases(patterns){
 const pending=[...patterns];let count=0;
 while(pending.length){const pattern=pending.pop();if(pattern.k==='alias')count+=1;pending.push(...(pattern.args??[]));if(pattern.pattern)pending.push(pattern.pattern);}
 return count;
}
export function sameRustStandardSumGuardLineage(left,right){
 const project=origin=>[origin.sourceBindingIdentity??null,origin.sourcePlaceBindingIdentity??null,origin.sourceLoanBindingIdentities??[],origin.sourcePlaceLoanBindingIdentities??[],origin.sourceLifetimes??[],origin.sourceBorrowedPlace===true,origin.sourceOwnedResultProof===true,origin.sourceStandardSumPatternMove===true,origin.sourceStandardSumBorrowedField===true,origin.sourceExtendingTemporaryBindings??[]];
 return JSON.stringify(project(left))===JSON.stringify(project(right));
}
export function deferRustStandardSumGuard(parser,patterns,scrutinee,path,guarded,aliases){
 const owned=isRustOwnedStandardSumValue(parser.origins,parser.origins.origin(scrutinee,path));
 if(readRustStandardSumGuardMode(owned,guarded)!=='deferred guard')return false;
 const error=readRustStandardSumGuardAlternativePatternError(patterns.length,aliases.length+countRustStandardSumGuardAliases(patterns));if(error)throw unsupported('Rust owned standard sum guard',error,scrutinee.span);
 return true;
}
