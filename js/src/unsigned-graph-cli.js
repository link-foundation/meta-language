// Saved graph and file transports; all root/argument/bound decisions are shared.
import {UnsignedLinkId,UnsignedLinkNetwork,createDenseUnsignedLinkNetwork,rebaseUnsignedLinkNetwork,encodeUnsignedLinkNetworkBinary,decodeUnsignedLinkNetworkBinary} from './unsigned-link-network.js';
import {emitUnsignedTypedProgramLinks} from './unsigned-typed-program-links.js';
import {LinkId} from './primitives.js';
import {BinaryLinksError} from './link-network-binary.js';
import {planUnsignedGraphCommand,readUnsignedGraphCommandUsage,planUnsignedTypedGraphRoot,readUnsignedGraphReadBoundsError,readUnsignedTypedRootBoundsError,readUnsignedGraphTextSurfaceError} from './unsigned-graph-command-rules.js';
const encoder=new TextEncoder();
/** Read exactly the supplied snapshot; source parsers and source restoration are
 * never inputs to this operation. The complete graph remains editable. */
export function readUnsignedGraphSnapshot(source,encoding='text',options={}){
 const byteLimit=options.byteLimit??128*1024*1024,recordLimit=options.recordLimit??100000;
 if(encoding==='binary')return decodeUnsignedLinkNetworkBinary(source,options);
 if(encoding!=='text'||typeof source!=='string')throw new TypeError('a saved graph needs explicit text or binary input');
 let reason=readUnsignedGraphReadBoundsError(source.length,0,byteLimit,recordLimit);if(reason!=='')throw new BinaryLinksError(reason);
 const bytes=encoder.encode(source).length;reason=readUnsignedGraphReadBoundsError(bytes,0,byteLimit,recordLimit);if(reason!=='')throw new BinaryLinksError(reason);
 reason=readUnsignedGraphTextSurfaceError(source);if(reason!=='')throw new BinaryLinksError(reason);
 const graph=UnsignedLinkNetwork.fromLino(source);reason=readUnsignedGraphReadBoundsError(bytes,graph.len(),byteLimit,recordLimit);if(reason!=='')throw new BinaryLinksError(reason);
 createDenseUnsignedLinkNetwork(graph,options);return graph;
}
/** Select one actual stored schema root; multiple roots require a choice. */
export function selectUnsignedTypedProgramRoot(network,requested=undefined,{recordLimit=100000}={}){
 if(!(network instanceof UnsignedLinkNetwork))throw new TypeError('expected an unsigned link network');
 const reason=readUnsignedTypedRootBoundsError(network.len(),recordLimit);if(reason!=='')throw new TypeError(reason);
 let request='';
 if(requested!==undefined){
  if(requested instanceof UnsignedLinkId||requested instanceof LinkId)request=UnsignedLinkId.from(requested).toString();
  else{const kind=typeof requested;request=kind==='string'&&requested!==''||kind==='number'&&Number.isSafeInteger(requested)||kind==='bigint'?String(requested):'@invalid';}
 }
 const links=network.links(),plan=planUnsignedTypedGraphRoot(links.map(link=>String(link.id())),links.map(link=>link.metadata().linkType??''),links.map(link=>link.metadata().language??''),links.map(link=>link.metadata().term??''),links.map(link=>link.metadata().definition??''),request,recordLimit);
 if(plan[0]!=='root')throw new TypeError(plan[1]);return UnsignedLinkId.from(plan[1]);
}
export function runUnsignedGraphCommand(args,{readFile,readBytes}){
 const plan=planUnsignedGraphCommand(args),usage=readUnsignedGraphCommandUsage();
 if(plan.length===1&&plan[0]==='help')return{exitCode:0,stdout:usage,stderr:''};
 if(plan.length!==6)return{exitCode:2,stdout:'',stderr:'error: invalid exact graph arguments\n'+usage};
 try{
  let network=plan[1]==='binary'?readUnsignedGraphSnapshot(readBytes(plan[0]),'binary'):readUnsignedGraphSnapshot(readFile(plan[0]),'text');
  let root=plan[3]===''?undefined:selectUnsignedTypedProgramRoot(network,plan[4]===''?undefined:plan[4]);
  if(plan[5]!==''){const rebased=rebaseUnsignedLinkNetwork(network,plan[5]);if(root!==undefined){const at=rebased.sourceIdentities.findIndex(id=>id.equals(root));if(at<0)throw new TypeError('a saved typed graph lost its actual root during coordinate allocation');root=rebased.identities[at];}network=rebased.network;}
  return{exitCode:0,stdout:plan[3]!==''?emitUnsignedTypedProgramLinks(network,root,plan[3]).code:plan[2]==='binary'?encodeUnsignedLinkNetworkBinary(network):network.toLino(),stderr:''};
 }catch(error){return{exitCode:1,stdout:'',stderr:'error: '+error.message+'\n'};}
}
