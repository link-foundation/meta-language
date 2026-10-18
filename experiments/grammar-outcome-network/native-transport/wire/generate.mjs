// PRIVATE source-CST transport producer. It never runs rustfmt or Cargo.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {outcomeWirePolicy as policy,repository} from './wire-policy.mjs';
import {explicitOutputDirectory} from './output.mjs';

const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const tokenText = source => tokenize(source,'Rust').tokens.filter(token=>token.kind!=='eof').map(token=>token.value).join('');
const snake = name => name.replace(/([a-z0-9])([A-Z])/gu,'$1_$2').toLowerCase();
const ignored = new Set(['line_comment','block_comment','attribute_item','visibility_modifier']);
const names = policy.sources.flatMap(source=>source.names);
const pathOf = name => {assert.ok(names.includes(name),'unknown crate-root owner');return 'crate::'+name;};
const same = (actual,expected,label) => {
  assert.equal(new Set(actual).size,actual.length,label+' has no duplicate members');
  assert.deepEqual([...actual].sort(),[...expected].sort(),label+' covers every source member');
};

/** Read complete actual source CSTs; never copy a native declaration by hand. */
export function readOutcomeInventory(root=repository) {
  const pins=[];
  for(const [file,sha256] of Object.entries(policy.pins)) {
    const bytes=readFileSync(resolve(root,file));
    assert.equal(digest(bytes),sha256,'reviewed complete input changed: '+file);
    pins.push({file,bytes:bytes.length,sha256});
  }
  const inputs=[],declarations=[];
  for(const source of policy.sources) {
    const bytes=readFileSync(resolve(root,source.file));
    assert.ok(bytes.length<=131072,'bounded complete source CST');
    const parsed=readRustSourceItems(bytes);
    const imports=parsed.items.filter(item=>item.kind==='use_declaration').map(item=>({
      source:parsed.text(item.node),span:{start:item.node.start,end:item.node.end}}));
    const protectedNames=new Set(['Vec','Option','Box','String','usize','i64','bool']);
    assert.ok(!parsed.items.some(item=>protectedNames.has(item.name)),'no local shadow of primitive transport owners');
    for(const item of imports) {
      const tokens=tokenize(item.source,'Rust').tokens.map(token=>token.value);
      assert.ok(!tokens.includes('*'),'unknown glob imports refuse');
      assert.ok(!tokens.some(token=>protectedNames.has(token)),'no imported shadow of primitive transport owners');
      assert.ok(!tokens.includes('as')||tokenText(item.source)==='usestd::fmt::Writeas_;','unknown import aliases refuse');
    }
    for(const binding of policy.bindings.filter(binding=>binding.file===source.file))
      assert.equal(imports.filter(item=>tokenText(item.source)===tokenText(binding.declaration)).length,1,'actual import/reexport binding remains owned');
    const requireText=node=>{assert.ok(node,'actual CST field required');return parsed.text(node);};
    const dataFields=body=>{
      assert.equal(body?.term,'field_declaration_list');
      const nodes=body.children.filter(({node})=>node.named&&!ignored.has(node.term)).map(({node})=>node);
      assert.ok(nodes.every(node=>node.term==='field_declaration'),'all named source fields are inventoried');
      return nodes.map(node=>({name:requireText(parsed.field(node,'name')),
        type:tokenText(requireText(parsed.field(node,'type'))),source:parsed.text(node),span:{start:node.start,end:node.end}}));
    };
    inputs.push({file:source.file,bytes:bytes.length,sha256:digest(bytes),source:bytes.toString('utf8'),imports,
      importsSha256:digest(imports.map(item=>item.source).join('\n'))});
    for(const name of source.names) {
      const found=parsed.items.filter(item=>item.name===name);
      assert.equal(found.length,1,'one source declaration owns '+name);
      const item=found[0],body=parsed.field(item.node,'body');
      const declaration={name,kind:item.kind,file:source.file,span:{start:item.start,end:item.end},
        source:bytes.subarray(item.start,item.end).toString('utf8'),sha256:digest(bytes.subarray(item.start,item.end))};
      if(item.kind==='struct_item') declaration.fields=dataFields(body);
      else if(item.kind==='enum_item') {
        assert.ok(body,'actual enum body required');
        const variants=body.children.filter(({node})=>node.named&&!ignored.has(node.term)).map(({node})=>node);
        assert.ok(variants.every(node=>node.term==='enum_variant'),'all source variants are inventoried');
        declaration.variants=variants.map(node=>{
          const payload=parsed.field(node,'body');
          const result={name:requireText(parsed.field(node,'name')),source:parsed.text(node),span:{start:node.start,end:node.end}};
          if(!payload) result.unit=true;
          else if(payload.term==='field_declaration_list') result.fields=dataFields(payload);
          else {
            assert.equal(payload.term,'ordered_field_declaration_list');
            result.tuple=payload.children.filter(({node})=>node.named&&!ignored.has(node.term)).map(({node})=>({
              type:tokenText(parsed.text(node)),source:parsed.text(node),span:{start:node.start,end:node.end}}));
          }
          return result;
        });
      } else {
        assert.equal(item.kind,'type_item','unknown declaration kinds refuse');
        declaration.alias=tokenText(requireText(parsed.field(item.node,'type')));
      }
      declarations.push(declaration);
    }
  }
  const rootBytes=readFileSync(resolve(root,'rust/src/lib.rs'));
  const rootItems=readRustSourceItems(rootBytes);
  const exports=rootItems.items.filter(item=>item.kind==='use_declaration');
  const rootExport=exports.filter(item=>tokenText(rootItems.text(item.node))===tokenText(policy.rootReexport));
  assert.equal(rootExport.length,1,'actual crate-root reexport remains complete');
  for(const name of names) assert.equal(exports.filter(item=>tokenize(rootItems.text(item.node),'Rust').tokens.some(token=>token.value===name)).length,1,'crate-root owner is unique: '+name);
  return {schemaVersion:1,model:policy.model,pins,inputs,declarations,
    rootReexport:{file:'rust/src/lib.rs',source:rootItems.text(rootExport[0].node),span:{start:rootExport[0].node.start,end:rootExport[0].node.end}}};
}

