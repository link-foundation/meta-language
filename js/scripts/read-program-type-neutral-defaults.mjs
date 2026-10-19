// Source guard for the finite false fields actually omitted by the Rust Type enum.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {parseNative} from '../src/native-grammar-parser.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';
const root=new URL('../../',import.meta.url);
const field=(node,name)=>{const selected=node.children.filter(child=>child.field===name);assert.ok(selected.length<=1);return selected[0]?.node;};

export function readProgramTypeNeutralDefaults(){
  const path='rust/src/translation/types.rs',bytes=readFileSync(new URL(path,root)),source=bytes.toString('utf8');
  const declaration=readSourceTextRustBlock(source,'enum','Type'),at=source.indexOf(declaration);
  assert.equal(source.indexOf(declaration,at+1),-1);
  assert.ok(source.slice(Math.max(0,at-100),at).includes('#[serde(tag = "kind", rename_all = "camelCase")]'));
  const tree=parseNative('native-rust',declaration),units=Buffer.from(declaration);
  assert.equal(tree.hasError,false);assert.equal(tree.start,0);assert.equal(tree.end,units.length);
  const text=node=>{assert.ok(node);return units.subarray(node.start,node.end).toString('utf8');};
  const owners=tree.children.map(child=>child.node).filter(node=>field(node,'name')&&text(field(node,'name'))==='Type');
  assert.equal(owners.length,1);
  const variants=field(owners[0],'body');assert.ok(variants);
  const defaults={},evidence=[];
  for(const {node:variant}of variants.children){
    if(variant.term!=='enum_variant')continue;
    const name=text(field(variant,'name')),body=field(variant,'body');if(!body)continue;
    const kind=name[0].toLowerCase()+name.slice(1);let attributes=[];
    for(const {node}of body.children){
      if(node.term==='attribute_item'){attributes.push(text(node));continue;}
      if(node.term!=='field_declaration')continue;
      const type=field(node,'type'),key=field(node,'name');assert.ok(type&&key);
      if(attributes.includes('#[serde(default, skip_serializing_if = "is_false")]')){
        assert.equal(text(type),'bool');const name=text(key);
        (defaults[kind]??={})[name]='false';
        evidence.push({kind,field:name,fieldSource:text(node),attributes:[...attributes]});
      }
      attributes=[];
    }
  }
  const isFalse=readSourceTextRustBlock(source,'fn','is_false');
  assert.equal(isFalse,'fn is_false(value: impl std::ops::Not<Output = bool>) -> bool {\n    !value\n}');
  assert.deepEqual(defaults,{array:{reference:'false'},record:{open:'false',reference:'false'},map:{reference:'false'},set:{reference:'false'}});
  assert.equal(evidence.length,5);
  return {defaults,source:{path,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),declaration,isFalse},fields:evidence};
}
