// Typed native storage only. No replacement numeric, lifetime or parser semantics.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {captureOrdinaryInversePlan,extendRecursiveInversePlan,readOrdinaryInversePlan} from './plan.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';

const root=new URL('../../../../',import.meta.url);
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});
const rustLiteral=value=>{assert.ok(typeof value==='string'&&/^[\x20-\x7e]*$/u.test(value),'the captured constructor identifier profile is ASCII');return JSON.stringify(value);};

/** Derive every actual field and variant from the authenticated source graph. */
export function readNativeConstructorStorageInventory(authority) {
  const plan=readOrdinaryInversePlan(authority),byId=new Map(plan.graph.nodes.map(row=>[row.identity,row]));
  const bindings=new Map(plan.bindings.map(row=>[row.declarationReference,row]));
  const declarations=plan.graph.roots.map(id=>byId.get(id));assert.equal(declarations.length,9);
  const storage=new Map();
  const describe=id=>{
    const node=byId.get(id);assert.ok(node);
    const generic=(prefix,parameters,carrier)=>{const children=parameters.map(describe);return{rust:prefix+'<'+children.map(row=>row.rust).join(', ')+'>',carrier:carrier+children.map(row=>row.carrier).join('And'),boxed:node.kind==='Option'&&children[0].boxed};};
    if(['named','struct_item','enum_item','type_item'].includes(node.kind)){
      const declaration=node.kind==='named'?byId.get(node.owner):node;assert.ok(declarations.includes(declaration));
      assert.match(declaration.name,/^[A-Za-z_][A-Za-z_0-9]*$/u);
      return{rust:'meta_language::'+declaration.name,carrier:declaration.name,boxed:declaration.kind!=='type_item'};
    }
    if(['Option','Vec','Box','IndexMap'].includes(node.kind))return generic(({Option:'Option',Vec:'Vec',Box:'Box',IndexMap:'indexmap::IndexMap'})[node.kind],node.arguments,({Option:'Optional',Vec:'SequenceOf',Box:'Boxed',IndexMap:'OrderedMapOf'})[node.kind]);
    if(node.kind==='reference'){
      assert.equal(node.lifetime,'static');assert.equal(node.mutable,false);assert.equal(byId.get(node.references[0]).kind,'str');
      return{rust:"&'static str",carrier:'ExistingStaticReference',boxed:false};
    }
    const primitive={bool:['bool','Boolean'],usize:['usize','Size'],i64:['i64','SignedInteger'],String:['String','Text'],str:['String','Text'],json:['serde_json::Value','StructuredValue']}[node.kind];
    assert.ok(primitive,'unsupported native storage source form '+node.kind);return{rust:primitive[0],carrier:primitive[1],boxed:false};
  };
  const operations=plan.graph.nodes.filter(row=>!['builtin','external'].includes(row.kind))
    .sort((left,right)=>Buffer.compare(Buffer.from(left.identity),Buffer.from(right.identity))).map(node=>{
      const type=describe(node.identity);if(storage.has(type.carrier))assert.equal(storage.get(type.carrier).rust,type.rust);else storage.set(type.carrier,type);
      const fields=rows=>rows.map(field=>({name:field.nativeName,typeReference:field.typeReference,storage:describe(field.typeReference),source:field.source}));
      const binding=bindings.get(node.identity);
      return{sourceReference:node.identity,form:node.kind,storage:type,source:node,
        ...(node.kind==='named'?{child:node.owner}:node.kind==='type_item'?{child:node.aliasTypeReference}:node.kind==='reference'?{child:node.references[0]}:{}),
        ...(node.arguments?{arguments:node.arguments}:{}),
        ...(binding?.fields?{fields:fields(binding.fields)}:{}),
        ...(binding?.variants?{variants:binding.variants.map(variant=>({identity:variant.identity,name:variant.nativeName,source:variant.source,
          ...(variant.fields?{fields:fields(variant.fields)}:{tuple:fields(variant.tuple)})}))}:{})};
    });
  assert.equal(operations.length,plan.graph.nodes.length-10,'every source occurrence remains independently qualified');
  return{schemaVersion:1,planSha256:authority.sha256,roots:plan.graph.roots,sourceInventory:plan.graph.sourceInventory,
    operations,storage:[...storage.values()],storageCarrier:'typed native values; declared records and variants have boxed carriers without merging source identities',
    boundary:'Complete typed host packing for the captured native declarations. Source-qualified types remain distinct even when they use the same Rust storage carrier. Scalars arrive as actual typed native values; JSON number variants and existing static references are never manufactured from projections. This does not validate a complete owned model, attest compiler provenance, decode recursive JSON or integrate the public default CST path.'};
}

