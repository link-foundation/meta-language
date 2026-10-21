// Explicit private experiment only. The actual repository/frozen sources stay
// unchanged and these controlled loaded bytes are never ledger observations.
import {registerHooks} from 'node:module';
import {readGlobalGrammarContextProposal} from './proposal.mjs';
const files=['js/src/grammar-runtime/grammar-selection.js','js/tests/global-concept-archive-parser.test.js'];
const proposals=new Map(files.map(file=>[new URL('../../'+file,import.meta.url).href,readGlobalGrammarContextProposal(file)]));
registerHooks({load(url,context,nextLoad){
  if(proposals.has(url))return{format:'module',shortCircuit:true,source:proposals.get(url).source};
  return nextLoad(url,context);
}});
