// Rebracket contiguous primitive arguments without changing their decision body.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

export function readBooleanParameterTupleStyle(source,name) {
  const block=readSourceTextRustBlock(source,'fn',name),{tokens}=tokenize(block,'Rust');
  const matching=new Map(),stack=[];
  for(let index=0;index<tokens.length;index++)if(tokens[index].kind==='punct'){
    const value=tokens[index].value;
    if(['(','[','{'].includes(value))stack.push(index);
    else if([')',']','}'].includes(value)){const open=stack.pop();assert.equal(tokens[open]?.value,({')':'(',']':'[','}':'{'})[value]);matching.set(open,index);}
  }
  assert.equal(stack.length,0);
  const declaration=tokens.findIndex(token=>token.value==='fn'),open=declaration+2,close=matching.get(open);
  assert.equal(tokens[declaration+1].value,name);assert.equal(tokens[open].value,'(');
  const parameters=[];
  for(let start=open+1,end=start;end<=close;end++){
    if(matching.has(end)){end=matching.get(end);continue;}
    if(end!==close&&tokens[end].value!==',')continue;
    if(start<end){
      const binder=tokens[start].value==='mut'?start+1:start;
      assert.equal(tokens[binder]?.kind,'identifier');assert.equal(tokens[binder+1]?.value,':');
      parameters.push({start,end,boolean:end===binder+3&&tokens[binder+2].value==='bool',name:tokens[binder].value,mutable:binder!==start});
    }
    start=end+1;
  }
  const booleanCount=parameters.filter(row=>row.boolean).length;
  if(booleanCount<=3)return{source,actions:[],groups:[]};
  const all=tokenize(source,'Rust').tokens;
  assert.ok(!all.some(token=>['use','type','struct','enum','trait'].includes(token.value)),'the checked primitive module has no ambient type aliases or imported Boolean replacement');
  const runs=[];
  for(let first=0;first<parameters.length;first++){
    if(!parameters[first].boolean)continue;
    let end=first+1;while(end<parameters.length&&parameters[end].boolean)end++;
    if(end-first>=2)runs.push({first,end});first=end-1;
  }
  assert.ok(runs.reduce((sum,row)=>sum+row.end-row.first,0)>=booleanCount-3,'uncovered separated Boolean parameter profile');
  const used=new Set(all.map(token=>token.value)),edits=[],groups=[];let binding='';
  for(const run of runs){
    let carrier='ml_boolean_parameters',suffix=1;while(used.has(carrier))carrier='ml_boolean_parameters_'+suffix++;used.add(carrier);
    const selected=parameters.slice(run.first,run.end),first=selected[0],last=selected.at(-1);
    edits.push({start:tokens[first.start].start,end:tokens[last.end-1].end,text:carrier+': ('+selected.map(()=>'bool').join(', ')+')'});
    binding+='\n    let ('+selected.map(row=>(row.mutable?'mut ':'')+row.name).join(', ')+') = '+carrier+';';
    groups.push({first:run.first,end:run.end,carrier,names:selected.map(row=>row.name),originalArity:parameters.length});
  }
  let body=close+1;while(tokens[body]?.value!=='{')body++;
  assert.ok(body<tokens.length);edits.push({start:tokens[body].end,end:tokens[body].end,text:binding});
  let styled=block;for(const edit of edits.sort((left,right)=>right.start-left.start))styled=styled.slice(0,edit.start)+edit.text+styled.slice(edit.end);
  assert.equal(source.split(block).length,2);
  const action={op:'replace',field:'source',from:block,to:styled};
  return{source:source.replace(block,styled),actions:[action],groups};
}

export function readBooleanParameterTupleCallStyle(source,qualified,groups) {
  assert.ok(groups.length>0);
  const {tokens}=tokenize(source,'Rust'),path=tokenize(qualified,'Rust').tokens.filter(token=>token.kind!=='eof');
  const matching=new Map(),stack=[];
  for(let index=0;index<tokens.length;index++)if(tokens[index].kind==='punct'){
    const value=tokens[index].value;
    if(['(','[','{'].includes(value))stack.push(index);
    else if([')',']','}'].includes(value)){const open=stack.pop();assert.equal(tokens[open]?.value,({')':'(',']':'[','}':'{'})[value]);matching.set(open,index);}
  }
  assert.equal(stack.length,0);const calls=[];
  for(let index=0;index<tokens.length;index++)if(path.every((token,offset)=>tokens[index+offset]?.value===token.value)&&tokens[index+path.length]?.value==='(')calls.push(index);
  assert.equal(calls.length,1,'one exact qualified source call owner');
  const start=calls[0],open=start+path.length,close=matching.get(open),arguments_=[];
  for(let first=open+1,end=first;end<=close;end++){
    if(matching.has(end)){end=matching.get(end);continue;}
    if(end!==close&&tokens[end].value!==',')continue;
    if(first<end)arguments_.push({start:tokens[first].start,end:tokens[end-1].end});first=end+1;
  }
  const edits=[];
  for(const group of groups){
    assert.equal(arguments_.length,group.originalArity);assert.ok(group.end-group.first>=2);
    edits.push({offset:arguments_[group.first].start,text:'('},{offset:arguments_[group.end-1].end,text:')'});
  }
  let styled=source;
  for(const edit of edits.sort((left,right)=>right.offset-left.offset))styled=styled.slice(0,edit.offset)+edit.text+styled.slice(edit.offset);
  return{source:styled,edits,qualified,groups};
}
