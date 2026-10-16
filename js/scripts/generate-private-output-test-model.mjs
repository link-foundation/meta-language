// Build the isolated native test model from actual owned Rust CST items.
// This copies declarations; it is not a claim of semantic self-translation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {existsSync,readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {dirname,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {nativeGrammarText,parseNative} from '../src/native-grammar-parser.js';
import {readProjectData,writeProjectData} from './project-data.mjs';

const root=fileURLToPath(new URL('../..',import.meta.url));
const policyFile='parity/self-translation/private-output-test-model.lino';
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');

export function readRustSourceItems(bytes){
  const tree=parseNative('native-rust',bytes.toString('utf8'));
  assert.equal(tree.term,'source_file');assert.equal(tree.hasError,false,'owned Rust input must have a complete native CST');
  assert.equal(tree.end,bytes.length,'the native CST covers the entire input');
  const text=node=>bytes.subarray(node.start,node.end).toString('utf8');
  const field=(node,name)=>node.children.find(child=>child.field===name)?.node;
  const items=[];let preceding=null;
  for(const {node}of tree.children){
    if(['line_comment','block_comment','attribute_item','inner_attribute_item'].includes(node.term)){
      preceding??=node.start;continue;
    }
    items.push({kind:node.term,name:field(node,'name')?text(field(node,'name')):null,start:preceding??node.start,end:node.end,node});
    preceding=null;
  }
  return {items,text,field};
}

