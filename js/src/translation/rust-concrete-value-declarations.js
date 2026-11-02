// Explicit value-only projection for source controls. Original declarations,
// hashes and exact removed annotation ranges stay in the returned record.
// This is no certificate for serialization/derive implementation equivalence.
import {createHash} from 'node:crypto';
import {tokenize} from './lexer.js';
const model='Rust concrete value declaration projection 1';
const hash=text=>createHash('sha256').update(text).digest('hex');
const fail=reason=>{throw new TypeError('Rust concrete value declaration projection: '+reason);};
function rewriteAttribute(text){
 const tokens=tokenize(text,'Rust').tokens.filter(token=>token.kind!=='eof');
 if(tokens[0]?.value!=='#'||tokens[1]?.value!=='['||tokens.at(-1)?.value!==']')fail('each annotation retains its complete actual outer attribute spelling');
 if(tokens[2]?.value==='serde')return {text:'',removed:['serde helper attribute']};
 if(tokens[2]?.value!=='derive')return {text,removed:[]};
 if(tokens[3]?.value!=='('||tokens.at(-2)?.value!==')')fail('a derive projection needs its actual complete trait list');
 const rows=[];let from=4;for(let at=4;at<=tokens.length-2;at++){if(at===tokens.length-2||tokens[at].value===','){if(at>from){const row=tokens.slice(from,at);if(row.some((token,index)=>index%2===0?token.kind!=='identifier':token.value!=='::'))fail('derive traits must retain explicit actual identifier paths');rows.push({name:row.map(token=>token.value).join(''),text:text.slice(row[0].start,row.at(-1).end)});}from=at+1;}}
 const erased=rows.filter(row=>['Serialize','Deserialize','serde::Serialize','serde::Deserialize'].includes(row.name));if(!erased.length)return {text,removed:[]};
 const kept=rows.filter(row=>!erased.includes(row));return {text:kept.length?text.slice(0,tokens[4].start)+kept.map(row=>row.text).join(', ')+text.slice(tokens.at(-2).start):'',removed:erased.map(row=>row.name)};
}
export function projectRustConcreteValueDeclaration(item){
 if(!item||!['struct_item','enum_item'].includes(item.kind)||typeof item.declarationText!=='string'||item.declarationText.length>16777216||!Number.isSafeInteger(item.declarationRange?.start)||!Number.isSafeInteger(item.declarationRange?.end)||item.declarationRange.end-item.declarationRange.start!==item.declarationText.length||hash(item.declarationText)!==item.declarationSha256||!Array.isArray(item.attributes)||item.attributes.length>4096)fail('a bounded complete actual captured value declaration is required');
 const changes=[];for(const attribute of item.attributes){const range=attribute.range;if(attribute.kind!=='attribute_item'||!range||!Number.isSafeInteger(range.start)||!Number.isSafeInteger(range.end)||range.start<item.declarationRange.start||range.end>item.declarationRange.end||range.end<=range.start||item.declarationText.slice(range.start-item.declarationRange.start,range.end-item.declarationRange.start)!==attribute.text)fail('each actual annotation remains inside this exact declaration source slice');const rewritten=rewriteAttribute(attribute.text);if(rewritten.removed.length)changes.push({sourceLink:attribute.sourceLink,range:{...range},original:attribute.text,replacement:rewritten.text,removed:rewritten.removed});}
 changes.sort((a,b)=>a.range.start-b.range.start);let end=item.declarationRange.start,text='';for(const change of changes){if(change.range.start<end)fail('actual projection changes cannot overlap');text+=item.declarationText.slice(end-item.declarationRange.start,change.range.start-item.declarationRange.start)+change.replacement;end=change.range.end;}text+=item.declarationText.slice(end-item.declarationRange.start);
 return {model,scope:'constructor, field, value ownership and Clone controls; serialization implementations remain excluded',kind:item.kind,name:item.name,sourceLink:item.sourceLink,originalRange:{...item.declarationRange},originalSha256:item.declarationSha256,original:item.declarationText,text,sha256:hash(text),changes};
}
