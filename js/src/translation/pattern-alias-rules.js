import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
/** @param {string} kind @returns {boolean} */
export function testPatternAliasKind(kind){return kind==='alias';}
/** Select each column's actual inner pattern and bind each alias to that
 * column, without constructing another value. Node identities are local facts,
 * not an assertion that equal source names have globally equivalent meanings.
 * Output: status, root count, selected nodes, then column/node binding pairs.
 * @param {string[]} kinds @param {string[]} names @param {number[]} children
 * @param {number[]} roots @param {string[]} boundNames @returns {number[]} */
export function planPatternAliasBindings(kinds,names,children,roots,boundNames){
 if(kinds.length!==names.length||kinds.length!==children.length)return [-1];
 const selected=[],bindings=[],used=boundNames.slice();
 for(let column=0;column<roots.length;column+=1){
  let node=readRequiredNumberFact(roots,column),steps=0;
  for(;;){
   if(!Number.isInteger(node)||node<0||node>=kinds.length||steps>kinds.length)return [-1];
   if(!testPatternAliasKind(readRequiredTextFact(kinds,node))){selected.push(node);break;}
   const name=readRequiredTextFact(names,node);if(name===''||name==='_'||used.includes(name))return [-1];
   used.push(name);bindings.push(column,node);node=readRequiredNumberFact(children,node);steps+=1;
  }
 }
 return [0,roots.length].concat(selected,bindings);
}
