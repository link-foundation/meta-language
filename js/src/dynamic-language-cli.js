// Files/bytes and command arguments are transport. The dynamic library owns
// grammar execution, recursive resolution, concept proofs and saved CSTs.
import {DynamicMetaLanguage, createDynamicMetaLanguage} from './dynamic-meta-language.js';
import {parseGrammarLinks, renderGrammarLinks} from './grammar-links.js';
import {readMetaLanguageArchiveText, readMetaLanguageArchiveBinary,
  metaLanguageArchiveToText, metaLanguageArchiveToBinary,
  metaLanguageArchiveToLinkCliStore, readMetaLanguageArchiveLinkCliStore} from './meta-language-archive.js';
import {readStructuredDataLino, structuredDataToLino} from './structured-data-links.js';
import {inferRegularGrammar} from './grammar-inference.js';
import {inferGrammarAttributeAction} from './grammar-attribute-inference.js';
import {readRefinedRegularGrammarInferenceLinks, refineRegularGrammarInference,regularGrammarRefinementArchiveLimits} from './grammar-inference-refinement.js';
import {inferStructuralGrammar, readStructuralGrammarInferenceLinks, refineStructuralGrammarInference, structuralGrammarInferenceModelIdentifier} from './grammar-structural-inference.js';
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
  if ((result.options.has('--input-binary') && result.options.has('--input-link-cli-store'))
    || (result.options.has('--binary') && result.options.has('--link-cli-store')))
    throw new TypeError('select one archive encoding per input or output');
  if(command!=='prepare' && result.file===undefined) {
    if(command==='discover')throw new TypeError('a labelled examples input is required');
    if(command==='infer-attribute')throw new TypeError('an attribute observations input is required');
    throw new TypeError('a language archive input is required');
  }
  return result;
}

const scannerObservationFields=['scannerObservations','scannerCharacterClasses','scannerModeStackObservations'];

function labelledExamples(path,readFile,allowDiscoveryOptions) {
  if(path===undefined)throw new TypeError('refine needs a labelled examples input');
  const examples=readStructuredDataLino(readFile(path));
  const sequences=value=>Array.isArray(value)&&value.every(sequence=>Array.isArray(sequence)&&sequence.every(symbol=>typeof symbol==='string'));
  if(!examples||typeof examples!=='object'||Array.isArray(examples)
    ||Object.keys(examples).some(key=>key!=='positives'&&key!=='negatives'&&!(allowDiscoveryOptions&&(key==='delimiters'||scannerObservationFields.includes(key))))
    ||(Object.hasOwn(examples,'delimiters')&&(!Array.isArray(examples.delimiters)
      ||!examples.delimiters.every(pair=>Array.isArray(pair)&&pair.length===2&&pair.every(token=>typeof token==='string'))))
    ||!sequences(examples.positives)||!sequences(examples.negatives)) {
    throw new TypeError('labelled examples must contain positive and negative symbol sequences');
  }
  if(Object.hasOwn(examples,'delimiters')&&scannerObservationFields.some(key=>Object.hasOwn(examples,key))) {
    throw new TypeError('structural delimiters and scanner observations require separate discovery inputs');
  }
  return examples;
}

function archiveInput(file,options,readFile,readBytes,limits) {
  if(options.has('--input-link-cli-store'))return readMetaLanguageArchiveLinkCliStore(readBytes(file),limits);
  return options.has('--input-binary')?readMetaLanguageArchiveBinary(readBytes(file),limits):readMetaLanguageArchiveText(readFile(file),limits);
}

function inferAttribute(file, grammarFile, readFile) {
  if (grammarFile === undefined) throw new TypeError('infer-attribute needs a native grammar file');
  const data = readStructuredDataLino(readFile(file));
  const fields = (record, keys) => record && typeof record === 'object' && !Array.isArray(record)
    && Object.keys(record).every(key => keys.includes(key));
  const optionalText = value => value == null || typeof value === 'string';
  const coordinate = value => Number.isSafeInteger(value) && value >= 0;
  if (!fields(data, ['request', 'observations']) || !fields(data.request, ['rule', 'attribute', 'field', 'childAttribute'])
    || typeof data.request.rule !== 'string' || typeof data.request.attribute !== 'string'
    || !optionalText(data.request.field) || !optionalText(data.request.childAttribute)
    || !Array.isArray(data.observations)
    || !data.observations.every(row => fields(row, ['source', 'path', 'start', 'end', 'value'])
      && typeof row.source === 'string' && Array.isArray(row.path) && row.path.every(coordinate)
      && coordinate(row.start) && coordinate(row.end)
      && (typeof row.value === 'string' || Number.isSafeInteger(row.value)))) {
    throw new TypeError('attribute observations must contain an exact request and concrete occurrences');
  }
  const result = inferGrammarAttributeAction(parseGrammarLinks(readFile(grammarFile)), data.request, data.observations);
  return structuredDataToLino({
    candidates: result.candidates, diagnostic: result.diagnostic,
    grammar: result.grammar === null ? null : renderGrammarLinks(result.grammar),
  });
}

