import {sourceDefinitionCatalog,createSourceDefinitionProgramLinks,createSourceDefinitionFormalFunctionLinks} from './source-definition-catalog.js';
import {structuredDataToLino} from './structured-data-links.js';
import {createMetaLanguageArchive,metaLanguageArchiveToText,metaLanguageArchiveToBinary} from './meta-language-archive.js';
import {acceptSourceDefinitionCommandArguments,readSourceDefinitionCommandUsage} from './source-definition-catalog-rules.js';
/** Transport over complete existing source/definition graphs, not a code cache. */
export function runSourceDefinitionCommand(args){
 const usage=readSourceDefinitionCommandUsage();
 if(['help','--help','-h'].includes(args[0]))return {exitCode:0,stdout:usage,stderr:''};
 try{
  const [command,...rest]=args,positionals=[];let binary=false;
  for(const value of rest){if(value==='--binary'&&!binary)binary=true;else if(value.startsWith('--'))throw new TypeError('unexpected definition argument '+value);else positionals.push(value);}
  if(!acceptSourceDefinitionCommandArguments(command??'',positionals.length,binary))throw new TypeError('unknown definition operation or argument count');
  if(command==='list')return {exitCode:0,stdout:structuredDataToLino(sourceDefinitionCatalog()),stderr:''};
  const program=command==='function'?createSourceDefinitionFormalFunctionLinks(positionals[0],positionals[1]):createSourceDefinitionProgramLinks(positionals[0]);
  const archive=createMetaLanguageArchive([{identifier:positionals[0]+(command==='function'?'#'+positionals[1]:''),kind:command==='function'?'formal concept definition':'typed program',network:program.network,root:program.root}]);
  return {exitCode:0,stdout:binary?metaLanguageArchiveToBinary(archive):metaLanguageArchiveToText(archive),stderr:''};
 }catch(error){return {exitCode:2,stdout:'',stderr:'error: '+error.message+'\n'+usage};}
}
