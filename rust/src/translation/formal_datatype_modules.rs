//! Target placement transport. The shared JavaScript policy decides prefixes;
//! this does not establish ownership, positivity, defaults or emit aliases.
use std::collections::{HashMap,HashSet};
use super::diagnostics::{Result,unsupported};
use super::ir::{Decl,Program};

#[derive(Clone,Debug)]
pub(crate) struct FormalDatatypeAlias {
    pub source_full_name:String,
    pub target_full_name:String,
    pub source_name:String,
    pub source_module_path:Vec<String>,
    pub target_module_path:Vec<String>,
    pub constructors:Vec<String>,
}

pub(crate) struct FormalDatatypeModules<'a> {
    pub source_program:&'a Program,
    pub program:Program,
    pub aliases:Vec<FormalDatatypeAlias>,
}

fn scope_key(path:&[String])->String {
    path.iter().map(|part|format!("{}:{part}",part.encode_utf16().count())).collect::<Vec<_>>().concat()
}

pub(crate) fn formal_datatype_module_dependencies(plan:&FormalDatatypeModules<'_>)->Result<HashMap<String,HashSet<String>>> {
    let mut scopes=HashMap::<String,HashSet<String>>::new();
    let mut target_scopes=HashMap::<String,Vec<String>>::new();
    for alias in &plan.aliases {
        let boundary=&alias.source_module_path[..alias.target_module_path.len()+1];
        scopes.entry(scope_key(boundary)).or_default().insert(alias.target_full_name.clone());
        for depth in 1..=alias.target_module_path.len(){let path=alias.target_module_path[..depth].to_vec();let key=scope_key(&path);if !target_scopes.contains_key(&key)&&target_scopes.len()>=8192{return Err(unsupported("target declaration dependencies","datatype scope prerequisites exceed their finite traversal budget",None));}target_scopes.insert(key,path);}
    }
    if !plan.aliases.is_empty(){for entry in &plan.program.declarations{if matches!(entry,Decl::Data(_)){for depth in 1..=entry.module_path().len(){
        let path=entry.module_path()[..depth].to_vec();let key=scope_key(&path);
        if !target_scopes.contains_key(&key)&&target_scopes.len()>=8192{return Err(unsupported("target declaration dependencies","datatype scope prerequisites exceed their finite traversal budget",None));}
        target_scopes.insert(key,path);
    }}}}
    let known=plan.program.declarations.iter().map(|entry|(entry.full_name(),entry)).collect::<HashMap<_,_>>();
    let mut external_scopes=HashMap::<String,HashSet<String>>::new();let mut visits=0usize;
    for(key,path)in target_scopes{
        let mut pending=plan.program.declarations.iter().filter(|entry|matches!(entry,Decl::Data(_))&&entry.module_path().starts_with(&path)).collect::<Vec<_>>();
        let mut seen=HashSet::new();let mut outside=HashSet::new();
        while let Some(entry)=pending.pop(){
            if !seen.insert(entry.full_name()){continue;}
            visits+=1;if visits>1048576{return Err(unsupported("target declaration dependencies","datatype scope prerequisites exceed their finite traversal budget",None));}
            for name in super::emit_common::dependencies(entry){
                let Some(dependency)=known.get(name.as_str()).copied().filter(|entry|matches!(entry,Decl::Data(_)))else{continue;};
                if !dependency.module_path().starts_with(&path){outside.insert(name);}
                if !seen.contains(dependency.full_name()){pending.push(dependency);}
            }
        }
        external_scopes.insert(key,outside);
    }
    let mut result=HashMap::new();let mut edges=0usize;
    for entry in &plan.program.declarations {
        let mut required=HashSet::new();
        for depth in 1..=entry.module_path().len(){let key=scope_key(&entry.module_path()[..depth]);for owners in [scopes.get(&key),external_scopes.get(&key)].into_iter().flatten(){for owner in owners{
            if !known.contains_key(owner.as_str()){return Err(unsupported("target declaration dependencies","extra ordering dependencies require actual declared owners",None));}
            if required.insert(owner.clone()){edges+=1;if edges>131072{return Err(unsupported("target declaration dependencies","extra ordering dependencies require bounded actual declaration edges",None));}}
        }}}
        if !required.is_empty(){result.insert(entry.full_name().to_owned(),required);}
    }
    Ok(result)
}

