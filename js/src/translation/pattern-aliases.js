// Actual normalized pattern objects into the shared alias-column plan.
import {testPatternAliasKind,planPatternAliasBindings} from './pattern-alias-rules.js';
import {typeError} from './diagnostics.js';
export function projectPatternAliasRow(row,columns,span){
 if(!row.patterns.some(pattern=>testPatternAliasKind(pattern.k)))return row;
 const nodes=[],kinds=[],names=[],children=[],roots=[],positions=new Map();
 for(const root of row.patterns){
  let node=root,parent=null;
  for(;;){
   if(positions.has(node)){const index=positions.get(node);if(parent===null)roots.push(index);else children[parent]=index;break;}
   const index=nodes.length;positions.set(node,index);nodes.push(node);kinds.push(node.k);names.push(node.name??'');children.push(-1);
   if(parent===null)roots.push(index);else children[parent]=index;
   if(!testPatternAliasKind(node.k))break;parent=index;node=node.pattern;
  }
 }
 const plan=planPatternAliasBindings(kinds,names,children,roots,row.binds.map(([name])=>name));
 if(plan[0]!==0||plan[1]!==columns.length)throw typeError('a pattern alias needs complete distinct source bindings and column roots',span);
 const patterns=plan.slice(2,2+columns.length).map(index=>nodes[index]),binds=[...row.binds];
 for(let at=2+columns.length;at<plan.length;at+=2)binds.push([names[plan[at+1]],columns[plan[at]].name]);
 return {...row,patterns,binds};
}