/** Parse ONLY source-owned type forms; no display/serde/debug fallback exists. */
export function readOwnedType(type,owner) {
  if(type==='Self') {assert.equal(owner,'SyntaxTree');return {kind:'owned',name:owner};}
  if(type==='super::grammar_selection::GrammarSelection') {assert.equal(owner,'SyntaxTree');return {kind:'owned',name:'GrammarSelection'};}
  if(type==='Value') {assert.ok(['GrammarSelection','GrammarSelectionAlternative'].includes(owner));return {kind:'json'};}
  if(type==='serde_json::Value') return {kind:'json'};
  if(['String',"&'staticstr"].includes(type)) return {kind:'string'};
  if(['bool','usize','i64'].includes(type)) return {kind:type};
  if(names.includes(type)) return {kind:'owned',name:type};
  const generic=/^(Vec|Box|Option)<(.+)>$/u.exec(type);
  assert.ok(generic,'unknown source-owned type: '+type);
  return {kind:generic[1],item:readOwnedType(generic[2],owner)};
}

export function checkOutcomeInventory(inventory) {
  same(inventory.declarations.map(item=>item.name),names,'declaration inventory');
  same(inventory.inputs.map(item=>item.file),policy.sources.map(item=>item.file),'complete source inventory');
  for(const input of inventory.inputs) {
    assert.equal(digest(input.source),policy.pins[input.file],'complete source authentication');
    assert.equal(digest(input.imports.map(item=>item.source).join('\n')),input.importsSha256,'complete import inventory');
  }
  assert.equal(tokenText(inventory.rootReexport.source),tokenText(policy.rootReexport),'root reexport ownership');
  const aliases=inventory.declarations.filter(item=>item.alias);
  assert.deepEqual(aliases.map(item=>[item.name,item.alias]),[['SyntaxAttributes','IndexMap<String,OperationValue>']],'only the actual IndexMap alias is admitted');
  const checkEntries=(entries,fields,owner)=>{
    same(entries.filter(entry=>!entry.literal).map(entry=>entry.native),fields.map(field=>field.name),owner+' fields');
    for(const entry of entries) {
      if(entry.literal) continue;
      const type=readOwnedType(fields.find(field=>field.name===entry.native).type,owner);
      if(entry.flatten) assert.deepEqual(type,{kind:'owned',name:'LeafText'});
      else if(['some','nullable'].includes(entry.presence)) assert.equal(type.kind,'Option','optional wire role requires actual Option');
      else if(entry.presence==='true') assert.deepEqual(type,{kind:'bool'});
      else {assert.equal(entry.presence,'always');assert.notEqual(type.kind,'Option','None omission must be explicit');}
    }
  };
  for(const shape of policy.shapes) {
    const declaration=inventory.declarations.find(item=>item.name===shape.native);
    if(shape.entries) {assert.equal(declaration.kind,'struct_item');checkEntries(shape.entries,declaration.fields,shape.native);}
    else {
      assert.equal(declaration.kind,'enum_item');
      same(shape.variants.map(item=>item.name),declaration.variants.map(item=>item.name),shape.native+' variants');
      for(const variant of shape.variants) {
        const actual=declaration.variants.find(item=>item.name===variant.name);
        if(variant.entries) checkEntries(variant.entries,actual.fields,shape.native);
        else {
          assert.equal(actual.tuple?.length,1,'all tuple payloads are owned');
          assert.equal(actual.tuple[0].type,shape.native==='OperationValue'&&variant.name==='Integer'?'i64':'String');
        }
      }
    }
  }
  return inventory;
}

