// Exact item capture from a supplied current CST. This neither reparses a body
// nor certifies semantic equivalence of a later dependency/annotation projection.
import {createHash} from 'node:crypto';
import {readProgramConcreteSyntaxFacts} from '../program-representation.js';
import {tokenize} from './lexer.js';
const model='Rust actual concrete source items 1';
// Exact source-names in parity/grammars/native/rust.lino, not alternate node
// authorities. The native adapter projects these through oracleKinds; retain
// the actual supplied CST term rather than claiming the grammar rule is a node.
const sourceKinds={structure_item:'struct_item',enumeration_item:'enum_item',constant_item:'const_item',module_item:'mod_item',implementation_item:'impl_item'};
const kinds=new Set(['function_item','struct_item','enum_item','union_item','trait_item','type_item','const_item','static_item']);
const fail=reason=>{throw new TypeError('Rust concrete source item: '+reason);};
const hash=text=>createHash('sha256').update(text).digest('hex');
const inside=(parent,child)=>parent.start<=child.start&&child.end<=parent.end;
const encloses=(parent,child)=>inside(parent,child)&&(parent.start<child.start||child.end<parent.end);
function bound(value,name){if(!Number.isSafeInteger(value)||value<1)fail(name+' must be a positive safe integer');return value;}
function identifier(text){return text.startsWith('r#')?text.slice(2):text;}
/** Select actual complete named items, including their directly preceding
 * outer attributes and all enclosed attribute nodes. modulePath means inline
 * CST modules in this supplied file, not an invented filesystem namespace.
 */
