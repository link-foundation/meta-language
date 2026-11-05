// Reviewed rendered diagnostics, not machine-applicable compiler suggestions:
// https://github.com/link-foundation/meta-language/actions/runs/39890308543/job/120227286631
// Complete emitted operations remain in the checked JavaScript program. These
// mechanical corrections become ordinary family Links decorators.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

const families=new Set(['ascii_text_predicate_rules','declaration_dependency_rules','external_emission_rules','inert_module_binding_rules','javascript_record_parameter_rules','reference_identity_rules','rust_literal_match_rules','rust_shared_array_handle_rules']);
function readNativeCompilerStyleSyntax(source){
 const {tokens,comments}=tokenize(source,'Rust'),matching=new Map(),stack=[];
 for(let at=0;at<tokens.length;at++)if(tokens[at].kind==='punct'){
  if(['(','[','{'].includes(tokens[at].value))stack.push(at);
  else if([')',']','}'].includes(tokens[at].value)){const open=stack.pop();assert.notEqual(open,undefined);assert.equal(tokens[open].value,({')':'(',']':'[','}':'{'})[tokens[at].value]);matching.set(open,at);matching.set(at,open);}
 }
 assert.equal(stack.length,0);return{tokens,comments,matching};
}
function applyNativeCompilerStyleEdits(source,changes){
 let end=source.length;
 for(const[start,finish,value]of changes.sort((a,b)=>b[0]-a[0])){assert.ok(start>=0&&finish>=start&&finish<=end);source=source.slice(0,start)+value+source.slice(finish);end=start;}
 return source;
}
function renameNativeCompilerStyleBindings(source,names){
 const {tokens}=readNativeCompilerStyleSyntax(source),all=new Set(tokens.filter(token=>token.kind==='identifier').map(token=>token.value));
 for(const[before,after]of Object.entries(names)){assert.ok(all.has(before),'reviewed binding is present: '+before);assert.ok(!all.has(after),'a reviewed rename cannot capture '+after);}
 const changes=[];
 for(let at=0;at<tokens.length;at++){
  const token=tokens[at];if(token.kind!=='identifier'||!Object.hasOwn(names,token.value))continue;
  assert.ok(!['.','::'].includes(tokens[at-1]?.value)&&tokens[at+1]?.value!=='::','rename actual value occurrences only');
  assert.ok(!tokens.some(value=>value.kind==='string'&&value.value.includes('{'+token.value)),'implicit format capture needs its own proof');
  changes.push([token.start,token.end,names[token.value]]);
 }
 return applyNativeCompilerStyleEdits(source,changes);
}
// Repeating every numeric digit is injective, and distinct suffixes differ by
// at least two edits. Every actual occurrence is renamed together; strings,
// comments and function names retain their original bytes.
function distinguishNativeCompilerStyleBranches(source){
 const {tokens}=readNativeCompilerStyleSyntax(source),changes=[];
 for(let at=0;at<tokens.length;at++){
  const token=tokens[at],match=token.kind==='identifier'&&/^ml_s([0-9]+)$/u.exec(token.value);if(!match)continue;
  assert.ok(!['.','::','fn'].includes(tokens[at-1]?.value)&&tokens[at+1]?.value!=='::');
  assert.ok(!tokens.some(value=>value.kind==='string'&&value.value.includes('{'+token.value)));
  changes.push([token.start,token.end,'ml_s'+[...match[1]].map(digit=>digit+digit).join('')]);
 }
 return applyNativeCompilerStyleEdits(source,changes);
}
function readNativeCompilerStyleConditional(tokens,matching,at){
 assert.equal(tokens[at]?.value,'if');let open=at+1;
 while(open<tokens.length&&tokens[open].value!=='{'){
  if(tokens[open].value==='if'){const nested=readNativeCompilerStyleConditional(tokens,matching,open);open=nested.end+1;}
  else if(['(','['].includes(tokens[open].value))open=matching.get(open)+1;else open++;
 }
 assert.equal(tokens[open]?.value,'{');const close=matching.get(open);let end=close,alternative;
 if(tokens[close+1]?.value==='else'){
  if(tokens[close+2]?.value==='if'){alternative=readNativeCompilerStyleConditional(tokens,matching,close+2);end=alternative.end;}
  else{assert.equal(tokens[close+2]?.value,'{');alternative={open:close+2,close:matching.get(close+2)};end=alternative.close;}
 }
 return{at,open,close,alternative,end};
}
function mergeNativeCompilerStyleEmptyBranches(source){
 const {tokens,matching,comments}=readNativeCompilerStyleSyntax(source),at=tokens.findIndex(token=>token.value==='if'),first=readNativeCompilerStyleConditional(tokens,matching,at),second=first.alternative;
 assert.ok(second&&second.at!==undefined);const key=(open,close)=>tokens.slice(open+1,close).map(token=>[token.kind,token.value]);
 assert.deepEqual(key(first.open,first.close),key(second.open,second.close));
 assert.deepEqual(tokens.slice(first.open+1,first.close).map(token=>token.value),['Vec','::','<','f64','>','::','new','(',')']);
 assert.ok(!comments.some(comment=>comment.start>=tokens[first.at].start&&comment.end<=tokens[second.close].end));
 const condition=node=>source.slice(tokens[node.at+1].start,tokens[node.open].start).trim();
 const block=source.slice(tokens[first.open].start,tokens[first.close].end);
 return applyNativeCompilerStyleEdits(source,[[tokens[first.at].start,tokens[second.close].end,'if ('+condition(first)+') || ('+condition(second)+') '+block]]);
}
function removeNativeCompilerStyleContinuingElse(source){
 const {tokens,matching}=readNativeCompilerStyleSyntax(source),candidates=[];
 for(let at=0;at<tokens.length;at++)if(tokens[at].value==='if'){
  const node=readNativeCompilerStyleConditional(tokens,matching,at);if(!node.alternative||node.alternative.at!==undefined)continue;
  const body=tokens.slice(node.open+1,node.close),jump=body.findIndex(token=>token.value==='continue');
  if(jump<0||body.some(token=>['if','match','loop','fn','return','break'].includes(token.value)))continue;
  if(!body.slice(jump+1).every(token=>[';','}'].includes(token.value)))continue;
  candidates.push([tokens[node.close+1].start,tokens[node.close+1].end,'']);
 }
 assert.equal(candidates.length,1,'one reviewed branch unconditionally continues');return applyNativeCompilerStyleEdits(source,candidates);
}
function hoistNativeCompilerStyleCommonRefusal(source){
 const {tokens,matching,comments}=readNativeCompilerStyleSyntax(source),candidates=[];
 for(let at=0;at<tokens.length;at++)if(tokens[at].value==='if'){
  const node=readNativeCompilerStyleConditional(tokens,matching,at),other=node.alternative;if(!other||other.at!==undefined)continue;
  const tails=[node,other].map(branch=>tokens[branch.close-1]?.value==='}'?matching.get(branch.close-1):undefined);
  if(tails.some(value=>value===undefined))continue;
  const left=source.slice(tokens[tails[0]].start,tokens[node.close-1].end),right=source.slice(tokens[tails[1]].start,tokens[other.close-1].end);
  if(JSON.stringify(readNativeCompilerStyleSyntax(left).tokens.map(token=>[token.kind,token.value]))!==JSON.stringify(readNativeCompilerStyleSyntax(right).tokens.map(token=>[token.kind,token.value])))continue;
  if(!/^\{\s*self::[A-Za-z_0-9]+::[A-Za-z_0-9]+\(Vec::<f64>::new\(\)\)\s*\}$/u.test(left))continue;
  if(comments.some(comment=>comment.start>=tokens[node.at].start&&comment.end<=tokens[other.close].end))continue;
  // Only a conditional precedes each common refusal; no outer binding moves
  // across its lifetime. Nested successful paths continue in their own scopes.
  if([node,other].some((branch,index)=>tokens[branch.open+1]?.value!=='if'||readNativeCompilerStyleConditional(tokens,matching,branch.open+1).end!==tails[index]-1))continue;
  candidates.push({node,other,tails,left});
 }
 assert.equal(candidates.length,1,'one reviewed common refusal tail');const {node,other,tails,left}=candidates[0];
 const changed=applyNativeCompilerStyleEdits(source,[[tokens[tails[0]].start,tokens[node.close-1].end,''],[tokens[tails[1]].start,tokens[other.close-1].end,'']]);
 const removed=(tokens[node.close-1].end-tokens[tails[0]].start)+(tokens[other.close-1].end-tokens[tails[1]].start);
 const end=tokens[other.close].end-removed;return changed.slice(0,end)+'\n'+left+changed.slice(end);
}
function normalizeNativeCompilerStylePublicationPredicate(source){
 const {tokens,matching}=readNativeCompilerStyleSyntax(source),at=tokens.findIndex(token=>token.value==='if'),node=readNativeCompilerStyleConditional(tokens,matching,at);
 const negate=(start,end)=>{
  while(tokens[start]?.value==='('&&matching.get(start)===end-1){start++;end--;}
  for(const operation of ['||','&&']){
   const cuts=[];for(let index=start;index<end;index++){if(matching.get(index)>index){index=matching.get(index);continue;}if(tokens[index].value===operation)cuts.push(index);}
   if(cuts.length){const ranges=[];let begin=start;for(const cut of cuts){ranges.push([begin,cut]);begin=cut+1;}ranges.push([begin,end]);return ranges.map(([from,to])=>'('+negate(from,to)+')').join(operation==='||'?' && ':' || ');}
  }
  if(tokens[start]?.value==='!')return source.slice(tokens[start+1].start,tokens[end-1].end);
  for(let index=start;index<end;index++){
   if(matching.get(index)>index){index=matching.get(index);continue;}
   if(tokens[index].value==='!=')return source.slice(tokens[start].start,tokens[index].start)+'=='+source.slice(tokens[index].end,tokens[end-1].end);
  }
  return '!('+source.slice(tokens[start].start,tokens[end-1].end)+')';
 };
 return applyNativeCompilerStyleEdits(source,[[tokens[at+1].start,tokens[node.open].start,'!('+negate(at+1,node.open)+') ']]);
}
function mapNativeCompilerStyleOptionalType(source){
 const pattern=/match\s+(self::ml_array::optional_at\([\s\S]*?\))\s*\{\s*Some\(ml_present\)\s*=>\s*\{\s*let _ = ml_present;\s*String::from\("string"\)\s*\}\s*,?\s*None\s*=>\s*String::from\("undefined"\),?\s*\}/gu;
 const matches=[...source.matchAll(pattern)];assert.equal(matches.length,1,'one reviewed optional type observation');
 return source.replace(pattern,(_,value)=>'('+value+').map_or_else(|| String::from("undefined"), |ml_present| {let _ = ml_present; String::from("string")})');
}
function normalizeNativeCompilerStyleTextOperations(source){
 const {tokens}=readNativeCompilerStyleSyntax(source),changes=[];
 for(let at=0;at<tokens.length;at++){
  if(tokens[at].value==='.'&&tokens[at+1]?.value==='contains'&&tokens[at+2]?.value==='('&&tokens[at+3]?.kind==='string'&&tokens[at+3].value==='\\'&&tokens[at+4]?.value===')')changes.push([tokens[at+3].start,tokens[at+3].end,"'\\\\'"]);
  if(tokens[at].value==='.'&&tokens.slice(at,at+12).map(token=>token.value).join(' ')==='. clone ( ) . unwrap_or_else ( || f64 :: NAN )'){
   const prefix=source.slice(Math.max(0,tokens[at].start-160),tokens[at].start);
   assert.match(prefix,/code_point_at\(0f64\)\)*\s*$/u,'the runtime text operation returns Option<f64>');
   changes.push([tokens[at].start,tokens[at+11].end,'.unwrap_or(f64::NAN)']);
  }
 }
 return applyNativeCompilerStyleEdits(source,changes);
}

