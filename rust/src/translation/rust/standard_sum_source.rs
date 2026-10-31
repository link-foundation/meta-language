//! Actual token/catalog/origin transport into source-generated standard sum rules.
//! No independent native Option/Result evaluation or matching policy lives here.
use std::collections::{HashMap, HashSet};

use super::{Alias, Options, Result, RustParser, SCtor, SData, SExpr, SField, SItem,
    SNode, SPattern, SPatternNode, SProgram, SourceDeclaration, SourceOrigin, SourceOrigins,
    Span, Token, Type, unsupported, type_error};
use crate::translation::rust_standard_sum_source_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use super::standard_sum_metadata::{measure_origins,measure_identities};

pub(super) const DEFERRED_PATTERN: &str = "#rust-standard-sum-source";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct StandardSumSource {
    pub(super) owner: String,
    pub(super) arguments: Vec<SourceOrigin>,
    pub(super) type_path: Vec<String>,
    pub(super) constructor_paths: HashMap<String, Vec<String>>,
    /// Source encounter order, retained across hash-catalog parser passes.
    pub(super) ordinal: usize,
    pub(super) provisional: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum StandardSumIdentity {
    /// Private header syntax identity; never an admitted type equivalence.
    Header { ty:Type, path:Vec<String>, owner:Option<SourceOwner>, wrappers:Vec<String> },
    Sum { owner: String, arguments: Vec<Self>, wrappers: Vec<String> },
    Value { ty: Type, owner: Option<SourceOwner>, wrappers: Vec<String>,
        fields: Option<Vec<Self>>, float: Option<&'static str>, text: Option<&'static str> },
}

#[derive(Default)]
pub(super) struct StandardSumSourceState {
    pub(super) expected_origin: Option<SourceOrigin>,
    pub(super) return_origin: Option<SourceOrigin>,
    pub(super) depth: usize,
    initialized: bool,
    rows: Vec<(String, SourceOrigin)>,
    upgrade_name: Option<String>,
}

fn count(value: usize) -> f64 { u32::try_from(value).map_or(f64::INFINITY, f64::from) }
fn failure(message: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust standard sum source", message, span)
}

impl SourceOrigins {
    fn actual_standard_sum_depth(&self,origin:&SourceOrigin,active:&mut HashSet<String>,depth:usize)->f64{
        if depth>256 {return -1.0;}
        let entry=match origin.ty.as_ref(){Some(Type::Named {path,..})=>self.standard_sum_metadata_lookup(path,&origin.path),_=>None};
        if let Some((name,_))=entry {
            if let Some(sum)=self.standard_sum_source_origins.get(&name).and_then(|stored|stored.standard_sum_source.as_ref()){
                if !active.insert(name.clone()){return -1.0;}
                let children=sum.arguments.iter().map(|argument|self.actual_standard_sum_depth(argument,active,depth+1)).collect::<Vec<_>>();
                active.remove(&name);return rules::read_rust_standard_sum_source_depth(&children,true);
            }
        }
        let children=origin.fields.iter().flatten().chain(origin.callable_parameters.iter().flatten()).chain(origin.callable_result.iter().map(Box::as_ref))
            .map(|child|self.actual_standard_sum_depth(child,active,depth+1)).collect::<Vec<_>>();
        rules::read_rust_standard_sum_source_depth(&children,false)
    }
    pub(super) fn standard_sum_source_origin(&self, origin: &SourceOrigin) -> Option<SourceOrigin> {
        if !origin.proven { return None; }
        let Type::Named { path, .. } = origin.ty.as_ref()? else { return None; };
        let (name,declaration) = self.standard_sum_metadata_lookup(path, &origin.path)?;
        if !matches!(declaration, SourceDeclaration::Data) { return None; }
        let stored = self.standard_sum_source_origins.get(&name)?;
        if !stored.proven || stored.copy_evidence.is_none() || !stored.wrappers.is_empty() { return None; }
        measure_origins(&[stored]).ok()?;
        let mut stored=stored.clone();
        stored.wrappers = origin.wrappers.clone();
        Some(stored)
    }

    fn standard_sum_qualified_type(&self, ty: &Type, path: &[String]) -> Option<Type> {
        Some(match ty {
            Type::Named { path: names, .. } => {
                let (name,declaration) = self.standard_sum_metadata_lookup(names,path)?;
                if !matches!(declaration, SourceDeclaration::Data) { return None; }
                let mut rooted = vec!["crate".to_owned()];
                rooted.extend(name.split('.').map(str::to_owned));
                Type::Named { path: rooted, span: None }
            }
            Type::Tuple { elements } => Type::Tuple {
                elements: elements.iter().map(|ty| self.standard_sum_qualified_type(ty,path)).collect::<Option<Vec<_>>>()?,
            },
            Type::Function { params, ret } => Type::Function {
                params: params.iter().map(|ty| self.standard_sum_qualified_type(ty,path)).collect::<Option<Vec<_>>>()?,
                ret: Box::new(self.standard_sum_qualified_type(ret,path)?),
            },
            _ => ty.clone(),
        })
    }

    fn standard_sum_qualified_origin(&self, origin: &SourceOrigin) -> Option<SourceOrigin> {
        let mut result = origin.clone();
        result.fields = match result.fields.as_ref() {
            Some(fields)=>Some(fields.iter().map(|field|self.standard_sum_qualified_origin(field)).collect::<Option<Vec<_>>>()?),
            None=>None,
        };
        if result.ty.is_none() {
            let fields = result.fields.as_ref()?;
            let owners = fields.iter().map(|field| field.expanded_source_owner.clone().or_else(||field.source_owner.clone())).collect::<Option<Vec<_>>>()?;
            result.ty = Some(Type::Tuple { elements: fields.iter().map(|field|field.ty.clone()).collect::<Option<Vec<_>>>()? });
            result.source_owner = Some(SourceOwner { kind:"tuple".into(), path:Vec::new(), arguments:owners });
        }
        result.ty = Some(self.standard_sum_qualified_type(result.ty.as_ref()?, &result.path)?);
        self.retain_nominal_copy_origin(&mut result);
        Some(result)
    }

    fn standard_sum_header_type(&self,ty:&Type,path:&[String])->Type{
        match ty {
            Type::Named {path:names,..}=>self.standard_sum_qualified_type(ty,path).unwrap_or_else(||Type::Named {path:names.clone(),span:None}),
            Type::Tuple {elements}=>Type::Tuple {elements:elements.iter().map(|ty|self.standard_sum_header_type(ty,path)).collect()},
            Type::Function {params,ret}=>Type::Function {params:params.iter().map(|ty|self.standard_sum_header_type(ty,path)).collect(),ret:Box::new(self.standard_sum_header_type(ret,path))},
            _=>ty.clone(),
        }
    }

    fn standard_sum_header_identity(&self,origin:&SourceOrigin)->Option<StandardSumIdentity>{
        measure_origins(&[origin]).ok()?;measure_identities(self,&[origin],true).ok()?;
        let ty=self.standard_sum_header_type(origin.ty.as_ref()?,&origin.path);
        Some(StandardSumIdentity::Header {ty,path:origin.path.clone(),owner:origin.expanded_source_owner.clone().or_else(||origin.source_owner.clone()),wrappers:origin.wrappers.clone()})
    }

    pub(super) fn standard_sum_source_identity(&self, origin: &SourceOrigin) -> Option<StandardSumIdentity> {
        measure_origins(&[origin]).ok()?;measure_identities(self,&[origin],false).ok()?;
        self.standard_sum_source_identity_project(origin)
    }
    fn standard_sum_source_identity_project(&self, origin: &SourceOrigin) -> Option<StandardSumIdentity> {
        if !origin.proven || origin.diverging { return None; }
        let stored=match origin.ty.as_ref(){Some(Type::Named {path,..})=>self.standard_sum_metadata_lookup(path,&origin.path).and_then(|(name,declaration)|matches!(declaration,SourceDeclaration::Data).then(||self.standard_sum_source_origins.get(&name)).flatten()),_=>None};
        if let Some(actual) = stored.filter(|actual|actual.proven&&actual.copy_evidence.is_some()&&actual.wrappers.is_empty()) {
            let sum = actual.standard_sum_source.as_deref()?;
            return Some(StandardSumIdentity::Sum { owner:sum.owner.clone(),
                arguments:sum.arguments.iter().map(|child|self.standard_sum_source_identity_project(child)).collect::<Option<Vec<_>>>()?,
                wrappers:origin.wrappers.clone() });
        }
        let ty = self.standard_sum_qualified_type(origin.ty.as_ref()?, &origin.path)?;
        let fields = match origin.fields.as_ref() {
            Some(fields) => Some(fields.iter().map(|field| self.standard_sum_source_identity_project(field)).collect::<Option<Vec<_>>>()?),
            None => None,
        };
        Some(StandardSumIdentity::Value { ty, owner:origin.expanded_source_owner.clone().or_else(||origin.source_owner.clone()),
            wrappers:origin.wrappers.clone(), fields, float:origin.float_source_model, text:origin.text_source_model })
    }

    pub(super) fn resolve_standard_sum_source_pattern(&mut self, pattern: &mut SPattern, origin: &SourceOrigin) -> Result<()> {
        let SPatternNode::Ctor { path, args } = &pattern.node else { return Ok(()); };
        if path.first().map(String::as_str) != Some(DEFERRED_PATTERN) { return Ok(()); }
        let Some(actual) = self.standard_sum_source_origin(origin) else {
            if self.collecting { self.standard_sum_source_needs_final_admission = true; return Ok(()); }
            return Err(failure("the constructor pattern needs its actual complete scrutinee source owner", pattern.span));
        };
        let sum = actual.standard_sum_source.as_deref().expect("actual standard sum");
        let family = path.get(1).map(String::as_str).unwrap_or("");
        let variant = path.get(2).map(String::as_str).unwrap_or("");
        let arity = rules::read_rust_standard_sum_source_constructor_arity(&sum.owner, variant);
        if !origin.wrappers.is_empty() || sum.owner != family || arity < 0.0 || arity != count(args.len()) {
            return Err(failure("the pattern must retain its actual direct family and exact fields", pattern.span));
        }
        let target = sum.constructor_paths.get(variant).cloned().ok_or_else(||failure("the actual constructor is absent",pattern.span))?;
        let args = args.clone();
        pattern.node = SPatternNode::Ctor { path:target, args };
        Ok(())
    }
}

impl RustParser {
    pub(super) fn initialize_standard_sum_source(&mut self)->Result<()> {
        if let Some(error)=self.origins.standard_sum_metadata_budget.error.as_deref(){return Err(failure(error,None));}
        if self.standard_sum_source.initialized { return Ok(()); }
        let mut rows = self.origins.standard_sum_source_origins.iter()
            .map(|(name,origin)|(name.clone(),origin.clone())).collect::<Vec<_>>();
        rows.sort_by_key(|(_,origin)| origin.standard_sum_source.as_ref().map_or(usize::MAX,|sum|sum.ordinal));
        for (name,_) in &rows { self.reserved_binding_names.insert(name.clone()); }
        self.standard_sum_source.rows = rows;
        self.standard_sum_source.initialized = true;
        Ok(())
    }

    fn standard_sum_range(&self, start:&Token) -> Option<Span> {
        Some(Span { start:start.start, end:self.cursor.tokens[self.cursor.index.saturating_sub(1)].end })
    }

    fn standard_sum_resolved_owner(&self, segments:&[String], path:&[String], value_namespace:bool) -> Option<(String,String)> {
        let resolved = self.resolve(segments,path);
        let selected = rules::read_rust_standard_sum_source_path_owner(&resolved.join("::"));
        if selected.is_empty() { return None; }
        for names in [segments, resolved.as_slice()] {
            if self.origins.lookup(names,path).is_some() || names.first().is_some_and(|root|self.origins.lookup(std::slice::from_ref(root),path).is_some()) {
                return None;
            }
            if value_namespace && names.first().is_some_and(|root|self.origins.scope.contains_key(root)) { return None; }
            if let Some(root)=names.first() {
                let keys=crate::translation::rust_type_alias_rules::read_rust_type_alias_lookup_keys(std::slice::from_ref(root),path);
                if keys.iter().any(|key|self.source_type_aliases.declarations.contains_key(key)) { return None; }
            }
        }
        let owner = rules::read_rust_standard_sum_source_owner(&selected,true,0.0);
        if owner.is_empty() { return None; }
        let variant = rules::read_rust_standard_sum_source_path_constructor(&resolved.join("::"));
        Some((owner,variant))
    }

    pub(super) fn resolve_standard_sum_source_owner(&self,segments:&[String],path:&[String]) -> String {
        self.standard_sum_resolved_owner(segments,path,false).map_or_else(String::new,|(owner,_)|owner)
    }

    pub(super) fn read_standard_sum_source_import(&self,segments:&[String],path:&[String]) -> bool {
        self.ordinary_completion_context() && segments.first().is_some_and(|root|root=="std"||root=="core")
            && self.standard_sum_resolved_owner(segments,path,false).is_some()
    }

    pub(super) fn instantiate_standard_sum_source(&mut self,owner:&str,arguments:Vec<SourceOrigin>,span:Option<Span>) -> Result<SourceOrigin> {
        self.initialize_standard_sum_source()?;
        let provisional=rules::read_rust_standard_sum_source_phase(self.header_discovery,false,false)=="provisional";
        if count(arguments.len())!=rules::read_rust_standard_sum_source_argument_count(owner){return Err(failure("the source sum needs every declared type argument",span));}
        let references=arguments.iter().collect::<Vec<_>>();
        let cost=measure_origins(&references).and_then(|raw|measure_identities(&self.origins,&references,provisional).map(|identity|raw.plus(identity))).map_err(|error|failure(&error,span))?;
        self.origins.standard_sum_metadata_budget.reserve(cost).map_err(|error|failure(&error,span))?;
        let depths=arguments.iter().map(|argument|self.origins.actual_standard_sum_depth(argument,&mut HashSet::new(),0)).collect::<Vec<_>>();
        if rules::read_rust_standard_sum_source_depth(&depths,true)<0.0{return Err(failure("the complete concrete sum exceeds its shared nested depth bound",span));}
        let arguments = if provisional { arguments } else { arguments.iter().map(|argument|self.origins.standard_sum_qualified_origin(argument)).collect::<Option<Vec<_>>>()
            .ok_or_else(||failure("every child needs its actual resolved source type",span))? };
        let identities = arguments.iter().map(|argument|if provisional {self.origins.standard_sum_header_identity(argument)}else{self.origins.standard_sum_source_identity(argument)}).collect::<Option<Vec<_>>>()
            .ok_or_else(||failure("every child needs complete checked source identity",span))?;
        // These labels project the already established typed identities into the
        // shared nonempty-identity admission gate; equality uses the structures.
        let labels = identities.iter().map(|_|"complete checked source identity".to_owned()).collect::<Vec<_>>();
        let copies = arguments.iter().map(SourceOrigin::direct_copy_value).collect::<Vec<_>>();
        if !provisional&&!rules::accept_rust_standard_sum_source_arguments(owner,&labels,&copies) {
            return Err(failure("every child needs a direct immutable Copy source type",span));
        }
        for (_,origin) in &self.standard_sum_source.rows {
            let sum = origin.standard_sum_source.as_ref().expect("registered sum");
            if sum.provisional==provisional&&sum.owner==owner && sum.arguments.iter().map(|child|if provisional{self.origins.standard_sum_header_identity(child)}else{self.origins.standard_sum_source_identity(child)}).collect::<Option<Vec<_>>>().as_ref()==Some(&identities) {
                return Ok(origin.clone());
            }
        }
        if self.standard_sum_source.upgrade_name.is_none()&&count(self.standard_sum_source.rows.len())>=rules::read_rust_standard_sum_source_instantiation_limit() {
            return Err(failure("the concrete standard sum inventory exceeds its shared bound",span));
        }
        let mut collision=0usize;
        let name=if let Some(name)=self.standard_sum_source.upgrade_name.clone(){name}else{loop {
            let name=rules::read_rust_standard_sum_source_specialization_name(count(self.standard_sum_source.rows.len()),count(collision));
            if name.is_empty() { return Err(failure("standard sum specialization name collisions exceed their shared bound",span)); }
            if !self.reserved_binding_names.contains(&name)&&!self.origins.catalog.contains_key(&name) { break name; }
            collision+=1;
        }};
        self.reserved_binding_names.insert(name.clone());
        let type_path=vec!["crate".into(),name.clone()];
        let constructors=rules::read_rust_standard_sum_source_constructors(owner);
        let mut constructor_paths=HashMap::new();
        for constructor in &constructors { let mut path=type_path.clone();path.push(constructor.clone());constructor_paths.insert(constructor.clone(),path); }
        let owners=arguments.iter().map(|argument|argument.expanded_source_owner.clone().or_else(||argument.source_owner.clone())).collect::<Option<Vec<_>>>()
            .ok_or_else(||failure("every child must retain its actual source owner",span))?;
        let source_owner=SourceOwner { kind:"library".into(),path:owner.split("::").map(str::to_owned).collect(),arguments:owners };
        let mut origin=SourceOrigin::with_type(Type::Named { path:type_path.clone(),span:None },&[],Vec::new());
        origin.proven=!provisional;origin.copy_evidence=(!provisional).then_some(());origin.source_owner=Some(source_owner.clone());origin.expanded_source_owner=Some(source_owner);
        let ordinal=self.standard_sum_source.rows.iter().find(|(existing,_)|existing==&name).and_then(|(_,origin)|origin.standard_sum_source.as_ref()).map_or(self.standard_sum_source.rows.len(),|sum|sum.ordinal);
        origin.standard_sum_source=Some(Box::new(StandardSumSource { owner:owner.into(),arguments:arguments.clone(),type_path,constructor_paths,ordinal,provisional }));
        self.origins.catalog.insert(name.clone(),SourceDeclaration::Data);
        self.origins.standard_sum_source_origins.insert(name.clone(),origin.clone());
        for constructor in constructors {
            let argument=rules::read_rust_standard_sum_source_field_argument(owner,&constructor);
            let fields=if argument<0.0 { Vec::new() } else { vec![arguments[argument as usize].clone()] };
            self.origins.catalog.insert(format!("{name}.{constructor}"),SourceDeclaration::Constructor { owner:name.clone(),fields,result:origin.clone() });
        }
        if let Some(row)=self.standard_sum_source.rows.iter_mut().find(|(existing,_)|existing==&name){row.1=origin.clone();}else{self.standard_sum_source.rows.push((name,origin.clone()));}
        Ok(origin)
    }

    fn refresh_standard_sum_header_argument(&mut self,origin:&SourceOrigin,rows:&[(String,SourceOrigin)],resolved:&mut HashMap<String,SourceOrigin>,active:&mut HashSet<String>)->Result<SourceOrigin>{
        let entry=match origin.ty.as_ref(){Some(Type::Named {path,..})=>self.origins.lookup(path,&origin.path),_=>None};
        if let Some(entry)=entry {
            if self.origins.standard_sum_source_origins.contains_key(&entry.full_name){
                let mut actual=self.resolve_standard_sum_header_row(&entry.full_name,rows,resolved,active)?;
                actual.wrappers=origin.wrappers.clone();return Ok(actual);
            }
        }
        let mut argument=origin.clone();
        if let Some(fields)=argument.fields.as_mut(){for field in fields { *field=self.refresh_standard_sum_header_argument(field,rows,resolved,active)?; }}
        if let Some(parameters)=argument.callable_parameters.as_mut(){for parameter in parameters {*parameter=self.refresh_standard_sum_header_argument(parameter,rows,resolved,active)?;}}
        if let Some(result)=argument.callable_result.as_mut(){**result=self.refresh_standard_sum_header_argument(result,rows,resolved,active)?;}
        let mut argument=self.origins.standard_sum_qualified_origin(&argument).ok_or_else(||failure("the complete header child must resolve its actual source datatype",None))?;
        if let (Some(Type::Tuple {..}),Some(fields))=(argument.ty.as_ref(),argument.fields.as_ref()){
            argument.ty=Some(Type::Tuple {elements:fields.iter().map(|field|field.ty.clone()).collect::<Option<Vec<_>>>().ok_or_else(||failure("the actual tuple child has an unresolved type",None))?});
        }
        if let (Some(Type::Function {..}),Some(parameters),Some(result))=(argument.ty.as_ref(),argument.callable_parameters.as_ref(),argument.callable_result.as_ref()){
            argument.ty=Some(Type::Function {params:parameters.iter().map(|parameter|parameter.ty.clone()).collect::<Option<Vec<_>>>().ok_or_else(||failure("the actual pointer parameter is unresolved",None))?,ret:Box::new(result.ty.clone().ok_or_else(||failure("the actual pointer result is unresolved",None))?)});
        }
        if argument.source_function_pointer_header{
            let copies=argument.callable_parameters.as_deref().unwrap_or_default().iter().map(SourceOrigin::direct_copy_value).collect::<Vec<_>>();
            let reason=crate::translation::rust_function_pointer_rules::read_rust_function_pointer_type_error(self.ordinary_completion_context(),&copies,argument.callable_result.as_ref().is_some_and(|result|result.direct_copy_value()));
            if !reason.is_empty(){return Err(failure(&reason,None));}
            argument.source_function_pointer_header=false;argument.proven=true;argument.copy_evidence=Some(());
        }
        if let Some(Type::Named {path,..})=argument.ty.as_ref(){
            if let Some(owner)=self.origins.lookup(path,&argument.path){
                argument.proven=self.origins.nominal_copy_owners.contains(&owner.full_name);
                if let Some(source)=argument.source_owner.as_mut().filter(|source|source.kind=="declaration"){source.path=owner.full_name.split('.').map(str::to_owned).collect();}
                if let Some(source)=argument.expanded_source_owner.as_mut().filter(|source|source.kind=="declaration"){source.path=owner.full_name.split('.').map(str::to_owned).collect();}
            }
        }
        let children=match argument.ty.as_ref(){
            Some(Type::Tuple {..})=>argument.fields.clone(),
            Some(Type::Function {..})=>argument.callable_parameters.clone().zip(argument.callable_result.clone()).map(|(mut parameters,result)|{parameters.push(*result);parameters}),
            _=>None,
        };
        if let Some(children)=children{
            let owners=children.iter().map(|child|child.expanded_source_owner.clone().or_else(||child.source_owner.clone())).collect::<Option<Vec<_>>>().ok_or_else(||failure("the complete compound child must retain every source owner",None))?;
            let kind=argument.ty.as_ref().expect("qualified type").kind();
            if let Some(source)=argument.source_owner.as_mut().filter(|source|source.kind==kind){source.arguments=owners.clone();}
            if let Some(source)=argument.expanded_source_owner.as_mut().filter(|source|source.kind==kind){source.arguments=owners;}
        }
        Ok(argument)
    }

    fn resolve_standard_sum_header_row(&mut self,name:&str,rows:&[(String,SourceOrigin)],resolved:&mut HashMap<String,SourceOrigin>,active:&mut HashSet<String>)->Result<SourceOrigin>{
        if let Some(origin)=resolved.get(name){return Ok(origin.clone());}
        if !active.insert(name.to_owned()){return Err(failure("the private header sum inventory is cyclic",None));}
        let sum=rows.iter().find(|(key,_)|key==name).and_then(|(_,origin)|origin.standard_sum_source.clone()).ok_or_else(||failure("the private header sum has no source inventory row",None))?;
        let mut arguments=Vec::new();for argument in &sum.arguments {arguments.push(self.refresh_standard_sum_header_argument(argument,rows,resolved,active)?);}
        let previous=self.standard_sum_source.upgrade_name.replace(name.to_owned());
        let actual=self.instantiate_standard_sum_source(&sum.owner,arguments,None);
        self.standard_sum_source.upgrade_name=previous;
        let actual=actual?;active.remove(name);resolved.insert(name.to_owned(),actual.clone());Ok(actual)
    }

    fn rewrite_standard_sum_header_type(&self,ty:&Type,resolved:&HashMap<String,SourceOrigin>,path:&[String])->Type{
        match ty {
            Type::Named {path:names,..}=>self.origins.lookup(names,path).and_then(|entry|resolved.get(&entry.full_name)).and_then(|origin|origin.ty.clone()).unwrap_or_else(||ty.clone()),
            Type::Tuple {elements}=>Type::Tuple {elements:elements.iter().map(|ty|self.rewrite_standard_sum_header_type(ty,resolved,path)).collect()},
            Type::Array {element,reference}=>Type::Array {element:Box::new(self.rewrite_standard_sum_header_type(element,resolved,path)),reference:*reference},
            Type::Function {params,ret}=>Type::Function {params:params.iter().map(|ty|self.rewrite_standard_sum_header_type(ty,resolved,path)).collect(),ret:Box::new(self.rewrite_standard_sum_header_type(ret,resolved,path))},
            _=>ty.clone(),
        }
    }

    fn rewrite_standard_sum_header_origin(&self,origin:&SourceOrigin,resolved:&HashMap<String,SourceOrigin>)->SourceOrigin{
        if let Some(Type::Named {path,..})=origin.ty.as_ref(){
            if let Some(mut actual)=self.origins.lookup(path,&origin.path).and_then(|entry|resolved.get(&entry.full_name)).cloned(){actual.wrappers=origin.wrappers.clone();return actual;}
        }
        let mut actual=origin.clone();actual.ty=actual.ty.as_ref().map(|ty|self.rewrite_standard_sum_header_type(ty,resolved,&actual.path));
        if let Some(fields)=actual.fields.as_mut(){for field in fields {*field=self.rewrite_standard_sum_header_origin(field,resolved);}}
        if let Some(parameters)=actual.callable_parameters.as_mut(){for parameter in parameters {*parameter=self.rewrite_standard_sum_header_origin(parameter,resolved);}}
        if let Some(result)=actual.callable_result.as_mut(){**result=self.rewrite_standard_sum_header_origin(result,resolved);}
        actual
    }

    fn finalize_standard_sum_header_pointer_origins(&mut self,origin:&SourceOrigin,rows:&[(String,SourceOrigin)],resolved:&mut HashMap<String,SourceOrigin>,active:&mut HashSet<String>)->Result<SourceOrigin>{
        if origin.source_function_pointer_header{return self.refresh_standard_sum_header_argument(origin,rows,resolved,active);}
        let mut actual=origin.clone();
        if let Some(fields)=actual.fields.as_mut(){for field in fields {*field=self.finalize_standard_sum_header_pointer_origins(field,rows,resolved,active)?;}}
        if let Some(parameters)=actual.callable_parameters.as_mut(){for parameter in parameters {*parameter=self.finalize_standard_sum_header_pointer_origins(parameter,rows,resolved,active)?;}}
        if let Some(result)=actual.callable_result.as_mut(){**result=self.finalize_standard_sum_header_pointer_origins(result,rows,resolved,active)?;}
        Ok(actual)
    }

    pub(super) fn finalize_standard_sum_source_declarations(&mut self)->Result<()>{
        self.initialize_standard_sum_source()?;let header=self.header_discovery;self.header_discovery=false;
        let outcome=(||{
            let rows=self.standard_sum_source.rows.clone();let mut resolved=HashMap::new();let mut active=HashSet::new();
            for (name,_) in &rows {self.resolve_standard_sum_header_row(name,&rows,&mut resolved,&mut active)?;}
            if !header&&rows.iter().any(|(name,origin)|resolved.get(name).is_some_and(|actual|actual.ty!=origin.ty||origin.standard_sum_source.as_ref().is_some_and(|sum|sum.provisional))){self.origins.standard_sum_source_needs_final_admission=true;}
            let mut catalog=HashMap::new();
            for (key,entry) in self.origins.catalog.clone(){
                let entry=match entry {
                    SourceDeclaration::Function {parameters,result}=>{
                        let parameters=parameters.iter().map(|origin|self.finalize_standard_sum_header_pointer_origins(origin,&rows,&mut resolved,&mut active)).collect::<Result<Vec<_>>>()?;
                        let result=self.finalize_standard_sum_header_pointer_origins(&result,&rows,&mut resolved,&mut active)?;
                        SourceDeclaration::Function {parameters:parameters.iter().map(|origin|self.rewrite_standard_sum_header_origin(origin,&resolved)).collect(),result:self.rewrite_standard_sum_header_origin(&result,&resolved)}
                    },
                    SourceDeclaration::Constructor {owner,fields,result}=>{
                        let fields=fields.iter().map(|origin|self.finalize_standard_sum_header_pointer_origins(origin,&rows,&mut resolved,&mut active)).collect::<Result<Vec<_>>>()?;
                        let result=self.finalize_standard_sum_header_pointer_origins(&result,&rows,&mut resolved,&mut active)?;
                        SourceDeclaration::Constructor {owner,fields:fields.iter().map(|origin|self.rewrite_standard_sum_header_origin(origin,&resolved)).collect(),result:self.rewrite_standard_sum_header_origin(&result,&resolved)}
                    },
                    SourceDeclaration::Constant {result}=>{
                        let result=self.finalize_standard_sum_header_pointer_origins(&result,&rows,&mut resolved,&mut active)?;
                        SourceDeclaration::Constant {result:self.rewrite_standard_sum_header_origin(&result,&resolved)}
                    },
                    other=>other,
                };catalog.insert(key,entry);
            }
            let mut unique=Vec::new();let mut names=HashSet::new();
            for (old,_) in &rows {
                let actual=resolved.get(old).expect("complete header row");let sum=actual.standard_sum_source.as_ref().expect("actual sum");let name=sum.type_path[1].clone();
                if names.insert(name.clone()){unique.push((name,actual.clone()));}
            }
            for (name,origin) in &rows {if !names.contains(name){catalog.remove(name);for variant in rules::read_rust_standard_sum_source_constructors(&origin.standard_sum_source.as_ref().expect("source sum").owner){catalog.remove(&format!("{name}.{variant}"));}}}
            for (ordinal,(_,origin)) in unique.iter_mut().enumerate(){origin.standard_sum_source.as_mut().expect("actual sum").ordinal=ordinal;}
            let ordinals=unique.iter().map(|(name,origin)|(name.clone(),origin.standard_sum_source.as_ref().expect("actual sum").ordinal)).collect::<HashMap<_,_>>();
            for entry in catalog.values_mut(){if let SourceDeclaration::Constructor {owner,result,..}=entry {if let Some(sum)=result.standard_sum_source.as_mut(){if let Some(ordinal)=ordinals.get(owner){sum.ordinal=*ordinal;}}}}
            self.origins.catalog=catalog;
            self.origins.standard_sum_source_origins=unique.iter().cloned().collect();self.standard_sum_source.rows=unique;
            Ok(())
        })();
        self.header_discovery=header;outcome
    }

    fn read_standard_sum_type_arguments(&mut self,owner:&str,start:&Token) -> Result<SourceOrigin> {
        self.cursor.expect("<",Some("standard sum type"))?;
        self.standard_sum_source.depth+=1;
        let result=(||{
            if count(self.standard_sum_source.depth)>rules::read_rust_standard_sum_source_depth_limit() { return Err(failure("nested standard sum exceeds its shared depth bound",self.standard_sum_range(start))); }
            let count=rules::read_rust_standard_sum_source_argument_count(owner) as usize;
            let mut arguments=Vec::new();
            for at in 0..count {
                if at>0 { self.cursor.expect(",",Some("standard sum type arguments"))?; }
                self.parse_type()?;arguments.push(self.origins.current_type.clone());
            }
            self.cursor.eat(",");self.split_type_closing_token();self.cursor.expect(">",Some("standard sum type"))?;
            self.instantiate_standard_sum_source(owner,arguments,self.standard_sum_range(start))
        })();
        self.standard_sum_source.depth-=1;
        result
    }

    pub(super) fn read_standard_sum_source_type(&mut self,segments:&[String],start:&Token) -> Result<Option<Type>> {
        if !self.ordinary_completion_context() || !self.cursor.is("<") { return Ok(None); }
        let Some((owner,variant))=self.standard_sum_resolved_owner(segments,&self.module_path,false) else { return Ok(None); };
        if !variant.is_empty() { return Ok(None); }
        let origin=self.read_standard_sum_type_arguments(&owner,start)?;
        let ty=origin.ty.clone().expect("actual instantiated type");self.origins.current_type=origin;
        Ok(Some(ty))
    }

    pub(super) fn read_standard_sum_source_constructor(&mut self,segments:&[String],path:&[String],start:&Token) -> Result<Option<SExpr>> {
        if !self.ordinary_completion_context() { return Ok(None); }
        let Some((owner,mut variant))=self.standard_sum_resolved_owner(segments,path,true) else { return Ok(None); };
        let explicit=if self.cursor.is("::")&&self.cursor.is_at("<",1) {
            self.cursor.advance();let origin=self.read_standard_sum_type_arguments(&owner,start)?;
            if variant.is_empty() {
                self.cursor.expect("::",Some("standard sum variant"))?;
                let token=self.cursor.identifier(Some("standard sum variant"))?;
                variant=rules::read_rust_standard_sum_source_path_constructor(&format!("{owner}::{}",token.value));
                if variant.is_empty() { return Err(failure("the variant must belong to its actual standard owner",self.standard_sum_range(start))); }
            }
            Some(origin)
        } else { None };
        if variant.is_empty() { return Ok(None); }
        let mut expected=explicit.or_else(||self.standard_sum_source.expected_origin.as_ref().and_then(|origin|self.origins.standard_sum_source_origin(origin)));
        if expected.as_ref().is_some_and(|origin|!origin.wrappers.is_empty()||origin.standard_sum_source.as_ref().is_none_or(|sum|sum.owner!=owner)) {
            return Err(failure("the constructor needs its actual direct expected standard owner",self.standard_sum_range(start)));
        }
        let arity=rules::read_rust_standard_sum_source_constructor_arity(&owner,&variant);
        let argument=rules::read_rust_standard_sum_source_field_argument(&owner,&variant);
        let mut args=Vec::new();
        if arity==1.0 {
            self.cursor.expect("(",Some("standard sum constructor"))?;
            let child=expected.as_ref().and_then(|origin|origin.standard_sum_source.as_ref()).and_then(|sum|sum.arguments.get(argument as usize)).cloned();
            let previous=std::mem::replace(&mut self.standard_sum_source.expected_origin,child);
            let parsed=self.expr(path,Options::default());self.standard_sum_source.expected_origin=previous;
            args.push(parsed?);self.cursor.eat(",");self.cursor.expect(")",Some("standard sum constructor"))?;
        } else if self.cursor.is("(") { return Err(type_error("None is a nullary standard constructor",self.standard_sum_range(start))); }
        if expected.is_none() {
            if !rules::accept_rust_standard_sum_source_inferred_constructor(&owner,&variant) { return Err(failure("this constructor needs a complete standard sum context",self.standard_sum_range(start))); }
            expected=Some(self.instantiate_standard_sum_source(&owner,vec![self.origins.origin(&args[0],path)],self.standard_sum_range(start))?);
        }
        let expected=expected.expect("actual constructor context");let sum=expected.standard_sum_source.as_ref().expect("actual sum");
        if arity==1.0 { self.origins.argument(&self.origins.origin(&args[0],path),&sum.arguments[argument as usize],self.standard_sum_range(start))?; }
        let target=sum.constructor_paths.get(&variant).cloned().expect("actual variant path");let range=self.standard_sum_range(start);
        let node=if arity==0.0 { SNode::Name { path:target } } else { SNode::App { func:Box::new(SExpr::new(SNode::Name { path:target },range)),args } };
        let value=SExpr::new(node,range);self.origins.mark(&value,expected);Ok(Some(value))
    }

    pub(super) fn read_standard_sum_source_pattern(&mut self,segments:&[String],path:&[String],start:&Token,aliases:&mut Vec<Alias>) -> Result<Option<SPattern>> {
        if !self.ordinary_completion_context()||self.cursor.is("@") { return Ok(None); }
        let Some((owner,variant))=self.standard_sum_resolved_owner(segments,path,true) else { return Ok(None); };
        if variant.is_empty() { return Ok(None); }
        let arity=rules::read_rust_standard_sum_source_constructor_arity(&owner,&variant);let mut args=Vec::new();
        if arity==1.0 {
            self.cursor.expect("(",Some("standard sum pattern"))?;
            if self.cursor.eat("..").is_some() { self.origins.wildcard_pattern();args.push(SPattern { node:SPatternNode::Wild,span:None }); }
            else {
                let previous=self.standard_sum_source.expected_origin.take();
                let parsed=self.pattern(path,aliases);self.standard_sum_source.expected_origin=previous;args.push(parsed?);
            }
            self.cursor.eat(",");self.cursor.expect(")",Some("standard sum pattern"))?;
        } else if self.cursor.is("(") { return Err(type_error("None pattern expects no fields",self.standard_sum_range(start))); }
        Ok(Some(SPattern { node:SPatternNode::Ctor { path:vec![DEFERRED_PATTERN.into(),owner,variant],args },span:self.standard_sum_range(start) }))
    }

    pub(super) fn finish_standard_sum_source(&mut self,items:&mut Vec<SItem>) -> Result<()> {
        if self.header_discovery {return Ok(());}
        self.initialize_standard_sum_source()?;
        let rows=self.standard_sum_source.rows.clone();
        for (name,origin) in rows {
            let sum=origin.standard_sum_source.as_ref().expect("registered source sum");let mut groups=Vec::new();let mut ctors=Vec::new();
            for variant in rules::read_rust_standard_sum_source_constructors(&sum.owner) {
                let argument=rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);
                let origins=if argument<0.0 { Vec::new() } else { vec![sum.arguments[argument as usize].clone()] };
                let fields=origins.iter().map(|origin|SField { name:None,ty:origin.ty.clone().expect("complete child"),rocq_type:None,span:None }).collect();
                groups.push((variant.clone(),origins));ctors.push(SCtor { name:variant,fields });
            }
            let previous=self.retain_source_type_owners;self.retain_source_type_owners=true;
            let captured=self.capture_data_field_signatures(&name,&[],groups.iter().map(|(name,fields)|(name.as_str(),fields.as_slice())));
            self.retain_source_type_owners=previous;
            items.push(SItem::Data(SData { name,ctors,span:None,generated:true,source_unit_clone_derivation:None,source_field_signatures:captured? }));
        }
        Ok(())
    }

    pub(super) fn refuse_standard_sum_source_patterns(&self,program:&SProgram) -> Result<()> {
        // This existing surface serialization is traversed only to reject an
        // impossible source-path marker. It is never a semantic payload,
        // identity, source restoration path or replacement pattern interpreter.
        let root=serde_json::to_value(program).map_err(|_|type_error("standard sum pattern surface transport failed",None))?;
        let mut pending=vec![&root];let mut visited=0usize;
        while let Some(value)=pending.pop() {
            match value {
                serde_json::Value::Object(fields)=>{
                    visited+=1;
                    if visited>100000 { return Err(failure("source pattern admission exceeds its finite node budget",None)); }
                    if fields.get("path").and_then(serde_json::Value::as_array).and_then(|path|path.first()).and_then(serde_json::Value::as_str)==Some(DEFERRED_PATTERN) {
                        return Err(failure("a standard constructor pattern has no actual resolved scrutinee owner",None));
                    }
                    pending.extend(fields.values());
                }
                serde_json::Value::Array(values)=>{
                    visited+=1;
                    if visited>100000 { return Err(failure("source pattern admission exceeds its finite node budget",None)); }
                    pending.extend(values);
                }
                _=>{}
            }
        }
        Ok(())
    }
}

