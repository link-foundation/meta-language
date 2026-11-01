//! Completion-aware arms retain actual source patterns and owner snapshots.
//! Core matching still establishes constructor coverage and exhaustiveness.
use super::*;
impl RustParser {
 pub(super) fn has_ordinary_match_completion(&self)->Result<bool>{
  if self.cursor.peek().raw!="match"{return Ok(false);}let mut size=64;
  loop{let tokens=self.cursor.tokens.iter().skip(self.cursor.index).take(size).collect::<Vec<_>>();let kinds=tokens.iter().map(|token|crate::translation::source_diagnostic_attributes::token_kind(token).to_owned()).collect::<Vec<_>>();let values=tokens.iter().map(|token|if token.kind==TokenKind::Identifier{token.raw.clone()}else{token.value.clone()}).collect::<Vec<_>>();let selected=rules::read_rust_ordinary_match_completion_presence(&kinds,&values);if selected != -1.0{return Ok(selected==1.0);}if tokens.len()<size{return Ok(false);}if size>=100000{return Err(unsupported("ordinary Rust match completion","the complete source match exceeds its inspection budget",Some(self.peek().span())));}size=(size*2).min(100000);}
 }
 pub(super) fn ordinary_match_completion(&mut self,path:&[String])->Result<CompletionTree>{
  let start=self.cursor.advance();let scrutinee=self.with_standard_sum_source_context(None,|parser|parser.expr(path,Options{statement:false,no_struct:true}))?;let before=self.ordinary_completion_snapshot()?;self.cursor.expect("{",Some("match"))?;
  let mut arms=Vec::new();let mut joined=HashMap::new();let mut continuing=false;let mut kind=None;
  while !self.cursor.is("}"){
   let error=rules::read_rust_ordinary_match_arm_count_error((arms.len()+1) as f64);if !error.is_empty(){return Err(unsupported("ordinary Rust match completion",&error,span(&start,self.cursor.peek())));}
   let restored=self.clone_ordinary_completion_snapshot(&before)?;self.origins.restore_owned_bindings(restored);let arm_start=self.peek();let mut aliases=Vec::new();self.cursor.eat("|");let expected=self.origins.origin(&scrutinee,path);
   let mut patterns=self.with_standard_sum_source_context(Some(expected),|parser|{let mut patterns=vec![parser.pattern(path,&mut aliases)?];while parser.cursor.eat("|").is_some(){patterns.push(parser.pattern(path,&mut aliases)?);}Ok(patterns)})?;
   if self.cursor.is("if"){return Err(unsupported("ordinary Rust match completion","a guarded completed arm needs its selected guard fallthrough continuation",span(&arm_start,self.cursor.peek())));}
   if patterns.len()>1&&!aliases.is_empty(){return Err(unsupported("binding in an or-pattern","bind the value in each alternative separately",span(&arm_start,self.cursor.peek())));}
   let scope=self.origins.cases(&mut patterns,&scrutinee,path)?;let previous=self.origins.enter(Some(scope));
   let read=(||->Result<_>{
    for alias in &aliases{self.origins.bind(&alias.name,&alias.value,path,None)?;}self.cursor.expect("=>",Some("match arm"))?;let block=self.cursor.is("{")||self.cursor.is("if")||self.cursor.is("match");
    let mut body=if self.cursor.is("{"){self.ordinary_completion_block(path)?}else if self.cursor.is("if"){self.ordinary_conditional_completion(path)?}else if self.has_ordinary_match_completion()?{self.ordinary_match_completion(path)?}
    else if self.cursor.peek().raw=="return"{let token=self.cursor.advance();let value=if self.cursor.is(",")||self.cursor.is("}"){SExpr::new(SNode::Unit,span(&token,&token))}else{self.expr(path,Options::default())?};self.completion_leaf("return",value,path)?}
    else if self.cursor.peek().raw=="break"||self.cursor.peek().raw=="continue"{let token=self.cursor.advance();if self.owned_record_state.effects.as_ref().and_then(|effects|effects.loop_scope)!=Some(self.owned_record_state_callable()){return Err(unsupported("Rust loop exit","a loop exit needs its actual enclosing callable and loop scope",span(&token,&token)));}if !self.cursor.is(",")&&!self.cursor.is("}"){return Err(unsupported("Rust loop exit","labelled exits and break values need their explicit value and target interpretation",span(&token,self.cursor.peek())));}self.completion_leaf(&token.raw,SExpr::new(SNode::Unit,span(&token,&token)),path)?}
    else{let value=self.expr(path,Options::default())?;let kind=super::super::expressions::source_completion_kind(&value);self.completion_leaf(if kind=="abort"{"abort"}else{"value"},value,path)?};
    for alias in aliases.into_iter().rev(){let kind=body.kind.clone();body=CompletionTree{kind,ownership:None,node:CompletionNode::Binding(Binding{name:alias.name,ty:None,value:alias.value,span:span(&arm_start,self.cursor.peek())},Box::new(body))};}
    let current=self.ordinary_completion_snapshot()?;let values=self.expire_ordinary_completion_paths(&mut body,&before)?;self.origins.restore_owned_bindings(current);if values.is_empty(){let restored=self.clone_ordinary_completion_snapshot(&before)?;self.origins.restore_owned_bindings(restored);}else{let origin=self.origins.checked_join(&values)?;self.origins.expire_owned_bindings(&before,&origin)?;}
    let falls=rules::test_rust_ordinary_completion_falls_through(&body.kind);let states=self.ordinary_completion_snapshot()?;self.origins.join_owned_bindings(&joined,&states,continuing,falls);joined=self.ordinary_completion_snapshot()?;continuing|=falls;kind=Some(kind.take().map_or_else(||body.kind.clone(),|first:String|rules::read_rust_ordinary_completion_alternative_kind(&first,&body.kind)));
    Ok((body,block))
   })();self.origins.scope=previous;let(body,block)=read?;arms.push(CompletionArm{patterns,body:Box::new(body),span:span(&arm_start,self.cursor.peek())});if block{self.cursor.eat(",");}else if !self.cursor.is("}"){self.cursor.expect(",",Some("match arm"))?;}
  }
  self.cursor.expect("}",Some("match"))?;if arms.is_empty(){return Err(unsupported("empty match","uninhabited types are outside the portable core",span(&start,self.cursor.peek())));}self.origins.restore_owned_bindings(joined);Ok(CompletionTree{kind:kind.expect("actual nonempty match arms"),ownership:None,node:CompletionNode::Match{scrutinee,arms,span:span(&start,self.cursor.peek())}})
 }
}