function archiveOutput(archive,options,limits) {
  if(options.has('--link-cli-store'))return metaLanguageArchiveToLinkCliStore(archive,limits);
  return options.has('--binary')?metaLanguageArchiveToBinary(archive,limits):metaLanguageArchiveToText(archive,limits);
}

function discoveryArchive(command,file,options,readFile,readBytes) {
  const identifier=options.get('--identifier');
  if(command==='discover') {
    const {positives,negatives,delimiters,scannerObservations,scannerCharacterClasses,scannerModeStackObservations}=labelledExamples(file,readFile,true);
    const source={identifier,language:options.get('--language'),edition:options.get('--edition')};
    if(delimiters!==undefined)return inferStructuralGrammar(positives,negatives,{...source,delimiters});
    return inferRegularGrammar(positives,negatives,{...source,scannerObservations,scannerCharacterClasses,scannerModeStackObservations});
  }
  const {network,root,entries}=archiveInput(file,options,readFile,readBytes,regularGrammarRefinementArchiveLimits);
  const structural=entries.some(entry=>entry.identifier===structuralGrammarInferenceModelIdentifier);
  if(command==='refine') {
    const {positives,negatives}=labelledExamples(options.get('--examples'),readFile);
    const refine=structural?refineStructuralGrammarInference:refineRegularGrammarInference;
    return refine(network,root,positives,negatives,{identifier});
  }
  const readInference=structural?readStructuralGrammarInferenceLinks:readRefinedRegularGrammarInferenceLinks;
  const selected=readInference(network,root);
  return createDynamicMetaLanguage([selected.source]).archive;
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
function parseOptions(options, includeProjection = true) {
  const result = {};
  if (includeProjection) {
    result.projection = readDynamicLanguageProjection(options.get('--projection') ?? 'cst');
    if (!result.projection) throw new TypeError('unknown syntax projection');
  }
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
    if(command==='infer-attribute')return {exitCode:0,stdout:inferAttribute(file,options.get('--grammar'),readFile),stderr:''};
    if(command==='discover'||command==='refine'||command==='install') {
      const archive=discoveryArchive(command,file,options,readFile,readBytes);
      return {exitCode:0,stdout:archiveOutput(archive,options,command==='install'?undefined:regularGrammarRefinementArchiveLimits),stderr:''};
    }
    const config=configuration(options.get('--configuration'),readFile);
    let session;
    const additions=sources.map(([id,language,edition,path])=>({id,language,edition,grammar:parseGrammarLinks(readFile(path))}));
    if(command==='prepare') {
      if(!additions.length)throw new TypeError('prepare needs complete source grammars');
      session=createDynamicMetaLanguage(additions,config);
    } else {
      const archive=archiveInput(file,options,readFile,readBytes);
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
    if (command === 'translate-attributes') {
      const source = options.get('--source'), input = options.get('--input');
      const target = options.get('--target'), selectionFile = options.get('--selections');
      if (source === undefined || input === undefined || target === undefined || selectionFile === undefined) {
        throw new TypeError('translate-attributes needs a source identity, input file, target and selections file');
      }
      const selections = readStructuredDataLino(readFile(selectionFile));
      if (!Array.isArray(selections) || !selections.every(selection => selection !== null
        && typeof selection === 'object' && !Array.isArray(selection)
        && Object.keys(selection).every(key => ['path', 'kind', 'attribute'].includes(key)))) {
        throw new TypeError('invalid attribute translation selection');
      }
      const translation = session.translateAttributes(source, readBytes(input), target, selections, parseOptions(options, false));
      return { exitCode: 0, stdout: translation.output.code, stderr: '' };
    }
    let archive=session.archive,exitCode=0,stderr='';
    if(command==='parse') {
      if(!options.has('--source')||!options.has('--input'))throw new TypeError('parse needs a source identity and an input file');
      const parsed=session.parse(options.get('--source'),readBytes(options.get('--input')),parseOptions(options));
      if(!parsed.archive)return {exitCode:1,stdout:'',stderr:structuredDataToLino({rejection:parsed.rejection??null})};
      archive=parsed.archive;
      if(!parsed.ok){exitCode=1;stderr=structuredDataToLino({rejection:parsed.rejection??null});}
    }
    return {exitCode,stdout:archiveOutput(archive,options),stderr};
  } catch(error) {return {exitCode:2,stdout:'',stderr:'error: '+error.message+'\n'+usage};}
}