export function captureRustConcreteSourceItems(network,source,{path,selectors,byteLimit=16777216,nodeLimit=1000000,fieldLimit=4000000,selectorLimit=256,workLimit=8000000}={}){
 for(const [name,value]of Object.entries({byteLimit,nodeLimit,fieldLimit,selectorLimit,workLimit}))bound(value,name);
 if(typeof source!=='string'||Buffer.byteLength(source)>byteLimit||typeof path!=='string'||!path||!Array.isArray(selectors)||!selectors.length||selectors.length>selectorLimit)fail('bounded source text, logical path and explicit selectors are required');
 const links=network.links();if(links.length>nodeLimit)fail('the supplied current graph exceeds its node budget');
 const facts=readProgramConcreteSyntaxFacts(network,source),byId=new Map(facts.map(row=>[row.linkId,row])),fields=new Map();let fieldCount=0,work=0;
 const charge=(amount=1)=>{if(amount>workLimit-work)fail('source selection exceeds its bounded inspection work');work+=amount;};
 const parentsById=new Map();
 for(const link of links){const parent=byId.get(link.id().asU64());if(!parent)continue;for(const reference of link.references()){charge();const child=byId.get(reference.asU64());if(!child)continue;if(!inside(parent,child))continue;const owners=parentsById.get(child.linkId)??[];owners.push(parent.linkId);parentsById.set(child.linkId,owners);}}
 function actualAncestor(parent,node){const pending=[node.linkId],seen=new Set();while(pending.length){charge();const id=pending.pop();if(seen.has(id))continue;seen.add(id);for(const owner of parentsById.get(id)??[]){charge();if(owner===parent.linkId)return true;pending.push(owner);}}return false;}
 for(const edge of network.fieldRelations()){
  if(++fieldCount>fieldLimit)fail('the supplied graph exceeds its field relation budget');const parent=edge.parent.asU64(),child=edge.child.asU64();if(!byId.has(parent))continue;
  let labels=fields.get(parent);if(!labels){labels=new Map();fields.set(parent,labels);}const children=labels.get(edge.label)??[];children.push(child);labels.set(edge.label,children);
 }
 function field(parent,label,required=false){const ids=fields.get(parent.linkId)?.get(label)??[];if(ids.length>1||required&&ids.length!==1)fail('an actual '+label+' field must have one unambiguous CST owner');const selected=ids.length?byId.get(ids[0]):undefined;if(ids.length&&!selected||selected&&(!inside(parent,selected)||!(parentsById.get(selected.linkId)??[]).includes(parent.linkId)))fail('an actual '+label+' field must remain inside its owner');return selected;}
 function named(parent){const name=field(parent,'name',true);if(!['identifier','type_identifier'].includes(name.term))fail('an item name must retain its actual identifier field');return {node:name,value:identifier(source.slice(name.start,name.end))};}
 function parents(node,term){return facts.filter(parent=>{charge();return (sourceKinds[parent.term]??parent.term)===(sourceKinds[term]??term)&&encloses(parent,node)&&actualAncestor(parent,node);}).sort((a,b)=>a.start-b.start||b.end-a.end);}
 function container(node){
  const modules=parents(node,'module_item').map(parent=>{const body=field(parent,'body',true);if(!inside(body,node))fail('an inline module must actually contain the selected source item');const name=named(parent);return {name:name.value,sourceLink:parent.linkId,range:{start:parent.start,end:parent.end},nameLink:name.node.linkId};});
  const implementations=parents(node,'implementation_item');if(implementations.length>1)fail('nested implementation ownership requires its separate source interpretation');
  let implementation=null;if(implementations.length){const parent=implementations[0],type=field(parent,'type',true),body=field(parent,'body',true),trait=field(parent,'trait');if(!inside(body,node))fail('the actual implementation body must contain the selected source item');implementation={sourceLink:parent.linkId,range:{start:parent.start,end:parent.end},type:source.slice(type.start,type.end),typeLink:type.linkId,typeRange:{start:type.start,end:type.end},...(trait?{trait:source.slice(trait.start,trait.end),traitLink:trait.linkId}:{})};}
  return {modules,implementation};
 }
 function attributes(node){
  const leading=[],before=facts.filter(row=>{charge();return row.term==='attribute_item'&&row.end<=node.start;}).sort((a,b)=>b.end-a.end||b.start-a.start);let start=node.start;
  for(const candidate of before){const length=start-candidate.end;charge(length+1);const gap=source.slice(candidate.end,start);if(tokenize(gap,'Rust').tokens.some(token=>token.kind!=='eof'))break;leading.push(candidate);start=candidate.start;}
  const enclosed=facts.filter(row=>{charge();return ['attribute_item','inner_attribute_item'].includes(row.term)&&inside(node,row);});
  return {start,rows:[...leading.reverse().map(row=>({row,ownership:'leading'})),...enclosed.map(row=>({row,ownership:'enclosed'}))].map(({row,ownership})=>({sourceLink:row.linkId,kind:row.term,ownership,range:{start:row.start,end:row.end},text:source.slice(row.start,row.end)}))};
 }
 const selected=[];const identities=new Set();
 for(const selector of selectors){
  if(!selector||typeof selector!=='object'||Object.keys(selector).some(key=>!['kind','name','implementationType','implementationTrait','modulePath'].includes(key))||!kinds.has(sourceKinds[selector.kind]??selector.kind)||typeof selector.name!=='string'||!selector.name||selector.modulePath!==undefined&&(!Array.isArray(selector.modulePath)||selector.modulePath.some(name=>typeof name!=='string'||!name))||selector.implementationType!==undefined&&typeof selector.implementationType!=='string'||selector.implementationTrait!==undefined&&typeof selector.implementationTrait!=='string')fail('selectors retain explicit item kind, source name and real enclosing ownership');
  const candidates=[];
  for(const node of facts){charge();if((sourceKinds[node.term]??node.term)!==(sourceKinds[selector.kind]??selector.kind))continue;const name=named(node);if(name.value!==selector.name||parents(node,'function_item').length)continue;const owner=container(node),modules=selector.modulePath??[];
   if(owner.modules.length!==modules.length||owner.modules.some((row,index)=>row.name!==modules[index]))continue;
   if((owner.implementation?.type)!==selector.implementationType||(owner.implementation?.trait)!==selector.implementationTrait)continue;candidates.push({node,name,owner});
  }
  if(candidates.length!==1)fail('one actual source declaration must match '+selector.kind+' '+selector.name);const {node,name,owner}=candidates[0];
  if(identities.has(node.linkId))fail('source item selectors cannot duplicate an actual declaration');identities.add(node.linkId);
  const actual=network.link(node.linkId);if(actual?.metadata().language!=='Rust')fail('the selected CST item must retain its actual Rust language metadata');
  const body=field(node,'body'),annotations=attributes(node),text=source.slice(node.start,node.end),declarationText=source.slice(annotations.start,node.end);
  selected.push({kind:node.term,name:name.value,sourceLink:node.linkId,range:{start:node.start,end:node.end,byteStart:node.byteStart,byteEnd:node.byteEnd},nameLink:name.node.linkId,nameRange:{start:name.node.start,end:name.node.end},text,sha256:hash(text),declarationRange:{start:annotations.start,end:node.end},declarationText,declarationSha256:hash(declarationText),...(body?{bodyLink:body.linkId,bodyRange:{start:body.start,end:body.end},bodyText:source.slice(body.start,body.end)}:{}),...owner,attributes:annotations.rows});
 }
 return {model,language:'Rust',path,sourceSha256:hash(source),sourceBytes:Buffer.byteLength(source),sourceUnits:source.length,items:selected};
}
