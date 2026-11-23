// Recorded ABI and spelling styles; the seven numeric algorithms stay generated.
import {tokenize} from '../src/translation/lexer.js';
import {readProductRuleStyleCorrections} from './product-rule-style-corrections.mjs';

function syntax(source){
 const tokens=tokenize(source,'Rust').tokens,matching=new Map(),stack=[];
 for(let index=0;index<tokens.length;index++)if(tokens[index].kind==='punct'){
  const value=tokens[index].value;
  if(['(','[','{'].includes(value))stack.push(index);
  else if([')',']','}'].includes(value)){const start=stack.pop();matching.set(start,index);matching.set(index,start);}
 }
 return{tokens,matching};
}
export function readSourceTextRustBlock(source,kind,name){
 const{tokens,matching}=syntax(source),index=tokens.findIndex((token,index)=>token.value===kind&&tokens[index+1]?.value===name);
 if(index<0)throw new Error(`source-text styles lost ${kind} ${name}`);
 let open=index+2;while(tokens[open]?.value!=='{')open++;
 return source.slice(tokens[index-1]?.value==='pub'?tokens[index-1].start:tokens[index].start,tokens[matching.get(open)].end);
}
function parentheses(source){
 for(let pass=0;pass<1000;pass++){
  const{tokens,matching}=syntax(source);let pair;
  for(let index=0;index<tokens.length;index++){
   const token=tokens[index];if(token.value!=='('||token.kind!=='punct')continue;
   const close=matching.get(index),previous=tokens[index-1],next=tokens[close+1];
   let tuple=false;for(let at=index+1;at<close;at++){if(tokens[at].value===','){tuple=true;break;}if(['(','[','{'].includes(tokens[at].value))at=matching.get(at);}
   if(tuple)continue;
   const call=previous&&(previous.kind==='identifier'&&!['return','if','match'].includes(previous.value)||[')',']','!'].includes(previous.value));
   const literal=close===index+2&&tokens[index+1].kind==='number'||close===index+3&&tokens[index+1].value==='-'&&tokens[index+2].kind==='number';
   if(literal&&!call){pair=[index,close];break;}
   if(previous?.value==='if'&&next?.value==='{'){pair=[index,close];break;}
   if(tokens[index+1]?.value==='('&&matching.get(index+1)===close-1){
    pair=call?[index+1,close-1]:[index,close];break;
   }
   if(previous?.value==='='&&next?.value===';'||previous?.value==='{'&&next?.value==='}'){
    pair=[index,close];break;
   }
  }
  if(!pair)return source;
  const[open,close]=pair;source=source.slice(0,tokens[open].start)+source.slice(tokens[open].end,tokens[close].start)+source.slice(tokens[close].end);
 }
 throw new Error('source-text numeric parentheses styles did not converge');
}

export function readSourceTextByteLibraryStyles(raw,provider){
 const actions=[];let current=raw;
 const apply=action=>{if(!current.includes(action.from))throw new Error('source-text library style lost its original context');current=current.split(action.from).join(action.to);};
 for(const action of readProductRuleStyleCorrections(raw,'source',provider))if(current.includes(action.from)){actions.push(action);apply(action);}
 const replace=(from,to)=>{if(current.includes(from)){const action={op:'replace',field:'source',from,to};actions.push(action);apply(action);}};
 replace('pub fn is_integer(', 'fn source_text_integer(');
 replace('crate::ml_math::is_integer(', 'source_text_integer(');
 replace('mut units: Vec<f64>','units: &[f64]');replace('units: Vec<f64>','units: &[f64]');
 replace('mut offset: f64','offset: f64');
 replace('crate::ml_array::','ml_array::');
 replace('units.clone()','units');replace('&units,','units,');
 replace('.clone()','');replace('.unwrap_or_else(|| (-1f64))','.unwrap_or(-1f64)');
 replace('(units.len() as f64)','serde_json::Number::from(units.len()).as_f64().expect("a source unit vector length has a finite binary64 representation")');
 for(const match of [...current.matchAll(/crate::(?:MlReadSourceText\w+|ml_read_source_text_\w+|read_source_text_\w+)\b/gu)])replace(match[0],match[0].replace('crate::','crate::translation::source_text_byte_rules::'));
 // Give the existing multiplication its own binding so its original rounding
 // remains separate from the following addition; no fused operation is added.
 replace('(((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)',
  '{ let scaled = (first - 55296f64) * 1024f64; ((65536f64 + scaled) + second) - 56320f64 }');
 const initial=parentheses(current);if(initial!==current)replace(current,initial);
 for(const[left,right]of[['x.trunc()','x'],['bytes_2','offset'],['width','1f64'],['position','0f64'],['width','2f64'],['width','3f64'],['position','(width - 1f64)'],['width','4f64'],['position','1f64']]){
  replace(`${left} == ${right}`,`matches!(${left}.partial_cmp(&${right}), Some(std::cmp::Ordering::Equal))`);
 }
 for(const[variable,start,end]of[['first',55296,56319],['first',56320,57343],['second',56320,57343],['scalar',55296,57343]])
  replace(`(${variable} >= ${start}f64) && (${variable} <= ${end}f64)`,`(${start}f64..=${end}f64).contains(&${variable})`);
 const offsetMatch=/let ml_s2 = ([^;]+);\s*match ml_s2 \{\s*(crate::translation::source_text_byte_rules::MlReadSourceTextCodeUnitOffsetLoop2Result::MlReadSourceTextCodeUnitOffsetLoop2Done\(index_2, bytes_2\)) => \{([\s\S]*?)\n\s*\}\n\s*\}/u.exec(current);
 if(offsetMatch)replace(offsetMatch[0],`let ${offsetMatch[2]} = ${offsetMatch[1]};\n${offsetMatch[3]}`);
 const lengthMatch=/match ml_s1 \{\s*(crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Done)\(bytes_2\) => \{\s*bytes_2\s*\}\s*(crate::translation::source_text_byte_rules::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Return)\(ml_result1\) => \{\s*ml_result1\s*\}\s*\}/u.exec(current);
 if(lengthMatch)replace(lengthMatch[0],`match ml_s1 { ${lengthMatch[1]}(value) | ${lengthMatch[2]}(value) => value }`);
 for(const match of [...current.matchAll(/\b\d{5,}(?:\.\d+)?f64\b/gu)])replace(match[0],match[0].replace(/\B(?=(\d{3})+(?!\d))/gu,'_'));
 const cleaned=parentheses(current);if(cleaned!==current)replace(current,cleaned);
 for(const match of [...current.matchAll(/^#\[must_use\]\npub fn (\w+)\(/gmu)])replace(match[0],`/// Evaluate the shared Unicode source-text byte model.\n${match[0]}`);
 return actions;
}

export {parentheses as simplifySourceTextRustParentheses};
