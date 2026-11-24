// Actual complete constructor columns, not names inferred from an encoding ID.
/** Select the constructor holding each standard sum argument. Empty refuses.
 * @param {string} owner @param {string[]} names @param {number[]} fields
 * @returns {number[]} */
export function readRustStandardSumSignatureArgumentConstructors(owner,names,fields){
 if(names.length!==2||fields.length!==2)return [];
 if(owner==='std::option::Option'&&names[0]==='None'&&names[1]==='Some'&&fields[0]===0&&fields[1]===1)return [1];
 if(owner==='std::result::Result'&&names[0]==='Ok'&&names[1]==='Err'&&fields[0]===1&&fields[1]===1)return [0,1];
 return [];
}
