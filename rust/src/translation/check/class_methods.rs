//! Actual checked receiver/signature transport for source-owned prototypes.
use super::{Checker,Decl,Env,Expr,Language,Node,Result,SExpr,SNode,Span,Type,unsupported};
use crate::translation::javascript_class_source_rules::{self as rules,ml_string::Text};
impl Checker{
 pub(super) fn validate_closed_class_method_definitions(&self)->Result<()>{
  let methods=&self.source_class_methods;
  if methods.len()%3!=0||methods.len()>12288||methods.iter().any(String::is_empty){return Err(unsupported("closed class method table","complete bounded source-owned brand, method and helper triples are required",None));}
  if !methods.is_empty()&&(self.language!=Language::JavaScript||self.checked_projection){return Err(unsupported("closed class method table","only actual JavaScript source may supply a prototype dispatch table",None));}
  let mut owners=std::collections::HashSet::new();
  for row in methods.chunks_exact(3){
   let brand=&row[0];let helper=&row[2];
   let prefix=brand.rfind("class_identity_").map_or("",|at|&brand[..at]);
   let constructor=self.decls.values().any(|entry|matches!(entry,Decl::Fn(function)if !function.name.starts_with(&format!("{prefix}class_method_"))&&matches!(&function.ret,Type::Record{fields,..}if fields.iter().any(|field|&field.name==brand))));
   let Some(Decl::Fn(function))=self.decls.get(helper)else{return Err(unsupported("closed class method table","every actual brand needs its constructor and declared receiver helper",None));};
   let parameter=function.params.first();let fields=parameter.map_or_else(Vec::new,|parameter|match &parameter.ty{Type::Record{fields,..}=>fields.iter().map(|field|Text::from_text(&field.name)).collect(),_=>Vec::new()});
   let valid=rules::accept_java_script_closed_class_method_definition(&Text::from_text(brand),&Text::from_text(&row[1]),&Text::from_text(helper),&Text::from_text(parameter.map_or("",|parameter|parameter.name.as_str())),&fields,constructor);
   if !owners.insert((brand.clone(),row[1].clone()))||!function.module_path.is_empty()||!valid{return Err(unsupported("closed class method table","every actual brand needs its constructor and declared receiver helper",None));}
  }Ok(())
 }
 pub(super) fn closed_class_method_call(&mut self,receiver:&Expr,operation:&str,args:&[Option<SExpr>],env:&Env,path:&[String],span:Option<Span>)->Result<Option<Expr>>{
  let fields=match &receiver.ty{Type::Record{fields,..}=>fields.iter().map(|field|Text::from_text(&field.name)).collect::<Vec<_>>(),_=>return Ok(None)};
  let target=rules::read_java_script_closed_class_method_target(&self.source_class_methods.iter().map(|value|Text::from_text(value)).collect::<Vec<_>>(),&fields,&Text::from_text(operation)).to_string();
  if target.is_empty(){return Ok(None);}
  let name=self.fresh_name("ml_class_receiver");let mut inner=env.clone();inner.bind_local(&name,receiver.ty.clone());
  let mut arguments=vec![SExpr::new(SNode::Name{path:vec![name.clone()]},span)];
  for argument in args{arguments.push(argument.clone().ok_or_else(||unsupported("closed class method call","actual ordered source arguments are required",span))?);}
  let body=self.application(&SExpr::new(SNode::Name{path:vec![target]},span),&arguments,&inner,path,None,span)?;
  let ty=body.ty.clone();let mut value=Expr::new(Node::Let{name,value:Box::new(receiver.clone()),body:Box::new(body)},ty);value.span=span;Ok(Some(value))
 }
}