pub(crate) fn plan_formal_datatype_modules(program:&Program)->Result<FormalDatatypeModules<'_>> {
    let data=program.declarations.iter().filter_map(|entry|match entry{Decl::Data(data)=>Some(data),_=>None}).collect::<Vec<_>>();
    let positions=data.iter().enumerate().map(|(index,entry)|(entry.full_name.as_str(),index)).collect::<HashMap<_,_>>();
    if positions.len()!=data.len(){return Err(unsupported("formal datatype modules","nominal declarations need distinct actual identities",None));}
    let mut groups=(0..data.len()).map(|index|index as f64).collect::<Vec<_>>();
    for(name,members)in super::emit_common::datatype_groups(program)? {
        if members.first().is_none_or(|entry|entry.full_name()!=name){continue;}
        let indices=members.iter().map(|entry|positions.get(entry.full_name()).copied().ok_or_else(||unsupported("formal datatype modules","a mutual group has an unknown nominal declaration",None))).collect::<Result<Vec<_>>>()?;
        let representative=*indices.iter().min().ok_or_else(||unsupported("formal datatype modules","a mutual group has no actual owner",None))?;
        for index in indices{groups[index]=representative as f64;}
    }
    let paths=data.iter().map(|entry|entry.module_path.clone()).collect::<Vec<_>>();
    let prefixes=super::formal_datatype_module_rules::plan_formal_datatype_module_prefixes(&paths,&groups);
    let depths=super::formal_datatype_module_rules::plan_formal_datatype_module_closure(&paths,&prefixes);
    if depths.len()!=data.len(){return Err(unsupported("formal datatype modules","actual module paths exceed the bounded placement profile",None));}
    let mut aliases=Vec::new();
    let mut projected=program.clone();
    let declaration_positions=program.declarations.iter().enumerate().filter_map(|(index,entry)|match entry{Decl::Data(_)=>Some(index),_=>None}).collect::<Vec<_>>();
    for(index,entry)in data.iter().enumerate(){
        let depth=depths[index];
        if !depth.is_finite()||depth.fract()!=0.0||depth<0.0||depth>entry.module_path.len() as f64{return Err(unsupported("formal datatype modules","the shared placement plan has an invalid actual prefix",None));}
        let depth=depth as usize;
        if depth==entry.module_path.len(){continue;}
        let target_module_path=entry.module_path[..depth].to_vec();
        let actual=match projected.declarations.get_mut(declaration_positions[index]){Some(Decl::Data(data))if data.full_name==entry.full_name=>data,_=>return Err(unsupported("formal datatype modules","a target placement lost its actual nominal declaration",None))};
        actual.module_path=target_module_path.clone();
        aliases.push(FormalDatatypeAlias{source_full_name:entry.full_name.clone(),target_full_name:entry.full_name.clone(),source_name:entry.name.clone(),source_module_path:entry.module_path.clone(),target_module_path,constructors:entry.ctors.iter().map(|ctor|ctor.name.clone()).collect()});
    }
    Ok(FormalDatatypeModules{source_program:program,program:projected,aliases})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn placement_preserves_actual_source_ids_and_explicit_public_aliases(){
        // Structural projection control, not a claim that unguarded Rust
        // recursive fields form a valid source-sized declaration.
        let surface=crate::translation::rust::parse_rust("mod left { fn unrelated()->bool { true } enum A { Value(bool) } } mod right { enum B { Value(bool) } } fn main() {}").unwrap();
        let mut original=crate::translation::check::check_program(&surface).unwrap();
        for entry in &mut original.declarations {
            if let Decl::Data(data)=entry {
                let target=if data.full_name=="left.A"{"right.B"}else{"left.A"};
                data.ctors[0].fields[0].ty=crate::translation::types::Type::Data{name:target.to_owned()};
            }
        }
        let saved=original.clone();
        let projected=plan_formal_datatype_modules(&original).unwrap();
        assert_eq!(original,saved);
        assert_eq!(projected.source_program,&original);
        assert_eq!(projected.aliases.len(),2);
        for alias in &projected.aliases {
            assert_eq!(alias.source_full_name,alias.target_full_name);
            assert!(alias.target_module_path.is_empty());
            assert_eq!(alias.constructors,vec!["Value".to_owned()]);
            assert_eq!(projected.program.declaration(&alias.target_full_name).unwrap().module_path(),&[] as &[String]);
            assert!(!original.declaration(&alias.source_full_name).unwrap().module_path().is_empty());
        }
        assert!(crate::translation::emit_common::order_declarations(&original,true).is_err());
        assert!(crate::translation::emit_common::order_declarations(&projected.program,true).is_ok());
        let extra=formal_datatype_module_dependencies(&projected).unwrap();
        assert_eq!(extra["left.unrelated"],HashSet::from(["left.A".to_owned()]));
        let order=crate::translation::emit_common::order_declarations_with_dependencies(&projected.program,true,&extra).unwrap();
        assert_eq!(order.last().unwrap().full_name(),"left.unrelated");
        let mut state=crate::translation::emit_common::EmitState::new(&original,crate::translation::Language::Rocq,str::to_owned,&[],crate::translation::emit_common::EmitOptions{ctor_style:crate::translation::emit_common::CtorStyle::Module,modules_share_term_space:true,..Default::default()});
        let public=projected.aliases.iter().flat_map(|alias|[state.reference(&alias.source_full_name,"."),state.ctor_ref(&alias.source_full_name,"Value",".")]).collect::<Vec<_>>();
        state.relocate_datatypes(&projected.program,&projected.aliases).unwrap();
        for alias in &projected.aliases {
            assert!(!public.contains(&state.reference(&alias.target_full_name,".")));
            assert!(!public.contains(&state.ctor_ref(&alias.target_full_name,"Value",".")));
        }
        assert!(state.relocate_datatypes(&projected.program,&projected.aliases).is_err());
    }
}
