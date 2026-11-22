#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn placement_preserves_actual_source_ids_and_explicit_public_aliases() {
        // Structural projection control, not a claim that unguarded Rust
        // recursive fields form a valid source-sized declaration.
        let surface=crate::translation::rust::parse_rust("mod left { fn unrelated()->bool { true } enum A { Value(bool) } } mod right { enum B { Value(bool) } } fn main() {}").unwrap();
        let mut original = crate::translation::check::check_program(&surface).unwrap();
        for entry in &mut original.declarations {
            if let Decl::Data(data) = entry {
                let target = if data.full_name == "left.A" {
                    "right.B"
                } else {
                    "left.A"
                };
                data.ctors[0].fields[0].ty = crate::translation::types::Type::Data {
                    name: target.to_owned(),
                };
            }
        }
        let saved = original.clone();
        let projected = plan_formal_datatype_modules(&original).unwrap();
        assert_eq!(original, saved);
        assert_eq!(projected.source_program, &original);
        assert_eq!(projected.aliases.len(), 2);
        for alias in &projected.aliases {
            assert_eq!(alias.source_full_name, alias.target_full_name);
            assert!(alias.target_module_path.is_empty());
            assert_eq!(alias.constructors, vec!["Value".to_owned()]);
            assert_eq!(
                projected
                    .program
                    .declaration(&alias.target_full_name)
                    .unwrap()
                    .module_path(),
                &[] as &[String]
            );
            assert!(
                !original
                    .declaration(&alias.source_full_name)
                    .unwrap()
                    .module_path()
                    .is_empty()
            );
        }
        assert!(crate::translation::emit_common::order_declarations(&original, true).is_err());
        assert!(
            crate::translation::emit_common::order_declarations(&projected.program, true).is_ok()
        );
        let extra = formal_datatype_module_dependencies(&projected).unwrap();
        assert_eq!(
            extra["left.unrelated"],
            HashSet::from(["left.A".to_owned()])
        );
        let order = crate::translation::emit_common::order_declarations_with_dependencies(
            &projected.program,
            true,
            &extra,
        )
        .unwrap();
        assert_eq!(order.last().unwrap().full_name(), "left.unrelated");
        let mut state = crate::translation::emit_common::EmitState::new(
            &original,
            crate::translation::Language::Rocq,
            str::to_owned,
            &[],
            crate::translation::emit_common::EmitOptions {
                ctor_style: crate::translation::emit_common::CtorStyle::Module,
                modules_share_term_space: true,
                ..Default::default()
            },
        );
        let public = projected
            .aliases
            .iter()
            .flat_map(|alias| {
                [
                    state.reference(&alias.source_full_name, "."),
                    state.ctor_ref(&alias.source_full_name, "Value", "."),
                ]
            })
            .collect::<Vec<_>>();
        state
            .relocate_datatypes(&projected.program, &projected.aliases)
            .unwrap();
        for alias in &projected.aliases {
            assert!(!public.contains(&state.reference(&alias.target_full_name, ".")));
            assert!(!public.contains(&state.ctor_ref(&alias.target_full_name, "Value", ".")));
        }
        assert!(
            state
                .relocate_datatypes(&projected.program, &projected.aliases)
                .is_err()
        );
    }
}
