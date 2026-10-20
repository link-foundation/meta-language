// Shared recorded styles over the complete checked integer-library output.
import {readPortableLiteralOptionStyles} from './portable-literal-option-style.mjs';
import {readPortableOwnedSourceStyles} from './portable-owned-source-style.mjs';
import {removeFamilyLibraryDivergentElse} from './portable-conditional-source-style.mjs';

export function readPortableIntegerLibraryStyles(source) {
  const literal=readPortableLiteralOptionStyles(source);
  const names=[...literal.source.matchAll(/^pub fn (\w+)\(/gmu)].map(row=>row[1]);
  const owned=readPortableOwnedSourceStyles(literal.source,{scopedCallBorrows:true,functionNames:names});
  const current=removeFamilyLibraryDivergentElse(owned.source,{divergentMatches:true});
  const actions=[...literal.actions,...owned.actions];
  if(current!==owned.source)actions.push({op:'replace',field:'source',from:owned.source,to:current});
  return{source:current,actions};
}
