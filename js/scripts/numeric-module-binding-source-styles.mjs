// The existing compiler diagnostic distinguishes singular and plural SSA names.
// This mechanical alpha rename remains a recorded Links source decorator.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';
export function readNumericModuleBindingSourceStyles(source,family){
 if(family!=='inert_module_binding_rules'||!/^pub fn read_numeric_module_binding\(/mu.test(source))return{source,actions:[]};
 const before=readSourceTextRustBlock(source,'fn','read_numeric_module_binding'),tokens=tokenize(before,'Rust').tokens;
 const oldName='values_2',newName='numeric_literal_decisions',changes=[];
 assert.ok(!tokens.some(token=>token.kind==='identifier'&&token.value===newName));
 assert.ok(!tokens.some(token=>token.kind==='string'&&token.value.includes('{'+oldName)));
 for(let at=0;at<tokens.length;at++)if(tokens[at].kind==='identifier'&&tokens[at].value===oldName){
  assert.ok(!['.','::','fn'].includes(tokens[at-1]?.value)&&tokens[at+1]?.value!=='::');changes.push(tokens[at]);
 }
 assert.ok(changes.length>0,'the actual generated Boolean decision array is present');
 let after=before;for(const token of changes.reverse())after=after.slice(0,token.start)+newName+after.slice(token.end);
 assert.equal(source.split(before).length,2);
 return{source:source.replace(before,()=>after),actions:[{op:'replace',field:'source',from:before,to:after}]};
}
