// Capture actual parsed ancestry and source atoms for the shared decision.
import {unsupported} from './diagnostics.js';
import {readRustScalarArrayIndexOperandError} from './rust-array-index-rules.js';
import {readRustUsizeIndexExpressionError} from './rust-usize-index-expression-rules.js';
export function readRustUsizeIndexExpression(parser,index,path){
 const kinds=[],operators=[],atoms=[],literals=[],parents=[],pending=[{node:index,parent:-1,depth:0}];
 while(pending.length){
  const {node,parent,depth}=pending.pop();
  if(kinds.length>=4096||depth>128)throw unsupported('Rust usize index expression','the index expression exceeds its source '+(depth>128?'depth':'node')+' limit',index.span);
  const structural=['binary','if','match'].includes(node.k);
  const origin=structural?null:parser.origins.origin(node,path),position=kinds.length;
  const owner=origin?.sourceOwner?.kind==='primitive'?origin.sourceOwner.path.join('::'):'';
  kinds.push(node.k);operators.push(node.op??'');parents.push(parent);
  atoms.push(!structural&&readRustScalarArrayIndexOperandError(false,origin.type?.kind??'',origin.type?.bits??0,origin.type?.signed===true,owner,origin.wrappers.length,origin.proven===true)==='');
  literals.push(node.k==='num'&&!node.type&&!node.negative);
  const children=node.k==='binary'?[node.left,node.right]:node.k==='if'?[node.then,node.else]:node.k==='match'?node.rows.map(row=>row.body):[];
  for(let child=children.length-1;child>=0;child-=1)pending.push({node:children[child],parent:position,depth:depth+1});
 }
 return readRustUsizeIndexExpressionError(kinds,operators,atoms,literals,parents);
}
