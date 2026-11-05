// Files/bytes and command arguments are transport. The dynamic library owns
// grammar execution, recursive resolution, concept proofs and saved CSTs.
import {DynamicMetaLanguage, createDynamicMetaLanguage} from './dynamic-meta-language.js';
import {parseGrammarLinks, renderGrammarLinks} from './grammar-links.js';
import {readMetaLanguageArchiveText, readMetaLanguageArchiveBinary,
  metaLanguageArchiveToText, metaLanguageArchiveToBinary} from './meta-language-archive.js';
import {readStructuredDataLino, structuredDataToLino} from './structured-data-links.js';
import {acceptDynamicLanguageCommand, readDynamicLanguageArgumentArity,
  readDynamicLanguageProjection, acceptDynamicLanguageConfigurationField,
  readDynamicLanguageResourceField, readDynamicLanguageCommandUsage} from './dynamic-language-command-rules.js';

const usage = readDynamicLanguageCommandUsage();

function argumentsOf(args) {
  const [command, ...rest] = args;
  if (!acceptDynamicLanguageCommand(command ?? '')) throw new TypeError('unknown language operation');
  const result = {command, sources:[], options:new Map(), file:undefined};
  for (let index=0; index<rest.length; index+=1) {
    const option=rest[index];
    if (!option.startsWith('--') && command!=='prepare' && result.file===undefined) {result.file=option;continue;}
    const arity=readDynamicLanguageArgumentArity(command,option);
    if (arity<0 || index+arity>=rest.length) throw new TypeError('unexpected or incomplete language argument '+option);
    const values=rest.slice(index+1,index+arity+1); index+=arity;
    if(option==='--source' && arity===4) result.sources.push(values);
    else {
      if(result.options.has(option)) throw new TypeError('duplicate language argument '+option);
      result.options.set(option,values[0]??true);
    }
  }
  if(command!=='prepare' && result.file===undefined) throw new TypeError('a language archive input is required');
  return result;
}
function configuration(path,readFile) {
  if(path===undefined)return {};
  const value=readStructuredDataLino(readFile(path));
  if(!value || typeof value!=='object' || Array.isArray(value)
    || Object.keys(value).some(key=>!acceptDynamicLanguageConfigurationField(key))) throw new TypeError('unknown language configuration field');
  if(value.records!==undefined&&!Array.isArray(value.records))throw new TypeError('records must be an array');
  if(value.equivalences!==undefined&&(!Array.isArray(value.equivalences)||value.equivalences.some(pair=>!Array.isArray(pair)||pair.length!==2||pair.some(item=>typeof item!=='string'))))throw new TypeError('equivalences must be pairs of source identities');
  if(value.formalDefinitions!==undefined&&(!value.formalDefinitions||typeof value.formalDefinitions!=='object'||Array.isArray(value.formalDefinitions)))throw new TypeError('formal definitions must be a record');
  return value;
}
function parseOptions(options) {
  const result={projection:readDynamicLanguageProjection(options.get('--projection')??'cst')};
  if(!result.projection)throw new TypeError('unknown syntax projection');
  for(const [option,value]of options) {
    const field=readDynamicLanguageResourceField(option);
    if(field) {
      if(typeof value!=='string'||! /^(0|[1-9][0-9]*)$/u.test(value)||!Number.isSafeInteger(Number(value)))throw new TypeError('a parser limit needs a nonnegative safe integer');
      result[field]=Number(value);
    }
  }
  if(options.has('--start-rule'))result.startRule=options.get('--start-rule');
  if(options.has('--identifier'))result.identifier=options.get('--identifier');
  if(options.has('--replace-snapshot'))result.replaceSnapshot=true;
  if(options.has('--error-recovery'))result.errorRecovery=true;
  if(options.has('--accept-recovery'))result.recovery='accept';
  if(options.has('--reject-ambiguity'))result.ambiguity='reject';
  return result;
}

export function runDynamicLanguageCommand(args,{readFile,readBytes}) {
  if(['help','--help','-h'].includes(args[0]))return {exitCode:0,stdout:usage,stderr:''};
  try {
    const {command,sources,options,file}=argumentsOf(args);
    const config=configuration(options.get('--configuration'),readFile);
    let session;
    const additions=sources.map(([id,language,edition,path])=>({id,language,edition,grammar:parseGrammarLinks(readFile(path))}));
    if(command==='prepare') {
      if(!additions.length)throw new TypeError('prepare needs complete source grammars');
      session=createDynamicMetaLanguage(additions,config);
    } else {
      const archive=options.has('--input-binary')?readMetaLanguageArchiveBinary(readBytes(file)):readMetaLanguageArchiveText(readFile(file));
      session=new DynamicMetaLanguage(archive);
      if(command==='extend') {
        if(!additions.length)throw new TypeError('extend needs complete source grammars');
        session=session.extend(additions,{...config,replace:options.has('--replace')});
      }
    }
    if(command==='list') {
      const selected=options.get('--source');
      const rows=session.sources.filter(source=>selected===undefined||source.id===selected).map(source=>({identifier:source.id,language:source.language,edition:source.edition,rules:source.grammar.rules.size}));
      if(selected!==undefined&&!rows.length)throw new TypeError('unknown source-qualified grammar identifier');
      return {exitCode:0,stdout:structuredDataToLino(rows),stderr:''};
    }
    if(command==='grammar') {
      const source=session.sources.find(source=>source.id===options.get('--source'));
      if(!source)throw new TypeError('grammar needs a known source-qualified identifier');
      return {exitCode:0,stdout:renderGrammarLinks(source.grammar),stderr:''};
    }
    let archive=session.archive,exitCode=0,stderr='';
    if(command==='parse') {
      if(!options.has('--source')||!options.has('--input'))throw new TypeError('parse needs a source identity and an input file');
      const parsed=session.parse(options.get('--source'),readBytes(options.get('--input')),parseOptions(options));
      if(!parsed.archive)return {exitCode:1,stdout:'',stderr:structuredDataToLino({rejection:parsed.rejection??null})};
      archive=parsed.archive;
      if(!parsed.ok){exitCode=1;stderr=structuredDataToLino({rejection:parsed.rejection??null});}
    }
    return {exitCode,stdout:options.has('--binary')?metaLanguageArchiveToBinary(archive):metaLanguageArchiveToText(archive),stderr};
  } catch(error) {return {exitCode:2,stdout:'',stderr:'error: '+error.message+'\n'+usage};}
}
