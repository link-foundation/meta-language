// Node frontend metadata comes from the same checked capture/CST and original
// module receipts as executable publication. It never evaluates source bodies.
import {analyzeProgram} from '../program-representation.js';
import {projectSourceLibraryOriginalInterval} from './source-library-classes.js';
import {constructJavaScriptSourceText,testJavaScriptSemanticTextEqual} from './javascript-semantic-values.js';
/** @param {object} capture @returns {object[]} */
export function captureJavaScriptFunctionTemplateOrigins(capture){
 if((capture.plan.templates??[]).length===0)return [];
 const program=analyzeProgram(capture.source,'JavaScript'),sites=term=>[...new Map(program.sourceMappings.filter(row=>row.term===term).map(row=>[row.start+':'+row.end,row])).values()].sort((a,b)=>a.start-b.start||b.end-a.end),arrows=sites('arrow_function'),methods=sites('method_definition');
 return capture.plan.templates.map(template=>{const node=template.ordinal<arrows.length?arrows[template.ordinal]:methods[template.ordinal-arrows.length];if(node===undefined)throw new TypeError('function source metadata retains its actual CST lexical site');return {owner:template.owner,ordinal:template.ordinal,start:node.start,end:node.end};});
}
/** Private witness creators supply an already fully checked archive. Missing
 * generated-to-original sites remain unavailable rather than inventing text.
 * @param {object} archive @returns {object[]} */
export function captureJavaScriptOriginalFunctionSourceOwners(archive){
 const sites=captureJavaScriptFunctionTemplateOrigins(archive.capture),syntax=new Map();
 return archive.owners.map(owner=>{
  const input=archive.sourceClosure.inputs.find(row=>row.path===owner.path);if(input===undefined)throw new TypeError('original function source needs its actual complete module receipt');
  const templates=[];for(const site of sites.filter(row=>row.owner===owner.identity)){
   const range=input.closureRanges.find(row=>site.start>=row.start&&site.end<=row.end);if(range===undefined)continue;
   let span;try{span=projectSourceLibraryOriginalInterval(input,range.sourceStart+site.start-range.start,range.sourceStart+site.end-range.start);}catch(error){if(error instanceof TypeError)continue;throw error;}let program=syntax.get(input.path);if(program===undefined){program=analyzeProgram(input.source,'JavaScript');syntax.set(input.path,program);}
   if(!program.sourceMappings.some(row=>['arrow_function','method_definition'].includes(row.term)&&row.start===span.start&&row.end===span.end))continue;
   if(span.start<owner.originalSpan.start||span.end>owner.originalSpan.end)continue;templates.push({ordinal:site.ordinal,start:span.start,end:span.end});
  }
  return {...owner,sourceUnit:{identifier:input.path+'#sha256:'+input.sourceSha256,text:input.source},templates};
 });
}
/** @param {JavaScriptCallProgram} program @param {object} capture @param {object} registry @param {object[]|null} [originalOwners] @returns {JavaScriptCallProgram} */
export function publishJavaScriptFunctionSources(program,capture,registry,originalOwners=null){
 const sources=[...(program.sources??[])],sites=captureJavaScriptFunctionTemplateOrigins(capture),add=(identifier,raw)=>{const text=constructJavaScriptSourceText(raw),prior=sources.find(row=>row.identifier===identifier);if(prior!==undefined&&!testJavaScriptSemanticTextEqual(prior.text,text))throw new TypeError('one source unit cannot acquire conflicting complete source text');if(prior===undefined)sources.push({$:'callSourceUnit',identifier,text});return identifier;};
 const mappings=new Map();for(const owner of registry.owners){const origin=capture.origins.find(row=>row.identity===owner.source);if(origin===undefined)continue;
  if(originalOwners!==null){const original=originalOwners.find(row=>row.identity===owner.source&&row.compiledName===capture.plan.functions.find(definition=>definition.identity===owner.source)?.name);if(original?.sourceUnit===undefined||original.originalSpan===undefined)continue;const unit=add(original.sourceUnit.identifier,original.sourceUnit.text);mappings.set(owner.identity,{span:{$:'callSourceSpan',unit,start:original.originalSpan.start,end:original.originalSpan.end},templates:original.templates??[]});}
  else{const unit=add(capture.identifier+'#sha256:'+capture.sourceSha256,capture.source);mappings.set(owner.identity,{span:{$:'callSourceSpan',unit,start:origin.start,end:origin.end},templates:sites.filter(row=>row.owner===owner.source)});}
 }
 const functions=program.functions.map(fn=>{const mapped=mappings.get(fn.identity);return mapped===undefined?fn:{...fn,source:mapped.span};}),templates=program.templates?.map(template=>{const owner=mappings.get(template.owner),site=owner?.templates.find(row=>row.ordinal===template.ordinal);return site===undefined?template:{...template,source:{$:'callSourceSpan',unit:owner.span.unit,start:site.start,end:site.end}};});
 return {...program,functions,...(templates===undefined?{}:{templates}),...(sources.length===0&&program.sources===undefined?{}:{sources})};
}
