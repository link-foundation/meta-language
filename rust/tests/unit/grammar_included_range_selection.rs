//! CI-only mirrors of the authored JavaScript included-range controls.
//! Helpers transport public CST fields and official LiNo syntax; the actual
//! generated policies, parser and mapped-tree checker make every decision.
use std::cell::Cell;
use std::collections::HashMap;

use links_notation::{ParserConfig, parse_lino_to_links_with_config};
use meta_language::{
    FeatureParseOptions, Grammar, GrammarExpr, GrammarRule, ParseOutcome, SyntaxTree,
    check_grammar_selection_mapped_tree, compile_feature_grammar, parse_grammar_links,
    parse_links_expression, parse_native_grammar, percent_encode_links_text, render_grammar_links,
    render_native_grammar,
};
use serde_json::{Value, json};

#[path = "grammar_included_range_transport.rs"]
mod included_range_transport;
use included_range_transport::{
    included_range_bundle, included_range_link_data, included_range_link_text,
    included_range_physical, included_range_stored, included_range_tree_data,
};

#[path = "../../src/translation/grammar_selection_included_range_rules.rs"]
mod included_range_rules;
use included_range_rules::{
    accept_grammar_selection_included_range_agreement,
    accept_grammar_selection_included_range_byte_boundary,
    accept_grammar_selection_included_range_candidate,
    accept_grammar_selection_included_range_mapping,
    accept_grammar_selection_included_range_source_range,
    read_grammar_selection_included_range_context,
    read_grammar_selection_included_range_field_order, read_grammar_selection_included_range_part,
    read_grammar_selection_included_range_plan, read_grammar_selection_included_range_reservation,
};

#[path = "grammar_included_range_archive.rs"]
mod included_range_archive;

#[path = "grammar_field_range_selection.rs"]
mod field_range_selection;

