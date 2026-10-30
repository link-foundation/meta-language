// Type-directed UTF8 source/schema I/O facades. These call the actual generated
// operations; no source rendering/selection semantics are implemented in Rust.
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {writeFileSync} from 'node:fs';
import {writeProjectData} from './project-data.mjs';
import {createHash} from 'node:crypto';
export function draftNativeSourceInterfaces({program,emitted,definition,module,output,receipt}){
 if(definition.schemaVersion!==1||definition.model!=='native Rust source Unicode scalar interface 1'||!Array.isArray(definition.functions)||definition.functions.length===0||definition.functions.length>64
  ||definition.module!==module||!/^[a-z][a-z0-9_]*$/u.test(module)||!/^rust\/src\/translation\/[a-z][a-z0-9_]*\.rs$/u.test(output))throw new TypeError('a separately qualified bounded source interface definition is required');
 const utf16=emitted.encodings.some(row=>row.id==='utf16-strings'),seen=new Set(),rows=[];
 let source='// Generated from actual checked source function signatures and target mappings.\n// UTF8 source/schema fragments only; runtime text retains its UTF16 interface.\nuse crate::translation::'+definition.module+' as implementation;\n';
 for(const selected of definition.functions){
  if(seen.has(selected.source))throw new TypeError('duplicate source interface owner');seen.add(selected.source);
  const declaration=program.declarations.get(selected.source),mapping=emitted.mappings.find(row=>row.kind==='function'&&row.source===selected.source);
  const scalar={bool:'bool',float:'f64',string:'String'},result=declaration?.ret;
  const resultArray=result?.kind==='array'&&!result.reference&&['float','string'].includes(result.element.kind);
  if(declaration?.k!=='fn'||!mapping||!/^[a-z][a-z0-9_]*$/u.test(mapping.target)||!scalar[result.kind]&&!resultArray)throw new TypeError('unsupported actual source interface: '+selected.source);
  const parameters=[],bindings=[],arguments_=[];
  for(const [index,parameter]of declaration.params.entries()){
   const name='source_argument_'+index,type=parameter.type;
   if(type.kind==='bool'||type.kind==='float'){parameters.push(name+':'+scalar[type.kind]);arguments_.push(name);}
   else if(type.kind==='string'){
    parameters.push(name+':&str');
    if(utf16){bindings.push('let native_argument_'+index+'=implementation::ml_string::Text::from_text('+name+');');arguments_.push('&native_argument_'+index);}else arguments_.push(name);
   }else if(type.kind==='array'&&['string','float'].includes(type.element.kind)&&!type.reference){
    parameters.push(name+':&['+scalar[type.element.kind]+']');
    if(utf16&&type.element.kind==='string'){bindings.push('let native_argument_'+index+'='+name+'.iter().map(|value|implementation::ml_string::Text::from_text(value)).collect::<Vec<_>>();');arguments_.push('&native_argument_'+index);}else arguments_.push(name);
   }else throw new TypeError('unregistered source interface operand '+type.kind);
  }
  const call='implementation::'+mapping.target+'('+arguments_.join(',')+')';
  const checked='String::from_utf16(&value.0).expect("a native source operation must preserve Unicode scalar spelling")';
  const body=result.kind==='string'&&utf16?'let value='+call+'; '+checked
   :resultArray&&result.element.kind==='string'&&utf16?call+'.into_iter().map(|value|'+checked+').collect::<Vec<_>>()':call;
  source+='pub(crate) fn '+mapping.target+'('+parameters.join(',')+')->'+(resultArray?'Vec<'+scalar[result.element.kind]+'>':scalar[result.kind])+'{'+bindings.join('\n')+body+'}\n';
  rows.push({source:selected.source,target:mapping.target,parameters:declaration.params,result:declaration.ret});
 }
 source=formatProductRuleSource(source);const root=new URL('../../',import.meta.url);writeFileSync(new URL(output,root),source);
 writeProjectData(new URL(receipt,root),{schemaVersion:1,model:definition.model,module:definition.module,utf16Implementation:utf16,functions:rows,output,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex'),
  boundary:'Actual generated function calls and checked Unicode-scalar source/schema transport only. No runtime UTF16-value conversion, replacement rendering semantics, fixture execution, native compilation or observation.'});
}
