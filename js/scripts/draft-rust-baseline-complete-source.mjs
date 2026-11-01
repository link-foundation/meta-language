// Future source generation only. Do not invoke during source-edit-only drafting.
// Fresh processes prevent a predecessor schema/classifier cached by static
// imports from selecting the successor's source interpretation.
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
export const rustBaselineSourceAuthoringSteps=Object.freeze([
 Object.freeze(['js/scripts/draft-class-owner-source-profiles.mjs','--draft']),
 Object.freeze(['js/scripts/generate-typed-program-link-rules.mjs','--draft']),
 Object.freeze(['js/scripts/draft-rust-baseline-source-policies.mjs','--draft']),
 Object.freeze(['js/scripts/draft-javascript-call-machine.mjs','--draft']),
 Object.freeze(['js/scripts/draft-javascript-call-source.mjs','--draft']),
]);
export function draftRustBaselineCompleteSource({run=spawnSync}={}){
 const cwd=fileURLToPath(new URL('../../',import.meta.url));
 for(const [script,...args]of rustBaselineSourceAuthoringSteps){
  const result=run(process.execPath,['--expose-gc',script,...args],{cwd,stdio:'inherit',env:{...process.env}});
  if(result.error)throw result.error;
  if(result.signal||result.status!==0)throw new Error('source authoring stopped at '+script+(result.signal?' ('+result.signal+')':' (exit '+String(result.status)+')'));
 }
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft only after complete source drafting permits generation');
 draftRustBaselineCompleteSource();
}
