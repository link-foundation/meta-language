// Source selection for actual standard primitive vector copies. The existing
// full-slice operation evaluates the receiver once and retains an owned copy.
/** @param {boolean} ordinary @param {string} operation @param {string} model @param {number} layers @returns {boolean} */
export function acceptRustScalarVectorCopyQuery(ordinary,operation,model,layers){
 if(!ordinary||!Number.isInteger(layers)||layers<0||layers>256)return false;
 if(operation==='to_vec')return model==='owned vector'||model==='immutable slice'&&layers>0;
 return (operation==='clone'||operation==='to_owned')&&model==='owned vector'&&layers===0;
}
/** @param {string} kind @param {string} element @param {boolean} proven @param {boolean} references @param {boolean} copy @param {number} argumentsCount @returns {boolean} */
export function acceptRustScalarVectorCopyReceiver(kind,element,proven,references,copy,argumentsCount){
 return kind==='array'&&proven&&references&&copy&&argumentsCount===0&&(element==='fixed'||element==='float'||element==='bool'||element==='unit');
}
/** @param {string} kind @param {string} path @param {number} paths @param {number} children @returns {boolean} */
export function acceptRustScalarVectorCopyOwner(kind,path,paths,children){
 return children===1&&((kind==='slice'&&paths===0)||(kind==='library'&&path==='std::vec::Vec'&&paths===3));
}
/** @returns {string} */
export function readRustScalarVectorCopyOperation(){return 'slice';}
/** @param {string} kind @param {string} model @param {number} wrappers @param {boolean} recorded @returns {boolean} */
export function acceptRustScalarVectorCopyOccurrence(kind,model,wrappers,recorded){
 return recorded&&kind==='method'&&model==='owned vector'&&wrappers===0;
}
/** @param {string} language @param {string} kind @param {string} operation @param {string} receiver @param {string} element @param {boolean} reference @param {number} count @param {string} startKind @param {string} start @param {string} endKind @param {string} end @returns {boolean} */
export function acceptRustScalarVectorCopyValue(language,kind,operation,receiver,element,reference,count,startKind,start,endKind,end){
 return language==='Rust'&&kind==='method'&&operation==='slice'&&receiver==='array'&&!reference&&count===2&&startKind==='float'&&(start==='0'||start==='0.0')&&endKind==='float'&&end==='Infinity'&&(element==='fixed'||element==='float'||element==='bool'||element==='unit');
}
/** @param {string} target @param {string} object @returns {string} */
export function renderRustScalarVectorCopyValue(target,object){
 if(target==='Rust')return '::std::clone::Clone::clone('+object+')';
 if(target==='JavaScript')return '[...('+object+')]';
 if(target==='Lean'||target==='Rocq')return '('+object+')';
 return '';
}
/** @returns {string} */
export function readRustScalarVectorCopyEncoding(){return 'standard-scalar-vector-copy';}
/** @returns {string} */
export function readRustScalarVectorCopyStatement(){return 'standard dense scalar vector copies evaluate one receiver and copy every element; Rust uses the actual Clone operation without floating-point length conversion, JavaScript creates a fresh array and proof targets retain immutable list values';}
