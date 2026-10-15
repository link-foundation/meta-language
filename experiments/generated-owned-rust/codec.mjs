// The ordinary typed codec owns the core. This small experimental adapter reads
// its separate inspectable source proof and resolves real existing core LinkIds.
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {readProgramContainerKind,readProgramScalarKind} from '../../js/src/typed-program-link-rules.js';
const fail=reason=>{throw new TypeError('Generated owned Rust archive: '+reason);};
const key=id=>id.asU64().toString();
const fieldName=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());
export function encodeGeneratedOwnedRust(program,proof){return encodeTypedProgramLinks(program,{provenance:{experimentalSourceModel:proof.model,sourceInterpretation:proof}});}
export function decodeGeneratedOwnedRust(network,root){
 const program=decodeTypedProgramLinks(network,root),fields=new Map();
 for(const row of network.fieldRelations()){const rows=fields.get(key(row.parent))??[];rows.push(row);fields.set(key(row.parent),rows);}
 const field=(id,name)=>fields.get(key(id))?.find(row=>fieldName(row.label)===name)?.child;
 const anchors=new Map(),visited=new Set();let visits=0;
 function bind(id,value,depth=0){
  if(!value||typeof value!=='object')return;if(depth>256||++visits>100_000)fail('the core identity graph exceeds its bound');
  const idKey=key(id),previous=anchors.get(idKey);if(previous&&previous!==value)fail('a raw core identity has inconsistent semantic objects');anchors.set(idKey,value);if(visited.has(idKey))return;visited.add(idKey);
  const link=network.link(id);if(!link)fail('a core identity is missing');
  if(Array.isArray(value)){if(link.references().length!==value.length)fail('an actual core array differs from its identity graph');link.references().forEach((child,index)=>bind(child,value[index],depth+1));}
  else if(value instanceof Map){for(const pairId of link.references()){const pair=network.link(pairId)?.references();if(pair?.length!==2)fail('an actual declaration pair is malformed');const name=network.link(pair[0])?.metadata().definition;if(!value.has(name))fail('the actual declaration mapping differs from its identity graph');bind(pair[1],value.get(name),depth+1);}}
  else for(const row of fields.get(idKey)??[]){const name=fieldName(row.label);if(Object.hasOwn(value,name))bind(row.child,value[name],depth+1);}
 }
 const body=network.link(root)?.references()[0];if(!body)fail('the actual core root is missing');bind(body,program);
 const memo=new Map(),active=new Set();visits=0;
 function read(id,depth=0){
  const idKey=key(id);if(anchors.has(idKey))return anchors.get(idKey);if(active.has(idKey))fail('the source interpretation has a structural cycle');if(memo.has(idKey))return memo.get(idKey);
  if(depth>256||++visits>100_000)fail('the source interpretation exceeds its bound');const link=network.link(id);if(!link||link.metadata().language!==schema.language)fail('the source interpretation contains a foreign value');
  const container=readProgramContainerKind(link.metadata().term??'');
  if(container){if(container==='map')fail('the source interpretation has an unrepresented Map');active.add(idKey);const value=container==='array'?[]:{};memo.set(idKey,value);
   if(container==='array'){if(fields.has(idKey))fail('an ordered proof container acquired fields');for(const child of link.references())value.push(read(child,depth+1));}
   else{if(link.references().length)fail('a proof record acquired positional values');for(const row of fields.get(idKey)??[]){const name=fieldName(row.label);if(Object.hasOwn(value,name)||['__proto__','constructor','prototype'].includes(name))fail('a source proof has a duplicate or unsafe field');value[name]=read(row.child,depth+1);}}
   active.delete(idKey);return value;
  }
  if(link.references().length||fields.has(idKey))fail('a proof scalar acquired children');const kind=readProgramScalarKind(link.metadata().term??''),text=link.metadata().definition;
  if(kind==='null')return null;if(kind==='string')return text??'';if(kind==='boolean'&&['true','false'].includes(text))return text==='true';if(kind==='number'&&typeof text==='string'){try{const value=JSON.parse(text);if(typeof value==='number'&&Number.isFinite(value))return value;}catch{}}
  fail('the proof scalar has an unsupported domain');
 }
 const provenance=field(root,'provenance');if(!provenance)fail('the complete source interpretation is absent');const wrapper=read(provenance);
 if(!wrapper||Object.keys(wrapper).sort().join(',')!=='experimentalSourceModel,sourceInterpretation'||wrapper.experimentalSourceModel!==wrapper.sourceInterpretation?.model)fail('the source interpretation wrapper is inconsistent');
 return{program,proof:wrapper.sourceInterpretation};
}
