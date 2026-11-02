// Copy a real shared-reference element; never manufacture Copy on its referent.
import {sameType} from './types.js';
import {unsupported} from './diagnostics.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {isRustBorrowedVectorSourceOrigin,readRustBorrowedVectorSignature,rustBorrowedVectorPositionOrigin,readRustBorrowedVectorArgumentRegions} from './rust-borrowed-vectors.js';
/** Source operand checks and exact usize conversion remain in rust-array-index.
 * Header deferral retains the existing complete-discovery contract; final source
 * admission still requires the actual closed referent graph.
 */
export function projectRustBorrowedVectorIndexOrigin(parser,actual,path,span){
 if(!actual.arrayElementPosition)return null;
 if(!isRustBorrowedVectorSourceOrigin(parser.origins,actual)||actual.proven!==true||!sameType(actual.type.element,actual.arrayElementPosition.type)||!readRustBorrowedVectorSignature(parser,actual))throw unsupported('Rust borrowed vector index','indexing requires its actual standard vector and admitted shared-reference element position',span);
 checkRustOwnedOrigin(parser.origins,actual,span);
 const loans=readRustBorrowedVectorArgumentRegions(actual);
 return {...rustBorrowedVectorPositionOrigin(actual.arrayElementPosition,path),sourceLoanBindingIdentities:[...loans],sourceValueBorrowBindings:[...loans],sourcePlaceBindingIdentity:null,sourcePlaceLoanBindingIdentities:[],sourceBindingIdentity:null,sourceLexicalBindingIdentity:null,copyEvidence:true};
}
/** Read one copied reference from a checked borrowed vector. The normal checked
 * index operation owns bounds/errors; there is no invented in-bounds proof.
 * The result keeps referent regions, not a borrow of the vector's allocation.
 */
export function readRustBorrowedVectorIndexedRegion(visitor,node,env,bindings,depth){
 const array=visitor.read(visitor.expression(node.array,env,bindings,null,depth+1),'read',node.array),position=array.position,element=position?.arrayElementPosition;
 if(array.role!=='borrowed vector'||position.wrappers.length!==0||position.owner.kind!=='library'||position.owner.path.join('::')!=='std::vec::Vec'||position.owner.arguments.length!==1||!element||element.wrappers.length!==1||element.wrappers[0]!=='reference'||element.lifetimes.length!==1||element.arrayElementPosition||!sameType(element.type,node.type))throw unsupported('Rust source binder regions','an indexed reference retains its actual complete borrowed vector element position',node.span);
 const index=visitor.read(visitor.expression(node.index,env,bindings,null,depth+1),'read',node.index);
 if(!['copy','inferred numeric'].includes(index.role))throw unsupported('Rust source binder regions','an index retains its checked scalar coordinate without borrowed authority',node.span);
 return visitor.value(visitor.position(element),null,[...array.regions],false);
}
