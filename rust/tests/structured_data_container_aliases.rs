//! Original container-alias controls compile the actual private production codec.
//! The test-only network bridge uses public Links serialization for reference
//! updates; these controls test the included alias codec, not private storage.

pub use meta_language::{LinkFlags, LinkId, LinkMetadata, LinkType};

/// Test-local transport around the actual public network implementation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkNetwork(meta_language::LinkNetwork);

impl std::ops::Deref for LinkNetwork {
    type Target = meta_language::LinkNetwork;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LinkNetwork {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LinkNetwork {
    const fn new() -> Self {
        Self(meta_language::LinkNetwork::new())
    }

    fn from_lino(
        source: &str,
    ) -> Result<Self, meta_language::lino_serialization::LinoSerializationError> {
        meta_language::LinkNetwork::from_lino(source).map(Self)
    }

    // Only the exact numeric statement is edited. Percent-encoded metadata is
    // retained verbatim and the actual public reader restores the network.
    fn set_references(&mut self, id: LinkId, references: &[LinkId]) -> bool {
        if self.0.link(id).is_none() {
            return false;
        }
        let prefix = format!("({id}:");
        let references = references
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        let mut selected = 0_usize;
        let statements = self
            .0
            .to_lino()
            .lines()
            .map(|statement| {
                let Some(body) = statement.strip_prefix(&prefix) else {
                    return statement.to_owned();
                };
                selected += 1;
                let (_, metadata) = body
                    .split_once("(meta:")
                    .expect("an actual canonical network statement retains its metadata");
                format!("{prefix} {references} (meta:{metadata}")
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(selected, 1, "exactly one numeric statement owns the link");
        self.0 = meta_language::LinkNetwork::from_lino(&(statements + "\n"))
            .expect("the public Links reference edit preserves canonical metadata");
        true
    }
}

pub mod link_network {
    pub use meta_language::Link;
}

#[path = "../src/structured_data_link_rules.rs"]
pub mod structured_data_link_rules;

#[path = "../src/structured_data_links.rs"]
pub mod structured_data_links;

use structured_data_links::{
    StructuredDataValue, encode_structured_data_links_ordered,
    encode_structured_data_links_ordered_with_container_aliases,
};

#[test]
fn explicit_container_aliases_preserve_owned_sharing_without_merging_equal_values() {
    let mut bridge = LinkNetwork::new();
    let untouched = bridge.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_term("untouched")
            .with_definition("metadata (meta: λ)"),
    );
    let changed = bridge.insert_link(
        [untouched],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("changed")
            .with_language("fixture"),
    );
    let original = bridge.0.clone();
    assert!(bridge.set_references(changed, &[changed, untouched]));
    assert_eq!(bridge.len(), original.len());
    for link in original.links() {
        let restored = bridge.link(link.id()).unwrap();
        assert_eq!(restored.id(), link.id());
        assert_eq!(restored.metadata(), link.metadata());
        if link.id() == changed {
            assert_eq!(restored.references(), [changed, untouched]);
        } else {
            assert_eq!(restored.references(), link.references());
        }
    }
    let after = bridge.0.clone();
    assert!(!bridge.set_references(LinkId::from_u64(u64::MAX), &[]));
    assert_eq!(bridge.0, after);

    let container = StructuredDataValue::Object(vec![
        (
            "value".encode_utf16().collect(),
            StructuredDataValue::Number(-0.0),
        ),
        (
            "units".encode_utf16().collect(),
            StructuredDataValue::String(vec![0xd800]),
        ),
    ]);
    let value = StructuredDataValue::Array(vec![container.clone(), container.clone(), container]);
    let StructuredDataValue::Array(children) = &value else {
        unreachable!();
    };
    let independent = encode_structured_data_links_ordered(&value).unwrap();
    let unchanged =
        encode_structured_data_links_ordered_with_container_aliases(&value, &[]).unwrap();
    assert_eq!(independent.network.to_lino(), unchanged.network.to_lino());
    let body = independent
        .network
        .link(independent.root)
        .unwrap()
        .references()[0];
    let references = independent.network.link(body).unwrap().references();
    assert_ne!(references[0], references[1]);
    assert_ne!(references[0], references[2]);
    let shared = encode_structured_data_links_ordered_with_container_aliases(
        &value,
        &[(&children[1], &children[0])],
    )
    .unwrap();
    let body = shared.network.link(shared.root).unwrap().references()[0];
    let references = shared.network.link(body).unwrap().references();
    assert_eq!(references[0], references[1]);
    assert_ne!(references[0], references[2]);
    assert_eq!(shared.network.len() + 3, independent.network.len());

    let arrays = StructuredDataValue::Array(vec![
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
    ]);
    let StructuredDataValue::Array(children) = &arrays else {
        unreachable!();
    };
    let shared = encode_structured_data_links_ordered_with_container_aliases(
        &arrays,
        &[(&children[1], &children[0])],
    )
    .unwrap();
    let body = shared.network.link(shared.root).unwrap().references()[0];
    let references = shared.network.link(body).unwrap().references();
    assert_eq!(references[0], references[1]);
    assert_ne!(references[0], references[2]);
}

#[test]
fn container_aliases_refuse_unowned_changed_or_incomplete_targets() {
    let value = StructuredDataValue::Array(vec![
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
        StructuredDataValue::Array(vec![]),
    ]);
    let StructuredDataValue::Array(children) = &value else {
        unreachable!();
    };
    let foreign = StructuredDataValue::Array(vec![]);
    let scalar = StructuredDataValue::Null;
    for aliases in [
        vec![(&foreign, &children[0])],
        vec![(&children[1], &foreign)],
        vec![(&children[0], &children[1])],
        vec![(&children[1], &children[0]), (&children[1], &children[2])],
        vec![(&children[0], &children[0])],
        vec![(&children[1], &scalar)],
        vec![(&scalar, &children[0])],
    ] {
        assert!(
            encode_structured_data_links_ordered_with_container_aliases(&value, &aliases).is_err()
        );
    }

    let negative_zero = StructuredDataValue::Array(vec![StructuredDataValue::Number(-0.0)]);
    let positive_zero = StructuredDataValue::Array(vec![StructuredDataValue::Number(0.0)]);
    let first_order = StructuredDataValue::Object(vec![
        (vec![97], StructuredDataValue::Null),
        (vec![98], StructuredDataValue::Null),
    ]);
    let second_order = StructuredDataValue::Object(vec![
        (vec![98], StructuredDataValue::Null),
        (vec![97], StructuredDataValue::Null),
    ]);
    let first_units = StructuredDataValue::Array(vec![StructuredDataValue::String(vec![0xd800])]);
    let second_units = StructuredDataValue::Array(vec![StructuredDataValue::String(vec![0xd801])]);
    let infinite = StructuredDataValue::Array(vec![StructuredDataValue::Number(f64::INFINITY)]);
    for (target, alias) in [
        (negative_zero, positive_zero),
        (first_order, second_order),
        (first_units, second_units),
        (infinite.clone(), infinite),
    ] {
        let value = StructuredDataValue::Array(vec![target, alias]);
        let StructuredDataValue::Array(children) = &value else {
            unreachable!();
        };
        assert!(
            encode_structured_data_links_ordered_with_container_aliases(
                &value,
                &[(&children[1], &children[0])],
            )
            .is_err()
        );
    }

    let target = StructuredDataValue::Array(vec![StructuredDataValue::Null]);
    let mut nested = target.clone();
    for _ in 0..254 {
        nested = StructuredDataValue::Array(vec![nested]);
    }
    let value = StructuredDataValue::Array(vec![target, nested]);
    let StructuredDataValue::Array(children) = &value else {
        unreachable!();
    };
    let mut alias = &children[1];
    for _ in 0..254 {
        let StructuredDataValue::Array(children) = alias else {
            unreachable!();
        };
        alias = &children[0];
    }
    assert!(encode_structured_data_links_ordered(&value).is_err());
    assert!(
        encode_structured_data_links_ordered_with_container_aliases(
            &value,
            &[(alias, &children[0])],
        )
        .is_err()
    );
}
