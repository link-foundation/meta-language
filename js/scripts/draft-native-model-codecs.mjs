// Generate test-wire transport from actual checked declarations and actual
// target mappings. This never executes represented operations or fixtures.
import {writeFileSync} from 'node:fs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {writeProjectData} from './project-data.mjs';
import {createHash} from 'node:crypto';
import {draftJavaScriptModelCodecs} from './draft-javascript-model-codecs.mjs';
import {modelTextWirePolicy} from './model-text-wire.mjs';
export function draftNativeModelCodecs({program,emitted,roots,wireRules,output,receipt,javascriptOutput}){
 const root=new URL('../../',import.meta.url);
 if(program.sourceLanguage!=='JavaScript')throw new TypeError('this separately qualified model transport uses the checked JavaScript tuple-constructor layout');
 if(!Array.isArray(roots)||roots.length===0||roots.length>16||!/^rust\/tests\/unit\/[a-z][a-z0-9_]*\.rs$/u.test(output)||!/^parity\/self-translation\/[a-z][a-z0-9-]*\.lino$/u.test(receipt))throw new TypeError('explicit bounded model codec roots and owned outputs are required');
 const data=new Map([...program.declarations].filter(([,entry])=>entry.k==='data'));
 const mapping=(kind,name)=>{const row=emitted.mappings.find(row=>row.kind===kind&&row.source===name);if(!row||!/^[A-Za-z_][A-Za-z0-9_.]*$/u.test(row.target))throw new TypeError('a native model needs its actual declared target mapping: '+name);return row.target.replaceAll('.','::');};
 const q=value=>{if(typeof value!=='string'||!/^[\x20-\x21\x23-\x5b\x5d-\x7e]*$/u.test(value))throw new TypeError('this model transport needs separately qualified non-ASCII/escaped schema names');return '"'+value+'"';};
 const snake=name=>name.replaceAll('::','_').replace(/([a-z0-9])([A-Z])/gu,'$1_$2').toLowerCase();
 const names=new Map([...data].map(([name])=>[name,snake(mapping('data',name))]));
 const utf16=emitted.encodings?.some(row=>row.id==='utf16-strings')===true;
 const selected=new Map();
 function collect(type,depth=0){
  if(depth>64)throw new RangeError('model codec type depth exceeds its explicit budget');
  if(type.kind==='data'){
   if(selected.has(type.name))return;const declaration=data.get(type.name);if(!declaration||declaration.ctors.length===0||declaration.ctors.length>256)throw new TypeError('a complete bounded native declaration is required');
   selected.set(type.name,declaration);for(const ctor of declaration.ctors){if(ctor.fields.length>128)throw new RangeError('model codec field budget');for(const field of ctor.fields)collect(field.type,depth+1);}return;
  }
  if(type.kind==='array'){if(type.reference)throw new TypeError('mutable array wire transport requires its actual identity model');collect(type.element,depth+1);return;}
  if(type.kind==='record'){if(type.reference||type.ordered||type.fields.length>128)throw new TypeError('noncanonical record wire layout needs a separate profile');for(const field of type.fields)collect(field.type,depth+1);return;}
  if(!['bool','float','int','nat','string','unit'].includes(type.kind))throw new TypeError('unregistered model wire type '+type.kind);
 }
 for(const name of roots){if(!data.has(name))throw new TypeError('model root does not identify its actual declaration: '+name);collect({kind:'data',name});}
 if(new Set([...selected.keys()].map(name=>names.get(name))).size!==selected.size)throw new TypeError('different actual native owners need distinct wire function identities');
 const overrides=wireRules.map(rule=>{
  if(rule.codec!=='ieee754-binary64-hex')throw new TypeError('unregistered native model wire codec');
  const declaration=data.get(rule.owner),ctor=declaration?.ctors.find(ctor=>ctor.name===rule.constructor),field=ctor?.fields.find(field=>field.name===rule.field);
  if(!field||field.type.kind!=='float'||!selected.has(rule.owner))throw new TypeError('a wire override must target its actual selected primitive field');q(rule.wire);return rule;
 });
 if(new Set(overrides.map(rule=>rule.owner+'\0'+rule.constructor+'\0'+rule.field)).size!==overrides.length)throw new TypeError('a model field can have only one qualified wire override');
 function decode(type,value,borrowed=false){switch(type.kind){
  case 'bool':return value+'.as_bool().expect("model Boolean")';
  case 'float':return value+'.as_f64().expect("model Number")';
  case 'int':case 'nat':return 'native::ml::Big::parse('+value+'.as_str().expect("model exact integer"))';
  case 'string':return utf16?'native::ml_string::Text::from_units(wire_text_units(&'+value+'))':'String::from_utf16(&wire_text_units(&'+value+')).expect("model text requires Unicode scalar values in this native profile")';
  case 'unit':return '()';
  case 'data':return 'decode_'+names.get(type.name)+'('+(borrowed?'':'&')+value+')';
  case 'array':return value+'.as_array().expect("model array").iter().map('+(type.element.kind==='data'?'decode_'+names.get(type.element.name):'|wire_value|'+decode(type.element,'wire_value',true))+').collect()';
  case 'record':return '('+type.fields.map(field=>decode(field.type,value+'['+q(field.name)+']')).join(',')+',)';
  default:throw new TypeError('unknown model wire type');}}
 function encode(type,value,borrowed=true){switch(type.kind){
  case 'bool':return 'Data::Bool('+(borrowed?'*':'')+value+')';
  case 'float':return 'wire_number('+(borrowed?'*':'')+value+')';
  case 'int':case 'nat':return 'Data::String('+value+'.to_string())';
  case 'string':return utf16?'wire_text(&'+value+'.0)':'wire_text(&'+value+'.encode_utf16().collect::<Vec<u16>>())';
  case 'unit':return 'Data::Null';
  case 'data':return 'encode_'+names.get(type.name)+'('+(borrowed?'':'&')+value+')';
  case 'array':return 'Data::Array('+value+'.iter().map('+(type.element.kind==='data'?'encode_'+names.get(type.element.name):'|wire_value|'+encode(type.element,'wire_value'))+').collect())';
  case 'record':return 'json!({'+type.fields.map((field,index)=>q(field.name)+':'+encode(field.type,value+'.'+index,false)).join(',')+'})';
  default:throw new TypeError('unknown model wire type');}}
 let source='// Generated from actual checked source declarations and target mappings.\n// Transport only; no source operation semantics are implemented here.\nuse super::native;\nuse serde_json::{Value as Data,json};\nfn wire_number(value:f64)->Data {\n if value.to_bits()==0x8000_0000_0000_0000 {return Data::Number(serde_json::Number::from_f64(value).unwrap());}\n Data::Number(value.to_string().parse().expect("finite model Number"))\n}\n';
 source+=`fn wire_text_units(value:&Data)->Vec<u16> {
 let units=if let Some(text)=value.as_str(){text.encode_utf16().collect::<Vec<u16>>()}else{
 let object=value.as_object().expect("qualified model text units");
 assert_eq!(object.len(),2,"exact model text keys");
 assert_eq!(object.get("textEncoding").and_then(Data::as_str),Some(${q(modelTextWirePolicy.tag)}),"model text encoding");
 let input=object.get("units").and_then(Data::as_array).expect("model text units");
 assert!(input.len()<=${modelTextWirePolicy.maximumUnits},"model text unit budget");
 input.iter().map(|unit|u16::try_from(unit.as_u64().expect("model UTF16 unit")).expect("model UTF16 unit range")).collect()
 };
 assert!(units.len()<=${modelTextWirePolicy.maximumUnits},"model text unit budget");units
}
fn wire_text(units:&[u16])->Data {
 assert!(units.len()<=${modelTextWirePolicy.maximumUnits},"model text unit budget");
 match String::from_utf16(units){Ok(text)=>Data::String(text),Err(_)=>json!({"textEncoding":${q(modelTextWirePolicy.tag)},"units":units})}
}
`;
 writeProjectData(new URL('parity/translation/model-text-wire.lino',root),{schemaVersion:1,...modelTextWirePolicy});
 const layout=[];
 for(const [name,declaration]of selected){
  const native='native::'+mapping('data',name),decoded=[],encoded=[];
  for(const ctor of declaration.ctors){
   const keys=['$',...ctor.fields.map(field=>overrides.find(rule=>rule.owner===name&&rule.constructor===ctor.name&&rule.field===field.name)?.wire??field.name)];
   if(new Set(keys).size!==keys.length)throw new TypeError('different declared fields cannot collapse to one wire key');
   const target='native::'+mapping('constructor',name+'.'+ctor.name),fields=[],values=[];
   for(const [index,field]of ctor.fields.entries()){
    const override=overrides.find(rule=>rule.owner===name&&rule.constructor===ctor.name&&rule.field===field.name);
    let decoded=override?'f64::from_bits(u64::from_str_radix(data['+q(override.wire)+'].as_str().expect("model IEEE bits"),16).expect("complete model IEEE bits"))':decode(field.type,'data['+q(field.name)+']');
    if(field.type.kind==='data')decoded='Box::new('+decoded+')';fields.push(decoded);
    values.push(q(override?.wire??field.name)+':'+(override?'Data::String(format!("{:016x}",wire_field_'+index+'.to_bits()))':encode(field.type,'wire_field_'+index)));
   }
   decoded.push(q(ctor.name)+'=>'+target+(fields.length?'('+fields.join(',')+')':''));
   encoded.push(target+(fields.length?'('+fields.map((_,index)=>'wire_field_'+index).join(',')+')':'')+'=>json!({"$":'+q(ctor.name)+(values.length?','+values.join(','):'')+'})');
  }
  source+='pub(super) fn decode_'+names.get(name)+'(data:&Data)->'+native+' {match data["$"].as_str().expect("model constructor"){'+decoded.join(',')+',_=>panic!("unregistered actual model constructor")}}\n';
  source+='pub(super) fn encode_'+names.get(name)+'(value:&'+native+')->Data {match value{'+encoded.join(',')+'}}\n';
  layout.push({name,target:mapping('data',name),constructors:declaration.ctors.map(ctor=>({name:ctor.name,target:mapping('constructor',name+'.'+ctor.name),fields:ctor.fields}))});
 }
 source=formatProductRuleSource(source);writeFileSync(new URL(output,root),source);
 const javascript=javascriptOutput===undefined?undefined:draftJavaScriptModelCodecs({selected,overrides,output:javascriptOutput});
 writeProjectData(new URL(receipt,root),{schemaVersion:1,model:'checked native nominal model wire transport 1',roots,wireRules,textWire:modelTextWirePolicy,layout,output,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex'),
  ...(javascript?{javascript}:{}),boundary:'Actual source type/constructor/target transport only, preserving explicit IEEE-bit overrides. No fixture decoding, modeled operation, behavior comparison, native compilation or execution observation.'});
}
