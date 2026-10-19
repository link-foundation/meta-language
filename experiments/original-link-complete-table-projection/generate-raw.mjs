// Staged JavaScript-first generator. --native is explicit and must follow the
// bounded raw JavaScript batch; this file never invokes a native compiler.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {writeProjectData,checkProjectData,readProjectData} from '../../js/scripts/project-data.mjs';
import {originalLinkIdentityRulesSource} from '../original-link-identity/policy.mjs';
import {originalLinkRawPolicy,renderOriginalLinkRawInstructions} from './raw-policy.mjs';
const sha256=value=>createHash('sha256').update(value).digest('hex');
function recordOriginalLinkRawTranslationAttempt(source,operation){
 const path=new URL('./data/raw-translation-attempts.lino',import.meta.url),previous=existsSync(path)?readProjectData(path):{version:1,boundary:'Actual opt-in draft translation attempts. These are source/diagnostic receipts, not native execution or current capability evidence. Earlier unhashed diagnostics are not reconstructed. No check requires a historical failure to remain.',attempts:[]};
 assert.equal(previous.version,1);assert.ok(Array.isArray(previous.attempts));let bytes=Buffer.byteLength(source);for(const attempt of previous.attempts){assert.equal(attempt.sourceSha256,sha256(attempt.source));assert.equal(attempt.sourceBytes,Buffer.byteLength(attempt.source));bytes+=attempt.sourceBytes;}
 if(previous.attempts.length>=32||bytes>1048576)throw new RangeError('raw draft translation attempt log bound');
 writeProjectData(path,{...previous,attempts:[...previous.attempts,{source,sourceSha256:sha256(source),sourceBytes:Buffer.byteLength(source),operation}]});
}
export function generateOriginalLinkRawJavaScript(check=false){
 const path=new URL('./raw-rules.mjs',import.meta.url),before=readFileSync(path,'utf8'),pattern=/\/\* RAW POLICY DATA BEGIN \*\/[\s\S]*?\/\* RAW POLICY DATA END \*\//u;
 assert.equal((before.match(/\/\* RAW POLICY DATA BEGIN \*\//gu)??[]).length,1);assert.ok(before.includes(originalLinkIdentityRulesSource()),'the exact identity kernel source must remain unchanged');const source=before.replace(pattern,()=>renderOriginalLinkRawInstructions());
 const policyPath=new URL('./data/raw-policy.lino',import.meta.url);if(check){assert.equal(before,source,'raw rule instructions are stale');checkProjectData(policyPath,originalLinkRawPolicy);}else{mkdirSync(new URL('./data/',import.meta.url),{recursive:true});writeFileSync(path,source);writeProjectData(policyPath,originalLinkRawPolicy);}return source;
}
export async function generateOriginalLinkRawNative(check=false,{recordAttempt=false}={}){
 if(check&&recordAttempt)throw new TypeError('a freshness check cannot record or mutate translation attempts');
 const source=generateOriginalLinkRawJavaScript(check);console.log('Actual raw translation input SHA256 '+sha256(source)+'; '+Buffer.byteLength(source)+' bytes');
 const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([import('../../js/src/translation/javascript.js'),import('../../js/src/translation/check.js'),import('../../js/src/translation/emit-rust.js')]);
 // No source extraction, native fallback or generated Rust algorithm edits.
 let artifact,stage='parse complete JavaScript';
 try{const surface=parseJavaScript(source,{referenceSemantics:false});stage='check complete program';const program=checkProgram(surface);stage='emit complete Rust text';const rust=emitRust(program);artifact={version:1,model:originalLinkRawPolicy.model,sourceSha256:sha256(source),declarationNames:program.items.map(row=>row.fullName),declarations:rust.definitions,completeRust:rust.text};}
 catch(error){if(recordAttempt)recordOriginalLinkRawTranslationAttempt(source,{kind:'authentic translation diagnostic',stage,name:error.name,diagnosticKind:error.kind??null,reason:error.reason??error.message,span:error.span??null,details:error.details??null});throw error;}
 const path=new URL('./data/raw-native-definitions.lino',import.meta.url);if(check)checkProjectData(path,artifact);else writeProjectData(path,artifact);
 if(recordAttempt)recordOriginalLinkRawTranslationAttempt(source,{kind:'generated complete Rust text',declarations:artifact.declarations.length,rustBytes:Buffer.byteLength(artifact.completeRust),rustSha256:sha256(artifact.completeRust)});return artifact;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const check=process.argv.includes('--check');if(process.argv.includes('--native'))console.log('Raw automatic definitions: '+(await generateOriginalLinkRawNative(check,{recordAttempt:process.argv.includes('--record-attempt')})).declarations.length);else console.log('Raw JavaScript instructions: '+Buffer.byteLength(generateOriginalLinkRawJavaScript(check))+' bytes; native generation not selected');
}
