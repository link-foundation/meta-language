// Record and translate the complete partial-callable context source module.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {readProjectData,writeProjectData} from './project-data.mjs';

const sourceFile=new URL('../src/translation/partial-callable-rules.js',import.meta.url);
const policyFiles=[new URL('../src/translation/data/partial-callable-rules.lino',import.meta.url),new URL('../../rust/src/data/partial-callable-rules.lino',import.meta.url)];

export async function generatePartialCallableRules({check=false,native=false}={}) {
  const source=readFileSync(sourceFile,'utf8');
  const policy={schemaVersion:1,model:'partial callable expected result context 1',source,
    sourceSha256:createHash('sha256').update(source).digest('hex'),
    decisions:[...source.matchAll(/^export function (\w+)\(/gmu)].map(match=>match[1])};
  for(const file of policyFiles){
    if(check)assert.ok(isDeepStrictEqual(readProjectData(file),policy),'the complete partial callable context source is fresh');
    else writeProjectData(file,policy);
  }
  if(native){
    const {parseJavaScript}=await import('../src/translation/javascript.js');
    const {checkProgram}=await import('../src/translation/check.js');
    const {emitRust}=await import('../src/translation/emit-rust.js');
    const {writeRecordRuleLibrary}=await import('./record-rule-library.mjs');
    const emitted=emitRust(checkProgram(parseJavaScript(source,{referenceSemantics:false})),{arrayConsumption:true});
    writeRecordRuleLibrary(emitted,'partial_callable_rules',{check});
  }
  return policy;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  await generatePartialCallableRules({check:process.argv.includes('--check'),native:process.argv.includes('--native')});
  console.log('Partial callable expectations retain actual checked branch result types');
}