/** Single inspectable ordered plan used by both control/preflight and emitter. */
export function buildOrderedWirePlan(inventory) {
  checkOutcomeInventory(inventory);
  const entries=(rows,fields,owner)=>rows.map(row=>row.literal?{...row}:{...row,
    type:readOwnedType(fields.find(field=>field.name===row.native).type,owner)});
  return {schemaVersion:1,model:policy.model,root:'ParseOutcome',attributeAlias:'SyntaxAttributes',
    shapes:policy.shapes.map(shape=>{
      const actual=inventory.declarations.find(item=>item.name===shape.native);
      return shape.entries?{native:shape.native,sourceFields:actual.fields.map(field=>field.name),entries:entries(shape.entries,actual.fields,shape.native)}
        :{native:shape.native,variants:shape.variants.map(variant=>{
          const fields=actual.variants.find(item=>item.name===variant.name);
          return variant.entries?{name:variant.name,sourceFields:fields.fields.map(field=>field.name),entries:entries(variant.entries,fields.fields,shape.native)}
            :{...variant,payloadType:readOwnedType(fields.tuple[0].type,shape.native)};
        })};
    })};
}

const sinkContract = `// PRIVATE GENERATED PROPOSAL; no concrete sink or native execution.
// One ordered borrowed visitor drives CountSink(Node=()) and DirectSink(Node=LinkId).
// Recursive control flow below is NOT an arbitrary-depth resource proof.
trait OutcomeWireSink: Sized {
    type Node;
    type Object;
    type Array;
    fn begin_object(&mut self, depth: usize) -> Result<Self::Object, String>;
    fn object_field<F>(&mut self, object: &mut Self::Object, key: &str, visit: F) -> Result<(), String>
    where F: FnOnce(&mut Self, usize) -> Result<Self::Node, String>;
    fn end_object(&mut self, object: Self::Object) -> Result<Self::Node, String>;
    fn begin_array(&mut self, count: usize, depth: usize) -> Result<Self::Array, String>;
    fn array_item<F>(&mut self, array: &mut Self::Array, visit: F) -> Result<(), String>
    where F: FnOnce(&mut Self, usize) -> Result<Self::Node, String>;
    fn end_array(&mut self, array: Self::Array) -> Result<Self::Node, String>;
    fn null(&mut self, depth: usize) -> Result<Self::Node, String>;
    fn boolean(&mut self, value: bool, depth: usize) -> Result<Self::Node, String>;
    fn string(&mut self, value: &str, depth: usize) -> Result<Self::Node, String>;
    fn usize(&mut self, value: usize, depth: usize) -> Result<Self::Node, String>;
    fn i64(&mut self, value: i64, depth: usize) -> Result<Self::Node, String>;
    fn json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<Self::Node, String>;
}
`;

