// Shared layer decisions over the actual owner coordinates selected by the host.
/** @param {string[]} layers @param {number} lifetimes @param {boolean} references
 * @param {boolean[]} canonicalBoxes @param {boolean} terminalBox @returns {number} */
export function readRustRecordPlaceReferenceCount(layers,lifetimes,references,canonicalBoxes,terminalBox){
 if(layers.length>256||terminalBox)return -1;
 let count=0,boxes=0;
 for(const layer of layers){
  if(layer==='reference'&&references&&boxes===0){count++;continue;}
  if(layer!=='box'||canonicalBoxes[boxes]!==true)return -1;
  boxes++;
 }
 return count===lifetimes&&boxes===canonicalBoxes.length?count:-1;
}

/** @param {string} language @param {string} model @param {number} field
 * @param {number} start @param {number} end @returns {boolean} */
export function acceptRustSourceRecordPlace(language,model,field,start,end){
 return language==='Rust'&&model==='Rust source record place 1'&&Number.isSafeInteger(field)&&field>=0&&!Object.is(field,-0)&&Number.isSafeInteger(start)&&Number.isSafeInteger(end)&&start>=0&&end>start;
}
