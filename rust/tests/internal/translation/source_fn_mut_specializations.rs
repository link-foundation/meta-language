#[cfg(test)]
mod tests {
    use super::*;
    use crate::translation::source_signatures::SourceOwner;

    fn place(ty: Type, owner: SourceOwner) -> SourceTypeSignature {
        SourceTypeSignature {
            ty,
            owner,
            wrappers: Vec::new(),
            lifetimes: Vec::new(),
            array_element_position: None,
        }
    }

    fn packet(types: Vec<Type>) -> Type {
        Type::Tuple { elements: types }
    }

    #[test]
    fn checked_callback_interface_rejects_forged_roles_and_writebacks() {
        let value = Type::Fixed {
            bits: 8,
            signed: false,
        };
        let unsigned = SourceOwner {
            kind: "primitive".into(),
            path: vec!["u8".into()],
            arguments: Vec::new(),
        };
        let written = place(value.clone(), unsigned.clone());
        let returned = packet(vec![value.clone(), value.clone()]);
        let callback = Type::Function {
            params: vec![value.clone(), value.clone()],
            ret: Box::new(returned.clone()),
        };
        let callable = place(
            callback.clone(),
            SourceOwner {
                kind: "function".into(),
                path: Vec::new(),
                arguments: vec![
                    unsigned.clone(),
                    unsigned.clone(),
                    SourceOwner {
                        kind: "tuple".into(),
                        path: Vec::new(),
                        arguments: vec![unsigned.clone(), unsigned],
                    },
                ],
            },
        );
        let params = [("map", callback), ("state", value)].map(|(name, ty)| Param {
            name: name.into(),
            ty,
            optional: false,
            guard: None,
            default_value: None,
        });
        let range = |start, end| SourceRange { start, end };
        let facet = RustFnMutSpecialization {
            model: "Rust closed source FnMut specializations 1".into(),
            language: "Rust".into(),
            source_version: "1.90.0".into(),
            declaration: SourceDeclaration {
                path: vec!["apply".into()],
                range: range(0, 70),
                body_range: range(51, 70),
                generics: Vec::new(),
                formal_count: 1,
                callback: OriginalCallback {
                    index: 0,
                    name: "map".into(),
                    mode: "value".into(),
                    mutable: true,
                    binder: range(13, 16),
                    type_range: range(18, 39),
                    parameter_ranges: vec![range(28, 30)],
                    result_range: range(37, 39),
                },
            },
            type_arguments: Vec::new(),
            callback: Callback {
                parameter: 0,
                parameters: vec![written.clone()],
                result: written.clone(),
                captures: vec![Capture {
                    parameter: 1,
                    binder: SourceStateBinder {
                        name: "state".into(),
                        identity: "main:80".into(),
                        start: 80,
                        end: 85,
                    },
                    position: written.clone(),
                }],
            },
            parameters: vec![
                Parameter {
                    mode: "callback".into(),
                    position: callable,
                },
                Parameter {
                    mode: "exclusive".into(),
                    position: written.clone(),
                },
            ],
            result: written,
            packet: vec![1],
        };
        let check = |facet: &RustFnMutSpecialization| {
            check_interface(
                facet,
                &params,
                &returned,
                "Rust",
                |ty| Ok(ty.clone()),
                |_| None,
            )
        };
        assert_eq!(check(&facet).unwrap(), facet);
        let mut forged = facet.clone();
        forged.packet[0] = 0;
        assert!(check(&forged).is_err());
        let mut forged = facet.clone();
        forged.parameters[1].mode = "value".into();
        assert!(check(&forged).is_err());
        let mut forged = facet.clone();
        forged.callback.captures[0].parameter = 0;
        assert!(check(&forged).is_err());
        let mut forged = facet;
        forged.source_version = "1.100.0".into();
        assert!(check(&forged).is_err());
    }
}
