// Bytecode transports to distinct weak slots through the shared collection
// instruction. Actual protected callable identities remain realm supplied.
import {readJavaScriptWeakCollectionOperationKind} from './javascript-weak-collection-rules.js';
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptWeakCollectionFunction(identity,realm,operation){
 if(readJavaScriptWeakCollectionOperationKind(operation)==='')return undefined;
 return {$:'callFunction',identity,realm,receiverMode:'collection intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'collectionOperation',destination:1,operation},{$:'return',source:1}]};
}
