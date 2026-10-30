// Source compiler pass: resolved CST bindings supply every renamed occurrence.
import { createHash } from 'node:crypto';
import { analyzeProgram } from '../src/program-representation.js';
import { tokenize } from '../src/translation/lexer.js';
import { readModuleBindingFields } from '../src/module-binding-fields.js';
import { readSourceModuleBindingRangeRole,readSourceBindingRenameAffixes } from '../src/translation/module-binding-rules.js';
import {testJavaScriptSourceFunctionExpressionTerm} from '../src/translation/source-library-binding-rules.js';
const digest=source=>createHash('sha256').update(source).digest('hex');

export function composeSourceLibraryAliases(inputs,entryPath,readImports) {
  const names=new Set(),counts=new Map(),selected=new Map();
  for(const input of inputs){
    for(const token of tokenize(input.source,'JavaScript').tokens)if(token.kind==='identifier')names.add(token.value);
    for(const name of input.declarations)counts.set(name,(counts.get(name)??0)+1);
  }
  const key=(path,name)=>path+'\0'+name;
  const aliased=new Set();
  for(const input of inputs)for(const imported of input.removedImports)for(const binding of imported.bindings)
    if(binding.local!==binding.imported)aliased.add(key(imported.dependency,binding.imported));
  let ordinal=0;
  function fresh(){let name;do{name='dependencyFunction'+(++ordinal);}while(names.has(name));names.add(name);return name;}
  for(const input of inputs)for(const name of input.declarations){
    const identity=key(input.path,name);
    selected.set(identity,input.path!==entryPath&&((counts.get(name)??0)>1||aliased.has(identity))?fresh():name);
  }
  let rewrites=0;
  for(const input of inputs){
    const requests=[],imports=[];
    for(const name of input.declarations){const target=selected.get(key(input.path,name));if(target!==name)requests.push({name,target,kind:input.declarationBindings?.find(binding=>binding.name===name)?.kind??'function'});}
    for(const imported of input.removedImports)for(const binding of imported.bindings){
      const target=selected.get(key(imported.dependency,binding.imported));
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
      const before=digest(source);
      program=program.renameBinding(binding.id,request.target);source=program.source;
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
      const before=digest(source);
      // The ordinary rename operation proves every binding/reference stays
      // resolved to its original owner, including default parameter reads.
      program=program.renameBinding(inner.id,target);source=program.source;
      transformations.push({operation:'resolved private function binding rename',binding:inner.id,original:inner.name,replacement:target,beforeSourceSha256:before,sourceSha256:digest(source)});
    }
    // Substitute all imported references against one original binding model.
    // Imports are removed later, so two aliases can retain the same provider
    // identity without creating duplicate lexical declarations in a module.
    const sourceEdits=[];
    if(imports.length){
      program??=analyzeProgram(source,'JavaScript');
      const roles=readModuleBindingFields(program.network,program.sourceMappings,'JavaScript');
      const before=digest(source);
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
    }
    const transformedImports=readImports(source);
    input.compiledDeclarations=input.declarations.map(name=>selected.get(key(input.path,name)));
    input.declarationAliases=input.declarations.map((original,index)=>({source:input.path,original,compiled:input.compiledDeclarations[index]}));
    input.compiledImportNames=input.removedImports.flatMap(imported=>imported.bindings.map(binding=>selected.get(key(imported.dependency,binding.imported))));
    // Preserve the entire original module/import source separately. The current
    // source and removed spans describe the actual compiled closure, not history.
    input.transformedSource=source;input.transformedImports=transformedImports;input.sourceTransformations=transformations;input.sourceAliasEdits=sourceEdits;
  }
  return inputs.map(input=>{
    let body=input.transformedSource;
    for(const imported of input.transformedImports.toReversed())body=body.slice(0,imported.start)+body.slice(imported.end);
    return body;
  });
}
