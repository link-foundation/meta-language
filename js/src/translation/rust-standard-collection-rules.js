// Closed standard HashMap/HashSet decisions. Parser facts must come from actual
// resolved imports, concrete type origins and lexical method/loan witnesses.
/** @typedef {{kind:string,path:string[],arguments:RustStandardCollectionTypeOwner[]}} RustStandardCollectionTypeOwner */
/** @typedef {{proven:boolean,wrappers:string[],sourceOwner:RustStandardCollectionTypeOwner,textSourceModel?:string}} RustStandardCollectionScalarOrigin */
/** @typedef {{$:'rustStandardCollectionOrigin',ordinary:boolean,proven:boolean,shadowed:boolean,typeIdentity:string,resolvedOwner:string,sourceOwner:RustStandardCollectionTypeOwner,arguments:RustStandardCollectionScalarOrigin[],sourceVersionMinor:number,pointerWidth:number,customHasher:boolean,customEquality:boolean}} RustStandardCollectionOrigin */
/** @typedef {{domain:string,proven:boolean,available:boolean,borrowed:boolean,ownedTransfer:boolean,sourceOwner:RustStandardCollectionTypeOwner}} RustStandardCollectionArgumentOrigin */
/** @typedef {{$:'rustStandardCollectionCallOrigin',owner:string,typeIdentity:string,receiverRoot:string,method:string,dispatchOwner:string,proven:boolean,shadowed:boolean,mutable:boolean,available:boolean,overlappingLoan:boolean,arguments:RustStandardCollectionArgumentOrigin[],outputRoot?:string}} RustStandardCollectionCallOrigin */

/** core has no HashMap/HashSet declarations. Trait witnesses remain separate.
 * @param {string} path @param {boolean} ordinary @param {boolean} shadowed @returns {string} */
