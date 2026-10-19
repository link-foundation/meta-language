// Preserve the frozen export and guard its portable-library derivative.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {tokenize} from '../src/translation/lexer.js';

const address=new URL('./product-rule-style-corrections.mjs',import.meta.url);
const source=readFileSync(address,'utf8');
assert.equal(createHash('sha256').update(source).digest('hex'),'9728ead881e57f16401321be9eb00e25416aab74cdb4d7164485cd0a3abbc216','the complete reviewed producer remains frozen');
const exported=source+'\nexport { simplifyBranches as simplifyRustBranches, simplifyReturns, bindConditionBlocks };\n';
const linked=exported.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>'from '+quote+new URL(path,address).href+quote)
  .replaceAll('import.meta.url',JSON.stringify(address.href));
const reviewed=await import('data:text/javascript;base64,'+Buffer.from(linked).toString('base64'));

export const simplifyRustBranches=reviewed.simplifyRustBranches;

/** Recognize a complete leading negation in generated Boolean decisions. */
export function acceptRustWholeConditionNegation(condition) {
  const tokens=tokenize(condition,'Rust').tokens.filter(token=>token.kind!=='eof');
  if(tokens[0]?.kind!=='punct'||tokens[0].value!=='!')return false;
  const matching=new Map(),stack=[];
  for(let index=0;index<tokens.length;index++){
    if(tokens[index].kind!=='punct')continue;
    const value=tokens[index].value;
    if(['(','[','{'].includes(value))stack.push(index);
    else if([')',']','}'].includes(value)){
      const open=stack.pop();
      if(open===undefined||tokens[open].value!==({')':'(',']':'[','}':'{'})[value])return false;
      matching.set(open,index);
    }
  }
  if(stack.length!==0)return false;
  let index=1;
  while(tokens[index]?.kind==='punct'&&tokens[index].value==='!')index++;
  if(tokens[index]?.kind==='identifier')index++;
  else if(tokens[index]?.kind==='punct'&&tokens[index].value==='('){
    if(matching.get(index)===index+1)return false;
    index=matching.get(index)+1;
  }else return false;
  while(index<tokens.length){
    const token=tokens[index];
    if(token.kind!=='punct')return false;
    if(['(','['].includes(token.value))index=matching.get(index)+1;
    else if(token.value==='.'||token.value==='::'){
      const member=tokens[index+1];
      if(member?.kind!=='identifier'&&!(token.value==='.'&&member?.kind==='number'&&/^\d+$/u.test(member.raw)))return false;
      index+=2;
    }else if(token.value==='?')index++;
    else return false;
  }
  return true;
}

const originalNegationGuard="if (condition.startsWith('!')) {";
assert.equal(source.split(originalNegationGuard).length,2,'the guarded derivative changes exactly one frozen negation guard');
const originalBranchJoin='+ source.slice(tokens[inner].start, tokens[child.end].end)';
assert.equal(source.split(originalBranchJoin).length,2,'the guarded derivative retains the complete nested if at its one frozen branch join');
const guardedSource=source.replace(originalNegationGuard,"if (condition.startsWith('!') && acceptRustWholeConditionNegation(condition)) {")
  .replace(originalBranchJoin,"+ (tokens[node.alternate.open - 1].end === tokens[node.alternate.open].start ? ' ' : '')"
    + " + source.slice(tokens[inner].start, tokens[child.end].end)")
  +'\n'+acceptRustWholeConditionNegation.toString()
  +'\nexport { simplifyBranches as simplifyRustBranches };\n';
const guardedLinked=guardedSource.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>'from '+quote+new URL(path,address).href+quote)
  .replaceAll('import.meta.url',JSON.stringify(address.href));
const guarded=await import('data:text/javascript;base64,'+Buffer.from(guardedLinked).toString('base64'));

/** Compose a guarded branch derivative with the other unchanged reviewed styles. */
export function simplifyPortableProductLibraryStyles(source) {
  return reviewed.bindConditionBlocks(reviewed.simplifyReturns(guarded.simplifyRustBranches(source)));
}

/** Keep an Option's selected value and its original pure missing-value policy. */
export function simplifyRustOptionIdentityMatches(source) {
  const tokens=tokenize(source,'Rust').tokens,matching=new Map(),stack=[];
  for(let index=0;index<tokens.length;index++){
    if(tokens[index].kind!=='punct')continue;
    const value=tokens[index].value;
    if(['(','[','{'].includes(value))stack.push(index);
    else if([')',']','}'].includes(value)){
      const open=stack.pop();
      assert.notEqual(open,undefined,'an Option style delimiter retains its opening');
      assert.equal(tokens[open].value,({')':'(',']':'[','}':'{'})[value]);
      matching.set(open,index);
    }
  }
  assert.equal(stack.length,0,'an Option style retains every closing delimiter');
  const edits=[];
  for(let index=0;index<tokens.length;index++){
    if(tokens[index].kind!=='identifier'||tokens[index].value!=='match')continue;
    let open=index+1;
    while(open<tokens.length&&tokens[open].value!=='{'){
      if(matching.has(open))open=matching.get(open)+1;
      else open++;
    }
    const close=matching.get(open);
    if(close===undefined)continue;
    const arm=tokens.slice(open+1,close),values=arm.map(token=>token.value);
    if(values.at(-1)===',')values.pop();
    if(values.length<10||values[0]!=='Some'||values[1]!=='('||arm[2]?.kind!=='identifier'
      ||values[3]!==')'||values[4]!=='=>'||values[5]!==values[2]||values[6]!==','
      ||values[7]!=='None'||values[8]!=='=>')continue;
    const fallback=values.slice(9);
    let method;
    if(fallback.length===1&&fallback[0]==='false')method='unwrap_or_default()';
    else if(fallback.length===3&&fallback[0]==='f64'&&fallback[1]==='::'&&fallback[2]==='NAN')method='unwrap_or(f64::NAN)';
    else continue;
    const selected=source.slice(tokens[index+1].start,tokens[open].start).trim();
    edits.push([tokens[index].start,tokens[close].end,'('+selected+').'+method]);
    index=close;
  }
  for(const[start,end,text]of edits.reverse())source=source.slice(0,start)+text+source.slice(end);
  return source;
}
