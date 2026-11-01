// Host source transport. Reuse the actual supplied-module CST alias compiler;
// archived names/edit tables never certify an Error declaration by themselves.
import {createHash} from 'node:crypto';
import {analyzeProgram} from '../program-representation.js';
import {readSuppliedSourceLibraryClosure} from './source-library-closure.js';
import {projectSourceLibraryOriginalInterval} from './source-library-classes.js';
import {captureJavaScriptErrorClassModule} from './javascript-error-classes.js';
import {acceptSourceLibraryErrorDeclarationOwner,acceptSourceLibraryErrorEntryOwner} from './source-library-error-rules.js';
import {assertPlainSourceReceipt} from './source-receipt-data.js';
const model='resolved supplied Error source modules 1',digest=source=>createHash('sha256').update(source).digest('hex');
const witnesses=new WeakSet();

export function captureSourceLibraryErrorClosure(closure){
 assertPlainSourceReceipt(closure);
 if(closure?.sourceErrorClasses!==model||typeof closure.entryPath!=='string'||!Array.isArray(closure.inputs))throw new TypeError('an actual explicitly selected Error source closure is required');
 const actual=readSuppliedSourceLibraryClosure(closure.entryPath,closure.inputs.map(input=>({path:input.path,source:input.source})),{aliases:true,errorClasses:model,reExports:closure.sourceReExports??''});
 if(JSON.stringify(actual)!==JSON.stringify(closure))throw new TypeError('the complete source closure must recompose from its actual original modules and CST bindings');
 const capture=captureJavaScriptErrorClassModule(actual.source,{profile:'actual intrinsic Error subclasses 1',identifier:actual.entryPath});
 const owners=[];
 for(const definition of capture.plan.functions){
  const origin=capture.origins.find(row=>row.identity===definition.identity),matches=actual.inputs.flatMap(input=>(input.compiledSourceDeclarations??[]).filter(row=>row.name===definition.name).map(row=>({input,row})));
  if(!acceptSourceLibraryErrorDeclarationOwner(definition.name,definition.name,matches.length)||!origin)throw new TypeError('each Error source callable needs one actual composed declaration owner');
  const {input,row}=matches[0],aliases=input.declarationAliases.filter(alias=>alias.compiled===definition.name);
  if(aliases.length!==1)throw new TypeError('each composed callable needs one actual original declaration binding');
  const originalName=aliases[0].original,program=analyzeProgram(input.source,'JavaScript'),kind=definition.$==='errorConstructorPlan'?'class':'function';
  const binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.kind===kind&&binding.name===originalName);
  const syntax=binding&&program.sourceMappings.filter(node=>node.term===(kind==='class'?'class_declaration':'function_declaration')&&node.start<=binding.declaration.start&&node.end>=binding.declaration.end).sort((left,right)=>(left.end-left.start)-(right.end-right.start))[0];
  const range=input.closureRanges.find(range=>origin.start>=range.start&&origin.end<=range.end);
  if(!syntax||!range)throw new TypeError('a composed callable needs its actual complete original CST declaration and source interval');
  const projected=projectSourceLibraryOriginalInterval(input,range.sourceStart+origin.start-range.start,range.sourceStart+origin.end-range.start);
  if(projected.start<syntax.start||projected.end>syntax.end||row.kind!==kind)throw new TypeError('the composed callable interval cannot acquire another source declaration owner');
  owners.push({identity:definition.identity,compiledName:definition.name,originalName,kind,path:input.path,sourceSha256:input.sourceSha256,source:input.source.slice(syntax.start,syntax.end),originalSpan:{start:syntax.start,end:syntax.end,coordinateUnit:'UTF-16 code units'},compiledSpan:{start:origin.start,end:origin.end,coordinateUnit:'UTF-16 code units'}});
 }
 const entry=actual.inputs.find(input=>input.path===actual.entryPath);if(!entry)throw new TypeError('the supplied entry module must retain its actual source identity');
 const entries=entry.exports.map(name=>{
  const mapping=entry.compiledExports?.find(row=>row.exported===name),compiled=mapping?.compiled??entry.declarationAliases.find(row=>row.original===name)?.compiled,path=mapping?.source??entry.path;
  const selected=owners.filter(owner=>owner.path===path&&owner.compiledName===compiled);
  if(!acceptSourceLibraryErrorEntryOwner(entry.exports,name,selected.length))throw new TypeError('a public Error module entry needs its actual resolved exported declaration');
  return {exported:name,identity:selected[0].identity,compiledName:selected[0].compiledName,path:selected[0].path,construct:selected[0].kind==='class'};
 });
 return {schemaVersion:1,artifactKind:'authenticated semantic source plan/input 1',model,sourceClosure:actual,capture,owners,entries,completeSourceReceipts:actual.inputs.map(input=>({path:input.path,source:input.source,bytes:Buffer.byteLength(input.source),sourceSha256:digest(input.source),closureRanges:input.closureRanges,sourceCompositionSteps:input.sourceCompositionSteps}))};
}

export function checkSourceLibraryErrorClosure(archive){
 assertPlainSourceReceipt(archive);
 const actual=captureSourceLibraryErrorClosure(archive.sourceClosure);if(JSON.stringify(actual)!==JSON.stringify(archive))throw new TypeError('Error linkage owners, public entries and complete source receipts must match actual recomposition');return actual;
}

/** A transient authenticated witness is rebuilt from complete archived sources
 * on every reload. No private generated spelling or caller-provided name row
 * is treated as source authority. The semantic archive itself stays ordinary
 * inspectable Links data; this transient marker is not serialized. */
export function createSourceLibraryErrorDeclarationWitness(archive){
 const actual=checkSourceLibraryErrorClosure(archive),owners=actual.owners.map(owner=>Object.freeze({identity:owner.identity,compiledName:owner.compiledName,originalName:owner.originalName,path:owner.path,sourceSha256:owner.sourceSha256}));
 const witness=Object.freeze({sourceSha256:actual.capture.sourceSha256,owners:Object.freeze(owners)});witnesses.add(witness);return witness;
}

export function readSourceLibraryErrorDeclarationWitness(capture,witness){
 if(!witnesses.has(witness)||witness.sourceSha256!==capture.sourceSha256)throw new TypeError('renamed Error declarations require their actual recomposed source witness');return witness.owners;
}
