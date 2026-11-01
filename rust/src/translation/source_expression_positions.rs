//! Source expression shape transport. Actual body binder/region proofs are a
//! separate whole-program operation; this does not infer enclosing lifetimes.
use super::diagnostics::{Result,type_error};
use super::ir::DataDecl;
use super::types::Type;
use super::source_signatures::{RustSourceExpressionPosition,validate_source_position_with_bindings};
use super::rust_borrowed_vector_rules as rules;
pub(crate) fn check_rust_source_expression_position<'data>(facet:&RustSourceExpressionPosition,actual_type:&Type,language:&str,resolve_type:impl FnMut(&Type)->Result<Type>,resolve_data:impl FnMut(&Type)->Option<&'data DataDecl>)->Result<RustSourceExpressionPosition>{
 let error=rules::read_rust_borrowed_expression_position_model_error(&facet.model,language);if !error.is_empty(){return Err(type_error(&error,None));}
 super::rust::measure_source_position_metadata(&facet.position).map_err(|message|type_error(&message,None))?;
 let mut bindings=Vec::new();let mut names=std::collections::HashSet::new();let mut current=Some(&facet.position);let mut visits=0;
 while let Some(position)=current{let error=rules::read_rust_borrowed_vector_position_budget_error((visits+1) as f64,visits as f64);if !error.is_empty(){return Err(type_error(&error,None));}for lifetime in position.lifetimes.iter().flatten(){if lifetime!="static"&&lifetime!="_"&&names.insert(lifetime.as_str()){bindings.push(lifetime.clone());}}visits+=1;current=position.array_element_position.as_deref();}
 let position=validate_source_position_with_bindings(&facet.position,actual_type,language,bindings,resolve_type,resolve_data)?;Ok(RustSourceExpressionPosition{model:facet.model.clone(),position})
}
