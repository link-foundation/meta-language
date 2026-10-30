/** Exact ordered record roles are shared; owning records and value edges are
 * separate occurrences. The adapter supplies actual link/field facts.
 * @param {boolean} selected @param {number} references @param {boolean} direct
 * @param {boolean} shapeFields @param {boolean} qualifiedShape @param {string} language @param {string} term
 * @param {string} definition @param {string} expectedLanguage @param {string} expectedTerm
 * @param {string[]} names @param {boolean[]} fields @param {boolean[]} self
 * @param {boolean[]} roleFields @returns {string} */
export function readTypedProgramRecordShapeError(selected,references,direct,shapeFields,qualifiedShape,language,term,definition,expectedLanguage,expectedTerm,names,fields,self,roleFields){
 if(!selected)return 'ordered program records require their explicit root layout profile';
 if(direct||shapeFields)return 'ordered program record values and shapes cannot also carry named fields';
 if(!qualifiedShape||language!==expectedLanguage||term!==expectedTerm||definition!=='')return 'ordered program records require their actual qualified field shape';
 if(!Number.isSafeInteger(references)||references<1||references>100001||names.length!==references-1||names.length!==fields.length||names.length!==self.length||names.length!==roleFields.length)return 'ordered program record roles must match every actual value coordinate';
 const ordered=names.toSorted();
 for(let index=0;index<names.length;index+=1){
  const name=names[index]??'';
  if(name.length===0||name.length>1000000||(index>0&&ordered[index]===ordered[index-1])||fields[index]!==true||self[index]!==true||roleFields[index]!==false)return 'ordered program record roles require complete unique field points';
 }
 return '';
}

/** @param {number} count @param {boolean} qualified @param {number} references
 * @param {boolean} fields @returns {string} */
export function readTypedProgramRecordShapeSelectionError(count,qualified,references,fields){
 if(count===0)return '';
 if(count!==1)return 'duplicate root layout selection';
 return !qualified||references!==0||fields?'invalid root layout selection':'';
}

/** Bound derived role text before adapters allocate owned copies. Repeated
 * references cannot amplify one stored label into an unbounded projection.
 * @param {number} units @param {boolean} oversized @returns {string} */
export function readTypedProgramRecordRoleTextError(units,oversized){
 if(oversized)return 'ordered program record roles require complete unique field points';
 return !Number.isSafeInteger(units)||units<0||units>16777216?'ordered program record role text exceeds its projection resource budget':'';
}
