import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {originalLinkCompleteTablePolicy,originalLinkCompleteTableRulesSource} from './policy.mjs';
export function generateOriginalLinkCompleteTableProjection(check=false){const source=originalLinkCompleteTableRulesSource(),program=checkProgram(parseJavaScript(source,{referenceSemantics:originalLinkCompleteTablePolicy.translation.referenceSemantics})),emitted=emitRust(program);const artifact={version:1,model:originalLinkCompleteTablePolicy.model,sourceSha256:createHash('sha256').update(source).digest('hex'),declarationNames:program.items.map(item=>item.fullName),declarations:emitted.definitions,completeRust:emitted.text};for(const[path,value]of[['./data/policy.lino',originalLinkCompleteTablePolicy],['./data/native-definitions.lino',artifact]]){const file=new URL(path,import.meta.url);if(check)checkProjectData(file,value);else writeProjectData(file,value);}const file=new URL('./rules.mjs',import.meta.url);if(check)assert.equal(readFileSync(file,'utf8'),source);else writeFileSync(file,source);return artifact;}
if(process.argv[1]===fileURLToPath(import.meta.url))console.log('Complete original-ID tables: '+generateOriginalLinkCompleteTableProjection(process.argv.includes('--check')).declarations.length+' automatically translated native definitions');
