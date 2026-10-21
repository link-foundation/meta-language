// Simplify primitive Boolean predicates only under their actual parameter types.
// Every replacement is recorded against the complete owning source function.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

export function readTypedBooleanParameterStyles(source,name) {
  const block=readSourceTextRustBlock(source,'fn',name),{tokens}=tokenize(block,'Rust');
  const pairs=new Map(),stack=[];
  for(let index=0;index<tokens.length;index++){
    const value=tokens[index].value;
    if(tokens[index].kind!=='punct')continue;
    if(['(','[','{'].includes(value))stack.push(index);
    else if([')',']','}'].includes(value)){
      const start=stack.pop();assert.equal(tokens[start]?.value,({')':'(',']':'[','}':'{'})[value]);
      pairs.set(start,index);
    }
  }
  assert.equal(stack.length,0);
  const declaration=tokens.findIndex(token=>token.value==='fn'),open=declaration+2,close=pairs.get(open);
  assert.equal(tokens[declaration+1].value,name);assert.equal(tokens[open].value,'(');
  const booleans=new Set(),strings=new Set();
  for(let start=open+1,end=start;end<=close;end++){
    if(end!==close&&tokens[end].value!==',')continue;
    if(start===end){start=end+1;continue;}
    assert.equal(tokens[start].kind,'identifier');assert.equal(tokens[start+1].value,':');
    const type=tokens.slice(start+2,end).map(token=>token.value).join('');
    assert.ok(['bool','&str'].includes(type),'primitive immutable parameter types required');
    const parameters=type==='bool'?booleans:strings;assert.ok(!booleans.has(tokens[start].value)&&!strings.has(tokens[start].value));
    parameters.add(tokens[start].value);start=end+1;
  }
  // No inner binder can shadow a parameter or change a primitive decision.
  assert.ok(!tokens.slice(close+1).some(token=>['let','for','match','fn','move','async','loop','while'].includes(token.value)||token.value==='|'),
    'a closed parameter-only branch body is required');
  const literal=value=>({kind:'literal',value,boolean:true});
  const render=(node,parent=0)=>{
    if(node.kind==='literal')return String(node.value);
    if(node.kind==='not')return '!'+render(node.child,3);
    if(node.kind==='comparison')return parent>2?'('+node.source+')':node.source;
    if(node.kind!=='binary')return node.source;
    const priority=node.operator==='||'?1:2;
    const text=render(node.left,priority)+' '+node.operator+' '+render(node.right,priority);
    return priority<parent?'('+text+')':text;
  };
  const parse=(start,end)=>{
    while(tokens[start]?.value==='('&&pairs.get(start)===end-1){start++;end--;}
    if(start>=end)return null;
    for(const operator of ['||','&&','!=','==']){
      for(let at=start;at<end;at++){
        if(pairs.has(at)){at=pairs.get(at);continue;}
        if(tokens[at].value!==operator)continue;
        const left=parse(start,at),right=parse(at+1,end);if(!left||!right)return null;
        if(['!=','=='].includes(operator)){
          if(!left.boolean||!right.boolean)return {kind:'comparison',boolean:left.string===true&&right.string===true,source:block.slice(tokens[start].start,tokens[end-1].end)};
          if(left.kind==='literal'&&right.kind==='literal')return literal(operator==='=='?left.value===right.value:left.value!==right.value);
          if(right.kind==='literal')return (operator==='==')===right.value?left:{kind:'not',child:left,boolean:true};
          if(left.kind==='literal')return (operator==='==')===left.value?right:{kind:'not',child:right,boolean:true};
          return null;
        }
        if(!left.boolean||!right.boolean)return null;
        if(left.kind==='literal')return operator==='&&'?(left.value?right:left):(left.value?left:right);
        if(right.kind==='literal')return operator==='&&'?(right.value?left:right):(right.value?right:left);
        const complement=left.kind==='not'&&render(left.child)===render(right)||right.kind==='not'&&render(right.child)===render(left);
        if(complement)return literal(operator==='||');
        return{kind:'binary',operator,left,right,boolean:true};
      }
    }
    if(tokens[start].value==='!'){
      const child=parse(start+1,end);if(!child?.boolean)return null;
      return child.kind==='literal'?literal(!child.value):child.kind==='not'?child.child:{kind:'not',child,boolean:true};
    }
    if(end!==start+1)return null;
    const token=tokens[start];
    if(['true','false'].includes(token.value))return literal(token.value==='true');
    if(booleans.has(token.value))return{kind:'leaf',source:token.value,boolean:true};
    if(strings.has(token.value)||token.kind==='string')return{kind:'leaf',source:block.slice(token.start,token.end),string:true,boolean:false};
    return null;
  };
  const edits=[];
  for(let index=close+1;index<tokens.length;index++){
    if(tokens[index].value!=='if')continue;
    const start=index+1;let end=start;
    while(end<tokens.length&&tokens[end].value!=='{'){if(pairs.has(end))end=pairs.get(end);end++;}
    assert.ok(end<tokens.length);const expression=parse(start,end);
    if(!expression?.boolean)continue;
    const before=block.slice(tokens[start].start,tokens[end-1].end),after=render(expression);
    if(before!==after)edits.push({start:tokens[start].start,end:tokens[end-1].end,after});
  }
  let styled=block;
  for(const edit of edits.sort((left,right)=>right.start-left.start))styled=styled.slice(0,edit.start)+edit.after+styled.slice(edit.end);
  if(styled===block)return{source,actions:[]};
  assert.equal(source.split(block).length,2,'one complete source owner for the recorded primitive proof');
  const action={op:'replace',field:'source',from:block,to:styled};
  return{source:source.replace(block,styled),actions:[action]};
}