export function readNativeCompilerSourceStyles(source,family){
 if(!families.has(family))return{source,actions:[]};
 let current=source;const actions=[];
 const change=(before,after)=>{if(before===after)return;assert.equal(current.split(before).length,2);actions.push({op:'replace',field:'source',from:before,to:after});current=current.replace(before,()=>after);};
 const functionStyle=(name,apply)=>{const before=readSourceTextRustBlock(current,'fn',name);change(before,apply(before));};
 if(family==='declaration_dependency_rules'){
  functionStyle('ml_read_declaration_dependency_components_loop12',body=>renameNativeCompilerStyleBindings(body,{leaders_2:'component_representatives'}));
  functionStyle('ml_read_recursive_declaration_source_owners_loop22',body=>renameNativeCompilerStyleBindings(body,{owners_2:'collected_source_owners'}));
  functionStyle('accept_recursive_declaration_owned_publication',normalizeNativeCompilerStylePublicationPredicate);
  functionStyle('ml_read_declaration_deferred_binding_loop61',hoistNativeCompilerStyleCommonRefusal);
 }
 if(family==='inert_module_binding_rules')functionStyle('read_inert_module_binding',body=>renameNativeCompilerStyleBindings(body,{values_2:'accepted_literal_values'}));
 if(family==='javascript_record_parameter_rules')functionStyle('ml_find_java_script_record_parameter_strict_directive_loop15',body=>renameNativeCompilerStyleBindings(body,{raw_:'directive_source_text'}));
 if(family==='ascii_text_predicate_rules'){
  functionStyle('plan_rust_ascii_character_callback',mergeNativeCompilerStyleEmptyBranches);
  functionStyle('ml_plan_rust_ascii_character_callback_loop6',removeNativeCompilerStyleContinuingElse);
 }
 if(family==='rust_literal_match_rules')functionStyle('ml_plan_rust_literal_match_pattern_loop1',mapNativeCompilerStyleOptionalType);
 const booleanInterfaces=family==='reference_identity_rules'?['read_reference_identity_comparison_error']:family==='rust_shared_array_handle_rules'?['read_reference_identity_comparison_error','read_rust_shared_array_handle_source_form']:[];
 for(const name of booleanInterfaces)functionStyle(name,body=>{
  const header=/^pub fn \w+\(([^)]*)\)/u.exec(body);assert.ok(header);assert.ok([...header[1].matchAll(/:\s*bool\b/gu)].length>3);
  return '#[expect(clippy::fn_params_excessive_bools, reason = "independent checked source facts retain their ordered Boolean interface")]\n'+body;
 });
 for(const [,name]of [...current.matchAll(/^pub fn (\w+)\(/gmu)])functionStyle(name,body=>normalizeNativeCompilerStyleTextOperations(distinguishNativeCompilerStyleBranches(body)));
 return{source:current,actions};
}

/** A one-character String pattern and char pattern select the same Rust search.
 * Applied only to the Number-literal classifier's generated loop. */
export function readNativeNumericPatternSourceStyles(source,family){
 if(family!=='inert_module_binding_rules')return{source,actions:[]};
 const names=[...source.matchAll(/^pub fn ((?:ml_)?(?:read|accept)_numeric_module_binding_[a-z_0-9]+)\(/gmu)].map(match=>match[1]);
 const actions=[];let current=source;
 for(const name of names){
  const before=readSourceTextRustBlock(current,'fn',name),after=before.replaceAll('.contains("n")',".contains('n')");
  if(before===after)continue;
  assert.equal(current.split(before).length,2);actions.push({op:'replace',field:'source',from:before,to:after});current=current.replace(before,()=>after);
 }
 return{source:current,actions};
}
