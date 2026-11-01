//! Actual token/type/origin transport for source-generated generic alias rules.
//! The normative decisions live in rust-generic-type-alias-rules.js. Native
//! policy/Links products are produced by its deferred source-library recipe.
use super::{Result, RustParser, SExpr, SNode, SourceDeclaration, SourceOrigin, Span, Token, TokenKind, Type, UNIT};
use super::type_aliases::AliasDeclaration;
use crate::translation::rust_generic_type_alias_rules as rules;
use crate::translation::rust_type_alias_rules as alias_rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
use crate::translation::source_signatures::SourceOwner;
use std::rc::Rc;

// Rc preserves private template sharing during composition. Every traversal and
// every eventual complete origin copy is charged before materialization.
pub(super) enum GenericAliasTemplate {
    Parameter(String),
    Origin(Box<SourceOrigin>),
    Tuple(Vec<Rc<Self>>),
    StandardSum { owner: String, arguments: Vec<Rc<Self>> },
    Vector { owner: String, element: Rc<Self> },
}

impl RustParser {
    fn charge_generic_alias_template(&mut self, row: &AliasDeclaration, count: usize) -> Result<()> {
        let error = alias_rules::read_rust_type_alias_clone_error(self.source_type_aliases.nodes as f64, count as f64);
        if !error.is_empty() { return Err(self.alias_error(&error, Some(row))); }
        self.source_type_aliases.nodes += count;
        Ok(())
    }

