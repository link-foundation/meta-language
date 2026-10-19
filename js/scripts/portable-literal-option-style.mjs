// Recorded spelling correction for generated standard Option<f64> reads.
// Only a primitive literal becomes eager; calls and owned defaults stay lazy.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';

export function readPortableLiteralOptionStyles(source) {
  const {tokens,comments}=tokenize(source,'Rust'),matching=new Map(),depths=[],stack=[];
  for(let at=0;at<tokens.length;at++){
    depths[at]=stack.length;const token=tokens[at];if(token.kind!=='punct')continue;
    if(['(','[','{'].includes(token.value))stack.push(at);
    else if([')',']','}'].includes(token.value)){
      const open=stack.pop();assert.notEqual(open,undefined);
      assert.equal(tokens[open].value,{')':'(',']':'[','}':'{'}[token.value]);matching.set(open,at);matching.set(at,open);
    }
  }
  assert.equal(stack.length,0,'complete generated source delimiters');
  const unchanged=()=>({source,actions:[],proofs:[]});
  // No imported/custom Option, extension trait, macro-generated type or custom
  // prelude can supply a different method behind this spelling correction.
  for(let at=0;at<tokens.length;at++){
    const token=tokens[at];
    if(token.kind==='identifier'&&['use','extern','impl','trait','no_implicit_prelude','prelude_import'].includes(token.value))return unchanged();
    if(token.kind==='identifier'&&['mod','type','struct','enum','union'].includes(token.value)
      &&['Option','Some','None'].includes(tokens[at+1]?.value))return unchanged();
    if(token.kind==='macro'&&!['vec','format','panic','assert'].includes(token.value))return unchanged();
  }
  const modules=[];
  for(let at=0;at<tokens.length;at++)if(depths[at]===0&&tokens[at].value==='mod'
    &&tokens[at+1]?.value==='ml_array'&&tokens[at+2]?.value==='{')modules.push([at,matching.get(at+2)]);
  if(modules.length!==1)return unchanged();
  const [moduleStart,moduleEnd]=modules[0],helpers=[];
  for(let at=moduleStart;at<moduleEnd;at++)if(tokens[at].value==='fn'&&tokens[at+1]?.value==='optional_at'){
    let open=at+2;while(open<moduleEnd&&tokens[open].value!=='{')open++;
    if(open===moduleEnd)return unchanged();
    helpers.push(tokens.slice(tokens[at-1]?.value==='pub'?at-1:at,matching.get(open)+1));
  }
  const expected=tokenize('pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> { index.and_then(|index| values.get(index)).cloned() }','Rust').tokens.filter(token=>token.kind!=='eof');
  if(helpers.length!==1||JSON.stringify(helpers[0].map(token=>[token.kind,token.raw]))!==JSON.stringify(expected.map(token=>[token.kind,token.raw])))return unchanged();
  const functions=[];
  for(let at=0;at<tokens.length;at++)if(depths[at]===0&&tokens[at].value==='fn'){
    let open=at+2;while(open<tokens.length&&tokens[open].value!=='{')open++;
    if(open===tokens.length)return unchanged();
    functions.push({start:tokens[at-1]?.value==='pub'?tokens[at-1].start:tokens[at].start,end:tokens[matching.get(open)].end,open,close:matching.get(open)});
  }
  const actions=[],proofs=[];
  for(const owner of functions){
    if(tokens.slice(owner.open+1,owner.close).some(token=>token.kind==='identifier'
      &&['fn','mod','type','struct','enum','union'].includes(token.value)))continue;
    const edits=[];
    for(let at=owner.open+1;at<owner.close;at++){
      if(tokens[at].value!=='.'||tokens[at+1]?.value!=='unwrap_or_else'||tokens[at+2]?.value!=='(')continue;
      const close=matching.get(at+2);if(tokens[at+3]?.value!=='||')continue;
      let first=at+4,last=close;
      while(tokens[first]?.value==='('&&matching.get(first)===last-1){first++;last--;}
      const negative=tokens[first]?.value==='-';const literal=tokens[first+(negative?1:0)];
      if(last!==first+(negative?2:1)||literal?.kind!=='number'
        ||!/^\d[\d_]*(?:\.[\d_]+)?(?:[eE][+-]?[\d_]+)?f64$/u.test(literal.raw))continue;
      if(tokens[at-1]?.value!==')')continue;
      let left=matching.get(at-1),right=at-1;
      if(tokens[left-1]?.value==='optional_at')left-=5;
      while(tokens[left]?.value==='('&&matching.get(left)===right){left++;right--;}
      if(tokens.slice(left,left+5).map(token=>token.value).join(' ')!=='self :: ml_array :: optional_at'
        ||tokens[left+5]?.value!=='('||matching.get(left+5)!==right)continue;
      if(comments.some(comment=>comment.start>=tokens[left].start&&comment.end<=tokens[close].end))continue;
      const literalSource=source.slice(tokens[first].start,tokens[last-1].end);
      edits.push({start:tokens[at+1].start,end:tokens[close].end,text:'unwrap_or('+literalSource+')',literal:literalSource});
      at=close;
    }
    if(edits.length===0)continue;
    const from=source.slice(owner.start,owner.end);let to=from;
    for(const edit of edits.toReversed())to=to.slice(0,edit.start-owner.start)+edit.text+to.slice(edit.end-owner.start);
    assert.equal(source.split(from).length,2,'the complete source owner occurs once');
    actions.push({op:'replace',field:'source',from,to});
    proofs.push({sourceStart:owner.start,sourceEnd:owner.end,literals:edits.map(edit=>edit.literal),
      basis:'The actual module helper is the complete checked standard Option array read; no custom Option, import, extension, macro type or prelude is admitted. Each fallback is only a typed floating literal, so eager evaluation preserves the same value, signed zero, receiver evaluation and absence of effects.'});
  }
  let current=source;for(const action of actions)current=current.replace(action.from,action.to);
  return{source:current,actions,proofs};
}
