// Files and process transport only; argument decisions come from shared rules.
import {planUnsignedSourceCommand,readUnsignedSourceCommandUsage} from './unsigned-source-command-rules.js';
import {createUnsignedSourceDocumentLinks,encodeUnsignedLinkNetworkBinary} from './unsigned-link-network.js';
import {createUnsignedTypedProgramLinks,createUnsignedRustSourceOwnerProgramLinks,emitUnsignedTypedProgramLinks} from './unsigned-typed-program-links.js';
export function runUnsignedSourceCommand(args,{readFile}){
 const plan=planUnsignedSourceCommand(args),usage=readUnsignedSourceCommandUsage();
 if(plan.length===1&&plan[0]==='help')return{exitCode:0,stdout:usage,stderr:''};
 if(plan.length!==7)return{exitCode:2,stdout:'',stderr:'error: invalid exact source arguments\n'+usage};
 try{
  const source=readFile(plan[1]),identityOptions={firstIdentity:plan[3]};
  const packet=plan[0]==='concrete source'?createUnsignedSourceDocumentLinks(source,plan[2],undefined,identityOptions):plan[0]==='Rust source owners'?createUnsignedRustSourceOwnerProgramLinks(source,plan[5]===''?{}:{sourceVersion:plan[5]},identityOptions):createUnsignedTypedProgramLinks(source,plan[2],{},identityOptions);
  return{exitCode:0,stdout:plan[4]!==''?emitUnsignedTypedProgramLinks(packet.network,packet.root,plan[4]).code:plan[6]==='binary'?encodeUnsignedLinkNetworkBinary(packet.network):packet.network.toLino(),stderr:''};
 }catch(error){return{exitCode:1,stdout:'',stderr:'error: '+error.message+'\n'};}
}
