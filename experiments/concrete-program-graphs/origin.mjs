// Read physical source from inspectable origin links. Semantic emission remains
// the existing checked emitter; this reader never presents an origin as its code.
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {acceptProgramSchemaRoot,readProgramScalarKind,readProgramContainerKind} from '../../js/src/typed-program-link-rules.js';
import {readGrammarSelectionTreeChildren,acceptGrammarSelectionPhysicalSpan} from '../../js/src/translation/grammar-selection-mapping-rules.js';
import {readProjectData} from '../../js/scripts/project-data.mjs';
const rules=readProjectData(new URL('../../parity/fixtures/concrete-program-origin-rules.lino',import.meta.url));
const labelOf=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const fieldOf=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]
  ??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());
const refuse=message=>{throw new TypeError('concrete program origin: '+message);};

function concreteOriginSettings(options) {
  const settings={...rules.limits};
  for(const[key,value]of Object.entries(options)){
    if(key==='plane')continue;
    if(!Object.hasOwn(settings,key)||!Number.isSafeInteger(value)||value<0)refuse('invalid resource option');
    settings[key]=value;
  }
  const plane=options.plane??'source';if(!rules.planes.includes(plane))refuse('unknown source plane');
  return{...settings,plane};
}

function concreteOriginGraphReader(network,root,settings) {
  if(network.len()>settings.maxGraphLinks)refuse('graph resource limit');
  const rootLink=network.link(root);
  if(!rootLink||!acceptProgramSchemaRoot(rootLink.metadata().term??'',rootLink.metadata().definition??'',
    String(rootLink.references().length)))refuse('invalid program root');
  const index=new Map();
  for(const relation of network.fieldRelations()){
    const parent=relation.parent.asU64();if(!index.has(parent))index.set(parent,new Map());
    const fields=index.get(parent);if(!fields.has(relation.label))fields.set(relation.label,[]);
    fields.get(relation.label).push(relation.child);
  }
  let visits=0,scalarBytes=0;
  const visit=depth=>{if(++visits>settings.maxReadNodes)refuse('data resource limit');
    if(depth>settings.maxDataDepth)refuse('data depth limit');};
  const field=(parent,name,required=true)=>{
    const values=index.get(parent.asU64())?.get(labelOf(name));
    if(!values?.length){if(required)refuse('missing '+name+' field');return undefined;}
    if(values.length!==1)refuse('duplicate '+name+' field');return values[0];
  };
  const string=id=>{const metadata=network.link(id)?.metadata();
    if(!metadata||readProgramScalarKind(metadata.term??'')!=='string'||typeof metadata.definition!=='string')refuse('a string field is required');
    return metadata.definition;};
  const active=new Set(),copies=new Map();
  function read(id){
    const box=Object.create(null),pending=[{id,depth:0,parent:box,name:'value'}];
    while(pending.length){
      const task=pending.pop(),key=task.id.asU64();
      if(task.leave){active.delete(key);copies.set(key,task.result);continue;}
      visit(task.depth);if(active.has(key))refuse('cyclic origin data');
      const assign=value=>{
        if(Array.isArray(task.parent))task.parent[task.name]=value;
        else Object.defineProperty(task.parent,task.name,{value,enumerable:true});
      };
      if(copies.has(key)){assign(copies.get(key));continue;}
      const link=network.link(task.id);if(!link)refuse('dangling origin link');
      const metadata=link.metadata(),container=readProgramContainerKind(metadata.term??'');
      const scalar=readProgramScalarKind(metadata.term??'');
      if(scalar==='string'){
        const value=string(task.id);scalarBytes+=Buffer.byteLength(value);if(scalarBytes>settings.maxBytes)refuse('scalar byte limit');assign(value);continue;
      }
      if(scalar==='number'){
        const value=Number(metadata.definition);
        if(!Number.isFinite(value)||(String(value)!==metadata.definition
          &&!(metadata.definition==='-0'&&Object.is(value,-0))))refuse('invalid numeric field');
        assign(value);continue;
      }
      if(scalar==='boolean'){if(!['true','false'].includes(metadata.definition))refuse('invalid boolean field');assign(metadata.definition==='true');continue;}
      if(scalar==='null'){assign(null);continue;}if(scalar==='undefined'){assign(undefined);continue;}
      if(container!=='array'&&container!=='record')refuse('unknown origin data kind');
      const result=container==='array'?[]:Object.create(null),entries=[];assign(result);active.add(key);
      if(container==='array')for(const[index,child]of link.references().entries())entries.push([index,child]);
      else{
        const names=new Set();
        for(const[label,children]of index.get(key)??[]){
          if(children.length!==1)refuse('duplicate origin data field');
          const name=fieldOf(label);if(names.has(name))refuse('ambiguous origin field spelling');
          names.add(name);entries.push([name,children[0]]);
        }
      }
      pending.push({id:task.id,leave:true,result});
      for(let offset=entries.length-1;offset>=0;offset--){const[name,child]=entries[offset];
        pending.push({id:child,depth:task.depth+1,parent:result,name});}
    }
    return box.value;
  }

  return{field,string,read,visit};
}

