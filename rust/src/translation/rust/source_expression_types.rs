//! Retain only actual complete source types while lexical scopes are live.
//! No value ownership, Copy, loan, binding or producer fact is cached here.
use std::collections::HashMap;
use super::{Result,SExpr,SNode,SourceDeclaration,SourceOrigin,SourceOrigins,Span,Type,unsupported};
use super::standard_sum_metadata::{MetadataBudget,measure_expression_type};
use crate::translation::rust_primitive_expression_rules as primitive;
use crate::translation::rust_pointer_integer_owner_rules as pointer;
// Borrow actual primitive contracts. This projection deliberately has no
// binding, Copy, producer, loan or child-origin storage to clone or upgrade.
struct ScalarRead<'a>{ty:Option<&'a Type>,path:&'a [String],owner:String,pointer:String,known:bool,signed:bool}
fn scalar(origin:&SourceOrigin)->Option<ScalarRead<'_>>{
 if !origin.proven||origin.integer_source_owner_conflict||origin.wrappers.iter().any(|layer|layer=="box"){return None;}
 let owner=origin.source_owner.as_ref();let floating=origin.ty==Some(Type::Float)&&origin.float_source_model==Some("binary64 scalar");
 let name=owner.filter(|owner|owner.path.len()==1).map(|owner|owner.path[0].as_str()).unwrap_or(if floating{"f64"}else{""});
 let(bits,signed)=match origin.ty.as_ref(){Some(Type::Fixed{bits,signed})=>(bits.to_string(),*signed),_=>(String::new(),false)};
 let known=primitive::accept_rust_primitive_arithmetic_origin(origin.ty.as_ref().map_or("",Type::kind),&bits,signed,owner.map_or(if floating{"primitive"}else{""},|owner|owner.kind.as_str()),name,owner.map_or(if floating{0.0}else{-1.0},|owner|owner.arguments.len() as f64),true,origin.wrappers.len() as f64);
 Some(ScalarRead{ty:origin.ty.as_ref(),path:&origin.path,owner:name.to_owned(),pointer:origin.pointer_integer_owner(),known,signed})
}
fn literal(node:&SExpr)->&'static str{
 let(number,negative)=match &node.node{SNode::Num{..}=>(node,false),SNode::Unary{op:crate::translation::surface::UnaryOp::Neg,arg}if matches!(arg.node,SNode::Num{..})=>(arg.as_ref(),true),_=>return ""};
 match &number.node{SNode::Num{ty:None,..}=>if negative{"negative integer numeral"}else{"integer numeral"},SNode::Num{ty:Some(Type::Float),..}=>if negative{"negative floating numeral"}else{"floating numeral"},_=>""}
}
fn primitive_actual<'a>(origins:&'a SourceOrigins,node:&'a SExpr,path:&'a [String],depth:usize)->Option<ScalarRead<'a>>{
 if depth>256{return None;}if let Some(origin)=origins.recorded_expression_origin(node){return scalar(origin);}
 if let SNode::Name{path:names}=&node.node{if names.len()==1{return origins.scope.get(&names[0]).and_then(scalar);}}
 if let SNode::Num{ty,..}=&node.node{return Some(ScalarRead{ty:ty.as_ref(),path,owner:String::new(),pointer:String::new(),known:false,signed:false});}
 let(operation,left,right)=match &node.node{SNode::Binary{op,left,right}=>(op.name(),left.as_ref(),Some(right.as_ref())),SNode::Unary{op:crate::translation::surface::UnaryOp::Neg,arg}=>("neg",arg.as_ref(),None),_=>return None};
 if super::indirection_rules::read_rust_source_expression_origin(node.node.kind(),operation)!="primitive-operation"{return None;}
 let first=primitive_actual(origins,left,path,depth+1)?;let second=match right{Some(right)=>Some(primitive_actual(origins,right,path,depth+1)?),None=>None};
 let owners=std::iter::once(first.pointer.clone()).chain(second.as_ref().map(|value|value.pointer.clone())).collect::<Vec<_>>();if !pointer::read_rust_pointer_integer_owner_join_error(&owners).is_empty(){return None;}
 let selected=primitive::select_rust_primitive_arithmetic_origin(operation,first.known,second.as_ref().is_some_and(|value|value.known),&first.owner,second.as_ref().map_or("",|value|value.owner.as_str()),first.signed,second.as_ref().is_some_and(|value|value.signed),literal(left),right.map_or("",literal));
 if selected==0.0{return Some(first);}if selected==1.0{return second;}
 let owner=pointer::select_rust_pointer_integer_result_owner(&owners);if owner.is_empty(){return None;}if first.pointer==owner{Some(first)}else{second.filter(|value|value.pointer==owner)}
}
pub(super) struct RecordedExpressionType{pub(super) ty:Type,pub(super) path:Vec<String>,pub(super) proven:bool}
#[derive(Default)]
pub(super) struct ExpressionTypeCache{types:HashMap<Span,RecordedExpressionType>,budget:MetadataBudget}
fn result(origin:&SourceOrigin)->Option<(&Type,&[String])>{if !origin.proven||origin.source_function_pointer_header||origin.standard_sum_source.as_deref().is_some_and(|sum|sum.provisional){return None;}origin.ty.as_ref().map(|ty|(ty,origin.path.as_slice()))}
fn callable(origin:&SourceOrigin)->Option<(&Type,&[String])>{if !origin.proven||origin.source_function_pointer_header{return None;}if let Some(value)=origin.callable_result.as_deref(){return result(value);}match origin.ty.as_ref()?{Type::Function{ret,..}=>Some((ret,&origin.path)),_=>None}}
fn actual<'a>(origins:&'a SourceOrigins,node:&'a SExpr,path:&'a [String])->Option<(&'a Type,&'a [String])>{
 if matches!(node.node,SNode::Binary{..}|SNode::Unary{..}){let actual=primitive_actual(origins,node,path,0)?;return actual.ty.map(|ty|(ty,actual.path));}
 let SNode::App{func,..}=&node.node else{return None;};
 if let SNode::Name{path:names}=&func.node{
  if names.len()==1{if let Some(origin)=origins.scope.get(&names[0]){return callable(origin);}}
  if let Some((_,entry))=origins.lookup_value_owner_ref(names,path){return match entry{SourceDeclaration::Function{result:origin,..}|SourceDeclaration::Constructor{result:origin,..}=>result(origin),_=>None};}
 }
 if let Some(origin)=origins.recorded_expression_origin(func){return callable(origin);}
 let facts=origins.recorded_expression_type(func)?;if !facts.proven{return None;}match &facts.ty{Type::Function{ret,..}=>Some((ret,&facts.path)),_=>None}
}
impl SourceOrigins{
 pub(super) fn record_expression_type(&mut self,node:&SExpr,path:&[String])->Result<()>{
  let Some(span)=node.span else{return Ok(());};if self.recorded_expression_origin(node).is_some(){self.expression_type_cache.types.remove(&span);return Ok(());}if self.collecting{return Ok(());}if self.expression_type_cache.types.contains_key(&span){return Ok(());}
  let Some((ty,scope))=actual(self,node,path)else{return Ok(());};let cost=measure_expression_type(ty,scope).map_err(|error|unsupported("Rust source expression type",&error,node.span))?;self.expression_type_cache.budget.reserve(cost).map_err(|error|unsupported("Rust source expression type",&error,node.span))?;
  let Some((ty,scope))=actual(self,node,path)else{return Ok(());};let facts=RecordedExpressionType{ty:ty.clone(),path:scope.to_vec(),proven:true};self.expression_type_cache.types.insert(span,facts);Ok(())
 }
 pub(super) fn recorded_expression_type(&self,node:&SExpr)->Option<&RecordedExpressionType>{node.span.and_then(|span|self.expression_type_cache.types.get(&span))}
 pub(super) fn recorded_expression_type_at_span(&self,span:Option<Span>)->Option<&RecordedExpressionType>{span.and_then(|span|self.expression_type_cache.types.get(&span))}
}
