#[cfg(test)]
mod deferred_source_controls {
    use super::*;
    const MAP: &str = "struct Node(u8);struct Field{ty:Node}fn children<'a>(fields:&'a Vec<Field>)->Vec<&'a Node>{fields.iter().map(|field|&field.ty).collect()}fn main(){let fields=vec![Field{ty:Node(7u8)}];println!(\"{}\",children(&fields).len());}";
    const CHAIN: &str = "struct Node(u8);fn children<'a>(values:&'a Vec<Node>,tail:&'a Box<Node>)->Vec<&'a Node>{values.iter().chain(std::iter::once(tail.as_ref())).collect()}fn main(){let values=vec![Node(7u8)];let tail=Box::new(Node(8u8));println!(\"{}\",children(&values,&tail).len());}";
    const TYPE_CHILDREN: &str = r#"enum Type{Array{element:Box<Self>},Option{element:Box<Self>},Set{element:Box<Self>},Cell{element:Box<Self>},Tuple{elements:Vec<Self>},Union{alternatives:Vec<Self>},Record{fields:Vec<TypeField>},Map{key:Box<Self>,value:Box<Self>},Function{params:Vec<Self>,ret:Box<Self>},Leaf}struct TypeField{ty:Type}impl Type{pub fn children(&self) -> Vec<&Self> {
        match self {
            Self::Array { element, .. }
            | Self::Option { element, .. }
            | Self::Set { element, .. }
            | Self::Cell { element } => {
                vec![element]
            }
            Self::Tuple { elements } => elements.iter().collect(),
            Self::Union { alternatives } => alternatives.iter().collect(),
            Self::Record { fields, .. } => fields.iter().map(|field| &field.ty).collect(),
            Self::Map { key, value, .. } => vec![key, value],
            Self::Function { params, ret } => {
                params.iter().chain(std::iter::once(ret.as_ref())).collect()
            }
            _ => Vec::new(),
        }
    }}fn main(){let record=Type::Record{fields:vec![TypeField{ty:Type::Leaf}]};println!("{}",record.children().len());let function=Type::Function{params:vec![Type::Leaf],ret:Box::new(Type::Leaf)};println!("{}",function.children().len());let leaf=Type::Leaf;println!("{}",leaf.children().len());}"#;
    fn checked(source: &str, version: &str) -> crate::translation::ir::Program {
        let surface = super::super::parse_rust_with_selected_profiles(
            source,
            super::super::SourceProfiles {
                source_version: version,
                source_signature: "Rust source function signature 3",
                ..super::super::SourceProfiles::default()
            },
        )
        .expect("authored selected readonly source");
        crate::translation::check::check_program(&surface)
            .expect("actual readonly body and type proof")
    }
    #[test]
    fn frontend_readonly_map_chain_and_contextual_collect_retain_all_four_outputs() {
        for version in ["1.90.0", "1.99.0"] {
            for source in [
                MAP,
                CHAIN,
                "struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().collect::<Vec<_>>()}fn main(){let values=vec![Node(7u8)];println!(\"{}\",children(&values).len());}",
            ] {
                let program = checked(source, version);
                let value =
                    serde_json::to_value(&program).expect("owned checked source serialization");
                let reloaded: crate::translation::ir::Program =
                    serde_json::from_value(value).expect("owned checked source reload");
                assert_eq!(reloaded, program);
                crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(&reloaded).expect("reconstructed binder proof");
                assert_eq!(
                    crate::translation::emit_javascript::emit_javascript(&program)
                        .expect("JavaScript target"),
                    crate::translation::emit_javascript::emit_javascript(&reloaded)
                        .expect("reloaded JavaScript target")
                );
                assert_eq!(
                    crate::translation::emit_rust::emit_rust(&program).expect("Rust target"),
                    crate::translation::emit_rust::emit_rust(&reloaded)
                        .expect("reloaded Rust target")
                );
                assert_eq!(
                    crate::translation::emit_lean::emit_lean(&program).expect("Lean target"),
                    crate::translation::emit_lean::emit_lean(&reloaded)
                        .expect("reloaded Lean target")
                );
                assert_eq!(
                    crate::translation::emit_rocq::emit_rocq(&program).expect("Rocq target"),
                    crate::translation::emit_rocq::emit_rocq(&reloaded)
                        .expect("reloaded Rocq target")
                );
            }
        }
    }
    fn forge_lambda_static(value: &mut serde_json::Value) -> bool {
        match value {
            serde_json::Value::Object(fields) => {
                if fields.get("k").and_then(serde_json::Value::as_str) == Some("lambda") {
                    if let Some(life) = fields
                        .get_mut("body")
                        .and_then(|v| v.get_mut("sourcePosition"))
                        .and_then(|v| v.get_mut("position"))
                        .and_then(|v| v.get_mut("lifetimes"))
                        .and_then(serde_json::Value::as_array_mut)
                        .and_then(|v| v.first_mut())
                    {
                        *life = serde_json::json!("static");
                        return true;
                    }
                }
                for (name, child) in fields {
                    if name != "type" && name != "sourcePosition" && forge_lambda_static(child) {
                        return true;
                    }
                }
                false
            }
            serde_json::Value::Array(values) => values.iter_mut().any(forge_lambda_static),
            _ => false,
        }
    }
    #[test]
    fn source_free_callback_cannot_forge_static_regions() {
        let program = checked(MAP, "1.90.0");
        let mut value = serde_json::to_value(program).expect("owned checked source serialization");
        assert!(forge_lambda_static(&mut value));
        let program: crate::translation::ir::Program =
            serde_json::from_value(value).expect("structurally represented forged source");
        assert!(
            crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
                &program
            )
            .is_err()
        );
    }
    #[test]
    fn old_copy_callback_parameter_gate_is_retained() {
        assert!(!crate::translation::rust_copy_closure_parameter_rules::accept_rust_copy_closure_parameter_source(true,false,1.0,true,false));
    }
    #[test]
    fn actual_own_type_children_method_has_a_retained_readonly_source_control() {
        for version in ["1.90.0", "1.99.0"] {
            let program = checked(TYPE_CHILDREN, version);
            crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
                &program,
            )
            .expect("actual own method binder proof");
            crate::translation::emit_javascript::emit_javascript(&program)
                .expect("actual own method JavaScript target");
            crate::translation::emit_rust::emit_rust(&program)
                .expect("actual own method Rust target");
            crate::translation::emit_lean::emit_lean(&program)
                .expect("actual own method Lean target");
            crate::translation::emit_rocq::emit_rocq(&program)
                .expect("actual own method Rocq target");
        }
    }
    #[test]
    fn captured_local_and_wrong_owner_refusals_remain() {
        for source in [
            "struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().map(|value|{let local=Node(1u8);&local}).collect()}fn main(){}",
            "struct Node(u8);fn children<'a>(values:&'a Vec<Node>,extra:&'a Node)->Vec<&'a Node>{values.iter().map(|value|extra).collect()}fn main(){}",
            "struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a u8>{values.iter().collect()}fn main(){}",
        ] {
            let result = super::super::parse_rust_with_selected_profiles(
                source,
                super::super::SourceProfiles {
                    source_version: "1.90.0",
                    source_signature: "Rust source function signature 3",
                    ..super::super::SourceProfiles::default()
                },
            )
            .and_then(|surface| crate::translation::check::check_program(&surface));
            assert!(result.is_err());
        }
    }
}
