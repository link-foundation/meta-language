// Derive complete declaration styles from the actual checked identity helpers.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {DecoratorSet} from '../src/decorators.js';
import {readBinaryMetadataCandidateStyles} from './binary-metadata-candidate-styles.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

export function readGlobalConceptIdentityStyles(source) {
  const array=readSourceTextRustBlock(source,'mod','ml_array');
  const provider=readSourceTextRustBlock(readFileSync(new URL('../../rust/src/global_concept_rules.rs',import.meta.url),'utf8'),'mod','ml_array');
  assert.equal(array,provider,'the entire shared array provider has the same operations and representation');
  const names=[...source.matchAll(/^pub fn ((?:contains_global_concept_identity|accept_global_concept_history|select_global_concept_identity|ml_contains_global_concept_identity_loop\d+|ml_select_global_concept_identity_loop\d+))\(/gmu)].map(match=>match[1]);
  assert.equal(names.length,5,'all complete identity decisions and continuations remain present');
  const items=names.map(name=>readSourceTextRustBlock(source,'fn',name));
  const base=items.map(item=>{
    let adapted=item;
    // The checked operations only read these arrays. Native continuations
    // borrow those exact values instead of copying the entire history.
    for(const name of ['identities','aliases','reserved','assigned']) {
      adapted=adapted.replace(new RegExp('(?:mut )?'+name+': Vec<String>','gu'),name+': &[String]')
        .replaceAll(name+'.clone()',name).replaceAll('&'+name+',',name+',');
    }
    adapted=adapted.replaceAll('String::from("")','String::new()').replaceAll('preferred != ""','!preferred.is_empty()');
    for(const match of adapted.matchAll(/\blet (ml_s\d+|ml_result\d+|candidate_\d+) =/gu)) {
      const name=match[1];
      if([...adapted.matchAll(new RegExp('\\b'+name+'\\b','gu'))].length===2) adapted=adapted.replaceAll(name+'.clone()',name);
    }
    // A loop can consume the same invariant on several iterations even when
    // its spelling occurs only once in the body. Only an acyclic wrapper
    // gives this complete declaration a single final parameter use.
    for(const match of /\b(?:loop|while|for)\b/u.test(adapted)?[]:adapted.matchAll(/\b([a-z_][a-z_0-9]*): String\b/gu)) {
      const name=match[1];
      if([...adapted.matchAll(new RegExp('\\b'+name+'\\b','gu'))].length===2) adapted=adapted.replaceAll(name+'.clone()',name);
    }
    return formatProductRuleSource(adapted);
  });
  const adapter={id:'global-concept-identity-borrow-style',level:'emitter',order:0,when:[['format','Rust']],actions:[{op:'set',field:'checked',value:'true'}]};
  const result=readBinaryMetadataCandidateStyles({items,base,previousItems:[],previousStyled:[],adapter,format:formatProductRuleSource});
  const derived=new DecoratorSet(result.decorators);
  const actions=[{op:'replace',field:'source',from:array,to:'pub use crate::global_concept_rules::ml_array;'},...items.map((from,index)=>({op:'replace',field:'source',from,
    to:'/// Execute the checked shared global concept identity decision.\n'+derived.decorate('emitter',{format:'Rust',scope:'formatted-item',source:base[index]}).source
      .replace('return { candidate.clone() };','return candidate;')
      .replace('} else {\n            preferred.clone()\n','} else {\n            preferred\n').trimEnd()}))];
  return new DecoratorSet([{id:'global-concept-identity-library-style',level:'emitter',order:10000,
    when:[['format','Rust'],['scope','formatted-module']],actions}]);
}
