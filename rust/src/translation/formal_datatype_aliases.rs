//! Original name/scope transport over shared target-alias rendering decisions.
//! No projected program becomes an archived or source-checked nominal graph.
use std::collections::HashSet;
use super::diagnostics::{Result,unsupported};
use super::emit_common::EmitState;
use super::formal_datatype_modules::FormalDatatypeAlias;
use super::formal_datatype_alias_rules as rules;
use super::ir::Decl;
fn fail(reason:&str)->super::diagnostics::TranslationError{unsupported("formal datatype public aliases",reason,None)}
pub(crate) struct CapturedFormalDatatypeAlias{
 alias:FormalDatatypeAlias,
 public_name:String,
 public_constructors:Vec<String>,
}
#[cfg(test)]
mod deferred_controls{
 use super::*;
 #[test]
 fn actual_nested_datatypes_emit_complete_mutual_blocks_and_public_pattern_aliases(){
  // Backend graph control: the original parsed bool fields are explicitly
  // replaced in a checked graph. This is not source-size admission evidence.
  let surface=crate::translation::rust::parse_rust("mod ir { struct Expr(bool); mod nodes { enum Node { Unit, Unary(bool) } } } fn main() {}").unwrap();
  let mut original=crate::translation::check::check_program(&surface).unwrap();
  for entry in &mut original.declarations{if let Decl::Data(data)=entry{if data.full_name=="ir.Expr"{data.ctors[0].fields[0].ty=crate::translation::types::Type::Data{name:"ir.nodes.Node".into()};}else if data.full_name=="ir.nodes.Node"{data.ctors[1].fields[0].ty=crate::translation::types::Type::Data{name:"ir.Expr".into()};}}}
  let saved=original.clone();assert!(crate::translation::emit_common::order_declarations(&original,true).is_err());
  let lean=crate::translation::emit_lean::emit_lean(&original).unwrap().text;let rocq=crate::translation::emit_rocq::emit_rocq(&original).unwrap().text;
  assert!(lean.contains("mutual\ninductive"));assert!(lean.contains("abbrev Node := _root_."));assert!(lean.contains("@[match_pattern] abbrev"));
  assert!(rocq.contains("\nwith "));assert!(rocq.contains("Definition Node : Type := "));assert!(rocq.contains("Notation "));assert_eq!(rocq.matches("Module nodes.").count(),1);assert_eq!(original,saved);
 }
}
pub(crate) fn capture_formal_datatype_aliases(aliases:&[FormalDatatypeAlias],state:&EmitState<'_>)->Result<Vec<CapturedFormalDatatypeAlias>>{
 let mut count=0;for alias in aliases{count+=1+alias.constructors.len();let error=rules::read_formal_datatype_alias_budget_error(count as f64,0.0);if !error.is_empty(){return Err(fail(&error));}}
 Ok(aliases.iter().map(|alias|CapturedFormalDatatypeAlias{alias:alias.clone(),public_name:state.local_name(&alias.source_full_name).to_owned(),public_constructors:alias.constructors.iter().map(|name|state.ctor_local(&alias.source_full_name,name).to_owned()).collect()}).collect())
}
pub(crate) struct FormalDatatypeAliases{
 rows:Vec<CapturedFormalDatatypeAlias>,count:usize,target:String,required:HashSet<String>,completed:HashSet<String>,emitted:HashSet<String>,opened:HashSet<Vec<String>>,units:usize,visits:usize,
}
impl FormalDatatypeAliases{
 pub(crate) fn new(rows:Vec<CapturedFormalDatatypeAlias>,target:&str)->Self{let count=rows.iter().map(|row|1+row.alias.constructors.len()).sum();let required=rows.iter().map(|row|row.alias.target_full_name.clone()).collect();Self{rows,count,target:target.to_owned(),required,completed:HashSet::new(),emitted:HashSet::new(),opened:HashSet::new(),units:0,visits:0}}
 fn step(&mut self,depth:usize)->Result<()>{self.visits+=1;let error=rules::read_formal_datatype_alias_traversal_error(self.visits as f64,depth as f64);if error.is_empty(){Ok(())}else{Err(fail(&error))}}
 pub(crate) fn has_aliases(&self)->bool{!self.rows.is_empty()}
 pub(crate) fn complete(&mut self,entries:&[&Decl]){for entry in entries{if self.required.contains(entry.full_name()){self.completed.insert(entry.full_name().to_owned());}}}
 fn append(&mut self,lines:&mut Vec<String>,value:String)->Result<()>{if value.is_empty(){return Err(fail("the selected target needs its complete alias syntax"));}self.units+=value.encode_utf16().count();let error=rules::read_formal_datatype_alias_budget_error(self.count as f64,self.units as f64);if !error.is_empty(){return Err(fail(&error));}lines.push(value);Ok(())}
 pub(crate) fn open(&mut self,path:&[String],state:&EmitState<'_>)->Result<Vec<String>>{
  if self.rows.is_empty(){return Ok(Vec::new());}
  self.step(path.len())?;if self.opened.contains(path){for index in 0..self.rows.len(){self.step(path.len())?;if self.rows[index].alias.source_module_path.starts_with(path){return Err(fail("an original alias module would have to reopen"));}}}self.opened.insert(path.to_vec());let mut lines=Vec::new();
  for index in 0..self.rows.len(){self.step(path.len())?;let row=&self.rows[index];if row.alias.source_module_path!=path||self.emitted.contains(&row.alias.source_full_name){continue;}if !self.completed.contains(&row.alias.target_full_name){return Err(fail("the original alias scope opened before its complete lifted datatype block"));}
   let source=row.alias.source_full_name.clone();let mut values=vec![rules::render_formal_datatype_type_alias(&self.target,&row.public_name,&state.reference(&row.alias.target_full_name,"."))];
   for (index,name) in row.alias.constructors.iter().enumerate(){values.push(rules::render_formal_datatype_constructor_alias(&self.target,&row.public_name,&row.public_constructors[index],&state.ctor_ref(&row.alias.target_full_name,name,".")));}
   for value in values{self.append(&mut lines,value)?;}self.emitted.insert(source);
  }Ok(lines)
 }
 pub(crate) fn close(&mut self,path:&[String],state:&EmitState<'_>)->Result<Vec<String>>{
  if self.rows.is_empty(){return Ok(Vec::new());}
  self.step(path.len())?;let mut lines=Vec::new();loop{let mut selected=None;for index in 0..self.rows.len(){self.step(path.len())?;let row=&self.rows[index];if !self.emitted.contains(&row.alias.source_full_name)&&row.alias.source_module_path.starts_with(path){selected=Some(index);break;}}let Some(index)=selected else{break;};let row=&self.rows[index];if row.alias.source_module_path.len()==path.len(){return Err(fail("the complete original alias scope was not emitted when opened"));}let mut child=path.to_vec();child.push(row.alias.source_module_path[path.len()].clone());let name=state.module_name(&child).ok_or_else(||fail("the original alias module has no actual reserved target name"))?;
   self.append(&mut lines,rules::render_formal_datatype_alias_module(&self.target,name,true))?;lines.extend(self.open(&child,state)?);lines.extend(self.close(&child,state)?);self.append(&mut lines,rules::render_formal_datatype_alias_module(&self.target,name,false))?;
  }Ok(lines)
 }
 pub(crate) fn finish(&self)->Result<()>{if self.emitted.len()!=self.rows.len(){Err(fail("every relocated datatype and constructor needs its original public alias"))}else{Ok(())}}
}
