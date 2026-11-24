// Actual standard type contracts into existing literal/constructor nodes.
// No trait implementation is inferred for a user-defined nominal datatype.
import {unsupported} from './diagnostics.js';
import {cloneRustSourceTypeOrigin} from './rust-source-type-origins.js';
import {projectRustPrimitiveDefault} from './rust-primitive-default.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {readRustOwnedStandardSumDefaultTerm} from './rust-owned-standard-sum-method-rules.js';
export function createRustStandardDefaultValue(parser,origin,path,span){
 const sum=readRustStandardSumSourceOrigin(parser,origin),owner=origin.expandedSourceOwner??origin.sourceOwner;
 const actual=sum?.standardSumSource.owner??owner?.path.join('::')??'';
 const term=readRustOwnedStandardSumDefaultTerm(origin.type?.kind??'',owner?.kind??'',actual,origin.arraySourceModel??origin.textSourceModel??'',origin.wrappers.length,origin.proven===true);
 let value;
 if(term==='empty owned string')value={k:'str',value:'',span};
 else if(term==='unit')value={k:'unit',span};
 else if(term==='empty owned vector')value={k:'array',items:[],element:origin.type.element,span};
 else if(term==='empty actual Option')value={k:'name',path:sum.standardSumSource.constructorPaths.None,span};
 else if(['false','positive floating zero','integer zero'].includes(term)){
  value=projectRustPrimitiveDefault(parser,actual,path,span);
  const produced=parser.origins.origin(value,path);parser.origins.argument(produced,origin,span);
  if(readRustStandardSumSourceIdentity(parser,produced)!==readRustStandardSumSourceIdentity(parser,origin))throw unsupported('Rust standard Default','the actual primitive default must preserve its complete source identity',span);
  return value;
 }else throw unsupported('Rust standard Default','unwrap_or_default needs an actual supported standard Default implementation; user trait implementations retain their separate obligation',span);
 return parser.origins.mark(value,cloneRustSourceTypeOrigin(origin));
}
