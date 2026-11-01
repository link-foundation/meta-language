// Bounded future-control diagnostics. Definitions and actual measurements are
// separate: an unavailable dependency is an explicit failed assertion.
export function createControlDiagnostics(){
 const assertions=[],failures=[];
 function attempt(assertion,read){
  if(assertions.length>=8192)throw new Error('control assertion budget exceeded');
  assertions.push(assertion);
  try{return read();}catch(error){
   failures.push({assertion,name:String(error?.name??'Error').slice(0,128),message:String(error?.message??error).slice(0,4096),...(typeof error?.kind==='string'?{kind:error.kind}:{}),...(typeof error?.reason==='string'?{reason:error.reason.slice(0,4096)}:{})});
   return undefined;
  }
 }
 return {assertions,failures,attempt};
}
export function requireControlProduct(value,assertion){if(value===undefined)throw new Error('required measured product unavailable: '+assertion);return value;}
export function finishControlDiagnostics(record){if(record.diagnostics.failures.length){const error=new AggregateError(record.diagnostics.failures.map(failure=>new Error(failure.assertion+': '+failure.message)),record.name+': '+record.diagnostics.failures.length+' control assertions failed');error.controlRecord=record;throw error;}return record;}
export function readControlDiagnosticFallback(definition,read){
 try{const record=read();if(!record||!Array.isArray(record.diagnostics?.assertions)||!Array.isArray(record.diagnostics?.failures))throw new Error('control did not supply its declared assertion diagnostics');return record;}catch(error){if(error?.controlRecord)return error.controlRecord;return {...definition,diagnostics:{assertions:['control construction'],failures:[{assertion:'control construction',name:String(error?.name??'Error').slice(0,128),message:String(error?.message??error).slice(0,4096)}]}};}
}
