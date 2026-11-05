// Process I/O around the automatically shared argument and composition plans.
import {readRustSourceProjectCommandPlan} from './rust-project-source-rules.js';
import {readStructuredDataLino} from './structured-data-links.js';
import {createRustSourceProjectProgramLinks} from './rust-source-project.js';
import {emitTypedProgramLinks} from './typed-program-links.js';
import {encodeLinkNetworkBinary} from './link-network-binary.js';
export const RUST_SOURCE_PROJECT_USAGE='usage: meta-language source-project <inventory.lino> --entry <path> [--library] [--to <JavaScript|Rust|Lean|Rocq> | --binary]\nThe inventory contains inputs (original path/source records) and optional resource options. Default output retains checked semantics and complete original concrete graphs as Links Notation.\n';
export function runRustSourceProjectCommand(args,{readFile}){
 let plan;try{plan=readRustSourceProjectCommandPlan(args);}catch(error){return {exitCode:2,stdout:'',stderr:'error: '+error.message+'\n'+RUST_SOURCE_PROJECT_USAGE};}
 if(plan.length===0)return {exitCode:0,stdout:RUST_SOURCE_PROJECT_USAGE,stderr:''};
 try{const [input,entry,target,binary,library]=plan,value=readStructuredDataLino(readFile(input));
  if(!value||typeof value!=='object'||Array.isArray(value))throw new TypeError('Rust source project: expected a source inventory record');
  if(value.options!==undefined&&value.options!==null&&(typeof value.options!=='object'||Array.isArray(value.options)))throw new TypeError('Rust source project: expected a resource options record');
  const options={...(value.options??{}),...(library==='true'?{library:true}:{})};
  const packet=createRustSourceProjectProgramLinks(value.inputs,entry,options);
  const stdout=target?emitTypedProgramLinks(packet.network,packet.root,target).code:binary==='true'?Buffer.from(encodeLinkNetworkBinary(packet.network)):packet.network.toLino();
  return {exitCode:0,stdout,stderr:''};
 }catch(error){return {exitCode:1,stdout:'',stderr:'error: '+error.message+'\n'};}
}
