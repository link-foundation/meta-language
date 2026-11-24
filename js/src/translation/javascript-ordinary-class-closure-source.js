// Complete supplied module receipts and the existing resolved CST composer
// remain the only authority for source aliases and declaration ownership.
import {analyzeProgram} from '../program-representation.js';
import {readSuppliedSourceLibraryClosure} from './source-library-closure.js';
import {projectSourceLibraryOriginalInterval} from './source-library-classes.js';
import {captureJavaScriptOrdinaryClassModule} from './javascript-ordinary-class-source.js';
import {assertPlainSourceReceipt} from './source-receipt-data.js';
import {captureJavaScriptOriginalFunctionSourceOwners} from './javascript-function-source-publication.js';
const model='resolved supplied ordinary class source modules 1',witnesses=new WeakSet();

export function captureJavaScriptOrdinaryClassSourceClosure(closure,{entryPath}={}){
 assertPlainSourceReceipt(closure);
 if(closure?.sourceClasses!=='closed local class declarations 1'||typeof entryPath!=='string'||!Array.isArray(closure.inputs))throw new TypeError('ordinary class source linkage requires its explicit complete supplied closure and entry module');
 const actual=readSuppliedSourceLibraryClosure(entryPath,closure.inputs.map(row=>({path:row.path,source:row.source})),{aliases:true,classes:'closed local class declarations 1',reExports:closure.sourceReExports??''});
 if(JSON.stringify(actual)!==JSON.stringify(closure))throw new TypeError('ordinary class linkage must recompose from every actual original module and lexical alias binding');
 const entry=actual.inputs.find(row=>row.path===entryPath);if(!entry)throw new TypeError('the selected entry must retain its actual supplied module identity');
 const publicRows=entry.exports.map(name=>{const mapping=entry.compiledExports?.find(row=>row.exported===name),compiled=mapping?.compiled??entry.declarationAliases.find(row=>row.original===name)?.compiled,path=mapping?.source??entryPath;if(typeof compiled!=='string')throw new TypeError('each ordinary class module export retains its actual resolved provider binding');return {exported:name,compiled,path};});
 // Class export prefixes are owned and removed by the existing composer.
 // This entry profile calls actual exported FUNCTIONS which can construct,
 // return or invoke their actual supplied classes with ordinary source new.
 if(publicRows.some(row=>actual.inputs.find(input=>input.path===row.path)?.compiledSourceDeclarations.find(declaration=>declaration.name===row.compiled)?.kind!=='function'))throw new TypeError('public direct class entry construction needs its separate supplied source entry operation');
 const capture=captureJavaScriptOrdinaryClassModule(actual.source,{identifier:entryPath,entries:publicRows.map(row=>row.compiled)}),owners=[];
 for(const definition of capture.plan.functions){
  const origin=capture.origins.find(row=>row.identity===definition.identity);if(!origin)throw new TypeError('every portable callable needs its actual complete CST source origin');
  if(origin.classBinding!==undefined&&origin.constructor!==true)continue;
  const matches=actual.inputs.flatMap(input=>(input.compiledSourceDeclarations??[]).filter(row=>row.name===definition.name).map(row=>({input,row})));
  if(matches.length!==1)throw new TypeError('each supplied class/function needs exactly one actual declaration owner');
  const {input,row}=matches[0],alias=input.declarationAliases.filter(alias=>alias.compiled===definition.name);if(alias.length!==1)throw new TypeError('each supplied callable needs exactly one actual original lexical declaration binding');
  const kind=definition.$==='errorOrdinaryConstructorPlan'?'class':'function',program=analyzeProgram(input.source,'JavaScript'),binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.kind===kind&&binding.name===alias[0].original),syntax=binding&&program.sourceMappings.filter(node=>node.term===(kind==='class'?'class_declaration':'function_declaration')&&node.start<=binding.declaration.start&&node.end>=binding.declaration.end).sort((a,b)=>(a.end-a.start)-(b.end-b.start))[0],range=input.closureRanges.find(range=>origin.start>=range.start&&origin.end<=range.end);
  if(!syntax||!range||row.kind!==kind)throw new TypeError('a supplied callable needs its actual complete original CST declaration interval');
  const projected=projectSourceLibraryOriginalInterval(input,range.sourceStart+origin.start-range.start,range.sourceStart+origin.end-range.start);if(projected.start<syntax.start||projected.end>syntax.end)throw new TypeError('a supplied callable cannot acquire another module declaration owner');
  owners.push({identity:definition.identity,compiledName:definition.name,originalName:alias[0].original,kind,path:input.path,sourceSha256:input.sourceSha256,originalSpan:{start:syntax.start,end:syntax.end,coordinateUnit:'UTF-16 code units'},compiledSpan:{start:origin.start,end:origin.end,coordinateUnit:'UTF-16 code units'}});
 }
 for(const definition of capture.plan.functions){const origin=capture.origins.find(row=>row.identity===definition.identity);if(origin?.classBinding===undefined||origin.constructor===true)continue;
  const owner=owners.find(row=>row.identity===origin.classBinding),input=owner&&actual.inputs.find(row=>row.path===owner.path),range=input?.closureRanges.find(range=>origin.start>=range.start&&origin.end<=range.end);if(!owner||!input||!range)throw new TypeError('an instance method needs its actual supplied class and source range');
  const projected=projectSourceLibraryOriginalInterval(input,range.sourceStart+origin.start-range.start,range.sourceStart+origin.end-range.start),program=analyzeProgram(input.source,'JavaScript'),member=program.sourceMappings.find(row=>row.term==='method_definition'&&row.start===projected.start&&row.end===projected.end);
  if(!member||member.start<owner.originalSpan.start||member.end>owner.originalSpan.end)throw new TypeError('an instance method cannot acquire another original class/member owner');
  owners.push({identity:definition.identity,compiledName:definition.name,originalName:definition.name,kind:'method',classBinding:origin.classBinding,path:input.path,sourceSha256:input.sourceSha256,originalSpan:projected,compiledSpan:{start:origin.start,end:origin.end,coordinateUnit:'UTF-16 code units'}});
 }
 const entries=publicRows.map(row=>{const matches=owners.filter(owner=>owner.path===row.path&&owner.compiledName===row.compiled&&owner.kind==='function');if(matches.length!==1)throw new TypeError('a public entry cannot acquire an unrelated composed helper or method');return {exported:row.exported,identity:matches[0].identity,compiledName:row.compiled,path:row.path};});
 return {schemaVersion:1,artifactKind:'authenticated semantic source plan/input 1',model,entryPath,sourceClosure:actual,capture,owners,entries,completeSourceReceipts:actual.inputs.map(input=>({path:input.path,source:input.source,sourceSha256:input.sourceSha256,bytes:input.bytes,closureRanges:input.closureRanges,sourceCompositionSteps:input.sourceCompositionSteps}))};
}
export function checkJavaScriptOrdinaryClassSourceClosure(archive){assertPlainSourceReceipt(archive);const actual=captureJavaScriptOrdinaryClassSourceClosure(archive.sourceClosure,{entryPath:archive.entryPath});if(JSON.stringify(actual)!==JSON.stringify(archive))throw new TypeError('ordinary class module entries, aliases and complete source receipts must match actual CST recomposition');return actual;}
export function createJavaScriptOrdinaryClassDeclarationWitness(archive){
 const actual=checkJavaScriptOrdinaryClassSourceClosure(archive),owners=captureJavaScriptOriginalFunctionSourceOwners(actual).map(owner=>Object.freeze({identity:owner.identity,compiledName:owner.compiledName,originalName:owner.originalName,path:owner.path,sourceSha256:owner.sourceSha256,sourceUnit:Object.freeze({...owner.sourceUnit}),originalSpan:Object.freeze({...owner.originalSpan}),templates:Object.freeze(owner.templates.map(site=>Object.freeze({...site})))}));
 const witness=Object.freeze({sourceSha256:actual.capture.sourceSha256,owners:Object.freeze(owners)});witnesses.add(witness);return witness;
}
export function readJavaScriptOrdinaryClassDeclarationWitness(capture,witness){if(!witnesses.has(witness)||witness.sourceSha256!==capture.sourceSha256)throw new TypeError('ordinary class/function alias names require their actual complete recomposed source witness');return witness.owners;}
