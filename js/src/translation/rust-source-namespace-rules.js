// Rust source namespace and lexical candidate decisions, shared by transports.
/** @param {string} kind @returns {string} */
export function readRustSourceNamespaceRole(kind){
 if(kind==='data'||kind==='module'||kind==='externalCrate')return 'type';
 return kind==='fn'||kind==='const'||kind==='ctor'?'value':'';
}
/** Empty/mixed filters retain the historical unqualified catalog role.
 * @param {string[]} kinds @returns {string} */
export function readRustSourceNamespaceLookupRole(kinds){
 let role='';for(let at=0;at<kinds.length;at+=1){const next=readRustSourceNamespaceRole(kinds[at]??'');if(next===''||role!==''&&role!==next)return 'any';role=next;}
 return role!==''?role:'any';
}
/** A nearer item in the requested namespace stops lookup even when a narrower
 * kind filter refuses it. An item in the other namespace does not shadow it.
 * @param {string} kind @param {string} role @param {string[]} kinds @returns {string} */
export function readRustSourceNamespaceCandidate(kind,role,kinds){
 const actual=readRustSourceNamespaceRole(kind);if(actual===''||role!=='any'&&actual!==role)return 'continue';
 return kinds.length===0||kinds.includes(kind)?'select':'stop';
}
/** Qualified constructor prefixes belong to the type namespace; a bare
 * constructor competes with values and functions of the same spelling.
 * @param {number} segmentCount @param {boolean} valuePosition @returns {string} */
export function readRustSourceNamespacePathRole(segmentCount,valuePosition){
 return valuePosition&&segmentCount===1?'value':'type';
}
/** @param {string} previous @param {string} next @returns {boolean} */
export function acceptRustSourceNamespaceDeclarationPair(previous,next){
 return readRustSourceNamespaceRole(previous)!=='type'||readRustSourceNamespaceRole(next)!=='type'||previous===next||previous!=='externalCrate'&&next!=='externalCrate';
}
/** A qualified path commits to its nearest actual type/module prefix even
 * when that item has no matching child; another outer item cannot supply it.
 * @param {number} segmentCount @param {string} prefixKind @returns {boolean} */
export function acceptRustSourceNamespaceOuterCandidate(segmentCount,prefixKind){
 return segmentCount<2||readRustSourceNamespaceRole(prefixKind)!=='type';
}