/** Emit the actual native constructors; all field/type choices come from Links instructions. */
export function renderNativeConstructorStorage(inventory) {
  const byId=new Map(inventory.operations.map(row=>[row.sourceReference,row]));
  const files=new Map(),parts=[];
  const requireEmpty='require_empty_choice(choice)?; require_no_scalar(scalar)?;';
  const borrowsScalar=operation=>operation.form==='reference'||(!operation.fields&&!operation.variants&&['bool','usize','i64'].includes(operation.storage.rust));
  const take=(reference)=>'take_child!(children, '+rustLiteral(reference)+', '+byId.get(reference).storage.carrier+(byId.get(reference).storage.boxed?', boxed':'')+')';
  const finish=(operation,value)=>'require_no_children(&mut children)?; Ok(NativeConstructorValue::'+operation.storage.carrier+'('+(operation.storage.boxed?'Box::new('+value+')':value)+'))';
  const constructFields=(operation,fields,constructor)=>{
    const lines=fields.map((field,index)=>'let field_'+index+' = '+take(field.typeReference)+';');
    return[...lines,finish(operation,constructor+' { '+fields.map((field,index)=>field.name+': field_'+index).join(', ')+' }')].join('\n');
  };
  for(let offset=0;offset<inventory.operations.length;offset+=24){
    const rows=inventory.operations.slice(offset,offset+24),number=parts.length,functions=[];
    for(let local=0;local<rows.length;local++){
      const operation=rows[local],index=offset+local,extra=[],borrowScalar=borrowsScalar(operation);let body;
      if(operation.variants){
        body=requireEmpty.replace('require_empty_choice(choice)?; ','')+'\nmatch choice {\n';
        for(let variantIndex=0;variantIndex<operation.variants.length;variantIndex++){
          const variant=operation.variants[variantIndex],name='construct_variant_'+index+'_'+variantIndex;
          body+=rustLiteral(variant.identity)+' => '+name+'(children),\n';
          let variantBody;
          if(variant.fields)variantBody=constructFields(operation,variant.fields,operation.storage.rust+'::'+variant.name);
          else{
            const fields=variant.tuple.map((field,index)=>'let field_'+index+' = '+take(field.typeReference)+';');
            variantBody=[...fields,finish(operation,operation.storage.rust+'::'+variant.name+'('+variant.tuple.map((_,index)=>'field_'+index).join(', ')+')')].join('\n');
          }
          extra.push('fn '+name+'(children: Vec<NativeConstructorSlot>) -> Result<NativeConstructorValue, String> {\nlet mut children = children.into_iter();\n'+variantBody+'\n}');
        }
        body+='_ => Err(fail("unknown source-qualified native variant")),\n}';
      }else if(operation.fields)body=requireEmpty+'\n'+constructFields(operation,operation.fields,operation.storage.rust);
      else if(operation.form==='Option'){
        body='require_no_scalar(scalar)?;\nmatch choice {\n"None" => { '+finish(operation,'None')+' },\n"Some" => { let value = '+take(operation.arguments[0])+'; '+finish(operation,'Some(value)')+' },\n_ => Err(fail("an explicit native Some or None choice is required")),\n}';
      }else if(operation.form==='Vec')body=requireEmpty+'\nlet mut values = Vec::with_capacity(children.len());\nwhile !children.as_slice().is_empty() { values.push('+take(operation.arguments[0])+'); }\n'+finish(operation,'values');
      else if(operation.form==='IndexMap')body=requireEmpty+'\nlet mut values = indexmap::IndexMap::new();\nwhile !children.as_slice().is_empty() { let key = '+take(operation.arguments[0])+'; let value = '+take(operation.arguments[1])+'; if values.insert(key, value).is_some() { return Err(fail("duplicate native map key")); } }\n'+finish(operation,'values');
      else if(['named','type_item','Box'].includes(operation.form)){
        const reference=operation.child??operation.arguments[0],value=operation.form==='Box'?'Box::new(value)':'value';
        body=requireEmpty+'\nlet value = '+take(reference)+';\n'+finish(operation,value);
      }else if(operation.form==='reference')body='require_empty_choice(choice)?;\nlet text = '+take(operation.child)+';\nlet Some(NativeConstructorValue::ExistingStaticReference(value)) = scalar else { return Err(fail("an actual existing static reference is required")); };\nlet value = *value;\nif value != text { return Err(fail("the existing static reference differs from its owned text")); }\n'+finish(operation,'value');
      else body='require_empty_choice(choice)?;\nlet Some(NativeConstructorValue::'+operation.storage.carrier+'(value)) = scalar else { return Err(fail("wrong native scalar storage type")); };\n'+finish(operation,borrowScalar?'*value':'value');
      functions.push('fn construct_'+index+'(choice: &str, children: Vec<NativeConstructorSlot>, scalar: Option<'+(borrowScalar?'&':'')+'NativeConstructorValue>) -> Result<NativeConstructorValue, String> {\n'+(operation.variants?'':'let mut children = children.into_iter();\n')+body+'\n}\n'+extra.join('\n'));
    }
    const source='// Generated typed host constructors from complete actual source declarations.\nuse super::*;\npub(super) fn construct(index: usize, choice: &str, children: Vec<NativeConstructorSlot>, scalar: Option<NativeConstructorValue>) -> Result<NativeConstructorValue, String> {\nmatch index {\n'+rows.map((operation,local)=>(offset+local)+' => construct_'+(offset+local)+'(choice, children, scalar'+(borrowsScalar(operation)?'.as_ref()':'')+'),').join('\n')+'\n_ => Err(fail("invalid constructor partition")),\n}\n}\n'+functions.join('\n');
    parts.push('part_'+number);files.set('part_'+number+'.rs',source);
  }
  const source=`// Generated typed host storage, not an owned-model validator or lifetime allocator.
/// Actual native storage values. Source identities remain on their separate slots.
#[derive(Debug)]
pub enum NativeConstructorValue {
${inventory.storage.map(row=>'    /// Native storage for `'+row.rust+'`.\n    '+row.carrier+'('+(row.boxed?'Box<'+row.rust+'>':row.rust)+'),').join('\n')}
}
impl NativeConstructorValue {
    /// The actual native storage type; this is not a concept equivalence proof.
    pub const fn storage_type(&self) -> &'static str {
        match self { ${inventory.storage.map(row=>'Self::'+row.carrier+'(_) => '+rustLiteral(row.rust)+',').join('\n')} }
    }
}
/// A moved typed value together with its exact source-qualified type occurrence.
#[derive(Debug)]
pub struct NativeConstructorSlot {
    /// Source identity, independently of the shared native storage carrier.
    pub type_reference: &'static str,
    /// The actual native value.
    pub value: NativeConstructorValue,
}
fn fail(message: &str) -> String { format!("native source storage: {message}") }
fn require_empty_choice(choice: &str) -> Result<(), String> { if choice.is_empty() { Ok(()) } else { Err(fail("unexpected constructor choice")) } }
fn require_no_scalar(value: Option<NativeConstructorValue>) -> Result<(), String> { value.map_or(Ok(()), |value| { drop(value); Err(fail("unexpected scalar input")) }) }
fn require_no_children(children: &mut std::vec::IntoIter<NativeConstructorSlot>) -> Result<(), String> { if children.next().is_none() { Ok(()) } else { Err(fail("extra source child")) } }
macro_rules! take_child {
    ($children:expr, $source:expr, $variant:ident, boxed) => { *take_child!($children, $source, $variant) };
    ($children:expr, $source:expr, $variant:ident) => {{
        let slot = $children.next().ok_or_else(|| fail("missing source child"))?;
        if slot.type_reference != $source { return Err(fail("foreign or reordered source child")); }
        match slot.value { NativeConstructorValue::$variant(value) => value, _ => return Err(fail("wrong source child storage type")) }
    }};
}
${parts.map(name=>'mod '+name+';').join('\n')}
/// Every captured source type occurrence, in byte-sorted order.
pub const SOURCE_REFERENCES: &[&str] = &[
${inventory.operations.map(row=>rustLiteral(row.sourceReference)+',').join('\n')}
];
/// Pack actual typed children using the exact source constructor and field order.
/// Scalars are already native; this function does not recreate their provenance.
pub fn construct_source_slot(reference: &str, choice: &str, children: Vec<NativeConstructorSlot>, scalar: Option<NativeConstructorValue>) -> Result<NativeConstructorSlot, String> {
    if children.len() > 100_000 { return Err(fail("source child storage budget")); }
    let index = SOURCE_REFERENCES.binary_search(&reference).map_err(|_| fail("unknown source type occurrence"))?;
    let value = match index / 24 {
${parts.map((name,index)=>index+' => '+name+'::construct(index, choice, children, scalar)?,').join('\n')}
        _ => return Err(fail("invalid constructor partition")),
    };
    Ok(NativeConstructorSlot { type_reference: SOURCE_REFERENCES[index], value })
}
`;
  files.set('mod.rs',source);return files;
}

