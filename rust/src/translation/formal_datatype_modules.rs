//! Target placement transport. The shared JavaScript policy decides prefixes;
//! this does not establish ownership, positivity, defaults or emit aliases.
use std::collections::HashMap;
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
    let depths=super::formal_datatype_module_rules::plan_formal_datatype_module_prefixes(&paths,&groups);
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
        let surface=crate::translation::rust::parse_rust("mod left { enum A { Value(bool) } } mod right { enum B { Value(bool) } } fn main() {}").unwrap();
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
