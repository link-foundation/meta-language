//! Owned, opt-in facts from actual Rust function source headers.
//! Source owner decisions use shared rules; datatype callbacks expose actual
//! constructor layouts and their existing retained field-signature facet.
use super::diagnostics::{Result, type_error};
use super::ir::DataDecl;
use super::rust::indirection_rules::accept_rust_source_lifetime_binding;
use super::rust_source_owner_retention_rules::{accept_rust_retained_function_owner,accept_rust_retained_source_signature_model};
use super::source_signature_rules::{accept_rust_source_signature_model,accept_rust_source_signature_owner_kind,accept_rust_source_signature_primitive,accept_rust_source_signature_wrapper};
use super::rust_standard_sum_signature_rules::read_rust_standard_sum_signature_argument_constructors;
use super::types::Type;
use serde::{Deserialize, Serialize};

/// A resolved owner of a source type, including owned generic children.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceOwner {
    /// Primitive, declaration, standard library, tuple, unit or slice.
    pub kind: String,
    /// Actual resolved owner segments, never a source text envelope.
    pub path: Vec<String>,
    /// Parsed child type owners in source argument order.
    pub arguments: Vec<Self>,
}

/// One owned source parameter or result type and its erased qualifiers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceTypeSignature {
    /// The value type resolved by the existing checker.
    #[serde(rename = "type")]
    pub ty: Type,
    /// Pointer layers in their actual source order.
    pub wrappers: Vec<String>,
    /// One named or elided lifetime per reference layer.
    pub lifetimes: Vec<Option<String>>,
    /// Actual header owner, including resolved declaration identities.
    pub owner: SourceOwner,
}

/// A source function signature, owned by its checked declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceSignature {
    /// Explicit selected source interpretation.
    pub model: String,
    /// The source header language.
    pub language: String,
    /// Actual parameter positions, matched to the function parameters.
    pub parameters: Vec<SourceTypeSignature>,
    /// Actual result position, matched to the function result.
    pub result: SourceTypeSignature,
    /// Parsed function lifetime binders in declaration order.
    pub lifetime_bindings: Vec<String>,
}

fn fail(message: &str) -> super::diagnostics::TranslationError { type_error(message,None) }