fn included_range_grammar(
    region: &str,
    ranges: &[[i64; 2]],
    names: &[&str],
    mode: &str,
    prefix: &str,
    suffix: &str,
) -> Grammar {
    let ranges = ranges
        .iter()
        .map(|[start, end]| format!("(source-range {start} {end})"))
        .collect::<Vec<_>>()
        .join(" ");
    let candidates = names
        .iter()
        .map(|name| {
            format!(
                "(grammar-range-candidate {} included-ranges {ranges})",
                percent_encode_links_text(name),
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    parse_grammar_links(&format!(
        "(grammar (start document))\n(rule document normal (seq (literal {}) ({mode} (ref physical) {candidates}) (literal {})))\n(rule physical normal (literal {}))\n",
        percent_encode_links_text(prefix), percent_encode_links_text(suffix), percent_encode_links_text(region),
    )).unwrap()
}

fn included_range_literal(source: &str) -> Grammar {
    Grammar::new()
        .with_start("decoded")
        .with_rule(GrammarRule::new(
            "decoded",
            GrammarExpr::Terminal(source.to_owned()),
        ))
}

fn included_range_parse(
    grammar: &Grammar,
    grammars: &HashMap<String, Grammar>,
    source: &str,
) -> ParseOutcome {
    let resolve = |name: &str| grammars.get(name).cloned();
    compile_feature_grammar(grammar, Some(&resolve), FeatureParseOptions::default())
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap()
}

fn included_range_embedding_path(tree: &Value, path: &str) -> Option<String> {
    if tree["type"] == "embed" && tree["selection"]["mapping"] == "included-ranges" {
        return Some(path.to_owned());
    }
    for field in ["children", "sourceChildren"] {
        if let Some(children) = tree[field].as_array() {
            for (index, child) in children.iter().enumerate() {
                if let Some(found) =
                    included_range_embedding_path(child, &format!("{path}/{field}/{index}"))
                {
                    return Some(found);
                }
            }
        }
    }
    tree.get("root")
        .and_then(|root| included_range_embedding_path(root, &format!("{path}/root")))
}

fn included_range_embedding(tree: &Value) -> &Value {
    tree.pointer(&included_range_embedding_path(tree, "").unwrap())
        .unwrap()
}

#[test]
fn included_range_plans_preserve_kept_bytes_gaps_boundaries_and_original_inputs() {
    for (ranges, length, expected) in [
        (
            vec![0.0, 4.0, 6.0, 9.0],
            9.0,
            vec![
                0.0, 4.0, 0.0, 4.0, 4.0, 6.0, 4.0, 4.0, 6.0, 9.0, 4.0, 7.0, 9.0, 9.0, 7.0, 7.0,
            ],
        ),
        (
            vec![1.0, 3.0],
            5.0,
            vec![
                0.0, 1.0, 0.0, 0.0, 1.0, 3.0, 0.0, 2.0, 3.0, 5.0, 2.0, 2.0, 5.0, 5.0, 2.0, 2.0,
            ],
        ),
        (
            vec![0.0, 1.0, 1.0, 3.0],
            4.0,
            vec![
                0.0, 1.0, 0.0, 1.0, 1.0, 3.0, 1.0, 3.0, 3.0, 4.0, 3.0, 3.0, 4.0, 4.0, 3.0, 3.0,
            ],
        ),
    ] {
        let original = ranges.clone();
        let mut plan = vec![0.0; 4];
        plan.extend(expected);
        assert_eq!(
            read_grammar_selection_included_range_plan(&ranges, length),
            plan
        );
        assert_eq!(ranges, original);
        assert!(accept_grammar_selection_included_range_agreement(
            &ranges, &original
        ));
    }
    for (ranges, length) in [
        (vec![], 5.0),
        (vec![0.0], 5.0),
        (vec![0.0, 0.0], 5.0),
        (vec![-1.0, 1.0], 5.0),
        (vec![1.5, 2.0], 5.0),
        (vec![2.0, 1.0], 5.0),
        (vec![0.0, 6.0], 5.0),
        (vec![2.0, 3.0, 1.0, 2.0], 5.0),
        (vec![0.0, 3.0, 2.0, 4.0], 5.0),
        (vec![0.0, 5.0], 5.0),
        (vec![0.0, 2.0, 2.0, 5.0], 5.0),
        (vec![0.0, 1.0], 0.0),
        (vec![0.0, 1.0], f64::INFINITY),
        (vec![0.0, 1.0], f64::NAN),
        (vec![0.0, 1.0], 4_294_967_296.0),
        (vec![0.0, f64::NAN], 5.0),
    ] {
        assert!(read_grammar_selection_included_range_plan(&ranges, length).is_empty());
    }
    assert!(!accept_grammar_selection_included_range_agreement(
        &[0.0, 4.0],
        &[0.0, 3.0]
    ));
    assert!(!accept_grammar_selection_included_range_agreement(
        &[0.0, 4.0],
        &[0.0, 4.0, 6.0, 9.0]
    ));
    assert_eq!(
        read_grammar_selection_included_range_reservation(9.0, 4.0, false).to_bits(),
        352.0_f64.to_bits()
    );
    assert_eq!(
        read_grammar_selection_included_range_reservation(9.0, 4.0, true).to_bits(),
        640.0_f64.to_bits()
    );
    for cells in [0.0, 1.0, 19.0, f64::INFINITY, f64::NAN] {
        assert_eq!(
            read_grammar_selection_included_range_reservation(9.0, cells, false).to_bits(),
            (-1.0_f64).to_bits()
        );
    }
}

#[test]
fn included_range_candidates_require_one_closed_syntactic_mapping() {
    assert!(accept_grammar_selection_included_range_mapping(
        "included-ranges"
    ));
    assert!(!accept_grammar_selection_included_range_mapping(
        "json-string"
    ));
    assert!(accept_grammar_selection_included_range_candidate(
        "grammar-range-candidate",
        "included-ranges"
    ));
    assert!(!accept_grammar_selection_included_range_candidate(
        "grammar-candidate",
        "included-ranges"
    ));
    assert!(!accept_grammar_selection_included_range_candidate(
        "grammar-range-candidate",
        "json-string"
    ));
    for (mappings, in_token, expected) in [
        (vec!["identity"], false, 0.0),
        (vec!["json-string"], false, 0.0),
        (vec!["future-view"], false, 0.0),
        (vec!["included-ranges"], false, 1.0),
        (vec!["included-ranges", "included-ranges"], false, 1.0),
        (vec!["included-ranges"], true, 2.0),
        (vec!["included-ranges", "identity"], false, 2.0),
        (vec!["json-string", "included-ranges"], false, 2.0),
        (vec!["included-ranges", "future-view"], false, 2.0),
    ] {
        let mappings = mappings.into_iter().map(str::to_owned).collect::<Vec<_>>();
        assert_eq!(
            read_grammar_selection_included_range_context(&mappings, in_token).to_bits(),
            f64::to_bits(expected)
        );
    }
    assert!(accept_grammar_selection_included_range_source_range(
        "source-range",
        0.0,
        4.0
    ));
    for (kind, start, end) in [
        ("range", 0.0, 4.0),
        ("source-range", 0.0, 0.0),
        ("source-range", -1.0, 4.0),
        ("source-range", 0.0, 1.5),
        ("source-range", 0.0, 4_294_967_296.0),
    ] {
        assert!(!accept_grammar_selection_included_range_source_range(
            kind, start, end
        ));
    }
    assert_eq!(
        read_grammar_selection_included_range_field_order("derivation"),
        "operation|input|start|end|ranges"
    );
    assert_eq!(
        read_grammar_selection_included_range_field_order("range"),
        "kind|start|end"
    );
    assert_eq!(
        read_grammar_selection_included_range_field_order("json-string"),
        ""
    );
}

#[test]
fn included_range_scalar_stream_preserves_the_original_multibyte_example() {
    let source = "*é\n> β*";
    let bytes = source.as_bytes();
    let widths = source
        .char_indices()
        .map(|(index, scalar)| (index, scalar.len_utf8()))
        .collect::<HashMap<_, _>>();
    let plan = read_grammar_selection_included_range_plan(&[0.0, 4.0, 6.0, 9.0], 9.0);
    let (mut row, mut position, mut output) = (0.0, 0.0, 0.0);
    let (mut intervals, mut result, mut done) = (Vec::new(), Vec::new(), false);
    for _ in 0..20 {
        let index = included_range_rules::ml_array::number_index(position).unwrap();
        let width = f64::from(u32::try_from(*widths.get(&index).unwrap_or(&0)).unwrap());
        let part = read_grammar_selection_included_range_part(&plan, row, position, output, width);
        if part.first().map(|value| value.to_bits()) == Some(1.0_f64.to_bits()) {
            done = true;
            break;
        }
        assert_eq!(part.len(), 8);
        assert_eq!(part[0].to_bits(), 0.0_f64.to_bits());
        intervals.push([part[4] + 2.0, part[5] + 2.0, part[6], part[7]]);
        if part[7] > part[6] {
            let start = included_range_rules::ml_array::number_index(part[4]).unwrap();
            let end = included_range_rules::ml_array::number_index(part[5]).unwrap();
            result.extend_from_slice(&bytes[start..end]);
        }
        (row, position, output) = (part[1], part[2], part[3]);
    }
    assert!(done);
    assert_eq!(
        intervals,
        vec![
            [2.0, 2.0, 0.0, 0.0],
            [2.0, 3.0, 0.0, 1.0],
            [3.0, 5.0, 1.0, 3.0],
            [5.0, 6.0, 3.0, 4.0],
            [6.0, 8.0, 4.0, 4.0],
            [8.0, 10.0, 4.0, 6.0],
            [10.0, 11.0, 6.0, 7.0],
            [11.0, 11.0, 7.0, 7.0]
        ]
    );
    assert_eq!(result, [42, 195, 169, 10, 206, 178, 42]);
    assert_eq!(std::str::from_utf8(&result).unwrap(), "*é\nβ*");
    assert!(read_grammar_selection_included_range_part(&plan, 0.0, 0.0, 1.0, 1.0).is_empty());
}

#[test]
fn included_range_utf8_boundaries_reject_every_continuation_byte() {
    let source = format!(
        "\0A{}€😀\u{feff}",
        (128..192)
            .map(|value| char::from_u32(value).unwrap())
            .collect::<String>()
    );
    let bytes = source.as_bytes();
    for index in 0..=bytes.len() {
        let boundary = std::str::from_utf8(&bytes[..index]).is_ok()
            && std::str::from_utf8(&bytes[index..]).is_ok();
        let byte = bytes.get(index).map_or(-1.0, |byte| f64::from(*byte));
        assert_eq!(
            accept_grammar_selection_included_range_byte_boundary(byte, index == bytes.len()),
            boundary
        );
    }
    for byte in [-1.0, 0.5, 256.0, f64::NAN, f64::INFINITY] {
        assert!(!accept_grammar_selection_included_range_byte_boundary(
            byte, false
        ));
    }
    assert!(accept_grammar_selection_included_range_byte_boundary(
        -1.0, true
    ));
}

#[test]
fn included_range_any_and_all_keep_physical_cst_and_complete_derived_witnesses() {
    let child = included_range_literal("*é\nβ*");
    let direct = included_range_parse(&child, &HashMap::new(), "*é\nβ*");
    assert!(direct.ok, "{:?}", direct.rejection);
    for mode in ["any-grammar", "all-grammar"] {
        for prefix in ["> ", "前\n> "] {
            let grammar = included_range_grammar(
                "*é\n> β*",
                &[[0, 4], [6, 9]],
                &["Inline", "InlineTwin"],
                mode,
                prefix,
                "\n",
            );
            let listing = render_grammar_links(&grammar);
            assert_eq!(
                render_grammar_links(&parse_grammar_links(&listing).unwrap()),
                listing
            );
            assert_eq!(
                render_grammar_links(
                    &parse_native_grammar(&render_native_grammar(&grammar)).unwrap()
                ),
                listing
            );
            let source = format!("{prefix}*é\n> β*\n");
            let grammars = HashMap::from([
                ("Inline".to_owned(), child.clone()),
                ("InlineTwin".to_owned(), child.clone()),
            ]);
            let outcome = included_range_parse(&grammar, &grammars, &source);
            assert!(outcome.ok, "{:?}", outcome.rejection);
            let bundle = included_range_bundle(&outcome);
            let embedded = included_range_embedding(&bundle["tree"]);
            assert_eq!(included_range_physical(&bundle["tree"]), source.as_bytes());
            assert_eq!(
                included_range_physical(&embedded["root"]),
                [42, 195, 169, 10, 206, 178, 42]
            );
            assert_eq!(
                embedded["root"],
                included_range_tree_data(direct.tree.as_ref().unwrap())
            );
            assert_eq!(
                embedded["selection"]["alternatives"]
                    .as_array()
                    .unwrap()
                    .len(),
                1
            );
            assert_eq!(
                embedded["selection"]["alternatives"][0]["root"],
                embedded["root"]
            );
            assert_eq!(
                embedded["selection"]["sourceMapping"]["ranges"],
                json!([[0, 4], [6, 9]])
            );
            let start = prefix.len();
            let intervals = [
                [0, 0, 0, 0],
                [0, 1, 0, 1],
                [1, 3, 1, 3],
                [3, 4, 3, 4],
                [4, 6, 4, 4],
                [6, 8, 4, 6],
                [8, 9, 6, 7],
                [9, 9, 7, 7],
            ]
            .map(|[begin, end, output_start, output_end]| {
                [start + begin, start + end, output_start, output_end]
            });
            assert_eq!(
                embedded["selection"]["sourceMapping"]["intervals"],
                json!(intervals)
            );
            assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 2);
            assert_eq!(
                bundle["sourcePlanes"][1]["derivation"],
                json!({"operation":"included-ranges","input":0,
            "start":start,"end":start+9,"ranges":[[0,4],[6,9]]})
            );
            assert!(std::ptr::eq(
                check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default())
                    .unwrap(),
                &bundle
            ));
        }
    }
}

#[test]
fn included_range_views_execute_the_shipped_json_grammar_without_changing_its_cst() {
    let source = "{\"a\":\n> 1}";
    let decoded = "{\"a\":\n1}";
    let child =
        parse_grammar_links(include_str!("../../src/data/native-grammars/json.lino")).unwrap();
    let direct = included_range_parse(&child, &HashMap::new(), decoded);
    assert!(direct.ok, "{:?}", direct.rejection);
    let host = included_range_grammar(source, &[[0, 6], [8, 10]], &["JSON"], "any-grammar", "", "");
    let outcome = included_range_parse(&host, &HashMap::from([("JSON".to_owned(), child)]), source);
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = included_range_bundle(&outcome);
    let embedded = included_range_embedding(&bundle["tree"]);
    assert_eq!(
        embedded["root"],
        included_range_tree_data(direct.tree.as_ref().unwrap())
    );
    assert_eq!(included_range_physical(&bundle["tree"]), source.as_bytes());
    let physical = included_range_physical(&embedded["root"]);
    assert_eq!(physical, decoded.as_bytes());
    assert_eq!(
        serde_json::from_slice::<Value>(&physical).unwrap(),
        json!({"a":1})
    );
    assert!(std::ptr::eq(
        check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap(),
        &bundle
    ));
}

#[test]
fn included_range_nested_json_source_planes_replay_from_text_and_binary_grammar_archives() {
    let physical = "> *é\n> β*\n";
    let source = serde_json::to_string(physical).unwrap();
    let child = included_range_literal("*é\nβ*");
    let middle = included_range_grammar(
        "*é\n> β*",
        &[[0, 4], [6, 9]],
        &["Inline"],
        "any-grammar",
        "> ",
        "\n",
    );
    let mut outer =
        parse_grammar_links(include_str!("../../src/data/native-grammars/json.lino")).unwrap();
    outer.add_rule(GrammarRule::new(
        "quoted_source",
        parse_links_expression(&format!(
            "(any-grammar (ref {}) (grammar-candidate Middle json-string))",
            outer.start().unwrap(),
        ))
        .unwrap(),
    ));
    outer.set_start("quoted_source");
    let grammars = HashMap::from([
        ("Outer".to_owned(), outer.clone()),
        ("Middle".to_owned(), middle),
        ("Inline".to_owned(), child),
    ]);
    let outcome = included_range_parse(&outer, &grammars, &source);
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = included_range_bundle(&outcome);
    assert_eq!(
        bundle["sourcePlanes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|plane| plane["byteLength"].clone())
            .collect::<Vec<_>>(),
        vec![json!(source.len()), json!(12), json!(7)]
    );
    assert_eq!(
        bundle["sourcePlanes"][1]["derivation"]["operation"],
        "json-string"
    );
    assert_eq!(
        bundle["sourcePlanes"][1]["derivation"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["operation", "input", "start", "end"]
    );
    assert_eq!(
        bundle["sourcePlanes"][2]["derivation"],
        json!({"operation":"included-ranges","input":1,
        "start":2,"end":11,"ranges":[[0,4],[6,9]]})
    );
    assert_eq!(included_range_physical(&bundle["tree"]), source.as_bytes());
    assert_eq!(
        included_range_physical(&included_range_embedding(&bundle["tree"])["root"]),
        [42, 195, 169, 10, 206, 178, 42]
    );
    check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    let mut documents = grammars.iter().map(|(name,grammar)| {
        let syntax = parse_lino_to_links_with_config(&render_grammar_links(grammar),&ParserConfig::without_comments()).unwrap();
        json!({"name":name,"syntax":syntax.iter().map(included_range_link_data).collect::<Vec<_>>()})
    }).collect::<Vec<_>>();
    documents.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
    let archive = json!({"grammars":documents,"bundle":bundle});
    for decoded in included_range_stored(&archive) {
        check_grammar_selection_mapped_tree(&decoded["bundle"], &FeatureParseOptions::default())
            .unwrap();
        let restored = decoded["grammars"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                let listing = row["syntax"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(included_range_link_text)
                    .collect::<Vec<_>>()
                    .join("\n");
                let name = row["name"].as_str().unwrap();
                let grammar = parse_grammar_links(&listing).unwrap();
                assert_eq!(
                    render_grammar_links(&grammar),
                    render_grammar_links(&grammars[name])
                );
                (name.to_owned(), grammar)
            })
            .collect::<HashMap<_, _>>();
        let replay = included_range_parse(&restored["Outer"], &restored, &source);
        assert!(replay.ok, "{:?}", replay.rejection);
        assert_eq!(included_range_bundle(&replay), decoded["bundle"]);
    }
}

#[test]
fn included_range_views_isolate_child_memos_and_keep_failed_source_definitions() {
    let host = parse_grammar_links(&format!("(grammar (start document))\n(rule document normal (choice ordered (any-grammar (ref physical) (grammar-range-candidate Letter included-ranges (source-range 0 1))) (any-grammar (ref physical) (grammar-range-candidate Letter included-ranges (source-range 1 2)))))\n(rule physical normal (literal {}))\n", percent_encode_links_text("xy!"))).unwrap();
    assert_eq!(
        host.rule("physical").unwrap().expr,
        GrammarExpr::Terminal("xy!".to_owned())
    );
    let outcome = included_range_parse(
        &host,
        &HashMap::from([("Letter".to_owned(), included_range_literal("y"))]),
        "xy!",
    );
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = included_range_bundle(&outcome);
    assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 3);
    assert_eq!(
        bundle["sourcePlanes"].as_array().unwrap()[1..]
            .iter()
            .map(|plane| plane["derivation"]["ranges"].clone())
            .collect::<Vec<_>>(),
        vec![json!([[0, 1]]), json!([[1, 2]])]
    );
    let embedded = included_range_embedding(&bundle["tree"]);
    assert_eq!(embedded["selection"]["sourceMapping"]["output"], 2);
    assert_eq!(included_range_physical(&embedded["root"]), [121]);
    check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
}

#[test]
fn included_range_archive_forgeries_fail_actual_source_correspondence() {
    let host = included_range_grammar(
        "*é\n> β*",
        &[[0, 4], [6, 9]],
        &["Inline"],
        "any-grammar",
        "> ",
        "\n",
    );
    let outcome = included_range_parse(
        &host,
        &HashMap::from([("Inline".to_owned(), included_range_literal("*é\nβ*"))]),
        "> *é\n> β*\n",
    );
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = included_range_bundle(&outcome);
    let embedded = included_range_embedding_path(&bundle["tree"], "/tree").unwrap();
    let mutations = [
        (
            format!("{embedded}/selection/sourceMapping/ranges"),
            json!([[0, 3], [6, 9]]),
        ),
        (
            "/sourcePlanes/1/derivation/ranges".to_owned(),
            json!([[6, 9], [0, 4]]),
        ),
        ("/sourcePlanes/1/derivation/ranges/0/1".to_owned(), json!(2)),
        (
            "/sourcePlanes/1/derivation/ranges/0".to_owned(),
            json!([0, 4, 1]),
        ),
        (
            format!("{embedded}/selection/sourceMapping/intervals/4/2"),
            json!(3),
        ),
        (
            format!("{embedded}/selection/sourceMapping/input"),
            json!(1),
        ),
        (format!("{embedded}/selection/sourceMapping"), {
            let mut mapping = bundle
                .pointer(&format!("{embedded}/selection/sourceMapping"))
                .unwrap()
                .clone();
            mapping["hidden"] = json!("extra");
            mapping
        }),
        (format!("{embedded}/root/end"), json!(6)),
    ];
    for (path, value) in mutations {
        let mut counterfeit = bundle.clone();
        *counterfeit.pointer_mut(&path).unwrap() = value;
        for decoded in included_range_stored(&counterfeit) {
            let error =
                check_grammar_selection_mapped_tree(&decoded, &FeatureParseOptions::default())
                    .unwrap_err();
            assert!(
                error.to_string().contains("mapped grammar CST:"),
                "{path}: {error}"
            );
        }
    }
}

#[test]
fn included_range_profiles_refuse_invalid_or_mixed_plans_before_child_execution_and_keep_budgets() {
    for ranges in [
        vec![[0, 2]],
        vec![[0, 10]],
        vec![[6, 9], [0, 4]],
        vec![[0, 5], [4, 9]],
        vec![[0, 9]],
        vec![[0, 4], [4, 9]],
    ] {
        let calls = Cell::new(0);
        let resolve = |_: &str| {
            calls.set(calls.get() + 1);
            Some(included_range_literal("*é\nβ*"))
        };
        let host =
            included_range_grammar("*é\n> β*", &ranges, &["Inline"], "any-grammar", "> ", "\n");
        let parser =
            compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).unwrap();
        // The borrowed native resolver prepares each unique selected grammar
        // during compile (load_selection.rs), unlike JavaScript's lazy loader.
        // Parse-phase diagnostics independently distinguish mapping admission
        // from an actual child attempt; a resolver counter alone cannot do so.
        assert_eq!(calls.get(), 1, "one unique candidate is prepared");
        calls.set(0);
        let failure = parser
            .parse_tree("> *é\n> β*\n".as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(!failure.ok);
        assert_eq!(calls.get(), 0, "{ranges:?}");
        let rejection = failure.rejection.unwrap();
        assert_eq!(rejection.reason, "invalidEmbeddingMapping");
        let diagnostics = rejection.diagnostics.unwrap();
        assert!(!diagnostics.is_empty());
        assert_eq!(diagnostics[0]["reason"], "invalidEmbeddingMapping");
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.get("language").is_none()),
            "invalid mapping is refused before language-tagged child diagnostics: {diagnostics:?}"
        );
    }
    let host = parse_grammar_links(&format!("(grammar (start document))\n(rule document normal (seq (literal %3E%20) (all-grammar (ref physical) (grammar-range-candidate Inline included-ranges (source-range 0 4) (source-range 6 9)) (grammar-range-candidate Different included-ranges (source-range 0 1))) (literal %0A)))\n(rule physical normal (literal {}))\n",percent_encode_links_text("*é\n> β*"))).unwrap();
    let calls = Cell::new(0);
    let resolve = |_: &str| {
        calls.set(calls.get() + 1);
        Some(included_range_literal("*"))
    };
    let parser =
        compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).unwrap();
    assert_eq!(calls.get(), 2, "both unique candidates are prepared");
    calls.set(0);
    let failure = parser
        .parse_tree("> *é\n> β*\n".as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert!(!failure.ok);
    assert_eq!(calls.get(), 0);
    let rejection = failure.rejection.unwrap();
    assert_eq!(rejection.reason, "unsupportedEmbeddingMapping");
    let diagnostics = rejection.diagnostics.unwrap();
    assert!(!diagnostics.is_empty());
    assert_eq!(
        diagnostics[0]["detail"],
        "included range candidates require one common ordered range plan"
    );
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| diagnostic.get("language").is_none()),
        "inconsistent plans refuse before language-tagged child diagnostics: {diagnostics:?}"
    );
    let valid = included_range_grammar(
        "*é\n> β*",
        &[[0, 4], [6, 9]],
        &["Inline"],
        "any-grammar",
        "> ",
        "\n",
    );
    let refusing_calls = Cell::new(0);
    let refusing = |_: &str| {
        refusing_calls.set(refusing_calls.get() + 1);
        Some(included_range_literal("!"))
    };
    let prepared =
        compile_feature_grammar(&valid, Some(&refusing), FeatureParseOptions::default()).unwrap();
    assert_eq!(refusing_calls.get(), 1);
    refusing_calls.set(0);
    let failure = prepared
        .parse_tree("> *é\n> β*\n".as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert!(!failure.ok);
    assert_eq!(refusing_calls.get(), 0);
    let rejection = failure.rejection.unwrap();
    assert_eq!(rejection.reason, "syntax");
    assert!(
        rejection
            .diagnostics
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["reason"] == "mappedSyntax"
                && diagnostic["language"] == "Inline"),
        "the admitted range really executes its refusing child and exposes its syntax diagnostic"
    );
    let host = included_range_grammar(
        "*é\n> β*",
        &[[0, 4], [6, 9]],
        &["Inline"],
        "any-grammar",
        "> ",
        "\n",
    );
    let resolve = |_: &str| Some(included_range_literal("*é\nβ*"));
    let parser =
        compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).unwrap();
    for (options, reason) in [
        (
            FeatureParseOptions {
                step_limit: Some(0),
                ..FeatureParseOptions::default()
            },
            "stepLimit",
        ),
        (
            FeatureParseOptions {
                memory_limit: Some(0),
                ..FeatureParseOptions::default()
            },
            "memoryBudget",
        ),
        (
            FeatureParseOptions {
                max_depth: Some(0),
                ..FeatureParseOptions::default()
            },
            "nestingDepth",
        ),
    ] {
        let failure = parser
            .parse_tree("> *é\n> β*\n".as_bytes(), &options)
            .unwrap();
        assert!(!failure.ok);
        assert_eq!(failure.rejection.unwrap().reason, reason);
    }
}

