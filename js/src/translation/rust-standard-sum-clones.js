// Ordinary immutable value IR already clones non-Copy Rust variable uses.
// Preserve one receiver evaluation and independent source ownership identities.
import {unsupported} from './diagnostics.js';
import {checkRustOwnedOrigin,trackRustOwnedBindings} from './rust-owned-bindings.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {createRustOwnedStandardSumMethodResult} from './rust-owned-standard-sum-methods.js';
import {readRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {readRustStandardSumCloneSourceError} from './rust-owned-standard-sum-method-rules.js';
import {carryRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {readRustSourceCloneImplementation} from './rust-nominal-ownership.js';
import {readRustNominalCloneImplementationError} from './rust-nominal-ownership-rules.js';
export function readRustStandardSumClone(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext()||method.value!=='clone')return null;
 const table=parser.origins,actual=table.origin(receiver,path),source=readRustStandardSumSourceOrigin(parser,actual);if(!source)return null;
 checkRustOwnedOrigin(table,actual,receiver.span);
 const role=readRustStandardSumOwnershipFacts(table,source).profile,error=readRustStandardSumCloneSourceError(source.standardSumSource.owner,role,actual.wrappers.length,actual.sourceBorrowedPlace===true,trackRustOwnedBindings(table),table.availableOwnedBinding(actual),actual.sourceOwnedResultProof===true);
 if(error)throw unsupported('Rust standard sum Clone',error,receiver.span);
 const implementationError=readRustNominalCloneImplementationError(readRustSourceCloneImplementation(table,source));if(implementationError)throw unsupported('Rust standard sum Clone',implementationError,receiver.span);
 const c=parser.cursor;c.expect('(','standard sum Clone');const close=c.expect(')','standard sum Clone'),span={start:receiver.span?.start??method.start,end:close.end},binding=parser.freshProductBinding();
 const local=table.mark({k:'name',path:[binding],span},actual),body={k:'let',name:binding,type:source.type,value:receiver,body:local,span};
 // Reading/structurally cloning this admitted value is an actual production;
 // self remains available. Type-only and unavailable sources were rejected.
 return table.mark(body,carryRustCopyClosureValueBorrows(createRustOwnedStandardSumMethodResult(parser,source,true,true),actual));
}
