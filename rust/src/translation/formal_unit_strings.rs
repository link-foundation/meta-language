//! Mechanical adapters over actual IR and shared formal UTF-16 source rules.
use super::formal_string_unit_rules as rules;
use super::{
    Language,
    diagnostics::{Result, unsupported},
    ir::{Expr, LitValue, Node, Program},
    surface::BinaryOp,
    types::Type,
};

pub(super) fn requires(program: &Program) -> bool {
    if program.utf16_strings.unwrap_or(false) {
        return true;
    }
    if program.source_language != Language::JavaScript && program.source_language != Language::Rust
    {
        return false;
    }
    fn walk(value: &serde_json::Value, language: Language) -> bool {
        match value {
            serde_json::Value::Array(values) => values.iter().any(|value| walk(value, language)),
            serde_json::Value::Object(fields) => {
                let text = |key: &str| {
                    fields
                        .get(key)
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("")
                };
                let receiver = fields
                    .get("object")
                    .and_then(|value| value.get("type"))
                    .and_then(|value| value.get("kind"))
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");
                (language==Language::JavaScript&&(rules::require_formal_string_unit_representation("type", text("kind"), "")
                    || rules::require_formal_string_unit_representation(
                        text("k"),
                        text("op"),
                        receiver,
                    )))
                    || crate::translation::rust_string_byte_view_rules::accept_rust_string_byte_kernel_unit_requirement(language.as_str(),text("k"),text("op"),receiver)
                    || fields
                        .iter()
                        .any(|(key, value)| key != "span" && walk(value,language))
            }
            _ => false,
        }
    }
    walk(
        &serde_json::to_value(program).expect("checked source program retains its real IR"),
        program.source_language,
    )
}

