// Generate data and the complete automatically translated experiment; no native compilation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {originalLinkIdentityPolicy,originalLinkIdentityRulesSource} from './policy.mjs';

export function originalLinkIdentityNativeDefinitions(){
 const source=originalLinkIdentityRulesSource(),program=checkProgram(parseJavaScript(source)),emitted=emitRust(program);
 return {schemaVersion:1,model:originalLinkIdentityPolicy.model,sourceSha256:createHash('sha256').update(source).digest('hex'),declarations:emitted.definitions};
}
export function generateOriginalLinkIdentity(check=false){
 const source=originalLinkIdentityRulesSource(),native=originalLinkIdentityNativeDefinitions();
 const policy=new URL('./data/policy.lino',import.meta.url),javascript=new URL('./rules.mjs',import.meta.url),definitions=new URL('./data/native-definitions.lino',import.meta.url);
 if(check){checkProjectData(policy,originalLinkIdentityPolicy);assert.equal(readFileSync(javascript,'utf8'),source);checkProjectData(definitions,native);}
 else{writeProjectData(policy,originalLinkIdentityPolicy);writeFileSync(javascript,source);writeProjectData(definitions,native);}
 return native;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const result=generateOriginalLinkIdentity(process.argv.includes('--check'));
 console.log('Exact original-link identity kernel: '+result.declarations.length+' automatically translated native definitions');
}
