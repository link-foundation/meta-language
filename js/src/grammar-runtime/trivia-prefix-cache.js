// A named syntax extra first skips anonymous extras. Refuse a prediction
// unless all of those starts are known and scanner expectations are absent.
export function readTriviaPrefixPolicy(program) {
  if(program.expectedReferences.size||program.expectedTerminals.size)return null;
  const padding=new Set(),named=new Map();
  for(const item of program.trivia){
    if(item.kind!==null)continue;
    const expression=item.expression;
    if(expression.kind==='literal'&&expression.value.length>0&&expression.value.charCodeAt(0)<128){
      padding.add(expression.value.charCodeAt(0));
    }else if(expression.kind==='charClass'&&!expression.negated
      &&expression.items.every(part=>part.kind==='char'&&part.value.length===1&&part.value.charCodeAt(0)<128)){
      for(const part of expression.items)padding.add(part.value.charCodeAt(0));
    }else return null;
  }
  for(const item of program.trivia){
    if(item.kind===null)continue;
    const rule=program.rules.get(item.kind);
    if(rule?.kind!=='normal')continue;
    const expression=rule.expression.kind==='seq'?rule.expression.items[0]:rule.expression;
    if(expression?.kind==='literal'&&expression.value.length>0&&expression.value.charCodeAt(0)<128){
      named.set(item.kind,expression.value.charCodeAt(0));
    }
  }
  return {padding,named};
}

export function acceptTriviaExtraPrefix(policy,kind,byte) {
  return policy===null||!policy.named.has(kind)||policy.padding.has(byte)||policy.named.get(kind)===byte;
}
