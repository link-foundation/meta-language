//! Host tree reconstruction from the generated shared proposition-value plan.
use serde_json::{Value,json};
use super::{Checker,Env,Expr,Result,Span,unsupported};
use crate::translation::surface::SProp;
use crate::translation::proposition_value_rules as rules;
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("proposition value",reason,span)}
fn literal(value:bool)->Value{json!({"k":"lit","value":value,"type":{"kind":"bool"}})}
fn conditional(operation:&str,left:Value,right:Value)->Value{let branch=|truth|{let mode=rules::read_proposition_value_conditional_branch(operation,truth);if mode=="right"{right.clone()}else{literal(mode=="true")}};json!({"k":"if","cond":left,"then":branch(true),"else":branch(false),"type":{"kind":"bool"}})}
fn lower(prop:&Value,expanded:&mut usize,depth:usize,span:Option<Span>)->Result<Value>{
 *expanded+=1;if *expanded>rules::read_proposition_value_expansion_limit() as usize||depth>rules::read_proposition_value_expansion_limit() as usize{return Err(fail("the exhaustive proposition expansion exceeds its finite source budget",span));}
 let kind=prop["p"].as_str().unwrap_or("");let form=rules::read_proposition_value_form(kind);
 if form=="value"{return Ok(prop["expr"].clone());}
 if form=="comparison"{let mut value=prop.clone();let fields=value.as_object_mut().ok_or_else(||fail("an actual checked comparison is required",span))?;fields.remove("p");fields.insert("k".into(),json!("propositionValue"));fields.insert("op".into(),json!(kind));fields.insert("type".into(),json!({"kind":"bool"}));if let Some(span)=span{fields.insert("span".into(),json!(span));}return Ok(value);}
 if form=="negation"{return Ok(json!({"k":"unary","op":"not","arg":lower(&prop["arg"],expanded,depth+1,span)?,"type":{"kind":"bool"}}));}
 if form=="and"||form=="or"||form=="implies"{let left=lower(&prop["left"],expanded,depth+1,span)?;let right=lower(&prop["right"],expanded,depth+1,span)?;return Ok(conditional(&form,left,right));}
 if form=="finite quantification"{return bind(prop,0,expanded,depth,span);}
 Err(fail("an actual checked proposition form is required",span))
}
fn bind(prop:&Value,index:usize,expanded:&mut usize,depth:usize,span:Option<Span>)->Result<Value>{
 *expanded+=1;if *expanded>rules::read_proposition_value_expansion_limit() as usize{return Err(fail("the exhaustive proposition expansion exceeds its finite source budget",span));}
 let binders=prop["binders"].as_array().ok_or_else(||fail("actual complete quantifier binders are required",span))?;if index==binders.len(){return lower(&prop["body"],expanded,depth+1,span);}
 let binder=&binders[index];let kind=binder["type"]["kind"].as_str().unwrap_or("");let error=rules::read_proposition_value_domain_error(kind);if !error.is_empty(){return Err(fail(&error,span));}let mut branches=Vec::new();for value in rules::plan_proposition_value_quantifier(kind){let value=if kind=="bool"{json!({"k":"lit","value":value==1.0,"type":binder["type"]})}else{json!({"k":"unit","type":binder["type"]})};branches.push(json!({"k":"let","name":binder["name"],"value":value,"body":bind(prop,index+1,expanded,depth,span)?,"type":{"kind":"bool"}}));}
 if branches.len()==1{Ok(branches.remove(0))}else{Ok(conditional(&rules::read_proposition_value_quantifier_join(),branches[0].clone(),branches[1].clone()))}
}
impl Checker{
 pub(super) fn proposition_value(&mut self,prop:&SProp,environment:&Env,path:&[String],span:Option<Span>)->Result<Expr>{let checked=self.prop(prop,environment,path)?;let value=serde_json::to_value(checked).map_err(|_|fail("checked proposition encoding failed",span))?;serde_json::from_value(lower(&value,&mut 0,0,span)?).map_err(|_|fail("checked proposition reconstruction failed",span))}
}
