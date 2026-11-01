// Actual expression attachment only. Positions never carry runtime region IDs.
import {unsupported} from './diagnostics.js';
import {measureRustSourceOriginPositionMetadata} from './rust-standard-sum-source-metadata.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {qualifyRustStandardSumSourcePosition} from './rust-standard-sum-source.js';
import {readRustBorrowedExpressionPositionReservation,planRustBorrowedInherentPosition} from './rust-borrowed-vector-rules.js';
import {acceptRustOwnedBoxTypeNode} from './rust-owned-recursive-datatype-rules.js';
export function prepareRustBorrowedInherentReceiverPosition(table,origin,wrappers,span){
 if(!table.borrowedVectorSourceEnabled)return {...origin,wrappers};
 const plan=planRustBorrowedInherentPosition(origin.wrappers,wrappers);if(plan.length!==3)throw unsupported('Rust borrowed expression source position','the selected receiver needs its complete source layer projection',span);
 const project=owner=>{let result=owner;for(let index=0;index<plan[0];index+=1){if(!result||!acceptRustOwnedBoxTypeNode(['box'],result.kind,result.path.join('::'),result.arguments.length))throw unsupported('Rust borrowed expression source position','receiver dereference needs its actual canonical Box owner',span);result=result.arguments[0];}return result;};
 const lifetimes=[...Array(plan[2]).fill(null),...(origin.sourceLifetimes??[]).slice(plan[1])];
 return {...origin,wrappers,sourceOwner:project(origin.sourceOwner),...(origin.expandedSourceOwner?{expandedSourceOwner:project(origin.expandedSourceOwner)}:{}),sourceLifetimes:lifetimes};
}
export function noteRustBorrowedExpressionSourcePosition(parser,node,path){
 const table=parser.origins;if(!table.borrowedVectorSourceEnabled)return node;
 const origin=table.origin(node,path),relevant=origin.wrappers.length>0||Boolean(origin.arrayElementPosition)||Boolean(origin.sourceBoxLayers?.length);
 if(!origin.type||!origin.sourceOwner||!origin.proven||origin.diverging){if(relevant&&!origin.diverging){table.borrowedVectorNeedsFinalAdmission=true;if(!table.collecting)throw unsupported('Rust borrowed expression source position','a qualified expression needs its actual complete source type and owner before reference erasure',node.span);}return node;}
 let cost;try{cost=measureRustSourceOriginPositionMetadata(origin);}catch(error){if(!(error instanceof RangeError))throw error;throw unsupported('Rust borrowed expression source position',error.message,node.span);}
 const next=readRustBorrowedExpressionPositionReservation(table.sourcePositionMaterializedNodes,table.sourcePositionMaterializedUnits,cost[0],cost[1]);
 if(next.length!==2)throw unsupported('Rust borrowed expression source position','the owned expression source positions exceed their materialization budget',node.span);
 table.sourcePositionMaterializedNodes=next[0];table.sourcePositionMaterializedUnits=next[1];
 const position=qualifyRustStandardSumSourcePosition(parser,createRustBorrowedVectorElementPosition(origin),origin.path);if(!position)throw unsupported('Rust borrowed expression source position','the actual source position needs its complete declaration directory',node.span);
 node.sourcePosition={model:'Rust borrowed expression source position 1',position};table.borrowedVectorNeedsFinalAdmission=true;return node;
}
