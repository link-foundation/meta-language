// Source roles and type contexts come from the original complete CST.
/** @param {string} term @returns {boolean} */
export function acceptSourceModuleRequestTerm(term){return term==='import_statement'||term==='export_statement'||term==='call_expression';}
/** @param {string} term @returns {boolean} */
export function acceptSourceModuleRequestTypeScope(term){return ['type_alias_declaration','type_annotation','type_arguments','type_parameters','interface_declaration','type_query','ambient_declaration'].includes(term);}
/** @param {string} term @returns {boolean} */
export function acceptSourceModuleRequestLiteral(term){return term==='string';}
/** @param {string} term @returns {boolean} */
export function acceptSourceModuleRequestImportCallee(term){return term==='import';}
/** @param {number} code @returns {string} */
export function readSourceModuleRequestKind(code){return code===1?'module import':code===2?'module export':code===3?'dynamic module import':code===4?'type module import':code===5?'type module export':'';}
/** @param {string} language @param {string} term @param {string} second
 * @param {string} third @param {boolean} context @returns {number} */
export function readSourceModuleRequestCode(language,term,second,third,context){
 const typeOnly=language==='TypeScript'&&(context||(second==='type'||second==='typeof')&&third!=='from'&&third!==','&&third!=='=');
 if(term==='call_expression')return typeOnly?4:3;
 if(term==='import_statement')return typeOnly?4:1;
 return term==='export_statement'?(typeOnly?5:2):0;
}

/** @param {string} language @param {boolean} tsx @returns {string} */
export function readSourceModuleRequestGrammar(language,tsx){return language==='JavaScript'?'JavaScript':language==='TypeScript'?(tsx?'tsx':'TypeScript'):'';}

/** [file argument index, language code (1 JS/2 TS), TSX flag, binary flag].
 * [-1] requests help; [-2] invalid arguments.
 * @param {string[]} args @returns {number[]} */
export function planSourceModuleRequestCommand(args){
 let file=-1,language=0,tsx=false,binary=false,ended=false;
 for(let at=0;at<args.length;at+=1){const arg=args[at]??'';
  if(!ended&&(arg==='--help'||arg==='-h'||arg==='help'))return [-1];
  if(!ended&&arg==='--'){ended=true;continue;}
  if(!ended&&arg==='--tsx'){if(tsx)return [-2];tsx=true;continue;}
  if(!ended&&arg==='--binary'){if(binary)return [-2];binary=true;continue;}
  if(!ended&&arg==='--language'){
   if(language!==0)return [-2];at+=1;const value=args[at]??'';
   if(['JavaScript','javascript','js'].includes(value))language=1;
   else if(['TypeScript','typescript','ts'].includes(value))language=2;
   else return [-2];continue;
  }
  if(file>=0||!ended&&arg.startsWith('-'))return [-2];file=at;
 }
 if(file<0||tsx&&language===1)return [-2];
 const path=args[file]??'';
 if(language!==1&&path.endsWith('.tsx'))tsx=true;
 if(language===0)language=tsx||path.endsWith('.ts')||path.endsWith('.mts')||path.endsWith('.cts')?2:1;
 return [file,language,tsx?1:0,binary?1:0];
}