export function resolveRustStandardCollectionOwner(path,ordinary,shadowed){
 if(!ordinary||shadowed)return '';
 if(path==='std::collections::HashMap'||path==='std::collections::hash_map::HashMap')return 'std::collections::HashMap';
 return path==='std::collections::HashSet'||path==='std::collections::hash_set::HashSet'?'std::collections::HashSet':'';
}
/** @param {string} owner @returns {string} */
export function readRustStandardCollectionKind(owner){return owner==='std::collections::HashMap'?'map':owner==='std::collections::HashSet'?'set':'';}
/** @param {number} minor @returns {string} */
export function readRustStandardCollectionSourceVersionError(minor){return Number.isInteger(minor)&&minor>=90&&minor<100?'':'standard collection source semantics are uncovered outside Rust >=1.90.0 <1.100.0';}
/** @param {string} domain @param {number} pointerWidth @returns {string} */
export function readRustStandardCollectionDomainError(domain,pointerWidth){
 if(pointerWidth!==32&&pointerWidth!==64)return 'collection integer domains require explicit supported target pointer width';
 return ['String','bool','i8','i16','i32','i64','i128','isize','u8','u16','u32','u64','u128','usize'].includes(domain)?'':'collections require closed String Boolean or integer domains; custom Hash/Eq and float domains need separate semantics';
}
/** @param {string} domain @param {number} pointerWidth @returns {number} */
export function readRustStandardCollectionIntegerWidth(domain,pointerWidth){
 if(readRustStandardCollectionDomainError(domain,pointerWidth)!==''||domain==='String'||domain==='bool')return 0;
 return domain==='isize'||domain==='usize'?pointerWidth:Number(domain.slice(1));
}
/** @param {RustStandardCollectionScalarOrigin} origin @param {string} domain @returns {boolean} */
export function acceptRustStandardCollectionScalarOrigin(origin,domain){
 if(!origin?.proven||origin.wrappers.length!==0||origin.sourceOwner?.arguments.length!==0)return false;
 const owner=origin.sourceOwner,path=owner.path.join('::');
 if(domain==='String')return owner.kind==='library'&&['std::string::String','alloc::string::String'].includes(path)&&origin.textSourceModel==='owned string';
 return owner.kind==='primitive'&&[domain,'std::primitive::'+domain,'core::primitive::'+domain].includes(path);
}
/** @param {RustStandardCollectionOrigin} origin @param {string} keyDomain @param {string} valueDomain @param {number} pointerWidth @returns {string} */
export function readRustStandardCollectionSourceOriginError(origin,keyDomain,valueDomain,pointerWidth){
 if(origin?.$!=='rustStandardCollectionOrigin'||!origin.ordinary||!origin.proven||origin.shadowed||typeof origin.typeIdentity!=='string'||origin.typeIdentity.length===0)return 'collections require actual proven concrete source type and unshadowed import-owner witnesses';
 const owner=resolveRustStandardCollectionOwner(origin.resolvedOwner,origin.ordinary,origin.shadowed),kind=readRustStandardCollectionKind(owner),source=origin.sourceOwner;
 if(kind===''||source?.kind!=='library'||resolveRustStandardCollectionOwner(source.path.join('::'),true,false)!==owner)return 'collection source TYPE owner must match the actual resolved standard declaration';
 const count=kind==='map'?2:1;
 if(origin.arguments.length!==count||source.arguments.length!==count||origin.customHasher!==false||origin.customEquality!==false)return 'collections retain complete actual type arguments and default hasher with primitive Eq/Hash';
 const version=readRustStandardCollectionSourceVersionError(origin.sourceVersionMinor);if(version!=='')return version;
 if(origin.pointerWidth!==pointerWidth)return 'collection source and active target retain the same pointer width';
 for(const domain of kind==='map'?[keyDomain,valueDomain]:[keyDomain]){const error=readRustStandardCollectionDomainError(domain,pointerWidth);if(error!=='')return error;}
 if(kind==='set'&&valueDomain!=='unit')return 'HashSet retains only its actual key domain';
 const domains=kind==='map'?[keyDomain,valueDomain]:[keyDomain];
 for(let index=0;index<count;index++){
  const child=origin.arguments[index],declared=source.arguments[index];
  if(!acceptRustStandardCollectionScalarOrigin(child,domains[index])||declared.kind!==child.sourceOwner.kind||declared.path.join('::')!==child.sourceOwner.path.join('::')||declared.arguments.length!==0)return 'collection domain arguments require matching complete actual scalar TYPE owners';
 }
 return '';
}
/** @param {string} owner @param {string} method @returns {number} */
export function readRustStandardCollectionMethodArity(owner,method){
 const kind=readRustStandardCollectionKind(owner);if(kind==='')return -1;
 if(['new','default','len','is_empty','clear','clone'].includes(method))return 0;
 if(method==='insert')return kind==='map'?2:1;
 if(method==='get'||method==='remove'||method===(kind==='map'?'contains_key':'contains'))return 1;
 return -1;
}
/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardCollectionMethodResult(owner,method){
 if(readRustStandardCollectionMethodArity(owner,method)<0)return '';
 if(['new','default','clone'].includes(method))return 'owned collection';
 if(method==='len')return 'usize';if(method==='clear')return 'unit';if(method==='get')return 'borrowed Option';
 return owner==='std::collections::HashMap'&&(method==='insert'||method==='remove')?'owned Option':'bool';
}
/** @param {string} owner @param {string} method @returns {boolean} */
export function testRustStandardCollectionMutation(owner,method){return readRustStandardCollectionMethodArity(owner,method)>=0&&['insert','remove','clear'].includes(method);}
/** @param {RustStandardCollectionOrigin} origin @param {string} root @param {string} method @param {number} argumentsCount @param {RustStandardCollectionCallOrigin} call @returns {string} */
export function readRustStandardCollectionCallOriginError(origin,root,method,argumentsCount,call){
 const owner=resolveRustStandardCollectionOwner(origin.resolvedOwner,origin.ordinary,origin.shadowed),arity=readRustStandardCollectionMethodArity(owner,method);
 if(arity<0)return 'collection iteration entries custom Hash/Eq and unregistered methods require their actual separate semantics';
 if(call?.$!=='rustStandardCollectionCallOrigin'||!call.proven||call.shadowed||!['new','default'].includes(method)&&!call.available||call.owner!==owner||call.typeIdentity!==origin.typeIdentity||call.receiverRoot!==root||call.method!==method||arity!==argumentsCount||call.arguments.length!==argumentsCount)return 'collection calls retain actual declaration type receiver binding availability and complete argument witnesses';
 const dispatch=call.dispatchOwner;
 if(method==='clone'){if(!['std::clone::Clone','core::clone::Clone'].includes(dispatch))return 'collection clone requires its actual standard Clone trait dispatch';if(typeof call.outputRoot!=='string'||call.outputRoot.length===0||call.outputRoot===root)return 'collection clone requires its independent fresh output binding root';}
 else if(method==='default'){if(!['std::default::Default','core::default::Default'].includes(dispatch))return 'collection default requires its actual standard Default trait dispatch';}
 else if(resolveRustStandardCollectionOwner(dispatch,true,false)!==owner)return 'collection inherent calls require their actual resolved implementation owner';
 if(testRustStandardCollectionMutation(owner,method)&&(!call.mutable||call.overlappingLoan))return 'collection mutation requires an actual mutable available root without overlapping live loans';
 return '';
}
/** @param {RustStandardCollectionArgumentOrigin} argument @param {string} expected @param {boolean} borrowed @returns {string} */
export function readRustStandardCollectionArgumentOriginError(argument,expected,borrowed){
 if(!argument?.proven||!argument.available||argument.borrowed!==borrowed)return 'collection arguments retain their actual available typed value or borrow';
 const domain=argument.domain,owner=argument.sourceOwner,path=owner?.path.join('::');
 if(owner?.arguments.length!==0||domain!==expected&&!(borrowed&&expected==='String'&&domain==='str'))return 'collection arguments retain exact closed domain identities without coercion';
 if(domain==='String'){if(owner.kind!=='library'||!['std::string::String','alloc::string::String'].includes(path))return 'String collection arguments retain their actual owned standard String type';if(!borrowed&&!argument.ownedTransfer)return 'String insertion requires its actual owned argument transfer';}
 else if(owner.kind!=='primitive'||![domain,'std::primitive::'+domain,'core::primitive::'+domain].includes(path))return 'collection scalar arguments retain their actual primitive source owners';
 return '';
}
/** Result occupies position zero; state roots use the existing ordinary tuple
 * packet recipe, including the same 256-root bound.
 * @param {number} roots @returns {number[]} */
export function planRustStandardCollectionStatePacket(roots){if(!Number.isInteger(roots)||roots<0||roots>256)return [];const positions=[0];for(let index=0;index<roots;index++)positions.push(index+1);return positions;}
