// Complete ordinary translation only; no native compiler or handwritten kernel.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {originalLinkRolePolicy,originalLinkRoleRulesSource} from './policy.mjs';
export function generateOriginalLinkRoleProjection(check=false){
 const source=originalLinkRoleRulesSource(),program=checkProgram(parseJavaScript(source,{referenceSemantics:originalLinkRolePolicy.translation.referenceSemantics})),emitted=emitRust(program);
 const native={version:1,model:originalLinkRolePolicy.model,sourceSha256:createHash('sha256').update(source).digest('hex'),declarationNames:program.items.map(item=>item.fullName),declarations:emitted.definitions,completeRust:emitted.text};
 for(const[path,value]of[['./data/policy.lino',originalLinkRolePolicy],['./data/native-definitions.lino',native]]){const file=new URL(path,import.meta.url);if(check)checkProjectData(file,value);else writeProjectData(file,value);}
 const file=new URL('./rules.mjs',import.meta.url);if(check)assert.equal(readFileSync(file,'utf8'),source);else writeFileSync(file,source);
 return native;
}
if(process.argv[1]===fileURLToPath(import.meta.url))console.log('Original-ID role projection: '+generateOriginalLinkRoleProjection(process.argv.includes('--check')).declarations.length+' automatically translated native definitions');
