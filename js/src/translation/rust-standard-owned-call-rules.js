// Concrete standard-library ownership transfers, not arbitrary trait dispatch.
/** @param {string} path @param {boolean} ordinary @param {boolean} shadowed @returns {string} */
export function readRustStandardOwnedCallRole(path,ordinary,shadowed){
 if(!ordinary||shadowed)return '';
 if(path==='std::clone::Clone::clone')return 'clone';
 return path==='std::borrow::ToOwned::to_owned'?'to_owned':'';
}
/** @param {string} role @param {string} kind @param {string} model @param {number} layers @param {boolean} proven @param {boolean} references @param {boolean} known @returns {boolean} */
export function acceptRustStandardOwnedTextCall(role,kind,model,layers,proven,references,known){return kind==='string'&&layers===1&&proven&&references&&known&&(role==='clone'&&model==='owned string'||role==='to_owned'&&(model==='owned string'||model==='string slice'));}
/** Direct scalar element validation remains with the shared vector-copy rule.
 * @param {string} role @param {string} model @param {number} layers @param {boolean} known @returns {boolean} */
export function acceptRustStandardOwnedArrayCall(role,model,layers,known){return layers===1&&known&&(model==='owned vector'&&(role==='clone'||role==='to_owned')||model==='immutable slice'&&role==='to_owned');}
/** @param {string} role @param {string} kind @param {number} layers @param {boolean} proven @param {boolean} references @param {boolean} known @returns {boolean} */
export function acceptRustStandardOwnedScalarCall(role,kind,layers,proven,references,known){return (role==='clone'||role==='to_owned')&&(kind==='fixed'||kind==='float'||kind==='bool'||kind==='unit')&&layers===1&&proven&&references&&known;}
