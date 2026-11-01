// Actual owned pattern shape transport. Scope construction remains in the
// source-origin table, which owns all root/loan identities and transitions.
import {unsupported} from './diagnostics.js';
import {isRustOwnedStandardSumValue} from './rust-standard-sum-ownership.js';
import {readRustStandardSumGuardMode,readRustStandardSumDeferredGuardPatternError} from './rust-standard-sum-guard-rules.js';
export function deferRustStandardSumGuard(parser,patterns,scrutinee,path,guarded,aliases){
 const owned=isRustOwnedStandardSumValue(parser.origins,parser.origins.origin(scrutinee,path));
 if(readRustStandardSumGuardMode(owned,guarded)!=='deferred guard')return false;
 const pending=[...patterns];let count=aliases.length;
 while(pending.length){const pattern=pending.pop();if(pattern.k==='alias')count+=1;pending.push(...(pattern.args??[]));if(pattern.pattern)pending.push(pattern.pattern);}
 const error=readRustStandardSumDeferredGuardPatternError(patterns.length,count);if(error)throw unsupported('Rust owned standard sum guard',error,scrutinee.span);
 return true;
}
