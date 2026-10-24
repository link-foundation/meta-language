// Actual source type facts select existing Unit and ordered binding operations.
/** A collecting pass defers only direct call-result ownership to the complete
 * declaration catalog. The final source pass requires an actual direct Unit.
 * @param {string} kind @param {boolean} collecting @param {boolean} proven
 * @param {string} type @param {boolean} indirected @returns {string} */
export function readRustUnitStatementDisposition(kind,collecting,proven,type,indirected){
 if(kind==='abort')return 'abort';
 if(proven&&type==='unit'&&!indirected)return 'bind';
 return collecting&&kind==='app'?'bind':'refuse';
}
