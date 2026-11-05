//! Opaque source provenance and an independently derived checked call graph.
//! Neither the flag nor the witness supplies Copy, ownership or type facts.
use std::collections::{HashMap,HashSet};
use std::sync::Arc;
use std::io::{self,Write};
use serde_json::Value;
use super::diagnostics::{Result,unsupported};
use super::ir::{Decl,FnDecl,Node,Program};
use super::surface::{SFn,SItem,SProgram};
use super::Span;
use super::rust_owned_record_loop_rules as rules;

/// Public opaque type for the public Surface function field. Its source
/// factory remains crate-internal and requires an admitted source-loop token.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct FiniteRecordLoopWitness{expected:Value,source_span:Option<Span>}
// Serialize borrowed fields to a bounded counting sink BEFORE allocating the
// snapshot. String contents and nested metadata cannot hide behind one Expr.
// Count JSON keys too: this is a conservative upper bound on source nodes.
#[derive(Default)]
struct WitnessBudget{bytes:usize,nodes:usize,depth:usize,quoted:bool,escaped:bool,scalar:bool}
impl Write for WitnessBudget{
 fn write(&mut self,bytes:&[u8])->io::Result<usize>{
  for &byte in bytes{
   self.bytes=self.bytes.saturating_add(1);
   if self.quoted{if self.escaped{self.escaped=false;}else if byte==b'\\'{self.escaped=true;}else if byte==b'"'{self.quoted=false;}}
   else{match byte{
    b'"'=>{self.nodes=self.nodes.saturating_add(1);self.quoted=true;self.scalar=false;},
    b'{'|b'['=>{self.nodes=self.nodes.saturating_add(1);self.depth=self.depth.saturating_add(1);self.scalar=false;},
    b'}'|b']'=>{self.depth=self.depth.saturating_sub(1);self.scalar=false;},
    b','|b':'|b' '|b'\n'|b'\r'|b'\t'=>self.scalar=false,
    _=>{if !self.scalar{self.nodes=self.nodes.saturating_add(1);self.scalar=true;}}
   }}
   if self.nodes>100000||self.bytes>16777216||self.depth>256{return Err(io::Error::other("complete source helper witness exceeds its finite source budget"));}
  }Ok(bytes.len())
 }
 fn flush(&mut self)->io::Result<()>{Ok(())}
}
fn snapshot(function:&SFn)->Result<Value>{
 preflight_source(function,function.span)?;
 serde_json::to_value(function).map_err(|_|unsupported("finite record loop helper","complete source helper encoding failed",function.span))
}
pub(crate) fn preflight_source<T:serde::Serialize>(value:&T,span:Option<Span>)->Result<()>{let mut budget=WitnessBudget::default();serde_json::to_writer(&mut budget,value).map_err(|_|unsupported("finite record loop helper","the complete source helper witness exceeds its finite source budget",span))}
pub(crate) fn seal(function:&mut SFn,source:&super::rust::owned_record_loops::AdmittedFiniteRecordLoop)->Result<()>{
 let ty=source.counter_type();let(bits,signed)=match ty{super::types::Type::Fixed{bits,signed}=>(*bits as f64,*signed),_=>(0.0,false)};
 let error=rules::read_rust_owned_record_loop_counter_error(ty.kind(),bits,signed,0.0,"u8","ne");if !error.is_empty(){return Err(unsupported("finite record loop helper",&error,source.source_span()));}
 if !function.generated||!function.source_finite_record_loop_helper||!rules::test_rust_owned_record_loop_chunk_name(&function.name){return Err(unsupported("finite record loop helper","only an actual generated chunk can receive a source witness",function.span));}
 // The witness field is serde-skipped, so this snapshot cannot capture itself.
 function.source_finite_record_loop_witness=None;let expected=snapshot(function)?;function.source_finite_record_loop_witness=Some(Arc::new(FiniteRecordLoopWitness{expected,source_span:source.source_span()}));Ok(())
}
pub(crate) fn validate(source:&SProgram,program:&Program,projected:bool)->Result<()>{
 let functions=program.declarations.iter().filter_map(|entry|match entry{Decl::Fn(function)=>Some((function.full_name.as_str(),function)),_=>None}).collect::<HashMap<_,_>>();
 let mut helpers=Vec::<&FnDecl>::new();let mut pending=source.items.iter().map(|item|(item,Vec::<String>::new())).collect::<Vec<_>>();let mut work=0_usize;
 while let Some((item,path))=pending.pop(){work+=1;if work>100000{return Err(unsupported("finite record loop helper","the complete declaration inventory exceeds its source budget",None));}match item{
  SItem::Module(module)=>{let mut nested=path;nested.push(module.name.clone());pending.extend(module.items.iter().map(|item|(item,nested.clone())));},
  SItem::Fn(function)if function.source_finite_record_loop_helper||function.source_finite_record_loop_witness.is_some()||(function.generated&&rules::test_rust_owned_record_loop_chunk_name(&function.name))=>{
   if !projected{let witness=function.source_finite_record_loop_witness.as_ref().ok_or_else(||unsupported("finite record loop helper","the complete source helper signature and body need their authentic loop witness",function.span))?;if !function.source_finite_record_loop_helper||witness.source_span!=function.span||witness.expected!=snapshot(function)?{return Err(unsupported("finite record loop helper","the complete source helper signature and body need their authentic loop witness",function.span));}}
   let name=path.iter().map(String::as_str).chain(std::iter::once(function.name.as_str())).collect::<Vec<_>>().join(".");let entry=functions.get(name.as_str()).copied().ok_or_else(||unsupported("finite record loop helper","the source helper needs its actual checked function declaration",function.span))?;helpers.push(entry);
  },_=>{}
 }}
 let mut analyzed=HashSet::new();for helper in helpers{let mut seen=HashSet::new();let mut reachable=vec![helper];while let Some(entry)=reachable.pop(){if !seen.insert(entry.full_name.as_str())||!analyzed.insert(entry.full_name.as_str()){continue;}let mut unknown=false;let mut nodes=vec![&entry.body];while let Some(node)=nodes.pop(){work+=1;if work>100000{return Err(unsupported("finite record loop helper","the actual helper graph exceeds its source budget",helper.span));}match &node.node{Node::Apply{..}=>unknown=true,Node::Call{func,..}=>{if let Some(target)=functions.get(func.as_str()){reachable.push(*target);}else{unknown=true;}},_=>{}}nodes.extend(node.children());}let error=rules::read_rust_owned_record_loop_helper_error(if std::ptr::eq(entry,helper){entry.generated}else{true},entry.recursive,entry.mutual,unknown);if !error.is_empty(){return Err(unsupported("finite record loop helper",&error,helper.span));}}}
 Ok(())
}
