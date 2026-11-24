//! Convert Rust frontend strings at the generated JavaScript policy boundary.

fn decode(units: &[u16]) -> String {
    // These policies return fixed labels or complete field identities. They
    // construct and split identities at ASCII delimiters in valid Rust text.
    String::from_utf16(units).expect("record policy preserves the frontend's Unicode text")
}

pub(super) mod fields {
    use crate::translation::rust_owned_record_update_rules::{self as rules, ml_string::Text};

    pub(in super::super) fn read_rust_owned_record_update_version_error(minor: f64) -> String {
        super::decode(&rules::read_rust_owned_record_update_version_error(minor).0)
    }

    pub(in super::super) fn test_rust_owned_record_field_copy(
        proven: bool,
        copy: bool,
        wrappers: &[String],
    ) -> bool {
        let wrappers: Vec<_> = wrappers
            .iter()
            .map(|value| Text::from_text(value))
            .collect();
        rules::test_rust_owned_record_field_copy(proven, copy, &wrappers)
    }

    pub(in super::super) fn read_rust_owned_record_field_mode(
        copy: bool,
        borrowed: bool,
    ) -> String {
        super::decode(&rules::read_rust_owned_record_field_mode(copy, borrowed).0)
    }

    pub(in super::super) fn read_rust_owned_record_field_identity(
        root: &str,
        index: f64,
    ) -> String {
        super::decode(
            &rules::read_rust_owned_record_field_identity(&Text::from_text(root), index).0,
        )
    }

    pub(in super::super) fn read_rust_owned_record_field_root(identity: &str) -> String {
        super::decode(&rules::read_rust_owned_record_field_root(&Text::from_text(identity)).0)
    }

    pub(in super::super) fn read_rust_owned_record_absent_field_state(root: &str) -> String {
        super::decode(&rules::read_rust_owned_record_absent_field_state(&Text::from_text(root)).0)
    }

    pub(in super::super) fn read_rust_owned_record_ultimate_root(identity: &str) -> String {
        super::decode(&rules::read_rust_owned_record_ultimate_root(&Text::from_text(identity)).0)
    }

    pub(in super::super) fn test_rust_owned_record_field_descendant(
        root: &str,
        identity: &str,
    ) -> bool {
        rules::test_rust_owned_record_field_descendant(
            &Text::from_text(root),
            &Text::from_text(identity),
        )
    }

    pub(in super::super) fn read_rust_owned_record_whole_error(
        root: &str,
        identities: &[String],
        states: &[String],
    ) -> String {
        let identities: Vec<_> = identities
            .iter()
            .map(|value| Text::from_text(value))
            .collect();
        let states: Vec<_> = states.iter().map(|value| Text::from_text(value)).collect();
        super::decode(
            &rules::read_rust_owned_record_whole_error(
                &Text::from_text(root),
                &identities,
                &states,
            )
            .0,
        )
    }

    pub(in super::super) fn read_rust_owned_record_field_error(root: &str, field: &str) -> String {
        super::decode(
            &rules::read_rust_owned_record_field_error(
                &Text::from_text(root),
                &Text::from_text(field),
            )
            .0,
        )
    }

    pub(in super::super) fn read_rust_owned_record_update_error(
        profile: &str,
        owner: bool,
        layers: f64,
        fields: f64,
        positions: f64,
    ) -> String {
        super::decode(
            &rules::read_rust_owned_record_update_error(
                &Text::from_text(profile),
                owner,
                layers,
                fields,
                positions,
            )
            .0,
        )
    }
}

pub(super) mod state {
    use crate::translation::rust_owned_record_state_rules::{self as rules, ml_string::Text};
    pub(in super::super) use rules::{
        plan_rust_owned_record_state_fields, plan_rust_owned_record_state_packet,
    };

    pub(in super::super) fn read_rust_owned_record_state_assignment_token_count(
        forms: &[String],
    ) -> f64 {
        let forms: Vec<_> = forms.iter().map(|value| Text::from_text(value)).collect();
        rules::read_rust_owned_record_state_assignment_token_count(&forms)
    }

    pub(in super::super) fn read_rust_owned_record_state_storage_disposition(
        kind: &str,
        layers: f64,
        role: &str,
    ) -> String {
        super::decode(
            &rules::read_rust_owned_record_state_storage_disposition(
                &Text::from_text(kind),
                layers,
                &Text::from_text(role),
            )
            .0,
        )
    }

    pub(in super::super) fn accept_rust_owned_record_state_storage_owner(
        kind: &str,
        path: &str,
    ) -> bool {
        rules::accept_rust_owned_record_state_storage_owner(
            &Text::from_text(kind),
            &Text::from_text(path),
        )
    }

    pub(in super::super) fn read_rust_owned_record_state_storage_error(
        nodes: f64,
        depth: f64,
    ) -> String {
        super::decode(&rules::read_rust_owned_record_state_storage_error(nodes, depth).0)
    }

    pub(in super::super) fn read_rust_owned_record_mutable_binding_error(
        ordinary: bool,
        written: bool,
        profile: &str,
        binding: bool,
        lexical: bool,
        layers: f64,
    ) -> String {
        super::decode(
            &rules::read_rust_owned_record_mutable_binding_error(
                ordinary,
                written,
                &Text::from_text(profile),
                binding,
                lexical,
                layers,
            )
            .0,
        )
    }

    pub(in super::super) fn read_rust_owned_record_state_write_error(
        mutable: bool,
        callable: bool,
        scope: bool,
        root: &str,
        field: &str,
        types_match: bool,
        loan: bool,
    ) -> String {
        super::decode(
            &rules::read_rust_owned_record_state_write_error(
                mutable,
                callable,
                scope,
                &Text::from_text(root),
                &Text::from_text(field),
                types_match,
                loan,
            )
            .0,
        )
    }

    pub(in super::super) fn test_rust_owned_record_state_loan_overlap(
        root: &str,
        target: &str,
        loans: &[String],
        lexical: bool,
    ) -> bool {
        let loans: Vec<_> = loans.iter().map(|value| Text::from_text(value)).collect();
        rules::test_rust_owned_record_state_loan_overlap(
            &Text::from_text(root),
            &Text::from_text(target),
            &loans,
            lexical,
        )
    }

    pub(in super::super) fn read_rust_owned_record_state_field_identity(
        root: &str,
        fields: &[f64],
    ) -> String {
        super::decode(
            &rules::read_rust_owned_record_state_field_identity(&Text::from_text(root), fields).0,
        )
    }
}
