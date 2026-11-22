//! Bounded checked-AST transport into shared immutable source-region decisions.
//! Local coordinates identify actual lexical binders, never persisted runtime IDs.
use crate::translation::source_signatures::source_policy_column;
use std::{collections::{HashMap,HashSet},rc::Rc};
use super::{diagnostics::{TranslationError,type_error},ir::{Program,Decl,FnDecl,DataDecl,Expr,Node,Effect,Prop,Pattern,LitValue},types::Type,surface::BinaryOp,source_signatures::{SourceTypeSignature as Position,SourceOwner},rust_borrowed_source_region_rules as rules,rust_borrowed_vector_rules as borrowed};
mod folds;
mod matching;
mod packets;
mod place_transforms;
mod record_fields;
mod storage;
mod state_transforms;
mod vector_mutations;
use folds::{FoldControl,FoldFrame};
use packets::StatePacket;
use record_fields::{FieldPlace, RecordImage};
use storage::{Storage, StorageValue};

const MODEL:&str="Rust source function signature 3";
const CLOSED_MODEL:&str="Rust source function signature 2";
#[derive(Debug)]
enum RegionError {
    Invalid(TranslationError),
    Diverged,
}
type Result<T> = std::result::Result<T, RegionError>;
impl From<TranslationError> for RegionError {
    fn from(error: TranslationError) -> Self {
        Self::Invalid(error)
    }
}
impl RegionError {
    fn into_diagnostic(self) -> TranslationError {
        match self {
            Self::Invalid(error) => error,
            Self::Diverged => type_error("a source abort left its checked control-flow scope", None),
        }
    }
}
fn fail(message:&str)->RegionError{type_error(message,None).into()}
fn accept(error:String)->Result<()>{if error.is_empty(){Ok(())}else{Err(fail(&error))}}
fn references(p:&Position)->usize{p.wrappers.iter().filter(|v|v.as_str()=="reference").count()}
fn role(p:&Position)->String{let(bits,signed)=match &p.ty{Type::Fixed{bits,signed}=>(bits.to_string(),*signed),_=>(String::new(),false)};rules::read_rust_borrowed_source_value_role(p.ty.kind(),&bits,signed,&p.owner.kind,&p.owner.path.join("::"),p.owner.arguments.len() as f64,&p.wrappers,p.array_element_position.is_some())}
fn lifetime(p:&Position)->String{if role(p)=="borrowed vector"{p.array_element_position.as_deref().and_then(|p|p.lifetimes.first()).and_then(Clone::clone).unwrap_or_default()}else{p.lifetimes.first().and_then(Clone::clone).unwrap_or_default()}}
fn same_owner(left:Option<&Position>,right:Option<&Position>)->bool{match(left,right){(Some(l),Some(r))=>l.owner==r.owner&&match(l.array_element_position.as_deref(),r.array_element_position.as_deref()){(None,None)=>true,(Some(a),Some(b))=>a.wrappers==b.wrappers&&same_owner(Some(a),Some(b)),_=>false},_=>false}}
fn instantiate_call_parameter(position:&mut Position){for life in &mut position.lifetimes{if life.as_deref()!=Some("static"){*life=None;}}if let Some(child)=position.array_element_position.as_deref_mut(){instantiate_call_parameter(child);}}
#[derive(Clone)]struct Value{position:Option<Rc<Position>>,root:Option<usize>,regions:Vec<usize>,borrow_roots:Vec<usize>,producer:bool,inference:Option<usize>,role:String,storage:Option<StorageValue>,packet:Option<StatePacket>,control:Option<FoldControl>,place:Option<FieldPlace>}
impl Value{fn new(position:Option<Rc<Position>>,root:Option<usize>,regions:Vec<usize>,producer:bool,inference:Option<usize>)->Self{let role=position.as_deref().map(role).unwrap_or_else(||"inferred numeric".into());let borrow_roots=if matches!(role.as_str(),"reference"|"borrowed vector"){regions.clone()}else{Vec::new()};Self{position,root,regions,borrow_roots,producer,inference,role,storage:None,packet:None,control:None,place:None}}}
fn observation_role(v:&Value)->String{match v.position.as_deref(){Some(p)=>rules::read_rust_borrowed_source_observation_role(&v.role,p.ty.kind(),&p.owner.kind,&p.owner.path.join("::"),p.owner.arguments.len() as f64,&p.wrappers),None=>v.role.clone()}}
type Environment=HashMap<String,Value>;
struct Regions<'p>{program:&'p Program,states:Vec<String>,lifetimes:Vec<String>,temporaries:Vec<usize>,constraints:HashMap<usize,Rc<Position>>,closed:HashSet<String>,visits:usize,loans:usize,position_nodes:usize,position_units:usize,branch_cells:usize,discovery:bool,composed:bool,storages:HashMap<String,Storage>,folds:Vec<Rc<FoldFrame>>,field_images:HashMap<usize,RecordImage>,field_lifetimes:HashMap<usize,usize>,borrowed_field_views:HashMap<usize,HashMap<String,usize>>}
impl<'p> Regions<'p>{
 fn new(program:&'p Program)->Self{Self{program,states:vec!["available".into()],lifetimes:vec!["static".into()],temporaries:Vec::new(),constraints:HashMap::new(),closed:HashSet::new(),visits:0,loans:0,position_nodes:0,position_units:0,branch_cells:0,discovery:true,composed:false,storages:HashMap::new(),folds:Vec::new(),field_images:HashMap::new(),field_lifetimes:HashMap::new(),borrowed_field_views:HashMap::new()}}
 fn reset_scope(&mut self) {
     self.states.truncate(1);
     self.states[0]="available".into();
     self.lifetimes.truncate(1);
     self.temporaries.clear();
     self.constraints.clear();
     self.storages.clear();
     self.folds.clear();
     self.field_images.clear();
     self.field_lifetimes.clear();
     self.borrowed_field_views.clear();
 }
 fn branches(&mut self,addition:usize)->Result<()>{let next=rules::reserve_rust_borrowed_source_branches(self.branch_cells as f64,addition as f64);if next<0.0{return Err(fail("source branch snapshots exceed their finite binder profile"));}self.branch_cells=next as usize;Ok(())}
 fn budget(&self,depth:usize)->Result<()>{if self.discovery{return Ok(());}accept(rules::read_rust_borrowed_source_region_budget_error(self.visits as f64,depth as f64,self.states.len() as f64,self.loans as f64))}
 fn position(&mut self,p:&Position)->Result<Rc<Position>>{let(nodes,units)=super::rust::measure_source_position_metadata(p).map_err(|e|fail(&e))?;let next=rules::reserve_rust_borrowed_source_position(self.position_nodes as f64,self.position_units as f64,nodes as f64,units as f64);if next.len()!=2{return Err(fail("source binder position transport exceeds its materialization budget"));}self.position_nodes=next[0] as usize;self.position_units=next[1] as usize;Ok(Rc::new(p.clone()))}
 fn inner(&mut self,p:Option<&Position>)->Result<Option<Rc<Position>>>{let Some(p)=p else{return Ok(None)};let Some(layer)=p.wrappers.first()else{return Ok(None)};let mut next=self.position(p)?.as_ref().clone();if layer=="reference"{next.wrappers.remove(0);next.lifetimes.remove(0);}else if layer=="box"{next.wrappers.remove(0);next.owner=p.owner.arguments.first().ok_or_else(||fail("a box position needs its actual owned argument"))?.clone();}else{return Ok(None)}Ok(Some(Rc::new(next)))}
 fn root(&mut self,life:String,temporary:bool)->Result<usize>{self.budget(0)?;let id=self.states.len();self.states.push("available".into());self.lifetimes.push(life);if temporary{self.temporaries.push(id);}self.budget(0)?;Ok(id)}
 fn union(&mut self,values:impl IntoIterator<Item=Vec<usize>>)->Result<Vec<usize>>{let mut seen=HashSet::new();let mut result=Vec::new();for values in values{for id in values{self.loans+=1;self.budget(0)?;if seen.insert(id){result.push(id);}}}Ok(result)}
 fn resolved(&self,mut v:Value)->Value{if let Some(p)=v.inference.and_then(|id|self.constraints.get(&id)){v.position=Some(p.clone());v.role="copy".into();}v}
 fn read_lifetime(&self, v: Value) -> Result<Value> {
     let v = self.resolved(v);
     let loans: Vec<String> = v.regions.iter()
         .chain(v.place.iter().flat_map(|place| place.ancestors.iter().chain(&place.roots)))
         .chain(v.place.iter().flat_map(|place| place.roots.iter().filter_map(|root| self.field_lifetimes.get(root))))
         .map(|id| self.states[*id].clone()).collect();
     accept(rules::read_rust_borrowed_source_read_error(
         v.root.map(|id| self.states[id].as_str()).unwrap_or(""), &loans,
     ))?;
     self.check_storage_revision(&v)?;
     if self.composed {
         self.check_record_roots(v.borrow_roots.clone())?;
     }
     Ok(v)
 }
 fn read(&mut self, v: Value, mode: &str) -> Result<Value> {
     let mut v = self.read_lifetime(v)?;
     if self.composed && !(mode == "transport" && v.storage.is_some()) {
         self.check_place_image(&v)?;
     }
     if v.packet.is_some() { return self.read_state_packet(v, mode); }
     if v.control.is_some() { return self.read_loop_control(v, mode); }
     let plan = rules::read_rust_borrowed_source_use_plan(&v.role, mode);
     if plan == "unsupported" { return Err(fail("the source value needs its independent ownership domain")); }
     if mode == "transport" {
         if let Some(id) = v.root.take()
             && matches!(v.role.as_str(), "owned" | "string" | "box" | "borrowed vector")
             && !v.storage.as_ref().is_some_and(|slot| slot.revision == id)
         {
             self.states[id] = rules::consume_rust_borrowed_source_region(&self.states[id]);
         }
         v.producer = true;
     } else if plan == "consume" {
         if v.place.as_ref().is_some_and(|place| place.borrowed) {
             return Err(fail("moving a borrowed source field requires an actual clone or transfer proof"));
         }
         if v.root.is_none() && !v.producer {
             return Err(fail("an owned source value needs an actual producer or available lexical binder"));
         }
         let mut places = Self::place_roots(&v);
         if let Some(root) = v.root.take()
             && !places.contains(&root)
         {
             places.push(root);
         }
         v.storage = None;
         for root in places {
             self.retire_field_lifetime(root,"move");
             self.states[root] = rules::consume_rust_borrowed_source_region(&self.states[root]);
         }
         v.producer = true;
         v.place = None;
     } else if mode == "move" {
         v.storage = None;
         v.place = None;
     }
     Ok(v)
 }
 fn scope(&self,p:&Position,bindings:&[String])->Result<()>{let mut current=Some(p);let mut depth=0;while let Some(p)=current{self.budget(depth)?;for life in &p.lifetimes{accept(borrowed::read_rust_borrowed_expression_lifetime_scope_error(life.as_deref().unwrap_or(""),bindings))?;}current=p.array_element_position.as_deref();depth+=1;}Ok(())}
 fn data(&self,name:&str)->Option<&'p DataDecl>{self.program.declarations.iter().find_map(|d|match d{Decl::Data(d)if d.full_name==name=>Some(d),_=>None})}
 fn closed_target(&mut self, position: &Position) -> Result<()> {
     let mut position = self.position(position)?;
     while !position.wrappers.is_empty() {
         position = self.inner(Some(&position))?
             .ok_or_else(|| fail("a referent needs its complete source position"))?;
     }
     let mut pending = vec![position];
     let mut visits = 0;
     while let Some(position) = pending.pop() {
         visits += 1;
         if visits > 100_000 {
             return Err(fail("the closed referent field graph exceeds its source budget"));
         }
         let position = if self.composed && !position.wrappers.is_empty()
             && let Some((owner, _)) = super::source_record_places::owner_view(&position, false)
         {
             let mut inner = position.as_ref().clone();
             inner.owner = owner.clone();
             inner.wrappers.clear();
             self.position(&inner)?
         } else {
             position
         };
         accept(rules::read_rust_borrowed_source_target_field_error(
             position.ty.kind(), &position.wrappers, position.array_element_position.is_some(),
         ))?;
         match &position.ty {
             Type::Data { name } => {
                 if self.closed.contains(name) {
                     continue;
                 }
                 let data = self.data(name).ok_or_else(|| fail("the referent needs its actual datatype"))?;
                 let fields = data.source_field_signatures.as_deref()
                     .filter(|entry| entry.model == "Rust data type field signatures 1")
                     .ok_or_else(|| fail("the actual referent datatype needs its complete retained source field inventory"))?;
                 super::callable_data::read_formal_datatype_constructive_profiles(&position.ty, &|name| self.data(name))?;
                 self.closed.insert(name.clone());
                 for constructor in &fields.ctors {
                     for field in &constructor.fields {
                         pending.push(self.position(field)?);
                     }
                 }
             }
             Type::Array { element, .. } => {
                 if position.owner.kind != "library" || position.owner.path.join("::") != "std::vec::Vec"
                     || position.owner.arguments.len() != 1
                 {
                     return Err(fail("a referent array needs its actual standard vector owner"));
                 }
                 pending.push(self.position(&Position {
                     ty: element.as_ref().clone(),
                     wrappers: Vec::new(),
                     lifetimes: Vec::new(),
                     owner: position.owner.arguments[0].clone(),
                     array_element_position: None,
                 })?);
             }
             _ => {}
         }
     }
     Ok(())
 }
 fn check_lifetime(&self,v:&Value,p:&Position)->Result<()>{accept(rules::read_rust_borrowed_source_lifetime_error(&lifetime(p),&v.regions.iter().map(|id|self.lifetimes[*id].clone()).collect::<Vec<_>>()))}
 fn box_reference(&mut self,mut value:Value,target:&Position)->Result<Value>{let Some(source)=value.position.as_deref()else{return Ok(value)};if source.wrappers.first().map(String::as_str)!=Some("reference")||target.wrappers.first().map(String::as_str)!=Some("reference")||source.wrappers.len()<=target.wrappers.len(){return Ok(value);}let drop=source.wrappers.len()-target.wrappers.len();let mut owner=&source.owner;let mut canonical=true;for _ in 0..drop{if !super::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(source_policy_column(&["box".into()]),&owner.kind,&owner.path.join("::"),owner.arguments.len() as f64){canonical=false;break;}owner=&owner.arguments[0];}let wrappers:Vec<String>=std::iter::once("reference".into()).chain(source.wrappers[1+drop..].iter().cloned()).collect();let same=owner==&target.owner&&match(source.array_element_position.as_deref(),target.array_element_position.as_deref()){(None,None)=>true,(Some(a),Some(b))=>a.wrappers==b.wrappers&&same_owner(Some(a),Some(b)),_=>false};if !rules::accept_rust_borrowed_source_box_reference(&source.wrappers,&target.wrappers,canonical,same){return Ok(value);}let mut projected=self.position(source)?.as_ref().clone();projected.owner=owner.clone();projected.wrappers=wrappers;value.position=Some(Rc::new(projected));Ok(value)}
 fn adjust(&mut self,v:Value,p:Option<Rc<Position>>,node:&Expr)->Result<Value>{let mut v=self.resolved(v);let Some(p)=p else{return Ok(v)};v=self.box_reference(v,&p)?;let wanted=role(&p);if wanted.is_empty(){return Err(fail("the actual expression source position needs a separate value domain"));}let target_inner=self.inner(Some(&p))?;if v.role=="inferred numeric"&&wanted=="reference"&&target_inner.as_deref().map(role).as_deref()==Some("copy"){if let Some(id)=v.inference{self.constraints.insert(id,target_inner.clone().unwrap());v=self.resolved(v);}}let source_inner=self.inner(v.position.as_deref())?;let plan=rules::read_rust_borrowed_source_adjustment(&v.role,&wanted,same_owner(v.position.as_deref(),Some(&p)),same_owner(v.position.as_deref(),target_inner.as_deref()),same_owner(source_inner.as_deref(),Some(&p)),v.root.is_some(),matches!(&node.node,Node::Lit{..})&&target_inner.as_deref().map(role).as_deref()==Some("copy"),source_inner.as_deref().map(role).as_deref()==Some("text"),v.position.as_deref().map(references).unwrap_or(0) as f64,references(&p) as f64);
 match plan.as_str(){"preserve"|"project reference"=>{},"constrain primitive"=>{let id=v.inference.ok_or_else(||fail("an inferred primitive needs its actual literal dependency"))?;self.constraints.insert(id,p.clone());v=self.resolved(v);},"borrow"|"temporary borrow"=>{self.closed_target(&p)?;v=self.read(v,"read")?;let root=match v.storage.as_ref().map(|storage|storage.revision).or(v.root){Some(id)=>id,None=>self.root(String::new(),true)?};v.borrow_roots=v.place.as_ref().map_or_else(||vec![root],|place|place.roots.clone());let place=match v.place.as_ref(){Some(place)=>{let mut regions=place.ancestors.clone();for root in &place.roots{regions.push(*root);regions.push(self.field_lifetime(*root)?);}regions},None=>vec![root]};v.regions=self.union([v.regions,place])?;v.place=None;v.producer=false;},"static literal"=>{self.closed_target(&p)?;v.regions=vec![0];v.borrow_roots=vec![0];v.producer=false;},"copy referent"=>{v=self.read(v,"read")?;v.root=None;v.storage=None;v.regions.clear();v.borrow_roots.clear();v.place=None;v.producer=false;},"construct box"|"move box payload"=>{v=self.read(v,"move")?;v.root=Some(self.root(String::new(),true)?);v.producer=true;},"copy box payload"=>{v=self.read(v,"read")?;v.root=None;v.producer=false;},"materialize text"=>{v=self.read(v,"read")?;v.root=Some(self.root(String::new(),true)?);v.regions.clear();v.borrow_roots.clear();v.place=None;v.producer=true;},_=>return Err(fail("the expression qualifier change needs its explicit source operation proof"))}
 if wanted=="reference"{self.closed_target(&p)?;}if wanted=="borrowed vector"{self.closed_target(p.array_element_position.as_deref().ok_or_else(||fail("a borrowed vector needs its actual element position"))?)?;}v.position=Some(p.clone());v.role=wanted;self.check_lifetime(&v,&p)?;Ok(v)}
 fn compatible(&mut self,v:Value,p:Option<&Position>)->Result<Value>{let mut v=self.resolved(v);let Some(p)=p else{return Ok(v)};if v.role=="inferred numeric"&&role(p)=="copy"{let id=v.inference.ok_or_else(||fail("an inferred primitive needs its actual literal dependency"))?;let p=self.position(p)?;self.constraints.insert(id,p);v=self.resolved(v);}let same=same_owner(v.position.as_deref(),Some(p))&&v.role==role(p)&&v.position.as_deref().map(references).unwrap_or(0)>=references(p);accept(rules::read_rust_borrowed_source_position_flow_error(&v.role,same,lifetime(p)=="static",v.regions.iter().all(|id|self.lifetimes[*id]=="static")))?;self.check_lifetime(&v,p)?;Ok(v)}
 fn bind(&mut self, env: &mut Environment, name: &str, value: Value) -> Result<usize> {
     let transport = value.storage.as_ref().is_some_and(|storage| storage.transport);
     let mut value = self.read(value, if transport { "transport" } else { "move" })?;
     let id = self.root(String::new(), false)?;
     if let Some(storage) = value.storage.as_mut() {
         storage.transport = false;
     }
     value.root = Some(id);
     value.producer = false;
     self.prepare_value_record_images(&value)?;
     env.insert(name.into(), value);
     Ok(id)
 }
 fn finish(&mut self,v:Value,start:usize)->Result<Value>{
     for index in start..self.temporaries.len(){
         let id=self.temporaries[index];
         self.states[id]=rules::expire_rust_borrowed_source_region(&self.states[id]);
         self.retire_field_lifetime(id,"expire");
     }
     if v.storage.is_some() {
         self.read_lifetime(v)
     } else {
         self.read(v, "read")
     }
 }
 fn sources(result:&Position,parameters:&[Position])->Vec<f64>{let r=role(result);if r!="reference"&&r!="borrowed vector"{return Vec::new();}borrowed::plan_rust_borrowed_vector_result_parameters(&lifetime(result),&parameters.iter().map(|p|matches!(role(p).as_str(),"reference"|"borrowed vector")).collect::<Vec<_>>(),&parameters.iter().map(lifetime).collect::<Vec<_>>())}
 fn iterator_item(&mut self,source:&Value)->Result<Rc<Position>>{let p=source.position.as_deref().ok_or_else(||fail("a readonly source method needs its actual borrowed vector receiver position"))?;let Type::Array{element,..}=&p.ty else{return Err(fail("a readonly source method needs its actual borrowed vector receiver position"))};if p.owner.kind!="library"||p.owner.path.join("::")!="std::vec::Vec"||p.owner.arguments.len()!=1||!matches!(source.role.as_str(),"reference"|"borrowed vector"){return Err(fail("a readonly source method needs its actual borrowed vector receiver position"));}if let Some(child)=p.array_element_position.as_deref(){return self.position(child);}let mut owner=&p.owner.arguments[0];let mut wrappers=vec!["reference".into()];while super::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(source_policy_column(&["box".into()]),&owner.kind,&owner.path.join("::"),owner.arguments.len() as f64){wrappers.push("box".into());owner=&owner.arguments[0];if wrappers.len()>129{return Err(fail("readonly iterator element nesting exceeds its finite position profile"));}}self.position(&Position{ty:element.as_ref().clone(),wrappers,lifetimes:vec![p.lifetimes.first().cloned().flatten()],owner:p.owner.arguments[0].clone(),array_element_position:None})}
 fn callback_projection(&mut self,root:&Expr)->Result<bool>{let mut pending=vec![root];while let Some(node)=pending.pop(){self.visits+=1;self.budget(0)?;let kind=if matches!(&node.node,Node::Var{..}){"name"}else{node.node.kind()};match super::rust_borrowed_iterator_rules::read_rust_borrowed_iterator_projection_syntax(kind).as_str(){"name"=>{},"match"=>{let Node::Match{scrutinee,cases}=&node.node else{return Ok(false)};pending.push(scrutinee);for arm in cases{if !matches!(arm.pattern,Pattern::Ctor{..}|Pattern::Wild|Pattern::Bind{..}){return Ok(false);}pending.push(&arm.body);}},_=>return Ok(false)}}Ok(true)}
 fn iterator_expression(&mut self,node:&Expr,env:&Environment,bindings:&[String],expected:Option<&Position>,depth:usize)->Result<Value>{let(receiver,operation)=match &node.node{Node::Method{object,op,..}=>(object.as_ref(),op.as_str()),Node::Append{left,..}=>(left.as_ref(),"concat"),_=>return Err(fail("a readonly method needs its actual checked term"))};let source=self.expression(receiver,env,bindings,None,depth+1)?;let source=self.read(source,"read")?;let item=self.iterator_item(&source)?;let wanted=node.source_position.as_deref().map(|v|&v.position).or(expected).filter(|p|role(p)=="borrowed vector").ok_or_else(||fail("a readonly method needs its complete owning borrowed-vector result position"))?;let mut regions=source.regions.clone();let mut output=item.clone();match (&node.node,operation){
  (Node::Method{args,..},"map")=>{let callback=args.first().and_then(Option::as_ref).ok_or_else(||fail("a readonly map needs its actual projection lambda"))?;let Node::Lambda{params,body}=&callback.node else{return Err(fail("a readonly map needs its actual capture-free one-parameter projection lambda"))};if args.len()!=1||params.len()!=1||!params[0].ty.same(&item.ty)||!self.callback_projection(body)?{return Err(fail("a readonly map needs its actual capture-free one-parameter projection lambda"));}let id=self.root(String::new(),false)?;let mut local=Environment::new();local.insert(params[0].name.clone(),Value::new(Some(item.clone()),Some(id),source.regions.clone(),false,None));let before=self.temporaries.len();self.closed_target(&item)?;let result=self.expression(body,&local,bindings,wanted.array_element_position.as_deref(),depth+1)?;let result=self.read(result,"move")?;self.states[id]=rules::expire_rust_borrowed_source_region(&self.states[id]);self.finish(result.clone(),before)?;output=result.position.ok_or_else(||fail("a readonly map needs its actual complete reference result"))?;regions=result.regions;},
  (Node::Append{right,..},"concat")=>{let value=self.expression(right,env,bindings,None,depth+1)?;let value=self.read(value,"read")?;let child=self.iterator_item(&value)?;if !same_owner(Some(&item),Some(&child))||item.wrappers!=child.wrappers{return Err(fail("readonly chain item source positions disagree"));}regions=self.union([regions,value.regions])?;},
  (Node::Method{args,..},"slice")=>{let kinds:Vec<String>=args.iter().map(|v|v.as_ref().map(|v|v.node.kind()).unwrap_or("").into()).collect();let types:Vec<String>=args.iter().map(|v|v.as_ref().map(|v|v.ty.kind()).unwrap_or("").into()).collect();let values:Vec<String>=args.iter().map(|v|match v.as_ref().map(|v|&v.node){Some(Node::Lit{value})=>value.text(),_=>String::new()}).collect();if !super::rust_borrowed_iterator_rules::accept_rust_borrowed_iterator_whole_slice(&kinds,&types,&values){return Err(fail("readonly collect materialization requires its exact whole-vector slice"));}},
  _=>return Err(fail("the readonly source method needs its registered common operation"))}
 let Type::Array{element,..}=&wanted.ty else{return Err(fail("a readonly method result needs its actual vector type"))};let child=wanted.array_element_position.as_deref().ok_or_else(||fail("a readonly method needs its actual child source position"))?;accept(super::rust_borrowed_iterator_rules::read_rust_borrowed_iterator_collection_error(output.ty.same(element),same_owner(Some(&output),Some(child)),output.wrappers.len()==1&&output.wrappers[0]=="reference",true))?;self.closed_target(&output)?;let root=self.root(String::new(),true)?;Ok(Value::new(Some(self.position(wanted)?),Some(root),regions,true,None))}
 fn expression(&mut self,node:&Expr,env:&Environment,bindings:&[String],expected:Option<&Position>,depth:usize)->Result<Value>{
     self.expression_in_context(node, env, bindings, expected, depth, false)
 }
 fn binding_value(&mut self,node:&Expr,env:&Environment,bindings:&[String],depth:usize)->Result<Value>{
     self.expression_in_context(node, env, bindings, None, depth, true)
 }
 fn expression_in_context(&mut self,node:&Expr,env:&Environment,bindings:&[String],expected:Option<&Position>,depth:usize,binding:bool)->Result<Value>{self.visits+=1;self.budget(depth)?;
     if let Some(facet)=node.source_state_transition.as_deref()
         && facet.operation == "initialize" && !binding
     {
         return Err(fail("initializing source storage requires its actual lexical binding and value"));
     }
     if matches!(&node.node, Node::Abort { .. }) {
         if node.source_position.is_some() || node.source_state_transition.is_some() || node.source_loop_fold.is_some() || node.source_record_place.is_some() || node.source_place_transform.is_some() {
             return Err(fail("a diverging source term cannot publish a value or storage position"));
         }
         return Err(RegionError::Diverged);
     }
     if let Some(facet) = node.source_place_transform.as_deref() {
         let value = self.place_transform(node, facet, env, bindings, depth)?;
         return self.compatible(value, expected);
     }
if let Some(facet)=node.source_loop_fold.as_deref(){return self.loop_expression(node,facet,env,bindings,expected,depth);}let mut operation=rules::read_rust_borrowed_source_operation(node.node.kind());if self.composed&&matches!(&node.node,Node::Tuple{..}|Node::TupleIndex{..}|Node::If{..}|Node::Append{..}){operation=node.node.kind().into();}if operation.is_empty(){operation=rules::read_rust_borrowed_source_match_operation(node.node.kind());}if operation.is_empty(){let op=match &node.node{Node::Method{op,..}=>op.as_str(),_=>""};operation=rules::read_rust_borrowed_source_iterator_operation(node.node.kind(),op);}if operation.is_empty(){return Err(fail("this checked expression needs its actual source binder operation adapter"));}let qualifier=node.source_position.as_deref().map(|v|&v.position);let transition=node.source_state_transition.as_deref();let position=qualifier.or_else(||transition.map(|facet|&facet.position));if let Some(p)=position{self.scope(p,bindings)?;}let mut value=if let Some(facet)=node.source_record_place.as_deref(){self.project_record_place(node,facet,env,bindings,depth)?}else{match &node.node{
 Node::Var{name}=>self.read(env.get(name).cloned().ok_or_else(||fail("the source variable needs its actual lexical binder"))?,"read")?,
 Node::Lit{..}|Node::Unit=>{if node.ty==Type::String{let p=self.position(&Position{ty:node.ty.clone(),wrappers:vec!["reference".into()],lifetimes:vec![Some("static".into())],owner:SourceOwner{kind:"primitive".into(),path:vec!["str".into()],arguments:Vec::new()},array_element_position:None})?;Value::new(Some(p),None,vec![0],false,None)}else{Value::new(None,None,Vec::new(),false,Some(self.visits))}},
 Node::Let{name,value,body}=>{let before=self.temporaries.len();let initial=self.binding_value(value,env,bindings,depth+1)?;let mut local=env.clone();let id=self.bind(&mut local,name,initial)?;self.finish(local[name].clone(),before)?;let body=self.expression(body,&local,bindings,expected,depth+1)?;let result=self.read(body,"move")?;self.expire_binding(id,&local[name]);self.read(result,"read")?},
 Node::If{cond,then,otherwise}=>self.conditional(cond,[then,otherwise],env,bindings,expected,depth)?,
 Node::Binary{op:op @ (BinaryOp::And|BinaryOp::Or),left,right,..}=>{
     let short_circuit=Expr::lit(Type::Bool,LitValue::Bool(*op==BinaryOp::Or));
     let alternatives:[&Expr;2]=if *op==BinaryOp::And{[right,&short_circuit]}else{[&short_circuit,right]};
     self.conditional(left,alternatives,env,bindings,expected,depth)?
 },
 Node::Binary{op:BinaryOp::Concat,left,right,..}
     if self.composed && position.or(expected).is_some_and(|p| role(p)=="string")=>{
     let wanted=position.or(expected).expect("checked owned String position");
     let current=self.expression(left,env,bindings,None,depth+1)?;
     let current=self.read(current,"read")?;
     let suffix=self.expression(right,env,bindings,None,depth+1)?;
     let suffix=self.read(suffix,"read")?;
     if !super::rust_exclusive_source_string_rules::accept_rust_source_owned_string_concatenation(
         &current.role,&observation_role(&suffix),
     ){
         return Err(fail("owned String concatenation retains its actual owned input and shared text operand"));
     }
     self.compatible(current,Some(wanted))?;
     self.closed_output(wanted)?;
     let position=self.position(wanted)?;
     let root=self.root(String::new(),true)?;
     Value::new(Some(position),Some(root),Vec::new(),true,None)
 },
 Node::Tuple{items,..}=>self.state_packet(node,items,env,bindings,depth)?,
 Node::TupleIndex{object,index}=>{
     let packet=self.expression(object,env,bindings,None,depth+1)?;
     let packet=self.read(packet,"read")?;
     self.read_packet_field(packet,*index,&node.ty)?
 },
 Node::Append{left,right} if self.composed && position.or(expected).is_some_and(|p| matches!(role(p).as_str(), "owned" | "string"))=>{
     let wanted=position.or(expected).expect("checked owned append position");
     for operand in [left, right] {
         let actual=self.expression(operand,env,bindings,Some(wanted),depth+1)?;
         self.read(actual,"move")?;
     }
     self.closed_target(wanted)?;
     let position=self.position(wanted)?;
     let root=self.root(String::new(),true)?;
     Value::new(Some(position),Some(root),Vec::new(),true,None)
 },
 Node::Method{..}|Node::Append{..}=>self.iterator_expression(node,env,bindings,expected,depth)?,
 Node::Print{text,body}=>{
     self.display(text,env,bindings,depth+1)?;
     self.expression(body,env,bindings,expected,depth+1)?
 },
 Node::Match{scrutinee,cases}=>{
  let source=self.expression(scrutinee,env,bindings,None,depth+1)?;
  let source=self.read(source,"read")?;
  if source.control.is_some() {
      self.loop_match(source,cases,env,bindings,expected,depth)?
  } else {
      self.nominal_match(source,cases,env,bindings,expected,depth)?
  }
 },
 Node::Array{items}=>{let wanted=position.or(expected).ok_or_else(||fail("an array source constructor needs its complete element position"))?;if !matches!(role(wanted).as_str(),"owned"|"borrowed vector"){return Err(fail("an array source constructor needs its complete element position"));}let element=match wanted.array_element_position.as_deref(){Some(p)=>self.position(p)?,None=>match &wanted.ty{Type::Array{element,..}=>self.position(&Position{ty:element.as_ref().clone(),wrappers:Vec::new(),lifetimes:Vec::new(),owner:wanted.owner.arguments.first().ok_or_else(||fail("a vector needs its actual element owner"))?.clone(),array_element_position:None})?,_=>return Err(fail("the source array needs its actual element type"))}};let mut values=Vec::new();for item in items{let v=self.expression(item,env,bindings,Some(&element),depth+1)?;let v=self.compatible(v,Some(&element))?;values.push(self.read(v,"move")?);}let root=self.root(String::new(),true)?;let regions=self.union(values.into_iter().map(|v|v.regions))?;Value::new(Some(self.position(wanted)?),Some(root),regions,true,None)},
 Node::Ctor{data,ctor,args,..}=>{if let Some(control)=self.loop_constructor(node,ctor,args,env,bindings,depth)?{if position.is_some(){return Err(fail("compiler loop control does not carry an independent source position"));}return self.compatible(control,expected);}let declaration=self.data(data).ok_or_else(||fail("a constructor needs its actual datatype"))?;let index=declaration.ctors.iter().position(|c|&c.name==ctor).ok_or_else(||fail("a constructor needs its actual declaration"))?;let fields=&declaration.source_field_signatures.as_deref().and_then(|f|f.ctors.get(index)).ok_or_else(||fail("a source constructor needs its actual complete field positions"))?.fields;if fields.len()!=args.len(){return Err(fail("a constructor needs its actual complete field positions"));}let mut values=Vec::new();for(arg,field)in args.iter().zip(fields){let v=self.expression(arg,env,bindings,Some(field),depth+1)?;let v=self.compatible(v,Some(field))?;values.push(self.read(v,"move")?);}let root=self.root(String::new(),true)?;let regions=self.union(values.into_iter().map(|v|v.regions))?;let p=self.position(&Position{ty:node.ty.clone(),wrappers:Vec::new(),lifetimes:Vec::new(),owner:SourceOwner{kind:"declaration".into(),path:data.split('.').map(str::to_owned).collect(),arguments:Vec::new()},array_element_position:None})?;Value::new(Some(p),Some(root),regions,true,None)},
 Node::Call{func,args}=>{let declaration=self.program.declarations.iter().find_map(|d|match d{Decl::Fn(f)if &f.full_name==func=>Some(f),_=>None}).ok_or_else(||fail("a source call needs its actual selected function interface"))?;if let Some(facet)=declaration.source_state_transform.as_ref(){return self.state_transform_call(node,declaration,facet,args,env,bindings,depth);}if let Some(facet)=declaration.source_vector_mutation.as_ref(){return self.vector_mutation_call(node,declaration,facet,args,env,bindings,depth);}let signature=declaration.source_signature.as_ref().filter(|s|(s.model==MODEL||self.composed&&s.model==CLOSED_MODEL)&&s.parameters.len()==args.len()).ok_or_else(||fail("a source call needs its actual complete selected function interface"))?;let mut values=Vec::new();for(arg,p)in args.iter().zip(&signature.parameters){let p=self.position(p)?;let mut parameter=p.as_ref().clone();instantiate_call_parameter(&mut parameter);let v=self.expression(arg,env,bindings,Some(&parameter),depth+1)?;let v=self.compatible(v,Some(&parameter))?;values.push(self.read(v,"move")?);}for actual in &values{let mut check=actual.clone();check.root=None;check.producer=true;self.read(check,"read")?;}let sources=Self::sources(&signature.result,&signature.parameters);if sources.contains(&-1.0){return Err(fail("a source call needs its actual parameter region interface"));}let regions=self.union(sources.iter().map(|index|values[*index as usize].regions.clone()))?;let borrow_roots=self.union(sources.iter().map(|index|values[*index as usize].borrow_roots.clone()))?;let root=self.root(String::new(),true)?;let mut result=Value::new(Some(self.position(&signature.result)?),Some(root),regions,true,None);result.borrow_roots=borrow_roots;result},
 Node::Length{array}=>{let v=self.expression(array,env,bindings,None,depth+1)?;let v=self.read(v,"read")?;accept(borrowed::read_rust_borrowed_vector_observation_error(v.role=="borrowed vector","len"))?;Value::new(None,None,Vec::new(),false,Some(self.visits))},
 _=>{for child in node.children(){let v=self.expression(child,env,bindings,None,depth+1)?;let v=self.read(v,"read")?;accept(borrowed::read_rust_borrowed_vector_observation_error(v.role=="borrowed vector",&operation))?;accept(rules::read_rust_borrowed_source_observation_error(&observation_role(&v),&operation))?;}Value::new(None,None,Vec::new(),false,Some(self.visits))}
 }};let p=position.map(|p|self.position(p)).transpose()?;value=self.adjust(value,p,node)?;if let Some(facet)=transition{value=self.apply_storage_transition(value,facet)?;}self.compatible(value,expected)}
 fn closed_output(&mut self, position:&Position)->Result<()> {
     if !position.lifetimes.is_empty()
         || position.array_element_position.is_some()
         || position.wrappers.iter().any(|layer| layer != "box")
         || role(position).is_empty()
     {
         return Err(fail("closed state calls retain complete owned positions without returned or stored references"));
     }
     let mut owner = &position.owner;
     for _ in &position.wrappers {
         if !Self::canonical_box_owner(owner) {
             return Err(fail("closed state calls retain every actual canonical owned Box layer"));
         }
         owner = &owner.arguments[0];
     }
     if Self::canonical_box_owner(owner) {
         return Err(fail("closed state calls retain the complete owned Box wrapper inventory"));
     }
     self.closed_target(position)
 }
 fn canonical_box_owner(owner: &super::source_signatures::SourceOwner) -> bool {
     super::rust_owned_recursive_datatype_rules::accept_rust_owned_box_type_node(
         source_policy_column(&["box".into()]),
         &owner.kind,
         &owner.path.join("::"),
         owner.arguments.len() as f64,
     )
 }
 fn function(&mut self,f:&FnDecl)->Result<()> {
     self.reset_scope();
     if let Some(facet) = f.source_state_transform.as_ref() {
         return self.state_transform_body(f, facet);
     }
     if let Some(facet) = f.source_vector_mutation.as_ref() {
         return self.vector_mutation_body(f, facet);
     }
     let signature=f.source_signature.as_ref();
     let Some(signature)=signature.filter(|signature|signature.model==MODEL) else {
         if self.contains(&f.body)? {
             return Err(fail("an expression source position needs its enclosing selected function interface"));
         }
         if let Some(closed)=signature.filter(|signature|self.composed&&signature.model==CLOSED_MODEL) {
             if closed.parameters.len()!=f.params.len()
                 || closed.parameters.len()>512
                 || closed.parameters.iter().zip(&f.params).any(|(position,parameter)|position.ty!=parameter.ty)
                 || closed.result.ty!=f.ret
             {
                 return Err(fail("the closed call retains its complete actual checked formal and result order"));
             }
             self.closed_output(&closed.result)?;
             for position in &closed.parameters {
                 self.closed_target(position)?;
             }
         }
         return Ok(());
     };
     let mut env=Environment::new();let mut parameters=Vec::new();for(param,p)in f.params.iter().zip(&signature.parameters){self.scope(p,&signature.lifetime_bindings)?;let r=role(p);if r.is_empty(){return Err(fail("a source parameter needs its separate value-domain proof"));}if r=="reference"{self.closed_target(p)?;}if r=="borrowed vector"{self.closed_target(p.array_element_position.as_deref().ok_or_else(||fail("a borrowed vector needs its actual element position"))?)?;}let regions=if matches!(r.as_str(),"reference"|"borrowed vector"){vec![self.root(lifetime(p),false)?]}else{Vec::new()};let root=self.root(String::new(),false)?;let p=self.position(p)?;let value=Value::new(Some(p),Some(root),regions,false,None);self.prepare_value_record_images(&value)?;env.insert(param.name.clone(),value.clone());parameters.push(value);}let value=self.expression(&f.body,&env,&signature.lifetime_bindings,Some(&signature.result),0)?;let value=self.read(value,"move")?;let sources=Self::sources(&signature.result,&signature.parameters);let allowed=self.union(sources.iter().filter(|i|**i>=0.0).map(|i|parameters[*i as usize].regions.clone()))?;let allowed=self.record_field_roots(allowed)?;let statics:Vec<f64>=self.lifetimes.iter().enumerate().filter(|(_,life)|life.as_str()=="static").map(|(i,_)|i as f64).collect();accept(rules::read_rust_borrowed_source_return_error(&value.regions.iter().map(|i|*i as f64).collect::<Vec<_>>(),&allowed.iter().map(|i|*i as f64).collect::<Vec<_>>(),&statics,!sources.contains(&-1.0)))}
 fn contains(&mut self,root:&Expr)->Result<bool>{
     let mut pending=vec![(root,0)];let mut selected=false;
     while let Some((node,depth))=pending.pop(){
         self.visits+=1;self.budget(depth)?;
         if node.source_state_transition.is_some()||node.source_loop_fold.is_some()||node.source_record_place.is_some()||node.source_place_transform.is_some(){self.composed=true;selected=true;}
         if node.source_position.is_some(){selected=true;}
         for child in node.children(){pending.push((child,depth+1));}
     }
     Ok(selected)
 }
 fn contains_prop(&mut self,root:&Prop)->Result<bool>{let mut pending=vec![(root,0)];while let Some((prop,depth))=pending.pop(){self.visits+=1;self.budget(depth)?;match prop{Prop::Forall{body,..}|Prop::Not{arg:body}=>pending.push((body,depth+1)),Prop::And{left,right}|Prop::Or{left,right}|Prop::Implies{left,right}=>{pending.push((left,depth+1));pending.push((right,depth+1));},Prop::Bool{expr}=>{if self.contains(expr)?{return Ok(true);}},_=>{if let Some((_,comparison))=prop.comparison(){if self.contains(&comparison.left)?||self.contains(&comparison.right)?{return Ok(true);}}}}}Ok(false)}
 fn display(&mut self, text: &Expr, env: &Environment, bindings: &[String], depth: usize) -> Result<()> {
     let before = self.temporaries.len();
     let value = self.expression(text, env, bindings, None, depth)?;
     let value = self.read(value, "read")?;
     if value.packet.is_some() || value.control.is_some() {
         return Err(fail("source state packets are projected by their actual slots before display"));
     }
     accept(borrowed::read_rust_borrowed_vector_observation_error(value.role == "borrowed vector", "display"))?;
     accept(rules::read_rust_borrowed_source_observation_error(&observation_role(&value), "display"))?;
     self.finish(value, before)?;
     Ok(())
 }
 fn main(&mut self, effects: &[Effect]) -> Result<()> {
     self.reset_scope();
     let mut env = Environment::new();
     for effect in effects {
         match effect {
             Effect::Let { name, value, .. } => {
                 let before = self.temporaries.len();
                 let value = self.binding_value(value, &env, &[], 0)?;
                 self.bind(&mut env, name, value)?;
                 self.finish(env[name].clone(), before)?;
             }
             Effect::Print { expr, .. } => self.display(expr, &env, &[], 0)?,
             _ => return Err(fail("this source effect needs its independent binder region adapter")),
         }
     }
     Ok(())
 }
 fn run(&mut self)->Result<()>{
     let program=self.program;
     for declaration in &program.declarations{
         match declaration{
             Decl::Fn(f) => match self.function(f) {
                 Ok(()) | Err(RegionError::Diverged) => {}
                 Err(error) => return Err(error),
             },
             Decl::Theorem(t) if self.contains_prop(&t.prop)? =>
                 return Err(fail("source positions in proof propositions need their independent binder adapter")),
             _=>{}
         }
     }
     if let Some(main)=&program.main{
         match self.main(&main.effects) {
             Ok(()) | Err(RegionError::Diverged) => {}
             Err(error) => return Err(error),
         }
     }
     Ok(())
 }
}
/// Reconstruct source loans from checked binder/value links, never from TYPE IDs.
pub(crate) fn validate_borrowed_vector_source_regions(program:&Program)->super::diagnostics::Result<()> {
    inspect_borrowed_vector_source_regions(program).map_err(RegionError::into_diagnostic)
}
fn inspect_borrowed_vector_source_regions(program:&Program)->Result<()>{let mut visitor=Regions::new(program);let mut selected=false;for declaration in &program.declarations{match declaration{Decl::Fn(f)=>{if f.source_signature.as_ref().is_some_and(|s|s.model==CLOSED_MODEL)||f.source_state_transform.is_some()||f.source_vector_mutation.is_some(){visitor.composed=true;}let body_selected=visitor.contains(&f.body)?;if f.source_signature.as_ref().is_some_and(|s|s.model==MODEL)||f.source_state_transform.is_some()||f.source_vector_mutation.is_some()||body_selected{selected=true;}},Decl::Theorem(t)=>{if visitor.contains_prop(&t.prop)?{selected=true;}},_=>{}}}if let Some(main)=&program.main{for effect in &main.effects{match effect{Effect::Let{value,..}|Effect::Print{expr:value,..}|Effect::Output{expr:value,..}|Effect::Unwrap{pair:value,..}=>{if visitor.contains(value)?{selected=true;}},Effect::Assert{prop,..}=>{if visitor.contains_prop(prop)?{selected=true;}}}}}if !selected{return Ok(());}accept(rules::read_rust_borrowed_source_runtime_error(selected,program.source_runtime_shell.is_some()))?;if program.source_language!=super::Language::Rust{return Err(fail("source binder regions require Rust source"));}visitor.discovery=false;visitor.visits=0;visitor.run()}

#[cfg(test)]mod source_controls{
 use super::*;
 use serde_json::{json,Value};
 fn fixture()->Value{
  let scalar=json!({"kind":"fixed","bits":8,"signed":false});let owner=json!({"kind":"primitive","path":["u8"],"arguments":[]});
  let element=json!({"type":scalar,"owner":owner,"wrappers":["reference"],"lifetimes":["a"]});
  let array=json!({"kind":"array","element":scalar,"reference":false});
  let result=json!({"type":array,"owner":{"kind":"library","path":["std","vec","Vec"],"arguments":[owner]},"wrappers":[],"lifetimes":[],"arrayElementPosition":element});
  json!({"schemaVersion":1,"sourceLanguage":"Rust","items":[],"main":{"effects":[]},"declarations":[{"k":"fn","name":"children","fullName":"children","modulePath":[],"params":[{"name":"value","type":scalar}],"ret":array,"recursive":false,"decreasing":null,"sourceSignature":{"model":MODEL,"language":"Rust","parameters":[element],"result":result,"lifetimeBindings":["a"]},"body":{"k":"array","type":array,"sourcePosition":{"model":"Rust borrowed expression source position 1","position":result},"items":[{"k":"var","name":"value","type":scalar,"sourcePosition":{"model":"Rust borrowed expression source position 1","position":element}}]}}]})
 }
 #[test]fn source_free_reload_uses_actual_binder(){let program:Program=serde_json::from_value(fixture()).expect("authored checked source fixture");validate_borrowed_vector_source_regions(&program).expect("actual parameter loan");}
 #[test]fn forged_static_body_cannot_upgrade_parameter(){let mut source=fixture();source["declarations"][0]["body"]["items"][0]["sourcePosition"]["position"]["lifetimes"]=json!(["static"]);let program:Program=serde_json::from_value(source).expect("authored source fixture");assert!(validate_borrowed_vector_source_regions(&program).is_err());}
 #[test]fn unknown_actual_binder_refuses(){let mut source=fixture();source["declarations"][0]["body"]["items"][0]["name"]=json!("unbound");let program:Program=serde_json::from_value(source).expect("authored source fixture");assert!(validate_borrowed_vector_source_regions(&program).is_err());}
 #[test]fn assertion_only_facet_does_not_bypass_detection(){let mut source=fixture();let expression=source["declarations"][0]["body"]["items"][0].clone();source["declarations"]=json!([]);source["main"]["effects"]=json!([{"k":"assert","prop":{"p":"bool","expr":expression}}]);let program:Program=serde_json::from_value(source).expect("authored source fixture");assert!(validate_borrowed_vector_source_regions(&program).is_err());}
 #[test]fn runtime_shell_conflict_refuses_before_body_proof(){let mut source=fixture();source["sourceRuntimeShell"]=json!({});let program:Program=serde_json::from_value(source).expect("authored source fixture");assert!(validate_borrowed_vector_source_regions(&program).is_err());}
 fn match_fixture()->Value{let mut source=fixture();let scalar=json!({"kind":"fixed","bits":8,"signed":false});let owner=json!({"kind":"primitive","path":["u8"],"arguments":[]});let field=json!({"type":scalar,"owner":owner,"wrappers":[],"lifetimes":[]});let data=json!({"kind":"data","name":"Node"});let parameter=json!({"type":data,"wrappers":["reference"],"lifetimes":["a"],"owner":{"kind":"declaration","path":["Node"],"arguments":[]}});source["declarations"][0]["params"][0]["type"]=data.clone();source["declarations"][0]["sourceSignature"]["parameters"]=json!([parameter]);let mut body=source["declarations"][0]["body"].clone();body["items"][0]["name"]=json!("child");source["declarations"][0]["body"]=json!({"k":"match","type":body["type"],"sourcePosition":body["sourcePosition"],"scrutinee":{"k":"var","name":"value","type":data,"sourcePosition":{"model":"Rust borrowed expression source position 1","position":parameter}},"cases":[{"pattern":{"k":"ctor","data":"Node","ctor":"Node","binds":["child"]},"body":body}]});source["declarations"].as_array_mut().expect("authored declarations").push(json!({"k":"data","name":"Node","fullName":"Node","modulePath":[],"ctors":[{"name":"Node","fields":[{"name":"value","type":scalar}]}],"sourceFieldSignatures":{"model":"Rust data type field signatures 1","language":"Rust","dataTypeName":"Node","ctors":[{"name":"Node","fields":[field]}]}}));source}
 #[test]fn borrowed_nominal_match_preserves_parent_region(){let program:Program=serde_json::from_value(match_fixture()).expect("authored source fixture");validate_borrowed_vector_source_regions(&program).expect("actual borrowed field region");}
 #[test]fn unknown_pattern_field_binder_cannot_create_loan(){let mut source=match_fixture();source["declarations"][0]["body"]["cases"][0]["body"]["items"][0]["name"]=json!("missing");let program:Program=serde_json::from_value(source).expect("authored source fixture");assert!(validate_borrowed_vector_source_regions(&program).is_err());}
 #[test]fn standard_box_projection_preserves_parameter_region(){let mut source=fixture();source["declarations"][0]["sourceSignature"]["parameters"][0]["wrappers"]=json!(["reference","box"]);source["declarations"][0]["sourceSignature"]["parameters"][0]["owner"]=json!({"kind":"library","path":["std","boxed","Box"],"arguments":[{"kind":"primitive","path":["u8"],"arguments":[]}]});let program:Program=serde_json::from_value(source).expect("authored source fixture");validate_borrowed_vector_source_regions(&program).expect("actual canonical Box parameter region");}
}

