//! Exact finite record-loop source transport. Admission tokens are created
//! only after actual counter syntax, scope, progress and exit-state checks.
use super::{Span,Type,Result,RustParser,SExpr,SNode,SPatternNode,SourceOrigin,SourceDeclaration,Options,TokenKind,unsupported};
use super::owned_record_state::Slot;
use super::owned_record_effects::EffectSource;
use crate::translation::surface::SArrayItem;
use crate::translation::rust_owned_record_loop_rules as rules;
use crate::translation::rust_owned_record_update_rules as fields;
use crate::translation::rust_owned_record_effect_rules as effects;
use std::collections::HashSet;
use std::sync::Arc;
use serde_json::Value;
/// Opaque inside the source reader; source JSON cannot manufacture admission.
pub(crate) struct AdmittedFiniteRecordLoop{span:Option<Span>,counter_type:Type,pub(super) slots:Vec<Slot>,pub(super) path:Vec<String>}
impl AdmittedFiniteRecordLoop{
 pub(crate) fn source_span(&self)->Option<Span>{self.span}
 pub(crate) fn counter_type(&self)->&Type{&self.counter_type}
}
pub(super) struct LoopEntry{pub(super) admission:Option<Arc<AdmittedFiniteRecordLoop>>}
#[derive(Default)]
pub(super) struct FiniteRecordLoopState{pub(super) entries:Vec<LoopEntry>,pub(super) nodes:usize}
#[derive(serde::Serialize)]
struct LoopProjection<'a>{guard:&'a SExpr,body:&'a SExpr,types:Vec<&'a Option<Type>>}
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust finite record loop",reason,span)}
struct Place<'a>{slot:&'a Slot,indices:Vec<usize>,origin:&'a SourceOrigin}
struct Counter{lexical:String,indices:Vec<usize>,target:String,ty:Type}
fn numeral(node:&SExpr,value:&str)->bool{matches!(&node.node,SNode::Num{value:actual,ty}if actual==value&&ty.as_ref().is_none_or(|ty|matches!(ty,Type::Fixed{bits:8,signed:false})))}
fn place<'a>(parser:&'a RustParser,node:&SExpr)->Option<Place<'a>>{
 if let Some(EffectSource::Read(slot))=parser.owned_record_effect_source(node){return Some(Place{slot,indices:Vec::new(),origin:&slot.origin});}
 let SNode::Match{scrutinees,rows}=&node.node else{return None;};if scrutinees.len()!=1||rows.len()!=1||rows[0].patterns.len()!=1{return None;}
 let SPatternNode::Ctor{path,args}=&rows[0].patterns[0].node else{return None;};let SNode::Name{path:names}=&rows[0].body.node else{return None;};if names.len()!=1{return None;}
 let mut parent=place(parser,&scrutinees[0])?;let Type::Named{path:ty,..}=parent.origin.ty.as_ref()? else{return None;};let owner=parser.origins.lookup_type_owner(ty,&parent.origin.path)?;let record=parser.named_records.get(&owner.full_name)?;
 if path.join(".")!=format!("crate.{}",record.constructor){return None;}let mut constructor=vec!["crate".to_owned()];constructor.extend(record.constructor.split('.').map(str::to_owned));let(_,actual)=parser.origins.lookup_value_owner_ref(&constructor,&[])?;let SourceDeclaration::Constructor{owner:actual,fields,..}=actual else{return None;};if actual!=&owner.full_name||args.len()!=fields.len(){return None;}
 let binds=args.iter().map(|arg|if let SPatternNode::Bind{name}=&arg.node{Some(name.as_str())}else{None}).collect::<Option<Vec<_>>>()?;if binds.iter().copied().collect::<HashSet<_>>().len()!=binds.len(){return None;}let index=binds.iter().position(|name|*name==names[0].as_str())?;parent.indices.push(index);Some(Place{slot:parent.slot,indices:parent.indices,origin:&fields[index]})
}
fn counter(parser:&RustParser,guard:&SExpr)->Result<Counter>{
 let SNode::Binary{op,left,right}=&guard.node else{return Err(fail("the loop head needs its actual pure counter comparison with zero",guard.span));};if !numeral(right,"0"){return Err(fail("the loop head needs its actual pure counter comparison with zero",guard.span));}let selected=place(parser,left).filter(|place|!place.indices.is_empty()).ok_or_else(||fail("the loop head needs an actual mutable record field place",guard.span))?;
 let origin=selected.origin;let owner=origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());let(bits,signed)=match origin.ty.as_ref(){Some(Type::Fixed{bits,signed})=>(*bits as f64,*signed),_=>(0.0,false)};let owner=owner.filter(|owner|owner.kind=="primitive"&&owner.arguments.is_empty()).map(|owner|owner.path.join("::")).unwrap_or_default();let error=rules::read_rust_owned_record_loop_counter_error(origin.ty.as_ref().map_or("",Type::kind),bits,signed,origin.wrappers.len() as f64,&owner,op.name());if !origin.proven||!error.is_empty(){return Err(fail(if error.is_empty(){"the loop counter needs its actual complete source proof"}else{&error},guard.span));}
 let indices=selected.indices;let target=crate::translation::rust_owned_record_state_rules::read_rust_owned_record_state_field_identity(&selected.slot.root,&indices.iter().map(|index|*index as f64).collect::<Vec<_>>());Ok(Counter{lexical:selected.slot.lexical.clone(),indices,target,ty:origin.ty.as_ref().expect("actual admitted counter type").clone()})
}
fn text<'a>(value:&'a Value,key:&str)->&'a str{value.get(key).and_then(Value::as_str).unwrap_or("")}
fn children(node:&Value)->Result<Vec<&Value>>{let kind=text(node,"k");let roles=match kind{"let"=>vec!["value".to_owned(),"body".to_owned()],"print"=>vec!["expr".to_owned(),"body".to_owned()],"name"|"num"|"str"|"bool"|"unit"=>return Ok(Vec::new()),_=>effects::read_rust_owned_record_effect_children(kind)};if roles.is_empty(){return Err(fail("the loop body needs complete ordered source roles",None));}let mut result=Vec::new();for role in roles{if let Some(values)=node[&role].as_array(){for child in values{result.push(if child.get("k").is_none()&&child.get("value").is_some(){&child["value"]}else{child});}}else if node[&role].is_object(){result.push(&node[&role]);}}Ok(result)}
fn add(left:Vec<usize>,right:Vec<usize>)->Result<Vec<usize>>{if left.len().saturating_mul(right.len())>256{return Err(fail("the complete record loop exits exceed their finite source budget",None));}Ok(left.into_iter().flat_map(|left|right.iter().map(move|right|left+right)).collect())}
fn counts(parser:&RustParser,node:&Value,selected:&Counter,work:&mut usize)->Result<Vec<usize>>{
 *work+=1;if *work>100000{return Err(fail("the complete record loop exits exceed their finite source budget",None));}
 if text(node,"k")=="math"{if let Some(EffectSource::Write{target,..})=parser.owned_record_effect_source_parts(text(node,"op"),text(node,"name"),node["args"].as_array().map_or(0,Vec::len)){
  let rhs=&node["args"][0]["value"];let overlaps=target==selected.target||fields::test_rust_owned_record_field_descendant(target,&selected.target)||fields::test_rust_owned_record_field_descendant(&selected.target,target);
  let exact=if target==selected.target&&text(rhs,"k")=="binary"&&text(rhs,"op")=="sub"{let left=serde_json::from_value::<SExpr>(rhs["left"].clone()).map_err(|_|fail("the counter decrement needs its complete actual source place",None))?;let right=serde_json::from_value::<SExpr>(rhs["right"].clone()).map_err(|_|fail("the counter decrement needs its complete actual numeral",None))?;numeral(&right,"1")&&place(parser,&left).is_some_and(|place|place.slot.lexical==selected.lexical&&place.indices==selected.indices)}else{false};let error=rules::read_rust_owned_record_loop_write_error(overlaps,exact);if !error.is_empty(){return Err(fail(&error,None));}return Ok(counts(parser,rhs,selected,work)?.into_iter().map(|count|count+usize::from(exact)).collect());
 }}
 let kind=text(node,"k");if kind=="lambda"||kind=="rustStandardSumTry"||kind=="propositionValue"||kind=="math"&&(text(node,"op")==rules::read_rust_owned_record_loop_operation()||!crate::translation::rust_expression_completion_rules::read_rust_expression_completion_packet_operation(text(node,"op")).is_empty()){return Err(fail("this loop body needs its separate callable, packet, residual or quantified exit derivation",None));}
 if kind=="if"{let mut paths=counts(parser,&node["then"],selected,work)?;paths.extend(counts(parser,&node["else"],selected,work)?);return add(counts(parser,&node["cond"],selected,work)?,paths);}
 if kind=="binary"&&["and","or"].contains(&text(node,"op")){let mut paths=vec![0];paths.extend(counts(parser,&node["right"],selected,work)?);return add(counts(parser,&node["left"],selected,work)?,paths);}
 if kind=="match"||kind=="match1"{let mut head=vec![0];if kind=="match"{for value in node["scrutinees"].as_array().into_iter().flatten(){head=add(head,counts(parser,value,selected,work)?)?;}}else{head=add(head,counts(parser,&node["scrutinee"],selected,work)?)?;}let mut paths=Vec::new();for row in node[if kind=="match"{"rows"}else{"cases"}].as_array().into_iter().flatten(){paths.extend(counts(parser,&row["body"],selected,work)?);}return add(head,paths);}
 let mut result=vec![0];for child in children(node)?{result=add(result,counts(parser,child,selected,work)?)?;}Ok(result)
}
fn bind_names(pattern:&Value,bound:&mut HashSet<String>){if ["bind","bindOrCtor","alias","natSucc"].contains(&text(pattern,"k")){bound.insert(text(pattern,"name").to_owned());}for child in pattern["args"].as_array().into_iter().flatten(){bind_names(child,bound);}if pattern["pattern"].is_object(){bind_names(&pattern["pattern"],bound);}}
fn closed(parser:&RustParser,node:&Value,path:&[String],bound:&HashSet<String>,work:&mut usize)->Result<()>{
 *work+=1;if *work>100000{return Err(fail("the loop capture inventory exceeds its source budget",None));}let kind=text(node,"k");
 if kind=="name"{let names=node["path"].as_array().ok_or_else(||fail("the loop name needs its actual complete path",None))?.iter().map(|name|name.as_str().map(str::to_owned)).collect::<Option<Vec<_>>>().ok_or_else(||fail("the loop name needs its actual complete path",None))?;if names.len()==1&&bound.contains(&names[0]){return Ok(());}if parser.origins.lookup_value_owner_ref(&names,path).is_some_and(|(_,entry)|matches!(entry,SourceDeclaration::Function{..}|SourceDeclaration::Constructor{..})){return Ok(());}return Err(fail("a loop chunk needs explicit complete immutable capture parameters before capturing this local",None));}
 if kind=="let"{closed(parser,&node["value"],path,bound,work)?;let mut inner=bound.clone();inner.insert(text(node,"name").to_owned());return closed(parser,&node["body"],path,&inner,work);}
 if kind=="if"{for role in ["cond","then","else"]{closed(parser,&node[role],path,bound,work)?;}return Ok(());}
 if kind=="match"||kind=="match1"{if kind=="match"{for value in node["scrutinees"].as_array().into_iter().flatten(){closed(parser,value,path,bound,work)?;}}else{closed(parser,&node["scrutinee"],path,bound,work)?;}for row in node[if kind=="match"{"rows"}else{"cases"}].as_array().into_iter().flatten(){let mut inner=bound.clone();if kind=="match"{for pattern in row["patterns"].as_array().into_iter().flatten(){bind_names(pattern,&mut inner);}}else{bind_names(&row["pattern"],&mut inner);}closed(parser,&row["body"],path,&inner,work)?;}return Ok(());}
 for child in children(node)?{closed(parser,child,path,bound,work)?;}Ok(())
}
fn measure(value:&Value)->Result<(usize,usize)>{let mut pending=vec![value];let(mut nodes,mut units)=(0_usize,0_usize);while let Some(value)=pending.pop(){match value{Value::String(value)=>units=units.saturating_add(value.encode_utf16().count()),Value::Array(values)=>{nodes+=1;pending.extend(values);},Value::Object(values)=>{nodes+=1;for(key,value)in values{units=units.saturating_add(key.encode_utf16().count());pending.push(value);}},_=>{}}if nodes>100000||units>16777216{return Err(fail("the complete finite record loop expansion exceeds its source budget",None));}}Ok((nodes,units))}
impl RustParser{
 pub(super) fn read_owned_record_finite_loop(&mut self,path:&[String])->Result<Option<SExpr>>{
  let start=self.cursor.peek().clone();if start.kind!=TokenKind::Identifier||start.value!="while"||start.raw!="while"{return Ok(None);}let slots=self.record_state_active_slots();if slots.is_empty(){return Ok(None);}self.cursor.advance();
  let guard=self.with_standard_sum_source_context(None,|parser|parser.expr(path,Options{no_struct:true,..Options::default()}))?;let before=self.ordinary_completion_snapshot()?;let body_start=self.cursor.index;let body=self.block(path)?;let end=self.cursor.tokens[self.cursor.index.saturating_sub(1)].end;let span=Some(Span::new(start.start,end));
  let ordinal=self.owned_record_state.effects.as_ref().map_or(0,|effects|effects.loops.entries.len());
  let admission=if self.origins.collecting{self.origins.standard_sum_source_needs_final_admission=true;None}else{
   if self.cursor.tokens[body_start..self.cursor.index].iter().any(|token|token.kind==TokenKind::Identifier&&["return","break","continue","while","loop","for"].contains(&token.raw.as_str())){return Err(fail("this finite loop body needs its explicit nested or completed exit-state consumer",span));}
   let selected=counter(self,&guard)?;let origin=self.origins.origin(&body,path);if !origin.proven||origin.ty!=Some(super::UNIT)||!origin.wrappers.is_empty(){return Err(fail("the finite record loop body must have its actual Unit result",body.span));}
   let projection=LoopProjection{guard:&guard,body:&body,types:slots.iter().map(|slot|&slot.origin.ty).collect()};crate::translation::finite_record_loop_helpers::preflight_source(&projection,span)?;
   let value=serde_json::to_value(&projection).map_err(|_|fail("complete loop source transport failed",span))?;drop(projection);
   let(nodes,units)=measure(&value)?;let used=self.owned_record_state.effects.as_ref().map_or(0,|effects|effects.loops.nodes);let error=rules::read_rust_owned_record_loop_expansion_error(used as f64,(nodes*2+128) as f64,(units*2) as f64,255.0);if !error.is_empty(){return Err(fail(&error,span));}
   let paths=counts(self,&value["body"],&selected,&mut 0)?;let error=rules::read_rust_owned_record_loop_progress_error(&paths.iter().map(|_|"value".to_owned()).collect::<Vec<_>>(),&paths.iter().map(|value|*value as f64).collect::<Vec<_>>());if !error.is_empty(){return Err(fail(&error,span));}closed(self,&value["guard"],path,&HashSet::new(),&mut 0)?;closed(self,&value["body"],path,&HashSet::new(),&mut 0)?;
   let after=self.ordinary_completion_snapshot()?;for key in before.keys().chain(after.keys()).collect::<HashSet<_>>(){if !before.contains_key(key)&&!slots.iter().any(|slot|fields::test_rust_owned_record_field_descendant(&slot.root,key)){continue;}let error=rules::read_rust_owned_record_loop_invariant_error(&super::owned_record_updates::field_state(&before,key),&super::owned_record_updates::field_state(&after,key));if !error.is_empty(){return Err(fail(&error,span));}}
   self.origins.restore_owned_bindings(before);self.owned_record_state.effects.get_or_insert_with(Default::default).loops.nodes+=(nodes*2+128)*255;
   Some(Arc::new(AdmittedFiniteRecordLoop{span,counter_type:selected.ty,slots,path:path.to_vec()}))
  };
  self.owned_record_state.effects.get_or_insert_with(Default::default).loops.entries.push(LoopEntry{admission});let mut node=SExpr::new(SNode::Math{op:rules::read_rust_owned_record_loop_operation(),name:ordinal.to_string(),args:vec![SArrayItem::new(guard,false),SArrayItem::new(body,false)]},span);node.block=true;self.origins.mark(&node,SourceOrigin::with_type(super::UNIT,path,Vec::new()));Ok(Some(node))
 }
}
