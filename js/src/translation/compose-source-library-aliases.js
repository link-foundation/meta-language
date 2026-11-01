// Source compiler pass: resolved CST bindings supply every renamed occurrence.
import { createHash } from 'node:crypto';
import { analyzeProgram } from '../program-representation.js';
import { tokenize } from './lexer.js';
import { readModuleBindingFields } from '../module-binding-fields.js';
import { readSourceModuleBindingRangeRole,readSourceBindingRenameAffixes } from './module-binding-rules.js';
import {testJavaScriptSourceFunctionExpressionTerm} from './source-library-binding-rules.js';
import {recordSourceLibraryCompositionStep} from './source-library-classes.js';
import {acceptSourceLibraryEntryExportOwner} from './source-library-class-rules.js';
const digest=source=>createHash('sha256').update(source).digest('hex');

export function composeSourceLibraryAliases(inputs,entryPath,readImports,{classes=false,reExports=false}={}) {
  const names=new Set(),counts=new Map(),selected=new Map();
  for(const input of inputs){
    for(const token of tokenize(input.source,'JavaScript').tokens)if(token.kind==='identifier')names.add(token.value);
    for(const name of input.declarations)counts.set(name,(counts.get(name)??0)+1);
  }
  const key=(path,name)=>path+'\0'+name;
  const modules=new Map(inputs.map(input=>[input.path,input]));
  function provider(path,name,seen=[]){
    if(!reExports)return {path,name};const identity=key(path,name);if(seen.includes(identity)||seen.length>=128)throw new TypeError('a named re-export needs an acyclic complete actual provider chain');
    const input=modules.get(path);if(!input)throw new TypeError('a named re-export lost its actual supplied provider module');
    if(input.directExports.includes(name)&&input.declarations.includes(name))return {path,name};
    const matches=input.removedReExports.flatMap(row=>row.bindings.filter(binding=>binding.exported===name).map(binding=>({path:row.dependency,name:binding.imported})));
    if(matches.length!==1)throw new TypeError('a named re-export needs exactly one actual declared provider for '+name+' in '+path);
    return provider(matches[0].path,matches[0].name,[...seen,identity]);
  }
  const aliased=new Set();
  for(const input of inputs)for(const imported of input.removedImports)for(const binding of imported.bindings)
    {const actual=provider(imported.dependency,binding.imported);if(binding.local!==actual.name)aliased.add(key(actual.path,actual.name));}
  let ordinal=0;
  function fresh(){let name;do{name='dependencyFunction'+(++ordinal);}while(names.has(name));names.add(name);return name;}
  function renameEdits(program,binding,target){const roles=readModuleBindingFields(program.network,program.sourceMappings,'JavaScript');return [binding.declaration,...binding.references].map(range=>{const shorthand=program.sourceMappings.some(node=>node.start===range.start&&node.end===range.end&&['shorthand_property_identifier','shorthand_property_identifier_pattern'].includes(node.term));const[prefix,suffix]=readSourceBindingRenameAffixes(readSourceModuleBindingRangeRole(roles,range.start,range.end),shorthand,binding.name);return {...range,text:prefix+target+suffix};});}
  for(const input of inputs)for(const name of input.declarations){
    const identity=key(input.path,name);
    const foreignReference=classes&&inputs.some(other=>other.path!==input.path&&other.classUnresolvedNames.includes(name));
    if(foreignReference&&input.path===entryPath&&input.exports.includes(name))throw new TypeError('a public entry binding needs its actual export wrapper before resolving a foreign implicit owner: '+name);
    selected.set(identity,foreignReference||input.path!==entryPath&&((counts.get(name)??0)>1||aliased.has(identity))?fresh():name);
  }
  let rewrites=0;
  for(const input of inputs){
    const requests=[],imports=[];
    for(const name of input.declarations){const target=selected.get(key(input.path,name));if(target!==name)requests.push({name,target,kind:input.declarationBindings?.find(binding=>binding.name===name)?.kind??'function'});}
    for(const imported of input.removedImports)for(const binding of imported.bindings){
      const actual=provider(imported.dependency,binding.imported),target=selected.get(key(actual.path,actual.name));
      if(!target)throw new TypeError('source alias has no actual declared provider '+binding.imported+' in '+imported.dependency);
      if(target!==binding.local)imports.push({name:binding.local,target,kind:'import'});
    }
    rewrites+=requests.length+imports.length;
    if(rewrites>1024)throw new RangeError('source alias composition exceeds 1024 actual binding rewrites');
    let source=input.source,program=null;
    const transformations=[];
    for(const request of requests){
      program??=analyzeProgram(source,'JavaScript');
      const binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.kind===request.kind&&binding.name===request.name);
      if(!binding)throw new TypeError('source alias has no actual resolved '+request.kind+' binding '+request.name+' in '+input.path);
      const before=digest(source),prior=source,edits=classes||reExports?renameEdits(program,binding,request.target):[];
      program=program.renameBinding(binding.id,request.target);source=program.source;
      if(classes||reExports)recordSourceLibraryCompositionStep(input,prior,source,'resolved lexical binding rename',edits);
      transformations.push({operation:'resolved lexical binding rename',binding:binding.id,original:request.name,replacement:request.target,beforeSourceSha256:before,sourceSha256:digest(source)});
    }
    for(const original of input.declarationBindings??[]){
      if(original.kind!=='const'||!original.expressionName)continue;
      program??=analyzeProgram(source,'JavaScript');
      const target=selected.get(key(input.path,original.name));
      const outer=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.kind==='const'&&binding.name===target);
      const declarator=outer&&program.sourceMappings.find(node=>node.term==='variable_declarator'&&node.start===outer.declaration.start);
      const expression=declarator&&program.sourceMappings.filter(node=>testJavaScriptSourceFunctionExpressionTerm(node.term)&&node.start>=outer.declaration.end&&node.end<=declarator.end).sort((left,right)=>left.start-right.start||right.end-left.end)[0];
      const inner=expression&&program.bindings.find(binding=>binding.kind==='function'&&binding.name===original.expressionName&&program.scopes.some(scope=>scope.id===binding.scope&&scope.start===expression.start&&scope.end===expression.end));
      if(!inner)throw new TypeError('source function-expression normalization needs its actual private lexical binding in '+input.path);
      if(inner.name===target)continue;
      const before=digest(source),prior=source,edits=classes||reExports?renameEdits(program,inner,target):[];
      // The ordinary rename operation proves every binding/reference stays
      // resolved to its original owner, including default parameter reads.
      program=program.renameBinding(inner.id,target);source=program.source;
      if(classes||reExports)recordSourceLibraryCompositionStep(input,prior,source,'resolved private function binding rename',edits);
      transformations.push({operation:'resolved private function binding rename',binding:inner.id,original:inner.name,replacement:target,beforeSourceSha256:before,sourceSha256:digest(source)});
    }
    // Substitute all imported references against one original binding model.
    // Imports are removed later, so two aliases can retain the same provider
    // identity without creating duplicate lexical declarations in a module.
    const sourceEdits=[];
    if(imports.length){
      program??=analyzeProgram(source,'JavaScript');
      const roles=readModuleBindingFields(program.network,program.sourceMappings,'JavaScript');
      const before=digest(source),prior=source;
      for(const request of imports){
        const binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.kind==='import'&&binding.name===request.name);
        if(!binding)throw new TypeError('source alias has no actual resolved import binding '+request.name+' in '+input.path);
        if(program.bindings.some(other=>other.id!==binding.id&&other.name===request.target)||program.unresolvedReferences.some(reference=>reference.name===request.target))
          throw new TypeError('source alias substitution would capture '+request.target+' in '+input.path);
        for(const reference of binding.references){
          const shorthand=program.sourceMappings.some(node=>node.start===reference.start&&node.end===reference.end&&['shorthand_property_identifier','shorthand_property_identifier_pattern'].includes(node.term));
          const [prefix,suffix]=readSourceBindingRenameAffixes(readSourceModuleBindingRangeRole(roles,reference.start,reference.end),shorthand,binding.name);
          sourceEdits.push({start:reference.start,end:reference.end,text:prefix+request.target+suffix,binding:binding.id,coordinateUnit:'UTF-16 code units'});
        }
        transformations.push({operation:'resolved import binding substitution',binding:binding.id,original:request.name,replacement:request.target,beforeSourceSha256:before});
      }
      sourceEdits.sort((left,right)=>right.start-left.start);
      let previous=source.length;
      for(const edit of sourceEdits){
        if(edit.end>previous)throw new TypeError('source alias substitutions need disjoint actual reference ranges');
        source=source.slice(0,edit.start)+edit.text+source.slice(edit.end);previous=edit.start;
      }
      for(const transformation of transformations)if(transformation.operation==='resolved import binding substitution')transformation.sourceSha256=digest(source);
      if(classes||reExports)recordSourceLibraryCompositionStep(input,prior,source,'resolved import binding substitution',sourceEdits);
    }
    const transformed=readImports(source),transformedImports=Array.isArray(transformed)?transformed:transformed.imports;
    input.compiledDeclarations=input.declarations.map(name=>selected.get(key(input.path,name)));
    input.declarationAliases=input.declarations.map((original,index)=>({source:input.path,original,compiled:input.compiledDeclarations[index]}));
    input.compiledImportNames=input.removedImports.flatMap(imported=>imported.bindings.map(binding=>{const actual=provider(imported.dependency,binding.imported);return selected.get(key(actual.path,actual.name));}));
    if(reExports){
      input.transformedReExports=transformed.namedReExports;
      input.compiledExports=input.exports.map(exported=>{const actual=provider(input.path,exported),compiled=selected.get(key(actual.path,actual.name));if(!compiled)throw new TypeError('a named public export lost its actual compiled declaration');if(input.path===entryPath&&!acceptSourceLibraryEntryExportOwner(exported,compiled))throw new TypeError('a public re-export needs its actual callable or constructor export wrapper: '+exported);return {exported,source:actual.path,original:actual.name,compiled};});
    }
    // Preserve the entire original module/import source separately. The current
    // source and removed spans describe the actual compiled closure, not history.
    input.transformedSource=source;input.transformedImports=transformedImports;input.sourceTransformations=transformations;input.sourceAliasEdits=sourceEdits;
  }
  return inputs.map(input=>{
    let body=input.transformedSource;
    const removed=[...input.transformedImports,...(input.transformedReExports??[])].toSorted((a,b)=>b.start-a.start);
    for(const imported of removed)body=body.slice(0,imported.start)+body.slice(imported.end);
    if(reExports&&!classes){recordSourceLibraryCompositionStep(input,input.transformedSource,body,'remove actual resolved module dependency statements',removed.map(row=>({start:row.start,end:row.end,text:''})));input.compiledModuleSource=body;}
    return body;
  });
}
