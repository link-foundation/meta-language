// Copy a real shared-reference element; never manufacture Copy on its referent.
import {acceptRustScalarArrayIndexElement} from './rust-array-index-rules.js';
import {readRustBorrowedSourceValueRole} from './rust-borrowed-source-region-rules.js';
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
/** Read a copied reference or direct scalar from an actual standard vector.
 * Existing checked indexing owns bounds/errors. Reference results retain their
 * referent regions; scalar results carry no borrow of the vector allocation.
 */
export function readRustSourceVectorIndexedRegion(visitor,node,env,bindings,depth){
 const array=visitor.read(visitor.expression(node.array,env,bindings,null,depth+1),'read',node.array),position=array.position;
 if(!position||position.type.kind!=='array'||!(position.wrappers.length===0||position.wrappers.length===1&&position.wrappers[0]==='reference')||position.owner.kind!=='library'||position.owner.path.join('::')!=='std::vec::Vec'||position.owner.arguments.length!==1||!sameType(position.type.element,node.type))throw unsupported('Rust source binder regions','an indexed value retains its actual standard vector and matching element type',node.span);
 const element=position.arrayElementPosition??{type:position.type.element,wrappers:[],lifetimes:[],owner:position.owner.arguments[0]};
 const scalar=acceptRustScalarArrayIndexElement(element.type.kind),borrowing=node.sourcePosition?.position?.wrappers;
 if(array.role==='borrowed vector'){
  if(element.wrappers.length!==1||element.wrappers[0]!=='reference'||element.lifetimes.length!==1||element.arrayElementPosition||!sameType(element.type,node.type))throw unsupported('Rust source binder regions','an indexed reference retains its actual complete borrowed vector element position',node.span);
 }else if(!['owned','reference'].includes(array.role)||position.arrayElementPosition||!scalar||readRustBorrowedSourceValueRole(element.type.kind,String(element.type.bits??''),element.type.signed===true,element.owner.kind,element.owner.path.join('::'),element.owner.arguments.length,element.wrappers,Boolean(element.arrayElementPosition))!=='copy')throw unsupported('Rust source binder regions','owned vector indexing copies only its actual direct scalar element owner',node.span);
 const index=visitor.read(visitor.expression(node.index,env,bindings,null,depth+1),'read',node.index);
 if(!['copy','inferred numeric'].includes(index.role))throw unsupported('Rust source binder regions','an index retains its checked scalar coordinate without borrowed authority',node.span);
 if(array.role==='borrowed vector')return {...visitor.value(visitor.position(element),null,[...array.regions],false),borrowRoots:[...(array.borrowRoots??array.regions)]};
 // Source-position validation owns the complete result qualifier. Creating a
 // shared reference borrows the actual vector allocation; copying the scalar
 // itself carries no loan of that allocation.
 if(borrowing?.length){
  if(borrowing.length!==1||borrowing[0]!=='reference')throw unsupported('Rust source binder regions','an indexed scalar borrow needs its actual single shared-reference operation',node.span);
  const places=array.role==='reference'?(array.borrowRoots??array.regions):(array.placeRoots??[array.storageRoot??array.root??visitor.root('',true)]),roots=visitor.union([array.regions,array.placeRegions??[],places]);
  return {...visitor.value(visitor.position({...element,wrappers:['reference'],lifetimes:[null]}),null,roots,false),borrowRoots:visitor.union([array.borrowRoots??[],array.inheritedBorrowRoots??[],places])};
 }
 return visitor.value(visitor.position(element),null,[],false);
}
