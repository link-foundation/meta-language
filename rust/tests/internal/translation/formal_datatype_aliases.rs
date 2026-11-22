#[cfg(test)]
mod deferred_controls {
    use super::*;
    #[test]
    fn a_direct_self_datatype_without_a_finite_witness_refuses_each_formal_target() {
        let surface =
            crate::translation::rust::parse_rust("struct Endless(bool);fn main(){}").unwrap();
        let mut checked = crate::translation::check::check_program(&surface).unwrap();
        for entry in &mut checked.declarations {
            if let Decl::Data(data) = entry {
                if data.full_name == "Endless" {
                    data.ctors[0].fields[0].ty = crate::translation::types::Type::Data {
                        name: "Endless".into(),
                    };
                }
            }
        }
        assert!(crate::translation::emit_lean::emit_lean(&checked).is_err());
        assert!(crate::translation::emit_rocq::emit_rocq(&checked).is_err());
    }
    #[test]
    fn actual_nested_datatypes_emit_complete_mutual_blocks_and_public_pattern_aliases() {
        // Backend graph control: the original parsed bool fields are explicitly
        // replaced in a checked graph. This is not source-size admission evidence.
        let surface=crate::translation::rust::parse_rust("mod ir { struct Expr(bool); mod nodes { enum Node { Unit, Unary(bool) } } } fn main() {}").unwrap();
        let mut original = crate::translation::check::check_program(&surface).unwrap();
        for entry in &mut original.declarations {
            if let Decl::Data(data) = entry {
                if data.full_name == "ir.Expr" {
                    data.ctors[0].fields[0].ty = crate::translation::types::Type::Data {
                        name: "ir.nodes.Node".into(),
                    };
                } else if data.full_name == "ir.nodes.Node" {
                    data.ctors[1].fields[0].ty = crate::translation::types::Type::Data {
                        name: "ir.Expr".into(),
                    };
                }
            }
        }
        let saved = original.clone();
        assert!(crate::translation::emit_common::order_declarations(&original, true).is_err());
        let lean = crate::translation::emit_lean::emit_lean(&original)
            .unwrap()
            .text;
        let rocq = crate::translation::emit_rocq::emit_rocq(&original)
            .unwrap()
            .text;
        assert!(lean.contains("mutual\ninductive"));
        assert!(lean.contains("abbrev Node := _root_."));
        assert!(lean.contains("@[match_pattern] abbrev"));
        assert!(rocq.contains("\nwith "));
        assert!(rocq.contains("Definition Node : Type := "));
        assert!(rocq.contains("Notation "));
        assert_eq!(rocq.matches("Module nodes.").count(), 1);
        assert_eq!(original, saved);
    }
}