#[test]
fn included_range_public_cst_equality_keeps_physical_and_derived_witness_fields() {
    let host = included_range_grammar(
        "*é\n> β*",
        &[[0, 4], [6, 9]],
        &["First", "Second"],
        "all-grammar",
        "> ",
        "\n",
    );
    let child = included_range_literal("*é\nβ*");
    let outcome = included_range_parse(
        &host,
        &HashMap::from([
            ("First".to_owned(), child.clone()),
            ("Second".to_owned(), child),
        ]),
        "> *é\n> β*\n",
    );
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let tree = outcome.tree.unwrap();
    assert_eq!(tree, tree.clone());
    // The public tree's derived equality is distinct from the private executor
    // comparison policy; this test makes no claim to execute private walk.rs.
    for mutation in 0..8 {
        let mut candidate = tree.clone();
        let SyntaxTree::Node { children, .. } = &mut candidate else {
            panic!("normal document node");
        };
        let SyntaxTree::Embed {
            selection,
            source_children,
            root,
            ..
        } = children
            .iter_mut()
            .find(|child| matches!(child, SyntaxTree::Embed { .. }))
            .unwrap()
        else {
            unreachable!();
        };
        let selected = selection.as_mut().unwrap();
        match mutation {
            0 => selected.source_mapping.as_mut().unwrap()["ranges"][0][1] = json!(3),
            1 => selected.source_mapping.as_mut().unwrap()["ranges"]
                .as_array_mut()
                .unwrap()
                .reverse(),
            2 => selected.source_mapping.as_mut().unwrap()["intervals"][4][2] = json!(3),
            3 => selected.source_mapping.as_mut().unwrap()["output"] = json!(2),
            4 => *source_children = None,
            5 => included_range_shorten(&mut source_children.as_mut().unwrap()[0]),
            6 => included_range_shorten(root),
            7 => included_range_shorten(&mut selected.alternatives[0].root),
            _ => unreachable!(),
        }
        assert_ne!(tree, candidate, "mutation {mutation}");
        assert_ne!(
            included_range_tree_data(&tree),
            included_range_tree_data(&candidate)
        );
    }
}

