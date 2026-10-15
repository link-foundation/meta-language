// Generator input only; all runtime decisions consume the generated linked data.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
const source=readProjectData(new URL('../rust-text-value-projection/data/policy.lino',import.meta.url));
const names=readProjectData(new URL('../../js/src/translation/data/rust-pure-unit-calls.lino',import.meta.url));
const strings=['','Rust',source.model,source.observationModel,source.signatureModel,'fn','pub','main','(',')',':',',','&','String','str','->','{','}','::','from','.','clone','to_owned','to_string','identifier','punct','string','eof','library','primitive','std','reference','lit','var','bool',source.inputAssumption.id,source.inputAssumption.meaning,source.valueAssumption.id,source.valueAssumption.meaning,...Object.values(source.domains),...source.operations.map(row=>row.name),'text value function','text value body','independent empty main','complete standard Rust text value module',...names.forbiddenNames,...source.reservedNames,'_'];
export const symbols=[...new Set(strings)];
export const symbol=string=>symbols.indexOf(string)+1;
export const textCertificatePolicy={
 version:1,model:'portable closed Rust text source certificate 1',sourceModel:source.model,
 sourcePolicy:'experiments/rust-text-value-projection/data/policy.lino',sourceSignatureModel:source.signatureModel,
 protectedSources:['../rust-text-value-projection/source.mjs','../rust-text-value-projection/model.mjs','../rust-text-value-projection/data/policy.lino','../rust-text-value-projection/data/fixtures.lino','../rust-text-value-projection/rules.mjs'].map(path=>({path,sha256:createHash('sha256').update(readFileSync(new URL(path,import.meta.url))).digest('hex')})),
 symbols,domains:source.domains,maximumParameters:source.maximumParameters,
 syntax:{visibility:['pub'],functionPrefix:['fn'],parametersOpen:['('],parameterType:[':'],parameterDelimiter:[','],result:[')','->','String'],bodyOpen:['{'],bodyClose:['}'],main:['fn','main','(',')','{','}'],end:['']},
 symbolsDictionaryMeaning:'Each fixed token and model symbol is its exact primitive String from this ordered linked dictionary; unknown symbols are interned by value, never by ownership.',
 maximumRecords:100000,maximumIdentity:9007199254740991,nullIdentity:0,absentIdentity:-1,numberDomain:'Actual finite safe integral Numbers from minus one through the safe maximum, excluding negative zero. Primitive Strings and booleans are distinct and refused.',
 identityKinds:{token:1,declaration:2,parameter:3,signature:4,position:5,core:6,operation:7,main:8,unregistered:9},
 layouts:{
  context:['schemaVersion','sourceLanguage','main','mainEffects','itemsCount','declarationsCount','proofWorker','proofModel','proofLanguage','observation','rootOperation','sourceStart','sourceEnd','mainStart','mainEnd'],
  token:['identity','kind','syntax','role','target','value','rawName','start','end'],
  declaration:['identity','name','fullName','moduleDepth','signature','core','parameterStart','parameterCount','resultKind','recursive','decreasing','start','end','mapKey','signatureModel','signatureLanguage','signatureLifetimeBindings','kind','signatureArity','signatureResult'],
  parameter:['identity','declaration','ordinal','name','type','guard','default','position'],
  position:['identity','signature','ordinal','type','ownerKind','pathStart','pathCount','layerStart','layerCount','lifetimeCount','ownerArguments'],
  core:['identity','declaration','kind','type','value','start','end'],
  function:['declaration','signature','resultPosition','operation','core','input','position','literal'],
  operation:['identity','code','start','end','partStart','partCount'],
  condition:['id','meaning','declaration','operation','core','input','position'],
  name:['start','length'],
 },
 roles:{fixed:0,declaration:1,binding:2,literal:3},
 operations:source.operations.map((row,index)=>({...row,code:index+1,prefix:row.name==='standard String construction from borrowed text'?['String','::','from','(']:[],suffix:row.name==='standard String construction from borrowed text'?[')']:row.name==='owned standard String move'?[]:['.',row.name==='standard String clone'?'clone':row.name==='standard borrowed str owned construction'?'to_owned':'to_string','(',')']})),
 operationCodes:{function:6,body:7,main:8,root:9},
 parameterTypes:[{domain:1,syntax:['String']},{domain:2,syntax:['&','String']},{domain:3,syntax:['&','str']}],
 forbiddenNames:[...new Set([...names.forbiddenNames,...source.reservedNames,'_',''])],
 output:['decision','consumedTokens','functions','inputConditions','allocationConditions'],
 limits:['The ordinary complete production parser remains the source authority. This kernel verifies its existing finite text grammar and actual graph role relations; it creates no second body AST.','The host assigns dense safe IDs by actual object identity and interned symbols by exact primitive String value. Equal symbols never create ownership identity. Null and absent references remain distinct.','The bridge may validate descriptors, record shapes, complete lexer/CST and existing shared SourceSignature records, then faithfully project actual roles. It does not choose an operation, classify a source receiver or approve ownership.','The kernel consumes every token, verifies ordered declarations and parameter scopes, actual operation children/header/core facts and all actual input/allocation conditions.','Array.push cannot yet be automatically translated. This kernel therefore returns a fixed numeric result array; this is a translator gap, not a handwritten native substitute.','Existing model, policies, fixtures, APIs and acceptance boundaries stay unchanged. This experiment supplies no production selector or native integration.'],
};
