//! Fully checked declaration/operand transport for shared snapshot boundaries.
use super::{diagnostics::{Result,unsupported},ir::{Decl,Effect,Expr,Node,Program,prop_exprs}};
use std::collections::BTreeSet;
pub(crate) fn check_snapshot_callbacks(program:&Program)->Result<()>{
 let mut pending:Vec<&Expr>=Vec::new();
 for declaration in &program.declarations{match declaration{Decl::Fn(function)=>pending.push(&function.body),Decl::Theorem(theorem)=>pending.extend(prop_exprs(&theorem.prop)),Decl::Data(_)=>{}}}
 if let Some(main)=&program.main{for effect in &main.effects{match effect{Effect::Print{expr,..}|Effect::Output{expr,..}=>pending.push(expr),Effect::Let{value,..}=>pending.push(value),Effect::Unwrap{pair,..}=>pending.push(pair),Effect::Assert{prop,..}=>pending.extend(prop_exprs(prop))}}}
 let mut seen=BTreeSet::new();let mut disjoint=None;
 while let Some(node)=pending.pop(){
  if !seen.insert(std::ptr::from_ref(node)){continue;}
  if let Node::Method{op,object,args}=&node.node&&op=="map"&&let Node::CollectionItems{object:source,op:snapshot_op}=&object.node&&super::collection_copy_rules::accept_collection_copy_mapper_snapshot(source.ty.kind(),snapshot_op){
   let callback=args.first().and_then(Option::as_ref);let readonly=callback.is_some_and(|callback|super::reference_callbacks::read_only_reference_callback(callback,&program.declarations));
   let safe=readonly||disjoint.get_or_insert_with(||super::disjoint_collection_iterations::read_program_proof(program,&[])).collection_mappers;
   let error=super::collection_copy_rules::read_collection_copy_mapper_error(true,safe);
   if !error.is_empty(){return Err(unsupported("collection snapshot callback",&error,callback.and_then(|callback|callback.span).or(node.span)));}
  }
  pending.extend(node.children());
 }Ok(())
}
