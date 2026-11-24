// Preserve every actual supplied source unit for independent native compilation.
// Encoding is a future control operation, not execution evidence on import.
import * as codecs from '../../../experiments/javascript-properties/error-class-source-codecs.mjs';
export function encodeSourceModuleCompilation(module){
 let count=0;
 function encode(unit,depth){
  if(depth>32||++count>64)throw new RangeError('actual source compilation report exceeds its explicit unit budget');
  return {
   plan:codecs.encodeJavaScriptErrorModulePlan(unit.capture.plan),
   registry:codecs.encodeJavaScriptErrorModuleRegistry(unit.registry),
   bindings:(unit.bindings??[]).map(codecs.encodeJavaScriptErrorModuleBinding),
   capabilities:(unit.capabilities??[]).map(codecs.encodeJavaScriptSourceCapabilityOwner),
   expected:codecs.encodeJavaScriptErrorModuleCompilation({$:'errorModuleCompilation',program:unit.sourceProgram??unit.program,error:''}),
   sourceUnits:(unit.sourceUnits??[]).map(child=>({capture:{model:child.capture.model,identifier:child.capture.identifier,source:child.capture.source,sourceSha256:child.capture.sourceSha256},compilation:encode(child,depth+1)})),
  };
 }
 return encode(module,0);
}
