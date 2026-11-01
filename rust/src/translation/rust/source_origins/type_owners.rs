//! Lexical catalog lookup with shared type-namespace owner admission.
use super::{SourceDeclaration, SourceLookup, SourceOrigins};
use crate::translation::rust_source_namespace_rules as namespace;

pub(super) fn namespace_kind(entry:&SourceDeclaration)->&'static str{
    match entry{SourceDeclaration::ExternalCrate=>"externalCrate",SourceDeclaration::Module=>"module",SourceDeclaration::Data=>"data",SourceDeclaration::Constructor {..}=>"ctor",SourceDeclaration::Function {..}=>"fn",SourceDeclaration::Constant {..}=>"const"}
}

#[cfg(test)]
mod deferred_controls{
    use super::*;
    use super::super::SourceOrigin;
    fn names(values:&[&str])->Vec<String>{values.iter().map(|value|(*value).to_owned()).collect()}
    #[test]
    fn independent_namespace_entries_survive_both_source_orders_and_passes(){
        for reverse in [false,true]{
            let mut first=SourceOrigins::new(None,"");
            let function=SourceDeclaration::Function {parameters:Vec::new(),result:SourceOrigin::value()};
            let entries=if reverse{vec![function,SourceDeclaration::Module]}else{vec![SourceDeclaration::Module,function]};
            for entry in entries{first.declaration("std",&[],entry).unwrap();}
            let mut next=SourceOrigins::new(Some(first.catalog),"");
            next.type_catalog=first.type_catalog;next.value_catalog=first.value_catalog;
            assert!(matches!(next.lookup_type_owner(&names(&["std"]),&[]).unwrap().declaration,SourceDeclaration::Module));
            assert!(matches!(next.lookup_value_owner(&names(&["std"]),&[]).unwrap().declaration,SourceDeclaration::Function {..}));
        }
    }
    #[test]
    fn nearer_type_prefix_stops_outer_child_and_data_filter(){
        let mut origins=SourceOrigins::new(None,"");
        origins.register_catalog_entry("X".into(),SourceDeclaration::Data);
        origins.register_catalog_entry("X.C".into(),SourceDeclaration::Constructor {owner:"X".into(),fields:Vec::new(),result:SourceOrigin::value()});
        origins.register_catalog_entry("inner.X".into(),SourceDeclaration::Module);
        assert!(origins.lookup_literal_match_owner(&names(&["X"]),&names(&["inner"])).is_none());
        assert!(origins.lookup_value_owner(&names(&["X","C"]),&names(&["inner"])).is_none());
        assert!(matches!(origins.lookup_type_owner(&names(&["X"]),&names(&["unrelated"])).unwrap().declaration,SourceDeclaration::Data));
    }
}
pub(super) fn namespace_role(entry:&SourceDeclaration)->String{namespace::read_rust_source_namespace_role(namespace_kind(entry))}

impl SourceOrigins {
    pub(in super::super) fn register_catalog_entry(&mut self,name:String,entry:SourceDeclaration){
        let role=namespace_role(&entry);
        if role=="type"{self.type_catalog.insert(name.clone(),entry.clone());}
        if role=="value"{self.value_catalog.insert(name.clone(),entry.clone());}
        self.catalog.insert(name,entry);
    }
    pub(in super::super) fn remove_catalog_entry(&mut self,name:&str){
        let Some(entry)=self.catalog.remove(name)else{return;};let role=namespace_role(&entry);
        if role=="type"{self.type_catalog.remove(name);}
        if role=="value"{self.value_catalog.remove(name);}
        let peer=if role=="type"{self.value_catalog.get(name)}else{self.type_catalog.get(name)};
        if let Some(peer)=peer{self.catalog.insert(name.to_owned(),peer.clone());}
    }
    pub(in super::super) fn lookup_namespace(&self,segments:&[String],path:&[String],role:&str)->Option<SourceLookup>{
        self.lookup_namespace_filtered(segments,path,role,None)
    }
    pub(in super::super) fn lookup_value_owner(&self,segments:&[String],path:&[String])->Option<SourceLookup>{self.lookup_namespace(segments,path,"value")}
    pub(in super::super) fn namespace_entry(&self,name:&str,role:&str)->Option<&SourceDeclaration>{
        let primary=self.catalog.get(name);
        let indexed=match role{"type"=>self.type_catalog.get(name),"value"=>self.value_catalog.get(name),_=>primary};
        primary.filter(|entry|role=="any"||namespace_role(entry)==role).or(indexed)
    }
    pub(in super::super) fn namespace_prefix_allows_outer(&self,segment_count:usize,prefix:&str)->bool{
        let kind=self.namespace_entry(prefix,"type").map_or("",namespace_kind);
        namespace::accept_rust_source_namespace_outer_candidate(segment_count as f64,kind)
    }
    pub(in super::super) fn lookup(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, false, None)
    }

    pub(in super::super) fn lookup_type_owner(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, true, None)
    }

    pub(in super::super) fn lookup_literal_match_owner(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_namespace_filtered(segments, path, "type", Some(crate::translation::rust_literal_match_rules::accept_rust_literal_match_catalog_owner))
    }

    fn lookup_filtered(
        &self,
        segments: &[String],
        path: &[String],
        type_owner: bool,
        catalog_owner: Option<fn(&str) -> bool>,
    ) -> Option<SourceLookup> {
        self.lookup_namespace_filtered(segments,path,if type_owner{"type"}else{"any"},catalog_owner)
    }

    fn lookup_namespace_filtered(&self,segments:&[String],path:&[String],role:&str,catalog_owner:Option<fn(&str)->bool>)->Option<SourceLookup>{
        let mut names = segments.to_vec();
        let mut start = path.to_vec();
        let mut absolute = false;
        match names.first().map(String::as_str) {
            Some("crate") => {
                names.remove(0);
                start.clear();
                absolute = true;
            }
            Some("self") => {
                names.remove(0);
                absolute = true;
            }
            Some("super") => {
                while names.first().is_some_and(|name| name == "super") {
                    names.remove(0);
                    start.pop();
                }
                absolute = true;
            }
            _ => {}
        }
        for depth in (0..=start.len()).rev() {
            let full_name = start[..depth]
                .iter()
                .chain(names.iter())
                .cloned()
                .collect::<Vec<_>>()
                .join(".");
            let found=self.namespace_entry(&full_name,role);
            if let Some(declaration)=found{
                let kind=namespace_kind(declaration);
                let action=namespace::read_rust_source_namespace_candidate(kind,role,&[]);
                if action=="select"{
                    if catalog_owner.is_none_or(|accept|accept(kind)){return Some(SourceLookup {declaration:declaration.clone(),full_name});}
                    return None;
                }
                if action=="stop"{return None;}
            }
            let prefix=start[..depth].iter().chain(names.first()).map(String::as_str).collect::<Vec<_>>().join(".");
            if !self.namespace_prefix_allows_outer(names.len(),&prefix){return None;}
            if absolute {
                break;
            }
        }
        None
    }
}
