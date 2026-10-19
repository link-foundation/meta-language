// Candidate-only library spelling. Historical exact contexts are validated by
// the caller against the complete frozen prior source before this operation.
import assert from 'node:assert/strict';
import {DecoratorSet} from '../src/decorators.js';
import {tokenize} from '../src/translation/lexer.js';
import {readFileSync} from 'node:fs';
import {readProductRuleStyleCorrections} from './product-rule-style-corrections.mjs';
import {readSourceTextRustBlock,simplifySourceTextRustParentheses} from './source-text-byte-library-style.mjs';

function syntax(source){
 const tokens=tokenize(source,'Rust').tokens.filter(token=>token.kind!=='eof');
 const bindings=new Set(),matching=new Map(),stack=[];
 for(const[index,token]of tokens.entries()){
  if(token.kind!=='punct')continue;
  if(['(','[','{'].includes(token.value))stack.push(index);
  else if([')',']','}'].includes(token.value)){const start=stack.pop();assert.equal(tokens[start]?.value,({')':'(',']':'[','}':'{'})[token.value],'generated candidate delimiter kinds remain balanced');matching.set(start,index);matching.set(index,start);}
 }
 assert.equal(stack.length,0,'generated candidate delimiters remain balanced');
 const declaration=tokens.findIndex(token=>token.kind==='identifier'&&['fn','enum','struct','mod'].includes(token.value));
 if(declaration>=0&&tokens[declaration].value==='fn'){
  const open=declaration+2,close=matching.get(open);
  assert.equal(tokens[open].value,'(');
  for(let index=open+1;index<close;index++)if(tokens[index].kind==='identifier'&&tokens[index+1]?.value===':')bindings.add(tokens[index].value);
 }
 for(let index=0;index<tokens.length;index++){
  if(tokens[index].kind==='identifier'&&tokens[index].value==='let'){
   let at=index+1;if(tokens[at]?.value==='mut')at++;
   if(tokens[at]?.kind==='identifier')bindings.add(tokens[at].value);
   else if(tokens[at]?.value==='(')for(let end=matching.get(at),part=at+1;part<end;part++)if(tokens[part].kind==='identifier'&&tokens[part].value!=='mut')bindings.add(tokens[part].value);
  }
  if(/^ml_(?:s|result|values)\d+$/u.test(tokens[index].value))bindings.add(tokens[index].value);
 }
 return{source,tokens,bindings,matching};
}
const generated=name=>/^(?:ml_(?:transcode_link_network_binary|read_link_network_binary_integer|accept_link_network_binary_unicode)_loop\d+|Ml(?:TranscodeLinkNetworkBinary|ReadLinkNetworkBinaryInteger|AcceptLinkNetworkBinaryUnicode)\w+)$/u.test(name);
function alphaMapping(previous,current){
 if(previous.tokens.length!==current.tokens.length)return null;
 const forward=new Map(),backward=new Map();
 for(let index=0;index<previous.tokens.length;index++){
  const left=previous.tokens[index],right=current.tokens[index];
  if(left.kind!==right.kind)return null;
  const variable=left.kind==='identifier'&&(previous.bindings.has(left.value)&&current.bindings.has(right.value)||generated(left.value)&&generated(right.value));
  if(variable){
   if(forward.has(left.value)&&forward.get(left.value)!==right.value||backward.has(right.value)&&backward.get(right.value)!==left.value)return null;
   forward.set(left.value,right.value);backward.set(right.value,left.value);
  }else if(previous.source.slice(left.start,left.end)!==current.source.slice(right.start,right.end))return null;
 }
 // Renaming is applied to identifier tokens. A spelling used both as a local
 // binder and as a fixed path/field/type name cannot be changed through only
 // its binder occurrence; every written occurrence must match the same map.
 for(let index=0;index<previous.tokens.length;index++)if(previous.tokens[index].kind==='identifier'&&forward.has(previous.tokens[index].value)
  &&forward.get(previous.tokens[index].value)!==current.tokens[index].value)return null;
 return forward;
}
function rename(source,names){
 const edits=tokenize(source,'Rust').tokens.filter(token=>token.kind==='identifier'&&names.has(token.value)&&names.get(token.value)!==token.value);
 for(const token of edits.reverse())source=source.slice(0,token.start)+names.get(token.value)+source.slice(token.end);
 return source;
}
function borrowHeader(source,original,signatures){
 if(!original.includes('let header = vec!'))return source;
 const{tokens,matching}=syntax(source);
 const edits=tokens.filter((token,index)=>{
  if(token.value!=='header'||!['(',','].includes(tokens[index-1]?.value)||![',',')'].includes(tokens[index+1]?.value))return false;
  let open=index-1;
  while(open>=0&&!(tokens[open].value==='('&&matching.get(open)>index))open--;
  assert.ok(tokens[open-2]?.value==='::'&&tokens[open-3]?.value==='super'&&!['::','.'].includes(tokens[open-4]?.value),
   'a constructed header may enter only the actual generated sibling call path');
  const callee=tokens[open-1]?.value;let argument=0;
  for(let at=open+1;at<index;at++){if(['(','[','{'].includes(tokens[at].value)&&tokens[at].kind==='punct')at=matching.get(at);else if(tokens[at].value===',')argument++;}
  assert.equal(signatures.get(callee)?.[argument]?.replace(/\s/gu,''),'header:&[f64]','a constructed header is borrowed only by an actual checked slice parameter');
  return true;
 });
 for(const token of edits.reverse())source=source.slice(0,token.start)+'&header'+source.slice(token.end);
 assert.ok(!/\bheader\.clone\(/u.test(source),'the constructed immutable header remains borrowed');
 return source;
}
function freshSpelling(source,provider){
 const array=readSourceTextRustBlock(provider,'mod','ml_array');
 const current=source.replace(/^#\[must_use\]\n/gmu,'');
 const{tokens,matching}=syntax(current);
 // The reused branch utility can complement a complete unary condition.
 // A leading ! on the first operand of a compound condition is different;
 // retain its enclosing grouping, and refuse an ungrouped ambiguous input.
 for(let index=0;index<tokens.length;index++)if(tokens[index].kind==='identifier'&&tokens[index].value==='if'&&tokens[index+1]?.value==='!'){
  const operand=index+2,end=tokens[operand]?.value==='('?matching.get(operand):operand;
  assert.ok(tokens[operand]?.value==='('||tokens[operand]?.kind==='identifier','a leading unary condition needs an actual owned operand');
  assert.equal(tokens[end+1]?.value,'{','a leading unary condition must own the complete condition before branch reversal');
 }
 const library=array+'\n\n'+current;
 const actions=readProductRuleStyleCorrections(library,'source',provider);
 const corrected=new DecoratorSet([{id:'binary-metadata-structural-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}])
  .decorate('emitter',{format:'Rust',source:library}).source;
 const prefix='pub use crate::translation::frontend_rules::ml_array;';
 assert.ok(corrected.startsWith(prefix),'the actual shared array provider is retained');
 const owning=corrected.slice(prefix.length).trimStart(),postfix=syntax(owning);
 for(let index=0;index<postfix.tokens.length;index++)if(postfix.tokens[index].kind==='punct'&&postfix.tokens[index].value==='('){
  const end=postfix.matching.get(index);
  if(!['.','[','('].includes(postfix.tokens[end+1]?.value))continue;
  const operand=postfix.tokens.slice(index+1,end).filter(token=>token.kind!=='punct'||!['(',')'].includes(token.value));
  assert.ok(!(operand.length===2&&operand[0].value==='-'&&operand[1].kind==='number'),'a negative literal receiver must retain its original unary grouping');
 }
 return simplifySourceTextRustParentheses(owning);
}
function declaration(source){
 return /^pub (?:const )?(?:fn|mod|struct|enum) (\w+)/mu.exec(source)?.[1]
  ?? /^pub use [^;]*::(\w+);/mu.exec(source)?.[1];
}

/** Derive exact linked styles from actual emitted declarations, never a saved candidate decision. */
export function readBinaryMetadataCandidateStyles({items,base,previousItems,previousStyled,adapter,format}){
 assert.equal(items.length,base.length);assert.equal(previousItems.length,previousStyled.length);
 const previous=previousItems.map(syntax),provider=readFileSync(new URL('../../rust/src/translation/frontend_rules.rs',import.meta.url),'utf8');
 const actions=[],declarations=[],signatures=new Map(base.flatMap(item=>{
  const match=/^pub (?:const )?fn (\w+)\(([^)]*)\)/mu.exec(item);return match?[[match[1],match[2].split(',')]]:[];
 }));
 for(let index=0;index<items.length;index++){
  const current=syntax(items[index]),name=declaration(items[index]);
  const equivalent=[];
  for(let prior=0;prior<previous.length;prior++){
   const mapping=alphaMapping(previous[prior],current);
   if(mapping){
    const styled=syntax(previousStyled[prior]);
    const introduced=[...styled.bindings].filter(name=>!previous[prior].bindings.has(name));
    if(introduced.some(name=>current.bindings.has(name)))continue;
    equivalent.push({prior,source:format(rename(previousStyled[prior],mapping))});
   }
  }
  const distinct=[...new Set(equivalent.map(row=>row.source))];
  let corrected,method;
  if(distinct.length===1){corrected=distinct[0];method=items[index]===previousItems[equivalent[0].prior]?'exact frozen declaration':'bijective bound-name spelling';}
  else{
   corrected=/^pub (?:const )?fn /mu.test(base[index])?freshSpelling(base[index],provider):base[index];
   method=distinct.length?'fresh structural style after ambiguous spelling':'fresh structural style for changed declaration';
  }
  corrected=format(borrowHeader(corrected,items[index],signatures));
  assert.equal(declaration(corrected),name,'every actual declaration retains its original position and symbol');
  if(corrected!==base[index])actions.push({op:'replace',field:'source',from:base[index],to:corrected});
  declarations.push({name,method});
 }
 return{decorators:[adapter,{id:'binary-metadata-candidate-library-style',level:'emitter',order:900,when:[['format','Rust'],['scope','formatted-item']],actions}],
  receipt:{schemaVersion:1,declarations,recordedCandidateContexts:actions.length}};
}