/** Exact typed destructures; no Value tree/clone/serde fallback is generated. */
export function renderBorrowedVisitors(inventory,plan=buildOrderedWirePlan(inventory)) {
  checkOutcomeInventory(inventory);
  assert.deepEqual(plan,buildOrderedWirePlan(inventory),'emission consumes the exact counted plan');
  const convert=(type,expression,depth)=>{
    if(type.kind==='owned') return `wire_${snake(type.name)}(${expression}, sink, ${depth})`;
    if(type.kind==='json') return `wire_json(${expression}, sink, ${depth})`;
    if(type.kind==='string') return `sink.string(${expression}, ${depth})`;
    if(['bool','usize','i64'].includes(type.kind)) return `sink.${type.kind==='bool'?'boolean':type.kind}(*${expression}, ${depth})`;
    if(type.kind==='Box') return convert(type.item,`${expression}.as_ref()`,depth);
    if(type.kind==='Vec') return `{ let mut array = sink.begin_array(${expression}.len(), ${depth})?;
        for item in ${expression} { sink.array_item(&mut array, |sink, depth| ${convert(type.item,'item','depth')})?; }
        sink.end_array(array) }`;
    throw new TypeError('unsupported conversion role: '+type.kind);
  };
  const fields=(entries,object)=>{
    const output=[];
    for(const entry of entries) {
      if(entry.literal) {
        assert.equal(typeof entry.value,'string');
        output.push(`sink.object_field(${object}, ${JSON.stringify(entry.key)}, |sink, depth| sink.string(${JSON.stringify(entry.value)}, depth))?;`);
      } else if(entry.flatten) output.push(`wire_leaf_text(${entry.native}, sink, ${object})?;`);
      else {
        const optional=['some','nullable'].includes(entry.presence),type=optional?entry.type.item:entry.type;
        const expression=optional?'item':entry.native;
        const body=`sink.object_field(${object}, ${JSON.stringify(entry.key)}, |sink, depth| ${convert(type,expression,'depth')})?;`;
        if(entry.presence==='some') output.push(`if let Some(item) = ${entry.native} { ${body} }`);
        else if(entry.presence==='nullable') output.push(`sink.object_field(${object}, ${JSON.stringify(entry.key)}, |sink, depth| match ${entry.native} {
            Some(item) => ${convert(type,'item','depth')}, None => sink.null(depth), })?;`);
        else if(entry.presence==='true') output.push(`if *${entry.native} { ${body} }`);
        else output.push(body);
      }
    }
    return output.join('\n');
  };
  const object=entries=>`let mut object = sink.begin_object(depth)?;\n${fields(entries,'&mut object')}\nsink.end_object(object)`;
  const functions=[];
  for(const shape of plan.shapes) {
    const declaration=inventory.declarations.find(item=>item.name===shape.native),path=pathOf(shape.native);
    if(shape.native==='LeafText') {
      const arms=shape.variants.map(variant=>`${path}::${variant.name}(payload) => { ${variant.flatten.map(entry=>
        `sink.object_field(object, ${JSON.stringify(entry.key)}, |sink, depth| ${entry.literal?'sink.null(depth)':'sink.string(payload, depth)'})?;`).join('\n')} }`);
      functions.push(`fn wire_leaf_text<S: OutcomeWireSink>(value: &${path}, sink: &mut S, object: &mut S::Object) -> Result<(), String> {
        match value { ${arms.join(',\n')} }\nOk(())\n}`);
      continue;
    }
    const body=shape.entries?`let ${path} { ${declaration.fields.map(field=>field.name).join(', ')} } = value;\n${object(shape.entries)}`
      :`match value {\n${shape.variants.map(variant=>{
        const actual=declaration.variants.find(item=>item.name===variant.name);
        return variant.entries?`${path}::${variant.name} { ${actual.fields.map(field=>field.name).join(', ')} } => { ${object(variant.entries)} }`
          :`${path}::${variant.name}(payload) => ${convert(variant.payloadType,'payload','depth')}`;
      }).join(',\n')}\n}`;
    functions.push(`fn wire_${snake(shape.native)}<S: OutcomeWireSink>(value: &${path}, sink: &mut S, depth: usize) -> Result<S::Node, String> {\n${body}\n}`);
  }
  functions.push(`fn wire_syntax_attributes<S: OutcomeWireSink>(value: &crate::SyntaxAttributes, sink: &mut S, depth: usize) -> Result<S::Node, String> {
    let mut object = sink.begin_object(depth)?;
    for (key, item) in value { sink.object_field(&mut object, key, |sink, depth| wire_operation_value(item, sink, depth))?; }
    sink.end_object(object)
}`);
  functions.push(`fn wire_json<S: OutcomeWireSink>(value: &serde_json::Value, sink: &mut S, depth: usize) -> Result<S::Node, String> {
    match value {
        serde_json::Value::Null => sink.null(depth),
        serde_json::Value::Bool(value) => sink.boolean(*value, depth),
        serde_json::Value::Number(value) => sink.json_number(value, depth),
        serde_json::Value::String(value) => sink.string(value, depth),
        serde_json::Value::Array(values) => {
            let mut array = sink.begin_array(values.len(), depth)?;
            for item in values { sink.array_item(&mut array, |sink, depth| wire_json(item, sink, depth))?; }
            sink.end_array(array)
        },
        serde_json::Value::Object(values) => {
            let mut object = sink.begin_object(depth)?;
            for (key, item) in values { sink.object_field(&mut object, key, |sink, depth| wire_json(item, sink, depth))?; }
            sink.end_object(object)
        },
    }
}`);
  const output=sinkContract+'\n'+functions.join('\n\n')+'\n';
  assert.ok(!output.includes('..')&&!output.includes('_ =>'),'no rest/wildcard patterns');
  assert.ok(!output.includes('.clone(')&&!output.includes('from_json')&&!output.includes('to_value')&&!output.includes(' as f64'),'no copied intermediate or alternate numeric conversion');
  return output;
}