struct SignatureValidation<'data,'call> {
    signature: &'call SourceSignature,
    resolve_type: &'call mut dyn FnMut(&Type) -> Result<Type>,
    resolve_data: &'call mut dyn FnMut(&Type) -> Option<&'data DataDecl>,
    visits: usize,
}
impl SignatureValidation<'_,'_> {
    fn visit(&mut self, depth: usize) -> Result<()> {
        self.visits = self.visits.saturating_add(1);
        if self.visits > 100_000 || depth > 256 { return Err(fail("an owned Rust source signature exceeds its resource limit")); }
        Ok(())
    }
    fn value_type(&mut self, ty: &Type, depth: usize) -> Result<()> {
        self.visit(depth)?;
        if matches!(ty,Type::Function {..}) && !accept_rust_retained_source_signature_model(&self.signature.model) { return Err(fail("an owned Rust source type has an unsupported value type")); }
        for child in ty.children() { self.value_type(child,depth+1)?; }
        Ok(())
    }
    fn owner_equal(&mut self, left: &SourceOwner, right: &SourceOwner) -> Result<()> {
        let mut pending=std::collections::VecDeque::from([(left,right)]);
        while let Some((left,right))=pending.pop_front() {
            // Both trees have already passed recursive owner depth checking.
            // Comparison shares the enclosing visit budget in the same order.
            self.visit(0)?;
            if left.kind!=right.kind || left.path!=right.path || left.arguments.len()!=right.arguments.len() { return Err(fail("an owned Rust standard sum changes a retained payload owner")); }
            pending.extend(left.arguments.iter().zip(&right.arguments));
        }
        Ok(())
    }
    fn owner(&mut self, owner: &SourceOwner, ty: &Type, depth: usize) -> Result<()> {
        self.visit(depth)?;
        let model = self.signature.model.as_str();
        if matches!(ty,Type::Function {..}) && !accept_rust_retained_source_signature_model(model) { return Err(fail("an owned Rust source type has an unsupported value type")); }
        if !(accept_rust_source_signature_owner_kind(&owner.kind) || accept_rust_retained_source_signature_model(model) && owner.kind == "function") || owner.path.iter().any(String::is_empty) { return Err(fail("an owned Rust source type has an invalid owner")); }
        let path = owner.path.join("::");
        match owner.kind.as_str() {
            "function" => {
                let Type::Function {params,ret} = ty else { return Err(fail("an owned Rust source function differs from its value type")); };
                if !accept_rust_retained_function_owner(model,&owner.kind,ty.kind(),owner.path.len() as f64,owner.arguments.len() as f64,params.len() as f64,false) { return Err(fail("an owned Rust source function differs from its value type")); }
                for (child,actual) in owner.arguments.iter().zip(params.iter().chain(std::iter::once(ret.as_ref()))) { self.owner(child,actual,depth+1)?; }
            }
            "primitive" => {
                let (bits,signed) = match ty { Type::Fixed {bits,signed} => (bits.to_string(),*signed), _ => (String::new(),false) };
                if owner.path.len()!=1 || !owner.arguments.is_empty() || !accept_rust_source_signature_primitive(ty.kind(),&path,&bits,signed) { return Err(fail("an owned Rust source primitive differs from its value type")); }
            }
            "declaration" => {
                if owner.path.is_empty() || !owner.arguments.is_empty() || !matches!(ty,Type::Data {name} if *name == owner.path.join(".")) { return Err(fail("an owned Rust source declaration differs from its value type")); }
            }
            "unit" => {
                if !owner.path.is_empty() || !owner.arguments.is_empty() || *ty != Type::Unit { return Err(fail("an owned Rust source unit differs from its value type")); }
            }
            "tuple" => {
                let Type::Tuple {elements} = ty else { return Err(fail("an owned Rust source tuple differs from its value type")); };
                if !owner.path.is_empty() || owner.arguments.len()!=elements.len() { return Err(fail("an owned Rust source tuple differs from its value type")); }
                for (child,element) in owner.arguments.iter().zip(elements) { self.owner(child,element,depth+1)?; }
            }
            "slice" | "library" if owner.kind=="slice" || path=="std::vec::Vec" => {
                let Type::Array {element,..} = ty else { return Err(fail("an owned Rust source array differs from its value type")); };
                if owner.kind=="slice" && !owner.path.is_empty() || owner.arguments.len()!=1 { return Err(fail("an owned Rust source array differs from its value type")); }
                self.owner(&owner.arguments[0],element,depth+1)?;
            }
            "library" if path=="std::boxed::Box" || path=="alloc::boxed::Box" => {
                if owner.arguments.len()!=1 { return Err(fail("an owned Rust source box has invalid arguments")); }
                self.owner(&owner.arguments[0],ty,depth+1)?;
            }
            "library" if path=="std::string::String" => {
                if !owner.arguments.is_empty() || *ty!=Type::String { return Err(fail("an owned Rust source string differs from its value type")); }
            }
            "library" if path=="std::option::Option" || path=="std::result::Result" => {
                let Type::Data {name} = ty else { return Err(fail("an owned Rust standard sum needs its actual datatype encoding")); };
                let declaration = (self.resolve_data)(ty).filter(|declaration|declaration.full_name == *name).ok_or_else(||fail("an owned Rust standard sum needs its complete source field signatures"))?;
                let fields = declaration.source_field_signatures.as_deref().ok_or_else(||fail("an owned Rust standard sum needs its complete source field signatures"))?;
                let names = declaration.ctors.iter().map(|ctor|ctor.name.clone()).collect::<Vec<_>>();
                let counts = declaration.ctors.iter().map(|ctor|ctor.fields.len() as f64).collect::<Vec<_>>();
                let selected = read_rust_standard_sum_signature_argument_constructors(&path,&names,&counts);
                if selected.is_empty() || selected.len()!=owner.arguments.len() { return Err(fail("an owned Rust standard sum differs from its complete constructor inventory")); }
                if fields.model!="Rust data type field signatures 1" || fields.language!=self.signature.language || fields.data_type_name!=*name || fields.ctors.len()!=declaration.ctors.len() { return Err(fail("an owned Rust standard sum has inconsistent source field ownership")); }
                for (retained,actual) in fields.ctors.iter().zip(&declaration.ctors) {
                    if retained.name!=actual.name || retained.fields.len()!=actual.fields.len() { return Err(fail("an owned Rust standard sum has inconsistent retained constructors")); }
                }
                for (argument,constructor) in owner.arguments.iter().zip(selected) {
                    // The source-generated selector proves these exact payload
                    // coordinates from complete actual constructor columns.
                    let constructor = constructor as usize;
                    let actual = &declaration.ctors[constructor].fields[0].ty;
                    let retained = &fields.ctors[constructor].fields[0];
                    self.position(retained,actual,depth+1)?;
                    if !retained.wrappers.is_empty() || !retained.lifetimes.is_empty() { return Err(fail("an owned Rust standard sum payload requires a direct source origin")); }
                    self.owner(argument,actual,depth+1)?;
                    self.owner_equal(argument,&retained.owner)?;
                }
            }
            _ => return Err(fail("an owned Rust source type has an unsupported owner")),
        }
        Ok(())
    }
    fn position(&mut self, position: &SourceTypeSignature, expected: &Type, depth: usize) -> Result<()> {
        self.visit(depth)?;
        self.value_type(&position.ty,0)?;
        let ty = (self.resolve_type)(&position.ty)?;
        self.value_type(&ty,0)?;
        if !ty.same(expected) { return Err(fail("an owned Rust source signature differs from its function type")); }
        if position.wrappers.iter().any(|wrapper|!accept_rust_source_signature_wrapper(wrapper)) || position.lifetimes.len()!=position.wrappers.iter().filter(|wrapper|*wrapper=="reference").count() || position.lifetimes.iter().flatten().any(|lifetime|lifetime!="static" && lifetime!="_" && !self.signature.lifetime_bindings.contains(lifetime)) { return Err(fail("an owned Rust source type has inconsistent wrapper or lifetime facts")); }
        self.owner(&position.owner,&ty,depth+1)?;
        let mut boxed = 0;
        let mut leaf = &position.owner;
        while leaf.kind=="library" && ["std::boxed::Box","alloc::boxed::Box"].contains(&leaf.path.join("::").as_str()) { boxed+=1;leaf=&leaf.arguments[0]; }
        if boxed!=position.wrappers.iter().filter(|wrapper|*wrapper=="box").count() || ((leaf.kind=="slice" || leaf.kind=="primitive" && leaf.path.first().is_some_and(|part|part=="str")) && !position.wrappers.iter().any(|wrapper|wrapper=="reference")) { return Err(fail("an owned Rust source type has inconsistent owner and wrapper facts")); }
        Ok(())
    }
}