/** Shared finite construction controls, without executing any native code. */
export function buildNativeConstructorStorageControls(inventory) {
  const byId=new Map(inventory.operations.map((row,index)=>[row.sourceReference,{...row,ordinal:index+1}]));
  const defaults=new Map(),active=new Set(),staticReferenceText='constructor static reference';
  const recipe=reference=>{
    if(defaults.has(reference))return defaults.get(reference);
    assert.ok(!active.has(reference),'the finite default constructor control must terminate');active.add(reference);
    const operation=byId.get(reference);assert.ok(operation);let choice='',references=[],scalar=null;
    if(operation.variants){const variant=operation.variants[0];choice=variant.identity;references=(variant.fields??variant.tuple).map(row=>row.typeReference);}
    else if(operation.fields)references=operation.fields.map(row=>row.typeReference);
    else if(operation.form==='Option')choice='None';
    else if(['named','type_item','Box','reference'].includes(operation.form))references=[operation.child??operation.arguments[0]];
    if(['bool','usize','i64','String','str','json','reference'].includes(operation.form)){
      const kind=operation.storage.carrier;
      scalar={kind,value:operation.form==='bool'?operation.ordinal%2===0:operation.form==='usize'?String(operation.ordinal):operation.form==='i64'?String(-operation.ordinal):
        operation.form==='json'?{source:reference,nested:[null,false,'text']}:['str','reference'].includes(operation.form)?staticReferenceText:'source text '+operation.ordinal};
    }
    const result={reference,choice,children:references.map(recipe),scalar};defaults.set(reference,result);active.delete(reference);return result;
  };
  const cases=[];
  for(const operation of inventory.operations){
    const base=recipe(operation.sourceReference),choices=[base];
    if(operation.form==='Option')choices.push({...base,choice:'Some',children:[recipe(operation.arguments[0])]});
    if(operation.form==='Vec')choices.push({...base,children:[recipe(operation.arguments[0])]});
    if(operation.form==='IndexMap')choices.push({...base,children:operation.arguments.map(recipe)});
    for(const variant of operation.variants?.slice(1)??[])choices.push({...base,choice:variant.identity,children:(variant.fields??variant.tuple).map(row=>recipe(row.typeReference))});
    for(const selected of choices)cases.push({sourceReference:operation.sourceReference,expectedStorageType:operation.storage.rust,recipe:selected});
  }
  return{schemaVersion:1,planSha256:inventory.planSha256,sourceReferences:inventory.operations.map(row=>row.sourceReference),staticReferenceText,cases,
    boundary:'Finite independent typed scalar inputs and complete captured constructor-branch controls. These recipes are reusable data, not evidence that native execution occurred, that an arbitrary owned model is valid or that public CST reconstruction is integrated.'};
}

