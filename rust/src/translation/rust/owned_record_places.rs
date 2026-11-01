//! Non-consuming nested place syntax, authenticated field by actual field.
use super::{Result,RustParser,SExpr,SNode,Span,TokenKind,unsupported};
use crate::translation::rust_owned_record_place_rules as rules;
impl RustParser{
 pub(super) fn read_owned_record_place_base(&mut self,path:&[String])->Result<Option<SExpr>>{
  let limit=rules::read_rust_owned_record_place_depth_limit() as usize;
  let tokens=self.cursor.tokens.iter().skip(self.cursor.index).take(limit*4+1).collect::<Vec<_>>();
  let forms=tokens.iter().map(|token|if ["(",")",".","}"].contains(&token.value.as_str()){token.value.clone()}else if token.kind==TokenKind::Identifier{"name".into()}else if token.kind==TokenKind::Number&&(token.raw=="0"||!token.raw.starts_with('0')&&token.raw.bytes().all(|value|value.is_ascii_digit())){"number".into()}else{"other".into()}).collect::<Vec<String>>();
  if rules::read_rust_owned_record_place_token_count(&forms)<0.0{return Ok(None);}
  if !tokens.iter().find(|token|token.value!="(").is_some_and(|token|self.origins.scope.contains_key(&token.value)){return Ok(None);}
  self.owned_record_place(path,0,limit).map(Some)
 }
 fn owned_record_place(&mut self,path:&[String],depth:usize,limit:usize)->Result<SExpr>{
  if depth>=limit{return Err(unsupported("Rust record place","the complete nested record place exceeds its source depth budget",Some(self.cursor.peek().span())));}
  let mut value=if self.cursor.eat("(").is_some(){let value=self.owned_record_place(path,depth+1,limit)?;self.cursor.expect(")",Some("record place"))?;value}else{
   let token=self.cursor.identifier(Some("record place"))?;let Some(origin)=self.origins.scope.get(&token.value).cloned()else{return Err(unsupported("Rust record place","a place root needs its actual lexical source binding",Some(token.span())));};self.origins.check_owned_record_root(&origin,Some(token.span()))?;self.owned_record_mutable_name(SExpr::new(SNode::Name{path:vec![token.value.clone()]},Some(token.span())),path)?
  };
  while self.cursor.eat(".").is_some(){let field=self.cursor.advance();let name=if field.kind==TokenKind::Number{field.raw.clone()}else{field.value.clone()};let span=Some(Span::new(value.span.map_or(field.start,|span|span.start),field.end));value=self.record_projection(&value,&name,path,span)?.ok_or_else(||unsupported("Rust record place","every selected place field needs its actual complete nominal declaration",span))?;}
  Ok(value)
 }
}
