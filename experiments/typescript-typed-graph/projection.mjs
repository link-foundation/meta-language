// JavaScript draft: TypeScript's existing linked annotation rules feed the
// checked editable program graph. Both complete concrete trees remain data
// in that same graph. This is not a new native implementation or a proof of
// complete TypeScript semantics, grammar unions or global concept equivalence.
import {compileGrammar} from '../../js/src/grammar.js';
import {parseGrammarLinks} from '../../js/src/grammar-links.js';
import {nativeGrammarText} from '../../js/src/native-grammar-parser.js';
import {projectTypeScriptTyping} from '../../js/src/typescript-portable-typing.js';
import {createTypedProgramLinks} from '../../js/src/typed-program-links.js';

const parsers=new Map();

function concreteSource(source,grammar) {
  if(!parsers.has(grammar))parsers.set(grammar,compileGrammar(parseGrammarLinks(nativeGrammarText(grammar))));
  const outcome=parsers.get(grammar).parseTree(source);
  if(!outcome.ok)throw new TypeError('typed TypeScript source: '+grammar+' refuses '+outcome.rejection.reason);
  return{tree:outcome.tree,ambiguities:outcome.ambiguities};
}

/** Preserve the actual TypeScript origin alongside its checked lowered program.
 * Existing unsupported/runtime-construct diagnostics propagate; no source
 * envelope or handwritten target semantics substitutes for a checked program.
 * The origin describes import time. Subsequent graph edits change the checked
 * program while retaining that origin; origin data is not an equivalence proof.
 */
export function createTypeScriptSemanticGraph(source,options={}) {
  if(typeof source!=='string'||Buffer.from(source).toString('utf8')!==source){
    throw new TypeError('typed TypeScript source requires exact Unicode scalar text');
  }
  const original=concreteSource(source,'native-typescript');
  const projection=projectTypeScriptTyping(source);
  const lowered=concreteSource(projection.code,'native-javascript');
  return createTypedProgramLinks(projection.code,'JavaScript',{
    ...options,
    provenance:{
      sourceProfile:{language:'TypeScript',model:'typescript-portable-typing-v1',
        concreteSyntax:original,
        lowering:{language:'JavaScript',concreteSyntax:lowered,
          erasures:projection.erasures,annotations:projection.projections}},
      userProvenance:options.provenance??null,
    },
  });
}
