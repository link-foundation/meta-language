// Actual source declaration owners into the existing checked value construction.
import {typeError,unsupported} from './diagnostics.js';
import {planRustSourceConstantValue,readRustSourceCallableValueError,readRustSourceConstantPatternError,readRustSourceValuePath} from './rust-source-constant-rules.js';
import {readRustCopyClosureCall} from './rust-copy-closures.js';
export function readRustSourceConstantValue(parser,entry,path,span){
 const plan=planRustSourceConstantValue(entry.k,entry.fullName,parser.freshProductBinding());
 if(!plan.length)return null;
 const node={k:plan[0],name:plan[1],value:{k:plan[2],path:['crate',...plan[3].split('.')],span},body:{k:plan[4],path:[plan[5]],span},span};
 return parser.origins.mark(node,entry.result);
}
export function readRustSourceConstantCall(parser,segments,args,path,span){
 if(segments.length===1&&parser.origins.scope.has(segments[0]))return null;
 const entry=parser.origins.lookup(readRustSourceValueName(parser,segments,path),path);if(entry?.k!=='const')return null;
 const error=readRustSourceCallableValueError(entry.result.type?.kind??'','constant');if(error)throw typeError(error,span);
 return readRustCopyClosureCall(parser,readRustSourceConstantValue(parser,entry,path,span),args,path,span);
}

export function requireRustNonconstantBinding(parser,name,path,span){
 const error=readRustSourceConstantPatternError(parser.origins.lookup(readRustSourceValuePath(parser.modulePath,parser.resolve([name],path)),path)?.k??'');if(error)throw unsupported('Rust constant pattern',error,span);
}

export function readRustSourceValueName(parser,segments,path){
 if(segments.length===1&&parser.origins.scope.has(segments[0]))return segments;
 const resolved=parser.resolve(segments,path);return parser.ordinaryCompletionContext()?readRustSourceValuePath(parser.modulePath,resolved):resolved;
}