/// Check a standalone signature. Standard sums require the contextual callback
/// API because their names alone do not prove their actual constructor layout.
///
/// # Errors
/// Refuses mismatched headers, unknown owners and unsupported qualifier facts.
pub fn validate_source_signature(signature:&SourceSignature,parameters:&[Type],result:&Type,language:&str)->Result<()> {
    validate_source_signature_with_data(signature,parameters,result,language,|ty|Ok(ty.clone()),|_|None)
}

/// Check positions against actual resolved datatype layouts and retained fields.
/// The resolver supplies declarations, never saved source or a cached emission.
///
/// # Errors
/// Refuses incomplete or mismatched data layouts, source owners and qualifiers.
pub fn validate_source_signature_with_data<'data>(signature:&SourceSignature,parameters:&[Type],result:&Type,language:&str,mut resolve_type:impl FnMut(&Type)->Result<Type>,mut resolve_data:impl FnMut(&Type)->Option<&'data DataDecl>)->Result<()> {
    if signature.model.is_empty() || !(accept_rust_source_signature_model(&signature.model) || accept_rust_retained_source_signature_model(&signature.model)) || signature.language!="Rust" || language!="Rust" { return Err(fail("an owned Rust source signature has an unsupported model or language")); }
    let unique:std::collections::HashSet<_> = signature.lifetime_bindings.iter().collect();
    if signature.parameters.len()!=parameters.len() || unique.len()!=signature.lifetime_bindings.len() || signature.lifetime_bindings.iter().any(|name|!accept_rust_source_lifetime_binding(name,false,false)) { return Err(fail("an owned Rust source signature has inconsistent parameter or lifetime bindings")); }
    let mut validation=SignatureValidation {signature,resolve_type:&mut resolve_type,resolve_data:&mut resolve_data,visits:0};
    for (position,expected) in signature.parameters.iter().zip(parameters).chain(std::iter::once((&signature.result,result))) { validation.position(position,expected,0)?; }
    Ok(())
}