pub(super) trait Emitter {
    fn unit_unicode_case(&mut self);
    fn unit_enabled(&self) -> bool;
    fn unit_target(&self) -> &'static str;
    fn unit_expr(&mut self, value: &Expr, depth: usize) -> Result<String>;
    fn unit_text(&mut self, value: &Expr, depth: usize, console: bool) -> Result<String>;
    fn unit_helper(&mut self, name: &'static str);
}

fn inclusion_equality(
    target: &str,
    ty: &Type,
    units: bool,
    left: &str,
    right: &str,
) -> Result<String> {
    if let Type::Option { element, absence } = ty {
        let compare = inclusion_equality(target, element, units, "ml_left", "ml_right")?;
        return Ok(
            super::optional_value_rules::render_optional_strict_equality(
                target, "option", absence, "option", absence, &compare, left, right, false,
            ),
        );
    }
    let signed = matches!(ty, Type::Fixed { signed: true, .. });
    let result = rules::render_formal_primitive_same_value_zero(
        target,
        ty.kind(),
        signed,
        units,
        left,
        right,
    );
    if result.is_empty() {
        return Err(unsupported(
            "formal array includes element",
            "the element needs its source SameValueZero interpretation",
            None,
        ));
    }
    Ok(result)
}
pub(super) fn literal(target: &str, text: &str) -> String {
    rules::render_formal_string_unit_literal(
        target,
        &text
            .encode_utf16()
            .map(|unit| unit.to_string())
            .collect::<Vec<_>>(),
    )
}
pub(super) fn length(target: &str, value: &str) -> String {
    if target == "Lean" {
        format!("(Float.ofNat ({value}).size)")
    } else {
        format!("(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length {value}))))")
    }
}
pub(super) fn truth(target: &str, value: &str) -> String {
    if target == "Lean" {
        format!("(!({value}).isEmpty)")
    } else {
        format!("(match {value} with nil => false | _ => true end)")
    }
}
fn policy(ty: &Type) -> &str {
    if let Type::Option { absence, .. } = ty {
        absence
    } else {
        ""
    }
}

fn join_text<E: Emitter>(emitter: &mut E, name: &str, ty: &Type, depth: usize) -> Result<String> {
    let target = emitter.unit_target();
    if *ty == Type::Unit {
        return Ok(literal(target, ""));
    }
    if let Type::Option { element, absence } = ty {
        let text = join_text(emitter, "ml_present", element, depth)?;
        let empty = literal(target, "");
        return Ok(if target == "Lean" {
            format!(
                "(match {name} with | {} => {text} | _ => {empty})",
                if absence == "both" {
                    "some (some ml_present)"
                } else {
                    "some ml_present"
                }
            )
        } else {
            format!(
                "(match {name} with | {} => {text} | _ => {empty} end)",
                if absence == "both" {
                    "Some (Some ml_present)"
                } else {
                    "Some ml_present"
                }
            )
        });
    }
    if *ty == Type::String {
        return Ok(name.to_owned());
    }
    emitter.unit_text(&Expr::var(name, ty.clone()), depth, false)
}

pub(super) fn expression<E: Emitter>(
    emitter: &mut E,
    value: &Expr,
    depth: usize,
) -> Result<Option<String>> {
    if let Node::Method { object, op, args } = &value.node {
        if let Type::Array { element, .. } = &object.ty
            && op == "includes"
        {
            emitter.unit_helper("utf16Text");
            if emitter.unit_target() == "Rocq" {
                emitter.unit_helper("floatIndex");
            }
            let equality = inclusion_equality(
                emitter.unit_target(),
                element,
                emitter.unit_enabled(),
                "ml_candidate",
                "ml_search",
            )?;
            let object = emitter.unit_expr(object, depth)?;
            let search =
                emitter.unit_expr(args[0].as_ref().expect("checked includes search"), depth)?;
            let position =
                emitter.unit_expr(args[1].as_ref().expect("checked includes position"), depth)?;
            return Ok(Some(rules::render_formal_array_inclusion(
                emitter.unit_target(),
                &object,
                &search,
                &position,
                &equality,
            )));
        }
        if object.ty.kind() == "array" && op == "slice" {
            emitter.unit_helper("utf16Text");
            if emitter.unit_target() == "Rocq" {
                emitter.unit_helper("floatIndex");
            }
            let object = emitter.unit_expr(object, depth)?;
            let begin = emitter.unit_expr(args[0].as_ref().expect("checked slice begin"), depth)?;
            let end = emitter.unit_expr(args[1].as_ref().expect("checked slice end"), depth)?;
            return Ok(Some(rules::render_formal_array_slice(
                emitter.unit_target(),
                &object,
                &begin,
                &end,
            )));
        }
    }
    if !emitter.unit_enabled() {
        return Ok(None);
    }
    let target = emitter.unit_target();
    let text=match &value.node{
  Node::Lit{value:LitValue::Text(text)} if value.ty==Type::String=>literal(target,text),
  Node::OutNil=>if target=="Lean"{"([] : List (Array Nat))"}else{"(@nil (list N))"}.to_owned(),
  Node::Math{op,args}=>{
   if super::javascript_string_unit_rules::accept_linked_java_script_string_unit_construction("math",op){
    let text=emitter.unit_expr(&args[0],depth)?;rules::render_formal_string_units_from_numbers(target,&text)
   }else if super::string_construction_rules::accept_linked_java_script_string_construction("math",op){
    let text=emitter.unit_expr(&args[0],depth)?;
    let text=super::string_construction_rules::render_java_script_string_construction(target,op,&text);
    if super::string_construction_rules::read_java_script_string_construction_form(op)=="characters"{rules::render_formal_string_units_from_numbers(target,&text)}else{text}
   }else if super::string_number_rules::accept_linked_java_script_number_conversion("math",op)&&args[0].ty==Type::String{
    emitter.unit_helper("stringNumber");let text=emitter.unit_expr(&args[0],depth)?;
    format!("(MLStringNumber.convertUnits {})",rules::render_formal_string_unit_numbers(target,&text))
   }else if super::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op)&&super::big_integer_conversion_rules::read_java_script_big_integer_result_operation(op)=="string-compare"{
    emitter.unit_helper("bigIntegerConversion");let text=emitter.unit_expr(&args[0],depth)?;let other=emitter.unit_expr(&args[1],depth)?;
    format!("(MLBigIntegerConversion.compareUnits {} {})",rules::render_formal_string_unit_numbers(target,&text),other)
   }else if super::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op)&&super::big_integer_conversion_rules::read_java_script_big_integer_result_operation(op)=="string"{
    emitter.unit_helper("bigIntegerConversion");let text=emitter.unit_expr(&args[0],depth)?;
    format!("(MLBigIntegerConversion.convertUnits {})",rules::render_formal_string_unit_numbers(target,&text))
   }else{return Ok(None);}
  }
  Node::Length{array} if array.ty==Type::String=>{let text=emitter.unit_expr(array,depth)?;length(target,&text)}
  Node::OptionalProject{object,..} if matches!(&object.ty,Type::Option{element,..} if **element==Type::String)=>{
   let text=emitter.unit_expr(object,depth)?;
   super::optional_value_rules::render_optional_projection(target,policy(&object.ty),&text,"","ml_value",&length(target,"ml_value"),"")
  }
  Node::StringPoints{object}=>{let text=emitter.unit_expr(object,depth)?;rules::render_formal_string_unit_operation(target,"points",&[text])}
  Node::Binary{op:BinaryOp::Concat,left,right,..}=>{let left=emitter.unit_expr(left,depth)?;let right=emitter.unit_expr(right,depth)?;if target=="Lean"{format!("({left} ++ {right})")}else{format!("(List.app {left} {right})")}}
  Node::Binary{op,left,right,domain:Some(Type::String),..}=>{
   let left=emitter.unit_expr(left,depth)?;let right=emitter.unit_expr(right,depth)?;rules::render_formal_string_unit_comparison(target,op.name(),&left,&right)
  }
  Node::StringTest{op,string,search}=>{
   let left=emitter.unit_expr(string,depth)?;let right=emitter.unit_expr(search,depth)?;
   let comparison=match op.as_str(){"unicodeCodeUnitLessThan"=>"lt","unicodeCodeUnitLessThanOrEqual"=>"le","unicodeCodeUnitGreaterThan"=>"gt","unicodeCodeUnitGreaterThanOrEqual"=>"ge",_=>""};
   let result=if comparison.is_empty(){rules::render_formal_string_unit_operation(target,op,&[left,right])}else{rules::render_formal_string_unit_comparison(target,comparison,&left,&right)};
   if result.is_empty(){return Ok(None);}result
  }
  Node::StringMap{op,string}=>{
   if super::unicode_case_emission_rules::accept_unicode_case_mapping_operation(op){emitter.unit_unicode_case();}
   let text=emitter.unit_expr(string,depth)?;let result=rules::render_formal_string_unit_operation(target,op,&[text]);if result.is_empty(){return Ok(None);}result
  }
  Node::Method{object,op,args} if object.ty==Type::String=>{
   let mut values=vec![emitter.unit_expr(object,depth)?];
   for value in args{values.push(if let Some(value)=value{emitter.unit_expr(value,depth)?}else{String::new()});}
   let result=rules::render_formal_string_unit_operation(target,op,&values);
   if result.is_empty(){return Err(unsupported(&format!("formal UTF16 string {op}"),"the operation needs its complete shared code-unit definition",value.span));}result
  }
  Node::OptionalText{arg,console}=>{
   let Type::Option{element,..}=&arg.ty else{unreachable!("actual checked optional text")};
   let mut present=if **element==Type::String{"ml_present".to_owned()}else if **element==Type::Unit{literal(target,"undefined")}else{emitter.unit_text(&Expr::var("ml_present",(**element).clone()),depth,*console)?};
   if *console&&matches!(element.kind(),"int"|"nat"|"fixed"){let suffix=literal(target,"n");present=if target=="Lean"{format!("({present} ++ {suffix})")}else{format!("(List.app {present} {suffix})")};}
   let text=emitter.unit_expr(arg,depth)?;
   super::optional_value_rules::render_optional_match(target,policy(&arg.ty),&text,"ml_present",&present,&literal(target,"null"),&literal(target,"undefined"))
  }
  Node::OptionalTypeOf{arg,present}=>{
   let text=emitter.unit_expr(arg,depth)?;
   super::optional_value_rules::render_optional_match(target,policy(&arg.ty),&text,"ml_present",&literal(target,present),&literal(target,"object"),&literal(target,"undefined"))
  }
  Node::Method{object,op,args} if matches!(&object.ty,Type::Array{..})&&op=="join"=>{
   let Type::Array{element,..}=&object.ty else{unreachable!("actual array receiver")};
   let value=emitter.unit_expr(object,depth)?;let converted=join_text(emitter,"ml_value",element,depth)?;
   let values=if target=="Lean"{format!("(({value}).map (fun ml_value => {converted}))")}else{format!("(List.map (fun ml_value => {converted}) {value})")};
   let separator=if let Some(Some(value))=args.first(){emitter.unit_expr(value,depth)?}else{literal(target,",")};
   rules::render_formal_string_unit_operation(target,"join",&[values,separator])
  }
  _=>return Ok(None),
 };
    Ok(Some(text))
}
