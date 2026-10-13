// JavaScript-only experimental decisions; native generation follows review.
import {fileURLToPath} from 'node:url';
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData} from '../js/scripts/project-data.mjs';
const policy=readProjectData(new URL('../js/src/translation/data/rust-owned-local-state.lino',import.meta.url));
export function rustOwnedLocalStatePortableRules(){
 const ranges=policy.textCodeUnits;
 const textInput=`function acceptRustOwnedLocalTextInput(text){
 if(typeof text !== "string")return false;
 for(let index=0;index<text.length;index++){
  const unit=text.charCodeAt(index);
  if(unit>=${ranges.highStart}&&unit<=${ranges.highEnd}){
   index++;
   const next=text.charCodeAt(index);
   if(!(next>=${ranges.lowStart}&&next<=${ranges.lowEnd}))return false;
  }else if(unit>=${ranges.lowStart}&&unit<=${ranges.lowEnd})return false;
 }
 return true;
}`;
 return '// Generated from linked owned local state policy. Do not edit.\n'
 + '/** @param {string} model @returns {boolean} */\nexport function acceptRustOwnedLocalStateModel(model){return model === "" || model === '+JSON.stringify(policy.models.moves)+' || model === '+JSON.stringify(policy.models.copies)+';}\n'
 + '/** @param {string} kind @param {string} bits @param {boolean} signed @returns {boolean} */\nexport function acceptRustOwnedLocalValueType(kind,bits,signed){return kind === "string" || kind === "bool" || (kind === "fixed" && bits === "32" && signed);}\n'
 + '/** @param {string} kind @returns {boolean} */\nexport function acceptRustOwnedLocalExpression(kind){return '+Object.keys(policy.expressions).map(kind=>'kind === '+JSON.stringify(kind)).join(' || ')+';}\n'
 + '/** @param {string} operation @returns {boolean} */\nexport function acceptRustOwnedLocalBinaryOperation(operation){return operation === "eq";}\n'
 + '/** @param {Array<number>} units @returns {boolean} */\nexport function acceptRustOwnedLocalTextUnits(units){\n for(let index=0;index<units.length;index++){\n  const unit=units[index]??-1;\n  if(unit>='+ranges.highStart+'&&unit<='+ranges.highEnd+'){\n   index++;\n   const next=units[index]??-1;\n   if(!(next>='+ranges.lowStart+'&&next<='+ranges.lowEnd+'))return false;\n  }else if(unit>='+ranges.lowStart+'&&unit<='+ranges.lowEnd+')return false;\n }\n return true;\n}\n'
 + '/** @returns {string} */\nexport function renderRustOwnedLocalTextInput(){return '+JSON.stringify(textInput.replace('acceptRustOwnedLocalTextInput','ml_ownedTextInput'))+';}\n';
}
export function rustOwnedLocalStateRules(){
 return rustOwnedLocalStatePortableRules()
 + '/** Convert only actual primitive strings through the selected standard realm. */\n'
 + '/** @param {string} text @returns {boolean} */\n'
 + 'export function acceptRustOwnedLocalTextInput(text){\n'
 + ' if(typeof text !== "string")return false;\n'
 + ' const units=[];\n'
 + ' for(let index=0;index<text.length;index++)units.push(text.charCodeAt(index));\n'
 + ' return acceptRustOwnedLocalTextUnits(units);\n}\n';
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const output=new URL('../js/src/translation/rust-owned-local-state-rules.js',import.meta.url),source=rustOwnedLocalStateRules();
 if(process.argv.includes('--check')){if(readFileSync(output,'utf8')!==source)throw new Error('owned local state decisions are stale');}else writeFileSync(output,source);
}