export function buildOutcomePacket(root=repository) {
  const inventory=readOutcomeInventory(root),plan=buildOrderedWirePlan(inventory),source=renderBorrowedVisitors(inventory,plan);
  return {inventory,plan,source,receipt:{schemaVersion:1,model:policy.model,declarations:inventory.declarations.length,
    inputs:inventory.pins,sourceBytes:Buffer.byteLength(source),sourceSha256:digest(source),
    generatorSha256:digest(readFileSync(fileURLToPath(import.meta.url))),policySha256:digest(readFileSync(new URL('./wire-policy.mjs',import.meta.url))),
    unresolved:['concrete counted/direct Encoder sinks','bounded native explicit-frame traversal','allocation and numeric scratch reservations','native compilation and parity execution']}};
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  const directory=explicitOutputDirectory(process.argv.slice(2));
  const packet=buildOutcomePacket();
  // Generated proposals and receipts remain in explicitly selected task scratch.
  writeFileSync(resolve(directory,'borrowed_outcome_visitors.rs'),packet.source);
  writeProjectData(resolve(directory,'wire-policy.lino'),policy);
  writeProjectData(resolve(directory,'inventory.lino'),packet.inventory);
  writeProjectData(resolve(directory,'ordered-plan.lino'),packet.plan);
  writeProjectData(resolve(directory,'receipt.lino'),packet.receipt);
  console.log('Prepared private source-only event plan for '+packet.inventory.declarations.length+' declarations; no native sink is implemented.');
}
