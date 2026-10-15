// JavaScript-first private draft; no native port is produced by this experiment.
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../js/scripts/project-data.mjs';
export const ownedTupleCallPolicy=readProjectData(new URL('../js/src/translation/data/rust-owned-tuple-calls.lino',import.meta.url));
export function rustOwnedTupleCallRules(){return '// Generated from linked owned tuple call policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\n'
 +'export function acceptRustOwnedTupleCallModel(model){return model === '+JSON.stringify(ownedTupleCallPolicy.model)+';}\n'
 +'/** @param {boolean} target @param {boolean} argumentsMatch @param {boolean} fieldsMatch @returns {boolean} */\n'
 +'export function acceptRustOwnedTupleCallInitializer(target,argumentsMatch,fieldsMatch){return target&&argumentsMatch&&fieldsMatch;}\n';}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../js/src/translation/rust-owned-tuple-call-rules.js',import.meta.url),source=rustOwnedTupleCallRules();
 if(process.argv.includes('--check')){if(readFileSync(file,'utf8')!==source)throw new Error('owned tuple call rules are stale');}else writeFileSync(file,source);
}
