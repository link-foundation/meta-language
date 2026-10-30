// Complete project products remain drafts; source membership is independently
// checkable from the actual full concrete archive. Equal hashes prove bytes,
// never semantic translation or equivalence.
/** @param {string} role @returns {string} */
export function readProjectTranslationArchiveVocabulary(role){
 if(role==='draft')return 'meta-language/project-translation/draft';
 if(role==='configuration')return 'meta-language/project-translation/configuration';
 if(role==='sources')return 'meta-language/project-translation/sources';
 return role==='interpretation'?'independent source semantic translation':'';
}
/** @param {string} language @param {string} path @returns {string} */
export function readProjectTranslationArchiveSourceLanguage(language,path){return language==='TypeScript'&&path.endsWith('.tsx')?'TSX':language;}
/** @param {number} version @param {string} interpretation @param {number} inventory
 * @param {number} modules @param {number} sources @returns {string} */
export function readProjectTranslationArchiveHeaderError(version,interpretation,inventory,modules,sources){
 return version===1&&interpretation===readProjectTranslationArchiveVocabulary('interpretation')&&inventory>0&&inventory===modules&&inventory===sources?'':'incomplete or unknown project translation archive';
}
/** @param {string} identifier @param {string} language @param {number} bytes
 * @param {string} digest @param {string} path @param {string} actualLanguage
 * @param {number} actualBytes @param {string} actualDigest @returns {string} */
export function readProjectTranslationArchiveSourceError(identifier,language,bytes,digest,path,actualLanguage,actualBytes,actualDigest){
 return identifier===path&&readProjectTranslationArchiveSourceLanguage(language,identifier)===actualLanguage&&Number.isSafeInteger(bytes)&&bytes>=0&&bytes===actualBytes&&digest===actualDigest?'':'project source identity, language, byte count or digest changed';
}
/** @param {boolean} draft @param {boolean} executable @param {number} gaps
 * @param {boolean} identical @param {boolean} digest @returns {string} */
export function readProjectTranslationArchiveProductError(draft,executable,gaps,identical,digest){
 if(executable&&(!draft||gaps!==0||!identical))return 'an unresolved or changed project product cannot be executable';
 return digest?'':'project draft product digest changed';
}
/** Ordered source facts retain duplicates and actual dependency identities.
 * @param {string[]} issues @param {string[]} actualIssues @param {string[]} gapIssues
 * @param {string[]} specifiers @param {string[]} providers
 * @param {string[]} actualSpecifiers @param {string[]} actualProviders @returns {string} */
export function readProjectTranslationArchiveBindingError(issues,actualIssues,gapIssues,specifiers,providers,actualSpecifiers,actualProviders){
 if(issues.length!==actualIssues.length||issues.length!==gapIssues.length||specifiers.length!==providers.length||specifiers.length!==actualSpecifiers.length||specifiers.length!==actualProviders.length)return 'project source issues or dependency directory changed';
 for(let index=0;index<issues.length;index+=1)if(issues[index]!==actualIssues[index]||issues[index]!==gapIssues[index])return 'project source issues or dependency directory changed';
 for(let index=0;index<specifiers.length;index+=1)if(specifiers[index]!==actualSpecifiers[index]||providers[index]!==actualProviders[index])return 'project source issues or dependency directory changed';
 return '';
}
/** File and target argument indices, AST flag, binary flag. [-1] is help;
 * [-2] invalid arguments. Selection does not execute source or products.
 * @param {string[]} args @returns {number[]} */
export function planProjectTranslationArchiveCommand(args){
 let file=-1,target=-1,abstract=false,binary=false,ended=false;
 for(let at=0;at<args.length;at+=1){const arg=args[at]??'';
  if(!ended&&(arg==='--help'||arg==='-h'||arg==='help'))return [-1];
  if(!ended&&arg==='--'){ended=true;continue;}
  if(!ended&&arg==='--abstract'){if(abstract)return [-2];abstract=true;continue;}
  if(!ended&&arg==='--binary'){if(binary)return [-2];binary=true;continue;}
  if(!ended&&arg==='--to'){if(target>=0||at+1>=args.length)return [-2];at+=1;target=at;continue;}
  if(file>=0||!ended&&arg.startsWith('-'))return [-2];file=at;
 }
 return file>=0&&target>=0?[file,target,abstract?1:0,binary?1:0]:[-2];
}

/** Selected settings remain data; this checks only schema and target identity.
 * @param {number} version @param {number} count @param {number} inventory
 * @param {string} target @param {string} selected @returns {string} */
export function readProjectTranslationArchiveConfigurationError(version,count,inventory,target,selected){return version===1&&count===inventory&&target!==''&&target===selected?'':'project translation configuration changed';}

/** Directory membership facts come from the complete actual source archive.
 * @param {string[]} specifiers @param {boolean[]} providers @returns {string} */
export function readProjectTranslationArchiveImportError(specifiers,providers){
 if(specifiers.length!==providers.length||new Set(specifiers).size!==specifiers.length)return 'duplicate or foreign project source dependency';
 for(const known of providers)if(!known)return 'duplicate or foreign project source dependency';
 return '';
}
