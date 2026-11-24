// Actual standard owned String addition into the common concatenation value.
/** @param {string} operation @param {boolean} ordinary @param {string} left @param {string} right @returns {boolean} */
export function acceptRustStringAdditionCandidate(operation,ordinary,left,right){return operation==='add'&&ordinary&&(left==='string'||right==='string');}
/** @param {string} side @param {string} kind @param {string} path @param {number} children @returns {boolean} */
export function acceptRustStringAdditionOwner(side,kind,path,children){return children===0&&(kind==='library'&&path==='std::string::String'||side==='right'&&kind==='primitive'&&path==='str');}
/** @param {string} side @param {string} kind @param {string} model @param {number} layers @param {boolean} proven @param {boolean} references @param {boolean} known @returns {boolean} */
export function acceptRustStringAdditionOperand(side,kind,model,layers,proven,references,known){return kind==='string'&&proven&&references&&known&&(side==='left'&&model==='owned string'&&layers===0||side==='right'&&(model==='owned string'||model==='string slice')&&layers===1);}
