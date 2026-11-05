// Catalog identities qualify actual complete programs, never equal meanings.
/** @param {number} version @param {string} model @param {number} count @returns {boolean} */
export function acceptSourceDefinitionCatalogHeader(version,model,count){
 return version===1&&model==='complete qualified typed source programs 1'&&count>=1&&count<=4096&&count%1===0;
}
/** @param {string} text @returns {boolean} */
export function acceptSourceDefinitionCatalogAsset(text){
 if(text.length<14||text.slice(text.length-13)!=='-program.lino')return false;
 for(let index=0;index<text.length-13;index+=1){const code=text.charCodeAt(index);if(!(code>=97&&code<=122||index>0&&code>=48&&code<=57||index>0&&code===45))return false;}
 return true;
}
/** @param {string} text @returns {boolean} */
export function acceptSourceDefinitionCatalogDigest(text){
 if(text.length!==64)return false;
 for(let index=0;index<text.length;index+=1){const code=text.charCodeAt(index);if(!(code>=48&&code<=57||code>=97&&code<=102))return false;}
 return true;
}
/** Positive decimal identifiers in the current exact safe-number graph profile.
 * This does not narrow the separate unsigned64 network profile.
 * @param {string} text @returns {boolean} */
export function acceptSourceDefinitionCatalogRoot(text){
 if(text.length<1||text.length>16||text.charCodeAt(0)===48)return false;
 for(let index=0;index<text.length;index+=1){const code=text.charCodeAt(index);if(code<48||code>57)return false;}
 return text.length<16||text<='9007199254740991';
}
/** @param {string[]} identifiers @param {string[]} assets @param {string[]} digests
 * @param {number[]} bytes @param {string[]} roots @returns {number} */
export function validateSourceDefinitionCatalogEntries(identifiers,assets,digests,bytes,roots){
 if(identifiers.length!==assets.length||identifiers.length!==digests.length||identifiers.length!==bytes.length||identifiers.length!==roots.length)return -1;
 for(let index=0;index<identifiers.length;index+=1){
  const identifier=identifiers[index]??'',asset=assets[index]??'',digest=digests[index]??'',count=bytes[index]??0,root=roots[index]??'';
  if(!acceptSourceDefinitionCatalogAsset(asset))return -2;
  if(identifier!=='meta-language/source-program/'+asset.slice(0,asset.length-5))return -3;
  if(!acceptSourceDefinitionCatalogDigest(digest))return -4;
  if(count<1||count>134217728||count%1!==0)return -5;
  if(!acceptSourceDefinitionCatalogRoot(root))return -6;
  for(let previous=0;previous<index;previous+=1)if(identifiers[previous]===identifiers[index]||assets[previous]===assets[index])return -7;
 }
 return 1;
}
/** @param {string[]} identifiers @param {string} identifier @returns {number} */
export function selectSourceDefinitionCatalogEntry(identifiers,identifier){
 for(let index=0;index<identifiers.length;index+=1)if(identifiers[index]===identifier)return index;
 return -1;
}
/** @param {number} byteLimit @param {number} nodeLimit @param {number} depthLimit @returns {boolean} */
export function acceptSourceDefinitionCatalogLimits(byteLimit,nodeLimit,depthLimit){
 return byteLimit>=1&&byteLimit<=134217728&&byteLimit%1===0&&nodeLimit>=1&&nodeLimit<=1000000&&nodeLimit%1===0&&depthLimit>=1&&depthLimit<=1024&&depthLimit%1===0;
}
/** @param {string} identifier @param {string} digest @returns {string} */
export function readSourceDefinitionCatalogIdentity(identifier,digest){return identifier+'@sha256:'+digest;}
/** @param {string} command @returns {number} */
export function readSourceDefinitionCommandArity(command){return command==='list'?0:command==='program'?1:command==='function'?2:-1;}
/** @param {string} command @param {number} count @param {boolean} binary @returns {boolean} */
export function acceptSourceDefinitionCommandArguments(command,count,binary){
 return readSourceDefinitionCommandArity(command)>=0&&readSourceDefinitionCommandArity(command)===count&&!(command==='list'&&binary);
}
/** @returns {string} */
export function readSourceDefinitionCommandUsage(){
 return 'usage: meta-language definition list\n       meta-language definition program <qualified-identifier> [--binary]\n       meta-language definition function <qualified-identifier> <declaration> [--binary]\n';
}
