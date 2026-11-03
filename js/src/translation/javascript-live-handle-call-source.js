// Boundary coordinators reuse the complete existing frame-owned argument List
// algorithm. Their private owner identities never replace represented callees.
import {compileJavaScriptFunctionInvocationFunction} from './javascript-function-invocation-call-source.js';
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptLiveHandleInvocationFunction(identity,realm,operation){
 if(operation==='get property'||operation==='set property')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:6,captures:[],instructions:[{$:'copy',destination:1,source:0},{$:'argument',destination:2,index:0},{$:'argument',destination:3,index:1},...(operation==='set property'?[{$:'argument',destination:4,index:2},{$:'ordinarySet',destination:5,object:1,key:3,value:4,receiver:2}]:[{$:'ordinaryGet',destination:5,object:1,key:3,receiver:2}]),{$:'return',source:5}]};
 if(operation!=='call'&&operation!=='construct')return undefined;
 const source=compileJavaScriptFunctionInvocationFunction(identity,realm,'Function.call');if(source===undefined||operation==='call')return source;
 // Keep instruction coordinates unchanged by placing both constructor checks
 // before the existing body, then shifting every absolute control destination.
 /** @type {JavaScriptCallInstruction[]} */const prefix=[{$:'copy',destination:1,source:0},{$:'argument',destination:2,index:0},{$:'constructorCheck',destination:6,input:1,required:true},{$:'constructorCheck',destination:6,input:2,required:true}];const offset=prefix.length;
 const instructions=source.instructions.map(instruction=>instruction.$==='jump'?{...instruction,target:instruction.target+offset}:instruction.$==='branch'?{...instruction,yes:instruction.yes+offset,no:instruction.no+offset}:instruction.$==='argumentListCall'?{...instruction,mode:'construct'}:instruction);
 return {...source,instructions:prefix.concat(instructions)};
}
