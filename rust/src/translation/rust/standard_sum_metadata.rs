//! Borrowed transport of the shared semantic metadata cost projection.
//! No serde tree, source-origin clone or identity allocation precedes this walk.
use super::{SourceDeclaration,SourceOrigin,SourceOrigins,Type};
use crate::translation::source_signatures::{SourceOwner,SourceTypeSignature};
use crate::translation::rust_standard_sum_source_metadata_rules as rules;
use std::collections::HashSet;

#[derive(Clone,Copy,Default)]
pub(super) struct MetadataCost {pub(super) nodes:usize,pub(super) units:usize}
impl MetadataCost {pub(super) fn plus(self,other:Self)->Self{Self {nodes:self.nodes.saturating_add(other.nodes),units:self.units.saturating_add(other.units)}}}
#[derive(Default)]
pub(super) struct MetadataBudget {pub(super) nodes:usize,pub(super) units:usize,pub(super) error:Option<String>}
impl MetadataBudget {
    pub(super) fn reserve(&mut self,cost:MetadataCost)->Result<(),String>{
        if let Some(error)=&self.error{return Err(error.clone());}
        let next=rules::read_rust_standard_sum_source_metadata_reservation(self.nodes as f64,self.units as f64,cost.nodes as f64,cost.units as f64);
        if next.len()!=2{return Err("the complete standard sum source metadata exceeds its materialization budget".to_owned());}
        self.nodes=next[0] as usize;self.units=next[1] as usize;Ok(())
    }
}
struct Counter<'a>{cost:MetadataCost,origins:Option<&'a SourceOrigins>,active:HashSet<usize>}
impl Counter<'_>{
    fn add(&mut self,nodes:usize,units:usize,depth:usize)->Result<(),String>{
        self.cost.nodes=self.cost.nodes.saturating_add(nodes);self.cost.units=self.cost.units.saturating_add(units);
        let error=rules::read_rust_standard_sum_source_metadata_count_error(self.cost.nodes as f64,self.cost.units as f64,depth as f64);
        if error.is_empty(){Ok(())}else{Err(error)}
    }
    fn node(&mut self,depth:usize)->Result<(),String>{self.add(1,0,depth)}
    fn text(&mut self,value:&str,depth:usize)->Result<(),String>{self.add(1,value.encode_utf16().count(),depth)}
    fn optional_text(&mut self,value:Option<&str>,depth:usize)->Result<(),String>{match value{Some(value)=>self.text(value,depth),None=>self.node(depth)}}
    fn record(&mut self,form:&str,depth:usize)->Result<(),String>{let cost=rules::read_rust_standard_sum_source_metadata_record_cost(form);if cost.len()!=2{return Err("standard sum source metadata has an unsupported projection role".to_owned());}self.add(cost[0] as usize,cost[1] as usize,depth)}
    fn strings(&mut self,values:&[String],depth:usize)->Result<(),String>{self.node(depth)?;for value in values{self.text(value,depth+1)?;}Ok(())}
    fn owner(&mut self,value:Option<&SourceOwner>,depth:usize)->Result<(),String>{
        let Some(value)=value else{return self.node(depth);};self.record("owner",depth)?;self.text(&value.kind,depth+1)?;self.strings(&value.path,depth+1)?;self.node(depth+1)?;
        for child in &value.arguments{self.owner(Some(child),depth+2)?;}Ok(())
    }
    fn ty(&mut self,value:Option<&Type>,depth:usize,path:Option<&[String]>,header:bool)->Result<(),String>{
        let Some(value)=value else{return self.node(depth);};let form=rules::read_rust_standard_sum_source_metadata_type_form(value.kind());if form.is_empty(){return Err("standard sum source metadata has an unsupported type projection".to_owned());}
        self.record(&form,depth)?;self.text(value.kind(),depth+1)?;
        match value{
            Type::Fixed {..}=>{self.node(depth+1)?;self.node(depth+1)?;},
            Type::Named {path:names,..}=>{
                let actual=path.and_then(|path|self.origins.and_then(|origins|origins.standard_sum_metadata_lookup(names,path)));
                if let Some((name,SourceDeclaration::Data))=actual{
                    self.node(depth+1)?;self.text("crate",depth+2)?;
                    for part in name.split('.') {self.text(part,depth+2)?;}
                }else{self.strings(names,depth+1)?;}
            },
            Type::Data {name}=>self.text(name,depth+1)?,
            Type::Array {element,..}|Type::Set {element,..}=>{self.ty(Some(element),depth+1,None,false)?;self.node(depth+1)?;},
            Type::Cell {element}=>self.ty(Some(element),depth+1,None,false)?,
            Type::Option {element,absence}=>{self.ty(Some(element),depth+1,None,false)?;self.text(absence,depth+1)?;},
            Type::Absence {absence}=>self.text(absence,depth+1)?,
            Type::Tuple {elements}=>{self.node(depth+1)?;for child in elements{self.ty(Some(child),depth+2,path,header)?;}},
            Type::Union {alternatives}=>{self.node(depth+1)?;for child in alternatives{self.ty(Some(child),depth+2,None,false)?;}},
            Type::Function {params,ret}=>{self.node(depth+1)?;for child in params{self.ty(Some(child),depth+2,path,header)?;}self.ty(Some(ret),depth+1,path,header)?;},
            Type::Record {fields,..}=>{self.node(depth+1)?;for field in fields{self.record("record field",depth+2)?;self.text(&field.name,depth+3)?;self.ty(Some(&field.ty),depth+3,None,false)?;}self.node(depth+1)?;self.node(depth+1)?;self.node(depth+1)?;},
            Type::Map {key,value,..}=>{self.ty(Some(key),depth+1,None,false)?;self.ty(Some(value),depth+1,None,false)?;self.node(depth+1)?;},
            Type::Nat|Type::Int|Type::Bool|Type::String|Type::Unit|Type::Float|Type::Literal|Type::Hypothesis|Type::Output=>{},
        }Ok(())
    }
    fn source_position(&mut self,value:&SourceTypeSignature,depth:usize)->Result<(),String>{
        self.record("source position",depth)?;self.ty(Some(&value.ty),depth+1,None,false)?;self.strings(&value.wrappers,depth+1)?;
        self.node(depth+1)?;for lifetime in &value.lifetimes{self.optional_text(lifetime.as_deref(),depth+2)?;}self.owner(Some(&value.owner),depth+1)?;
        if let Some(child)=value.array_element_position.as_deref(){self.record("array element position",depth+1)?;self.source_position(child,depth+2)?;}Ok(())
    }
    fn origin(&mut self,value:&SourceOrigin,depth:usize)->Result<(),String>{
        self.record("origin",depth)?;self.ty(value.ty.as_ref(),depth+1,None,false)?;self.strings(&value.path,depth+1)?;self.strings(&value.wrappers,depth+1)?;
        self.owner(value.source_owner.as_ref(),depth+1)?;self.owner(value.expanded_source_owner.as_ref(),depth+1)?;
        self.optional_origins(value.fields.as_deref(),depth+1)?;self.optional_origins(value.callable_parameters.as_deref(),depth+1)?;
        if let Some(result)=value.callable_result.as_deref(){self.origin(result,depth+1)?;}else{self.node(depth+1)?;}
        self.optional_text(value.float_source_model,depth+1)?;self.optional_text(value.text_source_model,depth+1)?;self.optional_text(value.array_source_model,depth+1)?;
        self.node(depth+1)?;for lifetime in &value.source_lifetimes{self.optional_text(lifetime.as_deref(),depth+2)?;}
        self.strings(&value.namespace_spelling,depth+1)?;
        if let Some(sum)=value.standard_sum_source.as_deref(){self.record("sum",depth+1)?;self.text(&sum.owner,depth+2)?;self.node(depth+2)?;for child in &sum.arguments{self.origin(child,depth+3)?;}}else{self.node(depth+1)?;}
        if let Some(child)=value.array_element_position.as_deref(){self.record("array element position",depth+1)?;self.source_position(child,depth+2)?;}
        Ok(())
    }
    fn optional_origins(&mut self,values:Option<&[SourceOrigin]>,depth:usize)->Result<(),String>{if let Some(values)=values{self.node(depth)?;for value in values{self.origin(value,depth+1)?;}}else{self.node(depth)?;}Ok(())}
    fn identity(&mut self,value:&SourceOrigin,depth:usize,header:bool)->Result<(),String>{
        if value.ty.is_none()||!header&&(!value.proven||value.diverging){return self.node(depth);}
        let key=std::ptr::from_ref(value) as usize;if !self.active.insert(key){return Err("standard sum source metadata cannot contain a structural cycle".to_owned());}
        let result=(||{
            let origins=self.origins.expect("identity metadata requires the actual source registry");
            let stored=match value.ty.as_ref(){Some(Type::Named {path,..})=>origins.standard_sum_metadata_lookup(path,&value.path).and_then(|(name,declaration)|matches!(declaration,SourceDeclaration::Data).then(||origins.standard_sum_source_origins.get(&name)).flatten()),_=>None};
            if !header&&stored.is_some_and(|stored|stored.proven&&stored.wrappers.is_empty()&&(stored.copy_evidence.is_some()||origins.standard_sum_ownership_facts(stored).profile=="owned")){
                let sum=stored.and_then(|stored|stored.standard_sum_source.as_deref()).ok_or_else(||"standard sum source metadata needs its actual argument directory".to_owned())?;
                self.record("identity sum",depth)?;self.text(&sum.owner,depth+1)?;self.node(depth+1)?;for child in &sum.arguments{self.identity(child,depth+2,false)?;}self.strings(&value.wrappers,depth+1)?;return Ok(());
            }
            self.record(if header{"identity header"}else{"identity value"},depth)?;self.ty(value.ty.as_ref(),depth+1,Some(&value.path),header)?;
            if header{self.strings(&value.path,depth+1)?;}
            self.owner(value.expanded_source_owner.as_ref().or(value.source_owner.as_ref()),depth+1)?;self.strings(&value.wrappers,depth+1)?;
            if !header{if let Some(fields)=&value.fields{self.node(depth+1)?;for child in fields{self.identity(child,depth+2,false)?;}}else{self.node(depth+1)?;}self.optional_text(value.float_source_model,depth+1)?;self.optional_text(value.text_source_model,depth+1)?;}
            if let Some(child)=value.array_element_position.as_deref(){self.record("array element position",depth+1)?;self.source_position(child,depth+2)?;}
            Ok(())
        })();self.active.remove(&key);result
    }
}
pub(super) fn measure_origins(values:&[&SourceOrigin])->Result<MetadataCost,String>{let mut counter=Counter {cost:MetadataCost::default(),origins:None,active:HashSet::new()};counter.node(0)?;for value in values{counter.origin(value,1)?;}Ok(counter.cost)}
pub(super) fn measure_expression_type(value:&Type,path:&[String])->Result<MetadataCost,String>{let mut counter=Counter {cost:MetadataCost::default(),origins:None,active:HashSet::new()};counter.ty(Some(value),0,None,false)?;counter.strings(path,0)?;Ok(counter.cost)}
pub(super) fn measure_identities(origins:&SourceOrigins,values:&[&SourceOrigin],header:bool)->Result<MetadataCost,String>{let mut counter=Counter {cost:MetadataCost::default(),origins:Some(origins),active:HashSet::new()};counter.node(0)?;for value in values{counter.identity(value,1,header)?;}Ok(counter.cost)}