    fn generic_alias_template_bindings(&mut self, root: &Rc<GenericAliasTemplate>, row: &AliasDeclaration) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut pending = vec![(Rc::clone(root), 0usize)];
        let mut count = 0;
        while let Some((node, depth)) = pending.pop() {
            count += 1;
            let error = rules::read_rust_generic_type_alias_template_depth_error(depth as f64);
            if !error.is_empty() { return Err(self.alias_error(&error, Some(row))); }
            let error = alias_rules::read_rust_type_alias_clone_error(self.source_type_aliases.nodes as f64, count as f64);
            if !error.is_empty() { return Err(self.alias_error(&error, Some(row))); }
            match node.as_ref() {
                GenericAliasTemplate::Parameter(identity) => result.push(identity.clone()),
                GenericAliasTemplate::Tuple(fields) | GenericAliasTemplate::StandardSum {arguments:fields,..} => {
                    for field in fields.iter().rev() { pending.push((Rc::clone(field), depth + 1)); }
                }
                GenericAliasTemplate::Origin(_) => {}
                GenericAliasTemplate::Vector {element,..} => pending.push((Rc::clone(element),depth+1)),
            }
        }
        self.source_type_aliases.nodes += count;
        Ok(result)
    }

    fn generic_alias_source_row(&self, segments: &[String]) -> Option<AliasDeclaration> {
        self.generic_alias_source_row_at(segments,&self.module_path)
    }
    fn generic_alias_source_row_at(&self, segments: &[String],path:&[String]) -> Option<AliasDeclaration> {
        let resolved = self.resolve(segments, path);
        for key in alias_rules::read_rust_type_alias_lookup_keys(&resolved, path) {
            if let Some(row) = self.source_type_aliases.declarations.get(&key) { return Some(row.clone()); }
            if alias_rules::read_rust_type_alias_namespace_disposition(false, self.alias_namespace_kind(&key)) == "block" { return None; }
        }
        None
    }

    fn close_generic_alias_arguments(&mut self, row: &AliasDeclaration) -> Result<()> {
        self.split_type_closing_token();
        if self.cursor.eat(">").is_none() { return Err(self.alias_error("generic alias arguments need a complete closing delimiter", Some(row))); }
        Ok(())
    }
    fn generic_alias_header_origin(&self,origin:&SourceOrigin)->bool{
        if self.generic_alias_callable_template(origin,true){return true;}
        let name=match origin.ty.as_ref(){Some(Type::Named {path,..})=>self.origins.lookup_type_owner(path,&origin.path).filter(|entry|matches!(entry.declaration,SourceDeclaration::Data)).map(|entry|entry.full_name),_=>None};
        let stored=name.as_ref().and_then(|name|self.origins.standard_sum_source_origins.get(name));let sum=stored.and_then(|origin|origin.standard_sum_source.as_deref());let owner=origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());
        rules::accept_rust_generic_type_alias_header_standard_sum_origin(self.header_discovery,origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),sum.map_or("",|sum|sum.owner.as_str()),sum.map_or(-1.0,|sum|sum.arguments.len() as f64),stored.map_or(-1.0,|origin|origin.wrappers.len() as f64),sum.is_some_and(|sum|sum.provisional))
    }
    fn generic_alias_callable_template(&self,origin:&SourceOrigin,header:bool)->bool{
        let owner=origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());
        let parameters=match origin.ty.as_ref(){Some(Type::Function {params,..})=>params.len() as f64,_=>-1.0};
        let copies=origin.callable_parameters.iter().flatten().map(SourceOrigin::direct_copy_value).collect::<Vec<_>>();
        let result_copy=origin.callable_result.as_deref().is_some_and(SourceOrigin::direct_copy_value);
        let result_typed=origin.callable_result.as_deref().is_some_and(|result|result.ty.is_some());
        let closure=origin.copy_closure_identity.is_some()||origin.copy_closure_capture_count.is_some();
        if header{
            rules::accept_rust_generic_type_alias_header_callable_template(self.ordinary_completion_context(),origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),owner.map_or(-1.0,|owner|owner.path.len() as f64),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),parameters,&copies,result_copy,result_typed,self.header_discovery,origin.source_function_pointer_header,closure)
        }else{
            rules::accept_rust_generic_type_alias_concrete_callable_template(self.ordinary_completion_context(),origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),owner.map_or(-1.0,|owner|owner.path.len() as f64),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),parameters,&copies,result_copy,result_typed,origin.proven,origin.copy_evidence.is_some(),closure)
        }
    }
    fn generic_alias_vector_owner(&self,segments:&[String])->String {
        let resolved=self.resolve(segments,&self.module_path);
        let mut shadowed=false;
        for root in [segments.first(),resolved.first()].into_iter().flatten(){
            let root=std::slice::from_ref(root);
            if self.origins.lookup_type_owner(root,&self.module_path).is_some() || alias_rules::read_rust_type_alias_lookup_keys(root,&self.module_path).iter().any(|key|self.source_type_aliases.declarations.contains_key(key)){shadowed=true;}
        }
        rules::read_rust_generic_type_alias_vector_owner(&resolved.join("::"),shadowed)
    }

    fn substitute_generic_alias_template(&mut self, node: &Rc<GenericAliasTemplate>, arguments: &[Rc<GenericAliasTemplate>], plan: &[String], occurrence: &mut usize, row: &AliasDeclaration, depth: usize) -> Result<Rc<GenericAliasTemplate>> {
        let error = rules::read_rust_generic_type_alias_template_depth_error(depth as f64);
        if !error.is_empty() { return Err(self.alias_error(&error, Some(row))); }
        self.charge_generic_alias_template(row, 1)?;
        match node.as_ref() {
            GenericAliasTemplate::Parameter(_) => {
                let index = plan[*occurrence].parse::<usize>().map_err(|_|self.alias_error("generic alias template contains an unresolved parameter binding",Some(row)))?;
                *occurrence += 1;
                Ok(Rc::clone(&arguments[index]))
            }
            GenericAliasTemplate::Tuple(fields) => {
                let mut result = Vec::new();
                for field in fields { result.push(self.substitute_generic_alias_template(field,arguments,plan,occurrence,row,depth+1)?); }
                Ok(Rc::new(GenericAliasTemplate::Tuple(result)))
            }
            GenericAliasTemplate::Origin(_) => Ok(Rc::clone(node)),
            GenericAliasTemplate::StandardSum {owner,arguments:children} => {
                let mut result=Vec::new();
                for child in children { result.push(self.substitute_generic_alias_template(child,arguments,plan,occurrence,row,depth+1)?); }
                Ok(Rc::new(GenericAliasTemplate::StandardSum {owner:owner.clone(),arguments:result}))
            }
            GenericAliasTemplate::Vector {owner,element}=>Ok(Rc::new(GenericAliasTemplate::Vector {owner:owner.clone(),element:self.substitute_generic_alias_template(element,arguments,plan,occurrence,row,depth+1)?})),
        }
    }

    fn generic_alias_symbolic_application(&mut self, row: &AliasDeclaration, arguments: &[Rc<GenericAliasTemplate>]) -> Result<Rc<GenericAliasTemplate>> {
        let template = self.prepare_generic_type_alias(row)?;
        let defaults=self.source_type_aliases.defaults.get(&row.key).expect("checked default directory").clone();
        let mut arguments=arguments.to_vec();
        let present=defaults.iter().map(Option::is_some).collect::<Vec<_>>();
        let missing=rules::read_rust_generic_type_alias_default_argument_plan(&present,arguments.len() as f64,true);
        if missing.first().map(String::as_str)!=Some("arguments"){return Err(self.alias_error(&missing[1],Some(row)));}
        for position in &missing[1..]{
            let index=position.parse::<usize>().expect("shared default position");let node=defaults[index].as_ref().expect("shared actual default");
            let occurrences=self.generic_alias_template_bindings(node,row)?;
            let plan=rules::read_rust_generic_type_alias_default_substitution_plan(&row.parameter_bindings,index as f64,arguments.len() as f64,&occurrences);
            if plan.first().map(String::as_str)!=Some("substitution"){return Err(self.alias_error(&plan[1],Some(row)));}
            let mut occurrence=1;let value=self.substitute_generic_alias_template(node,&arguments,&plan,&mut occurrence,row,0)?;arguments.push(value);
        }
        let occurrences = self.generic_alias_template_bindings(&template,row)?;
        let plan = rules::read_rust_generic_type_alias_binding_plan(&row.parameter_bindings,arguments.len() as f64,&occurrences);
        if plan.first().map(String::as_str) != Some("substitution") { return Err(self.alias_error(&plan[1],Some(row))); }
        let mut occurrence = 1;
        self.substitute_generic_alias_template(&template,&arguments,&plan,&mut occurrence,row,0)
    }

    fn generic_alias_template_value(&mut self, row: &AliasDeclaration, depth: usize) -> Result<Rc<GenericAliasTemplate>> {
        let error = rules::read_rust_generic_type_alias_template_depth_error(depth as f64);
        if !error.is_empty() { return Err(self.alias_error(&error,Some(row))); }
        self.charge_generic_alias_template(row,1)?;
        let start = self.cursor.index;
        let start_coordinate = self.cursor.peek().start;
        if self.cursor.eat("(").is_some() {
            if self.cursor.eat(")").is_some() {
                let mut origin = SourceOrigin::with_type(UNIT,&self.module_path,Vec::new());
                origin.source_owner = Some(SourceOwner { kind:"unit".to_owned(),path:Vec::new(),arguments:Vec::new() });
                return Ok(Rc::new(GenericAliasTemplate::Origin(Box::new(self.copy_type_alias_origin(&origin,row)?))));
            }
            let first = self.generic_alias_template_value(row,depth+1)?;
            if self.cursor.eat(",").is_none() { self.cursor.expect(")",Some("generic alias parenthesized template"))?; return Ok(first); }
            let mut fields = vec![first];
            while !self.cursor.is(")") {
                fields.push(self.generic_alias_template_value(row,depth+1)?);
                if self.cursor.eat(",").is_none() { break; }
            }
            self.cursor.expect(")",Some("generic alias tuple template"))?;
            return Ok(Rc::new(GenericAliasTemplate::Tuple(fields)));
        }
        if self.cursor.peek().kind == TokenKind::Identifier {
            if let Some(parameter) = row.parameters.iter().position(|name|name == &self.cursor.peek().value) {
                self.cursor.advance();
                if self.cursor.is("::") || self.cursor.is("<") { return Err(self.alias_error("generic alias parameter projections need their complete interpretation",Some(row))); }
                return Ok(Rc::new(GenericAliasTemplate::Parameter(row.parameter_bindings[parameter].clone())));
            }
            let segments = self.path_segments("generic alias template type")?;
            let target=self.generic_alias_source_row(&segments);
            if let Some(target) = target.as_ref().filter(|target|!target.parameters.is_empty()) {
                self.prepare_generic_type_alias(target)?;
                let explicit=self.cursor.eat("<").is_some();
                let mut arguments = Vec::new();
                if explicit{self.split_type_closing_token();
                while !self.cursor.is(">") {
                    arguments.push(self.generic_alias_template_value(row,depth+1)?);
                    self.split_type_closing_token();
                    if self.cursor.eat(",").is_none() { break; }
                    self.split_type_closing_token();
                }
                self.close_generic_alias_arguments(row)?;}
                let defaults=self.source_type_aliases.defaults.get(&target.key).expect("checked target defaults").iter().map(Option::is_some).collect::<Vec<_>>();
                let missing=rules::read_rust_generic_type_alias_default_argument_plan(&defaults,arguments.len() as f64,explicit);
                if missing.first().map(String::as_str)!=Some("arguments"){return Err(self.alias_error(&missing[1],Some(target)));}
                return self.generic_alias_symbolic_application(target,&arguments);
            }
            let sum_owner=if target.is_none() {self.resolve_standard_sum_source_owner(&segments,&self.module_path)} else {String::new()};
            if !sum_owner.is_empty() && self.cursor.eat("<").is_some() {
                let constructor=sum_rules::read_rust_standard_sum_source_path_constructor(&self.resolve(&segments,&self.module_path).join("::"));
                let mut arguments=Vec::new();
                self.split_type_closing_token();
                while !self.cursor.is(">") {
                    arguments.push(self.generic_alias_template_value(row,depth+1)?);
                    self.split_type_closing_token();
                    if self.cursor.eat(",").is_none() { break; }
                    self.split_type_closing_token();
                }
                self.close_generic_alias_arguments(row)?;
                let owner=rules::read_rust_generic_type_alias_standard_sum_owner(&sum_owner,&constructor,arguments.len() as f64);
                if owner.is_empty() { return Err(self.alias_error("generic alias standard sum needs its actual type owner and complete explicit arguments",Some(row))); }
                return Ok(Rc::new(GenericAliasTemplate::StandardSum {owner,arguments}));
            }
            let vector=if target.is_none(){self.generic_alias_vector_owner(&segments)}else{String::new()};
            if !vector.is_empty() && self.cursor.eat("<").is_some(){
                let mut arguments=Vec::new();self.split_type_closing_token();
                while !self.cursor.is(">"){
                    arguments.push(self.generic_alias_template_value(row,depth+1)?);
                    self.split_type_closing_token();if self.cursor.eat(",").is_none(){break;}self.split_type_closing_token();
                }
                self.close_generic_alias_arguments(row)?;
                if !rules::accept_rust_generic_type_alias_vector_arguments(&vector,arguments.len() as f64){return Err(self.alias_error("generic vector alias templates need the actual standard owner and one explicit element",Some(row)));}
                return Ok(Rc::new(GenericAliasTemplate::Vector {owner:vector,element:arguments.remove(0)}));
            }
            self.cursor.index = start;
        }
        self.parse_type()?;
        let origin = &self.origins.current_type;
        let owner = origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());
        let (bits,signed) = match origin.ty.as_ref() { Some(Type::Fixed {bits,signed}) => (bits.to_string(),*signed), _ => (String::new(),false) };
        let accepted = rules::accept_rust_generic_type_alias_concrete_template(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),&bits,signed,owner.map_or(-1.0,|owner|owner.arguments.len() as f64),origin.proven);
        let actual_data=match origin.ty.as_ref() {Some(Type::Named {path,..})=>self.origins.lookup_type_owner(path,&origin.path).filter(|entry|matches!(entry.declaration,SourceDeclaration::Data)).map(|entry|entry.full_name),_=>None};
        let named=rules::accept_rust_generic_type_alias_named_template(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),origin.proven,actual_data.as_deref().unwrap_or(""));
        let stored=actual_data.as_ref().and_then(|name|self.origins.standard_sum_source_origins.get(name));
        let sum=stored.and_then(|origin|origin.standard_sum_source.as_deref());
        let sum_leaf=rules::accept_rust_generic_type_alias_standard_sum_leaf(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,sum.map_or("",|sum|sum.owner.as_str()),sum.map_or(-1.0,|sum|sum.arguments.len() as f64),origin.proven,stored.is_some_and(|origin|origin.proven),stored.is_some_and(|origin|origin.copy_evidence.is_some()),stored.map_or(-1.0,|origin|origin.wrappers.len() as f64));
        let owned_profile=stored.map(|stored|self.origins.standard_sum_ownership_facts(stored).profile).unwrap_or_default();
        let owned_sum_leaf=rules::accept_rust_generic_type_alias_owned_standard_sum_leaf(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,sum.map_or("",|sum|sum.owner.as_str()),sum.map_or(-1.0,|sum|sum.arguments.len() as f64),origin.proven,stored.is_some_and(|origin|origin.proven),stored.map_or(-1.0,|origin|origin.wrappers.len() as f64),&owned_profile);
        let string=rules::accept_rust_generic_type_alias_owned_string_template(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),origin.proven,origin.text_source_model.unwrap_or(""));
        if !accepted && !named && !sum_leaf && !owned_sum_leaf && !self.generic_alias_callable_template(origin,false) && !self.generic_alias_header_origin(origin) && !string { return Err(self.alias_error("generic alias concrete leaves require a direct admitted scalar, Unit, owned String or actual source datatype",Some(row))); }
        let first = self.cursor.tokens.iter().position(|token|token.start == start_coordinate).expect("actual generic template source token");
        for index in first..self.cursor.index {
            let token=&self.cursor.tokens[index];let qualified=index>0&&self.cursor.tokens[index-1].value=="::";
            if token.kind == TokenKind::Identifier && !rules::accept_rust_generic_type_alias_concrete_identifier(&row.parameters,&token.value,qualified) { return Err(self.alias_error("parameter-dependent wrapper and library alias templates need their complete interpretation",Some(row))); }
        }
        Ok(Rc::new(GenericAliasTemplate::Origin(Box::new(self.copy_current_type_alias_origin(row)?))))
    }

    pub(super) fn prepare_generic_type_alias(&mut self, row: &AliasDeclaration) -> Result<Rc<GenericAliasTemplate>> {
        if let Some(template) = self.source_type_aliases.templates.get(&row.key) { return Ok(Rc::clone(template)); }
        let declarations = self.source_type_aliases.stack.iter().cloned().collect::<Vec<_>>();
        let error = rules::read_rust_generic_type_alias_expansion_error(&row.key,&declarations,self.source_type_aliases.expansions as f64);
        if !error.is_empty() { return Err(self.alias_error(&error,Some(row))); }
        let previous_start = self.cursor.peek().start;
        let previous_path = std::mem::replace(&mut self.module_path,row.path.clone());
        let previous_lifetimes = std::mem::take(&mut self.lifetime_bindings);
        let previous_header = std::mem::replace(&mut self.lifetime_header,true);
        let previous_inherent = self.inherent_owner.take();
        self.source_type_aliases.expansions += 1;
        self.source_type_aliases.stack.insert(row.key.clone());
        self.cursor.index = self.cursor.tokens.iter().position(|token|token.start == row.first).expect("actual generic alias definition start");
        let result = (|| -> Result<Rc<GenericAliasTemplate>> {
            let mut defaults=Vec::new();
            for (index,span) in row.default_spans.iter().enumerate(){
                let Some((start,end))=span else{defaults.push(None);continue;};
                self.cursor.index=self.cursor.tokens.iter().position(|token|token.start==*start).expect("actual default start");
                let node=self.generic_alias_template_value(row,0)?;
                if self.cursor.peek().start!=*end{return Err(self.alias_error("the complete generic alias default needs one admitted source type",Some(row)));}
                let occurrences=self.generic_alias_template_bindings(&node,row)?;
                let plan=rules::read_rust_generic_type_alias_default_substitution_plan(&row.parameter_bindings,index as f64,index as f64,&occurrences);
                if plan.first().map(String::as_str)!=Some("substitution"){return Err(self.alias_error(&plan[1],Some(row)));}defaults.push(Some(node));
            }
            self.cursor.index=self.cursor.tokens.iter().position(|token|token.start==row.first).expect("actual generic alias definition start");
            let template = self.generic_alias_template_value(row,0)?;
            if self.cursor.peek().start != row.end || !self.cursor.is(";") { return Err(self.alias_error("the complete alias definition needs one admitted source type",Some(row))); }
            let used = self.generic_alias_template_bindings(&template,row)?;
            let error = rules::read_rust_generic_type_alias_unused_parameter_error(&row.parameter_bindings,&used);
            if !error.is_empty() { return Err(self.alias_error(&error,Some(row))); }
            self.source_type_aliases.defaults.insert(row.key.clone(),defaults);
            self.source_type_aliases.templates.insert(row.key.clone(),Rc::clone(&template));
            Ok(template)
        })();
        self.cursor.index = self.cursor.tokens.iter().position(|token|token.start == previous_start).expect("actual generic alias caller token");
        self.module_path = previous_path;
        self.lifetime_bindings = previous_lifetimes;
        self.lifetime_header = previous_header;
        self.inherent_owner = previous_inherent;
        self.source_type_aliases.stack.remove(&row.key);
        result
    }

    fn materialize_generic_alias_template(&mut self, node: &Rc<GenericAliasTemplate>, arguments: &[SourceOrigin], plan: &[String], occurrence: &mut usize, row: &AliasDeclaration, depth: usize) -> Result<SourceOrigin> {
        let error = rules::read_rust_generic_type_alias_template_depth_error(depth as f64);
        if !error.is_empty() { return Err(self.alias_error(&error,Some(row))); }
        self.charge_generic_alias_template(row,1)?;
        match node.as_ref() {
            GenericAliasTemplate::Parameter(_) => {
                let index = plan[*occurrence].parse::<usize>().map_err(|_|self.alias_error("generic alias template contains an unresolved parameter binding",Some(row)))?;
                *occurrence += 1;
                self.copy_type_alias_origin(&arguments[index],row)
            }
            GenericAliasTemplate::Origin(origin) => {
                let mut origin=self.copy_type_alias_origin(origin,row)?;
                if matches!(origin.ty.as_ref(),Some(Type::Named {..})) && origin.standard_sum_source.is_none() { self.origins.retain_nominal_copy_origin(&mut origin); }
                Ok(origin)
            }
            GenericAliasTemplate::StandardSum {owner,arguments:children} => {
                self.standard_sum_source.depth+=1;
                let result=(||{
                    let error=rules::read_rust_generic_type_alias_standard_sum_depth_error(self.standard_sum_source.depth as f64);
                    if !error.is_empty() { return Err(self.alias_error(&error,Some(row))); }
                    let mut concrete_arguments=Vec::new();
                    for child in children { concrete_arguments.push(self.materialize_generic_alias_template(child,arguments,plan,occurrence,row,depth+1)?); }
                    let origin=self.instantiate_standard_sum_source(owner,concrete_arguments,Some(row.keyword))?;
                    self.copy_type_alias_origin(&origin,row)
                })();
                self.standard_sum_source.depth-=1;
                result
            }
            GenericAliasTemplate::Tuple(children) => {
                let mut fields = Vec::new();
                for child in children { fields.push(self.materialize_generic_alias_template(child,arguments,plan,occurrence,row,depth+1)?); }
                let elements = fields.iter().map(|field|field.ty.clone().expect("actual generic tuple field type")).collect();
                let mut origin = SourceOrigin::with_type(Type::Tuple {elements},&row.path,Vec::new());
                origin.copy_evidence = fields.iter().all(SourceOrigin::direct_copy_value).then_some(());
                origin.source_owner = Some(SourceOwner {kind:"tuple".to_owned(),path:Vec::new(),arguments:fields.iter().map(|field|field.source_owner.clone().expect("actual generic tuple field owner")).collect()});
                origin.fields = Some(fields);
                if !self.header_discovery && !super::product_rules::accept_rust_product_origin(origin.direct_copy_value(),self.origins.owned_local_product(&origin)) { return Err(self.alias_error("generic alias tuple fields need the existing immutable or owned product interpretation",Some(row))); }
                self.copy_type_alias_origin(&origin,row)
            }
            GenericAliasTemplate::Vector {owner,element}=>{
                let element=self.materialize_generic_alias_template(element,arguments,plan,occurrence,row,depth+1)?;
                let source=element.expanded_source_owner.as_ref().or(element.source_owner.as_ref());
                let (bits,signed)=match element.ty.as_ref(){Some(Type::Fixed {bits,signed})=>(bits.to_string(),*signed),_=>(String::new(),false)};
                let error=rules::read_rust_generic_type_alias_vector_element_error(element.ty.as_ref().map_or("",Type::kind),element.wrappers.len() as f64,source.map_or("",|owner|owner.kind.as_str()),&source.map(|owner|owner.path.join("::")).unwrap_or_default(),&bits,signed,source.map_or(-1.0,|owner|owner.arguments.len() as f64),element.proven,element.text_source_model.unwrap_or(""));
                if !error.is_empty(){return Err(self.alias_error(&error,Some(row)));}
                let mut origin=SourceOrigin::with_type(Type::Array {element:Box::new(element.ty.clone().expect("actual generic vector element type")),reference:false},&row.path,Vec::new());
                origin.source_owner=Some(SourceOwner {kind:"library".to_owned(),path:owner.split("::").map(str::to_owned).collect(),arguments:vec![source.cloned().expect("shared vector element policy proved actual source owner")]});
                self.copy_type_alias_origin(&origin,row)
            }
        }
    }

    pub(super) fn read_generic_type_alias_value(&mut self, row: &AliasDeclaration) -> Result<SourceOrigin> {
        self.read_generic_type_alias_value_arguments(row,true)
    }
    pub(super) fn read_generic_type_alias_default_value(&mut self,row:&AliasDeclaration)->Result<SourceOrigin>{
        self.read_generic_type_alias_value_arguments(row,false)
    }
    pub(super) fn prepare_default_type_alias_access(&mut self,segments:&[String],path:&[String])->Result<()>{
        if !self.ordinary_completion_context()||segments.len()<2{return Ok(());}
        self.prepare_default_type_alias_owner(&segments[..segments.len()-1],path,false)
    }
    pub(super) fn prepare_default_type_alias_owner(&mut self,segments:&[String],path:&[String],discovery:bool)->Result<()>{
        if !self.ordinary_completion_context()||rules::read_rust_generic_type_alias_default_owner_phase(self.origins.collecting,discovery)=="defer"{return Ok(());}
        let Some(row)=self.generic_alias_source_row_at(segments,path).filter(|row|!row.parameters.is_empty())else{return Ok(());};
        self.prepare_type_alias_default_origin(&row)
    }
    pub(super) fn prepare_default_type_alias_access_head(&mut self,path:&[String])->Result<()>{
        if !self.ordinary_completion_context()||self.cursor.is("::"){return Ok(());}
        let mut segments=Vec::new();let mut offset=0;
        while self.cursor.peek_at(offset).kind==TokenKind::Identifier&&(segments.len() as f64)<rules::read_rust_generic_type_alias_default_access_head_limit(){
            segments.push(self.cursor.peek_at(offset).value.clone());offset+=1;
            if !self.cursor.is_at("::",offset){return self.prepare_default_type_alias_access(&segments,path);}
            if self.cursor.peek_at(offset+1).kind!=TokenKind::Identifier{return Ok(());}
            offset+=1;
        }Ok(())
    }
    fn read_generic_type_alias_value_arguments(&mut self,row:&AliasDeclaration,allow_argument_tokens:bool)->Result<SourceOrigin>{
        let template = self.prepare_generic_type_alias(row)?;
        let explicit=allow_argument_tokens&&self.cursor.eat("<").is_some();
        let mut arguments = Vec::new();
        if explicit{self.split_type_closing_token();
        while !self.cursor.is(">") {
            self.parse_type()?;
            arguments.push(self.copy_current_type_alias_origin(row)?);
            self.split_type_closing_token();
            if self.cursor.eat(",").is_none() { break; }
            self.split_type_closing_token();
        }
        self.close_generic_alias_arguments(row)?;}
        let defaults=self.source_type_aliases.defaults.get(&row.key).expect("checked default directory").clone();
        let present=defaults.iter().map(Option::is_some).collect::<Vec<_>>();
        let missing=rules::read_rust_generic_type_alias_default_argument_plan(&present,arguments.len() as f64,explicit);
        if missing.first().map(String::as_str)!=Some("arguments"){return Err(self.alias_error(&missing[1],Some(row)));}
        for position in &missing[1..]{
            let index=position.parse::<usize>().expect("shared default argument position");let node=defaults[index].as_ref().expect("shared complete default");
            let occurrences=self.generic_alias_template_bindings(node,row)?;
            let plan=rules::read_rust_generic_type_alias_default_substitution_plan(&row.parameter_bindings,index as f64,arguments.len() as f64,&occurrences);
            if plan.first().map(String::as_str)!=Some("substitution"){return Err(self.alias_error(&plan[1],Some(row)));}
            let mut occurrence=1;let origin=self.materialize_generic_alias_template(node,&arguments,&plan,&mut occurrence,row,0)?;arguments.push(origin);
        }
        let occurrences = self.generic_alias_template_bindings(&template,row)?;
        let proven = arguments.iter().map(|origin|origin.proven && origin.ty.is_some()).collect::<Vec<_>>();
        let mut plan = rules::read_rust_generic_type_alias_substitution_plan(&row.parameter_bindings,&proven,&occurrences);
        if plan.first().map(String::as_str)!=Some("substitution"){
            let typed=arguments.iter().map(|origin|origin.ty.is_some()&&origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref()).is_some()).collect::<Vec<_>>();
            let provisional=arguments.iter().map(|origin|self.generic_alias_header_origin(origin)).collect::<Vec<_>>();
            plan=rules::read_rust_generic_type_alias_header_substitution_plan(&row.parameter_bindings,self.header_discovery,&typed,&proven,&provisional,&occurrences);
        }
        if plan.first().map(String::as_str) != Some("substitution") { return Err(self.alias_error(&plan[1],Some(row))); }
        let mut occurrence = 1;
        self.materialize_generic_alias_template(&template,&arguments,&plan,&mut occurrence,row,0)
    }

    /// Instantiate explicit or completely defaulted alias arguments before
    /// selecting an associated operation from its checked definition owner.
    pub(super) fn read_generic_type_alias_constructor(&mut self,segments:&[String],path:&[String],start:&Token)->Result<Option<SExpr>>{
        if !self.ordinary_completion_context(){return Ok(None);}
        let explicit=self.cursor.is("::")&&self.cursor.is_at("<",1);
        if !explicit&&segments.len()<2{return Ok(None);}
        let prefix=if explicit{segments}else{&segments[..segments.len()-1]};
        let Some(row)=self.generic_alias_source_row_at(prefix,path).filter(|row|!row.parameters.is_empty()) else {return Ok(None);};
        let defaults=row.default_spans.iter().map(Option::is_some).collect::<Vec<_>>();
        if !explicit&&rules::read_rust_generic_type_alias_default_argument_plan(&defaults,0.0,false).first().map(String::as_str)!=Some("arguments"){return Ok(None);}
        let previous_member=(!explicit).then(||self.cursor.tokens[self.cursor.index-1].clone());
        if explicit{self.cursor.advance();}
        let mut origin=if explicit{self.read_generic_type_alias_value(&row)?}else{self.default_type_alias_access_origin(&row)?.expect("shared all-default argument directory")};
        origin.expanded_source_owner=origin.source_owner.clone();
        let member=if explicit{self.cursor.expect("::",Some("generic alias associated constructor"))?;self.cursor.identifier(Some("generic alias constructor member"))?}else{previous_member.expect("actual alias member token")};
        let owner=origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref());
        let name=match origin.ty.as_ref(){Some(Type::Named {path:actual,..})=>self.origins.lookup_type_owner(actual,&origin.path).filter(|entry|matches!(entry.declaration,SourceDeclaration::Data)).map(|entry|entry.full_name),_=>None};
        let stored=name.as_ref().and_then(|name|self.origins.standard_sum_source_origins.get(name));
        let sum=stored.and_then(|origin|origin.standard_sum_source.as_deref());
        let element=match origin.ty.as_ref(){Some(Type::Array {element,..})=>element.kind(),_=>""};
        let mut plan=rules::read_rust_generic_type_alias_constructor_plan(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),element,origin.array_source_model.unwrap_or(""),origin.proven,sum.map_or("",|sum|sum.owner.as_str()),sum.map_or(-1.0,|sum|sum.arguments.len() as f64),stored.is_some_and(|origin|origin.proven),stored.is_some_and(|origin|origin.copy_evidence.is_some()),&member.value);
        if plan.is_empty(){
            let profile=stored.map(|stored|self.origins.standard_sum_ownership_facts(stored).profile).unwrap_or_default();
            plan=rules::read_rust_generic_type_alias_owned_constructor_plan(origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),origin.proven,sum.map_or("",|sum|sum.owner.as_str()),sum.map_or(-1.0,|sum|sum.arguments.len() as f64),stored.is_some_and(|origin|origin.proven),&profile,&member.value);
        }
        if plan.is_empty(){plan=rules::read_rust_generic_type_alias_header_constructor_plan(self.header_discovery,sum.is_some_and(|sum|sum.provisional),origin.ty.as_ref().map_or("",Type::kind),origin.wrappers.len() as f64,owner.map_or("",|owner|owner.kind.as_str()),&owner.map(|owner|owner.path.join("::")).unwrap_or_default(),owner.map_or(-1.0,|owner|owner.arguments.len() as f64),sum.map_or("",|sum|sum.owner.as_str()),sum.map_or(-1.0,|sum|sum.arguments.len() as f64),stored.map_or(-1.0,|origin|origin.wrappers.len() as f64),&member.value);}
        if plan.is_empty(){if !explicit{return Ok(None);}return Err(self.alias_error("generic alias associated construction needs its actual admitted vector or standard sum constructor",Some(&row)));}
        let arity=plan[1].parse::<usize>().expect("shared generic alias constructor arity");
        let constructor_path=sum.and_then(|sum|sum.constructor_paths.get(&member.value)).cloned();
        if plan[0]=="vector constructor"{
            self.cursor.expect("(",Some("generic alias Vec constructor"))?;
            let close=self.cursor.expect(")",Some("generic alias Vec constructor"))?;
            let element=match origin.ty.as_ref(){Some(Type::Array {element,..})=>element.as_ref().clone(),_=>unreachable!("shared vector constructor policy proved the actual element")};
            let node=SExpr::new(SNode::Array {items:Vec::new(),element:Some(element)},Some(Span::new(start.start,close.end)));
            origin.path=path.to_vec();self.origins.mark_vector_constructor(&node,origin);return Ok(Some(node));
        }
        let constructor_path=constructor_path.ok_or_else(||self.alias_error("the generic alias constructor needs its actual registered variant path",Some(&row)))?;
        let mut end=member.end;let mut args=Vec::new();
        if arity==0{
            if self.cursor.is("("){return Err(self.alias_error("a nullary generic alias variant is a value without call arguments",Some(&row)));}
        }else{
            self.cursor.expect("(",Some("generic alias standard sum constructor"))?;
            let callee=SExpr::new(SNode::Name {path:constructor_path.clone()},Some(Span::new(start.start,member.end)));
            args=self.contextual_call_arguments(path,&callee)?;
            end=self.cursor.expect(")",Some("generic alias standard sum constructor"))?.end;
            if args.len()!=arity{return Err(self.alias_error("generic alias constructor arguments differ from their actual source fields",Some(&row)));}
        }
        let span=Some(Span::new(start.start,end));self.origins.call(&constructor_path,&args,path,span)?;
        let name=SExpr::new(SNode::Name {path:constructor_path},span);
        let node=if args.is_empty(){name}else{SExpr::new(SNode::App {func:Box::new(name),args},span)};
        let arguments=match &node.node{SNode::App {args,..}=>args.as_slice(),_=>&[]};
        self.capture_standard_sum_constructor(&node,&origin,arguments,path,span)?;Ok(Some(node))
    }
}
