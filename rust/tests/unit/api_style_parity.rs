use meta_language::{
    ApiOperation, ApiStyle, ApiStyleFixture, FluentNetworkApi, LinkCliSubstitution,
    LinkCliSubstitutionKind, LinkMetadata, LinkNetwork, LinkQuery, LinkType, ParseConfiguration,
    ReplacementRule, API_OPERATIONS,
};

#[test]
fn api_operations_registry_has_explicit_style_cells_and_executable_fixtures() {
    for operation in API_OPERATIONS {
        for style in ApiStyle::ALL {
            let cell = operation.style(*style).unwrap_or_else(|| {
                panic!("{} is missing {:?} style cell", operation.name(), style)
            });

            match cell.fixture() {
                ApiStyleFixture::Executable(name) => {
                    assert!(
                        !name.is_empty(),
                        "{} {:?} executable fixture must be named",
                        operation.name(),
                        style
                    );
                }
                ApiStyleFixture::NotApplicable(reason) => {
                    assert!(
                        !reason.is_empty(),
                        "{} {:?} N/A cell must explain why the style cannot apply",
                        operation.name(),
                        style
                    );
                }
            }
        }
    }

    assert_registry_contains(ApiOperation::Parse);
    assert_registry_contains(ApiOperation::Query);
    assert_registry_contains(ApiOperation::Transform);
    assert_registry_contains(ApiOperation::Substitute);
    assert_registry_contains(ApiOperation::Serialize);
    assert_registry_contains(ApiOperation::Snapshot);
    assert_registry_contains(ApiOperation::Translate);
    assert_registry_contains(ApiOperation::Verify);
}

#[test]
fn executable_api_style_fixtures_cover_every_applicable_registry_cell() {
    for operation in API_OPERATIONS {
        for cell in operation.styles() {
            if let ApiStyleFixture::Executable(name) = cell.fixture() {
                meta_language::run_api_style_fixture(name)
                    .unwrap_or_else(|error| panic!("fixture {name} failed: {error}"));
            }
        }
    }
}

#[test]
fn fluent_pipeline_covers_parse_query_transform_and_reconstruct() {
    let output = LinkNetwork::parse_fluent(
        "const oldName = call(oldName);\n",
        "JavaScript",
        ParseConfiguration::default(),
    )
    .find(
        LinkQuery::from_sexpression(
            r#"
            (identifier) @target
            (#eq? @target "oldName")
            "#,
        )
        .expect("query parses"),
    )
    .replace(ReplacementRule::captured_text("target", "newName"))
    .reconstruct();

    assert_eq!(output, "const newName = call(newName);\n");
}

#[test]
fn link_cli_text_substitution_covers_create_read_update_and_delete() {
    let mut network = LinkNetwork::new();

    let create = network
        .apply_link_cli_substitution_text("() ((1 1))")
        .expect("create command parses and runs");
    assert_eq!(create.created().len(), 1);

    let relation = create.created()[0];
    assert_eq!(relation.as_u64(), 1);
    assert_eq!(
        network.link(relation).expect("created link").references(),
        &[relation, relation]
    );

    let read = network
        .apply_link_cli_substitution_text("((1: 1 1)) ((1: 1 1))")
        .expect("read identity command parses and runs");
    assert_eq!(read.updated(), &[relation]);
    assert_eq!(
        network
            .link(relation)
            .expect("read keeps link")
            .references(),
        &[relation, relation]
    );

    let update = network
        .apply_link_cli_substitution_text("((1: 1 1)) ((1: 1 2))")
        .expect("update command parses and runs");
    assert_eq!(update.updated(), &[relation]);
    assert_eq!(
        network.link(relation).expect("updated link").references(),
        &[
            LinkCliSubstitution::link_id(1),
            LinkCliSubstitution::link_id(2)
        ]
    );

    let delete = network
        .apply_link_cli_substitution_text("((1 2)) ()")
        .expect("delete command parses and runs");
    assert_eq!(delete.deleted(), &[relation]);
    assert!(network.link(relation).is_none());
}

#[test]
fn link_cli_text_substitution_classifies_operation_kinds() {
    assert_eq!(
        LinkCliSubstitution::parse("() ((1 1))")
            .expect("create parses")
            .kind(),
        LinkCliSubstitutionKind::Create
    );
    assert_eq!(
        LinkCliSubstitution::parse("((1: 1 1)) ((1: 1 1))")
            .expect("read parses")
            .kind(),
        LinkCliSubstitutionKind::ReadIdentity
    );
    assert_eq!(
        LinkCliSubstitution::parse("((1: 1 1)) ((1: 1 2))")
            .expect("update parses")
            .kind(),
        LinkCliSubstitutionKind::Update
    );
    assert_eq!(
        LinkCliSubstitution::parse("((1 1)) ()")
            .expect("delete parses")
            .kind(),
        LinkCliSubstitutionKind::Delete
    );
}

#[test]
fn fluent_pipeline_can_run_structural_substitution_without_reparsing() {
    let mut network = LinkNetwork::new();
    let one = network.insert_point("1");
    let two = network.insert_point("2");
    let relation = network.insert_link(
        [one, one],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );

    let pipeline = network
        .into_fluent()
        .substitute(meta_language::SubstitutionRule::new([one, one], [one, two]));

    assert_eq!(pipeline.last_report().substitution().updated(), &[relation]);
    assert_eq!(
        pipeline
            .network()
            .link(relation)
            .expect("updated relation")
            .references(),
        &[one, two]
    );
}

fn assert_registry_contains(operation: ApiOperation) {
    assert!(
        API_OPERATIONS
            .iter()
            .any(|candidate| candidate.operation() == operation),
        "missing API operation registry entry for {operation:?}"
    );
}