impl SourceOrigins{
    // Borrow the same lexical catalog entry as ordinary lookup, without cloning
    // a function/constructor's potentially large source-origin metadata.
    pub(super) fn standard_sum_metadata_lookup(&self,segments:&[String],path:&[String])->Option<(String,&SourceDeclaration)>{
        let mut names=segments;let mut start=path;let mut absolute=false;
        match names.first().map(String::as_str){Some("crate")=>{names=&names[1..];start=&[];absolute=true;},Some("self")=>{names=&names[1..];absolute=true;},Some("super")=>{while names.first().is_some_and(|name|name=="super"){names=&names[1..];start=&start[..start.len().saturating_sub(1)];}absolute=true;},_=>{}}
        for depth in (0..=start.len()).rev(){
            let name=start[..depth].iter().chain(names).map(String::as_str).collect::<Vec<_>>().join(".");
            let value=self.namespace_entry(&name,"type");
            if let Some(value)=value{return Some((name,value));}
            let prefix=start[..depth].iter().chain(names.first()).map(String::as_str).collect::<Vec<_>>().join(".");
            if !self.namespace_prefix_allows_outer(names.len(),&prefix){return None;}
            if absolute{break;}
        }
        None
    }
}

#[cfg(test)]
mod deferred_controls {
    use super::*;
    fn unit()->SourceOrigin{
        let mut value=SourceOrigin::with_type(Type::Unit,&[],Vec::new());
        value.source_owner=Some(SourceOwner {kind:"unit".into(),path:Vec::new(),arguments:Vec::new()});value
    }
    #[test]
    fn independently_authored_semantic_projection_costs(){
        let mut value=unit();let cost=measure_origins(&[&value]).unwrap();assert_eq!((cost.nodes,cost.units),(20,203));
        value.path=vec!["𐐀".into()];let cost=measure_origins(&[&value]).unwrap();assert_eq!((cost.nodes,cost.units),(21,205));
        value.path.clear();value.expanded_source_owner=value.source_owner.clone();
        let cost=measure_origins(&[&value]).unwrap();assert_eq!((cost.nodes,cost.units),(23,224));
    }
    #[test]
    fn copy_reservation_and_depth_boundaries(){
        for (nodes,units,accepted) in [(65536,0,true),(65537,0,false),(0,524288,true),(0,524289,false)]{
            assert_eq!(MetadataBudget::default().reserve(MetadataCost {nodes,units}).is_ok(),accepted);
        }
        let mut budget=MetadataBudget {nodes:1048560,units:0,error:None};
        assert!(budget.reserve(MetadataCost {nodes:1,units:0}).is_ok());
        assert_eq!(budget.nodes,1048576);
        assert!(budget.reserve(MetadataCost {nodes:1,units:0}).is_err());
        assert!(rules::read_rust_standard_sum_source_metadata_count_error(1.0,0.0,256.0).is_empty());
        assert!(!rules::read_rust_standard_sum_source_metadata_count_error(1.0,0.0,257.0).is_empty());
    }
}
