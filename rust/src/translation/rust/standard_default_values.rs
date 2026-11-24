//! Actual standard type contracts projected into existing surface constructors.
use super::{Result,RustParser,SExpr,SNode,SourceOrigin,Span,Type,unsupported};
use crate::translation::rust_owned_standard_sum_method_rules as rules;
impl RustParser{
 pub(super) fn standard_default_value(&mut self,origin:&SourceOrigin,path:&[String],span:Option<Span>)->Result<SExpr>{
  let sum=self.origins.standard_sum_source_origin(origin);let owner=origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());
  let actual=sum.as_ref().and_then(|value|value.standard_sum_source.as_deref()).map(|value|value.owner.clone()).unwrap_or_else(||owner.map(|value|value.path.join("::")).unwrap_or_default());
  let term=rules::read_rust_owned_standard_sum_default_term(origin.ty.as_ref().map_or("",Type::kind),owner.map_or("",|value|value.kind.as_str()),&actual,origin.array_source_model.or(origin.text_source_model).unwrap_or(""),origin.wrappers.len() as f64,origin.proven);
  let node=match term.as_str(){
   "empty owned string"=>SNode::Str{value:String::new()},
   "unit"=>SNode::Unit,
   "empty owned vector"=>match origin.ty.as_ref(){Some(Type::Array{element,..})=>SNode::Array{items:Vec::new(),element:Some(element.as_ref().clone().into())},_=>return Err(unsupported("Rust standard Default","the actual vector default needs its complete element type",span))},
   "empty actual Option"=>{let path=sum.as_ref().and_then(|value|value.standard_sum_source.as_deref()).and_then(|value|value.constructor_paths.get("None")).cloned().ok_or_else(||unsupported("Rust standard Default","the actual Option default needs its resolved empty constructor",span))?;SNode::Name{path}},
   "false"|"positive floating zero"|"integer zero"=>{
    let value=self.project_primitive_default(&actual,path,span)?;let produced=self.origins.origin(&value,path);self.origins.argument(&produced,origin,span)?;
    if self.origins.standard_sum_source_identity(&produced)!=self.origins.standard_sum_source_identity(origin){return Err(unsupported("Rust standard Default","the actual primitive default must preserve its complete source identity",span));}
    return Ok(value);
   },
   _=>return Err(unsupported("Rust standard Default","unwrap_or_default needs an actual supported standard Default implementation; user trait implementations retain their separate obligation",span)),
  };
  let value=SExpr::new(node,span);self.origins.mark(&value,super::source_type_origins::clone_source_type_origin(origin));Ok(value)
 }
}
