// Derive complete declaration styles from the actual checked identity helpers.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {DecoratorSet} from '../src/decorators.js';
import {readBinaryMetadataCandidateStyles} from './binary-metadata-candidate-styles.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {simplifyRustBranches} from './reviewed-rust-branch-styles.mjs';
import {readCompilerFeedbackCorrections} from './compiler-style-feedback.mjs';
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

// Replay the real non-overlapping compiler transaction, then use the existing
// reviewed branch styles for the deferred nested alternative. Its applicability
// metadata remains unchanged; the general feedback reader still defers overlap.
export function readGlobalConceptIdentityCompilerStyles(source,feedback) {
  const path='rust/src/global_concept_identity_rules.rs';
  const inputs=Object.values(feedback.sources).filter(input=>input.relativePath===path);
  assert.equal(inputs.length,1,'identity feedback retains one exact complete source');
  assert.equal(source,inputs[0].text,'identity feedback matches the complete fresh generated module');
  const corrections=readCompilerFeedbackCorrections({feedback,allowedPaths:[path]});
  assert.ok(corrections.actions.length>0,'actual compiler feedback supplies its machine transaction');
  const machine=new DecoratorSet([{id:'identity-machine-feedback',level:'emitter',order:0,
    when:[['format','Rust'],['scope','formatted-module']],actions:corrections.actions}]);
  const corrected=machine.decorate('emitter',{format:'Rust',scope:'formatted-module',source}).source;
  const names=[...source.matchAll(/^pub fn (\w+)\(/gmu)].map(match=>match[1]);
  const actions=[];
  for(const name of names){
    const from=readSourceTextRustBlock(source,'fn',name);
    const actual=readSourceTextRustBlock(corrected,'fn',name);
    const to=formatProductRuleSource(simplifyRustBranches(actual)).trimEnd();
    if(from!==to)actions.push({op:'replace',field:'source',from,to});
  }
  assert.ok(actions.length>0,'compiler and reviewed branch corrections change an entire owning function');
  return new DecoratorSet([{id:'global-concept-identity-compiler-style',level:'emitter',order:20000,
    when:[['format','Rust'],['scope','formatted-module']],actions}]);
}