function concreteOriginPhysicalBytes(tree,settings) {
  const pending=[{node:tree,depth:0}],active=new Set(),chunks=[];
  let cursor=0,visits=0;
  while(pending.length){
    const task=pending.pop(),node=task.node;
    if(task.leave){
      if(!acceptGrammarSelectionPhysicalSpan(node.start,node.end,task.begin,cursor-task.begin,false))refuse('physical container span differs');
      active.delete(node);continue;
    }
    if(++visits>settings.maxReadNodes||task.depth>settings.maxTreeDepth)refuse('tree resource limit');
    if(!node||typeof node!=='object'||active.has(node))refuse('invalid or cyclic physical tree');
    const role=readGrammarSelectionTreeChildren(node.type,node.sourceChildren!==undefined);
    if(role==='leaf'){
      let bytes;
      if(typeof node.text==='string'){
        const size=Buffer.byteLength(node.text);if(cursor+size>settings.maxBytes)refuse('output byte limit');
        bytes=Buffer.from(node.text);if(bytes.toString('utf8')!==node.text)refuse('non-scalar leaf text');
      }else if(node.text===null&&typeof node.hex==='string'&&/^(?:[0-9a-f]{2})*$/u.test(node.hex)){
        if(cursor+node.hex.length/2>settings.maxBytes)refuse('output byte limit');bytes=Buffer.from(node.hex,'hex');
      }else refuse('invalid physical leaf bytes');
      if(!acceptGrammarSelectionPhysicalSpan(node.start,node.end,cursor,bytes.length,false))refuse('physical leaf span differs');
      chunks.push(bytes);cursor=node.end;continue;
    }
    if(role==='missing'){
      if(!acceptGrammarSelectionPhysicalSpan(node.start,node.end,cursor,0,true))refuse('invalid missing span');continue;
    }
    const children=role==='children'?node.children:role==='sourceChildren'?node.sourceChildren:role==='root'?[node.root]:null;
    if(!Array.isArray(children))refuse('unknown or incomplete concrete node');
    active.add(node);pending.push({node,leave:true,begin:cursor});
    for(let index=children.length-1;index>=0;index--)pending.push({node:children[index],depth:task.depth+1});
  }
  if(tree.start!==0||tree.end!==cursor)refuse('incomplete physical source');
  return new Uint8Array(Buffer.concat(chunks,cursor));
}

/** Return the current stored concrete origin, without parser state or source
 * files. No claim about the meaning of the edited semantic program is made.
 */
export function readConcreteProgramOrigin(network,root,options={}) {
  const settings=concreteOriginSettings(options),reader=concreteOriginGraphReader(network,root,settings);
  let current=reader.field(root,'provenance');const seen=new Set();
  while(current){
    reader.visit(seen.size);const key=current.asU64();if(seen.has(key))refuse('cyclic origin profile');seen.add(key);
    const profile=reader.field(current,'sourceProfile',false);
    if(profile){
      const concrete=reader.field(profile,'concreteSyntax',false);
      if(concrete){
        const model=reader.string(reader.field(profile,'model')),language=reader.string(reader.field(profile,'language'));
        if(!rules.models.includes(model)||!rules.languages.includes(language))refuse('unknown source profile');
        const selected=settings.plane==='source'?profile:reader.field(profile,'lowering');
        const selectedLanguage=reader.string(reader.field(selected,'language'));
        const tree=reader.read(reader.field(reader.field(selected,'concreteSyntax'),'tree'));
        return{language:selectedLanguage,model,plane:settings.plane,bytes:concreteOriginPhysicalBytes(tree,settings)};
      }
    }
    current=reader.field(current,'userProvenance',false);
  }
  refuse('no concrete source profile');
}

export function readConcreteProgramOriginText(network,root,options={}) {
  const result=readConcreteProgramOrigin(network,root,options);
  return{...result,text:new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(result.bytes)};
}
