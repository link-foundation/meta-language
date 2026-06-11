use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use meta_language::{
    FromLinks, LinkNetwork, LinkQuery, LinkType, LinksDecoder, LinksEncoder, LinksObject,
    RustFieldShape, RustTypeShape, ToLinks,
};

#[derive(Debug, Default)]
struct FixtureNode {
    name: String,
    parent: Option<Rc<RefCell<Self>>>,
    primary: Option<Rc<RefCell<Self>>>,
    children: Vec<Rc<RefCell<Self>>>,
    metadata: BTreeMap<String, String>,
}

impl LinksObject for FixtureNode {
    const TYPE_NAME: &'static str = "tests::FixtureNode";

    fn type_shape() -> RustTypeShape {
        RustTypeShape::structure(
            Self::TYPE_NAME,
            [
                RustFieldShape::new("name", "String"),
                RustFieldShape::new("parent", "Option<tests::FixtureNode>"),
                RustFieldShape::new("primary", "Option<tests::FixtureNode>"),
                RustFieldShape::new("children", "Vec<tests::FixtureNode>"),
                RustFieldShape::new("metadata", "BTreeMap<String,String>"),
            ],
        )
    }

    fn encode_fields(&self, encoder: &mut LinksEncoder, object: meta_language::LinkId) {
        encoder.encode_field(object, "name", &self.name);
        encoder.encode_field(object, "parent", &self.parent);
        encoder.encode_field(object, "primary", &self.primary);
        encoder.encode_field(object, "children", &self.children);
        encoder.encode_field(object, "metadata", &self.metadata);
    }

    fn decode_fields(
        &mut self,
        decoder: &mut LinksDecoder<'_>,
        object: meta_language::LinkId,
    ) -> Result<(), meta_language::LinksCodecError> {
        self.name = decoder.decode_field(object, "name")?;
        self.parent = decoder.decode_field(object, "parent")?;
        self.primary = decoder.decode_field(object, "primary")?;
        self.children = decoder.decode_field(object, "children")?;
        self.metadata = decoder.decode_field(object, "metadata")?;
        Ok(())
    }
}

#[test]
fn user_struct_with_nesting_sharing_and_cycle_round_trips_through_links_and_lino() {
    // Mirrors lino-objects-codec shared-reference and circular-reference cases
    // through a user-defined Rust struct rather than hand-authored LiNo text.
    let root = node("root", [("role", "root")]);
    let child = node("child", [("role", "child")]);

    root.borrow_mut().children.push(Rc::clone(&child));
    root.borrow_mut().primary = Some(Rc::clone(&child));
    child.borrow_mut().parent = Some(Rc::clone(&root));

    let mut encoder = LinksEncoder::new();
    let root_link = root.to_links(&mut encoder);
    let network = encoder.into_network();

    let mut decoder = LinksDecoder::new(&network);
    let decoded_graph = Rc::<RefCell<FixtureNode>>::from_links(&mut decoder, root_link)
        .expect("links decode back into the cyclic graph");
    assert_fixture_graph(&decoded_graph);

    let lino = network.to_lino();
    let restored_network = LinkNetwork::from_lino(&lino).expect("LiNo text restores the network");
    let mut restored_decoder = LinksDecoder::new(&restored_network);
    let restored = Rc::<RefCell<FixtureNode>>::from_links(&mut restored_decoder, root_link)
        .expect("LiNo-restored links decode back into the cyclic graph");
    assert_fixture_graph(&restored);
}

#[test]
fn type_shape_links_are_queryable_via_link_query() {
    let root = node("root", [("role", "root")]);
    let mut encoder = LinksEncoder::new();
    let _ = root.to_links(&mut encoder);
    let network = encoder.into_network();

    let type_links = network.query_links(
        &LinkQuery::by_type(LinkType::Type).with_term("rust::type::tests::FixtureNode"),
    );
    assert_eq!(type_links.len(), 1);
    let type_id = type_links[0].id();

    let field_names = network
        .query_links(&LinkQuery::by_type(LinkType::Field))
        .into_iter()
        .filter(|link| link.references().first() == Some(&type_id))
        .filter_map(|link| {
            let label = link.references().get(1).copied()?;
            network.link(label)?.metadata().term()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        field_names,
        ["name", "parent", "primary", "children", "metadata"]
    );
}

fn node<const N: usize>(name: &str, metadata: [(&str, &str); N]) -> Rc<RefCell<FixtureNode>> {
    Rc::new(RefCell::new(FixtureNode {
        name: name.to_string(),
        metadata: metadata
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
        ..FixtureNode::default()
    }))
}

fn assert_fixture_graph(root: &Rc<RefCell<FixtureNode>>) {
    let root_ref = root.borrow();
    assert_eq!(root_ref.name, "root");
    assert_eq!(
        root_ref.metadata.get("role").map(String::as_str),
        Some("root")
    );
    assert_eq!(root_ref.children.len(), 1);

    let child = Rc::clone(&root_ref.children[0]);
    let primary = root_ref
        .primary
        .as_ref()
        .expect("primary child exists")
        .clone();
    assert!(
        Rc::ptr_eq(&child, &primary),
        "the repeated child reference should decode as one shared object"
    );
    drop(root_ref);

    let child_ref = child.borrow();
    assert_eq!(child_ref.name, "child");
    assert_eq!(
        child_ref.metadata.get("role").map(String::as_str),
        Some("child")
    );
    let parent = child_ref.parent.as_ref().expect("back-reference exists");
    assert!(
        Rc::ptr_eq(parent, root),
        "the child back-reference should preserve the root cycle"
    );
}
