/** Closed owned Vec mutation only; element destructors, stored references and
 * pointer/capacity observation are separately excluded by real source storage.
 * @param {string} method @returns {number} */
export function readRustExclusiveSourceVectorMethodArity(method){return method==='pop'||method==='clear'?0:method==='truncate'?1:-1;}
/** @param {string} kind @param {string} model @param {number} layers
 * @param {boolean} proven @param {string} owner @param {string} path
 * @param {number} arguments_ @returns {boolean} */
export function acceptRustExclusiveSourceVectorReceiver(kind,model,layers,proven,owner,path,arguments_){return kind==='array'&&model==='owned vector'&&layers===0&&proven&&owner==='library'&&path==='std::vec::Vec'&&arguments_===1;}
