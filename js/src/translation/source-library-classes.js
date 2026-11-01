// Source-only composition facts: CST lexical owners and exact edit receipts.
import {createHash} from 'node:crypto';
import {analyzeProgram} from '../program-representation.js';
import {tokenize} from './lexer.js';
import {acceptSourceLibraryClassDeclarationTerm,projectSourceLibraryClassEditInterval,readSourceLibraryClassDeclarationOwner,acceptSourceLibraryClassHelperCall,acceptSourceLibraryClassStaticCall} from './source-library-class-rules.js';
import {readJavaScriptClosedClassDirectory,readJavaScriptClosedClassStaticCalls,readJavaScriptClosedClassRefusal,acceptJavaScriptClosedClassSourceSize} from './javascript-class-source-rules.js';
import {readSourceParserIdentifierSupply} from './source-identifiers.js';
import {checkJavaScriptClassOwners} from './javascript-class-owners.js';
import {readJavaScriptErrorClassDirectory,readJavaScriptErrorClassRefusal} from './javascript-error-class-rules.js';
const digest=source=>createHash('sha256').update(source).digest('hex');
export function readSourceLibraryClassFacts(source,path,facts){
 const lexical=tokenize(source,'JavaScript'),values=lexical.tokens.map(token=>['identifier','punct'].includes(token.kind)?token.value:'#'+token.value),kinds=lexical.tokens.map(token=>token.kind);
 if(!acceptJavaScriptClosedClassSourceSize(source.length,lexical.tokens.length))throw new RangeError('class source module exceeds its actual closed frontend size profile in '+path);
 const program=analyzeProgram(source,'JavaScript');if(!program.network.verifyFullMatch().isClean())throw new TypeError('class source closure needs complete actual CST in '+path);
 const declarations=[],classes=[],ranges=[];
 for(const declared of facts.declarationBindings??facts.declarations.map(name=>({name,kind:'function'}))){
  const binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.name===declared.name&&binding.kind===declared.kind);
  if(!binding)throw new TypeError('class source closure lost actual module binding '+declared.name+' in '+path);
  const syntax=program.sourceMappings.filter(node=>acceptSourceLibraryClassDeclarationTerm(node.term,declared.kind)&&node.start<=binding.declaration.start&&node.end>=binding.declaration.end).sort((a,b)=>(a.end-a.start)-(b.end-b.start))[0];
  if(!syntax)throw new TypeError('class source closure needs actual declaration syntax '+declared.name+' in '+path);
  const exported=program.sourceMappings.find(node=>node.term==='export_statement'&&node.start<=syntax.start&&node.end>=syntax.end);
  ranges.push({start:exported?.start??syntax.start,end:exported?.end??syntax.end});declarations.push({...declared,start:syntax.start,end:syntax.end,coordinateUnit:'UTF-16 code units'});
  if(declared.kind==='class'){
   const first=lexical.tokens.findIndex(token=>token.start===syntax.start),directory=facts.sourceErrorClasses?readJavaScriptErrorClassDirectory(values,kinds,first):readJavaScriptClosedClassDirectory(values,kinds,first);
   if(directory[0]<0)throw new TypeError('class source closure '+path+': '+(facts.sourceErrorClasses?readJavaScriptErrorClassRefusal:readJavaScriptClosedClassRefusal)(directory[0]));
   const prefix=exported&&tokenize(source.slice(exported.start,syntax.start),'JavaScript').tokens.find(token=>token.kind==='identifier'&&token.value==='export');
   if(exported&&(!prefix||source.slice(exported.start+prefix.start,exported.start+prefix.end)!=='export'))throw new TypeError('a class export needs its actual owned prefix in '+path);
   let ownedStart=exported?.start??syntax.start;for(const comment of lexical.comments.toReversed())if(comment.end<=ownedStart&&source.slice(comment.end,ownedStart).trim()==='')ownedStart=comment.start;
   classes.push({name:declared.name,start:syntax.start,end:syntax.end,ownedStart,...(prefix?{exportStart:exported.start+prefix.start,exportEnd:exported.start+prefix.end}:{}),coordinateUnit:'UTF-16 code units'});
  }
 }
 for(const imported of facts.imports){if(!program.sourceMappings.some(node=>node.term==='import_statement'&&node.start===imported.start&&node.end===imported.end))throw new TypeError('class source imports need actual complete CST ownership in '+path);ranges.push(imported);}
 for(const exported of facts.namedReExports??[]){if(!program.sourceMappings.some(node=>node.term==='export_statement'&&node.start===exported.start&&node.end===exported.end))throw new TypeError('class re-exports need actual complete CST ownership in '+path);ranges.push(exported);}
 // A top-level nonfunction constant, statement, namespace or custom export
 // cannot acquire an accidental module environment through concatenation.
 for(const binding of program.bindings.filter(binding=>binding.scope==='scope:0'))if(!facts.declarations.includes(binding.name)&&!facts.imports.some(imported=>imported.bindings?.some(row=>row.local===binding.name)))throw new TypeError('class source closure has unrepresented module state '+binding.name+' in '+path);
 for(const token of lexical.tokens)if(token.kind!=='eof'&&token.value!==';'&&!ranges.some(range=>token.start>=range.start&&token.end<=range.end))throw new TypeError('class source closure needs declaration-only module syntax in '+path+':'+token.start);
 return {declarations,classes,unresolvedNames:[...new Set(program.unresolvedReferences.map(reference=>reference.name))]};
}
export function recordSourceLibraryCompositionStep(input,before,after,operation,edits){
 const ordered=[...edits].sort((a,b)=>a.start-b.start),pieces=[];let cursor=0;
 for(const edit of ordered){if(!Number.isSafeInteger(edit.start)||!Number.isSafeInteger(edit.end)||edit.start<cursor||edit.end<edit.start||edit.end>before.length||typeof edit.text!=='string')throw new TypeError('class source mapping needs disjoint actual source edits');pieces.push(before.slice(cursor,edit.start),edit.text);cursor=edit.end;}
 pieces.push(before.slice(cursor));if(pieces.join('')!==after)throw new TypeError('class source mapping edits disagree with resolved composition');
 input.sourceCompositionSteps??=[];input.sourceCompositionSteps.push({operation,beforeSourceSha256:digest(before),sourceSha256:digest(after),edits:ordered.map(edit=>({start:edit.start,end:edit.end,text:edit.text,coordinateUnit:'UTF-16 code units'}))});
}
export function projectSourceLibraryOriginalInterval(input,start,end){
 let interval=[start,end];for(const step of (input.sourceCompositionSteps??[]).toReversed()){interval=projectSourceLibraryClassEditInterval(step.edits.map(edit=>edit.start),step.edits.map(edit=>edit.end),step.edits.map(edit=>edit.text.length),interval[0],interval[1]);if(interval.length!==2)throw new TypeError('class source interval lost its actual source edit owner');}return {start:interval[0],end:interval[1],coordinateUnit:'UTF-16 code units'};
}
export function finishSourceLibraryClassComposition(input,body,readFacts){
 // Import deletion and export-prefix deletion are separate recorded phases.
 let before=input.transformedSource;
 const removed=[...input.transformedImports,...(input.transformedReExports??[])];
 recordSourceLibraryCompositionStep(input,before,body,input.transformedReExports?'remove actual resolved module dependency statements':'remove actual resolved import statements',removed.map(row=>({start:row.start,end:row.end,text:''})));
 const facts=readFacts(body),edits=facts.sourceErrorClasses?[]:facts.classDeclarations.filter(row=>row.exportStart!==undefined).map(row=>({start:row.exportStart,end:row.exportEnd,text:''}));
 let source=body;for(const edit of edits.toSorted((a,b)=>b.start-a.start))source=source.slice(0,edit.start)+source.slice(edit.end);
 recordSourceLibraryCompositionStep(input,body,source,'remove actual owned class export prefixes',edits);
 const compiled=readFacts(source),actual=readSourceLibraryClassFacts(source,input.path,compiled);input.compiledSourceDeclarations=actual.declarations;input.compiledClassDeclarations=compiled.classDeclarations;input.compiledModuleSource=source;
 return source;
}
export function composeSourceLibraryClassDeclarations(inputs,parts){
 const classes=[],declarations=[];
 for(let index=0;index<inputs.length;index+=1){const input=inputs[index],source=parts[index];let cursor=0;input.closureRanges=[];input.closureClassDeclarations=[];delete input.closureRange;
  for(const row of input.compiledClassDeclarations){const start=row.ownedStart??row.start;if(start<cursor||row.end<start)throw new TypeError('class composition needs disjoint actual declaration ownership');if(start>cursor)declarations.push({input,source,start:cursor,end:start});classes.push({input,source,start,end:row.end,classDeclaration:row});cursor=row.end;}
  if(cursor<source.length)declarations.push({input,source,start:cursor,end:source.length});
 }
 const chunks=[];let offset=0;
 for(const chunk of [...classes,...declarations]){
  const text=chunk.source.slice(chunk.start,chunk.end),range={start:offset,end:offset+text.length,sourceStart:chunk.start,sourceEnd:chunk.end,coordinateUnit:'UTF-16 code units'};
  chunk.input.closureRanges.push(range);if(chunk.classDeclaration)chunk.input.closureClassDeclarations.push({name:chunk.classDeclaration.name,start:range.start,end:range.end,syntaxStart:range.start+chunk.classDeclaration.start-chunk.start,coordinateUnit:'UTF-16 code units'});
  chunks.push(text);offset+=text.length+2;
 }
 return chunks;
}
// Checked spans intentionally retain conservative source intervals. Actual
// module binding names and checked declaration edges therefore own helpers;
// widened spans and generated identifier spelling never establish an owner.
export function readSourceLibraryClassOwnership(closure,program){
 if(program.main?.effects.length||program.externals?.length)throw new TypeError('class source closure needs actual closed declaration-only modules');
 checkJavaScriptClassOwners(program);
 const witnesses=program.sourceClassOwners?.owners??[],owned=new Map(),original=new Set();
 function install(name,info){const previous=owned.get(name);if(previous&&previous.input!==info.input)throw new TypeError('a declaration has contradictory actual source module owners: '+name);owned.set(name,info);}
 for(const input of closure.inputs)for(const declared of input.compiledSourceDeclarations){
  const entry=program.declarations.get(declared.name);if(!entry||entry.k!=='fn')throw new TypeError('a class source binding lost its actual checked function declaration: '+declared.name);
  if(original.has(declared.name))throw new TypeError('class source composition needs unique actual compiled bindings');original.add(declared.name);
  const classDeclaration=declared.kind==='class'?input.compiledClassDeclarations.find(row=>row.name===declared.name):undefined;
  const classOwner=classDeclaration?readSourceLibraryClassDeclarationOwner([classDeclaration.name],[classDeclaration.ownedStart??classDeclaration.start],[classDeclaration.end],witnesses.map(owner=>owner.constructor),declared.start,declared.end):'';
  if(declared.kind==='class'&&classOwner!==declared.name)throw new TypeError('an actual source class lost its validated constructor witness: '+declared.name);
  const sourceSpan=projectSourceLibraryOriginalInterval(input,classDeclaration?.ownedStart??declared.start,declared.end);
  install(declared.name,{input,sourceRoots:[declared.name],sourceSpan,...(classOwner?{sourceClassOwner:classOwner}:{})});
 }
 for(const witness of witnesses){const constructor=owned.get(witness.constructor);if(!constructor||constructor.sourceClassOwner!==witness.constructor)throw new TypeError('a class witness needs its actual supplied constructor binding');for(const method of witness.methods){if(original.has(method.helper))throw new TypeError('a generated class method cannot replace an actual supplied binding');install(method.helper,{...constructor});}}
 // Static helpers have no instance receiver and are not instance witness
 // methods. Reuse the actual source lowering plan, never its name pattern.
 const lexical=tokenize(closure.source,'JavaScript'),values=lexical.tokens.map(token=>['identifier','punct'].includes(token.kind)?token.value:'#'+token.value),kinds=lexical.tokens.map(token=>token.kind);
 const classDeclarations=closure.inputs.flatMap(input=>input.closureClassDeclarations).toSorted((a,b)=>a.syntaxStart-b.syntaxStart),directory=classDeclarations.map(row=>readJavaScriptClosedClassDirectory(values,kinds,lexical.tokens.findIndex(token=>token.start===row.syntaxStart)));
 if(directory.some(row=>row[0]<0))throw new TypeError('an actual composed class lost its admitted source directory');
 const statics=readJavaScriptClosedClassStaticCalls(values,directory,readSourceParserIdentifierSupply(closure.source,lexical.tokens)),staticOwners=new Map();
 for(let at=0;at<statics.length;at+=2){const qualified=statics[at],helper=statics[at+1],constructor=owned.get(qualified.slice(0,qualified.lastIndexOf('.')));if(!constructor||!constructor.sourceClassOwner||program.declarations.get(helper)?.k!=='fn'||original.has(helper))throw new TypeError('a static class helper lost its actual source constructor owner');install(helper,{...constructor});staticOwners.set(helper,constructor.sourceClassOwner);}
 function dependencies(entry){const result=new Set(),pending=[entry],seen=new WeakSet();let visits=0;while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(++visits>100000)throw new TypeError('class source dependency edges exceed their checked graph budget');seen.add(value);if(['call','functionRef'].includes(value.k))result.add(value.fn);if(value.k==='ctor')result.add(value.data);if(value.kind==='data')result.add(value.name);for(const child of Array.isArray(value)?value:Object.values(value))if(child&&typeof child==='object')pending.push(child);}for(const name of result)if(!program.declarations.has(name))throw new TypeError('a source dependency lost its actual checked declaration: '+name);return [...result];}
 const edges=new Map([...program.declarations].map(([name,entry])=>[name,dependencies(entry)]));
 const seeded=new Set(owned.keys());let changed=true,passes=0;
 while(changed){if(++passes>program.declarations.size+1)throw new TypeError('class source helper ownership exceeds its actual declaration fixed point');changed=false;
  for(const[name,info]of [...owned])for(const target of edges.get(name)){
   if(seeded.has(target))continue;const entry=program.declarations.get(target);if(entry.k==='data'&&!entry.generated)throw new TypeError('class module datatypes need their actual documentation binding profile: '+target);
   const previous=owned.get(target);if(previous&&previous.input!==info.input)throw new TypeError('a private generated helper has conflicting actual source module owners: '+target);
   if(!previous){owned.set(target,{...info,sourceRoots:[...info.sourceRoots]});changed=true;}
   else{const roots=[...new Set([...previous.sourceRoots,...info.sourceRoots])];if(roots.length!==previous.sourceRoots.length){owned.set(target,{input:info.input,sourceRoots:roots,sourceSpan:{start:Math.min(previous.sourceSpan.start,info.sourceSpan.start),end:Math.max(previous.sourceSpan.end,info.sourceSpan.end),coordinateUnit:'UTF-16 code units'},...(previous.sourceClassOwner&&previous.sourceClassOwner===info.sourceClassOwner?{sourceClassOwner:info.sourceClassOwner}:{})});changed=true;}}
  }
 }
 for(const name of program.declarations.keys())if(!owned.has(name))throw new TypeError('a private declaration needs a complete reachable actual source owner: '+name);
 const importedOwners=new Map();
 for(const input of closure.inputs){const imported=new Set(input.compiledImportNames),classes=new Set(),pending=[];for(const name of imported){const entry=program.declarations.get(name);if(entry?.k==='fn')pending.push(entry.ret,...entry.params.map(parameter=>parameter.type));}const seen=new WeakSet();let visits=0;while(pending.length){const type=pending.pop();if(!type||typeof type!=='object'||seen.has(type))continue;if(++visits>100000)throw new TypeError('class import interface exceeds its actual type graph budget');seen.add(type);if(type.kind==='record'&&typeof type.sourceClassOwner==='string')classes.add(type.sourceClassOwner);if(type.kind==='data'){const entry=program.declarations.get(type.name);if(entry?.k!=='data')throw new TypeError('class import interface lost its actual nominal declaration');pending.push(...entry.ctors.flatMap(ctor=>ctor.fields.map(field=>field.type)));}for(const child of Array.isArray(type)?type:Object.values(type))if(child&&typeof child==='object')pending.push(child);}importedOwners.set(input,{imported,classes});}
 for(const[name,entry]of program.declarations){const info=owned.get(name),scope=importedOwners.get(info.input),pending=[entry],seen=new WeakSet();let visits=0;while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(++visits>100000)throw new TypeError('class lexical capture exceeds its checked graph budget');seen.add(value);if(['call','functionRef'].includes(value.k)&&owned.get(value.fn)?.input!==info.input&&!scope.imported.has(value.fn)){const witness=value.k==='call'?witnesses.find(owner=>owner.methods.some(method=>method.helper===value.fn)):undefined,staticOwner=value.k==='call'?staticOwners.get(value.fn):undefined;const instanceAllowed=witness&&acceptSourceLibraryClassHelperCall([...scope.classes],witness.constructor,witness.methods.map(method=>method.helper),value.fn,value.args?.[0]?.type?.sourceClassOwner??''),staticAllowed=staticOwner&&acceptSourceLibraryClassStaticCall([...scope.classes],staticOwner,[...staticOwners.keys()],value.fn);if(!instanceAllowed&&!staticAllowed)throw new TypeError('source dependency cannot capture '+value.fn+' from another module in '+info.input.path);}for(const child of Array.isArray(value)?value:Object.values(value))if(child&&typeof child==='object')pending.push(child);}}
 return [...program.declarations].map(([name,entry])=>{const info=owned.get(name),index=info.input.compiledDeclarations.indexOf(name),sourceName=index<0?undefined:info.input.declarations[index];return {name,path:info.input.path,generated:entry.generated===true,sourceRoots:info.sourceRoots,sourceSpan:info.sourceSpan,...(sourceName!==undefined?{sourceName}:{}),...(info.sourceClassOwner?{sourceClassOwner:info.sourceClassOwner}:{})};});
}
