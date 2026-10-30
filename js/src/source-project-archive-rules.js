import {readRequiredTextFact,readRequiredNumberFact} from './translation/fact-column-rules.js';
/** @param {string} role @returns {string} */
export function readSourceProjectArchiveVocabulary(role){
 if(role==='model')return 'complete source project concrete graphs 1';
 if(role==='directory')return 'meta-language/source-project/directory';
 if(role==='sourcePrefix')return 'meta-language/source-project/source/';
 if(role==='root')return 'complete original source concrete roots';
 return role==='language'?'meta-language source project':'';
}
/** Project-relative source identities are independent of host path syntax.
 * @param {string} path @returns {boolean} */
export function acceptSourceProjectPath(path){
 if(path===''||path.startsWith('/')||path.includes('\\')||path.includes('\0')||path.includes(':'))return false;
 for(const part of path.split('/'))if(part===''||part==='.'||part==='..')return false;
 return true;
}
/** @param {string} projection @returns {boolean} */
export function acceptSourceProjectSyntaxProjection(projection){return projection==='ConcreteSyntax'||projection==='AbstractSyntax';}
/** Source roots are all actual Syntax nodes without an enclosing Syntax node.
 * The archive retains every other node, including error and missing nodes.
 * @param {string[]} kinds @param {number[]} identifiers @param {number[]} firsts
 * @param {number[]} counts @param {number[]} targets @returns {number[]} */
export function selectSourceProjectSyntaxRootPositions(kinds,identifiers,firsts,counts,targets){
 if(kinds.length!==identifiers.length||kinds.length!==firsts.length||kinds.length!==counts.length)throw new TypeError('unaligned source project graph columns');
 const nested=[];
 for(let position=0;position<kinds.length;position+=1){
  if(readRequiredTextFact(kinds,position)!=='syntax')continue;
  const first=readRequiredNumberFact(firsts,position),count=readRequiredNumberFact(counts,position);
  if(!Number.isSafeInteger(first)||!Number.isSafeInteger(count)||first<0||count<0||first+count>targets.length)throw new TypeError('invalid source project reference interval');
  for(let child=0;child<count;child+=1)nested.push(readRequiredNumberFact(targets,first+child));
 }
 const children=new Set(nested),roots=[];
 for(let position=0;position<kinds.length;position+=1)if(readRequiredTextFact(kinds,position)==='syntax'&&!children.has(readRequiredNumberFact(identifiers,position)))roots.push(position);
 return roots;
}
/** @param {string} value @returns {boolean} */
function acceptSourceProjectDigest(value){
 if(value.length!==64)return false;
 for(let index=0;index<value.length;index+=1)if(!'0123456789abcdef'.includes(value.slice(index,index+1)))return false;
 return true;
}
/** Complete supported directory admission, independent of a current catalog.
 * Old archives retain their actual grammar provenance instead of redispatching.
 * @param {string} model @param {number} version @param {string} projection
 * @param {string[]} paths @param {string[]} languages @param {string[]} identifiers
 * @param {number[]} bytes @param {string[]} digests @returns {string} */
export function readSourceProjectDirectoryError(model,version,projection,paths,languages,identifiers,bytes,digests){
 if(model!==readSourceProjectArchiveVocabulary('model')||version!==1||!acceptSourceProjectSyntaxProjection(projection))return 'unknown source project directory model';
 if(paths.length===0||languages.length!==paths.length||identifiers.length!==paths.length||bytes.length!==paths.length||digests.length!==paths.length)return 'unaligned source project directory';
 const names=new Set(paths);if(names.size!==paths.length)return 'duplicate source project path';
 for(let index=0;index<paths.length;index+=1){
  const size=readRequiredNumberFact(bytes,index),path=readRequiredTextFact(paths,index);
  if(!acceptSourceProjectPath(path)||readRequiredTextFact(languages,index)===''||readRequiredTextFact(identifiers,index)!==readSourceProjectArchiveVocabulary('sourcePrefix')+path||!Number.isSafeInteger(size)||size<0||!acceptSourceProjectDigest(readRequiredTextFact(digests,index)))return 'invalid source project directory entry';
 }
 return '';
}
/** Appending cannot silently replace an older source interpretation.
 * @param {string[]} existing @param {string[]} added @returns {string} */
export function readSourceProjectExtensionError(existing,added){
 if(added.length===0)return 'expected a bounded nonempty source inventory';
 const prior=new Set(existing),incoming=new Set(added);
 if(incoming.size!==added.length)return 'duplicate source project path';
 for(const path of added){if(!acceptSourceProjectPath(path))return 'invalid original source input';if(prior.has(path))return 'source project extension cannot replace an existing source';}
 return '';
}
