/** Closed owned tagged values have one complete own-field directory. This
 * profile starts with a whole value of the same actual nominal owner, then
 * permits only whole same-owner copies or declared-field overrides. It never
 * drops extra fields, changes tags, guesses unions or substitutes prototypes.
 * @param {string} language @param {number} constructors @param {string[]} declared
 * @param {string[]} kinds @param {string[]} names @param {string[]} owners
 * @param {string} owner @param {boolean} prototype @returns {string} */
export function readJavaScriptNominalSpreadError(language,constructors,declared,kinds,names,owners,owner,prototype){
  if(language!=='JavaScript')return 'nominal object copies require their actual JavaScript source owner';
  if(constructors!==1||owner.length===0)return 'nominal object copies need one complete actual constructor';
  if(prototype)return 'nominal object copies cannot replace their complete prototype interpretation';
  if(kinds.length===0||kinds.length>65536||kinds.length!==names.length||kinds.length!==owners.length||kinds[0]!=='spread')return 'nominal object copies start with a complete actual source value';
  if(declared.length>65536)return 'nominal object field directories exceed the explicit source budget';
  for(let index=0;index<declared.length;index+=1){const name=declared[index]??'';if(name.length===0||name==='$'||declared.indexOf(name)!==index)return 'nominal object copies require complete unique declared fields';}
  for(let index=0;index<kinds.length;index+=1){
    const kind=kinds[index]??'',name=names[index]??'',source=owners[index]??'';
    if(kind==='spread'){if(name!==''||source!==owner)return 'nominal object copies retain their actual complete source owner';}
    else if(kind!=='field'||source!==''||!declared.includes(name))return 'nominal object copies cannot add fields or replace their constructor tag';
  }
  return '';
}
/** Select only the branch that excludes one exact absence. The host transports
 * actual strict-equality syntax and unchanged lexical binder identities.
 * @param {string} kind @param {string} operation @param {string} absence @returns {number} */
export function readJavaScriptNominalSpreadGuardArm(kind,operation,absence){
  if(kind!=='binary'||absence!=='undefined'&&absence!=='null')return 0;
  return operation==='ne'?1:operation==='eq'?2:0;
}