export function buildPrivateOutputTestModel(repository=root,policy=readProjectData(resolve(repository,policyFile))){
  assert.equal(policy.schemaVersion,1);const cache=new Map(),inputs=[],bindings=[],sections=[],modules=[];
  const source=file=>{
    if(!cache.has(file)){
      const bytes=readFileSync(resolve(repository,file));assert.ok(bytes.length<=131072);
      const parsed=readRustSourceItems(bytes);cache.set(file,{...parsed,bytes});inputs.push({file,bytes:bytes.length,sha256:digest(bytes)});
    }
    return cache.get(file);
  };
  const select=(parsed,selector)=>{
    const found=parsed.items.filter(item=>item.kind===selector.kind&&item.name===selector.name);
    assert.equal(found.length,1,'each source item selector owns one declaration');return found[0];
  };
  const bind=(module,parsed,items)=>{
    const name=node=>{const value=parsed.field(node,'name');assert.ok(value,'source declaration has a named field');return parsed.text(value);};
    const descend=(node,kind)=>node.children.flatMap(({node:child})=>child.term===kind?[child]:descend(child,kind));
    for(const {node,kind}of items){
      if(kind==='function_item'&&node.children.some(({node})=>node.term==='visibility_modifier'))bindings.push(`let _ = ${module}::${name(node)};`);
      if(kind==='enum_item'){
        const variants=descend(node,'enum_variant'),arms=[];
        for(const variant of variants){
          const path=`${module}::${name(node)}::${name(variant)}`,body=parsed.field(variant,'body');
          bindings.push(`let _ = ${path};`);
          if(body){
            assert.equal(body.term,'ordered_field_declaration_list','enum payload binding uses complete tuple fields');
            const fields=body.children.filter(({node})=>node.named&&!['attribute_item','visibility_modifier'].includes(node.term)).map((_,index)=>'field_'+index);
            assert.ok(fields.length>0);
            arms.push(`${path}(${fields.join(', ')}) => { ${fields.map(field=>'let _ = '+field+';').join(' ')} }`);
          }else arms.push(`${path} => {}`);
        }
        if(variants.some(variant=>parsed.field(variant,'body')))bindings.push(`let _ = |value: &${module}::${name(node)}| { match value { ${arms.join(', ')} } };`);
      }
      if(kind==='struct_item'){
        const fields=descend(node,'field_declaration').map(name);
        if(fields.length)bindings.push(`let _ = |value: &${module}::${name(node)}| { let _ = (${fields.map(field=>'&value.'+field).join(', ')}); };`);
      }
      if(kind==='impl_item'&&!parsed.field(node,'trait')){
        const type=parsed.field(node,'type');assert.ok(type);
        for(const method of descend(node,'function_item').filter(node=>node.children.some(({node})=>node.term==='visibility_modifier')))
          bindings.push(`let _ = ${module}::${parsed.text(type)}::${name(method)};`);
      }
    }
  };
  for(const module of policy.modules){
    assert.match(module.name,/^[a-z_]+$/u);const parts=[module.prelude??''];
    for(const section of module.sections??[]){
      const parsed=source(section.file),first=select(parsed,section.first),last=select(parsed,section.last??section.first);
      assert.ok(first.start<=last.start);const items=parsed.items.filter(item=>item.start>=first.start&&item.end<=last.end);
      bind(module.name,parsed,items);parts.push(parsed.bytes.subarray(first.start,last.end).toString('utf8'));
      sections.push({module:module.name,file:section.file,start:first.start,end:last.end,sha256:digest(parsed.bytes.subarray(first.start,last.end)),declarations:items.map(({kind,name})=>({kind,name}))});
    }
    if(module.wholeSource){
      const parsed=source(module.wholeSource),omitted=parsed.items.filter(item=>item.kind==='mod_item'&&item.name==='tests');assert.ok(omitted.length<=1);
      let text=parsed.bytes.toString('utf8');
      if(omitted.length){const item=omitted[0],declaration=parsed.bytes.subarray(item.start,item.end).toString();assert.equal(declaration.trim(),'#[cfg(test)]\nmod tests;');text=parsed.bytes.subarray(0,item.start).toString()+parsed.bytes.subarray(item.end).toString();}
      bind(module.name,parsed,parsed.items.filter(item=>!omitted.includes(item)));parts.push(text);
      if(module.cases){
        const paths=module.cases.files.filter(file=>existsSync(resolve(repository,file)));assert.ok(paths.length>=1,'owned comparison cases remain present');
        const bytes=readFileSync(resolve(repository,paths[0]));for(const path of paths)assert.deepEqual(readFileSync(resolve(repository,path)),bytes,'case path aliases retain identical bytes');
        assert.equal(digest(bytes.subarray(0,module.cases.originalBytes)),module.cases.originalSha256,'every original test assertion remains byte exact');
        const cases=readRustSourceItems(bytes),names=cases.items.filter(item=>item.kind==='function_item'&&bytes.subarray(item.start,item.node.start).toString().includes('#[test]')).map(item=>item.name);
        assert.deepEqual(names.slice(0,module.cases.originalTests.length),module.cases.originalTests);
        inputs.push({file:paths[0],sourceAliases:module.cases.files,bytes:bytes.length,sha256:digest(bytes),testNames:names});
        const additional=[];
        for(const file of module.cases.extraCases??[]){
          assert.ok(!module.cases.files.includes(file),'additional cases have their own source identity');
          const extra=readFileSync(resolve(repository,file)),parsed=readRustSourceItems(extra);
          const testNames=parsed.items.filter(item=>item.kind==='function_item'&&extra.subarray(item.start,item.node.start).toString().includes('#[test]')).map(item=>item.name);
          assert.ok(testNames.length>0,'each additional case source owns actual tests');
          assert.equal(new Set([...names,...testNames,...additional.flatMap(row=>row.testNames)]).size,names.length+testNames.length+additional.reduce((count,row)=>count+row.testNames.length,0),'comparison test identities remain distinct');
          inputs.push({file,bytes:extra.length,sha256:digest(extra),testNames});
          additional.push({source:extra.toString('utf8'),testNames});
        }
        parts.push('mod tests {\n'+bytes.toString('utf8')+'\n'+additional.map(row=>row.source).join('\n')+'\n}');
      }
    }
    parts.push('');sections.push({module:module.name,sourceParts:parts.length});
    modules.push({name:module.name,parts});
  }
  const unformatted='// Generated from complete owned Rust CST declarations. Do not edit by hand.\n'+modules.map(module=>'mod '+module.name+' {\n'+module.parts.join('\n')+'\n}').join('\n')+'\n// Keep every copied declaration and field bound without relaxing native lints.\nconst _: fn() = || {\n'+[...new Set(bindings)].join('\n')+'\n};\n';
  const format=source=>{
    const formatted=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:source,encoding:'utf8',maxBuffer:2*1024*1024,timeout:10000});
    if(formatted.error||formatted.status!==0)throw formatted.error??new Error(formatted.stderr);
    return formatted.stdout;
  };
  const combinedSource=format(unformatted),notice='// Generated from complete owned Rust CST declarations. Do not edit by hand.\n';
  assert.match(policy.moduleDirectory,/^[a-z_]+$/u,'generated comparison modules have an explicit local directory');
  const outputs=modules.map(module=>({file:resolve(repository,dirname(policy.output),policy.moduleDirectory,module.name+'.rs'),source:format(notice+module.parts.join('\n')+'\n')}));
  const rootSource=notice+modules.map(module=>'#[path = "'+policy.moduleDirectory+'/'+module.name+'.rs"]\nmod '+module.name+';').join('\n')+'\n// Bind every copied declaration without relaxing native lints.\nconst _: fn() = || {\n'+[...new Set(bindings)].join('\n')+'\n};\n';
  outputs.unshift({file:resolve(repository,policy.output),source:format(rootSource)});
  for(const output of outputs){
    output.file=output.file.slice(resolve(repository).length+1);
    assert.ok(output.source.split('\n').length<1000,'every generated comparison module fits the repository source limit');
  }
  return {source:combinedSource,outputs,receipt:{schemaVersion:1,policySha256:digest(readFileSync(resolve(repository,policyFile))),parserGrammarSha256:digest(nativeGrammarText('native-rust')),inputs,sections,compileTimeBindings:bindings.length,outputSha256:digest(outputs[0].source),combinedSourceSha256:digest(combinedSource),outputs:outputs.map(({file,source})=>({file,bytes:Buffer.byteLength(source),sha256:digest(source)}))}};
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const policy=readProjectData(resolve(root,policyFile)),output=buildPrivateOutputTestModel(root,policy);
  if(process.argv.includes('--check')){
    for(const file of output.outputs)assert.equal(readFileSync(resolve(root,file.file),'utf8'),file.source,'each complete native source model module is fresh');
    assert.deepEqual(readProjectData(resolve(root,policy.receipt)),output.receipt,'the complete CST source receipt is fresh');
  }else{
    for(const file of output.outputs){mkdirSync(dirname(resolve(root,file.file)),{recursive:true});writeFileSync(resolve(root,file.file),file.source);}writeProjectData(resolve(root,policy.receipt),output.receipt);
  }
  console.log('Private output model retains '+output.receipt.inputs.find(row=>row.testNames)?.testNames.length+' original tests through complete native CST items.');
}
