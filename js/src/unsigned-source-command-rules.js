// The same command interpretation is automatically emitted to native Rust.
/** @returns {string} */
export function readUnsignedSourceCommandUsage(){return 'usage: meta-language exact-source <file> --language <language> [--first-id <decimal>] [--binary] [--typed | --rust-source-owners] [--source-version <Rust version>] [--to <JavaScript|Rust|Lean|Rocq>]\nExports complete concrete source links by default; typed source is checked before graph export or code emission.\n';}
/** @param {string} value @returns {boolean} */
export function acceptUnsignedSourceCommandIdentity(value){
 if(value.length===0||value.length>20)return false;
 let first=0;
 for(let at=0;at<value.length;at+=1){const code=value.charCodeAt(at);if(code<48||code>57)return false;if(first===at&&code===48)first+=1;}
 if(first===value.length)return false;
 const digits=value.slice(first);if(digits.length<20)return true;
 const maximum='18446744073709551615';for(let at=0;at<20;at+=1){const actual=digits.charCodeAt(at),limit=maximum.charCodeAt(at);if(actual!==limit)return actual<limit;}
 return true;
}
/** @param {string} language @returns {boolean} */
function acceptUnsignedSourceOwnerLanguage(language){const source=language.toLowerCase();return source==='rust'||source==='rs';}
/** @param {string} language @returns {boolean} */
function acceptUnsignedSourceTypedLanguage(language){return ['javascript','js','mjs','rust','rs'].includes(language.toLowerCase());}
/** @param {string} language @returns {boolean} */
export function acceptUnsignedSourceTarget(language){return ['javascript','js','mjs','rust','rs','lean','rocq'].includes(language.toLowerCase());}
/** @param {string} file @param {string} language @param {string} first @param {string} target @param {string} version @param {boolean} typed @param {boolean} owners @param {boolean} binary @returns {string[]} */
function finishUnsignedSourceCommand(file,language,first,target,version,typed,owners,binary){
 if(file===''||language===''||typed&&owners||binary&&target!==''||version!==''&&!owners)return ['error'];
 if(owners&&!acceptUnsignedSourceOwnerLanguage(language))return ['error'];
 if((typed||target!=='')&&!acceptUnsignedSourceTypedLanguage(language))return ['error'];
 if(target!==''&&!acceptUnsignedSourceTarget(target))return ['error'];
 return [owners?'Rust source owners':typed||target!==''?'typed source':'concrete source',file,language,first===''?'1':first,target,version,binary?'binary':'text'];
}
/** [mode, path, language, first identity, target, source version, encoding].
 * A one-element help/error result performs no I/O.
 * @param {string[]} args @returns {string[]} */
export function planUnsignedSourceCommand(args){
 let file='',language='',first='',target='',version='',typed=false,owners=false,binary=false,ended=false;
 for(let at=0;at<args.length;at+=1){const arg=args[at]??'';
  if(!ended&&(arg==='--help'||arg==='-h'||arg==='help'))return ['help'];
  if(!ended&&arg==='--'){ended=true;continue;}
  if(!ended&&arg==='--binary'){if(binary)return ['error'];binary=true;continue;}
  if(!ended&&arg==='--typed'){if(typed)return ['error'];typed=true;continue;}
  if(!ended&&arg==='--rust-source-owners'){if(owners)return ['error'];owners=true;continue;}
  if(!ended&&(arg==='--language'||arg==='--first-id'||arg==='--to'||arg==='--source-version')){
   at+=1;const value=args[at]??'';if(value===''||value.startsWith('-'))return ['error'];
   if(arg==='--language'){if(language!=='')return ['error'];language=value;}
   else if(arg==='--first-id'){if(first!==''||!acceptUnsignedSourceCommandIdentity(value))return ['error'];first=value;}
   else if(arg==='--to'){if(target!=='')return ['error'];target=value;}
   else{if(version!=='')return ['error'];version=value;}
   continue;
  }
  if(file!==''||arg===''||!ended&&arg.startsWith('-'))return ['error'];file=arg;
 }
 return finishUnsignedSourceCommand(file,language,first,target,version,typed,owners,binary);
}
