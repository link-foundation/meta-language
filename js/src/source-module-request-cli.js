// Process transport over shared argument and CST request definitions.
import {planSourceModuleRequestCommand} from './translation/source-module-request-rules.js';
import {analyzeSourceModuleRequests} from './source-module-requests.js';
import {encodeLinkNetworkBinary} from './link-network-binary.js';
const USAGE='usage: meta-language module-requests <file> [--language <JavaScript|TypeScript>] [--tsx] [--binary]\nOutputs complete CST and module-request links; TypeScript extensions select their grammar automatically.\n';
export function runSourceModuleRequestCommand(args,{readFile}){
 const plan=planSourceModuleRequestCommand(args);
 if(plan.length===1&&plan[0]===-1)return{exitCode:0,stdout:USAGE,stderr:''};
 if(plan.length!==4)return{exitCode:2,stdout:'',stderr:'error: invalid module request arguments\n'+USAGE};
 try{const packet=analyzeSourceModuleRequests(readFile(args[plan[0]]),plan[1]===2?'TypeScript':'JavaScript',{tsx:plan[2]===1});return{exitCode:0,stdout:plan[3]===1?encodeLinkNetworkBinary(packet.network):packet.network.toLino(),stderr:''};}
 catch(error){return{exitCode:1,stdout:'',stderr:'error: '+error.message+'\n'};}
}
