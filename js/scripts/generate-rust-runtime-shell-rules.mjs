import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
const policy=readProjectData(new URL('../src/translation/data/rust-runtime-shell.lino',import.meta.url));
const source='// Generated from linked closed Rust runtime-shell decisions. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustRuntimeShellModel(model){return model === "" || model === '+JSON.stringify(policy.model)+';}\n'
 +'/** @param {string} name @returns {boolean} */\nexport function acceptRustRuntimeShellReservedName(name){return !('+policy.reservedNames.map(name=>'name === '+JSON.stringify(name)).join(' || ')+');}\n'
 +'/** @param {number} value @returns {boolean} */\nexport function acceptRustRuntimeShellStackShift(value){return value >= '+policy.semantics.thread.stackShiftMinimum+' && value <= '+policy.semantics.thread.stackShiftMaximum+' && value % 1 === 0;}\n';
const file=new URL('../src/translation/rust-runtime-shell-rules.js',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(file,'utf8')!==source)throw Error('closed runtime-shell decisions are stale');}else writeFileSync(file,source);
