// Public aliases are target syntax, never new nominal source identities.
/** @param {number} rows @param {number} units @returns {string} */
export function readFormalDatatypeAliasBudgetError(rows,units){return !Number.isSafeInteger(rows)||!Number.isSafeInteger(units)||rows<0||units<0||rows>8192||units>16777216?'formal datatype public aliases exceed their finite target budget':'';}
/** @param {number} visits @param {number} depth @returns {string} */
export function readFormalDatatypeAliasTraversalError(visits,depth){return !Number.isSafeInteger(visits)||!Number.isSafeInteger(depth)||visits<0||depth<0||visits>1048576||depth>256?'formal datatype alias scope traversal exceeds its finite target budget':'';}
/** @param {string} target @param {string} name @param {string} value @returns {string} */
export function renderFormalDatatypeTypeAlias(target,name,value){return target==='Lean'?'abbrev '+name+' := _root_.'+value:target==='Rocq'?'Definition '+name+' : Type := '+value+'.':'';}
/** @param {string} target @param {string} data @param {string} name
 * @param {string} value @returns {string} */
export function renderFormalDatatypeConstructorAlias(target,data,name,value){return target==='Lean'?'namespace '+data+'\n@[match_pattern] abbrev '+name+' := _root_.'+value+'\nend '+data:target==='Rocq'?'Notation '+name+' := '+value+'.':'';}
/** @param {string} target @param {string} name @param {boolean} open
 * @returns {string} */
export function renderFormalDatatypeAliasModule(target,name,open){return target==='Lean'?(open?'namespace ':'end ')+name:target==='Rocq'?(open?'Module ':'End ')+name+'.':'';}
