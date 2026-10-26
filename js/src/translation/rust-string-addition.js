// Source facts only. The shared definition owns dispatch and operand admission.
import {unsupported} from './diagnostics.js';
import {STRING} from './types.js';
import {readRustSourceTypeOwner} from './rust-source-signatures.js';
import {acceptRustStringAdditionCandidate,acceptRustStringAdditionOwner,acceptRustStringAdditionOperand} from './rust-string-addition-rules.js';
export function readRustStringAddition(parser,node,path){
 const left=parser.origins.origin(node.left,path),right=parser.origins.origin(node.right,path);
 const ordinary=parser.ordinaryCompletionContext()&&!parser.localValueStateModel&&!parser.typeNamespaceModel&&!parser.constFunctionModel&&!parser.scalarLoopActive;
 if(!acceptRustStringAdditionCandidate(node.op,ordinary,left.type?.kind??'',right.type?.kind??''))return null;
 if(!parser.origins.collecting)for(const [side,origin] of [['left',left],['right',right]]){
  const owner=origin.sourceOwner??readRustSourceTypeOwner(parser,[origin.textSourceModel==='string slice'?'str':'String'],[],STRING),known=acceptRustStringAdditionOwner(side,owner.kind,owner.path.join('::'),owner.arguments.length);
  if(!acceptRustStringAdditionOperand(side,origin.type?.kind??'',origin.textSourceModel??'',origin.wrappers.length,origin.proven===true,origin.wrappers.every(layer=>layer==='reference'),known))throw unsupported('Rust string addition','an actual owned standard String and one immutable standard str or String reference are required',node.span);
 }
 return parser.origins.mark({...node,op:'concat'},{...parser.origins.typeOrigin(STRING,path),sourceOwner:{kind:'library',path:['std','string','String'],arguments:[]},textSourceModel:'owned string',wrappers:[],sourceLifetimes:[],copyEvidence:false});
}
