// File/process transport around shared argument planning and complete archives.
import {planProjectTranslationArchiveCommand} from './project-translation-archive-rules.js';
import {readStructuredDataLino} from './structured-data-links.js';
import {createProjectTranslationArchive} from './project-translation-archive.js';
import {metaLanguageArchiveToText,metaLanguageArchiveToBinary} from './meta-language-archive.js';
const USAGE='usage: meta-language project-translation <inventory.lino> --to <language> [--abstract] [--binary]\nDrafts every source and preserves complete original CSTs, products and diagnostics as Links. AST is an optional view. No generated program is executed.\n';
export function runProjectTranslationArchiveCommand(args,{readFile}){
 const plan=planProjectTranslationArchiveCommand(args);
 if(plan.length===1&&plan[0]===-1)return{exitCode:0,stdout:USAGE,stderr:''};
 if(plan.length!==4)return{exitCode:2,stdout:'',stderr:'error: invalid project translation arguments\n'+USAGE};
 try{const value=readStructuredDataLino(readFile(args[plan[0]]));
  if(!value||typeof value!=='object'||Array.isArray(value)||!Array.isArray(value.inputs)||value.options!==undefined&&value.options!==null&&(typeof value.options!=='object'||Array.isArray(value.options)))throw new TypeError('project translation archive: expected a source inventory record');
  const options={...(value.options??{}),targetLanguage:args[plan[1]],projection:plan[2]===1?'AbstractSyntax':'ConcreteSyntax'},packet=createProjectTranslationArchive(value.inputs,options),bounds={nodeLimit:options.nodeLimit??1000000,referenceLimit:options.referenceLimit??4000000,byteLimit:options.byteLimit??128*1024*1024};
  return{exitCode:0,stdout:plan[3]===1?metaLanguageArchiveToBinary(packet,bounds):metaLanguageArchiveToText(packet,bounds),stderr:''};
 }catch(error){return{exitCode:1,stdout:'',stderr:'error: '+error.message+'\n'};}
}
