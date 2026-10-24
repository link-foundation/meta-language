//! Future complete source exception controls; no local compilation or execution.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn computed_exception_messages_retain_class_units_presence_and_operand_order() {
    assert_nominal_translation_product(
        "../experiments/source-exceptions/oracle.mjs",
        "JavaScript source exceptions",
        meta_language::create_source_exception_program_links,
    );
}

#[test]
fn formal_exception_pairs_keep_class_message_and_presence_without_source_name_capture() {
    use meta_language::translation::{
        check::check_program, ir::Decl, javascript::parse_javascript,
        output::thread_output_for_target,
    };
    for target in ["Lean", "Rocq"] {
        let source = "/** @returns {number} */ function ml_io1(){throw TypeError();}\n/** @param {number} ml_out @returns {number} */ function ml_source1_forward(ml_out){const ml_o1=ml_out+1;return ml_io1()+ml_o1;}\nconsole.log(ml_source1_forward(7));";
        let original = check_program(&parse_javascript(source).unwrap()).unwrap();
        let threaded = thread_output_for_target(&original, target).unwrap();
        let mut count = 0;
        for entry in &threaded.declarations {
            let Decl::Data(pair) = entry else { continue };
            if !pair.output {
                continue;
            }
            count += 1;
            assert!(
                !original
                    .declarations
                    .iter()
                    .any(|entry| entry.full_name() == pair.full_name)
            );
            let fields = pair.ctors[1]
                .fields
                .iter()
                .map(|field| (field.name.as_str(), field.ty.kind()))
                .collect::<Vec<_>>();
            assert_eq!(
                fields,
                vec![
                    ("output", "output"),
                    ("message", "string"),
                    ("className", "string"),
                    ("messagePresent", "bool")
                ]
            );
        }
        assert!(count > 0);
        let packet = meta_language::encode_typed_program_links(&original).unwrap();
        let emitted =
            meta_language::emit_typed_program_links(&packet.network, packet.root, target).unwrap();
        assert!(emitted.code.contains("ml_source_exception_observation"));
    }
}
