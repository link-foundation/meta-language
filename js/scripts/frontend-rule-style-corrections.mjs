// Complete owning-function styles for the CI117015296223 frontend diagnostics.
// Source decisions stay in frontend-rules.js; these actions select Rust loans
// and equivalent branch, return and missing-value spellings only.
import assert from 'node:assert/strict';
import {readPortableSourceParameterStyles,readPortableSourceLintStyles} from './portable-source-library-style.mjs';
import {simplifyPortableProductLibraryStyles,simplifyRustOptionIdentityMatches} from './reviewed-rust-branch-styles.mjs';
import {readSourceTextRustBlock,simplifySourceTextRustParentheses} from './source-text-byte-library-style.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {readRustLibraryItems} from './rust-library-packaging.mjs';

export const frontendRuleNativeStyleId='reviewed-frontend-native-styles-117015296223';
export const frontendTypeQueryNativeStyleId='reviewed-frontend-type-query-and-optional-tuple-styles';
export const frontendCurrentBooleanNativeStyleId='reviewed-frontend-current-boolean-binding-style';
export const frontendTemporalBindingNativeStyleId='reviewed-frontend-temporal-binding-styles';
export const frontendLexicalDeclarationNativeStyleId='reviewed-frontend-lexical-declaration-styles';

/** Borrow the scalar token columns without changing the generated scan decisions. */
export function readFrontendLexicalDeclarationStyleCorrections(source) {
  const names=['read_java_script_lexical_declaration_step','require_java_script_branch_continuation'];
  const inventory=readRustLibraryItems(source).items;
  assert.ok(!inventory.some(item=>item.kind==='use'||['f64','bool','Option','Vec','String'].includes(item.name)),
    'the generated lexical declaration source retains primitive type ownership');
  return names.flatMap(name=>{
    const selected=inventory.filter(item=>item.kind==='fn'&&item.name===name);
    assert.equal(selected.length,1,'one complete generated scalar decision is retained for '+name);
    const from=selected[0].source;
    const loans=readPortableSourceParameterStyles(from).source;
    const to=readPortableSourceLintStyles(loans).source;
    assert.doesNotMatch(to,/#\[allow\(/u,'a lexical declaration style introduces no lint allowance');
    return from===to?[]:[{op:'replace',field:'source',from,to}];
  });
}

/** Derive exact, bounded contexts from the actual checked source library. */
export function readFrontendRuleStyleCorrections(source) {
  const names=['read_unit_alternative_expression_form','read_lexical_winner_index'];
  const continuations=[...source.matchAll(/^pub fn (ml_read_lexical_winner_index_loop[0-9]+)\(/gmu)].map(match=>match[1]);
  assert.equal(continuations.length,1,'the checked lexical election has its actual single loop owner');
  names.push(continuations[0]);
  const blocks=names.map(name=>readSourceTextRustBlock(source,'fn',name));
  const loans=readPortableSourceParameterStyles(blocks.join('\n\n')).source;
  return names.flatMap((name,index)=>{
    const borrowed=readSourceTextRustBlock(loans,'fn',name);
    const to=simplifyPortableProductLibraryStyles(simplifyRustOptionIdentityMatches(borrowed));
    assert.doesNotMatch(to,/#\[(?:allow|expect)\(/u,'a reviewed frontend style needs no lint suppression');
    return to===blocks[index]?[]:[{op:'replace',field:'source',from:blocks[index],to}];
  });
}

/** Derive current type-query and optional-tuple styles without rewriting older records. */
export function readFrontendTypeQueryStyleCorrections(source) {
  const names=['read_type_query_result','accept_java_script_optional_tuple_context'];
  const blocks=names.map(name=>readSourceTextRustBlock(source,'fn',name));
  const loans=readPortableSourceParameterStyles(blocks.join('\n\n')).source;
  return names.flatMap((name,index)=>{
    const borrowed=readSourceTextRustBlock(loans,'fn',name);
    const to=simplifySourceTextRustParentheses(simplifyPortableProductLibraryStyles(borrowed));
    assert.doesNotMatch(to,/#\[(?:allow|expect)\(/u,'a current frontend style needs no lint suppression');
    return to===blocks[index]?[]:[{op:'replace',field:'source',from:blocks[index],to}];
  });
}

function readFrontendBooleanStyleSpelling(previous,current) {
  const before=tokenize(previous,'Rust'),after=tokenize(current,'Rust');
  const tokens=value=>value.tokens.filter(token=>token.kind!=='eof');
  const left=tokens(before),right=tokens(after);
  if(left.length!==right.length||JSON.stringify(before.comments.map(comment=>comment.text))!==JSON.stringify(after.comments.map(comment=>comment.text)))return null;
  if([...left,...right].some(token=>token.kind==='macro')
    ||[left,right].some(list=>list.filter(token=>token.kind==='identifier'&&token.value==='fn').length!==1))return null;
  const bindings=list=>list.filter((token,index)=>token.kind==='identifier'&&/^ml_logical[0-9]+$/u.test(token.value)
    &&list[index-1]?.value==='let'&&list[index+1]?.value==='=');
  const original=bindings(left),actual=bindings(right);
  if(original.length!==1||actual.length!==1)return null;
  const from=original[0].value,to=actual[0].value;
  for(let at=0;at<left.length;at++){
    if(left[at].kind!==right[at].kind)return null;
    if(left[at].kind==='identifier'&&left[at].value===from){
      if(right[at].value!==to||['.','::'].includes(left[at-1]?.value)
        ||left[at+1]?.value==='::')return null;
    }else if(previous.slice(left[at].start,left[at].end)!==current.slice(right[at].start,right[at].end)
      ||right[at].kind==='identifier'&&right[at].value===to)return null;
  }
  return new Map([[from,to]]);
}

/** Transport one historical whole-function style only across its proved local binder spelling. */
export function readFrontendCurrentBooleanStyleCorrections(source,historical) {
  const name='ml_accept_precedence_operand_context_loop28';
  const candidates=historical.actions.filter(action=>action.op==='replace'&&action.field==='source'
    &&action.from.startsWith('pub fn '+name+'('));
  assert.equal(candidates.length,1,'the original Boolean row retains its single complete continuation action');
  const owners=readRustLibraryItems(source).items.filter(item=>item.kind==='fn'&&item.name===name);
  assert.equal(owners.length,1,'a current Boolean style owns its unique top-level source function');
  const action=candidates[0],from=readSourceTextRustBlock(owners[0].source,'fn',name);
  assert.equal(readSourceTextRustBlock(action.from,'fn',name),action.from);
  assert.equal(readSourceTextRustBlock(action.to,'fn',name),action.to);
  if(readFrontendBooleanStyleSpelling(action.to,from))return [];
  const mapping=readFrontendBooleanStyleSpelling(action.from,from);
  assert.ok(mapping,'a current Boolean style requires the same complete function with one unique bound temporary spelling');
  let to=action.to;
  for(const token of tokenize(to,'Rust').tokens.filter(token=>token.kind==='identifier'&&mapping.has(token.value)).reverse()){
    to=to.slice(0,token.start)+mapping.get(token.value)+to.slice(token.end);
  }
  return [{op:'replace',field:'source',from,to}];
}

function readFrontendTemporalBindingSourcePaths(source,names,local) {
  const {tokens,comments}=tokenize(source,'Rust'),edits=[];
  const prefix=local?['crate','::','translation','::','frontend_rules','::']:['crate','::'];
  for(let at=0;at+prefix.length<tokens.length;at++){
    if(tokens[at].kind!=='identifier'||tokens[at-1]?.value==='::'
      ||!prefix.every((value,offset)=>tokens[at+offset]?.value===value)
      ||!names.has(tokens[at+prefix.length]?.value))continue;
    const end=tokens[at+prefix.length-1].end;
    if(comments.some(comment=>comment.start>=tokens[at].start&&comment.end<=end))continue;
    edits.push([tokens[at].start,end,local?'crate::':'crate::translation::frontend_rules::']);
    at+=prefix.length-1;
  }
  for(const[start,end,text]of edits.reverse())source=source.slice(0,start)+text+source.slice(end);
  return source;
}

function simplifyFrontendTemporalBindingCopies(source) {
  const {tokens,comments}=tokenize(source,'Rust'),matching=new Map(),stack=[];
  for(let at=0;at<tokens.length;at++){
    if(tokens[at].kind!=='punct')continue;
    if(['(','[','{'].includes(tokens[at].value))stack.push(at);
    else if([')',']','}'].includes(tokens[at].value)){const open=stack.pop();matching.set(open,at);matching.set(at,open);}
  }
  const open=tokens.findIndex(token=>token.value==='('),close=matching.get(open),body=tokens.findIndex(token=>token.value==='{');
  if(close===undefined||body<close)return source;
  const parameters=new Set(),counts=new Map();
  for(const part of source.slice(tokens[open].end,tokens[close].start).split(',')){
    const match=/^\s*(?:mut\s+)?([A-Za-z_][A-Za-z_0-9]*)\s*:\s*(f64|bool|Option\s*<\s*(?:f64|bool)\s*>)\s*$/u.exec(part);
    if(match){counts.set(match[1],(counts.get(match[1])??0)+1);parameters.add(match[1]);}
  }
  if(tokens.slice(body+1).some(token=>token.kind==='macro'&&token.value!=='vec'
    ||token.kind==='identifier'&&['fn','use','type','struct','enum','ref'].includes(token.value)
    ||token.kind==='punct'&&token.value==='|'))return source;
  const opaque=new Set();
  for(let at=body+1;at<tokens.length;at++){
    if(tokens[at].kind==='macro'&&matching.has(at+1)){
      const end=matching.get(at+1);for(let inner=at;inner<=end;inner++)opaque.add(inner);at=end;continue;
    }
    if(tokens[at].value==='let'){
      let end=at+1;while(end<tokens.length&&!['=',';'].includes(tokens[end].value))end++;
      for(const token of tokens.slice(at+1,end))if(token.kind==='identifier')parameters.delete(token.value);
    }
    if(tokens[at].value==='=>'){
      let begin=at-1;
      while(begin>body&&!['{','}',','].includes(tokens[begin].value)){
        if(matching.get(begin)<begin)begin=matching.get(begin)-1;else begin--;
      }
      for(const token of tokens.slice(begin+1,at))if(token.kind==='identifier')parameters.delete(token.value);
    }
  }
  const edits=[];
  for(let at=body+1;at+4<tokens.length;at++){
    if(opaque.has(at)||tokens[at].kind!=='identifier'||!parameters.has(tokens[at].value)||counts.get(tokens[at].value)!==1
      ||['.','::'].includes(tokens[at-1]?.value)||tokens[at+1]?.value!=='.'||tokens[at+2]?.value!=='clone'
      ||tokens[at+3]?.value!=='('||tokens[at+4]?.value!==')'
      ||comments.some(comment=>comment.start>=tokens[at+1].start&&comment.end<=tokens[at+4].end))continue;
    edits.push([tokens[at+1].start,tokens[at+4].end]);
  }
  for(const[start,end]of edits.reverse())source=source.slice(0,start)+source.slice(end);
  return source;
}

function simplifyFrontendTemporalBindingArityAttributes(source) {
  const reason='checked source arguments and captured continuation state retain their original order';
  const expected=tokenize('#[expect(clippy::too_many_arguments, reason = "'+reason+'")]', 'Rust').tokens
    .filter(token=>token.kind!=='eof').map(token=>[token.kind,token.value]);
  const {tokens,comments}=tokenize(source,'Rust');
  const declaration=tokens.findIndex((token,index)=>token.value==='pub'&&tokens[index+1]?.value==='fn');
  if(declaration<0)return source;
  for(let at=0;at<declaration;at++){
    if(!expected.every(([kind,value],offset)=>tokens[at+offset]?.kind===kind&&tokens[at+offset]?.value===value))continue;
    const end=at+expected.length;
    const comment=comments.findLast(value=>value.end<=tokens[at].start);
    assert.equal(comment?.text,'// The checked primitive-column interface retains every ordered source argument.',
      'only the actual checked-source arity attribute is regenerated');
    assert.equal(end,declaration,'the checked-source arity attribute immediately precedes its function');
    assert.ok(/^\s*$/u.test(source.slice(comment.end,tokens[at].start))
      &&/^\s*$/u.test(source.slice(tokens[end-1].end,tokens[declaration].start)),
    'an arity style preserves every unrelated prefix byte');
    return source.slice(0,comment.start)+source.slice(tokens[declaration].start);
  }
  return source;
}

/** Adapt only the complete new temporal-binding family using its actual declarations. */
export function readFrontendTemporalBindingStyleCorrections(source) {
  const inventory=readRustLibraryItems(source).items;
  assert.ok(!inventory.some(item=>item.kind==='use'||['f64','bool','Option','Vec','String'].includes(item.name)),
    'the generated temporal-binding source retains its actual primitive type ownership');
  const selected=inventory.filter(item=>item.kind==='fn'&&/^(?:ml_)?read_java_script_temporal_binding_read(?:_loop[0-9]+)?$/u.test(item.name)
    ||item.kind==='enum'&&/^MlReadJavaScriptTemporalBindingReadLoop[0-9]+Result$/u.test(item.name));
  assert.equal(selected.filter(item=>item.kind==='fn').length,4,'the temporal-binding source owns its entry and three actual continuations');
  assert.equal(selected.filter(item=>item.kind==='enum').length,3,'the temporal-binding proof retains every actual result enum');
  const names=new Set([...selected.map(item=>item.name),'ml_array']);
  assert.equal(names.size,8,'the temporal-binding family has no shadowed source declarations');
  for(const item of selected.filter(item=>item.kind==='fn'&&item.name.startsWith('ml_'))){
    const number=/loop([0-9]+)$/u.exec(item.name)?.[1];
    assert.ok(names.has('MlReadJavaScriptTemporalBindingReadLoop'+number+'Result'),
      'each actual temporal-binding continuation retains its own result type');
  }
  const local=readFrontendTemporalBindingSourcePaths(selected.map(item=>item.kind==='fn'
    ?simplifyFrontendTemporalBindingArityAttributes(item.source):item.source).join(''),names,true);
  const loans=readPortableSourceParameterStyles(local).source;
  let prepared=loans;
  for(const item of readRustLibraryItems(loans).items.filter(item=>item.kind==='fn')){
    const copied=simplifyFrontendTemporalBindingCopies(item.source);
    const to=simplifyRustOptionIdentityMatches(copied);
    if(to!==item.source)prepared=prepared.replace(item.source,to);
  }
  const reviewed=readPortableSourceLintStyles(prepared).source;
  const restored=readFrontendTemporalBindingSourcePaths(reviewed,names,false);
  const targets=new Map(readRustLibraryItems(restored).items.map(item=>[item.name,item]));
  return selected.flatMap(item=>{
    const to=targets.get(item.name);
    assert.equal(to?.kind,item.kind,'a temporal-binding style retains its complete declaration inventory');
    assert.doesNotMatch(to.source,/#\[allow\(/u,'a temporal-binding style introduces no lint allowance');
    return item.source===to.source?[]:[{op:'replace',field:'source',from:item.source,to:to.source}];
  });
}