export function generateNativeConstructorStorage({check=false}={}) {
  const authority=extendRecursiveInversePlan(captureOrdinaryInversePlan()),inventory=readNativeConstructorStorageInventory(authority);
  const directory=new URL('./native-constructor-storage/',import.meta.url);if(!check)mkdirSync(directory,{recursive:true});
  const outputs=[];
  for(const[file,source]of renderNativeConstructorStorage(inventory)){
    const formatted=execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],{input:source,encoding:'utf8',maxBuffer:4*1024*1024});
    const path=new URL(file,directory);if(check)assert.equal(readFileSync(path,'utf8'),formatted);else writeFileSync(path,formatted);
    outputs.push({file:'experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/'+file,...pin(formatted)});
  }
  const receipt={schemaVersion:1,author:{file:'experiments/grammar-outcome-network/native-transport/inverse/generate-native-constructor-storage.mjs',...pin(readFileSync(fileURLToPath(import.meta.url)))},inventory,outputs,
    nativeCompilation:false,nativeExecution:false,boundary:inventory.boundary};
  const target=new URL('../../../../parity/self-translation/native-constructor-storage.lino',import.meta.url);
  const controls=buildNativeConstructorStorageControls(inventory),fixture=new URL('../../../../parity/fixtures/native-constructor-storage.lino',import.meta.url);
  if(check){checkProjectData(target,receipt);checkProjectData(fixture,controls);}else{writeProjectData(target,receipt);writeProjectData(fixture,controls);}return receipt;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--check','--write'].includes(process.argv[2]));
  const result=generateNativeConstructorStorage({check:process.argv[2]==='--check'});
  console.log('Derived '+result.inventory.operations.length+' source-qualified native storage constructors from all nine declarations; no native compilation occurred.');
}