fn included_range_shorten(tree: &mut SyntaxTree) {
    let (SyntaxTree::Node { end, .. }
    | SyntaxTree::Token { end, .. }
    | SyntaxTree::Error { end, .. }
    | SyntaxTree::Missing { end, .. }
    | SyntaxTree::Embed { end, .. }) = tree;
    *end -= 1;
}

#[test]
fn included_range_archived_parameters_retain_signed_zero_without_changing_grammar_integer_zero() {
    let host = included_range_grammar("xy!", &[[0, 1]], &["Letter"], "any-grammar", "", "");
    let listing = render_grammar_links(&host).replace("(source-range 0 1)", "(source-range -0 1)");
    assert!(listing.contains("(source-range -0 1)"));
    let host = parse_grammar_links(&listing).unwrap();
    assert!(render_grammar_links(&host).contains("(source-range 0 1)"));
    let outcome = included_range_parse(
        &host,
        &HashMap::from([("Letter".to_owned(), included_range_literal("x"))]),
        "xy!",
    );
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let mut bundle = included_range_bundle(&outcome);
    let embedded = included_range_embedding_path(&bundle["tree"], "/tree").unwrap();
    let origin = format!("{embedded}/selection/sourceMapping/ranges/0/0");
    assert_eq!(
        bundle.pointer(&origin).unwrap().as_f64().unwrap().to_bits(),
        0.0_f64.to_bits()
    );
    *bundle.pointer_mut(&origin).unwrap() = json!(-0.0);
    bundle["sourcePlanes"][1]["derivation"]["ranges"][0][0] = json!(-0.0);
    check_grammar_selection_mapped_tree(&bundle, &FeatureParseOptions::default()).unwrap();
    for stored in included_range_stored(&bundle) {
        assert_eq!(
            stored.pointer(&origin).unwrap().as_f64().unwrap().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(
            stored["sourcePlanes"][1]["derivation"]["ranges"][0][0]
                .as_f64()
                .unwrap()
                .to_bits(),
            (-0.0_f64).to_bits()
        );
        assert!(std::ptr::eq(
            check_grammar_selection_mapped_tree(&stored, &FeatureParseOptions::default()).unwrap(),
            &stored
        ));
    }
}
