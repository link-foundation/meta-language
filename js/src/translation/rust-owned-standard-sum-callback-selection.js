// Read-only source context. No cursor advance, type parse, alias expansion,
// rollback, specialization allocation, or ambient result context is used.
import {RUST_SOURCE_SIGNATURE_POLICY} from './rust-source-signatures.js';
import {readRustSourceValueName} from './rust-source-constants.js';
import {readRustTypeAliasLookupKeys} from './rust-type-alias-rules.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {readRustOwnedStandardSumCallbackSelectionTokenLimit,planRustOwnedStandardSumCallbackSelection,acceptRustOwnedStandardSumCallbackWrittenSelection} from './rust-owned-standard-sum-callback-rules.js';
const owned=(parser,origin)=>readRustStandardSumOwnershipFacts(parser.origins,origin).profile==='owned'&&Boolean(readRustStandardSumSourceIdentity(parser,origin));
export function selectsRustOwnedStandardSumCallback(parser,path){
 const tokens=parser.cursor.tokens.slice(parser.cursor.index,parser.cursor.index+readRustOwnedStandardSumCallbackSelectionTokenLimit()),plan=planRustOwnedStandardSumCallbackSelection(tokens.map(token=>token.value),tokens.map(token=>token.kind));if(plan.length!==3)return false;
 const names=[];for(let at=plan[1];at<plan[2];at+=2)names.push(tokens[at].value);
 if(plan[0]===3){
  const resolved=parser.resolve(names,path),declared=parser.origins.lookupTypeOwner(names,path)??parser.origins.lookupTypeOwner(resolved,path),library=RUST_SOURCE_SIGNATURE_POLICY.libraries[names.join('::')]??Object.values(RUST_SOURCE_SIGNATURE_POLICY.libraries).find(owner=>owner.join('::')===resolved.join('::'));
  const alias=[names,resolved,[names[0]],[resolved[0]]].some(head=>readRustTypeAliasLookupKeys(head,path).some(key=>parser.sourceTypeAliases?.has(key)));
  const shadow=names.length>1&&Boolean(parser.origins.lookupTypeOwner([names[0]],path))||resolved.length>1&&Boolean(parser.origins.lookupTypeOwner([resolved[0]],path));
  return acceptRustOwnedStandardSumCallbackWrittenSelection(declared?'declaration':library?'library':'',declared?declared.fullName.split('.').join('::'):library?.join('::')??'',alias,shadow);
 }
 if(names.length===1&&parser.origins.scope.has(names[0])){
  const callable=parser.origins.scope.get(names[0]);return plan[0]===1&&parser.origins.immutableProduct(callable)&&callable.wrappers.length===0&&callable.type?.kind==='function'&&owned(parser,callable.callableResult);
 }
 const entry=parser.origins.lookupValueOwner(readRustSourceValueName(parser,names,path),path);
 if(plan[0]===2){const result=entry?.k==='fn'?entry.result:null;return Boolean(result?.type?.kind==='function'&&parser.origins.immutableProduct(result)&&result.wrappers.length===0&&owned(parser,result.callableResult));}
 if(entry?.k==='fn')return owned(parser,entry.result);
 const callable=entry?.k==='const'?entry.result:null;return Boolean(callable?.type?.kind==='function'&&parser.origins.immutableProduct(callable)&&callable.wrappers.length===0&&owned(parser,callable.callableResult));
}
