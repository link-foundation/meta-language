// Read physical source from inspectable origin links. Semantic emission remains
// the existing checked emitter; this reader never presents an origin as its code.
import {LinkType} from '../../js/src/primitives.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {acceptProgramSchemaRoot,readProgramScalarKind,readProgramContainerKind} from '../../js/src/typed-program-link-rules.js';
import {readGrammarSelectionTreeChildren,acceptGrammarSelectionPhysicalSpan} from '../../js/src/translation/grammar-selection-mapping-rules.js';
import {readProjectData} from '../../js/scripts/project-data.mjs';
const rules=readProjectData(new URL('../../parity/fixtures/concrete-program-origin-rules.lino',import.meta.url));
const labelOf=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
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

// Link.references() returns an owned copy. This read-only repository adapter
// checks the actual own backing array before counting or indexing references.
function concreteOriginReferences(link) {
  const descriptor=Object.getOwnPropertyDescriptor(link,'_references');
  if(!descriptor||!Object.hasOwn(descriptor,'value')||!Array.isArray(descriptor.value))refuse('invalid reference storage');
  return descriptor.value;
}

function concreteOriginGraphReader(network,root,settings) {
  if(network.len()>settings.maxGraphLinks)refuse('graph resource limit');
  const rootLink=network.link(root);
  if(!rootLink||!acceptProgramSchemaRoot(rootLink.metadata().term??'',rootLink.metadata().definition??'',
    String(concreteOriginReferences(rootLink).length)))refuse('invalid program root');
  const index=new Map();
  for(const link of network.links()){
    if(link.metadata().linkType!==LinkType.Field)continue;
    const references=concreteOriginReferences(link);if(references.length!==3)continue;
    const label=network.link(references[1])?.metadata().term;if(typeof label!=='string')continue;
    const parent=references[0].asU64();if(!index.has(parent))index.set(parent,new Map());
    const fields=index.get(parent);if(!fields.has(label))fields.set(label,[]);
    fields.get(label).push(references[2]);
  }
  let visits=0,scalarBytes=0;
  const visit=depth=>{if(++visits>settings.maxReadNodes)refuse('data resource limit');
    if(depth>settings.maxDataDepth)refuse('data depth limit');};
  const field=(parent,name,required=true)=>{
    visit(0);const values=index.get(parent.asU64())?.get(labelOf(name));
    if(!values?.length){if(required)refuse('missing '+name+' field');return undefined;}
    if(values.length!==1)refuse('duplicate '+name+' field');return values[0];
  };
  const scalar=(id,kind)=>{
    visit(0);const metadata=network.link(id)?.metadata();
    if(!metadata||readProgramScalarKind(metadata.term??'')!==kind)refuse('a '+kind+' field is required');
    return metadata.definition;
  };
  const charge=value=>{
    if(typeof value!=='string')refuse('a scalar definition is required');
    scalarBytes+=Buffer.byteLength(value);if(scalarBytes>settings.maxBytes)refuse('scalar byte limit');return value;
  };
  const string=id=>charge(scalar(id,'string'));
  const number=id=>{
    const definition=charge(scalar(id,'number')),value=Number(definition);
    if(!Number.isFinite(value)||(String(value)!==definition&&!(definition==='-0'&&Object.is(value,-0))))refuse('invalid numeric field');
    return value;
  };
  const container=(id,kind)=>{
    visit(0);const link=network.link(id);
    if(!link||readProgramContainerKind(link.metadata().term??'')!==kind)refuse('a '+kind+' origin is required');return link;
  };
  return{field,string,number,visit,remaining:()=>settings.maxReadNodes-visits,
    record:id=>container(id,'record'),array:id=>concreteOriginReferences(container(id,'array')),
    nullableText:id=>{
      visit(0);const metadata=network.link(id)?.metadata();
      if(!metadata)refuse('dangling text link');
      if(readProgramScalarKind(metadata.term??'')==='null')return null;
      return string(id);
    }};
}

function concreteOriginPhysicalBytes(tree,reader,settings) {
  const stack=[{id:tree,depth:0}],active=new Set(),chunks=[];let cursor=0;
  while(stack.length){
    const frame=stack[stack.length-1];
    if(frame.children){
      if(frame.position<frame.children.length){
        if(reader.remaining()<1)refuse('tree resource limit');
        stack.push({id:frame.children[frame.position++],depth:frame.depth+1});continue;
      }
      if(!acceptGrammarSelectionPhysicalSpan(frame.start,frame.end,frame.begin,cursor-frame.begin,false))refuse('physical container span differs');
      active.delete(frame.id.asU64());stack.pop();continue;
    }
    reader.visit(frame.depth*2+1);if(frame.depth>settings.maxTreeDepth)refuse('tree resource limit');
    const key=frame.id.asU64();if(active.has(key))refuse('cyclic physical tree');reader.record(frame.id);
    const kind=reader.string(reader.field(frame.id,'type'));
    const start=reader.number(reader.field(frame.id,'start')),end=reader.number(reader.field(frame.id,'end'));
    const sourceChildren=reader.field(frame.id,'sourceChildren',false);
    const role=readGrammarSelectionTreeChildren(kind,sourceChildren!==undefined);
    if(role==='leaf'){
      const text=reader.nullableText(reader.field(frame.id,'text'));let bytes;
      if(text!==null){
        const size=Buffer.byteLength(text);if(cursor+size>settings.maxBytes)refuse('output byte limit');
        bytes=Buffer.from(text);if(bytes.toString('utf8')!==text)refuse('non-scalar leaf text');
      }else{
        const hex=reader.string(reader.field(frame.id,'hex'));
        if(!/^(?:[0-9a-f]{2})*$/u.test(hex))refuse('invalid physical leaf bytes');
        if(cursor+hex.length/2>settings.maxBytes)refuse('output byte limit');bytes=Buffer.from(hex,'hex');
      }
      if(!acceptGrammarSelectionPhysicalSpan(start,end,cursor,bytes.length,false))refuse('physical leaf span differs');
      chunks.push(bytes);cursor=end;stack.pop();continue;
    }
    if(role==='missing'){
      if(!acceptGrammarSelectionPhysicalSpan(start,end,cursor,0,true))refuse('invalid missing span');stack.pop();continue;
    }
    let children;
    if(role==='children')children=reader.array(reader.field(frame.id,'children'));
    else if(role==='sourceChildren')children=reader.array(sourceChildren);
    else if(role==='root')children=[reader.field(frame.id,'root')];
    else refuse('unknown concrete node');
    if(children.length>reader.remaining())refuse('child reference resource limit');
    active.add(key);Object.assign(frame,{children,position:0,start,end,begin:cursor});
  }
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
        const tree=reader.field(reader.field(selected,'concreteSyntax'),'tree');
        return{language:selectedLanguage,model,plane:settings.plane,bytes:concreteOriginPhysicalBytes(tree,reader,settings)};
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
