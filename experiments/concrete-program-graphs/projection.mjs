// Experimental composition of full concrete syntax and the existing checked
// program importers. No replacement parser, checker or target semantics.
import {compileGrammar} from '../../js/src/grammar.js';
import {parseGrammarLinks} from '../../js/src/grammar-links.js';
import {nativeGrammarText} from '../../js/src/native-grammar-parser.js';
import {createTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {createTypeScriptSemanticGraph} from '../typescript-typed-graph/projection.mjs';
import {readProjectData} from '../../js/scripts/project-data.mjs';

const profiles=readProjectData(new URL('../../parity/fixtures/concrete-program-source-profiles.lino',import.meta.url));
const parsers=new Map();

function parseConcreteProgramSource(source,profile,limits) {
  for(const key of Object.keys(limits)){
    if(!['maxDepth','stepLimit','memoLimit','memoryLimit'].includes(key)){
      throw new TypeError('concrete source limits do not replace grammar selection or syntax retention');
    }
  }
  if(!parsers.has(profile.grammar)){
    parsers.set(profile.grammar,compileGrammar(parseGrammarLinks(nativeGrammarText(profile.grammar))));
  }
  const outcome=parsers.get(profile.grammar).parseTree(source,limits);
  if(!outcome.ok)throw new TypeError('concrete source parser refused '+profile.language+': '+outcome.rejection.reason);
  return{tree:outcome.tree,ambiguities:outcome.ambiguities};
}

/** Retain complete import-time concrete syntax beside an editable program.
 * The existing frontend still decides the supported semantic domain. Editing
 * that program does not rewrite or certify its historical concrete origin.
 */
export function createConcreteProgramLinks(source,language,options={}) {
  if(typeof source!=='string'||Buffer.from(source).toString('utf8')!==source){
    throw new TypeError('concrete program source requires exact Unicode scalar text');
  }
  if(typeof language!=='string')throw new TypeError('concrete program language must be a registered name');
  const profile=profiles.languages.find(row=>row.aliases.includes(language.toLowerCase()));
  if(!profile)throw new TypeError('concrete program language is not registered');
  const {concreteLimits={},...semanticOptions}=options;
  if(concreteLimits===null||typeof concreteLimits!=='object'||Array.isArray(concreteLimits)){
    throw new TypeError('concrete source limits must be an object');
  }
  if(profile.adapter==='typed source'){
    // The existing TypeScript adapter already retains both complete trees.
    // An explicit smaller parse budget is checked before its semantic work.
    if(Object.keys(concreteLimits).length)parseConcreteProgramSource(source,profile,concreteLimits);
    return createTypeScriptSemanticGraph(source,semanticOptions);
  }
  if(profile.adapter!=='checked source')throw new TypeError('unknown concrete source adapter');
  const concreteSyntax=parseConcreteProgramSource(source,profile,concreteLimits);
  return createTypedProgramLinks(source,profile.language,{
    ...semanticOptions,
    provenance:{sourceProfile:{language:profile.language,model:profiles.model,
      grammar:profile.grammar,concreteSyntax},userProvenance:semanticOptions.provenance??null},
  });
}