#[cfg(test)]
mod state_controls {
    use super::*;
    use serde_json::{Value, json};

    fn position() -> Value {
        json!({
            "type":{"kind":"fixed","bits":8,"signed":false},
            "owner":{"kind":"primitive","path":["u8"],"arguments":[]},
            "wrappers":[],
            "lifetimes":[]
        })
    }

    fn transition(operation: &str, numeral: &str) -> Value {
        let position = position();
        json!({
            "k":"lit","type":position["type"],"value":numeral,
            "sourceStateTransition":{
                "model":"Rust source state transition 1",
                "operation":operation,
                "binder":{"name":"count","identity":"actual count","start":18,"end":23},
                "position":position
            }
        })
    }

    fn binding(name: &str, value: Value) -> Value {
        json!({"k":"let","name":name,"value":value})
    }

    fn inspect(effects: Vec<Value>) -> super::super::diagnostics::Result<()> {
        let program = serde_json::from_value(json!({
            "schemaVersion":1,"sourceLanguage":"Rust","items":[],"declarations":[],
            "main":{"effects":effects}
        }))
        .expect("well-shaped checked source control");
        validate_borrowed_vector_source_regions(&program)
    }

    #[test]
    fn only_the_current_source_storage_revision_is_readable() {
        let initial = binding("count", transition("initialize", "1"));
        let write = binding("updated", transition("write", "2"));
        let print = |name: &str| {
            json!({"k":"print","expr":{"k":"var","name":name,"type":position()["type"]}})
        };
        inspect(vec![initial.clone(), write.clone(), print("updated")])
            .expect("the actual published state is available");
        let error = inspect(vec![initial, write, print("count")])
            .expect_err("the superseded lexical value cannot supply a current state read");
        assert!(error.reason.contains("actual live selected storage revision"));
    }

    #[test]
    fn a_forged_storage_binder_cannot_publish_a_write() {
        let mut write = transition("write", "2");
        write["sourceStateTransition"]["binder"]["identity"] = json!("other count");
        let error = inspect(vec![
            binding("count", transition("initialize", "1")),
            binding("updated", write),
        ])
        .expect_err("the write must name its actual initialized binder");
        assert!(error.reason.contains("actual initialized binder"));
    }

    #[test]
    fn a_diverging_alternative_cannot_poison_the_continuing_state() {
        let scalar = position()["type"].clone();
        let reselect = json!({
            "k":"if","type":scalar,
            "cond":{"k":"lit","type":{"kind":"bool"},"value":false},
            "then":{"k":"let","name":"discarded","type":scalar,
                "value":transition("write","2"),
                "body":{"k":"abort","type":scalar,"message":"halt"}},
            "else":{"k":"var","name":"count","type":scalar}
        });
        inspect(vec![
            binding("count", transition("initialize", "1")),
            binding("result", reselect),
            json!({"k":"print","expr":{"k":"var","name":"count","type":scalar}}),
        ])
        .expect("the original storage survives its continuing alternative");
    }
}
